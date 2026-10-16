# Software and firmware reconstruction — continuation round 12

Reviewed on 2026-10-16. This lane proposes 7 new non-game projects; it does not edit the live catalog database or tracked site files.

## Proposed additions

| Project | Type | Current evidence/status | Language API predominant |
| --- | --- | --- | --- |
| [Elektron Analog Rytm MKII OS reverse engineering and replacement firmware](https://github.com/ethernet8023/rytm-fw-re) | Reimplementation | Experimental. The maintainer reports that an alpha replacement OS booted on hardware and passed a memory probe; the public build initializes core devices, draws a display test pattern, and parks. Static splash and LED palette changes were separately hardware-confirmed. No GitHub release is listed; the latest source push was 2026-10-13. | Python |
| [HSTNS-PD44 server power-supply firmware decompilation](https://github.com/darwinbeing/hstns-pd44-firmware) | Decompilation | Core control-algorithm decompilation and C refactoring are reported complete. Configurable OCP/OVP shutdown, fan control, PMBus, calibration storage, and boot-integrity work remain listed as in progress or planned. No GitHub release was returned; latest source push was 2026-04-05. | Assembly |
| [Lelouch (SJCAM SJ4000 Air Allwinner V3 interface replacement)](https://github.com/keowu/sjcam) | Reimplementation | Experimental source project; release v0.0.1 of the Lelouch replacement was published 2026-07-19. The README documents a replacement for the original `sdv` application and build instructions tested on x86-64 Linux. The project does not claim to replace the whole OS or support other SJ4000 board variants. | C |
| [VEXAPI (VEX V5/VEXos private API reverse analysis)](https://github.com/cetio/VEXAPI) | Decompilation | Repository release v1.1 was published 2026-09-11, but the README says the firmware/API snapshots were last updated 2024-01-11, are semi-incomplete, and lack current-hardware verification. The README notes that the VEXos extraction/download workflow is not automated and that the checked-in split boot image cannot be confirmed as an extraction of the listed update package. | C++ |
| [Radtel 950 Pro firmware reverse-engineering workspace](https://github.com/JKI757/radtel-950-pro) | Decompilation | Reverse-engineering workspace rather than a replacement-firmware release. The README describes decrypted OEM images, Ghidra work, decompiled function maps, and peripheral analysis; custom firmware is only a long-term goal. No release or exact OEM firmware revision is identified. Latest source commit was 2025-10-26; repository metadata was updated 2026-09-19. | C |
| [FNIRSI 101xd open firmware (1013D/1014D port)](https://github.com/Donwulff/FNIRSI_1014D_Firmware) | Reimplementation | Experimental open firmware. The maintainer documents a hardware-tested 1014D build on branch `atlan4-base`; 1013D firmware builds are explicitly untested. No GitHub release was returned; latest source push was 2026-10-02. The repository is a fork and documents code lineage from the original 1013D reverse-engineering and later community firmware projects. | C |
| [libcbus (Clipsal C-Bus PCI protocol reimplementation)](https://github.com/micolous/cbus) | Reimplementation | A published Python implementation with MQTT/Home Assistant bridge and PCI simulator. README reports the most recent user-facing update section as 2020-02-22 and says its Hass.io integration was still in progress; latest source commit captured from GitHub was 2021-10-24. No GitHub release was returned; repository is not marked archived. | Python |

The status and license details are deliberately cautious. Rytm’s replacement firmware is only an experimental hardware bring-up; HSTNS-PD44 lists unfinished controller features; SJCAM Lelouch targets one Allwinner V3 board and its repository contains material that remains proprietary; VEXAPI documents a dated, partial firmware/API snapshot; Radtel has no published replacement firmware or stated OEM image version; the FNIRSI tree is a GPL-3.0 fork with 1013D/1014D test coverage differences; and libcbus is an older but substantive protocol implementation.

## Files

- `project-additions.json`: project importer input with source links and verification notes.
- `programming-languages.json`: exact repository-wide language-byte maps from GitHub’s Languages API.
- `media.json`: one URI-only SJCAM project banner directly linked from its README. No media bytes were copied into the catalog repository.
- `evidence.json`: claim-level primary-source ledger.
- `dedupe-audit.json`: read-only SQLite and existing-manifest audit, including separate review of the already-staged microProject and Pinacoteca records.
- `exclusions.json`: ambiguous candidates, nearby tools, and related but distinct catalog entries held out.
- `primary/`: fetched GitHub API/README/status/license snapshots used during review.

The exact-project dedupe scan searched 4,205 JSON, Markdown, CSV, text, and TOML files under `data/research/` and `work/` while excluding this lane’s freshly captured `primary/` source copies. It found no prior manifest hits for these seven candidates. The live database contained 3,411 projects (1,050 software targets) during the read-only URL check. One related RT-950 Pro bare-metal replacement record exists; JKI757/radtel-950-pro is a separate decompilation/annotation workspace, and the audit records the relationship so the rows are not conflated.

## Already staged candidates rechecked

`microProject` and `Pinacoteca` were found in `work/research-2026-10-16/goal-refresh/round10/software-legacy/new-projects.json`; they are not duplicated in this batch. Current README/roadmap/license checks still support retaining both with clear maturity caveats. The microProject record’s `programming_language` had been Java despite its exact repository Languages API map having HTML at 40,880,690 bytes and Java at 11,639,306 bytes. Its existing language audit was already correct; this lane corrects the staged predominant-language field to HTML and explicitly keeps Java/Swing as the application implementation language. Pinacoteca’s exact map is Rust only (21,674 bytes), unchanged.

## Checks

- All manifests are JSON and use `scripts/catalog.py`’s project/language/media shapes.
- Each proposed row has a unique ID, HTTPS canonical URL, supported type, and primary-source evidence.
- Language totals are exact byte values copied from the official GitHub REST endpoint; `programming_language` matches the largest repository-wide category.
- The single image URI is a direct URL from the project README and is only referenced remotely.
- The catalog database was queried with `mode=ro`; no tracked files or database records were modified.
