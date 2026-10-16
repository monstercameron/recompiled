# Round 12 console/handheld port discovery

Researched 2026-10-16. This is a scratch import bundle only; it does not alter SQLite or tracked files.

## Proposed import

- 13 target rows across 7 public repositories: one each for CTR Native, ReRAC, MGB64, OpenJam, Advanced Daisenryaku, and Mister Mosquito; seven game-target rows share the single PokeRecomp repository.
- 13 exact repository-wide GitHub Languages API records, one per project row. The PokeRecomp rows deliberately share its one repository-wide language map.
- Four remote media records (three images and one YouTube video). Each URI is linked from the project's own README; no media was downloaded.

The live catalog snapshot contained 3,411 projects when this lane began. The final audit checks every proposed ID and canonical repository URL against SQLite and all `projects` arrays found in `work/` and `data/research/` JSON files. Separate PokeRecomp rows are intentional: the maintainer names seven GBA/GBC game targets, documents per-game status, and describes external sibling source builds for Ruby/Sapphire and Gold/Silver/Crystal.

## Included projects and primary evidence

- [CTR Native](https://github.com/CTR-tools/ctr-native): PlayStation Crash Team Racing decompilation-based PC source port; README lists Windows/Linux builds and the latest published tag inspected was beta 7.1 (2026-07-07).
- [PokeRecomp](https://github.com/sonsegajp/PokeRecomp): pret-based native x86/SDL2 port for FireRed, LeafGreen, Ruby, Sapphire, Gold, Silver, and Crystal. Its README distinguishes playable FireRed/LeafGreen and Gold/Silver/Crystal from Ruby/Sapphire's boot-and-battle state with rendering issues; the latter five targets need sibling source builds. Latest inspected release: v2.1.3 (2026-10-14). The records explicitly note that the upstream-derived game data is not licensed by the port's code license.
- [ReRAC](https://github.com/re-rac/rerac): Ratchet & Clank (PS2) native Rust/Bevy rewrite. The maintainer says it is early and active, not yet finishable, and that all levels load and render. Latest inspected release: [v0.3.4-alpha](https://github.com/re-rac/rerac/releases/tag/v0.3.4-alpha) (2026-10-15). This is a separate implementation from the existing `daino-selvatico/rac1-native-game` record.
- [MGB64](https://github.com/akratch/mgb64): independent GoldenEye 007 (N64) decompilation and native source port. The README marks it discontinued/unmaintained and says all 20 single-player levels reach mission-complete; its latest listed release is v0.4.0-alpha.3. This is a separate project from the GoldenEye native ports already cataloged.
- [OpenJam](https://github.com/GTTeancum/OpenJam): NBA Jam: On Fire Edition (Xbox 360/XBLA) ReXGlue static recompilation for Windows. The README reports it boots to gameplay, runs the full game rather than a trial, and has known rough edges; v1.0 was published 2026-09-26.
- [Advanced Daisenryaku](https://github.com/eric-wen-dev-it/AdvancedDaisenryaku): Mega Drive/Genesis strategy game reconstructed for native Windows with statically translated game logic. The README explicitly says full-game equivalence is not established and lists incomplete execution, audio, player-two, and campaign coverage; the record retains those limits rather than treating the port as complete.

- [Mister Mosquito PC](https://github.com/danielgomesvieira2000/mister-mosquito-pc): PS2-to-Windows static translation using the shared `ps2-port-engine`; its README reports the title, menus, and first stage working with sound, and says no release is available yet.

The Games rows use `Decompilation` for source ports based on reconstructed source, `Reimplementation` for ReRAC's native rewrite, and `Recompilation` for binary/static translation projects. Status is taken from each maintainer repository/release, not inferred from star counts or commit recency.

## Media evidence

- PokeRecomp's FireRed cross-region gameplay image: `docs/screenshots/ingame-firered-visitor-in-hoenn.png`, embedded by its README.
- MGB64's Runway gameplay video and YouTube thumbnail: linked by the MGB64 README.
- OpenJam's Heat-vs-Mavericks screenshot: embedded by its README.
- Advanced Daisenryaku's original-resolution battle preview: embedded by its README.

All are stored as HTTPS URIs in `media.json`; no image/video bytes were fetched or copied into the repository.

## Deduplication and exclusions

- Excluded exact discoveries already present in SQLite and/or existing project manifests, including SwitchWaker, God of War PS2 Recomp, The 3rd Birthday, ICO PC, Pokémon Pure Platinum, Metroid Prime PC, Kingdom Hearts 358/2 Days PC, Super Smash Bros. 64-to-DS, GoldenEye's `jkdansereau/goldeneye-pc-port` lead, and several Sega/Saturn examples.
- `nathanialf/ico-pc` was found in `data/research/console-phone-projects.json`, so it was not proposed again even though it was not part of the baseline DB. The expanded search also rejected `hctp-recomp` and `MinishCapRecomp` because they already appear in `data/research/recomp-additions.json`; `KillzoneRecomp`, Animal Crossing GameCube-to-3DS, GoldenEye N64-to-3DS, Minish Cap GBA-to-Switch, SSBU Recomp, SwitchWakerHD, and SA2 NX were already catalogued or staged by sibling research. No additional console-to-console row survived deduplication in this pass.
- General-purpose `ps2-port-engine` and `saturn-recomp` are toolchains, not game-specific target projects for this lane.
- `luispolis124/OpenInnocent` is a newly surfaced PSP-to-PC idea page with no substantive implementation beyond a short README/status plan, so it is held rather than represented as an implemented port.
- PokeRecomp's seven rows share one repository URL by design, and the language evidence is repeated by target ID because the catalog stores language evidence per project row.

## Files

- `project-additions.json` — importer-shaped project records.
- `programming-languages.json` — exact GitHub language-byte maps.
- `media.json` — project-linked external media URIs only.
- `candidate-audit.json` — machine-readable dedup and hold notes.
