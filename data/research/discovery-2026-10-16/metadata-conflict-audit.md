# Reconciled metadata merge audit

**Prepared:** 2026-10-16
**Database:** `data/catalog.sqlite` (`projects` rows: 1290; `PRAGMA integrity_check`: `ok`).
**Scope:** read-only comparison of the four proposed metadata manifests against current SQLite.
**No import was run; no SQLite, docs, or tracked files were edited.**

## Merge totals

- Input rows: round1-18 **547**; lanes19-43 **45**; Xbox/PC lane44 **1**; metadata hydration lane45 **27**; **620** source rows total.
- Unique candidate project IDs: **592**. All exist in the current database.
- Duplicate IDs consolidated: **28**. There are 27 overlaps between round1-18 and lane45, plus Microsoft Bob between round1-18 and lanes19-43. Lane44 has no cross-manifest duplicate.
- Conflicting ID/field pairs: **25**.
- Importer-ready records after removing duplicate IDs and unchanged fields: **528**; unchanged proposed fields omitted: **378**.
- Import after review with: `python3 scripts/catalog.py import-project-updates work/research-next/metadata-merge-audit/metadata-updates.json`. This command has not been run.

## Duplicate IDs and field resolutions

“Current SQLite” is the exact pre-import value. Proposed and recommended values are kept verbatim. Evidence links are the primary-source references carried into the reconciled manifest.

### `acgc-android-port`

Proposal rows: `round1-18`, `hydration45`.

**`platform`** — current SQLite: `Nintendo GameCube → Android`.
- `hydration45` proposed `Nintendo GameCube → Android 8+ (ARM64 or ARMv7; OpenGL ES 3)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/zwaetschge/ACGC-Android-Port/blob/master/README.md)
- **Recommended:** `Nintendo GameCube → Android 8+ (ARM64 or ARMv7; OpenGL ES 3)`. Resolution: single proposal. Evidence: [Maintainer README: source and game-data distribution](https://github.com/zwaetschge/ACGC-Android-Port/blob/master/README.md)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/zwaetschge/ACGC-Android-Port)
- `hydration45` proposed `Public source repository and APK releases; no game data is included. Users supply their own supported GameCube disc image.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/zwaetschge/ACGC-Android-Port/blob/master/README.md)
- **Recommended:** `Public source repository and APK releases; no game data is included. Users supply their own supported GameCube disc image.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/zwaetschge/ACGC-Android-Port); [Maintainer README: source and game-data distribution](https://github.com/zwaetschge/ACGC-Android-Port/blob/master/README.md)

### `aleph-one-marathon`

Proposal rows: `round1-18`, `hydration45`.

**`programming_language`** — current SQLite: ``.
- `round1-18` proposed `C++`. Evidence: [Aleph One README: engine scope, supported titles, and scenario data](https://github.com/Aleph-One-Marathon/alephone); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Aleph-One-Marathon/alephone)
- **Recommended:** `C++`. Resolution: single proposal. Evidence: [Aleph One README: engine scope, supported titles, and scenario data](https://github.com/Aleph-One-Marathon/alephone); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Aleph-One-Marathon/alephone)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository; scenario data is required for full game content. GitHub repository metadata reports GPL-3.0 (GNU General Public License v3.0).`. Evidence: [Aleph One README: engine scope, supported titles, and scenario data](https://github.com/Aleph-One-Marathon/alephone); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Aleph-One-Marathon/alephone)
- `hydration45` proposed `GPL-3.0 source repository and source tarballs are public; game scenario/data files are distributed separately or supplied by the user.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/Aleph-One-Marathon/alephone/blob/master/README.md)
- **Recommended:** `GPL-3.0 source repository and source tarballs are public; game scenario/data files are distributed separately or supplied by the user.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [Aleph One README: engine scope, supported titles, and scenario data](https://github.com/Aleph-One-Marathon/alephone); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Aleph-One-Marathon/alephone); [Maintainer README: source and game-data distribution](https://github.com/Aleph-One-Marathon/alephone/blob/master/README.md)
**`subtype`** — current SQLite: `Multi-title game engine port`.
- `round1-18` proposed `Multi-title native engine continuation and source port`. Evidence: [Aleph One README: engine scope, supported titles, and scenario data](https://github.com/Aleph-One-Marathon/alephone); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Aleph-One-Marathon/alephone)
- **Recommended:** `Multi-title native engine continuation and source port`. Resolution: single proposal. Evidence: [Aleph One README: engine scope, supported titles, and scenario data](https://github.com/Aleph-One-Marathon/alephone); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Aleph-One-Marathon/alephone)

### `ballpad-super-mario-strikers-ios`

Proposal rows: `round1-18`, `hydration45`.

**`platform`** — current SQLite: `Nintendo GameCube → iOS/iPadOS`.
- `hydration45` proposed `Nintendo GameCube → iOS/iPadOS 17+ (native ARM64)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/ballpad/blob/main/README.md)
- **Recommended:** `Nintendo GameCube → iOS/iPadOS 17+ (native ARM64)`. Resolution: single proposal. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/ballpad/blob/main/README.md)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository (GPL-3.0 license)`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ballpad)
- `hydration45` proposed `Public source repository; no ready-to-install app is published. PadMint builds a local app from source; users supply their own supported disc image.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/ballpad/blob/main/README.md)
- **Recommended:** `Public GPL-3.0 source repository; no ready-to-install app is published. PadMint builds a local app from source; users supply their own supported disc image.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ballpad); [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/ballpad/blob/main/README.md)

### `basilisk-ii-classic-mac`

Proposal rows: `round1-18`, `hydration45`.

**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/cebix/macemu)
- `hydration45` proposed `GPL source is public in the shared macemu repository; users must separately supply a Mac OS copy and Macintosh ROM image.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/cebix/macemu/blob/master/README.md)
- **Recommended:** `GPL source is public in the shared macemu repository; users must separately supply a Mac OS copy and Macintosh ROM image.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/cebix/macemu); [Maintainer README: source and game-data distribution](https://github.com/cebix/macemu/blob/master/README.md)

### `bellpad-animal-crossing-ios`

Proposal rows: `round1-18`, `hydration45`.

**`platform`** — current SQLite: `Nintendo GameCube → iOS/iPadOS`.
- `hydration45` proposed `Nintendo GameCube → Apple Silicon macOS and iOS/iPadOS 17+ (ARM64)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/bellpad/blob/main/README.md)
- **Recommended:** `Nintendo GameCube → Apple Silicon macOS and iOS/iPadOS 17+ (ARM64)`. Resolution: single proposal. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/bellpad/blob/main/README.md)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/bellpad)
- `hydration45` proposed `Public source repository and reproducible build recipe; no store-distributed app. Users supply legally obtained supported game data.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/bellpad/blob/main/README.md)
- **Recommended:** `Public source repository and reproducible build recipe; no store-distributed app. Users supply legally obtained supported game data.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/bellpad); [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/bellpad/blob/main/README.md)

### `cavalry-language-switcher`

Proposal rows: `round1-18`, `hydration45`.

**`updated`** — current SQLite: `NULL`.
- `hydration45` proposed `2026-09-29`. Evidence: [Latest maintainer release, Cavalry 2.7.2 patch 12](https://github.com/daftAI2026/Cavalry-i18n/releases/tag/cavalry-2.7.2-p12)
- **Recommended:** `2026-09-29`. Resolution: single proposal. Evidence: [Latest maintainer release, Cavalry 2.7.2 patch 12](https://github.com/daftAI2026/Cavalry-i18n/releases/tag/cavalry-2.7.2-p12)

### `crossover`

Proposal rows: `round1-18`, `hydration45`.

**`source_available`** — current SQLite: `Product page`.
- `hydration45` proposed `Commercial product; CodeWeavers publishes a FOSS source bundle for CrossOver 26.3.0 and lists its included upstream components.`. Evidence: [CodeWeavers source-code page: downloadable FOSS bundle and component list](https://www.codeweavers.com/crossover/source)
- **Recommended:** `Commercial product; CodeWeavers publishes a FOSS source bundle for CrossOver 26.3.0 and lists its included upstream components.`. Resolution: single proposal. Evidence: [CodeWeavers source-code page: downloadable FOSS bundle and component list](https://www.codeweavers.com/crossover/source)

### `ectopad-metroid-prime-ios`

Proposal rows: `round1-18`, `hydration45`.

**`kind`** — current SQLite: `Reimplementation`.
- `round1-18` proposed `Decompilation`. Evidence: [EctoPad README: Metaforce/Aurora engine lineage, source build, and IPA status](https://github.com/chrissotraidis/ectopad); [Metaforce project README](https://github.com/AxioDL/metaforce); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ectopad)
- **Recommended:** `Decompilation`. Resolution: single proposal. Evidence: [EctoPad README: Metaforce/Aurora engine lineage, source build, and IPA status](https://github.com/chrissotraidis/ectopad); [Metaforce project README](https://github.com/AxioDL/metaforce); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ectopad)
**`platform`** — current SQLite: `Nintendo GameCube → iOS/iPadOS`.
- `hydration45` proposed `Nintendo GameCube → iOS/iPadOS 14+ and Apple Silicon macOS`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/ectopad/blob/main/README.md)
- **Recommended:** `Nintendo GameCube → iOS/iPadOS 14+ and Apple Silicon macOS`. Resolution: single proposal. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/ectopad/blob/main/README.md)
**`programming_language`** — current SQLite: ``.
- `round1-18` proposed `C++`. Evidence: [EctoPad README: Metaforce/Aurora engine lineage, source build, and IPA status](https://github.com/chrissotraidis/ectopad); [Metaforce project README](https://github.com/AxioDL/metaforce); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ectopad)
- **Recommended:** `C++`. Resolution: single proposal. Evidence: [EctoPad README: Metaforce/Aurora engine lineage, source build, and IPA status](https://github.com/chrissotraidis/ectopad); [Metaforce project README](https://github.com/AxioDL/metaforce); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ectopad)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository; Metaforce/PrimeDecomp lineage; user-supplied game data required; no public IPA documented`. Evidence: [EctoPad README: Metaforce/Aurora engine lineage, source build, and IPA status](https://github.com/chrissotraidis/ectopad); [Metaforce project README](https://github.com/AxioDL/metaforce); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ectopad)
- `hydration45` proposed `Public source/build instructions; no public IPA is currently available. Users supply their own supported Metroid Prime disc image.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/ectopad/blob/main/README.md)
- **Recommended:** `Public source/build instructions for the Metaforce/PrimeDecomp-based iOS integration; no public IPA is currently documented. Users supply their own supported Metroid Prime disc image.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [EctoPad README: Metaforce/Aurora engine lineage, source build, and IPA status](https://github.com/chrissotraidis/ectopad); [Metaforce project README](https://github.com/AxioDL/metaforce); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ectopad); [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/ectopad/blob/main/README.md)
**`strategy`** — current SQLite: `Native engine reimplementation with Aurora/Dawn rendering adapted to Metal`.
- `round1-18` proposed `iOS integration/port of the Metaforce engine built on Metroid Prime decompilation source; adapts Aurora/Dawn rendering to Metal.`. Evidence: [EctoPad README: Metaforce/Aurora engine lineage, source build, and IPA status](https://github.com/chrissotraidis/ectopad); [Metaforce project README](https://github.com/AxioDL/metaforce); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ectopad)
- **Recommended:** `iOS integration/port of the Metaforce engine built on Metroid Prime decompilation source; adapts Aurora/Dawn rendering to Metal.`. Resolution: single proposal. Evidence: [EctoPad README: Metaforce/Aurora engine lineage, source build, and IPA status](https://github.com/chrissotraidis/ectopad); [Metaforce project README](https://github.com/AxioDL/metaforce); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ectopad)
**`subtype`** — current SQLite: `Metaforce Apple mobile integration`.
- `round1-18` proposed `iOS integration of the decompilation-derived Metaforce engine`. Evidence: [EctoPad README: Metaforce/Aurora engine lineage, source build, and IPA status](https://github.com/chrissotraidis/ectopad); [Metaforce project README](https://github.com/AxioDL/metaforce); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ectopad)
- **Recommended:** `iOS integration of the decompilation-derived Metaforce engine`. Resolution: single proposal. Evidence: [EctoPad README: Metaforce/Aurora engine lineage, source build, and IPA status](https://github.com/chrissotraidis/ectopad); [Metaforce project README](https://github.com/AxioDL/metaforce); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ectopad)

### `fire-emblem-warriors-3ds-ko`

Proposal rows: `round1-18`, `hydration45`.

**`updated`** — current SQLite: `NULL`.
- `hydration45` proposed `2026-10-11`. Evidence: [Latest maintainer patch release, v0.2.3](https://github.com/arqhive/fire-emblem-warriors-korean-translation/releases/tag/v0.2.3)
- **Recommended:** `2026-10-11`. Resolution: single proposal. Evidence: [Latest maintainer patch release, v0.2.3](https://github.com/arqhive/fire-emblem-warriors-korean-translation/releases/tag/v0.2.3)

### `gta-vice-city-stories-android-recomp`

Proposal rows: `round1-18`, `hydration45`.

**`platform`** — current SQLite: `PlayStation Portable → Android (ARM)`.
- `round1-18` proposed `PlayStation Portable → Android (ARM64 and ARMv7), Windows`. Evidence: [Maintainer README: Android requirements, device, APK variants and known issues](https://github.com/codepdbh/PSPRecomp-VCS-Android); [Maintainer Android releases](https://github.com/codepdbh/PSPRecomp-VCS-Android/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/codepdbh/PSPRecomp-VCS-Android)
- **Recommended:** `PlayStation Portable → Android (ARM64 and ARMv7), Windows`. Resolution: single proposal. Evidence: [Maintainer README: Android requirements, device, APK variants and known issues](https://github.com/codepdbh/PSPRecomp-VCS-Android); [Maintainer Android releases](https://github.com/codepdbh/PSPRecomp-VCS-Android/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/codepdbh/PSPRecomp-VCS-Android)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository (MIT license)`. Evidence: [Maintainer README: Android requirements, device, APK variants and known issues](https://github.com/codepdbh/PSPRecomp-VCS-Android); [Maintainer Android releases](https://github.com/codepdbh/PSPRecomp-VCS-Android/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/codepdbh/PSPRecomp-VCS-Android)
- `hydration45` proposed `Public source repository and APK releases contain only the program; users must supply their own game files.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/codepdbh/PSPRecomp-VCS-Android/blob/main/README.md)
- **Recommended:** `Public MIT-licensed source repository and Android APK releases contain only the program; users must supply their own game files.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [Maintainer README: Android requirements, device, APK variants and known issues](https://github.com/codepdbh/PSPRecomp-VCS-Android); [Maintainer Android releases](https://github.com/codepdbh/PSPRecomp-VCS-Android/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/codepdbh/PSPRecomp-VCS-Android); [Maintainer README: source and game-data distribution](https://github.com/codepdbh/PSPRecomp-VCS-Android/blob/main/README.md)
**`status`** — current SQLite: `Beta v0.3.0; maintainer reports gameplay on Galaxy S25 Ultra. Known limitations include freezes/black screens, skipped video intro, and high-resolution issues.`.
- `round1-18` proposed `Beta v0.3.0; Android 8+ with Vulkan is tested on a Galaxy S25 Ultra. The release offers both arm64 and 32-bit ARM APKs; the README lists skipped intro video and remaining freeze/black-screen and high-resolution issues.`. Evidence: [Maintainer README: Android requirements, device, APK variants and known issues](https://github.com/codepdbh/PSPRecomp-VCS-Android); [Maintainer Android releases](https://github.com/codepdbh/PSPRecomp-VCS-Android/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/codepdbh/PSPRecomp-VCS-Android)
- **Recommended:** `Beta v0.3.0; Android 8+ with Vulkan is tested on a Galaxy S25 Ultra. The release offers both arm64 and 32-bit ARM APKs; the README lists skipped intro video and remaining freeze/black-screen and high-resolution issues.`. Resolution: single proposal. Evidence: [Maintainer README: Android requirements, device, APK variants and known issues](https://github.com/codepdbh/PSPRecomp-VCS-Android); [Maintainer Android releases](https://github.com/codepdbh/PSPRecomp-VCS-Android/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/codepdbh/PSPRecomp-VCS-Android)
**`summary`** — current SQLite: `Android Vulkan app for a static PSP Allegrex-to-C++ recompilation, with touch controls. The maintainer reports a physical Galaxy S25 Ultra test and lists known rendering and stability limitations.`.
- `round1-18` proposed `Android Vulkan port of a static PSP Allegrex-to-C++ recompilation, with fullscreen play and save-state support. Requires the user's own PSP game data; APKs do not include commercial assets.`. Evidence: [Maintainer README: Android requirements, device, APK variants and known issues](https://github.com/codepdbh/PSPRecomp-VCS-Android); [Maintainer Android releases](https://github.com/codepdbh/PSPRecomp-VCS-Android/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/codepdbh/PSPRecomp-VCS-Android)
- **Recommended:** `Android Vulkan port of a static PSP Allegrex-to-C++ recompilation, with fullscreen play and save-state support. Requires the user's own PSP game data; APKs do not include commercial assets.`. Resolution: single proposal. Evidence: [Maintainer README: Android requirements, device, APK variants and known issues](https://github.com/codepdbh/PSPRecomp-VCS-Android); [Maintainer Android releases](https://github.com/codepdbh/PSPRecomp-VCS-Android/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/codepdbh/PSPRecomp-VCS-Android)

### `harvest-moon-64-android-recomp`

Proposal rows: `round1-18`, `hydration45`.

**`platform`** — current SQLite: `Nintendo 64 → Android`.
- `round1-18` proposed `Nintendo 64 → Android (arm64-v8a, Android 9/API 28+, Vulkan 1.1)`. Evidence: [Maintainer README: Android requirements, ROM import and touch/dual-screen UI](https://github.com/igawa6/HarvestMoon64Recomp); [Maintainer Android releases](https://github.com/igawa6/HarvestMoon64Recomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/igawa6/HarvestMoon64Recomp)
- `hydration45` proposed `Nintendo 64 → Android 9+ (arm64-v8a)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/igawa6/HarvestMoon64Recomp/blob/main/README.md)
- **Recommended:** `Nintendo 64 → Android 9+ (arm64-v8a; Vulkan 1.1)`. Resolution: merge Android minimum/ABI and documented Vulkan 1.1 requirement. Evidence: [Maintainer README: Android requirements, ROM import and touch/dual-screen UI](https://github.com/igawa6/HarvestMoon64Recomp); [Maintainer Android releases](https://github.com/igawa6/HarvestMoon64Recomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/igawa6/HarvestMoon64Recomp); [Maintainer README: source and game-data distribution](https://github.com/igawa6/HarvestMoon64Recomp/blob/main/README.md)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository (GPL-3.0 license)`. Evidence: [Maintainer README: Android requirements, ROM import and touch/dual-screen UI](https://github.com/igawa6/HarvestMoon64Recomp); [Maintainer Android releases](https://github.com/igawa6/HarvestMoon64Recomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/igawa6/HarvestMoon64Recomp)
- `hydration45` proposed `Public source repository and Android APK releases; game assets are not included and users must supply an NTSC-U ROM.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/igawa6/HarvestMoon64Recomp/blob/main/README.md)
- **Recommended:** `Public GPL-3.0 source repository and Android APK releases; game assets are not included and users must supply an NTSC-U ROM.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [Maintainer README: Android requirements, ROM import and touch/dual-screen UI](https://github.com/igawa6/HarvestMoon64Recomp); [Maintainer Android releases](https://github.com/igawa6/HarvestMoon64Recomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/igawa6/HarvestMoon64Recomp); [Maintainer README: source and game-data distribution](https://github.com/igawa6/HarvestMoon64Recomp/blob/main/README.md)
**`status`** — current SQLite: `Android APK v0.2.0 is released; README documents Android 9+ ARM64 and touch support, but no physical-device gameplay test.`.
- `round1-18` proposed `APK releases are available; the README specifies Android 9/API 28+, arm64-v8a and Vulkan 1.1. It documents touch controls and dual-screen companion UI, but does not name a physical test device.`. Evidence: [Maintainer README: Android requirements, ROM import and touch/dual-screen UI](https://github.com/igawa6/HarvestMoon64Recomp); [Maintainer Android releases](https://github.com/igawa6/HarvestMoon64Recomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/igawa6/HarvestMoon64Recomp)
- **Recommended:** `APK releases are available; the README specifies Android 9/API 28+, arm64-v8a and Vulkan 1.1. It documents touch controls and dual-screen companion UI, but does not name a physical test device.`. Resolution: single proposal. Evidence: [Maintainer README: Android requirements, ROM import and touch/dual-screen UI](https://github.com/igawa6/HarvestMoon64Recomp); [Maintainer Android releases](https://github.com/igawa6/HarvestMoon64Recomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/igawa6/HarvestMoon64Recomp)
**`summary`** — current SQLite: `Android build of a static N64 recompilation using N64: Recompiled. The README documents an Android APK, touch controls, and file selection; physical-device playability is not stated.`.
- `round1-18` proposed `Android build of the Harvest Moon 64 static recompilation with an N64 touch overlay and optional second-screen dashboard for dual-display handhelds. It requires the user's NTSC-U ROM; game assets are not distributed.`. Evidence: [Maintainer README: Android requirements, ROM import and touch/dual-screen UI](https://github.com/igawa6/HarvestMoon64Recomp); [Maintainer Android releases](https://github.com/igawa6/HarvestMoon64Recomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/igawa6/HarvestMoon64Recomp)
- **Recommended:** `Android build of the Harvest Moon 64 static recompilation with an N64 touch overlay and optional second-screen dashboard for dual-display handhelds. It requires the user's NTSC-U ROM; game assets are not distributed.`. Resolution: single proposal. Evidence: [Maintainer README: Android requirements, ROM import and touch/dual-screen UI](https://github.com/igawa6/HarvestMoon64Recomp); [Maintainer Android releases](https://github.com/igawa6/HarvestMoon64Recomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/igawa6/HarvestMoon64Recomp)

### `meleepad-melee-ios`

Proposal rows: `round1-18`, `hydration45`.

**`platform`** — current SQLite: `Nintendo GameCube → iOS/iPadOS`.
- `hydration45` proposed `Nintendo GameCube → iOS/iPadOS 16+ and Apple Silicon macOS 14+`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/meleepad/blob/main/README.md)
- **Recommended:** `Nintendo GameCube → iOS/iPadOS 16+ and Apple Silicon macOS 14+`. Resolution: single proposal. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/meleepad/blob/main/README.md)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository (GPL-3.0 license)`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/meleepad)
- `hydration45` proposed `Public source repository and module-free app shell; PadMint builds the game module from the user’s own supported disc. Generated game code is not distributed.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/meleepad/blob/main/README.md)
- **Recommended:** `Public GPL-3.0 source repository and module-free app shell; PadMint builds the game module from the user’s own supported disc. Generated game code is not distributed.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/meleepad); [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/meleepad/blob/main/README.md)

### `metroid-prime-hunters-android-recomp`

Proposal rows: `round1-18`, `hydration45`.

**`platform`** — current SQLite: `Nintendo DS → Android (AYN Thor dual-screen)`.
- `hydration45` proposed `Nintendo DS → Android 11+ (arm64; dual-screen; AYN Thor tested)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/aabrole/HuntersRecomp/blob/main/README.md)
- **Recommended:** `Nintendo DS → Android 11+ (arm64; dual-screen; AYN Thor tested)`. Resolution: single proposal. Evidence: [Maintainer README: source and game-data distribution](https://github.com/aabrole/HuntersRecomp/blob/main/README.md)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository (GPL-3.0 license)`. Evidence: [Maintainer README: Android device limits, input and ROM import](https://github.com/aabrole/HuntersRecomp); [Maintainer Android releases](https://github.com/aabrole/HuntersRecomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/aabrole/HuntersRecomp)
- `hydration45` proposed `Public source repository and APK releases; no game data is included. Users supply a verified USA revision 0 cartridge dump.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/aabrole/HuntersRecomp/blob/main/README.md)
- **Recommended:** `Public GPL-3.0 source repository and APK releases; no game data is included. Users supply a verified USA revision 0 cartridge dump.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [Maintainer README: Android device limits, input and ROM import](https://github.com/aabrole/HuntersRecomp); [Maintainer Android releases](https://github.com/aabrole/HuntersRecomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/aabrole/HuntersRecomp); [Maintainer README: source and game-data distribution](https://github.com/aabrole/HuntersRecomp/blob/main/README.md)
**`status`** — current SQLite: `v0.2.5 release; maintainer documents testing on AYN Thor with dual-screen/touch operation and approximately 58–59 FPS. Other devices are not established.`.
- `round1-18` proposed `v0.2.5 Android APK release. The only documented physical device is the AYN Thor (Android 13, dual-screen ARM64), where the maintainer reports about 58–59 fps and second-screen stylus/touch operation. Other-device support is not established.`. Evidence: [Maintainer README: Android device limits, input and ROM import](https://github.com/aabrole/HuntersRecomp); [Maintainer Android releases](https://github.com/aabrole/HuntersRecomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/aabrole/HuntersRecomp)
- **Recommended:** `v0.2.5 Android APK release. The only documented physical device is the AYN Thor (Android 13, dual-screen ARM64), where the maintainer reports about 58–59 fps and second-screen stylus/touch operation. Other-device support is not established.`. Resolution: single proposal. Evidence: [Maintainer README: Android device limits, input and ROM import](https://github.com/aabrole/HuntersRecomp); [Maintainer Android releases](https://github.com/aabrole/HuntersRecomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/aabrole/HuntersRecomp)
**`summary`** — current SQLite: `Android build of the Metroid Prime Hunters static recompilation, integrating the native ARM64 output with a dual-screen Android frontend and touch controls. The maintainer's named test target is the AYN Thor.`.
- `round1-18` proposed `Nintendo DS code is statically recompiled to native ARM64 for Android and packaged for dual-screen devices. Requires the user's own USA AMHE-0 cartridge dump; the app verifies/imports it and uses the lower touchscreen as the DS stylus screen.`. Evidence: [Maintainer README: Android device limits, input and ROM import](https://github.com/aabrole/HuntersRecomp); [Maintainer Android releases](https://github.com/aabrole/HuntersRecomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/aabrole/HuntersRecomp)
- **Recommended:** `Nintendo DS code is statically recompiled to native ARM64 for Android and packaged for dual-screen devices. Requires the user's own USA AMHE-0 cartridge dump; the app verifies/imports it and uses the lower touchscreen as the DS stylus screen.`. Resolution: single proposal. Evidence: [Maintainer README: Android device limits, input and ROM import](https://github.com/aabrole/HuntersRecomp); [Maintainer Android releases](https://github.com/aabrole/HuntersRecomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/aabrole/HuntersRecomp)

### `microsoft-bob-recomp`

Proposal rows: `round1-18`, `lanes19-43`.

**`platform`** — current SQLite: `Windows 3.1/95 Win16 → portable C host (bring-up)`.
- `lanes19-43` proposed `Microsoft Bob 1995 Win16 NE binaries → portable C recompilation hosted by the project's cross-platform runtime; project README targets Windows, macOS, and Linux.`. Evidence: [Microsoft Bob maintainer README: platform, six-module lift, current startup/database milestones, blocker, build requirements, and license boundary](https://github.com/sp00nznet/bob); [Microsoft Bob repository license: MIT applies to repository-owned code, not original binaries or generated lifted C](https://github.com/sp00nznet/bob/blob/master/LICENSE)
- **Recommended:** `Microsoft Bob 1995 Win16 NE binaries → portable C recompilation hosted by the project's cross-platform runtime; project README targets Windows, macOS, and Linux.`. Resolution: single proposal. Evidence: [Microsoft Bob maintainer README: platform, six-module lift, current startup/database milestones, blocker, build requirements, and license boundary](https://github.com/sp00nznet/bob); [Microsoft Bob repository license: MIT applies to repository-owned code, not original binaries or generated lifted C](https://github.com/sp00nznet/bob/blob/master/LICENSE)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/sp00nznet/bob)
- `lanes19-43` proposed `Public MIT-licensed tooling, runtime, hand-written code, and analysis metadata. Original Bob binaries/data and generated lifted C are not included and are not covered by the repository MIT license; users must provide their own disc files.`. Evidence: [Microsoft Bob maintainer README: platform, six-module lift, current startup/database milestones, blocker, build requirements, and license boundary](https://github.com/sp00nznet/bob); [Microsoft Bob repository license: MIT applies to repository-owned code, not original binaries or generated lifted C](https://github.com/sp00nznet/bob/blob/master/LICENSE)
- **Recommended:** `Public MIT-licensed tooling, runtime, hand-written code, and analysis metadata. Original Bob binaries/data and generated lifted C are not included and are not covered by the repository MIT license; users must provide their own disc files.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/sp00nznet/bob); [Microsoft Bob maintainer README: platform, six-module lift, current startup/database milestones, blocker, build requirements, and license boundary](https://github.com/sp00nznet/bob); [Microsoft Bob repository license: MIT applies to repository-owned code, not original binaries or generated lifted C](https://github.com/sp00nznet/bob/blob/master/LICENSE)
**`status`** — current SQLite: `Bring-up stage. The project reports that the original startup path and Jet databases execute in its host; a first rendered application frame is not yet documented.`.
- `lanes19-43` proposed `Bring-up; the maintainer reports that the six Bob and Jet/Access Basic modules lift to link-clean C, Bob's real startup and MFC window creation execute, and Jet opens the original databases. The current blocker is a host longjmp/stack fault before the first rendered frame; no GUI frame or usable application release is claimed.`. Evidence: [Microsoft Bob maintainer README: platform, six-module lift, current startup/database milestones, blocker, build requirements, and license boundary](https://github.com/sp00nznet/bob); [Microsoft Bob repository license: MIT applies to repository-owned code, not original binaries or generated lifted C](https://github.com/sp00nznet/bob/blob/master/LICENSE)
- **Recommended:** `Bring-up; the maintainer reports that the six Bob and Jet/Access Basic modules lift to link-clean C, Bob's real startup and MFC window creation execute, and Jet opens the original databases. The current blocker is a host longjmp/stack fault before the first rendered frame; no GUI frame or usable application release is claimed.`. Resolution: single proposal. Evidence: [Microsoft Bob maintainer README: platform, six-module lift, current startup/database milestones, blocker, build requirements, and license boundary](https://github.com/sp00nznet/bob); [Microsoft Bob repository license: MIT applies to repository-owned code, not original binaries or generated lifted C](https://github.com/sp00nznet/bob/blob/master/LICENSE)
**`summary`** — current SQLite: `Lifts Microsoft Bob's Win16 NE application and database-related modules into C, with replacement runtime work for Win16, Jet, and WinG dependencies. Requires a user-owned copy of the original application.`.
- `lanes19-43` proposed `Static recompilation of the original Win16 Bob house application, its engine, blitter, and load-bearing Jet/Access Basic modules into C. The host/runtime reconstructs Win16, Jet, and WinG behavior; the project has reached real application initialization and database opens but has not yet drawn the Bob house. Original binaries, data, and lifted C are user-generated and are not distributed.`. Evidence: [Microsoft Bob maintainer README: platform, six-module lift, current startup/database milestones, blocker, build requirements, and license boundary](https://github.com/sp00nznet/bob); [Microsoft Bob repository license: MIT applies to repository-owned code, not original binaries or generated lifted C](https://github.com/sp00nznet/bob/blob/master/LICENSE)
- **Recommended:** `Static recompilation of the original Win16 Bob house application, its engine, blitter, and load-bearing Jet/Access Basic modules into C. The host/runtime reconstructs Win16, Jet, and WinG behavior; the project has reached real application initialization and database opens but has not yet drawn the Bob house. Original binaries, data, and lifted C are user-generated and are not distributed.`. Resolution: single proposal. Evidence: [Microsoft Bob maintainer README: platform, six-module lift, current startup/database milestones, blocker, build requirements, and license boundary](https://github.com/sp00nznet/bob); [Microsoft Bob repository license: MIT applies to repository-owned code, not original binaries or generated lifted C](https://github.com/sp00nznet/bob/blob/master/LICENSE)

### `mini-vmac`

Proposal rows: `round1-18`, `hydration45`.

**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository (GPL-2.0 license)`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/minivmac/minivmac)
- `hydration45` proposed `Public source repository with build scripts for the documented Macintosh models and host platforms.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/minivmac/minivmac/blob/master/README.md)
- **Recommended:** `Public GPL-2.0 source repository with build scripts for the documented Macintosh models and host platforms.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/minivmac/minivmac); [Maintainer README: source and game-data distribution](https://github.com/minivmac/minivmac/blob/master/README.md)

### `minish-cap-aleks-ultimate-nx`

Proposal rows: `round1-18`, `hydration45`.

**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/Alexgg1014/The-Legend-of-Zelda-The-Minish-Cap-Alek-s-Ultimate-NX-Edition)
- `hydration45` proposed `Public source repository and Switch NRO releases; no ROM or copyrighted game data is included. Users supply their own legally obtained USA GBA ROM.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/Alexgg1014/The-Legend-of-Zelda-The-Minish-Cap-Alek-s-Ultimate-NX-Edition/blob/main/README.md)
- **Recommended:** `Public source repository and Switch NRO releases; no ROM or copyrighted game data is included. Users supply their own legally obtained USA GBA ROM.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/Alexgg1014/The-Legend-of-Zelda-The-Minish-Cap-Alek-s-Ultimate-NX-Edition); [Maintainer README: source and game-data distribution](https://github.com/Alexgg1014/The-Legend-of-Zelda-The-Minish-Cap-Alek-s-Ultimate-NX-Edition/blob/main/README.md)

### `perfect-dark-android`

Proposal rows: `round1-18`, `hydration45`.

**`platform`** — current SQLite: `Nintendo 64 → Android`.
- `hydration45` proposed `Nintendo 64 → Android 5+ (arm64-v8a, armeabi-v7a, x86_64, x86)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/izzy2lost/perfect_dark/blob/port/README.md); [Maintainer Android README: Android 5/API 21+, supported ABIs, and user-supplied ROM](https://github.com/izzy2lost/perfect_dark/blob/android-features/README.md); [Maintainer Android 2.0.0 release](https://github.com/izzy2lost/perfect_dark/releases/tag/2.0.0)
- **Recommended:** `Nintendo 64 → Android 5+ (arm64-v8a, armeabi-v7a, x86_64, x86)`. Resolution: single proposal. Evidence: [Maintainer README: source and game-data distribution](https://github.com/izzy2lost/perfect_dark/blob/port/README.md); [Maintainer Android README: Android 5/API 21+, supported ABIs, and user-supplied ROM](https://github.com/izzy2lost/perfect_dark/blob/android-features/README.md); [Maintainer Android 2.0.0 release](https://github.com/izzy2lost/perfect_dark/releases/tag/2.0.0)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository (MIT license)`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/izzy2lost/perfect_dark)
- `hydration45` proposed `Public source repository and automatic builds; the user must supply a supported Perfect Dark ROM, which is not included.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/izzy2lost/perfect_dark/blob/port/README.md); [Maintainer Android README: Android 5/API 21+, supported ABIs, and user-supplied ROM](https://github.com/izzy2lost/perfect_dark/blob/android-features/README.md); [Maintainer Android 2.0.0 release](https://github.com/izzy2lost/perfect_dark/releases/tag/2.0.0)
- **Recommended:** `Public MIT-licensed source repository and automatic builds; the user must supply a supported Perfect Dark ROM, which is not included.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/izzy2lost/perfect_dark); [Maintainer README: source and game-data distribution](https://github.com/izzy2lost/perfect_dark/blob/port/README.md); [Maintainer Android README: Android 5/API 21+, supported ABIs, and user-supplied ROM](https://github.com/izzy2lost/perfect_dark/blob/android-features/README.md); [Maintainer Android 2.0.0 release](https://github.com/izzy2lost/perfect_dark/releases/tag/2.0.0)

### `pokemon-azoth-en`

Proposal rows: `round1-18`, `hydration45`.

**`updated`** — current SQLite: `NULL`.
- `hydration45` proposed `2026-10-13`. Evidence: [Latest maintainer patch release, beta-22](https://github.com/RiFa-Innovation/pkmn-azoth-translation/releases/tag/beta-22)
- **Recommended:** `2026-10-13`. Resolution: single proposal. Evidence: [Latest maintainer patch release, beta-22](https://github.com/RiFa-Innovation/pkmn-azoth-translation/releases/tag/beta-22)

### `projectreach-halo-xbox-ios`

Proposal rows: `round1-18`, `hydration45`.

**`platform`** — current SQLite: `Original Xbox → iOS/iPadOS`.
- `hydration45` proposed `Original Xbox → iOS/iPadOS 17.4+ and Apple Silicon macOS 14.4+`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/projectreach/blob/main/README.md)
- **Recommended:** `Original Xbox → iOS/iPadOS 17.4+ and Apple Silicon macOS 14.4+`. Resolution: single proposal. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/projectreach/blob/main/README.md)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/projectreach)
- `hydration45` proposed `Public source repository and build instructions; there is no app download. Users supply their own supported Xbox disc image.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/projectreach/blob/main/README.md)
- **Recommended:** `Public source repository and build instructions; there is no app download. Users supply their own supported Xbox disc image.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/projectreach); [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/projectreach/blob/main/README.md)

### `sacred-stones-android-recomp`

Proposal rows: `round1-18`, `hydration45`.

**`platform`** — current SQLite: `Game Boy Advance → Android (ARM64)`.
- `hydration45` proposed `Game Boy Advance → Windows x64 and Android 9+ (ARM64)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/Lordingard/SacredStonesRecomp/blob/main/README.md)
- **Recommended:** `Game Boy Advance → Windows x64 and Android 9+ (ARM64)`. Resolution: single proposal. Evidence: [Maintainer README: source and game-data distribution](https://github.com/Lordingard/SacredStonesRecomp/blob/main/README.md)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/Lordingard/SacredStonesRecomp)
- `hydration45` proposed `Public source repository and Windows/Android release packages; the game ROM, GBA BIOS, and generated ROM-derived code are not included.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/Lordingard/SacredStonesRecomp/blob/main/README.md)
- **Recommended:** `Public source repository and Windows/Android release packages; the game ROM, GBA BIOS, and generated ROM-derived code are not included.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/Lordingard/SacredStonesRecomp); [Maintainer README: source and game-data distribution](https://github.com/Lordingard/SacredStonesRecomp/blob/main/README.md)

### `sheepshaver-classic-mac`

Proposal rows: `round1-18`, `hydration45`.

**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/cebix/macemu)
- `hydration45` proposed `GPL source is public in the shared macemu repository; users must separately supply a Mac OS copy and PowerMac ROM image.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/cebix/macemu/blob/master/README.md)
- **Recommended:** `GPL source is public in the shared macemu repository; users must separately supply a Mac OS copy and PowerMac ROM image.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/cebix/macemu); [Maintainer README: source and game-data distribution](https://github.com/cebix/macemu/blob/master/README.md)

### `sm64-android-port-base`

Proposal rows: `round1-18`, `hydration45`.

**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/VDavid003/sm64-port-android-base)
- `hydration45` proposed `Public source and Android build instructions; a user-supplied Super Mario 64 baserom is required and is not included.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/VDavid003/sm64-port-android-base/blob/master/README.md)
- **Recommended:** `Public source and Android build instructions; a user-supplied Super Mario 64 baserom is required and is not included.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/VDavid003/sm64-port-android-base); [Maintainer README: source and game-data distribution](https://github.com/VDavid003/sm64-port-android-base/blob/master/README.md)

### `snappad-pokemon-snap-ios`

Proposal rows: `round1-18`, `hydration45`.

**`platform`** — current SQLite: `Nintendo 64 → iPadOS`.
- `hydration45` proposed `Nintendo 64 → iOS/iPadOS 15+ and Apple Silicon macOS`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/snappad/blob/main/README.md)
- **Recommended:** `Nintendo 64 → iOS/iPadOS 15+ and Apple Silicon macOS`. Resolution: single proposal. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/snappad/blob/main/README.md)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/snappad)
- `hydration45` proposed `Public integration source and build instructions; public playable downloads are paused. Users provide their own USA ROM; the repository excludes ROM-derived code and game assets.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/snappad/blob/main/README.md)
- **Recommended:** `Public integration source and build instructions; public playable downloads are paused. Users provide their own USA ROM; the repository excludes ROM-derived code and game assets.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/snappad); [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/snappad/blob/main/README.md)

### `spaghettikart-android`

Proposal rows: `round1-18`, `hydration45`.

**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/izzy2lost/SpaghettiKart)
- `hydration45` proposed `Public source and releases; no copyrighted game assets are included. Users provide a supported USA ROM.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/izzy2lost/SpaghettiKart/blob/main/README.md)
- **Recommended:** `Public source and releases; no copyrighted game assets are included. Users provide a supported USA ROM.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/izzy2lost/SpaghettiKart); [Maintainer README: source and game-data distribution](https://github.com/izzy2lost/SpaghettiKart/blob/main/README.md)

### `starship-android`

Proposal rows: `round1-18`, `hydration45`.

**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository (CC0-1.0 license)`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/izzy2lost/Starship)
- `hydration45` proposed `Public source and releases; no copyrighted game assets are included. Users provide a supported US ROM to generate game assets.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/izzy2lost/Starship/blob/main/README.md)
- **Recommended:** `Public CC0-1.0 source repository and releases; no copyrighted game assets are included. Users provide a supported US ROM to generate game assets.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/izzy2lost/Starship); [Maintainer README: source and game-data distribution](https://github.com/izzy2lost/Starship/blob/main/README.md)

### `theft4-gta-iv-ios`

Proposal rows: `round1-18`, `hydration45`.

**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository (GPL-3.0 license)`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/KoreanSeats1/Theft4)
- `hydration45` proposed `Public source repository and unsigned iOS IPA release; no game files or generated game data are included. Users provide files from their own legally obtained Xbox 360 copy.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/KoreanSeats1/Theft4/blob/main/README.md)
- **Recommended:** `Public GPL-3.0 source repository and unsigned iOS IPA release; no game files or generated game data are included. Users provide files from their own legally obtained Xbox 360 copy.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/KoreanSeats1/Theft4); [Maintainer README: source and game-data distribution](https://github.com/KoreanSeats1/Theft4/blob/main/README.md)

### `wheeldroid-mario-kart-wii-android`

Proposal rows: `round1-18`, `hydration45`.

**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public source repository (GPL-3.0 license)`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/KeithKirenai/wheeldroid)
- `hydration45` proposed `Public source repository; neither Nintendo code nor game assets/data are included. Users supply their own legally dumped PAL RMCP01 disc image.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/KeithKirenai/wheeldroid/blob/main/README.md)
- **Recommended:** `Public GPL-3.0 source repository; neither Nintendo code nor game assets/data are included. Users supply their own legally dumped PAL RMCP01 disc image.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/KeithKirenai/wheeldroid); [Maintainer README: source and game-data distribution](https://github.com/KeithKirenai/wheeldroid/blob/main/README.md)

### `zelda3-android`

Proposal rows: `round1-18`, `hydration45`.

**`programming_language`** — current SQLite: ``.
- `round1-18` proposed `C/C++`. Evidence: [Android port README: upstream Zelda3 lineage and ROM asset extraction](https://github.com/Waterdish/zelda3-android); [Upstream Zelda3 reimplementation README](https://github.com/snesrev/zelda3); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Waterdish/zelda3-android)
- **Recommended:** `C/C++`. Resolution: single proposal. Evidence: [Android port README: upstream Zelda3 lineage and ROM asset extraction](https://github.com/Waterdish/zelda3-android); [Upstream Zelda3 reimplementation README](https://github.com/snesrev/zelda3); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Waterdish/zelda3-android)
**`source_available`** — current SQLite: `Repository`.
- `round1-18` proposed `Public Android port source; asset extraction from a user-owned A Link to the Past ROM is required`. Evidence: [Android port README: upstream Zelda3 lineage and ROM asset extraction](https://github.com/Waterdish/zelda3-android); [Upstream Zelda3 reimplementation README](https://github.com/snesrev/zelda3); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Waterdish/zelda3-android)
- `hydration45` proposed `Public source repository and Android release; users extract game assets locally from their own ROM, which is not included.`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/Waterdish/zelda3-android/blob/main/README.md)
- **Recommended:** `Public source repository and Android release; users extract game assets locally from their own A Link to the Past ROM, which is not included.`. Resolution: use detailed maintainer distribution caveat and retain explicit license evidence where present. Evidence: [Android port README: upstream Zelda3 lineage and ROM asset extraction](https://github.com/Waterdish/zelda3-android); [Upstream Zelda3 reimplementation README](https://github.com/snesrev/zelda3); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Waterdish/zelda3-android); [Maintainer README: source and game-data distribution](https://github.com/Waterdish/zelda3-android/blob/main/README.md)
**`subtype`** — current SQLite: `Android port of snesrev/zelda3`.
- `round1-18` proposed `Android app port of the native Zelda3 C reimplementation`. Evidence: [Android port README: upstream Zelda3 lineage and ROM asset extraction](https://github.com/Waterdish/zelda3-android); [Upstream Zelda3 reimplementation README](https://github.com/snesrev/zelda3); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Waterdish/zelda3-android)
- **Recommended:** `Android app port of the native Zelda3 C reimplementation`. Resolution: single proposal. Evidence: [Android port README: upstream Zelda3 lineage and ROM asset extraction](https://github.com/Waterdish/zelda3-android); [Upstream Zelda3 reimplementation README](https://github.com/snesrev/zelda3); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Waterdish/zelda3-android)

## Lane 45: all 27 metadata-hydration rows

Every row is accounted for once in the consolidated manifest. Distribution/platform claims cite the maintainer README; the three release dates cite their specific official releases. Where earlier source evidence added an explicit license or more precise platform requirement, it is retained and both source rows are cited.

### `acgc-android-port`
- `source_available` (included): `Public source repository and APK releases; no game data is included. Users supply their own supported GameCube disc image.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/zwaetschge/ACGC-Android-Port); [Maintainer README: source and game-data distribution](https://github.com/zwaetschge/ACGC-Android-Port/blob/master/README.md)
- `platform` (included): `Nintendo GameCube → Android 8+ (ARM64 or ARMv7; OpenGL ES 3)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/zwaetschge/ACGC-Android-Port/blob/master/README.md)

### `aleph-one-marathon`
- `source_available` (included): `GPL-3.0 source repository and source tarballs are public; game scenario/data files are distributed separately or supplied by the user.`. Evidence: [Aleph One README: engine scope, supported titles, and scenario data](https://github.com/Aleph-One-Marathon/alephone); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Aleph-One-Marathon/alephone); [Maintainer README: source and game-data distribution](https://github.com/Aleph-One-Marathon/alephone/blob/master/README.md)

### `ballpad-super-mario-strikers-ios`
- `source_available` (included): `Public GPL-3.0 source repository; no ready-to-install app is published. PadMint builds a local app from source; users supply their own supported disc image.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ballpad); [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/ballpad/blob/main/README.md)
- `platform` (included): `Nintendo GameCube → iOS/iPadOS 17+ (native ARM64)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/ballpad/blob/main/README.md)

### `basilisk-ii-classic-mac`
- `source_available` (included): `GPL source is public in the shared macemu repository; users must separately supply a Mac OS copy and Macintosh ROM image.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/cebix/macemu); [Maintainer README: source and game-data distribution](https://github.com/cebix/macemu/blob/master/README.md)

### `bellpad-animal-crossing-ios`
- `source_available` (included): `Public source repository and reproducible build recipe; no store-distributed app. Users supply legally obtained supported game data.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/bellpad); [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/bellpad/blob/main/README.md)
- `platform` (included): `Nintendo GameCube → Apple Silicon macOS and iOS/iPadOS 17+ (ARM64)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/bellpad/blob/main/README.md)

### `ectopad-metroid-prime-ios`
- `source_available` (included): `Public source/build instructions for the Metaforce/PrimeDecomp-based iOS integration; no public IPA is currently documented. Users supply their own supported Metroid Prime disc image.`. Evidence: [EctoPad README: Metaforce/Aurora engine lineage, source build, and IPA status](https://github.com/chrissotraidis/ectopad); [Metaforce project README](https://github.com/AxioDL/metaforce); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/ectopad); [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/ectopad/blob/main/README.md)
- `platform` (included): `Nintendo GameCube → iOS/iPadOS 14+ and Apple Silicon macOS`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/ectopad/blob/main/README.md)

### `gta-vice-city-stories-android-recomp`
- `source_available` (included): `Public MIT-licensed source repository and Android APK releases contain only the program; users must supply their own game files.`. Evidence: [Maintainer README: Android requirements, device, APK variants and known issues](https://github.com/codepdbh/PSPRecomp-VCS-Android); [Maintainer Android releases](https://github.com/codepdbh/PSPRecomp-VCS-Android/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/codepdbh/PSPRecomp-VCS-Android); [Maintainer README: source and game-data distribution](https://github.com/codepdbh/PSPRecomp-VCS-Android/blob/main/README.md)

### `harvest-moon-64-android-recomp`
- `source_available` (included): `Public GPL-3.0 source repository and Android APK releases; game assets are not included and users must supply an NTSC-U ROM.`. Evidence: [Maintainer README: Android requirements, ROM import and touch/dual-screen UI](https://github.com/igawa6/HarvestMoon64Recomp); [Maintainer Android releases](https://github.com/igawa6/HarvestMoon64Recomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/igawa6/HarvestMoon64Recomp); [Maintainer README: source and game-data distribution](https://github.com/igawa6/HarvestMoon64Recomp/blob/main/README.md)
- `platform` (included): `Nintendo 64 → Android 9+ (arm64-v8a; Vulkan 1.1)`. Evidence: [Maintainer README: Android requirements, ROM import and touch/dual-screen UI](https://github.com/igawa6/HarvestMoon64Recomp); [Maintainer Android releases](https://github.com/igawa6/HarvestMoon64Recomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/igawa6/HarvestMoon64Recomp); [Maintainer README: source and game-data distribution](https://github.com/igawa6/HarvestMoon64Recomp/blob/main/README.md)

### `meleepad-melee-ios`
- `source_available` (included): `Public GPL-3.0 source repository and module-free app shell; PadMint builds the game module from the user’s own supported disc. Generated game code is not distributed.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/meleepad); [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/meleepad/blob/main/README.md)
- `platform` (included): `Nintendo GameCube → iOS/iPadOS 16+ and Apple Silicon macOS 14+`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/meleepad/blob/main/README.md)

### `metroid-prime-hunters-android-recomp`
- `source_available` (included): `Public GPL-3.0 source repository and APK releases; no game data is included. Users supply a verified USA revision 0 cartridge dump.`. Evidence: [Maintainer README: Android device limits, input and ROM import](https://github.com/aabrole/HuntersRecomp); [Maintainer Android releases](https://github.com/aabrole/HuntersRecomp/releases); [GitHub repository API: source availability and license metadata](https://api.github.com/repos/aabrole/HuntersRecomp); [Maintainer README: source and game-data distribution](https://github.com/aabrole/HuntersRecomp/blob/main/README.md)
- `platform` (included): `Nintendo DS → Android 11+ (arm64; dual-screen; AYN Thor tested)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/aabrole/HuntersRecomp/blob/main/README.md)

### `mini-vmac`
- `source_available` (included): `Public GPL-2.0 source repository with build scripts for the documented Macintosh models and host platforms.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/minivmac/minivmac); [Maintainer README: source and game-data distribution](https://github.com/minivmac/minivmac/blob/master/README.md)

### `minish-cap-aleks-ultimate-nx`
- `source_available` (included): `Public source repository and Switch NRO releases; no ROM or copyrighted game data is included. Users supply their own legally obtained USA GBA ROM.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/Alexgg1014/The-Legend-of-Zelda-The-Minish-Cap-Alek-s-Ultimate-NX-Edition); [Maintainer README: source and game-data distribution](https://github.com/Alexgg1014/The-Legend-of-Zelda-The-Minish-Cap-Alek-s-Ultimate-NX-Edition/blob/main/README.md)

### `perfect-dark-android`
- `source_available` (included): `Public MIT-licensed source repository and automatic builds; the user must supply a supported Perfect Dark ROM, which is not included.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/izzy2lost/perfect_dark); [Maintainer README: source and game-data distribution](https://github.com/izzy2lost/perfect_dark/blob/port/README.md); [Maintainer Android README: Android 5/API 21+, supported ABIs, and user-supplied ROM](https://github.com/izzy2lost/perfect_dark/blob/android-features/README.md); [Maintainer Android 2.0.0 release](https://github.com/izzy2lost/perfect_dark/releases/tag/2.0.0)
- `platform` (included): `Nintendo 64 → Android 5+ (arm64-v8a, armeabi-v7a, x86_64, x86)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/izzy2lost/perfect_dark/blob/port/README.md); [Maintainer Android README: Android 5/API 21+, supported ABIs, and user-supplied ROM](https://github.com/izzy2lost/perfect_dark/blob/android-features/README.md); [Maintainer Android 2.0.0 release](https://github.com/izzy2lost/perfect_dark/releases/tag/2.0.0)

### `projectreach-halo-xbox-ios`
- `source_available` (included): `Public source repository and build instructions; there is no app download. Users supply their own supported Xbox disc image.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/projectreach); [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/projectreach/blob/main/README.md)
- `platform` (included): `Original Xbox → iOS/iPadOS 17.4+ and Apple Silicon macOS 14.4+`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/projectreach/blob/main/README.md)

### `sacred-stones-android-recomp`
- `source_available` (included): `Public source repository and Windows/Android release packages; the game ROM, GBA BIOS, and generated ROM-derived code are not included.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/Lordingard/SacredStonesRecomp); [Maintainer README: source and game-data distribution](https://github.com/Lordingard/SacredStonesRecomp/blob/main/README.md)
- `platform` (included): `Game Boy Advance → Windows x64 and Android 9+ (ARM64)`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/Lordingard/SacredStonesRecomp/blob/main/README.md)

### `sheepshaver-classic-mac`
- `source_available` (included): `GPL source is public in the shared macemu repository; users must separately supply a Mac OS copy and PowerMac ROM image.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/cebix/macemu); [Maintainer README: source and game-data distribution](https://github.com/cebix/macemu/blob/master/README.md)

### `sm64-android-port-base`
- `source_available` (included): `Public source and Android build instructions; a user-supplied Super Mario 64 baserom is required and is not included.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/VDavid003/sm64-port-android-base); [Maintainer README: source and game-data distribution](https://github.com/VDavid003/sm64-port-android-base/blob/master/README.md)

### `snappad-pokemon-snap-ios`
- `source_available` (included): `Public integration source and build instructions; public playable downloads are paused. Users provide their own USA ROM; the repository excludes ROM-derived code and game assets.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/chrissotraidis/snappad); [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/snappad/blob/main/README.md)
- `platform` (included): `Nintendo 64 → iOS/iPadOS 15+ and Apple Silicon macOS`. Evidence: [Maintainer README: source and game-data distribution](https://github.com/chrissotraidis/snappad/blob/main/README.md)

### `spaghettikart-android`
- `source_available` (included): `Public source and releases; no copyrighted game assets are included. Users provide a supported USA ROM.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/izzy2lost/SpaghettiKart); [Maintainer README: source and game-data distribution](https://github.com/izzy2lost/SpaghettiKart/blob/main/README.md)

### `starship-android`
- `source_available` (included): `Public CC0-1.0 source repository and releases; no copyrighted game assets are included. Users provide a supported US ROM to generate game assets.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/izzy2lost/Starship); [Maintainer README: source and game-data distribution](https://github.com/izzy2lost/Starship/blob/main/README.md)

### `theft4-gta-iv-ios`
- `source_available` (included): `Public GPL-3.0 source repository and unsigned iOS IPA release; no game files or generated game data are included. Users provide files from their own legally obtained Xbox 360 copy.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/KoreanSeats1/Theft4); [Maintainer README: source and game-data distribution](https://github.com/KoreanSeats1/Theft4/blob/main/README.md)

### `wheeldroid-mario-kart-wii-android`
- `source_available` (included): `Public GPL-3.0 source repository; neither Nintendo code nor game assets/data are included. Users supply their own legally dumped PAL RMCP01 disc image.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/KeithKirenai/wheeldroid); [Maintainer README: source and game-data distribution](https://github.com/KeithKirenai/wheeldroid/blob/main/README.md)

### `zelda3-android`
- `source_available` (included): `Public source repository and Android release; users extract game assets locally from their own A Link to the Past ROM, which is not included.`. Evidence: [Android port README: upstream Zelda3 lineage and ROM asset extraction](https://github.com/Waterdish/zelda3-android); [Upstream Zelda3 reimplementation README](https://github.com/snesrev/zelda3); [GitHub repository language metadata / GitHub repository API: source availability and license metadata](https://api.github.com/repos/Waterdish/zelda3-android); [Maintainer README: source and game-data distribution](https://github.com/Waterdish/zelda3-android/blob/main/README.md)

### `cavalry-language-switcher`
- `updated` (included): `2026-09-29`. Evidence: [Latest maintainer release, Cavalry 2.7.2 patch 12](https://github.com/daftAI2026/Cavalry-i18n/releases/tag/cavalry-2.7.2-p12)

### `fire-emblem-warriors-3ds-ko`
- `updated` (included): `2026-10-11`. Evidence: [Latest maintainer patch release, v0.2.3](https://github.com/arqhive/fire-emblem-warriors-korean-translation/releases/tag/v0.2.3)

### `pokemon-azoth-en`
- `updated` (included): `2026-10-13`. Evidence: [Latest maintainer patch release, beta-22](https://github.com/RiFa-Innovation/pkmn-azoth-translation/releases/tag/beta-22)

### `crossover`
- `source_available` (included): `Commercial product; CodeWeavers publishes a FOSS source bundle for CrossOver 26.3.0 and lists its included upstream components.`. Evidence: [CodeWeavers source-code page: downloadable FOSS bundle and component list](https://www.codeweavers.com/crossover/source)

## Microsoft Bob and You Don’t Know Jack

### Microsoft Bob (`microsoft-bob-recomp`)

Round1-18 only says “Public source repository”; lanes19-43 supplies the better primary-source boundary. The recommendation is `Public MIT-licensed tooling, runtime, hand-written code, and analysis metadata. Original Bob binaries/data and generated lifted C are not included and are not covered by the repository MIT license; users must provide their own disc files.`. Evidence: [GitHub repository API: source availability and license metadata](https://api.github.com/repos/sp00nznet/bob); [Microsoft Bob maintainer README: platform, six-module lift, current startup/database milestones, blocker, build requirements, and license boundary](https://github.com/sp00nznet/bob); [Microsoft Bob repository license: MIT applies to repository-owned code, not original binaries or generated lifted C](https://github.com/sp00nznet/bob/blob/master/LICENSE).

The current status records startup and Jet database execution but not the more granular six-module lift/MFC-window/host-fault state. Use the lane19-43 status, which stops before the first rendered frame and does not imply a usable app release. Current status: `Bring-up stage. The project reports that the original startup path and Jet databases execute in its host; a first rendered application frame is not yet documented.`. Recommended: `Bring-up; the maintainer reports that the six Bob and Jet/Access Basic modules lift to link-clean C, Bob's real startup and MFC window creation execute, and Jet opens the original databases. The current blocker is a host longjmp/stack fault before the first rendered frame; no GUI frame or usable application release is claimed.`. Evidence: [Microsoft Bob maintainer README: platform, six-module lift, current startup/database milestones, blocker, build requirements, and license boundary](https://github.com/sp00nznet/bob); [Microsoft Bob repository license: MIT applies to repository-owned code, not original binaries or generated lifted C](https://github.com/sp00nznet/bob/blob/master/LICENSE).

### You Don’t Know Jack (`ydkj-ps3-recomp`)

Lane44 has no duplicate proposal row, but it improves the current stale status. It distinguishes the rendered Scaleform attract/legal screen from gameplay, which remains blocked before gameplay by CRI movie-decoding/SPU work. Current status: `Early WIP; README describes partial boot/render progress, not complete`. Recommended: `Partial rendering only. The USA PS3 EBOOT (BLUS30569) now runs as a native Windows x86-64 executable and draws the Scaleform attract/legal screen through D3D12 at 1280×720 (the README reports 32 FPS telemetry); some dynamic text is garbled. It remains on the logo-video screen because the CRI movie-decoding/SPU task path is not implemented, so gameplay has not been reached.`. Evidence: [Maintainer README: current boot/render status, title ID, and CRI video frontier](https://github.com/sp00nznet/youdontknowjack#current-status).

Lane44 also gives the BLUS30569 USA EBOOT requirement, D3D12 render milestone, user-supplied game-data boundary, and recompilation method. `updated=2026-10-15` is the lane’s verification date, not a release date.

## Review notes

- Source availability distinguishes public tooling/source from copyrighted game binaries, ROMs, discs, assets, or generated lifted code. A public repository is not represented as a playable download.
- When lane45’s README-based caveat omitted an explicit license stated by the earlier GitHub language/license API row, the final source-availability value retains both pieces of information and cites both sources.
- The Harvest Moon 64 Android platform value merges lane45’s Android 9+/arm64-v8a claim with round1-18’s documented Vulkan 1.1 requirement.
- Non-overlapping fields are preserved. Values already equal to SQLite are omitted from the import manifest.
- The importer validates but does not persist row `sources` in SQLite; source URLs remain in the manifest and this audit.
