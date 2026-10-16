# Round 13 software / firmware research

Verified on 2026-10-16 against primary GitHub sources and official project documentation. The canonical catalog was re-read at 3,450 projects, including 1,060 Software rows. All proposals below are non-game targets.

## Proposed records

- [Spacebar Server](https://github.com/spacebarchat/server) is a self-hosted Discord backend reimplementation. Its README documents the HTTP API, WebSocket gateway, CDN, WebRTC, and compatibility limits. Official deployment docs cover bare metal, NixOS, and Docker and advise against Windows. Package metadata and COPYING support AGPL-3.0-only. GitHub API reports the last source push on 2026-10-16, no releases, and exact language bytes in `github-byte-language-evidence.json`.
- [Asahi Neural Engine Linux Driver](https://github.com/eiln/ane) is a reverse-engineered Linux kernel module and user-space library for Apple Neural Engine hardware, with Python bindings. README says the documentation is WIP and gives no specific supported-device matrix. The root LICENSE is MIT; the source push date is 2024-03-12 and the releases API is empty.
- [mkmsbr](https://github.com/jma24/mkmsbr) is a clean-room Rust library/CLI and NASM boot-code implementation for Microsoft-compatible legacy BIOS boot records. Its README and provenance/spec docs describe its clean-room process, shipped scope, tests, and known limits; v1.0.1 is released under MIT. The README reports successful Windows 7 installation-media boot on Dell E6410 hardware, with NTFS PBR still experimental.

## Dedupe

The three proposals have no catalog ID, canonical GitHub root, or prior-staging match. Other researched projects already represented in SQLite were left out: FrankenRedis, Weave, Punic, the AT91RM9200 FlexStor bootloader, MuMDIA, FastTracker II clone, DirectMusic, and the CVD1810 MCU reimplementation. Exact row IDs and URLs are in `deduplication-audit.json`.

## Search and exclusions

Discovery queries and candidate lead URLs are recorded in `search-evidence.json`. The principal searches covered clean-room software/backend reimplementations, Apple Neural Engine reverse engineering, firmware bootloader replacements, application recompilation, and software source reconstruction.

Open Pocket was held out because the repository contains only README.md, has no declared license, and the Languages API returns no language buckets. LuxonServer is a Photon replacement for game clients, so it is outside this non-game tranche. ProjectorRays decompiles Shockwave movie content rather than the application. The remaining exclusions are tools or reverse-engineering notes rather than implementations of a target product.
