# Goal-refresh round 4

Reviewed 2026-10-16 against the live SQLite catalog and project-owned sources. The manifests are importer inputs; detailed per-target facts and evidence links live in each row. Project images and video remain remote URIs.

- `project-additions-reviewed.json`: 28 verified additions: Resident Evil Gaiden’s Android static recompilation, seven software/protocol reimplementations, and twenty MiSTer FPGA system cores. Each row links to the project source and status evidence. `updated` is the review date; upstream activity and release dates are stated in source labels or status text.
- `programming-languages-reviewed.json`: official GitHub language-byte evidence for all 28 additions.
- `project-updates-reviewed.json`: 53 readable source-availability/license corrections for emulator rows whose prior boolean imported as `1`, plus five status/activity reviews. Review dates are `2026-10-16`; upstream dates remain explicit in status/source evidence.
- `game-facts-reviewed.json`: 60 sourced facts for 15 exact game keys rechecked to have no existing facts.
- `wikipedia-links-reviewed.json`: 12 direct English Wikipedia article links replacing search fallbacks.
- `project-media-reviewed.json`: one remote Cave Story DS screenshot URI, with its repository page as provenance; no image bytes copied.

The game-profile fact import is limited to zero-fact game keys because the importer replaces all facts for every key listed. No game code, ROMs, or project media were copied.

The MiSTer entries each identify one guest system or arcade board and link to that core’s own repository. Existing multi-system MiSTer framework and core rows were checked; these repository-level system cores are separately cataloged targets. Each row preserves the upstream BIOS/ROM and license caveats. All `updated` dates for these 20 projects mean reviewed on 2026-10-16; upstream activity dates are in status text and source labels.
