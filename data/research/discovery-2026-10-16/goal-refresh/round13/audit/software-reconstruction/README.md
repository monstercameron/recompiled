# Round 13 software reconstruction research

This staging batch proposes 12 non-game software, firmware, and protocol implementation projects, checked on 2026-10-16 against their maintainers’ repositories, documentation, release pages, and GitHub metadata. It contains two matching decompilations and ten reimplementations. No catalog import, SQLite write, tracked-file edit, or media download was performed.

## Staged files

- `project-additions-reviewed.json` — ordinary project-import shape with target, method, status, platform, source availability, language, and source links.
- `programming-languages.json` — exact repository-wide byte totals from each project’s official GitHub Languages API endpoint.
- `evidence.json` — maintainer claims, GitHub repository dates/license metadata, latest releases, language maps, and local source-snapshot filenames.
- `dedupe-audit.json` — read-only checks against `/Users/earlcameron/Desktop/recompiled/data/catalog.sqlite` for IDs, repository identity, source/link URLs, and target aliases.
- `exclusions.json` — reviewed discovery leads that were held, excluded, or linked as a related existing target.
- `primary/` — saved repository READMEs, relevant maintainer documents and license texts, repository metadata, exact language maps, and release-list snapshots. The directory also contains snapshots for candidates held or excluded during screening.

## Included targets

| Project | Target | Method | Current evidence |
| --- | --- | --- | --- |
| `forgiv` | GivEnergy Hybrid Gen3 LV inverter ARM A316 and DSP D316 firmware | Matching C/assembly decompilation | v1.0.0; byte-identical rebuild claims; not tested as firmware to flash |
| `pw` | Pokéwalker firmware | Matching C decompilation | Maintainer says reconstruction is complete and can match the retail hash |
| StarStruck | Nintendo Wii IOS/Starlet | System-software reimplementation | Incomplete low-level replacement, based on Team Twiizers’ mini |
| Mufus | Selected Rufus 4.15 USB-creation workflows | Native macOS reimplementation informed by executable analysis | v1.0.1 early release; physical USB boot tests remain outstanding |
| oxpinyin | libpinyin 15-series C ABI and behavior | Rust reimplementation | Stage 1 corpus parity reported; full ABI differential coverage remains incomplete |
| Grit | Git engine semantics and repository formats | Rust reimplementation | v0.5.10; modern client and library support a documented subset |
| openfirma | AutoFirma signing-client behavior | Rust reimplementation | Phase 1; selected signature formats crosschecked, desktop keystore gaps remain |
| Spacebar | Discord.com backend | Backend/protocol reimplementation | Development status; some clients remain incompatible |
| konica-native-musl | bizhub 206 `245igdirf` CUPS filter | C reimplementation from disassembly and captures | 60-case differential claim plus documented print tests; license is undeclared |
| exiftool-rs | ExifTool 13.59 | Rust reimplementation | v0.10.0; reported read parity, writer parity remains incomplete |
| ReticulumKit | Reticulum networking protocol | Swift protocol-stack implementation | Alpha, wire-tested against Python RNS; no release listed |
| Maxima | Minima Maxima communication layer | Java protocol-stack reimplementation | Live protocol checks reported; Android device behavior remains unverified |

The repository’s GitHub `created_at` is recorded as repository metadata, not treated as the project’s start date. All proposed `project_started` fields remain null because the reviewed sources did not provide a reliable project start date. API language totals cover all tracked repository content and may include tests, documentation, and tooling.

The catalog already contains a Pokéwalker emulator core. The proposed `pw` row targets a separate output: the retail firmware source reconstruction and binary hash check. This relationship is recorded in the deduplication audit for reviewer judgment.
