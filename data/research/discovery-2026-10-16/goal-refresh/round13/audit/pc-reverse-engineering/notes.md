# PC reverse-engineering research — 2026-10-16

This pass stages nine importer-shaped rows for PC games and software with direct evidence of reconstructed source, byte-matching, or recovered target behavior. It does not modify `data/catalog.sqlite`, tracked files, or generated catalog output. No source code, game assets, binaries, screenshots, or other media were downloaded into this staging folder.

## Import set

- `ecruells/resident-evil-pc-decomp`: 1997 PC Resident Evil source reconstruction from Ghidra analysis; GPL-3.0; release v1.3.0 on 2026-09-27; README claims full-game playability and tracks 1,716/1,717 in-scope logic functions. Original game data is not included.
- `hotlandsoftware/decomp95`: Windows 95 Build 950.6 component decompilation; two components are byte-matched, while Notepad is 53% matching (38/57 functions). No GitHub license is declared; original OS files are supplied by users.
- `jdelezenne/donalds-alphabet-chase-dos`: PC DOS matching decompilation; README reports byte-identical relinks for the retail and alternate builds and excludes original game files. No GitHub license is declared.
- `PatrikScully/OpenSpidey`: Spider-Man (2000) PC data-based native Windows/Linux preview built on the upstream `krystalgamer/spidey-decomp` work. The maintainer says it does not execute the original PC binary, full decompilation/restoration is unfinished, and game data is user-supplied. No GitHub license is declared.
- `abdulahmad/NHL95PC`: DOS NHL 95 assembly reconstruction; source rebuilds the retail executable byte-exactly and C matching is in progress. README says the repository includes the original retail executable; no GitHub license is declared.
- `pdromnt/tou_decomp`: Tunnels of Underworld behavioral reconstruction; v0.9 release (2026-10-11), major gameplay-parity pass reported complete. The maintainer states no project-wide license is offered, original game data is included, and the work is not clean-room.
- `nanofives/sc3kre`: SimCity 3000 Unlimited static reverse engineering and format/parser recovery, explicitly not a game port yet. The project reports measured parser/format checks and excludes game binaries/assets/decompiled code. MIT covers only the original notes/tools; GitHub API reports `NOASSERTION`. Separate mods in the repository are not represented as rows.
- `sqz269/battlestations-pacific-decompile`: Windows PC x86 C++ source reconstruction. Five math routines are byte-checked, but the current milestone has not reconstructed renderer, asset loading, simulation or gameplay; there is no playable rebuild. The repository includes neither original files nor a project license.
- `huntergdavis/poolrad-decomp`: IBM PC DOS v1.3 source reconstruction in portable C. The maintainer reports 62 routines behavior-checked in the roadmap (66 under a separate strict full-C workstream measure), but no native playable route; the source license is deferred to a later release. This is a distinct repository/method from the catalog’s OpenGold reimplementation of Pool of Radiance, and the existing same-title project is called out in the dedupe audit.

## Evidence and verification

`primary/` preserves dated repository metadata/readme API responses and text snapshots for the initial serious candidates; `discovery/` records the additional search results, readme snapshots and API evidence for follow-up candidates. `primary-source-evidence.json` condenses the nine included repositories’ head commits, last push/release and license metadata. The official GitHub Languages endpoint map for each included repository is preserved exactly in `programming-languages.json`, checked on 2026-10-16; the largest tracked language is assigned to `programming_language` and the full byte totals are retained.

`dedupe-audit.json` checks case-insensitive repository URL identity and exact target names against the 3,462-row catalog, and records potentially confusing cross-platform/version or same-title matches. Prior importer-shaped project manifests in the staged research sets did not already contain any of these nine repos. A wider discovery search found `albovsky/nd-blackmoor-decomp` and `vonhoff/lemball-decomp`; both exact repository URLs are already in the catalog, so they were excluded. Earlier raw discovery/search results mentioned Resident Evil PC and the upstream Spider-Man decompilation; these were not treated as actual project additions.

`exclusions-and-caveats.json` explains why the annotated assembly-only Chip’s Challenge reconstruction is not assigned a misleading `Decompilation` label, distinguishes SC3K reverse engineering from ordinary mods in the same repository, and retains the private MW2 article only as an unimportable lead. It also records exact-repository duplicate candidates and license/content cautions for included rows.

## Validation

All project facts and status statements are attributed to maintainer README/release/API sources. `first_found` and `updated` are the catalog discovery/status-check date; project start dates are left null because primary sources did not provide a reliable start date. No popularity data was inferred. The manifest is a candidate staging set only and has not been imported into the live catalog.
