# macOS/Linux compatibility and translation research — 2026-10-15

This lane checks new compatibility projects against the live SQLite catalog and every staged JSON manifest under `data/research/discovery-2026-10-15/` before proposing rows. It adds four non-duplicate projects and stages three clarifying/status updates. It does not import data into SQLite or regenerate `docs/catalog.json`.

## New projects

- **mtld3d** is a direct Direct3D 9 graphics API implementation for Wine on macOS. It translates shader and render commands to Metal, and its maintainer publishes a title-by-title test list and explicit unsupported behavior. It is distinct from the existing D3DMetal/DXMT records, which cover D3D 10/11/12, and WineD3D/DXVK paths.
- **Hangover** is ARM64 Wine for Windows x86/x86-64 applications on ARM64 Linux. Wine supplies the Windows API layer; FEX and/or Box64 translate application instructions. This is an application compatibility stack, not a system emulator.
- **x87sidecar** hooks Rosetta 2 on Apple silicon and substitutes an AArch64 JIT for selected x87 instruction blocks. It is a narrow ISA accelerator, not a complete x86 translator or API layer. Windows use depends on the maintainer's patched Wine handshake. The README cautions that translator microbenchmarks do not predict whole-game speedups and documents arithmetic-precision limits.
- **Ferrum** is a proprietary Apple-silicon Windows compatibility engine in development. It is included as a clearly marked project because the official source describes a concrete FEX/Wine/graphics-translation architecture and early rendering milestones. It is not represented as usable: there is no public Ferrum release or source, and the maintainer explicitly says no full game has been run end to end.

## Existing-record updates

- Refreshes the latest verified release and repository-activity dates for Box64 and FEX, and labels both as process-level CPU/ISA translators rather than full-system emulators or Windows API layers.
- Reclassifies Rosetta 2 from API Translation to Emulator because Apple's own documentation describes x86-64-to-ARM64 instruction translation. The current schema has no dedicated ISA-translation project kind; the subtype explicitly says this is process-level translation, not full-system emulation. Wine, Hangover, and Proton remain API/compatibility-layer records.

## Research and deduplication

- Primary sources reviewed: project README/docs/releases and GitHub API repository/release metadata for [mtld3d](https://github.com/athei/mtld3d), [Hangover](https://github.com/AndreRH/hangover), [x87sidecar](https://github.com/athei/x87sidecar), [Ferrum](https://github.com/PyroSoftPro/Ferrum), [Box64](https://github.com/ptitSeb/box64), and [FEX](https://github.com/FEX-Emu/FEX). Apple’s official Rosetta documentation supports the taxonomy correction.
- Exact GitHub release evidence checked with `gh api`: mtld3d v0.13.0 (2026-10-13); Hangover 11.16 (2026-08-30); x87sidecar v1.9.0 (2026-10-13); Box64 v0.4.4 (2026-08-02); FEX-2610 (2026-10-07). Repository `pushed_at` dates were treated only as activity signals.
- Before staging, queried SQLite by project ID/title/URL and scanned all existing staged discovery JSON for the candidate names. No rows or staged batches were found for mtld3d, Hangover, x87sidecar, or Ferrum. The existing Mac lane already contains Hadron, Sikarugir, Whisky, Scotch, Cosmos, Mac Gaming Uncle, D3DMetal, DXMT, DXVK-macOS, Darling, MageVK, MoltenVK, and Metal2Vulkan; none were duplicated.
- Current project classification: Wine/Hangover/graphics backends are API or application compatibility layers; FEX, Box64, Rosetta, and x87sidecar handle CPU instruction-set translation. The database's `Emulator` kind is used for CPU/ISA translators but the `subtype` states clearly when they are process-level and not whole-system emulators.

## Exclusions and boundaries

- **WineShelf**, **Bourbon 26**, **Scotch**, **Cosmos**, **Sikarugir**, and **Whisky** are launchers, bottle managers, or compatibility-stack frontends rather than distinct API/ISA implementation layers. Several are already cataloged. Bourbon is a real user-facing app, but it is not duplicated here because this lane is scoped to translator/compatibility engines; Ferrum is separated and marked in-development.
- **WineGlass-Native** targets iOS; it was not included in this Mac/Linux lane and is a lead for the console/PC-to-mobile research lane.
- No whole-system VMs/emulators were added. Optional DXVK, vkd3d-proton, MoltenVK, Rosetta 2, Wine, FEX, and Box64 are identified as separate layers rather than conflated into one translation method.

## Files

- `new-projects.json`: importer-shaped records for four projects with source URLs, status, scope, architecture/API direction, language when supported by repository metadata, and first-found date.
- `metadata-updates.json`: three updates to existing project entries.
- `notes.md`: primary-source findings, taxonomy, duplicate audit, and exclusions.
