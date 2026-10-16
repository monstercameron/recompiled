# Round 13 game decompilation/recompilation research

Checked: 2026-10-16. Catalog snapshot: `/Users/earlcameron/Desktop/recompiled/data/catalog.sqlite` (3,462 project rows). This folder is a review packet only; canonical SQLite and tracked files were not edited.

## Reviewed additions

The batch stages 12 source-verified repository-owned game projects: 8 decompilation projects and 4 static recompilation projects. Every staged status/method claim is tied to its maintainer README at a reviewed commit and GitHub repository metadata. Per-language byte maps are the exact GitHub Languages API response captured at each reviewed revision.

Project records are in `new-projects.json`; language audits are in `programming-languages.json`; license observations and repository snapshots are indexed separately. `evidence/candidates/` contains the captured repo metadata, README API response and decoded README, exact language map, latest commit, releases, license response/file where available, and fetch status for each lead. `evidence/search/` retains discovery search snapshots only as lead material. No binaries, media, game code, ROMs, or assets were copied.

## Game metadata

Ten new title/platform game keys have additive fact profiles and direct English Wikipedia references. Touhou 10: Mountain of Faith and Snail Mail already have five facts and direct Wikipedia links in the catalog, so those keys are deliberately omitted from the replacement-style `game-profiles.json` and `wikipedia-links.json`; the facts/references remain documented in `game-metadata-evidence.json`.

The English Zoids Saga article currently carries a single-source maintenance banner. For Zoids Saga Fuzors it names the alternate title and shows the Game Boy Advance series context but gives no game-specific developer/publisher or post-release date. Nintendo’s 2004 IR licensed-software list records `ZOIDS SAGA FUZORS` (Tomy) with a planned 2004-12-16 date and explicitly notes planned dates may change; this packet records year 2004 with that caveat, leaves exact release confirmation unresolved, and does not treat the listed licensee as independently verified developer.

Illbleed’s maintainer target table labels the supported executable `T45601M V1.003, 2001-03-06`; the direct Wikipedia game article lists the commercial Japanese release on 2001-03-29. These are preserved as separate target/build metadata and commercial release date rather than merged.

For Zoids Saga (first title), Wikipedia’s current English article provides Amedio, TOMY, GBA, RPG, and Japan 2001-11-30, but carries a single-source banner. The Fuzors record reuses the article only for game identity/platform context and uses the Nintendo IR schedule solely as planned-release evidence.

## Dedupe / held leads

Dedupe checks cover exact project IDs, normalized GitHub repository URLs, exact game keys, platform/build identifiers, and title-keyword aliases. Existing same-title rows and 18 exact repository duplicates among lead candidates are listed in `dedupe-audit.json` and `exclusions.json`. Top Gear SNES is distinct from Top Gear Rally N64; Dr. Mario NES from Dr. Mario 64; the two Zoids Saga titles from Zoids Legacy; Touhou PC matching decompilation from the Switch port; and Snail Mail Windows gameplay decompilation from the Android-to-Vita API translation. The possible 3on3 Xbox 360 addition is excluded because its game target already exists in the Windows and Android catalog rows.

`bignegro/TOTK-Decomp` is assigned to the sibling Switch/Wii U pass. `AdventureT/OpenToshi` is held for broad multi-game engine scope; `recomp-code/street-recomp-v3` is held pending target/method/status evidence from the maintainer. `phoboslab/wipeout-rewrite` is held out as a rewrite/source-port lead without verified decomp/recomp method. `SamidyFR/Game-Decompilations` and `Nio03/unricopie` are a discovery list and a tool, respectively, not game-specific projects.

## Import safety

`game-profiles.json` is additive only for keys with zero existing game facts at review time. It contains 10 profiles and 56 facts. `wikipedia-links.json` contains 10 direct links. Before importing, recheck the canonical database and preserve profile facts if another lane has since populated any of these keys. All JSON files in this folder were parsed and structurally validated after writing; no importer was run against canonical SQLite.
