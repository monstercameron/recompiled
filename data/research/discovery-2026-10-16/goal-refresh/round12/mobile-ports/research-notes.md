# Mobile ports research — 2026-10-16

## Results

This lane stages seven net-new catalog records: four game-specific Android targets, two Android-binary-to-Switch homebrew ports, and one reusable Android-to-Horizon software compatibility layer. The new-project manifest includes full source, platform, strategy, status, target-type, and content/license notes. Exact GitHub Languages API byte maps are in `programming-languages.json`.

The strongest new mobile sources are:

- [Burnout 3 Android static recompilation](https://github.com/aribrilliantsyah/Burnout3-Recompiled-Mobile): maintainer README, Android-device notes, changelog, source/license, and GitHub API evidence. Its NTSC-U XBE-to-ARM64 port has device-reported frame rates and three direct Android captures. It overlaps the already-cataloged Takedawn project’s experimental Android target; keep both only as separate maintained implementations, and do not double-count them as separate game coverage.
- [Yoshi’s Island: Super Mario Advance 3 Android recompilation](https://github.com/Edwin2929/sma3-recomp-android.): maintainer README, v0.9 Android APK release, validation report, and third-party license notes. The release confirms a 31.8 MB ARM64 APK despite an outdated README note; broad gameplay/audio/save validation is still incomplete.
- [WWF WrestleMania: The Arcade Game recompilation](https://github.com/lurendrejer/wwfmania_recomp_public): maintainer README and Android TV notes document the TMS34010-to-C project and an NVIDIA Shield TV Pro test. Packaging and data-copy steps remain partly untested. This is the arcade game, not the existing WWF WrestleMania 2000 N64 project.
- [The Binding of Isaac: Repentance clean-room engine](https://github.com/caos-obliquo/tboi-recomp): maintainer README and v0.1.0 release are explicit that this is a non-playable Android ARM64 scaffold with 0/655 native functions implemented. Its Repentance target is distinct from the cataloged Vita recompilation.
- [Bully: Anniversary Edition Switch wrapper](https://github.com/givethesourceplox/bully-NX): maintainer README and repository metadata identify an unfinished Android ARM64 binary wrapper that boots on Switch. It requires user-owned v1.4.311 files and is derived from [max_nx](https://github.com/fgsfdsfgs/max_nx); it is not a source recompilation.
- [Streets of Rage X Switch port](https://github.com/delsonazevedo/sorx_nx): maintainer README and v1.0.1 release document a native Switch wrapper for the Android OpenBOR release. It uses an Android/JNI/libc/graphics/audio bridge and does not bundle game content.
- [Viridite](https://github.com/Viridite/Viridite): maintainer README, source-available license, release, and API metadata document a general Android-to-Horizon compatibility layer. The author reports Hill Climb Racing playable on Switch hardware with touch and audio, while the README still records a Shop crash and unverified save persistence. Its custom license is attribution-required, non-commercial, and share-alike; do not describe it as permissively open source.

## Existing rows and refreshes

All seven requested Android leads were checked against their maintained repository/readme/release evidence and exact GitHub language maps: NFS Carbon Android, MW2 Android, Sonic Unleashed Android, LibertyRecompAndroid, Daytona USA, Zelda Wind Waker HD Android, and Marathon Recompiled Android. The source release/push dates do not justify new status/platform claims or a language-map change. No project-field patch is staged for these seven.

Two directly README-linked media groups are staged for existing projects: four LibertyRecomp Android images and the Marathon Android project logo. The Unleashed README attachment returned HTTP 403 and is omitted. A separate newer-source refresh is staged for the existing `pokemon-emerald-decompilation-android` row: the cataloged KamiKitsune420 repository is a fork of GradenGnostic’s parent repository; the parent’s README and v1 release show a Windows/Linux release (2026-08-11) but no Android APK, and the parent was last pushed 2026-08-15. This corrects the old “no tagged release” and July 26 source-date claims, updates the project URL/review date, and refreshes the exact language map. It is not a new project row.

The source-linked media manifests contain 12 URI-only links total: seven for four new projects and five for two existing projects. All 12 URLs returned HTTP 200 with an image content type. Images were not downloaded or copied into the repository.

## Dedupe and validation

The final dedupe audit read the 3,450-row shared live SQLite catalog in read-only mode (it contained 3,411 rows at the first check) and checked 149 staged project-addition manifests across both research roots, covering 1,277 unique staged project identities with no parse errors. None of the seven additions has an exact live ID/repository collision or a staged ID/repository collision. The audit lists same-game rows where relevant: Burnout 3 has four live rows and two staged cross-platform entries, including Takedawn; Isaac Repentance has one live PC-to-Vita project. These overlaps are called out in the manifest for manual review rather than silently collapsed.

The catalog importer was run only against a temporary copy of the database. It imported all seven candidates, the one existing-project update, all eight language records (seven additions plus Pokémon Emerald), and all media groups. The temporary copy began with the then-current 3,450 rows and contained 3,457 afterward; the live database was not written. All eight language byte maps exactly match their saved GitHub Languages API responses, and each recorded predominant language matches the API’s largest byte count.

## Search coverage and holds

Public web search followed by primary GitHub repository/API checks also found 15 projects already present in the live catalog, including Goemon 64 Android, New Super Mario Bros. Wii recompilation, Skate 3 Android, Metroid Prime Android, NFS Underground 2 Android, StarCraft 64 Android, Zelda 64 Android, PSX Android, and SwitchWakerHD; see `search-coverage.json` for all IDs and platforms.

Held from this mobile-port batch: the Far Cry 3 Android lead lacks a README/release or language evidence; the LibertyRecomp mid-range fork duplicates the same GTA IV Android target; the Vita-port and Switch-engine repositories are general toolkits rather than specific game ports; the PvZ2Native lead targets desktop and uses an ARM CPU JIT; and the Sasky NFS MW repository is a duplicate fork of the already-cataloged codepdbh project. Full reasons are in `exclusions.json`.
