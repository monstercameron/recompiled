# Console-to-mobile research — 2026-10-15

Reviewed mobile-focused catalog rows and project-owned repositories, README/install guides, release pages, compatibility notes and device reports. This lane records console-origin games with Android or Apple-mobile targets. The catalog was checked read-only by project URL/repository identity and game before adding rows; separate game targets in a multi-title repository and separate platform targets in one repository remain distinct when the upstream identifies them.

## New targets staged

13 new repo/game records are staged: five Android/iOS targets plus eight per-title Gen1Recomp rows.

- Super Mario Strikers Android is distinct from the cataloged desktop base and iOS PadMint target. The Android repository supplies an arm64 APK v1.2.0, Android 9/API 28+, Vulkan and touch/gamepad controls; users provide game data.
- Sonic R Android is based on the PC decompilation of a Saturn-origin game; the Android path is identified because that is the source actually used. The repository has releases through v1.2.7 and touch controls, but does not state a minimum Android version or named physical test.
- Sonic CD iOS uses the 2011 RSDKv3 version. The repository describes an unsigned ARM64 IPA for iOS 13+, touchscreen controls and user-provided Data.rsdk; its disclaimer calls this an engine reimplementation only.
- Pokémon Emerald Dual Screen is a separate native Android target built on the Emerald decompilation. Release v0.6.0 is available; it requires a user-supplied USA/Europe ROM and targets AYN Thor/other dual-screen Android devices. The README does not name a tested handset.
- Halo: Combat Evolved Android is a distinct mobile target in the same OpenCE Ports repository as the catalog's Switch row. The Android release ZIP contains an APK and requires Android 9+ / 64-bit ARM; it uses Xbox build 2342 decompiled code, Vulkan/OpenGL ES, touch/gyro controls and user-supplied ISO/XISO. It is not a duplicate of the Switch-only record.
- Gen1Recomp is a hand-written Lua/LÖVE2D reimplementation, not a static binary recompilation. Eight game records are staged for Red, Blue, Yellow, FireRed, LeafGreen, Ruby, Sapphire and Emerald because the upstream names separate titles. v0.3.89 publishes Android APK and iOS IPA; Gen 2 remains under construction, so Gold, Silver and Crystal are excluded. FireRed, LeafGreen and the Hoenn GBA titles remain beta. ROMs are verified/imported but not bundled; no phone/tablet test devices are identified.

## Existing-record updates

30 existing mobile rows receive source-backed status, summary or platform refinements. They capture minimum OS and ABI where stated, release/build artifact availability, touch/gamepad/motion input, physical-device tests, ROM/data requirements, exact port/recompilation method and maturity. Unknown API floors, handset coverage or full-game completion remain explicit where project documentation does not establish them.

Some corrections with practical impact: Yakumo's game is marked playable and Android APK releases exist, but its README still lists real-phone Android testing/graphics/performance as open; the N64 StarCraft record has one physical Android acceptance but no prebuilt binary; Mario Kart Wii KartPad currently has a ready Android APK while current iOS/iPadOS builds are generated through PadMint; Diddy Kong Racing's current Android APK is Android 9+/ARM64, while the project still requests physical-device testing and its iOS reference is a separate source port.

## Scope decisions

No emulator/front-end entries were added. Gen 2 Gen1Recomp rows were excluded because they are not yet supported. No SQLite import, catalog export, documentation edit or build was performed; only the three files in this lane directory are staged.
