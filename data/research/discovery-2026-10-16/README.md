# Continued discovery batch — 2026-10-16

This reviewed batch extends the source-linked software ownership catalog with projects discovered across game and application decompilation, recompilation, reimplementation, API translation, emulation, mobile ports, fan localizations, and firmware work.

## Imported data

- **284 project additions** in `project-additions-reviewed.json`; all IDs were checked against the 1,290-row source database and are new. Each row has a primary project URL, labeled HTTPS sources, a qualified status, and a distinct target/build description.
- **678 unique metadata updates** in `project-updates-reviewed.json`; updates for existing IDs were deduplicated and conflicting proposals were reviewed before merge. The full reconciliation is in `metadata-conflict-audit.md` and `metadata-conflicts.json`.
- **327 programming-language evidence records** in `programming-languages-reviewed.json` for the 284 additions plus the existing Budokai Tenkaichi 3 Android row, whose canonical repository changed from a stale fork to the maintained dfdx047 fork. Repository language values are aligned with the official byte-count endpoint; 46 addition-row labels were corrected to match that evidence. A further 42 existing projects with blank language values received official byte-count evidence; their field corrections are listed in `language-field-corrections.json`. For 125 remaining projects, the repository has no attributable language endpoint or no defensible dominant language, so the values remain blank with reasons recorded in `language-unknown-audit.csv`.
- **178 newly reviewed metadata patches** across 218 fields were merged into the update manifest. These add 145 update IDs beyond the previous reviewed batch; 33 shared IDs received distinct, non-conflicting fields. The new audit covers 42 language-field fixes, game source/platform/status details, API/port metadata, and software/runtime details. Verification-day dates were not written into the project activity-date field. See `metadata-gap-audit.md` and `language-unknown-audit.csv`.
- **179 exact-title Wikipedia search fallbacks** were added for game profiles without a verified direct English article. Existing direct links remain direct; fallback links are labeled as searches in the site UI.
- **79 media URLs** across **31 projects** in `media-reviewed.json`. Images and videos remain URI-only; no game art or video was copied into the repository. This includes 55 images and 24 video links, including one Reddit gameplay post found during the latest search.

## Project mix

| Type | Projects |
| --- | ---: |
| API Translation | 14 |
| Decompilation | 61 |
| Emulator | 23 |
| Fan Localization | 37 |
| Recompilation | 53 |
| Reimplementation | 96 |
| **Total** | **284** |

| Research lane | Additions |
| --- | ---: |
| `44-xbox-pc-ports` | 16 |
| `api-translation` | 1 |
| `console-decomp` | 9 |
| `console-to-console` | 3 |
| `continuation-api` | 7 |
| `continuation-emulators` | 8 |
| `continuation-games` | 41 |
| `continuation-misc` | 14 |
| `continuation-mobile` | 11 |
| `continuation-software` | 5 |
| `emulators` | 7 |
| `fan-localizations` | 11 |
| `federated-discovery` | 32 |
| `lanes-19-43-final` | 22 |
| `mobile-ports` | 6 |
| `playstation-sega-xbox` | 7 |
| `reddit-today-recomps` | 3 |
| `round1-18-final` | 67 |
| `software-reconstruction` | 14 |

## Review notes

- The 41-project game decomp/recomp lane received a separate target and taxonomy review. It had no ID or canonical-repository duplicates; parallel implementations remain separate only where the repositories/builds differ. The Turok: Rage Wars row is classified as a matching decompilation with a native Windows port, not a recompilation. Cosmo’s Cosmic Adventure remains a decompilation-based source port based on the upstream byte-matching reconstruction.
- Multiple rows share one repository only when it explicitly covers separate game targets or platform builds (for example, Gen1Recomp’s separate games and the Uncharted trilogy ports). The provenance manifest maps each row back to its reviewed input lane.
- The FM-1 firmware cluster contains 14 distinct custom firmware implementations/prototypes. Their device-test limits, release states, and license caveats remain in the per-project sources and summaries.
- Dates in this batch describe catalog discovery or the source’s latest verified activity. Unknown project start dates were not estimated.

## Excluded boundary cases

- `ti-84-plus-os-255mp-research` — The source is an evidence-indexed Ghidra analysis reference, not a matching decompilation or source reconstruction.
- `fm1-firmware-reverse-engineering` — This is protocol/disassembly research without a replacement firmware or matching source output; the repository also contains vendor firmware images whose rights are not covered by its project-file license.
- `fm1-transporter-firmware-dump-and-recovery-tool` — This is a firmware flashing/recovery utility, not a decompilation, recompilation, translation, emulator, or target-software reimplementation record.

Three other FM-1/TI candidates were held out after review: the TI-84 OS Ghidra reference is analysis documentation rather than a matching decompilation; FM-1-RE is disassembly/update-protocol research without a replacement or matching source output and includes vendor firmware files; FM-1 Transporter is a flash/recovery utility rather than a reconstruction project. The archived notes explain these boundaries.

`./scripts/build.sh` imports the project additions, source-backed updates, language evidence, and media URLs in that order, then exports the generated Pages catalog and builds the Rust/WASM site.
