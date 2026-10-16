# Deep 3D game-key correction

## Recommended exact key

`Deep 3D: Submarine Odyssey (J2ME, Sony Ericsson 1.0.8)`

This is a neutral, edition-qualified key: the project maintainer’s repository uses the title “Deep 3D: Submarine Odyssey,” while the upstream compatibility-engine README identifies the tested original DEEP build as the Sony Ericsson 1.0.8 JAR. The `J2ME` suffix identifies the original MIDlet/JAR game data; it does not claim the fan-made Android compatibility engine is an official Fishlabs Android edition. The publisher-style game list uses the canonical form *DEEP: Submarine Odyssey* and lists the 2007 entry on J2ME and Symbian.

The project game/platform/summary update is in `deep-3d-metadata-patch.json`; replacement sourced facts are in `deep-3d-game-profile.json`; and the direct Wikipedia link is in `deep-3d-wikipedia-link.json`. The platform clarification staged in the prior version of `deep-3d-metadata-patch.json` has been folded into the updated patch, which supersedes that earlier patch.

## Key usage check

Read-only check of `data/catalog.sqlite` on 2026-10-16 found the obsolete key `Deep 3D: Submarine Odyssey (Android release)` in exactly these live catalog locations:

- one project row: `deep-3d-submarine-odyssey-android-switch-port`
- one `games` row, whose Wikipedia URL is an exact-title search fallback and whose `wikipedia_title` repeats the obsolete key
- four `game_facts` rows attached to that key

No other project’s `game` value or `games.game_key` row uses the old key. The `game_facts.game_key` foreign key points to `games.game_key` with `ON DELETE CASCADE`. The project update, replacement profile import, and replacement Wikipedia-link import should be applied and verified under the new key first; then the obsolete `games` row can be deleted, which also removes its four old facts. No catalog deletion was performed here. Historical round7 review manifests still contain the old key as provenance; they were not modified.

## Evidence

- [ABYSSAL Switch port README](https://github.com/artslay/abyssal_nx/blob/main/README.md): names Deep 3D: Submarine Odyssey, accepts an original DEEP JAR, uses the original Android APK as a build input, credits TheWWWorm for the Android game-port source, and states the project is unofficial and unaffiliated with Fishlabs.
- [TheWWWorm upstream README](https://github.com/TheWWWorm/abyssal/blob/main/README.md): describes an independent compatibility engine that reads a user-owned DEEP JAR; identifies Sony Ericsson DEEP 1.0.8 as the build used for gameplay checking and lists the validated English JAR.
- [TheWWWorm v1.0.0 release](https://github.com/TheWWWorm/abyssal/releases/tag/v1.0.0): lists Android 8+ as an engine package target, says the engine ships without game content, and says the user supplies/imports the DEEP JAR.
- [Fishlabs article on English Wikipedia](https://en.wikipedia.org/wiki/Fishlabs): lists the game as DEEP: Submarine Odyssey (2007), J2ME/Symbian. No standalone English Wikipedia game article was found, so this direct article URL is more precise than retaining the stale Android-release search URL.

No database, docs, generated files, or tracked files were changed.
