# Emulator gap discovery — 2026-10-15

This lane checked the current SQLite catalog and prior staged additions before proposing records. It found 178 existing rows with kind `Emulator`; none of the 11 new project IDs or canonical project roots in this batch already occur in SQLite or prior staged `new-projects.json` batches. The additions fill several identifiable system and host-coverage gaps: IBM mainframe, TI-99/4A, Acorn Archimedes/Risc PC, SHARP X68000, RCA CDP1802, and Android-hosted PC-98/DOS software.

## Additions

| Project | Gap covered | Evidence-backed scope/status note |
|---|---|---|
| Hercules / Spinhawk | IBM System/370, ESA/390, z/Architecture | The production 3.xx Spinhawk stream has current source activity; distinct from the older Hyperion development line. QPL terms are identified by the official site. |
| Classic99 | TI-99/4A | Windows emulator with current repository activity. Its README explicitly warns that its source/package license is restrictive and that bundled TI ROM terms apply. |
| Swift 99/a | TI-99/4A, native macOS | Independent Swift implementation with chip-level hardware scope and a BSD-2-Clause source license; early-stage gaps and no tagged releases are stated. |
| Arculator | Acorn Archimedes | Historical C emulator with source and 0.99 documentation. Source tree does not declare a license; last observed repository commit is 2015-01-18. |
| RPCEmu Extended | Acorn Risc PC / A7000 | Current GPL-2.0 fork with current releases, broad desktop hosts, and explicit notes separating experimental models. |
| PX68K Libretro Core | SHARP X68000 | Public GPL-2.0 Libretro emulator core; users supply system BIOS and disk images. |
| MPX68K | SHARP X68000 on macOS | Current native macOS emulator application based on px68k; mixed components retain separate attribution/license terms. |
| WebX68k | SHARP X68000 in browsers | Current px68k/WASM-based browser emulator player with a live project build. Listed separately because it is a usable browser-delivered app and host path, not because it is a separate hardware core. |
| RCA Studio II/III Emulator | RCA Studio II/III, COSMAC VIP, FRED | Public Processing/Java application covering several RCA CDP1802-derived systems; repository has no declared SPDX license. |
| Kairo98 | PC-98 on Android ARM64 | GPL-2.0-or-later Android application integrating Neko Project 21/W and ymfm; v1.1.1 was released on 2026-10-15. Users supply required ROMs. |
| KairoDos | DOS on Android ARM64 | GPL-2.0-or-later Android application integrating DOSBox Staging with ARM64 dynamic recompilation; v1.1.1 was released on 2026-10-15. It is not an official DOSBox Staging release. |

`new-projects.json` includes target system/software, host platforms where evidenced, implementation approach, maturity/source status, languages, discovery date, and 27 primary project/documentation/release sources. Release/activity dates are snapshots checked on 2026-10-15 in the project’s local timezone. No ROMs, BIOS files, game binaries, or media were copied into the repository.

## Existing-record gaps

The current emulator table has 145/178 rows with blank `subtype`, 15 with blank `programming_language`, and 150 with blank `updated` dates. `metadata-updates.json` adds source-backed subtype labels to ten rows, leaving the rest for broader source audits. `metadata-gap-audit.json` lists every existing row missing any of those three fields, with its current source links, to guide future refresh work. These are database observations, not claims that every missing date or language can be established; dates/languages should remain blank when primary evidence does not support them.

## Classification and exclusions

System emulators are `target_type: Software`. The X68000 family has three entries because PX68K is the Libretro core, MPX68K is a native macOS app, and WebX68k is a browser/WASM delivery with a separate app surface; the latter two explicitly disclose core reuse. Kairo98 and KairoDos are likewise platform-specific emulator applications built on existing emulator cores, rather than new cores. They are useful records for the user's requested Android/mobile host coverage and are distinguished from the upstream projects in their descriptions.

General-purpose frontends and aggregators such as RetroArch/OpenEmu/EmuDeck, core managers, ordinary VM frontends, and repository search results with no verifiable emulator implementation were not added as emulators. No hardware images or ROM packs were imported. Arculator and RCA Studio II/III are retained as source-visible historical/community projects despite missing license declarations; their records explicitly warn about that limitation.

No SQLite database, generated docs, or build files were edited or imported by this lane.
