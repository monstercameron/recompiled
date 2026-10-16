# Game recompilation research — 2026-10-15

This batch stages 12 new game-specific recompilation candidates and 30 metadata refreshes for existing recompilation records. Claims come from the project repositories' own READMEs, compatibility/status notes, release pages, and GitHub repository metadata for primary-language values. No tracker, emulator compatibility list, or search-result snippet is used as proof.

The new candidates add coverage across Dreamcast, Nintendo DS, GBA, GameCube, WiiWare, PlayStation, PS2, Neo Geo, original Xbox, PSP, and a PC-to-Android target. Each row names the particular host/target and records the repository's own compatibility limits. Projects that are only general-purpose tools were not added: this excludes such leads as PSP recompilation toolkits, generic GameCube/Wii recompiler runtimes, Saturn/CPS-2 toolkits without a verified game-specific target, and Dolphin's recompilation CPU-core fork. Search results that matched an existing catalog game/repository were also excluded.

The 30 updates prioritize status and method corrections: GBA projects with interpreter/JIT or dynamic-RAM behavior are marked accordingly; the Super Mario World record no longer implies an end-to-end completion; the SNES projects distinguish LLE-first execution from full static recompilation; and PS2/PS1 rows now state tested routes, target revisions, renderer/runtime boundaries, or remaining compatibility limits. GitHub API primary-language values were checked against stored programming-language metadata where the repositories were accessible. The KHRecomp repository, README, and API currently return 404, so that row records unclear source availability and leaves its language field unchanged. No project start dates were inferred from repository creation or push dates because the reviewed project pages did not state when work began.

## New projects

- `ecco-defender-of-the-future-dreamcast-recomp`: full Linux playthrough reported; Europe disc only, other desktop hosts untested.
- `phantom-hourglass-ds-recomp`: static ARM9 recompilation plus melonDS for the remaining DS CPU/hardware; Windows 11 and one Pixel 8 are the reported test hosts.
- `kirby-amazing-mirror-gba-recomp`: Windows experimental build; first area and 8,000+ frames verified, deeper content untested.
- `lotr-return-of-the-king-gamecube-recomp`: Apple Silicon macOS GameCube recompilation; only the opening levels have been tested.
- `muscle-march-wiiware-recomp`: beta with whole-game playability claimed; few controllers/devices tested.
- `resident-evil-3-nemesis-ps1-recomp`: Windows x64 static recompilation; a v1.4.3 project release exists, but the README provides no full-game compatibility claim.
- `jet-moto-2-ps1-recomp`: one track/race tested on Windows 11; several features remain untested.
- `metropolismania-ps2-recomp`: early Windows bring-up reaches the city scene; no playable game is claimed.
- `king-of-fighters-95-neogeo-recomp`: Neo Geo MVS/AES static 68000 recompilation with desktop and PS5 build instructions; the README's 60 fps claim is not a full-playthrough metric.
- `jet-set-radio-future-xbox-recomp`: original Xbox game to Windows; Steam Deck is documented through Proton 11, not as a native Linux build.
- `chili-con-carnage-psp-recomp`: Windows x64 beta for the USA PSP release; the README does not quantify campaign compatibility.
- `call-of-duty-1-5-android-recomp`: experimental Android port of PC version 1.5; physical-device full-campaign validation remains open.

## Existing metadata updates

The updates file contains 30 reviewed rows: 15 GBA targets (including shared-repository targets as separate games), five SNES targets, eight PS2 targets, and two PS1 targets. Each row includes the project README and GitHub repository metadata source; release pages are added when the update depends on a release tag. The `updated` values record this review date, not project start dates or repository activity dates.

The most material method clarifications are:

- FireRed, LeafGreen, Ruby, Sapphire, Emerald, and Minish Cap use a static-code path plus interpreter/JIT coverage healing; Mega Man Zero is static-first with interpreter fallback.
- KHRecomp's former README described static ROM recompilation with a dynamic-RAM bridge, but the project page and API now return 404, so the current status and language metadata are left unverified.
- Super Metroid is LLE-first with static native routines and interpreter fallback; Mega Man X2 boots through an interpreter with AOT coverage as an optimization.
- SimCity SNES documents fully static W65C816 and SPC700 dispatch without a general interpreter/JIT/emulator fallback.
- The PS2 rows distinguish game-code recompilation from native renderers, emulator-derived graphics paths, and remaining host/runtime components.

All source URLs are project-owned GitHub READMEs, releases, or GitHub's repository metadata API. The staged files are research artifacts only; the database, generated catalog, and build outputs were not changed.
