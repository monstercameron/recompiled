# Apple Silicon and macOS game recompilation research

Reviewed on 2026-10-16 against catalog revision `c3cdb32` (3,462 project rows). This is a staging-only packet for parent review. The live SQLite catalog and tracked files were not edited, and none of these manifests was imported.

The reviewed additions contain three game-project rows, three exact GitHub Languages API byte maps, two new game profiles with ten sourced facts, and two Wikipedia links. Fallout: New Vegas already has a direct Wikipedia link and six sourced canonical facts in SQLite, so its project row reuses that game key without duplicating facts or links. Twilight Princess HD is a distinct Wii U game edition from the original Twilight Princess game key.

The candidates cover three different Apple Silicon paths:

- Fallout: New Vegas: 32-bit Windows x86 static recompilation to native arm64, with D3D9-to-Metal rendering.
- Casualties: Unknown: native Unity ARM64/Metal port preserving managed CIL and game data, plus a separate Wine/Rosetta/D3DMetal runtime path. The verified native target is the 7.0 Steam demo, not the unreleased full game.
- Twilight Princess HD: Wii U PowerPC static recompilation to C, with a native Cafe OS layer and Metal GX2 renderer. Only the USA v81/Rev 2 source is supported.

`athei/mtld3d` is already in the catalog as a released Wine D3D9-to-Metal translator, so no duplicate software row was staged. A separate repository, `notmgn/mtld3d`, has the same name and functional target but an unclear relationship; its README describes a release bundle while the GitHub releases API is empty. It is held in `exclusions.json` rather than added as a possible duplicate. Other Mac recompilation/port duplicates and thin candidates are listed there with their matching catalog IDs or exclusion reasons.

All code licenses are repository-code licenses only. The three project repositories require user-supplied game files and do not include game assets; their own pages describe local generation or use. Evidence snapshots include repository metadata, exact language-byte maps, release API responses, README/license text, and the relevant game-source pages. No game code, binaries, ROMs, assets, or release packages were downloaded.

## Manifests

- `new-projects.json` — 3 reviewed game projects.
- `programming-languages.json` — 3 repository-wide byte maps.
- `game-profiles.json` — 2 new game keys, 10 facts.
- `wikipedia-links.json` — 2 new links.
- `exclusions.json` — 12 duplicate, ambiguous, or insufficient-evidence candidates.
- `dedupe-audit.json` — exact repository and same-game/target screening against the catalog and earlier research manifests.
- `research-notes.md` and `search-log.md` — methods, status limits, and discovery notes.
- `evidence/` — source snapshots grouped by accepted or held candidate.

The parent agent should perform the reviewed imports, database conflict handling, and final build.
