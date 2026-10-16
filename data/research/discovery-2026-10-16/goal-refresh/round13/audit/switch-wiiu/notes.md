# Nintendo Switch / Wii U and console-to-console research

Checked 2026-10-16 against `/Users/earlcameron/Desktop/recompiled/data/catalog.sqlite` using read-only SQLite access. The catalog held 3,462 project rows at the dedupe read. Five importer-shaped proposals are staged. I did not import them or edit tracked files or the canonical database.

## Selected projects

The packet proposes four game-specific source reconstruction/recompilation targets plus one software toolkit. The five repositories are distinct from exact catalog repository roots. Related same-game records are disclosed in `dedupe-audit.json`; none is collapsed by title alone.

- `brandraft-exp/wiiu-recomp`: Wii U RPX → Linux static recompilation for Monster Hunter 3 Ultimate. The source and maintainer progress log document title/menu/character creation/village dialogue/HUD checkpoints while explicitly leaving free movement, quests, and complete audio unverified. A different MH3U Linux/Windows repository is already cataloged; these are distinct repository/codebase identities.
- `Swiftshine/yww`: very early Wii U Yoshi’s Woolly World reconstruction. Captured `compile.py` builds a single C++ translation unit with GHS MULTI PPC; `TODO.md` still lists assembly diffing and match-percentage work. The tiny repository is staged with those limitations rather than implying a measurable decompilation milestone.
- `jwfeniello/scott-pilgrim-switch-decomp`: Nintendo Switch Complete Edition matching reconstruction. The README reports 17.452721% of the executable .text as exact functions and explicitly says the complete game does not yet rebuild or boot. The original retail revision and original toolchain remain uncertain.
- `bignegro/TOTK-Decomp`: early clean-room Switch reconstruction with C/C++ game-source files and setup tooling. The maintainer README says decomp.dev and objdiff tracking are not configured yet; no completion metric or release is claimed. The existing CryRecomp row is a different repository and recompilation research project. The sibling game-decomps lane supplied an evidence snapshot but confirmed this repository is not among its additions, so this is the only proposed record for it.
- `Jdiller-86/CTR-2-WUP`: 3DS-to-Wii U static-recompilation/injector software target. Current support is limited to original homebrew diagnostics; commercial-game conversion and the Windows GUI are expressly unavailable. Super Mario 3D Land is a plan only.

Language evidence is the exact GitHub Languages API byte map. C leads Python by only 1,722 bytes for CTR2WUP; the note records that this ranking may not correspond to the core implementation language. For TOTK, Python is largest but includes repository tooling.

## Game profiles

`game-profiles.json` and `wikipedia-links.json` add canonical facts and direct English Wikipedia article links for the two new game keys: Yoshi’s Woolly World (Good-Feel, Nintendo, platform, Wii U, 2015) and Scott Pilgrim Complete Edition (original PS3/Xbox 360 release in 2010; Complete Edition developer, platforms, and 2021-01-14 release). Existing Monster Hunter 3 Ultimate and Tears of the Kingdom game keys already have sourced facts and a direct article or exact-title search reference, so this packet does not replace or duplicate their profiles.

## Holds and exclusions

The Mario Kart Wii Switch project `LockingFR/mkw-switch-recomp` is held because its public repository is documentation-only and the private translator/runtime and game-derived code are not available to build; the catalog also contains another Mario Kart Wii Switch project. The Wind Waker HD Linux preview `flossbud/WWHDRecomp` is held for same-target/lineage review against the existing `ZeldaWWHDRecomp` row. Other lead decisions and exact repository duplicates are listed in `exclusions.json`.

The specified CafeRecomp, SwitchRecomp, and Wind Waker HD Switch leads are already present by their canonical repository roots. Planned-only, demo-only, inaccessible, annotation-database, or out-of-scope projects are not promoted.

## Evidence and validation

The captured primary-source text/API snapshots and SHA-256 inventory are in `evidence/candidates/` and `source-snapshot-index.json`; the local snapshot set contains text/JSON only. No game files, screenshots, game art, other media, ROMs, or generated artifacts were fetched.
