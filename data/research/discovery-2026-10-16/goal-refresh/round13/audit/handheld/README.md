# Handheld continuation round 13

Reviewed on 2026-10-16 against the read-only catalog at `/Users/earlcameron/Desktop/recompiled/data/catalog.sqlite`. This folder stages **7 net-new project/game-target records across 5 GitHub repositories**. Nothing was imported into SQLite or edited in the repository's tracked files. No ROMs, game assets, releases, or other binary assets were downloaded; source snapshots are repository metadata, README text, language maps, release metadata, and small license texts.

## Candidate set

- `gba-recomp-toolkit` — GBA ROM → native runtime/package tool (Software, Recompilation). The README says the map database remains incomplete (169,778 / ~4.1 million functions mapped) and does not identify the only fully mapped game. Android build prerequisites are documented, so no Android package/release claim is made.
- `gb2swf-static-recompilation-tool` — Game Boy / Game Boy Color ROM → SWF + offline HTML/Ruffle player (Software, Recompilation). It recompiles profiled blocks and retains an interpreter fallback; the README documents compatibility limitations.
- `psprism-psp-static-recompilation` — PSP ISO/ELF/PRX → C++20 and native macOS/Windows runtime, with PSP diagnostic/partial-overlay output (Software, Recompilation). No tagged release was found. PSP full-game output is not described as a performant play target.
- `daxter-psprism-macos-recompilation` — the distinct Daxter target for the same psprism repository. The maintainer README explicitly says Daxter is fully playable on macOS; it does not claim Daxter works on Windows or as a full-speed PSP build.
- `insaniquarium-deluxe-vita-api-port` — Android ARMv7 `libmain.so` → PS Vita loader with Vita-side libc/SDL/GL shims (Game, API Translation). The README reports v1.00a and around 36 FPS without overclocking, while noting a paused-keyboard softlock. No license is declared.
- `epoch-equinox-oracle-of-ages-player` and `epoch-equinox-oracle-of-seasons-player` — two explicitly supported Game Boy Color game targets sharing one native desktop player (Game, Emulator). Current README says the runtime interprets the supplied ROM; an older static C generation path was removed because generated code was not used. The separate native Ages reconstruction is a hidden developer preview. The MIT license is stated in the README and LICENSE file; GitHub API metadata says `NOASSERTION`.

## Game facts and deduplication

`game-profiles.json` adds the new Daxter and Insaniquarium! Deluxe keys with developer, publisher, genre, original platform, and year fields, sourced to direct Wikipedia articles. `wikipedia-links.json` supplies those article URLs. The two Oracle game keys already had direct Wikipedia URLs and six sourced canonical facts each in SQLite; `existing-game-facts.json` snapshots those records for review and the additive profile manifest leaves them untouched. The Oracle project records link to the existing direct article.

`dedupe-audit.json` records the URL/game/platform/type check for each candidate plus known cataloged handheld leads omitted from the batch. Similar candidates from other projects remain separately eligible only where the repository/game/target is distinct. `exclusions.json` records ambiguous or weakly supported leads. In particular, OpenStrike is held because its maintainer describes a standalone CS-like FPS that reads maps from a user-owned Counter-Strike copy, not a verified Counter-Strike port or reconstruction.

## Evidence and validation

`evidence/<owner>__<repo>/` contains repository API JSON, exact GitHub language-byte maps, maintainer README snapshots, release metadata, and license snapshots where declared. `github-search-leads.json` is discovery-only; GitHub/reddit/search results were not used as implementation proof. The project and game claims in the import manifests point to maintainer README/release sources; Wikipedia is used only for canonical game facts.

All staged JSON files parsed successfully. Validation found 7 unique candidate IDs and 7 language records, verified each repo/game/platform/target tuple is unique and absent from the read-only catalog, and checked that the two profile imports refer only to new game keys. Apply `project-additions.json` before `programming-languages.json`, `game-profiles.json`, and `wikipedia-links.json` if this batch is later approved for import; this agent did not run any importer.
