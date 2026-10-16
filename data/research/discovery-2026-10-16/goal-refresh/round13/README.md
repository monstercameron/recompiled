# Round 13 source-verified catalog update

Round 13 adds 98 projects after exact repository and target checks against the 3,462-row catalog. The pass covers decompilation, recompilation, API translation, reimplementation, emulation, fan localization, desktop and mobile ports, handheld/console targets, and software/firmware reconstruction.

| Project kind | Added | Game target | Software target |
| --- | ---: | ---: | ---: |
| Decompilation | 30 | 27 | 3 |
| Recompilation | 19 | 12 | 7 |
| Emulator | 23 | 8 | 15 |
| API Translation | 3 | 1 | 2 |
| Fan Localization | 9 | 9 | 0 |
| Reimplementation | 14 | 2 | 12 |
| **Total** | **98** | **59** | **39** |

The import set includes 101 GitHub language audits: one per new project and three refreshed maps for existing entries. It also applies four source-backed project updates; adds 50 game profiles with 263 sourced facts and 61 direct or exact-title Wikipedia references; attaches 49 remote image/video URLs across 38 projects; and adds five discovery-directory links.

## Review and evidence

Every project row includes a maintainer repository or official project URL, at least one HTTPS evidence link, a method, target type, platform scope, and current status with limitations. Programming-language byte totals come from the repository’s GitHub Languages API; empty maps stay explicitly unknown. Project and game media are stored as URLs with source pages. No game images, executables, ROMs, or firmware files were copied.

The `audit/` subdirectories retain lane research notes, source indexes, deduplication decisions, exclusions, and validation reports. Repository pages and release state were checked on 2026-10-16. Same-repository projects are retained as separate rows only when the documented software or game target differs. Ambiguous, derivative, inactive-without-evidence, and insufficiently verified candidates remain in the lane exclusion files.

The five discovery links point to current project indexes and research sites; they do not import another catalog’s project dataset. The PCMR LATAM tracker pointer is qualified: its publisher’s X post links the article, but the article itself returned HTTP 403 during review, so its claims and project list remain unverified.

This is a point-in-time, source-verified discovery pass, not an exhaustive census of every project on the internet. The importer-ready aggregate manifests at this directory’s root are the files used by `scripts/build.sh`.
