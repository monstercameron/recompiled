# Round 13 Xbox-generation review packet

Reviewed 2026-10-16 against the 3,462-project catalog. This directory contains importer-ready research only; no catalog database or tracked project files were edited.

## Packet contents

- `new-projects.json`: 8 project rows across 7 distinct repositories (5 Game targets, 3 Software/tool records).
- `programming-languages.json`: 8 exact GitHub Languages API records, including identical per-repository maps for the two Monster Hunter game targets.
- `game-profiles.json`: 2 additive profiles for game keys with no existing facts (10 fact rows; the Monster Hunter G developer, publisher, and genre remain explicitly unknown).
- `wikipedia-links.json`: 3 records: a direct-link upgrade for Sonic 2006, a direct Monster Hunter article, and an exact-title search fallback for Monster Hunter G. Terraria and Ace Combat already have direct references in the catalog.
- `game-target-facts-audit.json`: source-linked canonical facts for all 5 staged Game targets. Existing Sonic, Terraria, and Ace Combat facts are audit-only; only the two empty Monster Hunter keys are staged as additive profiles.
- `dedupe-audit.json`: exact catalog checks, existing matches, related-but-distinct projects, held candidates, and the Monster Hunter hold re-evaluation.

## Review decisions

Eight proposed IDs and seven canonical repository URLs were absent from the catalog. Related same-game work is kept distinct only where the implementation or target differs: TerrariaOGC is a separate decompilation-based Windows port from TerrariaXDK; Project Gracemeria is a Xenon title module while the existing AC6 record is a ReXGlue port.

The Round 9 hold for `itsbigcforme67/mh1-xbox` is reversed. Round 9 relied on README text describing a phase-0 planning/toolkit scaffold. The current maintainer `docs/PLAYING.md` documents Linux, Windows, and Original Xbox builds, reports the single-player game playable, and says the Xbox build has been tested on one console and in xemu. An October 8 Windows test release exists but the maintainer says it has not been tested on real Windows hardware. The root README still contains the old phase-0 wording, so the project summaries preserve that contradiction and do not overstate platform validation. The same repository explicitly supports both Monster Hunter and Monster Hunter G, yielding two game-target rows and one unique source repository.

Carbon Android, Dante’s Inferno, Jet Set Radio Future, Lost Odyssey, Red Alert 2, and SCDA-XBERecomp all matched existing project records and were not added again. `mzzvxm/LARecomp` is held for successor/status reconciliation with the existing Midnight Club row; `jsenior10/jeff` is generic tooling; `LCERD/LegacyRecomp` lacks enough implementation detail; and `xemu-android` is an emulator.

Language byte counts come directly from the GitHub Languages API. No media were downloaded. Import and build remain with the catalog owner.
