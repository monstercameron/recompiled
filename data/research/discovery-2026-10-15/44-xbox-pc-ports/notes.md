# Xbox, Xbox 360, PS3, and PS4 desktop ports

Checked 2026-10-15. Discovery used GitHub CLI repository search (including `gh search repos 'ReXGlue' --limit 100 --sort updated --order desc`), repository metadata/readmes/releases, and primary project documentation. Every proposed row is linked to the maintainer repository and direct evidence for method, current status, host platform, and user-supplied game data. GitHub language API endpoints are included where the language field is populated.

## Staged

- 16 new game rows in `new-projects.json`: BLACK macOS, Midtown Madness 3 PC Native, Project 1944 (Call of Duty 3), three separate Nathan Drake Collection executable targets, Gears of War 2, Sega Superstars Tennis ReMatch, Diablo III Rehydrated, Forza Motorsport 3 (twilightysparkle), Pirates at World's End, Skullburglar (50 Cent), Catherine macOS, WWE All Stars, the separate Fable2Recomp project, and X-Men Destiny.
- One status/summary/method/source-availability/language update to existing `ydkj-ps3-recomp`.
- Project 1944 is separate from catalog row `call-of-duty-3-xbox360-recomp` ([FrankHUN88453/CoD3Recomp](https://github.com/FrankHUN88453/CoD3Recomp)): it is an independent ReXGlue implementation and has its own Windows host, launcher, release, and tested scope. The current Project 1944 README/releases show v2.0 as the latest release; v1.0.0 is linked as historical evidence. The v1.0 test note covers only the first Saint-Lô mission, checkpoint save/load, and 120 Hz timing on one PC. Current README describes additional v2.0 multiplayer work, but still does not claim a complete campaign.
- Forza Motorsport 3 has an existing independent implementation record for [`0herculabs/hl360-forza3`](https://github.com/0herculabs/hl360-forza3); this staged row is the separate `twilightysparkle/forza3-recomp` repository, with its own v0.1.0 release, USA Ultimate Collection disc checks, Direct3D 12 host, and limited opening-race play test.
- BLACK's existing Windows recompilation is [`portingpete/Black-2006-Recomp`](https://github.com/portingpete/Black-2006-Recomp), a different repository/host target; this row covers `elpetese/black-macos-recomp` and its Apple Silicon runtime.
- Fable II already has a record for [`himdo/Fable-2-Recomp`](https://github.com/himdo/Fable-2-Recomp). `Fable2Recomp/Fable2Recomp` is an independent non-fork repository and is staged separately with explicit WIP/no-playtest qualification; it identifies a GOTY TU1 base and Windows/Linux x64 build presets.
- Nathan Drake Collection has one row for each distinct game executable documented by the maintainer: `eboot.bin`, `big2-ps4_Shipping.elf`, and `big3-ps4_Shipping.elf`. The Windows project and its Linux fork are one project lineage per title, not duplicate records. Linux-specific support, CUSA02320 requirements, and fork status are cited in each row.
- Skullburglar has native desktop targets as its catalog scope here (Windows, Linux/Steam Deck, and Apple Silicon macOS); its experimental mobile builds are outside this batch. No Skullburglar record was found in the mobile lane or other staged files.
- The Sega Superstars Tennis v0.4 README does not identify gameplay coverage, so status says that rather than implying it is playable. X-Men Destiny's README mentions Linux/Vulkan generally, but only the Windows build path is documented; the row records that limitation.

## Metadata update

The existing You Don't Know Jack PS3 record lagged behind the README's current render milestone. The update now distinguishes the rendered Scaleform attract/legal screen from gameplay: the title remains stuck waiting on CRI movie decode/SPU work, so no gameplay is reached.

## Dedupe and exclusions

Checked project URLs, source URLs, IDs, and target/game names against SQLite, `project-additions-reviewed.json`, and all staged discovery `new-projects.json` / `metadata-updates.json` files, including Sony/Sega lane 37 and mobile lane 40. Existing BLACK Windows, Call of Duty 3, Forza Motorsport 3, Fable II, You Don't Know Jack, flOw, Silent Hill: Downpour, Project RE:Visited RE3, and Tony Hawk's Project 8 rows were not re-added under their existing repository/implementation. The independent CoD3, Forza 3, and Fable II implementations are called out above rather than being merged by game title alone. Uncharted was not present in lane 37, SQLite, or central additions.

No SQLite, generated docs, central additions, or imports were modified.
