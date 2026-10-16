# Round 13 cross-platform ports/recompilations

Reviewed the canonical SQLite catalog and `/Users/earlcameron/Desktop/recompiled/AGENTS.md` on 2026-10-16. The staging manifests contain three new, distinct project targets:

- Sonic Generations (Xbox 360 → Android ARM64): ReXGlue static recompilation. Maintainer device report reaches title/Green Hill at about 2.5–4 fps with a skinning glitch; no public Android release or full-game claim was found.
- Syphon Filter (PS1 USA v1.1 → Windows x64): a game-specific PS1 guest runtime/emulator with native Windows presentation. This is deliberately not labeled a decompilation or recompilation.
- Super Mario Strikers (GameCube → Meta Quest 3): decompilation-based native ARM/OpenXR source port. The maintainer identifies the Android phone port as its base; the Quest app has distinct passthrough, tabletop and stadium modes. Quest 3 is reported tested; other headsets are untested.

For the Quest row, the README says “fork,” while GitHub’s repository metadata has `fork=false`. The staging notes preserve that distinction and describe only maintainer-identified lineage. Super Mario Strikers already has a sourced game profile and direct Wikipedia link in SQLite, so those are reused without duplicate profile/link additions. New Sonic Generations and Syphon Filter game keys have sourced facts and direct English Wikipedia links.

Excluded/deduped candidates and reasons are in `excluded-candidates.json`. In particular, Snowboard Kids 2 Android has an APK but the project’s engineering notes still report a crash when starting the game; the Switch recomp framework does not run commercial games; the Twisted Metal 4 Android APK does not have maintainer documentation establishing an Android target; the Link’s Awakening and NFS Carbon targets and the other named leads are already represented.

Raw GitHub API metadata/language/release/commit responses are under `evidence/github-api/`. No binary release assets or game files were downloaded. No live database, tracked file or generated catalog was changed. All importer-shaped JSON was checked for parseability, source URLs, duplicate project IDs/URLs, and pre-existing game keys.
