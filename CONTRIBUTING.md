# Contributing to Recompiled

Thanks for helping keep the catalog useful and verifiable. Project records are claims about real projects, so check the project's own repository, release notes, official site, or maintainer update before changing them. Search results, trackers, social posts, and videos can point to candidates; use project-owned sources to confirm what the project does and its current status.

## Add a project

1. Search the SQLite catalog and reviewed manifests for the project's repository URL, target name, and aliases. A project should appear once per distinct game or software target. Do not add a second row just because another source uses a different title for the same repository and target.
2. Read the project README and recent release or status notes. Confirm the target, source and target platforms, implementation method, source availability, and maturity. Record unknown details as unknown instead of inferring them.
3. Add a record to a reviewed JSON manifest under `data/research/`. Use a stable lowercase hyphenated `id`; include a useful `sources` link for each key source. The `project_url` should be the canonical project home.
4. Import the manifest, export the catalog, and run the checks below. Review the resulting changes in both SQLite and the generated JSON.

Example record (replace every value with verified project information):

```json
{
  "id": "example-game-switch-port",
  "title": "Example Game Native Switch Port",
  "game": "Example Game",
  "target_type": "Game",
  "platform": "Game Boy Advance → Nintendo Switch homebrew",
  "kind": "Decompilation",
  "status": "Early preview; the maintainer reports that the title screen and first level run.",
  "summary": "A native Switch port built from the game's decompilation. Requires the user's own game data; the repository does not distribute it.",
  "project_url": "https://github.com/example/example-game-switch-port",
  "updated": "2026-10-15",
  "strategy": "Decompilation-based native port",
  "programming_language": "C",
  "subtype": "Native homebrew port",
  "sources": [
    {
      "label": "Project README and platform instructions",
      "url": "https://github.com/example/example-game-switch-port"
    },
    {
      "label": "Latest project release",
      "url": "https://github.com/example/example-game-switch-port/releases"
    }
  ]
}
```

Every record requires `id`, `title`, `game` (the target name), `platform`, `kind`, `status`, `summary`, `project_url`, and at least one HTTPS `sources` URL. `target_type` defaults to `Game`; set it to `Software` for applications and utilities. The importer fills some optional fields, but add them when a source supports them. `updated` is the date you reviewed the record. `first_found` is when Recompiled first added it; `project_started` is the date the project began only when a reliable source states it. Leave unknown dates empty.

Use `Source platform → Target platform` for a platform port, with the actual systems named explicitly. For multi-platform projects, list every verified target. A console-to-console record should identify both console sides; a desktop, mobile, or web target can be included when the project supports it too. Mobile ports sourced from a console or PC should identify the original platform and each documented Android/iOS target. Do not describe emulator compatibility as a native port.

Choose the closest supported `kind` in `scripts/catalog.py`:

- `Decompilation` for recovered source code and projects built from that source, including source-based native ports. Say “decompilation-based port” in `strategy` when that is the actual deliverable.
- `Recompilation` for a target's machine code translated into a host-native executable, such as static recompilation of a game or application.
- `API Translation` for a runtime that translates or implements APIs across platforms, such as graphics API translation. This catalog's “translation” category does not mean fan language patches.
- `Fan Localization` for fan-made text or voice localization patches.
- `Reimplementation` for a new implementation of a target's behavior that is not a matching source decompilation or static recompilation.
- `Emulator` for software that emulates a console or other system. Keep general multi-system emulators distinct from game-specific ports.

Use a precise status: “boots,” “title screen works,” “playable,” “released,” “experimental,” or “archived” only when project-owned evidence supports it. Separate observed behavior from goals, and preserve limitations such as untested hardware, missing features, or early-release constraints in `status` or `summary`.

## Update an existing project

Keep the existing `id` when a repository changes name, adds a platform, publishes a release, or has a meaningful status update. Update the record in its reviewed manifest and add or replace source links to the current README, release, compatibility notes, or maintainer announcement. Check whether the `project_url` still redirects to the canonical repository. Do not create a new row for a release or platform addition to the same project.

If one repository contains distinct ports for multiple games, create one game-specific row per verified title and explain the relationship in the title or summary. If a repository combines several ports that share one codebase and maintainers, keep the rows linked to that canonical repository and avoid counting the repository multiple times in popularity summaries.

To refresh media, update `data/research/media.json` and use the media importer. Keep external media as original URLs and include the page that links to each asset; do not download or mirror project media. Console-gallery photos are the documented exception and require a Commons file page, exact license, creator credit, attribution, and the original image URL in the gallery manifest. After caching the unchanged source images, run `python3 scripts/generate-console-gallery-previews.py` to create 760px JPEG display copies under `docs/assets/console-previews/`; the gallery loads these lighter previews while keeping the original files and Commons credits.

## Import and verify

For an ordinary project manifest:

```sh
python3 -m json.tool data/research/new-projects.json >/dev/null
python3 scripts/catalog.py import data/research/new-projects.json
python3 scripts/catalog.py export
cargo test --locked --lib
cargo fmt -- --check
./scripts/build.sh
git diff --check
```

The build script imports the reviewed batches wired into `scripts/build.sh`, including `data/research/mobile-port-projects.json` and its sourced game-profile additions, exports SQLite to `docs/catalog.json`, and builds the Rust/WebAssembly site. Some datasets have specialized importers; see `python3 scripts/catalog.py --help` and the commands documented in [AGENTS.md](AGENTS.md). Do not edit `docs/catalog.json` by hand. Check that the expected project appears in both SQLite and the exported catalog, then inspect the local site at desktop and narrow mobile widths.

The SQLite importer upserts by `id`. Re-importing a reviewed manifest is safe for the same IDs and refreshes that record's source links. Before import, check for duplicate IDs within the manifest and duplicate project URLs or game/project combinations against SQLite:

```sh
python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("data/research/new-projects.json").read_text())
projects = manifest["projects"] if isinstance(manifest, dict) else manifest
ids = [project["id"] for project in projects]
assert len(ids) == len(set(ids)), "duplicate project id in manifest"
print(f"Validated {len(projects)} records")
PY
```

## Research and attribution

- Link the canonical project and the specific source pages used to verify it. Prefer a README, status page, release, or compatibility list over a search result.
- Use GitHub's repository API for language and popularity metadata when applicable; record the verification date for values that change.
- Treat community trackers as discovery aids, not as ownership or implementation evidence.
- Never commit ROMs, disc images, copyrighted game art, or other game files. Do not download media merely to make the catalog look complete.
- Keep candidate lists and exclusion reasons as research notes when a lead cannot be verified; do not import guesses into the live catalog.

For broader catalog discovery and status refresh workflows, see [AGENTS.md](AGENTS.md).
