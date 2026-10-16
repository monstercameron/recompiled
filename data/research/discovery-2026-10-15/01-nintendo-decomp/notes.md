# Nintendo-family decompilation research — 2026-10-15

## Scope and method

Reviewed game-specific decompilations and assembly disassemblies for NES, SNES/Super Famicom, Game Boy/Color/Advance, Nintendo DS/3DS, GameCube, Wii, Wii U, and Switch. Used GitHub and project README searches as discovery paths, then based every proposed record and metadata claim on the project's own repository README or build instructions. No project tracker was used as evidence. Dates on these manifests are the catalog review/discovery date; `project_started` is omitted where a primary source did not state a reliable start date.

## New records

`new-projects.json` contains 8 game-target records:

- Wii Sports, Kirby Air Ride, and The Incredibles, for Wii/GameCube.
- Shin Megami Tensei (Super Famicom Japanese 1.0) and Arcana, for SNES.
- Adventure Island, Castlevania III, and Felix the Cat, for NES. These are separate game targets in one multi-title repository; each README title-list entry is distinct and the maintainer says the listed disassemblies build back to their original ROMs.

Before drafting, checked candidate repository URLs, IDs, and game/platform pairs against SQLite and all project arrays in the research manifests. The three NES rows intentionally share the collection repository because they are distinct targets; none duplicates an existing game/platform row.

## Metadata refreshes

`metadata-updates.json` contains 25 existing project IDs. It sharpens or corrects status, target-version/build details, source availability, strategy, or subtype based on the project's own README. Selected examples include:

- Frogger's Adventures: Temple of the Frog now distinguishes its byte-identical USA build (zero ROM-diff bytes and no nonmatching functions) from unfinished relocatable linking; the prior wording conflated the two.
- Metroid Fusion now records that USA is supported while Europe, Japan, China, and the European beta are still unsupported.
- The Link to the Past Japanese disassembly now reflects its byte-exact Japanese 1.0 build while preserving the maintainer's ongoing analysis work.
- Dragon Quest IX and Kingdom Hearts 358/2 Days now describe the matching/decompilation and build status at the level their READMEs report, including the source-only limitation for KH Days.
- DS Pokémon records now identify the specific U.S. ROM outputs/revisions documented by their READMEs.
- Zelda repositories for DS, 3DS, Switch, GameCube, and Wii now state when game assets/assembly are excluded and an original copy is needed, and record documented version limits.

All update IDs were checked against SQLite; all new project IDs/repository identities and exact game/platform pairs were checked against SQLite and the existing research manifests. Validation found 847 current catalog projects and scanned 700 manifest project records: zero new-ID collisions, zero missing update IDs, and zero repository or exact game/platform duplicates.

## Evidence and limits

Every row links to the relevant project-owned GitHub README. The NES collection README identifies the three selected games under its “Disassembled” list and states that listed sources are compilable and equal to the original ROM. For incomplete projects, status wording stays within what the README says; no completion percentage, supported platform, or start date was inferred when the source did not provide one. These files are research proposals only; SQLite, generated catalog JSON, documentation, and build scripts were not modified.
