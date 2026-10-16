"""Regression tests for non-destructive, source-backed project metadata patches."""
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path


CATALOG_PATH = Path(__file__).with_name("catalog.py")
SPEC = importlib.util.spec_from_file_location("catalog_updates_under_test", CATALOG_PATH)
catalog = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(catalog)


class ProjectUpdateImportTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        catalog.DATABASE = Path(self.temp.name) / "catalog.sqlite"
        with catalog.connect() as connection:
            connection.execute(
                """INSERT INTO projects
                (id,title,game,target_type,platform,kind,status,summary,project_url)
                VALUES(?,?,?,?,?,?,?,?,?)""",
                ("sample", "Sample", "Sample Game", "Game", "PC", "Decompilation",
                 "In progress", "Original summary", "https://github.com/example/sample"),
            )
            connection.executemany(
                "INSERT INTO sources(project_id,position,label,url) VALUES(?,?,?,?)",
                [
                    ("sample", 0, "README", "https://github.com/example/sample#readme"),
                    ("sample", 1, "Release", "https://github.com/example/sample/releases"),
                ],
            )

    def write_manifest(self, content):
        path = Path(self.temp.name) / "updates.json"
        path.write_text(json.dumps(content), encoding="utf-8")
        return path

    def test_patch_changes_only_named_fields_and_appends_unique_sources(self):
        path = self.write_manifest({
            "updated": "2026-10-15",
            "updates": [{
                "id": "sample",
                "fields": {"status": "Playable alpha"},
                "sources": [
                    {"label": "README refresh", "url": "https://github.com/example/sample#readme"},
                    {"label": "Compatibility report", "url": "https://example.org/report"},
                ],
            }],
        })

        catalog.import_project_updates(path)

        with catalog.connect() as connection:
            project = connection.execute("SELECT * FROM projects WHERE id='sample'").fetchone()
            sources = connection.execute(
                "SELECT label,url FROM sources WHERE project_id='sample' ORDER BY position"
            ).fetchall()
        self.assertEqual(project["status"], "Playable alpha")
        self.assertEqual(project["summary"], "Original summary")
        self.assertEqual(project["platform"], "PC")
        self.assertEqual([row["url"] for row in sources], [
            "https://github.com/example/sample#readme",
            "https://github.com/example/sample/releases",
            "https://example.org/report",
        ])
        self.assertEqual(sources[0]["label"], "README")

    def test_unknown_project_rolls_back_the_entire_batch(self):
        path = self.write_manifest({
            "updates": [
                {
                    "id": "sample",
                    "fields": {"status": "Changed"},
                    "sources": [{"label": "Evidence", "url": "https://example.org/evidence"}],
                },
                {
                    "id": "missing",
                    "fields": {"status": "Unknown"},
                    "sources": [{"label": "Evidence", "url": "https://example.org/missing"}],
                },
            ],
        })

        with self.assertRaisesRegex(ValueError, "unknown project ID"):
            catalog.import_project_updates(path)

        with catalog.connect() as connection:
            status = connection.execute("SELECT status FROM projects WHERE id='sample'").fetchone()[0]
            count = connection.execute("SELECT COUNT(*) FROM sources WHERE project_id='sample'").fetchone()[0]
        self.assertEqual(status, "In progress")
        self.assertEqual(count, 2)


if __name__ == "__main__":
    unittest.main()
