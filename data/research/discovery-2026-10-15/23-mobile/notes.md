# Console/PC-to-mobile discovery — 2026-10-15

## Scope and method

Searched GitHub repository metadata and Android/decomp/recomp queries, then reviewed each candidate's own README, source tree, releases, requirements, and available test notes. Cross-checked canonical repository URLs, IDs, and game titles against the current SQLite catalog and all previously staged `new-projects.json` batches. Source links are primary project/release/API evidence; no game files or media were downloaded.

## New records staged

- **Galaxy Android — Super Mario Galaxy** (`OpenPort-Labs/Galaxy-Android`): a distinct Wii-to-Android ARM64 port based on the Petari decompilation and GalaxyQuest mobile work. Published v0.0.1c includes an APK. The repository lists Android 11/API 30+, ARM64, Vulkan 1.0 or GLES 3.2, and a user-provided disc. It is an early project without a full-game or broad device-compatibility claim. The GitHub Releases API currently lists v0.0.1c as the latest published release; the repository also has a later `v0.0.1cx` tag, but no published release for that tag was present when checked.
- **NHL 07 Original Xbox Android port** (`Wannamingo/NHL07-Android`): a newly published ARM64 native recompilation with v0.4.3 APK and source archive. The maintainer validates on Anbernic RG477M/Mali and documents Android 10+, ARM64, and OpenGL ES 3.0. Validation is a playable experimental checkpoint, not a guarantee for all modes or devices.

Both additions have HTTPS repository, release, and GitHub language-statistics sources; source availability and user-provided game-data requirements are stated explicitly. Predominant languages are C++ for Galaxy Android and C for NHL 07 according to each repository's language API.

## Existing records hydrated

- **Open Nectar / Pikmin** now records the v0.9.2 Android APK, Android 10+/arm64-v8a/GLES 3.0 requirements, disc revisions, and the maintainer's “most of the game playable” scope, while leaving physical Android device coverage unspecified because the README does not name a device.
- **Need for Speed Underground 2 Recompiled** now records the v0.14.5 ARM64 and ARMv7 APKs, Android 10+/Vulkan 1.1 requirements, source-side static recompilation and DXVK Native path, plus separate Android device evidence. The ARM64 APK was installed on a Galaxy S25; the release notes say ARMv7 was build/package verified but not tested on physical 32-bit hardware. The README's reported 35–51+ FPS is kept as a project-reported sample, not a performance promise.

## Leads deliberately excluded from this port lane

- `mgrz18/turok2-decomp` is a real N64 decompilation, but its own description calls Android/macOS ports a future stepping stone; it does not currently document a mobile build.
- `elkir0/sc2-android-recomp` describes itself as tools/method only and contains no game code or assets; no Android game build or release was found. Do not classify a toolkit recipe as a verified port.
- `veritr1x/recomp-kit` is a general-purpose x86 game recompilation kit. Its reviewed first target, Populous: The Beginning, is a separate desktop project; this lane found no Android/iOS game release to record.
- `etruj/MetalGearSolidTouch-Decompilation-` is a substantial new decompilation of an iOS-origin app (MGS Touch v2.3.0), but it is mobile-to-C research rather than a console/PC-to-mobile port. Reddit comments mention an Android build without linking a maintainer Android repository or release. The decomp merits a separate mobile-software-decompilation entry; it is not included in this port lane based on unverified third-party claims.
- Android project requests, projects with only a mobile roadmap, and unrelated emulator ports were not added.

## Validation

The manifests are staged only; this lane did not import into SQLite, export generated docs, or run the site build. Read-only checks parse both JSON files, validate required project fields/kinds/HTTPS sources, confirm all metadata-update IDs exist in SQLite, and check new IDs/titles/repository URLs against the live catalog and every earlier staged additions manifest.
