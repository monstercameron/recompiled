# Round 10 catalog research

Round 10 combines source-checked project discovery, current metadata refresh, game-profile hydration, media URIs, and missing Wikipedia references. The catalog database remains the source of truth; `scripts/build.sh` imports the reviewed manifests and regenerates `docs/catalog.json` and the Rust/WASM site.

## New projects and tools

- `reconciliation/project-additions.json` adds 36 projects after primary-source verification and exact repository/target deduplication. The set spans API translation, decompilation, emulation, fan localization, recompilation, and software/game reimplementation.
- `reconciliation/project-language-evidence.json` contains exact repository-language evidence for all 36 additions: 28 predominant-language assignments, seven exact empty GitHub maps, and one GitLab percentage-only result retained as unknown.
- `reconciliation/directory-additions.json` adds Kuriimu2, ScummTR, and Telltale Workbench to the translation-tools directory.
- `reconciliation/audit.md` and `audit.json` explain inclusion, duplicate handling, holds, exclusions, and the 102 other reviewed leads. The project, language, and directory importers passed on a disposable copy of the full catalog with clean SQLite integrity and foreign-key checks.

## Existing-project metadata

- `status-refresh/project-updates-reviewed.json` refreshes 60 projects using current maintainer README, release, commit, and repository API evidence. Fourteen statuses are clarified or corrected; platform, summary, and source availability are updated where evidence justified it. The audit keeps release dates, tag dates, and repository activity separate.
- `project-media/media.json` adds 29 source-linked image/video URIs across all six project kinds. Media are referenced by URL and were not downloaded or cached. Every selected project had an empty media group before import, so no existing media is replaced.
- `programming-languages-reviewed.json` records 264 previously blank project-language audits: 133 assignments and 131 explicit unknowns. GitHub/Codeberg byte maps remain distinct from GitLab percentage maps.

## Game metadata and Wikipedia

- `game-profiles/` adds 57 exact-key game profiles with 326 sourced facts. Each target was checked to have no previous facts because the game-fact importer replaces the target key's fact rows.
- `wikipedia-links-reviewed.json` fills 124 missing references: 42 direct English articles and 82 exact-title search links. `wikipedia-candidate-audit.csv` records every key and ambiguity decision; no fuzzy title matches were used.
- `wikipedia-direct-improvements-reviewed.json` replaces 16 search fallbacks only when a reviewed article directly identifies the game, edition, prototype, or chapter. Existing direct articles are preserved.
- `new-game-wikipedia/wikipedia-links.json` supplies direct articles for all 17 new exact game keys and upgrades three existing exact-title searches. The audit records 23 game targets, including six existing direct references that remain untouched; emulator system labels are excluded.

Each lane was tested with the real importer against a disposable catalog before it was wired into the build.
