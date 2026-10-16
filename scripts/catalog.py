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
PROJECT_KINDS = (
    "Decompilation", "Recompilation", "API Translation", "Fan Localization",
    "Reimplementation", "Emulator",
)
PROJECT_UPDATE_FIELDS = {
    "title", "game", "target_type", "platform", "kind", "status", "summary",
    "project_url", "updated", "strategy", "language", "source_available",
    "first_found", "project_started", "programming_language", "subtype",
}
PROJECT_KIND_SQL = ", ".join(f"'{kind}'" for kind in PROJECT_KINDS)

SCHEMA = f"""
CREATE TABLE IF NOT EXISTS metadata (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY, title TEXT NOT NULL, game TEXT NOT NULL,
    target_type TEXT NOT NULL DEFAULT 'Game' CHECK (target_type IN ('Game','Software')),
    platform TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ({PROJECT_KIND_SQL})),
    status TEXT NOT NULL, summary TEXT NOT NULL, project_url TEXT NOT NULL, updated TEXT,
    strategy TEXT NOT NULL DEFAULT 'Unclassified',
    language TEXT NOT NULL DEFAULT '',
    source_available TEXT NOT NULL DEFAULT 'Project page',
    first_found TEXT,
    project_started TEXT,
    programming_language TEXT NOT NULL DEFAULT '',
    subtype TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS sources (
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    position INTEGER NOT NULL, label TEXT NOT NULL, url TEXT NOT NULL,
    PRIMARY KEY (project_id, position)
);
CREATE TABLE IF NOT EXISTS project_media (
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('image', 'video')),
    label TEXT NOT NULL, url TEXT NOT NULL, preview_url TEXT,
    source_url TEXT NOT NULL, discovered_on TEXT NOT NULL,
    PRIMARY KEY (project_id, position), UNIQUE (project_id, url)
);
CREATE TABLE IF NOT EXISTS games (
    game_key TEXT PRIMARY KEY,
    wikipedia_url TEXT,
    wikipedia_title TEXT
);
CREATE TABLE IF NOT EXISTS game_facts (
    game_key TEXT NOT NULL REFERENCES games(game_key) ON DELETE CASCADE,
    position INTEGER NOT NULL, field TEXT NOT NULL, value_json TEXT NOT NULL,
    source_url TEXT NOT NULL, evidence TEXT NOT NULL,
    date_granularity TEXT, verified_on TEXT,
    PRIMARY KEY (game_key, position)
);
CREATE TABLE IF NOT EXISTS project_links (
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    position INTEGER NOT NULL, kind TEXT NOT NULL, label TEXT NOT NULL,
    url TEXT NOT NULL, source_url TEXT NOT NULL, notes TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (project_id, position), UNIQUE (project_id, url)
);
CREATE TABLE IF NOT EXISTS project_language_evidence (
    project_id TEXT PRIMARY KEY REFERENCES projects(id) ON DELETE CASCADE,
    predominant_language TEXT, breakdown_json TEXT, source_url TEXT NOT NULL,
    evidence TEXT NOT NULL, verified_at TEXT NOT NULL, reason_unknown TEXT
);
CREATE TABLE IF NOT EXISTS directory_items (
    id TEXT PRIMARY KEY, item_type TEXT NOT NULL CHECK (item_type IN ('Tool', 'Website')),
    category TEXT NOT NULL, name TEXT NOT NULL, description TEXT NOT NULL,
    url TEXT NOT NULL, source_url TEXT NOT NULL, source_evidence TEXT NOT NULL,
    verified_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS featured_projects (
    project_id TEXT PRIMARY KEY REFERENCES projects(id) ON DELETE CASCADE,
    position INTEGER NOT NULL UNIQUE CHECK (position BETWEEN 1 AND 3),
    github_stars INTEGER NOT NULL CHECK (github_stars >= 0),
    verified_at TEXT NOT NULL, source_url TEXT NOT NULL, ranking_url TEXT NOT NULL,
    evidence TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS process_resources (
    id TEXT PRIMARY KEY, position INTEGER NOT NULL UNIQUE CHECK (position BETWEEN 1 AND 5),
    title TEXT NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('video', 'article', 'course')),
    publisher_author TEXT NOT NULL,
    topic_key TEXT NOT NULL CHECK (topic_key IN ('decompilation', 'static_recompilation', 'source_port', 'api_translation')),
    description TEXT NOT NULL, url TEXT NOT NULL, source_url TEXT NOT NULL,
    source_evidence TEXT NOT NULL, recommendation_rationale TEXT NOT NULL,
    popularity_platform TEXT NOT NULL CHECK (popularity_platform IN ('youtube', 'github')),
    popularity_value TEXT NOT NULL, popularity_unit TEXT NOT NULL CHECK (popularity_unit IN ('views', 'stars')),
    popularity_secondary_value TEXT, popularity_secondary_unit TEXT,
    popularity_checked_on TEXT NOT NULL, popularity_evidence TEXT NOT NULL,
    evidence_url TEXT NOT NULL,
    CHECK (
        (popularity_platform = 'youtube' AND popularity_unit = 'views'
            AND popularity_secondary_value IS NOT NULL AND popularity_secondary_unit = 'likes')
        OR
        (popularity_platform = 'github' AND popularity_unit = 'stars'
            AND popularity_secondary_value IS NULL AND popularity_secondary_unit IS NULL)
    )
);
CREATE TABLE IF NOT EXISTS gallery_makers (
    id TEXT PRIMARY KEY, maker TEXT NOT NULL, logo_asset_path TEXT NOT NULL,
    logo_page_url TEXT NOT NULL, logo_image_url TEXT NOT NULL,
    license TEXT NOT NULL, license_url TEXT NOT NULL DEFAULT '', creator TEXT NOT NULL, attribution TEXT NOT NULL,
    verified_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS gallery_source_logos (
    id TEXT PRIMARY KEY, label TEXT NOT NULL, logo_asset_path TEXT NOT NULL,
    logo_page_url TEXT NOT NULL, logo_image_url TEXT NOT NULL,
    license TEXT NOT NULL, license_url TEXT NOT NULL DEFAULT '', creator TEXT NOT NULL, attribution TEXT NOT NULL,
    verified_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS console_gallery (
    id TEXT PRIMARY KEY, name TEXT NOT NULL, maker_id TEXT NOT NULL REFERENCES gallery_makers(id),
    system_type TEXT NOT NULL CHECK (system_type IN ('home', 'handheld', 'hybrid', 'add_on')),
    release_year INTEGER, asset_path TEXT NOT NULL,
    source_page_url TEXT NOT NULL, source_image_url TEXT NOT NULL,
    license TEXT NOT NULL, license_url TEXT NOT NULL DEFAULT '', creator TEXT NOT NULL, attribution TEXT NOT NULL,
    alt TEXT NOT NULL, source_logo_id TEXT NOT NULL REFERENCES gallery_source_logos(id),
    verified_at TEXT NOT NULL, asset_changes TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS projects_game ON projects(game);
CREATE INDEX IF NOT EXISTS projects_kind ON projects(kind);
CREATE INDEX IF NOT EXISTS console_gallery_maker ON console_gallery(maker_id);
CREATE INDEX IF NOT EXISTS console_gallery_type ON console_gallery(system_type);
"""


def connect():
    DATABASE.parent.mkdir(parents=True, exist_ok=True)
    connection = sqlite3.connect(DATABASE)
    connection.row_factory = sqlite3.Row
    connection.execute("PRAGMA foreign_keys = OFF")
    connection.executescript(SCHEMA)
    columns = {row["name"] for row in connection.execute("PRAGMA table_info(projects)")}
    for name, declaration in {
        "target_type": "TEXT NOT NULL DEFAULT 'Game'",
        "strategy": "TEXT NOT NULL DEFAULT 'Unclassified'",
        "language": "TEXT NOT NULL DEFAULT ''",
        "source_available": "TEXT NOT NULL DEFAULT 'Project page'",
        "first_found": "TEXT",
        "project_started": "TEXT",
        "programming_language": "TEXT NOT NULL DEFAULT ''",
        "subtype": "TEXT NOT NULL DEFAULT ''",
    }.items():
        if name not in columns:
            connection.execute(f"ALTER TABLE projects ADD COLUMN {name} {declaration}")
    game_columns = {row["name"] for row in connection.execute("PRAGMA table_info(games)")}
    for name, declaration in {"wikipedia_url": "TEXT", "wikipedia_title": "TEXT"}.items():
        if name not in game_columns:
            connection.execute(f"ALTER TABLE games ADD COLUMN {name} {declaration}")
    for table in ("gallery_makers", "gallery_source_logos", "console_gallery"):
        gallery_columns = {row["name"] for row in connection.execute(f"PRAGMA table_info({table})")}
        if gallery_columns and "license_url" not in gallery_columns:
            connection.execute(f"ALTER TABLE {table} ADD COLUMN license_url TEXT NOT NULL DEFAULT ''")
    _migrate_console_gallery_types(connection)
    _migrate_project_kinds(connection)
    connection.executescript("CREATE INDEX IF NOT EXISTS projects_game ON projects(game); CREATE INDEX IF NOT EXISTS projects_kind ON projects(kind); CREATE INDEX IF NOT EXISTS console_gallery_maker ON console_gallery(maker_id); CREATE INDEX IF NOT EXISTS console_gallery_type ON console_gallery(system_type);")
    connection.execute("PRAGMA foreign_keys = ON")
    problems = connection.execute("PRAGMA foreign_key_check").fetchall()
    if problems:
        connection.close()
        raise sqlite3.IntegrityError(f"Foreign key check failed after catalog migration: {problems[:3]}")
    return connection


def _migrate_console_gallery_types(connection):
    table = connection.execute("SELECT sql FROM sqlite_master WHERE type='table' AND name='console_gallery'").fetchone()
    if not table or "'hybrid'" in table["sql"]:
        return
    connection.commit()
    connection.execute("BEGIN")
    try:
        connection.execute("""CREATE TABLE console_gallery_rebuilt (
            id TEXT PRIMARY KEY, name TEXT NOT NULL, maker_id TEXT NOT NULL REFERENCES gallery_makers(id),
            system_type TEXT NOT NULL CHECK (system_type IN ('home','handheld','hybrid','add_on')),
            release_year INTEGER, asset_path TEXT NOT NULL, source_page_url TEXT NOT NULL,
            source_image_url TEXT NOT NULL, license TEXT NOT NULL, license_url TEXT NOT NULL DEFAULT '',
            creator TEXT NOT NULL, attribution TEXT NOT NULL, alt TEXT NOT NULL,
            source_logo_id TEXT NOT NULL REFERENCES gallery_source_logos(id),
            verified_at TEXT NOT NULL, asset_changes TEXT NOT NULL
        )""")
        connection.execute("""INSERT INTO console_gallery_rebuilt
            (id,name,maker_id,system_type,release_year,asset_path,source_page_url,source_image_url,
             license,license_url,creator,attribution,alt,source_logo_id,verified_at,asset_changes)
            SELECT id,name,maker_id,system_type,release_year,asset_path,source_page_url,source_image_url,
             license,license_url,creator,attribution,alt,source_logo_id,verified_at,asset_changes
            FROM console_gallery""")
        connection.execute("DROP TABLE console_gallery")
        connection.execute("ALTER TABLE console_gallery_rebuilt RENAME TO console_gallery")
        connection.commit()
    except Exception:
        connection.rollback()
        raise


def _migrate_project_kinds(connection):
    table = connection.execute("SELECT sql FROM sqlite_master WHERE type='table' AND name='projects'").fetchone()
    if not table or all(f"'{kind}'" in table["sql"] for kind in PROJECT_KINDS):
        return
    connection.commit()
    connection.execute("BEGIN")
    try:
        connection.execute(f"""CREATE TABLE projects_rebuilt (
            id TEXT PRIMARY KEY, title TEXT NOT NULL, game TEXT NOT NULL,
            target_type TEXT NOT NULL DEFAULT 'Game' CHECK (target_type IN ('Game','Software')),
            platform TEXT NOT NULL,
            kind TEXT NOT NULL CHECK (kind IN ({PROJECT_KIND_SQL})),
            status TEXT NOT NULL, summary TEXT NOT NULL, project_url TEXT NOT NULL, updated TEXT,
            strategy TEXT NOT NULL DEFAULT 'Unclassified', language TEXT NOT NULL DEFAULT '',
            source_available TEXT NOT NULL DEFAULT 'Project page', first_found TEXT,
            project_started TEXT, programming_language TEXT NOT NULL DEFAULT '', subtype TEXT NOT NULL DEFAULT ''
        )""")
        connection.execute("""INSERT INTO projects_rebuilt
            (id,title,game,target_type,platform,kind,status,summary,project_url,updated,strategy,language,
             source_available,first_found,project_started,programming_language,subtype)
            SELECT id,title,game,target_type,platform,
                CASE kind WHEN 'Translation' THEN 'Fan Localization' ELSE kind END,
                status,summary,project_url,updated,strategy,language,source_available,
                first_found,project_started,programming_language,subtype
            FROM projects""")
        connection.execute("DROP TABLE projects")
        connection.execute("ALTER TABLE projects_rebuilt RENAME TO projects")
        connection.commit()
    except Exception:
        connection.rollback()
        raise


def write_media(connection, project_id, items):
    """Replace a project's reviewed media while retaining its source provenance."""
    if not isinstance(items, list):
        raise ValueError(f"Media must be a list: {project_id}")
    seen = set()
    for item in items:
        if item.get("kind") not in {"image", "video"} or not item.get("label"):
            raise ValueError(f"Media needs a kind and label: {project_id}")
        for field in ("url", "source_url", "preview_url"):
            value = item.get(field)
            if field == "preview_url" and not value:
                continue
            parsed = urlparse(value or "")
            if parsed.scheme != "https" or not parsed.hostname or parsed.username or parsed.password:
                raise ValueError(f"Media {field} must be a public HTTPS URL: {project_id}")
        if item["url"] in seen:
            raise ValueError(f"Duplicate media URL: {project_id}")
        seen.add(item["url"])
    connection.execute("DELETE FROM project_media WHERE project_id = ?", (project_id,))
    connection.executemany(
        """INSERT INTO project_media
        (project_id, position, kind, label, url, preview_url, source_url, discovered_on)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)""",
        [(project_id, index, item["kind"], item["label"], item["url"],
          item.get("preview_url"), item["source_url"], item.get("discovered_on", date.today().isoformat()))
         for index, item in enumerate(items)],
    )


def import_media(path):
    records = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    if not isinstance(records, list) or not records:
        raise ValueError("Media input must be a non-empty list of project ids and media arrays")
    connection = connect()
    try:
        with connection:
            for record in records:
                project_id = record.get("id")
                if not connection.execute("SELECT 1 FROM projects WHERE id=?", (project_id,)).fetchone():
                    raise ValueError(f"Unknown media project: {project_id}")
                write_media(connection, project_id, record["media"])
    finally:
        connection.close()
    count = sum(len(record["media"]) for record in records)
    print(f"Imported {count:,} media links for {len(records):,} projects in {DATABASE}")


def _https_url(value, field, project_id):
    if not isinstance(value, str):
        raise ValueError(f"{field} must be a public HTTPS URL: {project_id}")
    parsed = urlparse(value)
    if parsed.scheme != "https" or not parsed.hostname or parsed.username or parsed.password:
        raise ValueError(f"{field} must be a public HTTPS URL: {project_id}")


def normalize_source_availability(project):
    value = project.get("source_available")
    if isinstance(value, str) and value.strip():
        note = value.strip()
        lowered = note.lower()
        if "archive" in lowered or "pulled" in lowered or "community fork" in lowered:
            return "Archived or community source"
        if lowered.startswith("unclear"):
            return "Source availability unclear"
        if lowered.startswith("no "):
            return "No public source code"
        if lowered.startswith("yes") and "source" in lowered:
            return "Public source repository"
        return note
    if value is False:
        return "No public source code"
    if value is True:
        repo_hosts = {"github.com", "gitlab.com", "codeberg.org"}
        urls = [project.get("project_url", ""), *(source.get("url", "") for source in project.get("sources", []))]
        if any((urlparse(url).hostname or "").lower() in repo_hosts for url in urls):
            return "Public source repository"
    return "Repository" if (urlparse(project["project_url"]).hostname or "") == "github.com" else "Project page"


def import_links(path):
    records = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    if not isinstance(records, list) or not records:
        raise ValueError("Link input must be a non-empty list of project resource URLs")
    grouped = {}
    for item in records:
        project_id = item.get("project_id") or item.get("id")
        if not project_id or not item.get("label") or not item.get("url") or not item.get("source_url"):
            raise ValueError("Each resource needs a project id, label, URL, and source URL")
        _https_url(item["url"], "Resource URL", project_id)
        _https_url(item["source_url"], "Resource source URL", project_id)
        grouped.setdefault(project_id, []).append(item)
    connection = connect()
    total = 0
    try:
        with connection:
            for project_id, items in grouped.items():
                if not connection.execute("SELECT 1 FROM projects WHERE id=?", (project_id,)).fetchone():
                    raise ValueError(f"Unknown resource project: {project_id}")
                existing = [dict(item) for item in connection.execute(
                    "SELECT kind,label,url,preview_url,source_url,discovered_on FROM project_media WHERE project_id=? ORDER BY position",
                    (project_id,),
                ).fetchall()]
                media_by_url = {item["url"]: item for item in existing}
                link_rows = []
                for item in items:
                    kind = str(item.get("kind", "docs")).lower()
                    if kind in {"image", "video"}:
                        media_by_url.setdefault(item["url"], {
                            "kind": kind, "label": item["label"], "url": item["url"],
                            "preview_url": item["url"] if kind == "image" else None,
                            "source_url": item["source_url"], "discovered_on": date.today().isoformat(),
                        })
                    else:
                        link_rows.append(item)
                if len(media_by_url) != len(existing):
                    write_media(connection, project_id, list(media_by_url.values()))
                next_position = connection.execute(
                    "SELECT COALESCE(MAX(position), -1) + 1 FROM project_links WHERE project_id=?", (project_id,)
                ).fetchone()[0]
                for offset, item in enumerate(link_rows):
                    connection.execute(
                        """INSERT INTO project_links(project_id,position,kind,label,url,source_url,notes)
                        VALUES(?,?,?,?,?,?,?) ON CONFLICT(project_id,url) DO UPDATE SET
                        kind=excluded.kind,label=excluded.label,source_url=excluded.source_url,notes=excluded.notes""",
                        (project_id, next_position + offset, str(item.get("kind", "docs")).lower(), item["label"],
                         item["url"], item["source_url"], item.get("notes", "")),
                    )
                total += len(items)
    finally:
        connection.close()
    print(f"Imported {total:,} source-linked resources for {len(grouped):,} projects in {DATABASE}")


def import_game_profiles(path):
    data = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    profiles = data.get("profiles", []) if isinstance(data, dict) else data
    if not isinstance(profiles, list):
        raise ValueError("Game profiles must contain a profile list")
    batch_verified_on = data.get("verified_at") if isinstance(data, dict) else None
    if batch_verified_on is not None:
        try:
            date.fromisoformat(batch_verified_on)
        except (TypeError, ValueError) as error:
            raise ValueError("Game profile verification date must be YYYY-MM-DD") from error
    connection = connect()
    fact_count = 0
    try:
        with connection:
            for profile in profiles:
                game_key = profile.get("game_key")
                if not game_key:
                    raise ValueError("Every game profile needs an exact game_key")
                connection.execute("INSERT OR IGNORE INTO games(game_key) VALUES(?)", (game_key,))
                connection.execute("DELETE FROM game_facts WHERE game_key=?", (game_key,))
                facts = profile.get("facts", [])
                if not isinstance(facts, list):
                    raise ValueError(f"Facts must be a list: {game_key}")
                for position, fact in enumerate(facts):
                    if not fact.get("field") or not fact.get("evidence") or not fact.get("source_url"):
                        raise ValueError(f"A game fact needs a field, source, and evidence: {game_key}")
                    _https_url(fact["source_url"], "Game fact source URL", game_key)
                    verified_on = fact.get("verified_on")
                    if verified_on is None:
                        verified_on = batch_verified_on
                    if verified_on is not None:
                        try:
                            date.fromisoformat(verified_on)
                        except (TypeError, ValueError) as error:
                            raise ValueError(f"Game fact verification date must be YYYY-MM-DD: {game_key}") from error
                    connection.execute(
                        """INSERT INTO game_facts(game_key,position,field,value_json,source_url,evidence,date_granularity,verified_on)
                        VALUES(?,?,?,?,?,?,?,?)""",
                        (game_key, position, fact["field"], json.dumps(fact.get("value"), ensure_ascii=False),
                         fact["source_url"], fact["evidence"], fact.get("date_granularity"), verified_on),
                    )
                    fact_count += 1
    finally:
        connection.close()
    print(f"Imported {fact_count:,} sourced game facts across {len(profiles):,} profiles in {DATABASE}")


def import_wikipedia_links(path):
    data = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    entries = data.get("entries", []) if isinstance(data, dict) else data
    if not isinstance(entries, list):
        raise ValueError("Wikipedia links must contain an entries list")
    connection = connect()
    try:
        with connection:
            for entry in entries:
                game_key = entry.get("game_key")
                url = entry.get("wikipedia_url")
                if not game_key or not url:
                    raise ValueError("Every Wikipedia link needs a game_key and wikipedia_url")
                _https_url(url, "Wikipedia URL", game_key)
                if urlparse(url).hostname != "en.wikipedia.org":
                    raise ValueError(f"Wikipedia link must point to en.wikipedia.org: {game_key}")
                if not connection.execute("SELECT 1 FROM games WHERE game_key=?", (game_key,)).fetchone():
                    raise ValueError(f"Wikipedia link references an unknown game: {game_key}")
                connection.execute(
                    "UPDATE games SET wikipedia_url=?,wikipedia_title=? WHERE game_key=?",
                    (url, entry.get("wikipedia_title"), game_key),
                )
    finally:
        connection.close()
    print(f"Imported Wikipedia links for {len(entries):,} games in {DATABASE}")


def import_programming_languages(paths):
    connection = connect()
    record_count = 0
    try:
        with connection:
            for path in paths:
                data = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
                if isinstance(data, list):
                    records = data
                elif "records" in data:
                    records = data["records"]
                else:
                    records = []
                    for update in data.get("metadata_updates", []):
                        fields = update.get("fields", {})
                        primary = fields.get("predominant_programming_language", {})
                        breakdown_field = fields.get("programming_language_breakdown", {})
                        primary_source = (primary.get("sources") or breakdown_field.get("sources") or [{}])[0]
                        breakdown = breakdown_field.get("value")
                        if isinstance(breakdown, list):
                            breakdown = {item["language"]: round(float(item.get("percentage", 0)) * 100)
                                         for item in breakdown if item.get("language")}
                        records.append({
                            "project_id": update.get("project_id"),
                            "predominant_language": primary.get("value"),
                            "language_breakdown_bytes": breakdown,
                            "evidence_url": primary_source.get("url"),
                            "evidence": primary_source.get("evidence", ""),
                            "verified_at": data.get("updated"),
                        })
                for record in records:
                    project_id = record.get("project_id") or record.get("id")
                    if not project_id or not connection.execute("SELECT 1 FROM projects WHERE id=?", (project_id,)).fetchone():
                        raise ValueError(f"Unknown project in programming-language evidence: {project_id}")
                    language = record.get("predominant_language") or record.get("predominant_programming_language")
                    breakdown = record.get("language_breakdown_bytes") or record.get("language_breakdown")
                    source_url = record.get("evidence_url") or record.get("repository_url")
                    if not source_url:
                        raise ValueError(f"Language evidence needs a source URL: {project_id}")
                    _https_url(source_url, "Language evidence URL", project_id)
                    if isinstance(breakdown, list):
                        breakdown = {item["language"]: round(float(item.get("percentage", 0)) * 100)
                                     for item in breakdown if item.get("language")}
                    unknown_reason = record.get("reason_unknown") or record.get("reason")
                    if not record.get("evidence") and unknown_reason:
                        evidence = f"Predominant language was not assigned: {unknown_reason}"
                    else:
                        evidence = record.get("evidence") or "Predominant language could not be determined from the repository evidence."
                    connection.execute("UPDATE projects SET programming_language=? WHERE id=?", (language or "", project_id))
                    connection.execute(
                        """INSERT INTO project_language_evidence
                        (project_id,predominant_language,breakdown_json,source_url,evidence,verified_at,reason_unknown)
                        VALUES(?,?,?,?,?,?,?) ON CONFLICT(project_id) DO UPDATE SET
                        predominant_language=excluded.predominant_language,breakdown_json=excluded.breakdown_json,
                        source_url=excluded.source_url,evidence=excluded.evidence,verified_at=excluded.verified_at,
                        reason_unknown=excluded.reason_unknown""",
                        (project_id, language, json.dumps(breakdown, ensure_ascii=False) if breakdown is not None else None,
                         source_url, evidence, record.get("verified_at") or (data.get("verified_at") if isinstance(data, dict) else None) or date.today().isoformat(),
                         unknown_reason),
                    )
                    record_count += 1
    finally:
        connection.close()
    print(f"Imported predominant-language evidence for {record_count:,} projects in {DATABASE}")


def import_directory(path):
    data = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    items = data.get("items", []) if isinstance(data, dict) else data
    if not isinstance(items, list):
        raise ValueError("Directory input must contain an item list")
    connection = connect()
    try:
        with connection:
            for item in items:
                if item.get("item_type") not in {"Tool", "Website"}:
                    raise ValueError(f"Unknown directory item type: {item.get('id')}")
                for field in ("id", "category", "name", "description", "url", "source_url", "source_evidence"):
                    if not item.get(field):
                        raise ValueError(f"Directory item needs {field}: {item.get('id')}")
                _https_url(item["url"], "Directory URL", item["id"])
                _https_url(item["source_url"], "Directory source URL", item["id"])
                connection.execute(
                    """INSERT INTO directory_items(id,item_type,category,name,description,url,source_url,source_evidence,verified_at)
                    VALUES(?,?,?,?,?,?,?,?,?) ON CONFLICT(id) DO UPDATE SET
                    item_type=excluded.item_type,category=excluded.category,name=excluded.name,
                    description=excluded.description,url=excluded.url,source_url=excluded.source_url,
                    source_evidence=excluded.source_evidence,verified_at=excluded.verified_at""",
                    (item["id"], item["item_type"], item["category"], item["name"], item["description"], item["url"],
                     item["source_url"], item["source_evidence"], item.get("verified_at", date.today().isoformat())),
                )
    finally:
        connection.close()
    print(f"Imported {len(items):,} sourced tools and websites in {DATABASE}")


def import_process_resources(path):
    data = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    items = data.get("items", []) if isinstance(data, dict) else data
    if not isinstance(items, list) or len(items) != 5:
        raise ValueError("Process resource input must contain exactly five items")
    if not all(isinstance(item, dict) for item in items):
        raise ValueError("Every process resource must be an object")
    ids = [item.get("id") for item in items]
    positions = [item.get("position") for item in items]
    if not all(isinstance(item_id, str) and item_id.strip() for item_id in ids) or len(set(ids)) != len(ids):
        raise ValueError("Process resource IDs must be present and unique")
    if not all(type(position) is int for position in positions):
        raise ValueError("Process resource positions must be integers")
    if sorted(positions) != [1, 2, 3, 4, 5]:
        raise ValueError("Process resource positions must be 1 through 5")
    required = (
        "title", "kind", "publisher_author", "topic_key", "description", "url", "source_url",
        "source_evidence", "recommendation_rationale", "popularity_platform", "popularity_value",
        "popularity_unit", "popularity_checked_on", "popularity_evidence", "evidence_url",
    )
    allowed_topics = {"decompilation", "static_recompilation", "source_port", "api_translation"}
    connection = connect()
    try:
        with connection:
            connection.execute("DELETE FROM process_resources")
            for item in sorted(items, key=lambda record: record["position"]):
                for field in required:
                    if not item.get(field):
                        raise ValueError(f"Process resource needs {field}: {item.get('id')}")
                text_fields = ("title", "kind", "publisher_author", "topic_key", "description", "url", "source_url",
                               "source_evidence", "recommendation_rationale", "popularity_platform", "popularity_value",
                               "popularity_unit", "popularity_checked_on", "popularity_evidence", "evidence_url")
                if any(not isinstance(item[field], str) or not item[field].strip() for field in text_fields):
                    raise ValueError(f"Process resource text fields must be non-empty strings: {item['id']}")
                if item["kind"] not in {"video", "article", "course"}:
                    raise ValueError(f"Unknown process resource kind: {item['id']}")
                if item["topic_key"] not in allowed_topics:
                    raise ValueError(f"Unknown process resource topic: {item['id']}")
                for field in ("url", "source_url", "evidence_url"):
                    _https_url(item[field], f"Process resource {field}", item["id"])
                try:
                    date.fromisoformat(item["popularity_checked_on"])
                except ValueError as error:
                    raise ValueError(f"Process resource popularity date must be YYYY-MM-DD: {item['id']}") from error
                for field in ("popularity_value", "popularity_secondary_value"):
                    value = item.get(field)
                    if value is not None and (not isinstance(value, str) or not value.strip()):
                        raise ValueError(f"Process resource {field} must be a non-empty display string: {item['id']}")
                platform = item["popularity_platform"]
                unit = item["popularity_unit"]
                secondary_value = item.get("popularity_secondary_value")
                secondary_unit = item.get("popularity_secondary_unit")
                if platform == "youtube":
                    if unit != "views" or not secondary_value or secondary_unit != "likes":
                        raise ValueError(f"YouTube popularity needs view and like counts: {item['id']}")
                elif platform == "github":
                    if unit != "stars" or secondary_value is not None or secondary_unit is not None:
                        raise ValueError(f"GitHub popularity needs only a star count: {item['id']}")
                else:
                    raise ValueError(f"Unknown process resource metric platform: {item['id']}")
                connection.execute(
                    """INSERT INTO process_resources
                    (id,position,title,kind,publisher_author,topic_key,description,url,source_url,
                     source_evidence,recommendation_rationale,popularity_platform,popularity_value,
                     popularity_unit,popularity_secondary_value,popularity_secondary_unit,
                     popularity_checked_on,popularity_evidence,evidence_url)
                    VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)""",
                    (item["id"], item["position"], item["title"], item["kind"], item["publisher_author"],
                     item["topic_key"], item["description"], item["url"], item["source_url"],
                     item["source_evidence"], item["recommendation_rationale"], platform,
                     item["popularity_value"], unit, secondary_value, secondary_unit,
                     item["popularity_checked_on"], item["popularity_evidence"], item["evidence_url"]),
                )
    finally:
        connection.close()
    print(f"Imported {len(items):,} sourced process resources in {DATABASE}")


def _local_asset_path(value, allowed_prefixes, description):
    if not isinstance(value, str) or not value or value.startswith("/") or ".." in pathlib.PurePosixPath(value).parts:
        raise ValueError(f"{description} must be a safe path relative to docs/")
    if not any(value.startswith(prefix) for prefix in allowed_prefixes):
        raise ValueError(f"{description} is outside its supported asset directory")
    path = (ROOT / "docs" / value).resolve()
    try:
        path.relative_to((ROOT / "docs").resolve())
    except ValueError as error:
        raise ValueError(f"{description} must stay inside docs/") from error
    if not path.is_file() or path.stat().st_size == 0:
        raise ValueError(f"{description} does not exist or is empty: {value}")
    if path.suffix.lower() not in {".svg", ".jpg", ".jpeg", ".png", ".webp"}:
        raise ValueError(f"{description} has an unsupported file type: {value}")
    return value


def import_console_gallery(path):
    data = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise ValueError("Console gallery manifest must be an object")
    makers = data.get("makers")
    source_logos = data.get("source_logos")
    items = data.get("items")
    if not all(isinstance(rows, list) and rows for rows in (makers, source_logos, items)):
        raise ValueError("Console gallery needs non-empty makers, source_logos, and items arrays")

    def validate_logo(row, fields, label, asset_prefix):
        for field in fields:
            if not isinstance(row.get(field), str) or not row[field].strip():
                raise ValueError(f"{label} needs {field}: {row.get('id', '<unknown>')}")
        _local_asset_path(row["logo_asset_path"], (asset_prefix,), f"{label} asset")
        _https_url(row["logo_page_url"], f"{label} file-page URL", row["id"])
        _https_url(row["logo_image_url"], f"{label} original-image URL", row["id"])
        if row.get("license_url"):
            _https_url(row["license_url"], f"{label} license URL", row["id"])

    maker_ids = set()
    for maker in makers:
        validate_logo(maker, ("id", "maker", "logo_asset_path", "logo_page_url", "logo_image_url", "license", "creator", "attribution", "verified_at"), "Gallery maker logo", "assets/maker-logos/")
        if maker["id"] in maker_ids:
            raise ValueError(f"Duplicate gallery maker id: {maker['id']}")
        maker_ids.add(maker["id"])
    source_ids = set()
    for source in source_logos:
        validate_logo(source, ("id", "label", "logo_asset_path", "logo_page_url", "logo_image_url", "license", "creator", "attribution", "verified_at"), "Gallery source logo", "assets/source-logos/")
        if source["id"] in source_ids:
            raise ValueError(f"Duplicate gallery source-logo id: {source['id']}")
        source_ids.add(source["id"])

    ids = set()
    for item in items:
        required = ("id", "name", "maker_id", "system_type", "asset_path", "source_page_url", "source_image_url", "license", "creator", "attribution", "alt", "source_logo_id", "verified_at")
        if not isinstance(item, dict) or any(not isinstance(item.get(field), str) or not item[field].strip() for field in required):
            raise ValueError(f"Console gallery item needs all required text fields: {item.get('id', '<unknown>') if isinstance(item, dict) else '<unknown>'}")
        if item["id"] in ids:
            raise ValueError(f"Duplicate console gallery item id: {item['id']}")
        ids.add(item["id"])
        if item["maker_id"] not in maker_ids or item["source_logo_id"] not in source_ids:
            raise ValueError(f"Console gallery item references a missing maker/source logo: {item['id']}")
        if item["system_type"] not in {"home", "handheld", "hybrid", "add_on"}:
            raise ValueError(f"Unknown console gallery system type: {item['id']}")
        if item.get("release_year") is not None and (type(item["release_year"]) is not int or not 1950 <= item["release_year"] <= date.today().year):
            raise ValueError(f"Console release year must be null or a plausible integer: {item['id']}")
        for field in ("source_page_url", "source_image_url"):
            _https_url(item[field], f"Console gallery {field}", item["id"])
        if item.get("license_url"):
            _https_url(item["license_url"], "Console gallery license URL", item["id"])
        if urlparse(item["source_page_url"]).hostname != "commons.wikimedia.org":
            raise ValueError(f"Console gallery file page must be on Wikimedia Commons: {item['id']}")
        if urlparse(item["source_image_url"]).hostname != "upload.wikimedia.org":
            raise ValueError(f"Console gallery image must be cached from a Wikimedia upload: {item['id']}")
        _local_asset_path(item["asset_path"], ("assets/consoles/",), f"Console photo for {item['id']}")
        if len(item["license"].strip()) < 8 or len(item["attribution"].strip()) < 8:
            raise ValueError(f"Console gallery license and attribution must be explicit: {item['id']}")
        try:
            date.fromisoformat(item["verified_at"])
        except (TypeError, ValueError) as error:
            raise ValueError(f"Console verification date must be YYYY-MM-DD: {item['id']}") from error
        item.setdefault("asset_changes", "Cached and optimized for the gallery; source file page and original URI retained.")

    connection = connect()
    try:
        with connection:
            connection.execute("DELETE FROM console_gallery")
            connection.execute("DELETE FROM gallery_makers")
            connection.execute("DELETE FROM gallery_source_logos")
            connection.executemany(
                """INSERT INTO gallery_makers
                (id,maker,logo_asset_path,logo_page_url,logo_image_url,license,license_url,creator,attribution,verified_at)
                VALUES(?,?,?,?,?,?,?,?,?,?)""",
                [(row["id"], row["maker"], row["logo_asset_path"], row["logo_page_url"], row["logo_image_url"], row["license"], row.get("license_url", ""), row["creator"], row["attribution"], row["verified_at"]) for row in makers],
            )
            connection.executemany(
                """INSERT INTO gallery_source_logos
                (id,label,logo_asset_path,logo_page_url,logo_image_url,license,license_url,creator,attribution,verified_at)
                VALUES(?,?,?,?,?,?,?,?,?,?)""",
                [(row["id"], row["label"], row["logo_asset_path"], row["logo_page_url"], row["logo_image_url"], row["license"], row.get("license_url", ""), row["creator"], row["attribution"], row["verified_at"]) for row in source_logos],
            )
            connection.executemany(
                """INSERT INTO console_gallery
                (id,name,maker_id,system_type,release_year,asset_path,source_page_url,source_image_url,
                 license,license_url,creator,attribution,alt,source_logo_id,verified_at,asset_changes)
                VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)""",
                [(row["id"], row["name"], row["maker_id"], row["system_type"], row.get("release_year"), row["asset_path"], row["source_page_url"], row["source_image_url"], row["license"], row.get("license_url", ""), row["creator"], row["attribution"], row["alt"], row["source_logo_id"], row["verified_at"], row["asset_changes"]) for row in items],
            )
    finally:
        connection.close()
    print(f"Imported {len(items):,} locally cached console photos with source credits")


def import_featured_projects(path):
    data = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    entries = data.get("entries", []) if isinstance(data, dict) else data
    if not isinstance(entries, list) or len(entries) != 3:
        raise ValueError("The showcase must contain exactly three project entries")
    connection = connect()
    try:
        with connection:
            connection.execute("DELETE FROM featured_projects")
            for entry in entries:
                project_id = entry.get("project_id")
                if not project_id or not connection.execute("SELECT 1 FROM projects WHERE id=?", (project_id,)).fetchone():
                    raise ValueError(f"Unknown featured project: {project_id}")
                stars = entry.get("github_stars")
                position = entry.get("position")
                if not isinstance(stars, int) or stars < 0 or not isinstance(position, int) or position not in {1, 2, 3}:
                    raise ValueError(f"Showcase rank and star count must be non-negative integers: {project_id}")
                source_url = entry.get("source_url")
                _https_url(source_url, "Showcase source URL", project_id)
                ranking_url = entry.get("ranking_url")
                _https_url(ranking_url, "Showcase ranking evidence URL", project_id)
                project_url = connection.execute("SELECT project_url FROM projects WHERE id=?", (project_id,)).fetchone()[0]
                if source_url.rstrip("/") != project_url.rstrip("/"):
                    raise ValueError(f"Showcase popularity evidence must link to the project's own source: {project_id}")
                project_repo_path = urlparse(project_url).path.rstrip("/")
                ranking = urlparse(ranking_url)
                if (urlparse(project_url).hostname != "github.com" or ranking.hostname != "api.github.com"
                        or ranking.path.rstrip("/") != f"/repos{project_repo_path}"
                        or ranking.query or ranking.fragment):
                    raise ValueError(f"Showcase star-count evidence must be this repository's GitHub API endpoint: {project_id}")
                connection.execute(
                    """INSERT INTO featured_projects(project_id,position,github_stars,verified_at,source_url,ranking_url,evidence)
                    VALUES(?,?,?,?,?,?,?)""",
                    (project_id, position, stars, entry.get("verified_at") or data.get("verified_at") or date.today().isoformat(),
                     source_url, ranking_url, entry.get("evidence") or "GitHub repository search result sorted by stars."),
                )
    finally:
        connection.close()
    print(f"Imported {len(entries)} popularity-ranked projects for the showcase in {DATABASE}")


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
                if project["kind"] == "Translation":
                    project["kind"] = "Fan Localization"
                if project["kind"] not in PROJECT_KINDS:
                    raise SystemExit(f"Unknown project kind in {project['id']}: {project['kind']}")
                project["target_type"] = project.get("target_type", "Game")
                if project["target_type"] not in {"Game", "Software"}:
                    raise SystemExit(f"Unknown target type in {project['id']}: {project['target_type']}")
                if not project["project_url"].startswith("https://"):
                    raise SystemExit(f"Project URL must use HTTPS: {project['id']}")
                if project.get("strategy") in (None, "", "Unclassified"):
                    if project["kind"] == "Decompilation":
                        description = f"{project['title']} {project['summary']}".lower()
                        project["strategy"] = "Annotated disassembly" if "disassembly" in description else "Matching decompilation"
                    elif project["kind"] == "Recompilation":
                        project["strategy"] = "Static recompilation"
                    elif project["kind"] == "Fan Localization":
                        project["strategy"] = "Fan translation patch"
                    elif project["kind"] == "API Translation":
                        project["strategy"] = "Platform API translation"
                    elif project["kind"] == "Emulator":
                        project["strategy"] = "Emulation"
                    else:
                        project["strategy"] = "Native reimplementation"
                if not project.get("language"):
                    project["language"] = "English" if project["kind"] == "Fan Localization" else ""
                project.setdefault("programming_language", "")
                project.setdefault("subtype", "")
                if project.get("source_available") in (None, "", "Project page"):
                    host = urlparse(project["project_url"]).hostname or ""
                    if host == "github.com" or host.endswith(".github.io"):
                        project["source_available"] = "Repository" if host == "github.com" else "Project page / patch"
                    else:
                        project["source_available"] = "Patch page" if project["kind"] == "Fan Localization" else "Project page"
                if not project.get("first_found"):
                    project["first_found"] = data.get("updated", date.today().isoformat()) if isinstance(data, dict) else date.today().isoformat()
                project.setdefault("project_started", None)
                sources = project.get("sources", [])
                if not sources or any(not source.get("url", "").startswith("https://") for source in sources):
                    raise SystemExit(f"Each project needs at least one HTTPS source: {project['id']}")
                connection.execute(
                    """INSERT INTO projects
                    (id, title, game, target_type, platform, kind, status, summary, project_url, updated,
                     strategy, language, source_available, first_found, project_started, programming_language, subtype)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    ON CONFLICT(id) DO UPDATE SET
                      title=excluded.title, game=excluded.game, target_type=excluded.target_type, platform=excluded.platform,
                      kind=excluded.kind, status=excluded.status, summary=excluded.summary,
                      project_url=excluded.project_url, updated=excluded.updated,
                      strategy=excluded.strategy, language=excluded.language,
                      source_available=excluded.source_available,
                      first_found=COALESCE(projects.first_found, excluded.first_found),
                      project_started=excluded.project_started,
                      programming_language=excluded.programming_language, subtype=excluded.subtype""",
                    (project["id"], project["title"], project["game"], project["target_type"], project["platform"], project["kind"], project["status"], project["summary"], project["project_url"]) + (
                        project.get("updated"), project["strategy"], project["language"],
                        project["source_available"], project.get("first_found"), project.get("project_started"),
                        project["programming_language"], project["subtype"],
                    ),
                )
                if project["target_type"] == "Game":
                    connection.execute("INSERT OR IGNORE INTO games(game_key) VALUES(?)", (project["game"],))
                connection.execute("DELETE FROM sources WHERE project_id = ?", (project["id"],))
                connection.executemany(
                    "INSERT INTO sources(project_id, position, label, url) VALUES (?, ?, ?, ?)",
                    [(project["id"], index, source["label"], source["url"]) for index, source in enumerate(sources)],
                )
                if "media" in project:
                    write_media(connection, project["id"], project["media"])
            updated = data.get("updated", date.today().isoformat()) if isinstance(data, dict) else date.today().isoformat()
            connection.execute(
                "INSERT INTO metadata(key, value) VALUES('updated', ?) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                (updated,),
            )
    finally:
        connection.close()
    print(f"Imported or updated {len(projects):,} verified records in {DATABASE}")


def import_project_updates(path):
    """Apply source-backed metadata patches without replacing project records or sources."""
    data = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    if not isinstance(data, dict) or not isinstance(data.get("updates"), list) or not data["updates"]:
        raise ValueError("Project updates must contain a non-empty updates array")
    updated = data.get("updated")
    if updated is not None and (not isinstance(updated, str) or not updated.strip()):
        raise ValueError("The update batch date must be a non-empty string")

    # Validate the complete patch before opening a write transaction. This keeps a
    # malformed batch from partially updating the catalog.
    normalized = []
    seen_ids = set()
    for update in data["updates"]:
        if not isinstance(update, dict):
            raise ValueError("Every project update must be an object")
        project_id = update.get("id")
        fields = update.get("fields")
        sources = update.get("sources")
        if not isinstance(project_id, str) or not project_id.strip():
            raise ValueError("Every project update needs an ID")
        if project_id in seen_ids:
            raise ValueError(f"Project update IDs must be unique in a batch: {project_id}")
        seen_ids.add(project_id)
        if not isinstance(fields, dict) or not fields:
            raise ValueError(f"Project update needs at least one field: {project_id}")
        unknown = set(fields) - PROJECT_UPDATE_FIELDS
        if unknown:
            raise ValueError(f"Unsupported metadata fields for {project_id}: {sorted(unknown)}")
        for field, value in fields.items():
            if not isinstance(value, str) or not value.strip():
                raise ValueError(f"Metadata values must be non-empty strings: {project_id}.{field}")
            if field == "kind" and value not in PROJECT_KINDS:
                raise ValueError(f"Unknown project kind in {project_id}: {value}")
            if field == "target_type" and value not in {"Game", "Software"}:
                raise ValueError(f"Unknown target type in {project_id}: {value}")
            if field == "project_url":
                _https_url(value, "Project URL", project_id)
        if not isinstance(sources, list) or not sources:
            raise ValueError(f"Every project update needs an HTTPS evidence source: {project_id}")
        unique_sources = []
        seen_urls = set()
        for source in sources:
            if not isinstance(source, dict):
                raise ValueError(f"Project source must be an object: {project_id}")
            label, url = source.get("label"), source.get("url")
            if not isinstance(label, str) or not label.strip():
                raise ValueError(f"Project source needs a label: {project_id}")
            _https_url(url, "Project source URL", project_id)
            if url not in seen_urls:
                unique_sources.append({"label": label.strip(), "url": url})
                seen_urls.add(url)
        normalized.append({"id": project_id, "fields": fields, "sources": unique_sources})

    connection = connect()
    try:
        with connection:
            for update in normalized:
                project_id = update["id"]
                project = connection.execute(
                    "SELECT target_type FROM projects WHERE id=?", (project_id,)
                ).fetchone()
                if not project:
                    raise ValueError(f"Cannot patch an unknown project ID: {project_id}")
                assignments = ", ".join(f"{field}=?" for field in update["fields"])
                values = [update["fields"][field] for field in update["fields"]]
                connection.execute(
                    f"UPDATE projects SET {assignments} WHERE id=?", (*values, project_id)
                )
                target_type = update["fields"].get("target_type", project["target_type"])
                if "game" in update["fields"] and target_type == "Game":
                    connection.execute(
                        "INSERT OR IGNORE INTO games(game_key) VALUES(?)", (update["fields"]["game"],)
                    )

                existing = connection.execute(
                    "SELECT position,label,url FROM sources WHERE project_id=? ORDER BY position", (project_id,)
                ).fetchall()
                existing_by_url = {row["url"]: row for row in existing}
                position = max((row["position"] for row in existing), default=-1) + 1
                for source in update["sources"]:
                    existing_source = existing_by_url.get(source["url"])
                    if existing_source:
                        if existing_source["label"] != source["label"]:
                            connection.execute(
                                "UPDATE sources SET label=? WHERE project_id=? AND position=?",
                                (source["label"], project_id, existing_source["position"]),
                            )
                        continue
                    connection.execute(
                        "INSERT INTO sources(project_id,position,label,url) VALUES(?,?,?,?)",
                        (project_id, position, source["label"], source["url"]),
                    )
                    existing_by_url[source["url"]] = {"position": position, "label": source["label"]}
                    position += 1

            updated = updated or date.today().isoformat()
            connection.execute(
                "INSERT INTO metadata(key,value) VALUES('updated',?) "
                "ON CONFLICT(key) DO UPDATE SET value=excluded.value", (updated,),
            )
    finally:
        connection.close()
    print(f"Applied {len(normalized):,} source-backed project updates in {DATABASE}")


def import_emulators(path):
    """Add or update emulator records while preserving existing project and source rows."""
    data = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    projects = data if isinstance(data, list) else data.get("projects", []) if isinstance(data, dict) else None
    if not isinstance(projects, list) or not projects:
        raise ValueError("Emulator input must contain a non-empty project list")

    # Duplicate project IDs collapse to the last record's project fields, while
    # retaining every distinct source URL supplied for that project.
    deduplicated = {}
    for record in projects:
        if not isinstance(record, dict):
            raise ValueError("Every emulator record must be an object")
        required = ("id", "title", "game", "platform", "kind", "status", "summary", "project_url")
        if any(not isinstance(record.get(field), str) or not record[field].strip() for field in required):
            raise ValueError(f"Emulator record is missing a required field: {record.get('id', '<unknown>')}")
        project_id = record["id"]
        if record["kind"] != "Emulator":
            raise ValueError(f"Emulator record must have kind Emulator: {project_id}")
        target_type = record.get("target_type", "Software")
        if target_type not in {"Game", "Software"}:
            raise ValueError(f"Unknown target type in emulator {project_id}: {target_type}")
        _https_url(record["project_url"], "Project URL", project_id)
        sources = record.get("sources")
        if not isinstance(sources, list) or not sources:
            raise ValueError(f"Each emulator needs at least one HTTPS source: {project_id}")
        normalized_sources = []
        source_urls = set()
        for source in sources:
            if (not isinstance(source, dict) or not isinstance(source.get("label"), str)
                    or not source["label"].strip() or not isinstance(source.get("url"), str)
                    or not source["url"].strip()):
                raise ValueError(f"Each emulator source needs a label and URL: {project_id}")
            _https_url(source["url"], "Project source URL", project_id)
            if source["url"] not in source_urls:
                normalized_sources.append({"label": source["label"], "url": source["url"]})
                source_urls.add(source["url"])

        normalized = dict(record)
        normalized["sources"] = normalized_sources
        if project_id in deduplicated:
            prior_sources = deduplicated[project_id]["sources"]
            seen_urls = {source["url"] for source in prior_sources}
            normalized["sources"] = prior_sources + [
                source for source in normalized_sources if source["url"] not in seen_urls
            ]
        deduplicated[project_id] = normalized

    connection = connect()
    try:
        with connection:
            for project in deduplicated.values():
                project_id = project["id"]
                existing = connection.execute(
                    "SELECT kind FROM projects WHERE id=?", (project_id,)
                ).fetchone()
                if existing and existing["kind"] != "Emulator":
                    raise ValueError(f"Emulator ID collides with non-emulator project: {project_id}")

                strategy = project.get("strategy") or "Emulation"
                language = project.get("language") or ""
                target_type = project.get("target_type", "Software")
                source_available = normalize_source_availability(project)
                connection.execute(
                    """INSERT INTO projects
                    (id,title,game,target_type,platform,kind,status,summary,project_url,updated,
                     strategy,language,source_available,first_found,project_started,programming_language,subtype)
                    VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
                    ON CONFLICT(id) DO UPDATE SET
                      title=excluded.title,game=excluded.game,target_type=excluded.target_type,platform=excluded.platform,
                      kind=excluded.kind,status=excluded.status,summary=excluded.summary,
                      project_url=excluded.project_url,updated=excluded.updated,
                      strategy=excluded.strategy,language=excluded.language,
                      source_available=excluded.source_available,
                      first_found=COALESCE(projects.first_found,excluded.first_found),
                      project_started=excluded.project_started,
                      programming_language=excluded.programming_language,subtype=excluded.subtype""",
                    (project_id, project["title"], project["game"], target_type, project["platform"], "Emulator",
                     project["status"], project["summary"], project["project_url"], project.get("updated"),
                     strategy, language, source_available,
                     project.get("first_found") or (data.get("updated") if isinstance(data, dict) else None) or date.today().isoformat(),
                     project.get("project_started"), project.get("programming_language") or "",
                     project.get("subtype") or ""),
                )
                if target_type == "Game":
                    connection.execute("INSERT OR IGNORE INTO games(game_key) VALUES(?)", (project["game"],))

                existing_sources = connection.execute(
                    "SELECT position,url FROM sources WHERE project_id=? ORDER BY position", (project_id,)
                ).fetchall()
                existing_by_url = {source["url"]: source["position"] for source in existing_sources}
                seen_urls = set(existing_by_url)
                next_position = max((source["position"] for source in existing_sources), default=-1) + 1
                for source in project["sources"]:
                    if source["url"] in seen_urls:
                        if source["url"] in existing_by_url:
                            connection.execute(
                                "UPDATE sources SET label=? WHERE project_id=? AND position=?",
                                (source["label"], project_id, existing_by_url[source["url"]]),
                            )
                        continue
                    connection.execute(
                        "INSERT INTO sources(project_id,position,label,url) VALUES(?,?,?,?)",
                        (project_id, next_position, source["label"], source["url"]),
                    )
                    seen_urls.add(source["url"])
                    next_position += 1

            updated = (data.get("updated") if isinstance(data, dict) else None) or date.today().isoformat()
            connection.execute(
                "INSERT INTO metadata(key,value) VALUES('updated',?) "
                "ON CONFLICT(key) DO UPDATE SET value=excluded.value", (updated,),
            )
    finally:
        connection.close()
    print(f"Imported or updated {len(deduplicated):,} emulator records in {DATABASE}")


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
            project["media"] = [dict(item) for item in connection.execute(
                "SELECT kind, label, url, preview_url, source_url, discovered_on "
                "FROM project_media WHERE project_id=? ORDER BY position", (row["id"],)
            ).fetchall()]
            project["links"] = [dict(item) for item in connection.execute(
                "SELECT kind, label, url, source_url, notes FROM project_links WHERE project_id=? ORDER BY position", (row["id"],)
            ).fetchall()]
            language = connection.execute(
                "SELECT predominant_language, breakdown_json, source_url, evidence, verified_at, reason_unknown "
                "FROM project_language_evidence WHERE project_id=?", (row["id"],)
            ).fetchone()
            project["programming_language_evidence"] = None if language is None else {
                "predominant_language": language["predominant_language"],
                "breakdown": json.loads(language["breakdown_json"]) if language["breakdown_json"] else None,
                "source_url": language["source_url"], "evidence": language["evidence"],
                "verified_at": language["verified_at"], "reason_unknown": language["reason_unknown"],
            }
            projects.append(project)
        profiles = []
        for game in connection.execute("SELECT game_key,wikipedia_url,wikipedia_title FROM games ORDER BY game_key COLLATE NOCASE"):
            facts = [dict(item) for item in connection.execute(
                "SELECT field,value_json,source_url,evidence,date_granularity,verified_on FROM game_facts WHERE game_key=? ORDER BY position",
                (game["game_key"],),
            ).fetchall()]
            for fact in facts:
                fact["value"] = json.loads(fact.pop("value_json"))
                fact["date_granularity"] = fact.pop("date_granularity")
                fact.pop("verified_on", None)
            profiles.append({
                "game_key": game["game_key"], "facts": facts,
                "wikipedia_url": game["wikipedia_url"], "wikipedia_title": game["wikipedia_title"],
            })
        directory_items = [dict(item) for item in connection.execute(
            "SELECT item_type,category,name,description,url,source_url,source_evidence,verified_at FROM directory_items ORDER BY item_type,category,name COLLATE NOCASE"
        ).fetchall()]
        featured_projects = [dict(item) for item in connection.execute(
            """SELECT project_id,position,github_stars,verified_at,source_url,ranking_url,evidence
            FROM featured_projects ORDER BY position"""
        ).fetchall()]
        process_resources = [dict(item) for item in connection.execute(
            """SELECT id,position,title,kind,publisher_author,topic_key,description,url,source_url,
            source_evidence,recommendation_rationale,popularity_platform,popularity_value,
            popularity_unit,popularity_secondary_value,popularity_secondary_unit,
            popularity_checked_on,popularity_evidence,evidence_url
            FROM process_resources ORDER BY position"""
        ).fetchall()]
        console_gallery = [dict(item) for item in connection.execute(
            """SELECT c.id,c.name,m.maker,c.system_type,c.release_year,c.asset_path,
            c.source_page_url,c.source_image_url,c.license,c.license_url,c.creator,c.attribution,c.alt,c.verified_at,
            c.asset_changes,m.id AS maker_id,m.logo_asset_path AS maker_logo_path,
            m.logo_page_url AS maker_logo_page_url,m.license AS maker_logo_license,
            m.license_url AS maker_logo_license_url,m.creator AS maker_logo_creator,m.attribution AS maker_logo_attribution,
            s.logo_asset_path AS source_logo_path,s.label AS source_logo_label,
            s.logo_page_url AS source_logo_page_url,s.license AS source_logo_license,s.license_url AS source_logo_license_url,
            s.creator AS source_logo_creator,s.attribution AS source_logo_attribution
            FROM console_gallery c JOIN gallery_makers m ON m.id=c.maker_id
            JOIN gallery_source_logos s ON s.id=c.source_logo_id
            ORDER BY m.maker COLLATE NOCASE,c.release_year,c.name COLLATE NOCASE"""
        ).fetchall()]
    finally:
        connection.close()
    if not projects:
        raise SystemExit(f"SQLite catalog is empty: {DATABASE}")
    payload = {"updated": updated["value"] if updated else date.today().isoformat(), "projects": projects, "profiles": profiles, "directory_items": directory_items, "featured_projects": featured_projects, "process_resources": process_resources, "console_gallery": console_gallery}
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
    project_updates_importer = commands.add_parser(
        "import-project-updates", help="Apply source-backed field patches without replacing project sources"
    )
    project_updates_importer.add_argument("input", help="JSON file with an updates array")
    emulator_importer = commands.add_parser("import-emulators", help="Add or update source-verified emulator records")
    emulator_importer.add_argument("manifest", help="JSON emulator manifest")
    media_importer = commands.add_parser("import-media", help="Attach reviewed source-linked media to existing projects")
    media_importer.add_argument("input", help="JSON list of project ids and media arrays")
    links_importer = commands.add_parser("import-links", help="Attach reviewed project images and resource URLs")
    links_importer.add_argument("input", help="JSON list of project resource URLs")
    profiles_importer = commands.add_parser("import-game-profiles", help="Import source-linked game metadata facts")
    profiles_importer.add_argument("input", help="JSON game profile manifest")
    wikipedia_importer = commands.add_parser("import-wikipedia-links", help="Attach Wikipedia articles or searches to game profiles")
    wikipedia_importer.add_argument("input", help="JSON game-to-Wikipedia URL manifest")
    language_importer = commands.add_parser("import-programming-languages", help="Import sourced repository language audits")
    language_importer.add_argument("inputs", nargs="+", help="JSON language audit manifests")
    directory_importer = commands.add_parser("import-directory", help="Import sourced tooling and community links")
    directory_importer.add_argument("input", help="JSON tool/site directory manifest")
    resource_importer = commands.add_parser("import-process-resources", help="Import sourced explanations and project references")
    resource_importer.add_argument("input", help="JSON process resource manifest")
    featured_importer = commands.add_parser("import-featured-projects", help="Import the three current popularity-ranked project spotlights")
    featured_importer.add_argument("input", help="JSON project IDs and dated GitHub star counts")
    gallery_importer = commands.add_parser("import-console-gallery", help="Import cached console photographs, logo sources, and licenses")
    gallery_importer.add_argument("input", help="JSON console photo gallery manifest")
    commands.add_parser("export", help="Build docs/catalog.json from SQLite")
    args = parser.parse_args()
    if args.command == "import":
        import_file(args.input)
    elif args.command == "import-project-updates":
        import_project_updates(args.input)
    elif args.command == "import-emulators":
        import_emulators(args.manifest)
    elif args.command == "import-media":
        import_media(args.input)
    elif args.command == "import-links":
        import_links(args.input)
    elif args.command == "import-game-profiles":
        import_game_profiles(args.input)
    elif args.command == "import-wikipedia-links":
        import_wikipedia_links(args.input)
    elif args.command == "import-programming-languages":
        import_programming_languages(args.inputs)
    elif args.command == "import-directory":
        import_directory(args.input)
    elif args.command == "import-process-resources":
        import_process_resources(args.input)
    elif args.command == "import-featured-projects":
        import_featured_projects(args.input)
    elif args.command == "import-console-gallery":
        import_console_gallery(args.input)
    else:
        export_file()


if __name__ == "__main__":
    main()
