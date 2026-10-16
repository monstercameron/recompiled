# Lane 12 — application and system reconstruction research

Checked 2026-10-15. Candidates were discovered through GitHub, project-owned sites, and a reconstruction tracker used only as a lead list. Each staged claim is linked to a maintainer repository or first-party project page. The live SQLite catalog and same-day reviewed research batches were checked before staging.

## Staged candidates

| Candidate | Target and classification | Status caveat |
|---|---|---|
| FreeDOS | MS-DOS/PC DOS-compatible OS; Reimplementation | Stable distribution is FreeDOS 1.4; T2610 is a separate October 2026 test distribution. The kernel README's 2.45/2045 is a kernel component version. The kernel derives from DOS-C; the whole distribution is not Microsoft source. |
| osFree | OS/2 system and user-mode components; Reimplementation | Official status remains deep alpha; no complete OS/2 replacement or exact compatibility level is claimed. |
| Advanced Mac Substitute | Classic 68K Mac runtime; Emulator | Combines CPU emulation and a Mac OS API replacement without Apple ROM/system software. The official site demonstrates several apps but provides no exact System Software/app compatibility matrix. |
| dmusic | Microsoft DirectMusic middleware; Reimplementation | Incomplete, unstable, no release; maintainer reports only partial (“somewhat correctly”) testing for Gothic and Gothic II soundtracks. Not a drop-in DLL claim. |
| Windows 7 DWM | `dwm.exe` and `uDWM.dll`; Decompilation | Exact Windows build not stated; no release or successful rebuild documented. README limits use, and latest commit message says the tree will not compile. No license file found. |
| OpenWebView2Loader | WebView2Loader library; Reimplementation | Feature parity is the author's goal, not verified behavior. No release or SDK/runtime version matrix; latest push checked was 2022-08-25. |
| wibo | Win32 console runtime; API Translation | Version 1.2.0. Limited to simple CLI x86 executables; x86-64 Linux and macOS builds are experimental. Selected WinAPI fixtures are tested, not full compatibility. |
| Mono legacy upstream | ECMA CLI / .NET runtime; Reimplementation | Latest legacy tag 6.12.0.206 (2024-02-13); official README records a WineHQ stewardship handoff and says Microsoft recommends migrating active users to .NET. |

## Existing-record update

AROS already exists as `aros-research-operating-system`, so it is not proposed as a duplicate. Its existing platform metadata was broad; official July 2026 project highlights now explicitly document a 64-bit AArch64 Raspberry Pi port and an Apple Silicon hosted `darwin-aarch64` target. The patch adds those targets while retaining the official qualification that some ports are incomplete or unavailable as downloads.

## Dedupe and exclusions

- System 7 (`Kelsidavis/System7`) is already staged by lane 35 under `35-longtail/new-projects.json`; do not duplicate it here.
- ReactOS, Haiku, AROS, and osFree Janus's Windows 3.0 subsystem already have catalog records. Only the distinct whole-project OS/2 osFree clone is staged. WineVDM is already staged by lane 34 at `34-archives/new-projects.json`; its v0.9.0 release page documents many app-specific compatibility issues, and the README says DOS emulation is incomplete.
- `ChenPi11/cmd` is a faithful command-language reimplementation, but the README says it is a Unix shell and does not support Windows. It does not preserve or run the original `cmd.exe` binary, so it was excluded from this binary/runtime-compatibility batch.
- `jceel/libxpc` has public C source but no README or declared license was found, the repository's last push was 2015-10-04, and no binary/app compatibility boundary is documented. Excluded as insufficient evidence for a maintained, attributable application-compatibility project.
- Recomp tracker listings for PhotoCraft, GridCraft, FilmCraft, WordCraft, VectorCraft, and similar workflow-alike editors are clean-room replacement applications rather than binary-compatible reconstruction or ports. Apache POI and LibreDWG are file-format libraries rather than reconstructions of Office or AutoCAD. They were not staged.
- `Pash` is an archived shell-language replacement; Butterchurn, projectM, and other preset/data-compatible visualizers do not reconstruct the original Windows application binary or its API; they were excluded on scope grounds.
- Game-specific examples and runtimes were not added under this lane. Their target titles belong in game-focused lanes.

## Validation

`new-projects.json` and `metadata-updates.json` are import-ready candidate batches only. They do not alter SQLite, generated catalog JSON, or documentation. The parent task should verify cross-lane IDs/URLs again at integration time, then decide which staged rows to import.
