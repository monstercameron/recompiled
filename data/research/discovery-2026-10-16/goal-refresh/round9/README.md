# Round 9 reviewed catalog research

Round 9 combines 613 source-reviewed additions with two refreshed additions kept in round 8, for 615 unique projects. It also refreshes metadata for existing projects and adds sourced game facts, repository-language evidence, Wikipedia links, URI-only media references, and tools/websites.

## Included data

- `project-additions-reviewed.json`: 613 projects, with 379 Game rows and 234 Software rows. Project records include primary HTTPS sources and reviewed method, platform, status, and source-availability details. The two round-8 additions are BattleShip and PaperBoat; their duplicate base rows are omitted. GoldenEye remains held as an overlap with the live catalog.
- `project-updates-reviewed.json`: 139 existing project IDs and 153 source-backed field assignments. Five unsupported status proposals and other conflicts are documented in `reconciliation-audit.md`.
- `programming-languages-reviewed.json`: 406 exact GitHub Languages API records. Among the 613 round-9 additions, 322 have an exact same-day map: 294 nonempty maps and 28 explicit empty maps. Another 291 additions remain unknown because no exact repository byte map was verified. The file also includes 80 successful probes for previously blank GitHub rows, one verified Bochs map, and three existing-project refreshes. Round-8 projects have separate language evidence. Repository-wide byte totals do not necessarily represent only game code.
- `game-facts-missing-reviewed.json`: 260 sourced game profiles with 1,175 facts. This covers 224 of 332 new game keys and 36 existing keys that previously had no facts. Another 108 new keys are held for missing or ambiguous sources.
- `game-facts-additive-reviewed.json`: four complete profiles that preserve all existing facts and add six newly sourced facts.
- `wikipedia-links-reviewed.json`: 216 verified English Wikipedia links. Ten sourced profiles have no suitable direct article, and ambiguous or fuzzy matches are held.
- `project-media-reviewed.json`: 53 full replacement groups containing 93 remote media URIs. Existing groups are preserved, and no media files are downloaded or cached by this manifest.
- `directory-additions-reviewed.json`: seven sourced tools and websites.

The round-8 manifests are in `../round8/`. The project and profile evidence decisions, reconciliation counts, source caveats, held leads, and importer validation are recorded in `reconciliation-audit.md` and `game-facts-audit.md`.

## Build order

`scripts/build.sh` imports round-8 additions and language maps, then round-9 project additions, project updates, language evidence, missing and additive profiles, Wikipedia links, URI media, and directory items. It exports SQLite to `docs/catalog.json` and rebuilds the Rust/WASM site. The round-9 manifests were tested with the actual importers against a disposable catalog copy; SQLite integrity and foreign-key checks passed.
