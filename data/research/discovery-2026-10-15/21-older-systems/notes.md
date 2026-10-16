# Older-systems research lane — 2026-10-15

This batch covers Atari 2600/7800 and Atari arcade projects, Atari ST software reverse engineering, PC Engine/TurboGrafx fan translations, 3DO and CD-i ports/recompilation, an Atari Jaguar Linux port, and Neo Geo game ports/recompilations. Project state was checked against maintainer repositories, READMEs, release pages, and the Jaguar maintainer’s bring-up report. The catalog records only URLs and source references; no ROMs, patches, screenshots, or other game assets were copied.

The staged manifest has 49 records: 27 decompilations, 14 fan localizations, 5 reimplementations/ports, and 3 recompilations. Three records target software rather than a game: Atari ST `ST_TERM.TOS`, Atari TOS 1.00 (French), and Linux on Jaguar. PC Engine translation entries distinguish released patches from the Cyber Knight work in progress and the partial Dead of the Brain 1 release. Atari 7800 entries preserve the source READMEs’ byte-identity and code-coverage claims; a byte-identical ROM listing is not described as complete gameplay analysis.

I checked the proposals against the live SQLite catalog and every other staged discovery manifest, including the separate 46-row Sony/Sega batch. At validation time the live catalog had 1,000 rows and the other staged manifests had 363 project rows. There were no ID, exact project-URL, or same-game-and-platform collisions. `metadata-updates.json` is empty because this lane did not verify new status evidence for an existing catalog record; no SQLite or generated catalog files were changed.

## Exclusions and boundaries

- General-purpose emulators and tools were not added as game projects. This includes MAME, the Atari and Jaguar emulator records already in SQLite, the Neo Geo `neogeorecomp` toolkit, and the Atari vector emulator AAE. Their game-specific consumers are included where the repository identifies a distinct target.
- `Arcade-Tempest_MiSTer` is a playable FPGA arcade core, and Retro-3DO is an emulator. Both are useful leads but fall outside this software reconstruction/port lane; the existing catalog already has a general MiSTer record.
- Public original-source releases such as the 3DO DOOM source archive and Atari 7800 source-code mirrors were not counted as decompilation projects. The separate Phoenix DOOM desktop port and annotated 7800 reverse-engineering projects are included.
- Same-platform Jaguar Doom maintenance/mods, PC Engine Mini launcher/modding utilities, System Card translation-font helpers, and repositories with only a generic framework or no game-specific evidence were excluded.
- `nmikstas/asteroids-disassembly` is included with a cautious status: it publishes program and vector ROM assembly and says they assemble with Ophis, but the README does not claim byte-for-byte verification.
- The PC Engine translation pages provide a strong maintainer-maintained release index, but this lane is a dated source-backed discovery batch, not a claim that every historical patch or unindexed personal project on the Internet has been found.

## Validation

- Parsed both manifests and checked required fields, supported project kinds, target types, HTTPS source links, and unique IDs.
- Compared IDs, project URLs, and game/platform pairs against live SQLite and other staged project batches.
- `git diff --check -- data/research/discovery-2026-10-15/21-older-systems` passed.

Import this lane only after review with `python3 scripts/catalog.py import data/research/discovery-2026-10-15/21-older-systems/new-projects.json`.
