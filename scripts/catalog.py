#!/usr/bin/env python3
"""Import verified project records into SQLite and export the Pages JSON view."""
import argparse
import json
import pathlib
import sqlite3
from datetime import date
from urllib.parse import urlparse

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATABASE = ROOT / "data" / "catalog.sqlite"
EXPORT = ROOT / "docs" / "catalog.json"

SCHEMA = """
PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS metadata (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY, title TEXT NOT NULL, game TEXT NOT NULL,
    platform TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('Decompilation', 'Recompilation', 'Translation')),
    status TEXT NOT NULL, summary TEXT NOT NULL, project_url TEXT NOT NULL, updated TEXT,
    strategy TEXT NOT NULL DEFAULT 'Unclassified',
    language TEXT NOT NULL DEFAULT '',
    source_available TEXT NOT NULL DEFAULT 'Project page',
    first_found TEXT,
    project_started TEXT
);
CREATE TABLE IF NOT EXISTS sources (
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    position INTEGER NOT NULL, label TEXT NOT NULL, url TEXT NOT NULL,
    PRIMARY KEY (project_id, position)
);
CREATE INDEX IF NOT EXISTS projects_game ON projects(game);
CREATE INDEX IF NOT EXISTS projects_kind ON projects(kind);
"""


def connect():
    DATABASE.parent.mkdir(parents=True, exist_ok=True)
    connection = sqlite3.connect(DATABASE)
    connection.row_factory = sqlite3.Row
    connection.executescript(SCHEMA)
    columns = {row["name"] for row in connection.execute("PRAGMA table_info(projects)")}
    for name, declaration in {
        "strategy": "TEXT NOT NULL DEFAULT 'Unclassified'",
        "language": "TEXT NOT NULL DEFAULT ''",
        "source_available": "TEXT NOT NULL DEFAULT 'Project page'",
        "first_found": "TEXT",
        "project_started": "TEXT",
    }.items():
        if name not in columns:
            connection.execute(f"ALTER TABLE projects ADD COLUMN {name} {declaration}")
    return connection


def import_file(path):
    data = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    projects = data if isinstance(data, list) else data.get("projects", [])
    if not isinstance(projects, list) or not projects:
        raise SystemExit("Input must contain a non-empty project list")
    connection = connect()
    try:
        with connection:
            for project in projects:
                required = ("id", "title", "game", "platform", "kind", "status", "summary", "project_url")
                if any(not project.get(field) for field in required):
                    raise SystemExit(f"Project is missing a required field: {project.get('id', '<unknown>')}")
                if project["kind"] not in {"Decompilation", "Recompilation", "Translation"}:
                    raise SystemExit(f"Unknown project kind in {project['id']}: {project['kind']}")
                if not project["project_url"].startswith("https://"):
                    raise SystemExit(f"Project URL must use HTTPS: {project['id']}")
                if project.get("strategy") in (None, "", "Unclassified"):
                    if project["kind"] == "Decompilation":
                        description = f"{project['title']} {project['summary']}".lower()
                        project["strategy"] = "Annotated disassembly" if "disassembly" in description else "Matching decompilation"
                    elif project["kind"] == "Recompilation":
                        project["strategy"] = "Static recompilation"
                    else:
                        project["strategy"] = "Fan translation patch"
                if not project.get("language"):
                    project["language"] = "English" if project["kind"] == "Translation" else ""
                if project.get("source_available") in (None, "", "Project page"):
                    host = urlparse(project["project_url"]).hostname or ""
                    if host == "github.com" or host.endswith(".github.io"):
                        project["source_available"] = "Repository" if host == "github.com" else "Project page / patch"
                    else:
                        project["source_available"] = "Patch page" if project["kind"] == "Translation" else "Project page"
                if not project.get("first_found"):
                    project["first_found"] = data.get("updated", date.today().isoformat()) if isinstance(data, dict) else date.today().isoformat()
                project.setdefault("project_started", None)
                sources = project.get("sources", [])
                if not sources or any(not source.get("url", "").startswith("https://") for source in sources):
                    raise SystemExit(f"Each project needs at least one HTTPS source: {project['id']}")
                connection.execute(
                    """INSERT INTO projects
                    (id, title, game, platform, kind, status, summary, project_url, updated,
                     strategy, language, source_available, first_found, project_started)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    ON CONFLICT(id) DO UPDATE SET
                      title=excluded.title, game=excluded.game, platform=excluded.platform,
                      kind=excluded.kind, status=excluded.status, summary=excluded.summary,
                      project_url=excluded.project_url, updated=excluded.updated,
                      strategy=excluded.strategy, language=excluded.language,
                      source_available=excluded.source_available, first_found=excluded.first_found,
                      project_started=excluded.project_started""",
                    tuple(project[field] for field in required) + (
                        project.get("updated"), project["strategy"], project["language"],
                        project["source_available"], project.get("first_found"), project.get("project_started"),
                    ),
                )
                connection.execute("DELETE FROM sources WHERE project_id = ?", (project["id"],))
                connection.executemany(
                    "INSERT INTO sources(project_id, position, label, url) VALUES (?, ?, ?, ?)",
                    [(project["id"], index, source["label"], source["url"]) for index, source in enumerate(sources)],
                )
            updated = data.get("updated", date.today().isoformat()) if isinstance(data, dict) else date.today().isoformat()
            connection.execute(
                "INSERT INTO metadata(key, value) VALUES('updated', ?) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                (updated,),
            )
    finally:
        connection.close()
    print(f"Imported or updated {len(projects):,} verified records in {DATABASE}")


def export_file():
    connection = connect()
    try:
        updated = connection.execute("SELECT value FROM metadata WHERE key='updated'").fetchone()
        project_rows = connection.execute("SELECT * FROM projects ORDER BY title COLLATE NOCASE").fetchall()
        projects = []
        for row in project_rows:
            project = {key: row[key] for key in row.keys()}
            project["sources"] = [dict(source) for source in connection.execute(
                "SELECT label, url FROM sources WHERE project_id=? ORDER BY position", (row["id"],)
            ).fetchall()]
            projects.append(project)
    finally:
        connection.close()
    if not projects:
        raise SystemExit(f"SQLite catalog is empty: {DATABASE}")
    payload = {"updated": updated["value"] if updated else date.today().isoformat(), "projects": projects}
    EXPORT.parent.mkdir(parents=True, exist_ok=True)
    temporary = EXPORT.with_suffix(".json.tmp")
    temporary.write_text(json.dumps(payload, ensure_ascii=False, separators=(",", ":")) + "\n", encoding="utf-8")
    temporary.replace(EXPORT)
    print(f"Exported {len(projects):,} projects to {EXPORT}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    importer = commands.add_parser("import", help="Upsert verified records from a JSON file")
    importer.add_argument("input", help="JSON file with a projects array")
    commands.add_parser("export", help="Build docs/catalog.json from SQLite")
    args = parser.parse_args()
    if args.command == "import":
        import_file(args.input)
    else:
        export_file()


if __name__ == "__main__":
    main()
