# Emulator coverage audit

Verified for the 2026-10-15 discovery round. This lane reviewed the catalog’s 177 existing `Emulator` rows and staged eight additions plus 42 metadata refreshes. The SQLite catalog and all stored project/source URLs were read-only during this work.

## New coverage

| Addition | Platform coverage | User-facing scope and limits |
|---|---|---|
| EKA2L1 | Symbian OS / Nokia N-Gage | Most S60v1–Symbian Belle games and almost all official N-Gage libraries; application support is limited. Part-time maintenance. |
| FreeJ2ME | Java ME / J2ME | MIDlet JARs; project lists AWT, SDL2, and Libretro frontends. No complete compatibility list or versioned release. |
| CEmu | TI-84 Plus CE / TI-83 Premium CE | Stable and nightly desktop builds; a user-dumped calculator ROM is required. C core and Qt/C++ UI. |
| Firebird Emu | TI-Nspire Touchpad, CX, CX II | Android/iOS/desktop coverage; CX II boot ROM gaps can limit startup. Latest numbered release is older than current source activity. |
| Previous | NeXTcube / NeXTstation models | Current macOS/Windows and Linux builds; documented NEXTSTEP/OPENSTEP and selected BSD/Plan 9 guests; some devices remain unsupported. |
| QUASI88 | NEC PC-8801mkIISR and later | SDL2 port for macOS/Windows/Linux/Raspberry Pi, with Z80, display, audio, disk/tape, and peripheral emulation. |
| JNEXT | ZX Spectrum Next Issue 2 and classic Spectrum modes | Developer-focused debugger and headless tools; unfinished and explicitly not aimed at extreme cycle exactness. |
| Einstein | Apple Newton MessagePad / eMate family | macOS/iOS/Ubuntu supported; Android/Pi/Windows partial. User-provided Newton ROM required. |

The candidates are distinct maintained or community-maintained open-source implementations, not frontends, wrappers, or proposed forks. Exact repository roots and IDs were checked against the 847-row catalog’s project URLs and all saved source URLs; no exact root or ID collisions were found. The new rows do not include copyrighted firmware or game images.

## Existing-record audit

The 42 updates cover current Sony/Nintendo/Sega emulators; handheld and multi-system emulators; arcade emulators; and computer/PC emulation, including NeXT-adjacent and Atari/Commodore/Dragon systems already present in the catalog. Each update carries upstream documentation/repository sources, a release or activity date no later than the 2026-10-15 cutoff, and a compatibility-facing maintenance statement. Generic PlayStation software labels were made system-specific for DuckStation, PCSX2, RPCS3, PPSSPP, and Vita3K. Atari800’s newer upstream release version was reflected in both the general Atari 8-bit row and its Atari 5200 specialization.

Where no catalog field can express frontend/core packaging, the update’s `evidence_note` records whether the upstream documents standalone builds, a Libretro core, multiple bundled cores, browser builds, or a distinct frontend. The same notes preserve compatibility distinctions such as bootable versus playable states, machine-specific support, BIOS/ROM requirements, and experimental subsystems. They are audit annotations, not imported project fields. Programming-language values were checked against upstream repo language/build documentation and retained from the existing rows.

## Scope and limits

The existing catalog already has broad console, handheld, arcade, and classic-computer coverage, so this was a gap-focused round rather than an exhaustive census. The eight additions extend mobile/PDA, calculator, NeXT workstation, Japanese home-computer, and modern niche home-computer coverage. Compatibility is project-reported and changes by title, firmware, machine model, and host; percentages and release labels are snapshots rather than universal guarantees.

No database import, project documentation edit, or build was performed.
