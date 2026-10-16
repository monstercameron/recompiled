# PC-to-mobile ports and translations

Research date: 2026-10-15
Scope: PC game/software ports, mobile source builds, browser/PWA compatibility layers, and Vulkan-to-Metal translation where they fit the catalog taxonomy.

## Staged result

- new-projects.json: 9 candidate records, each checked against the SQLite catalog by ID, exact title, and normalized project URL.
- metadata-updates.json: 25 unique updates for existing SQLite IDs, each with source links and an evidence note.
- No database, catalog docs, or build files were edited or imported.

## Candidate coverage

The nine missing records span Android native ports and browser delivery:

- cataclysm-dda-android: official experimental Android build; arm32/arm64 release artifacts and extra documented ABI options; no declared runtime Android floor.
- corsixth-android: pre-release Android port; Android 8.1+ and 640×480 minimum; original Theme Hospital data or in-app official demo.
- brogue-ce-android: Android fork with F-Droid packages; Android 7.0+ and arm64-v8a/armeabi-v7a/x86_64; the port README still labels it unsupported.
- freeciv-android: legacy Android port based on Freeciv 2.6.0; Android SDK 26 is a build-tool requirement, not a confirmed runtime minimum.
- flare-engine-android: Android APK and manual game-data copy guide; the cited project guide does not state OS/API or ABI requirements.
- barony-android-port: prerelease native port; Android 8.0+, arm64-v8a, GLES3, and privately imported data from an owned PC installation. Release notes provide several real-device reports, while maintainers say GPU testing is ongoing.
- celeste-android: Android 8.0/API 26+, arm64-v8a, controller required, and user-provided PC FNA game files; project README reports a Galaxy S23 test on Android 16.
- warzone-2100-web-android: upstream maintainer points to a browser build playable on Android. The announcement does not state minimum browser/OS, CPU, or device tests.
- pocketzot-dcss-mobile: installable mobile PWA for iOS/Android browsers with online WebTiles and a separately downloaded offline WebAssembly engine; no native store package or minimum browser requirements are listed.

FLARE is classified as Software because it is a game engine. The Warzone and PocketZot records distinguish browser delivery from native app packaging. Source ports use the catalog's existing Reimplementation kind; MoltenVK stays API Translation.

## Audit notes

For existing records, the staged updates capture explicit Android API/OS floors, iOS deployment targets, CPU/ABI packages, release and install paths, device reports, required user-owned game data, public source status, and programming-language evidence where the current field was incomplete.

Several sources do not publish a minimum runtime OS, ABI list, or device test matrix. Those unknowns are stated directly in the update instead of treating build SDK/tool versions as runtime requirements. Reported hardware checks are source-reported only: no physical-device testing was performed in this research lane.

Notable compatibility details include the single open Augustus report of a black screen on one Pixel 9 Pro running Android 15 (the reporter says Android 12 worked), ScummVM's Android 4.1+ floor and reported testing through Android 16, Godot's current renderer-specific Android floors, and MoltenVK 1.4.2 deployment targets of macOS 12.0, iOS 15.0, and tvOS 15.0. Krita's old API 21 build example is explicitly treated as historical, not a current minimum.

## Validation

Both JSON files parse. Schema checks confirmed 9 unique additions and 25 unique metadata updates; all update IDs exist in SQLite; new IDs, exact titles, and normalized project URLs do not collide with SQLite rows; allowed project kinds/target types, fields, and HTTPS source URLs pass.
