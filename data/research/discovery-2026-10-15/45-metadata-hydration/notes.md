# Metadata audit — 2026-10-15

This audit used a read-only SQLite connection to `/Users/earlcameron/Desktop/recompiled/data/catalog.sqlite`. The selection was exactly the projects with an empty `projects.programming_language` value and no matching row in `project_language_evidence`: 64 projects. A final read-only recheck confirmed all 64 remain eligible and `PRAGMA integrity_check` reports `ok`; no live database, generated catalog, or build output was changed.

## Programming-language evidence

`candidates.json` contains 64 records in the `records` envelope accepted by `python3 scripts/catalog.py import-programming-languages`.

- 44 records have a predominant language derived as the maximum of the exact byte counts returned by GitHub’s official `GET /repos/{owner}/{repo}/languages` endpoint. Each record includes the complete API byte map, endpoint URL, project URL, and a statement of the total and top-language share. The API reports repository-level Linguist totals, not bytes attributed only to the project subdirectory. Basilisk II and SheepShaver therefore both cite their shared `cebix/macemu` repository totals.
- Seven GitHub language endpoints returned `{}`. The manifest records an explicit unknown rather than assigning a language without byte evidence: `ferrum-apple-silicon-windows-compat`, `khamrai-ps1-en`, `minish-cap-aleks-ultimate-nx`, `sikarugir-wine-wrapper`, `snakes-ngage-static-recomp`, `sonicn-ngage-static-recomp`, and `spriggan-mark-2-pce-english-translation`.
- Wine’s first-party GitLab Languages API reports percentages rather than bytes: C 81.17%, C++ 10.77%, Ruby 3.22%, POV-Ray SDL 1.12%, and Python 0.79%. The manifest assigns C as predominant and preserves the percentage values separately; it does not mislabel them as bytes.
- Twelve other primary project pages do not publish a source-repository language breakdown: CrossOver, Enkyori Ren'ai English Patch, the six Mumble Italian patches (The 7th Saga, Alcahest, Equinox, The Ignition Factor, Illusion of Gaia, Secret of Mana), Rosetta 2, Step On It, Xak III English Patch, and Ys IV English Text/Dub Patch. Each is recorded as unknown with a project-page/source-page URL and a reason; no guessed language was added.

The exact numeric GitHub maps are preserved in the candidate manifest. It also includes both `source_url` / `evidence_url` and a `sources` array; the importer consumes the recognized fields and safely ignores the supplementary provenance array.

## Other metadata patches

`metadata-updates.json` contains 27 unique, existing project IDs in the `updates` envelope accepted by `python3 scripts/catalog.py import-project-updates`. It proposes 24 source-availability clarifications from official project READMEs/product pages, 11 precise platform/OS/ABI refinements, and three `updated` dates from the latest official release entries. Some records contain multiple field changes, so those groups overlap.

The three `updated` dates use GitHub `published_at` release evidence only: Cavalry Language Switcher 2026-09-29 (patch p12), Fire Emblem Warriors Korean Translation 2026-10-11 (v0.2.3), and Pokémon Azoth English Translation 2026-10-13 (beta-22). Xiaomi PC Manager English has no tagged release, so its repository push/activity timestamp was not used as a project release date. No project-start dates were inferred from repository creation or commit timestamps.

## Verification and integration

Both files were run through the importer functions from `scripts/catalog.py` against an isolated **in-memory SQLite fixture** populated with the candidate IDs. The language importer accepted all 64 records and produced 64 evidence rows (19 unknown, 45 assigned); the project-update importer accepted all 27 records. This exercised the importer’s parsing and supported-field logic without opening or writing the live catalog.

After review, apply with:

```sh
python3 scripts/catalog.py import-programming-languages work/research-latest/17-metadata-audit/candidates.json
python3 scripts/catalog.py import-project-updates work/research-latest/17-metadata-audit/metadata-updates.json
```

These files are in the ignored `work/` research area. No project source code, game data, or media assets were downloaded or copied into the repository; README and API URLs are retained as evidence links.
