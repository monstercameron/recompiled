# Mobile-port and Android compatibility research — 2026-10-15

This pass looked for current console/PC-to-phone ports, static recompilations, decompilation-based Android builds, and Android front ends that run multi-title PC game libraries through translation layers. Community/repo-search leads were treated only as pointers; records below were checked against each project's own README, release page, and relevant build or data documentation.

## Staged additions

`new-projects.json` contains six verified, previously uncataloged targets:

- **Need for Speed III: Hot Pursuit (1998 PC release) → Android** — `ZloiKILLER/nfs3recompiled-android` documents a static recompilation to portable C++, Android 8+, ARM64/arm64-v8a, GLES 3.0, and a user import from the original retail-disc data. The latest release found is v0.77 with an APK. Its release notes also describe several systems as now using native code instead of the earlier recompiled implementations, so the method text retains both parts of the project's current history.
- **System Shock → Android ARM64** — `raposomiguel50/system-shock-android` is a Shockolate-based source port requiring Android 13+ and arm64-v8a. Stable v1.0.0 has a complete playthrough accepted on a Retroid Pocket 5; the data guide allows compatible original CD-ROM or Enhanced Edition resources and says the upstream baseline does not support floppy data.
- **FTL: Faster Than Light → Android** — `adamkulik/pocket-wormhole` ports the clean-room Project Wormhole engine to Android. Its latest release found is v0.2.7. Users must supply `ftl.dat` from their Steam/GOG PC installation. README documents Android 8+/API 26 and GLES 3.0; it does not state APK ABI or a full-game completion test.
- **Micropolis → Android** — `ds17f/andropolis` builds the GPL MicropolisCore C++ simulation with the Android NDK and connects it through a C ABI/JNI. Latest release found is v0.1.7; build configuration specifies API 24+, arm64-v8a, and x86_64. The README describes the original scenarios and sample cities and states Play Store/F-Droid releases are planned. This is the open Micropolis release, not a port requiring commercial SimCity files.
- **GameNative** — Android storefront/launch front end for multiple owned Windows PC games. The v1.3.0 release provides standard and legacy-XR APKs; the README warns that compatibility is early and title dependent. The Gradle file lists API 26 as the minimum and ARM64 plus legacy ARM32 variants. The maintainer's third-party notice documents the Wine/Proton, Box64/FEX, and DXVK/VKD3D/Mesa stack and identifies some source-withheld helper binaries.
- **DroidDeck** — Android SteamOS-style handheld front end using Valve's ARM64 Proton for Windows games. The latest release found is 0.4.0. The README requires Android 9+ and an Adreno 730-or-newer or 8xx device; Adreno 6xx is experimental, while several other GPU families are unsupported. Store logins/game entitlements and compatible hardware are required.

## Deduplication

Checked the SQLite catalog, generated `docs/catalog.json`, central reviewed project additions/updates, and all other JSON files under `data/research/` by exact repository URL, ID, game, and target. None of the six exact repositories/targets was already present, so `metadata-updates.json` is empty. Existing `nfs3-vita-recomp` covers Need for Speed III on PlayStation Vita from the same PC source game but is a separate repository and target platform; it was not merged with the Android project.

## Leads excluded from mobile-port additions

- `elkir0/sc2-android-recomp` is tooling/method material and does not identify a game-specific Android build.
- `Jstar269/nakagawa-recomp` targets desktop Windows with SDL3/Vulkan; the source game is PSP, but this repository does not provide a phone target.
- `Simeony2007/VibeRecomp-for-GoW-CoO` is an experimental PSP recompilation project for ARM64 handhelds such as R36S, not an Android/iOS build.
- `MaryNCRT/Ultimate-Mortal-Kombat-3-iOS-Recomp` uses the old iOS game as its source and targets desktop Windows/Linux; it is not a mobile destination.
- `OpenNative` is a fork of GameNative, so it is not staged as another app record. General-purpose PC compatibility apps were included only where the project itself has an Android front end and current release evidence.

No database, generated documentation, central reviewed manifest, or import was changed. Only the three requested staging files in this directory were written.
