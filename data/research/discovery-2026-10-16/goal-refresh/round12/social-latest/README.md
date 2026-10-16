# Social and recent-project research — 2026-10-16

## Scope and result

Research checked on **2026-10-16**, concentrating on public announcements and primary-project activity from approximately **2026-04-16 through 2026-10-16**. Social posts and search indexes were used only as discovery leads. Every included project was checked against its maintainer README or status file, and GitHub project metadata; exact GitHub language maps are preserved in `github-primary-evidence.json` and `programming-languages.json`.

The importer-ready set contains **12 new project rows** (10 game targets and 2 software targets): 7 decompilations, 4 recompilations, and 1 behavioral reimplementation. It includes **12 exact language-evidence records**; the Transformers progress-tracker repository does not publish source code, so its language is explicitly left unknown. Six media references are remote URLs only; no media files were copied. Three new directory rows were verified: one project registry and two shared recompilation tools. **No existing-project field updates** were justified by newer primary evidence.

The proposals were deduplicated against the read-only live catalog snapshot (**3,411 projects and 55 directory entries**) and the other staged research manifests visible during this round. There were no exact project ID, canonical project URL, or directory URL collisions. Same-game efforts were reviewed individually and retained only where the repo and implementation are distinct. The separate Toy Story 2 addition in `game-recomp/new-projects.json` was intentionally omitted here.

## Search log

- **GitHub discovery:** searched public repositories for recent `decomp`, `decompilation`, `recomp`, `static recompilation`, software/game reconstruction and platform-port terms; used `gh`/GitHub repository search as a fast candidate finder. For candidates, checked the canonical repo README and relevant docs/releases, then queried the repository metadata and `/languages` API. The 14 primary snapshots (including one unavailable candidate and the sibling-lane Toy Story 2 dedupe) are in `github-primary-evidence.json`. Included repository creation dates range from **2026-08-12 to 2026-10-16**; the README and repository API snapshots were checked on **2026-10-16**.
- **Reddit:** searched recent posts in `r/decomps`, `r/EmuDev`, and adjacent emulation communities for Mac/Apple-Silicon, native-port, recompilation and decompilation leads. The directly actionable announcement was [OpenPopTB-MacOS: static recompilation of Populous: The Beginning](https://www.reddit.com/r/decomps/comments/1x67yvk/openpoptbmacos_static_recompilation_of_populous/), posted **2026-10-15**. Its linked [maintainer repository](https://github.com/iagodemacedo/OpenPopTB-MacOS), [README](https://github.com/iagodemacedo/OpenPopTB-MacOS/blob/main/README.md), and [v0.1.2 release](https://github.com/iagodemacedo/OpenPopTB-MacOS/releases/tag/v0.1.2) support the method, target, release platform, and stated single-player scope.
- **X/Twitter:** searched indexed `site:x.com` and `site:twitter.com` results for recent `decomp`, `recomp`, Mac, native port, and API translation posts. Search returned no usable independently verifiable new lead in this pass. The requested computer-use browser path was unavailable because the Mac was locked and automatic unlock was paused after physical input; no claim is made that private, login-gated, or unindexed X results were exhaustively searched.
- **Project registries/news:** reviewed recent entries surfaced by [recomp.gg news](https://recomp.gg/news), the [Recompendium static-recompilation catalog](https://nio03.github.io/unricopie/), and [recomp.fyi](https://recomp.fyi/), then checked corresponding projects against the live database. Recompendium and recomp.fyi are already in the directory. The distinct [unbaked.games project registry](https://unbaked.games/) was added after checking its [About/methodology page](https://unbaked.games/about/). Recent project news also surfaced candidates such as Jet Set Radio Future, BLACK, Link's Awakening, DK64, Blue Dragon, and MW2; those had existing rows and were not added again.
- **Shared tools from GitHub discovery:** inspected the primary [recomp-ui repository](https://github.com/RetroPortingToolKit/recomp-ui) and [recomp-common repository](https://github.com/tmcleroy/recomp-common). Both qualify as reusable developer tools rather than game-specific project rows and are represented once in `directory-additions.json`.

## Included projects

The complete evidence-backed records and sourced status wording are in `project-additions-reviewed.json`:

| Project | Target | Type | Primary project evidence |
| --- | --- | --- | --- |
| Super Metroid C# Port | Game | Behavioral reimplementation | [MrJackSpade/SuperMetroidDecomp](https://github.com/MrJackSpade/SuperMetroidDecomp) |
| Bokura no Kazoku | Game | Matching decompilation | [conor-sometimes/bokura-no-kazoku-decomp](https://github.com/conor-sometimes/bokura-no-kazoku-decomp) |
| White Knight Chronicles II | Game | PS3-to-Windows recompilation research | [WK2D/White-Knight-Chronicles-2-Decomp](https://github.com/WK2D/White-Knight-Chronicles-2-Decomp) |
| HyperBlade | Game | Matching decompilation | [zach2good/hyperblade-decomp](https://github.com/zach2good/hyperblade-decomp) |
| Zombie Revenge | Game | Decompilation | [consolesplayingconsoles/zombie-revenge-decomp](https://github.com/consolesplayingconsoles/zombie-revenge-decomp) |
| Street Fighter III: 3rd Strike | Game | CPS3 reconstruction/decompilation | [gibletto/sfIII3-cps3-decomp](https://github.com/gibletto/sfIII3-cps3-decomp) |
| Transformers PS2 tracker | Game | Decompilation progress tracker | [kayliox/transformers-ps2-decomp](https://github.com/kayliox/transformers-ps2-decomp) |
| Vagrant Story | Game | PS1 native/dynamic recompilation port | [SomeoneIsWorking/psx-vagrant-story](https://github.com/SomeoneIsWorking/psx-vagrant-story) |
| Lezac DOS | Software | Decompilation / RE study | [Hubert-Rybak/lezac-decompilation](https://github.com/Hubert-Rybak/lezac-decompilation) |
| RoConstruct legacy Roblox clients | Software | Matching decompilation | [colingsnyder2-ux/RoConstruct](https://github.com/colingsnyder2-ux/RoConstruct) |
| Super Mario Galaxy Apple Silicon port | Game | Native macOS recompilation port | [classicrob/MacioGalaxy](https://github.com/classicrob/MacioGalaxy) |
| Populous: The Beginning / Undiscovered Worlds | Game | Native macOS static recompilation | [iagodemacedo/OpenPopTB-MacOS](https://github.com/iagodemacedo/OpenPopTB-MacOS) |

Current progress claims are carefully scoped in the importer manifest: e.g. White Knight Chronicles II is explicitly not playable; Transformers is tracker-only with no public source; Vagrant Story has reached a limited battle-prologue milestone; and Populous's single-player is reported playable while multiplayer remains unfinished.

## Directory additions

- `unbaked-games-project-registry` — cross-platform project registry, documented methodology and public JSON API; it links out to canonical project sources rather than serving as the authority for them.
- `recomp-ui-shared-static-recomp-launcher` — shared MIT-licensed launcher/settings UI for static recompilation ports.
- `recomp-common-shared-port-ui-and-settings` — shared GPL-3.0-only C++17 overlay/settings/launcher code used by multiple port projects.

Exact descriptions and evidence URLs are in `directory-additions.json`. The requested [Nio03/unricopie Recompendium](https://nio03.github.io/unricopie/) is already present as `recompendium-static-recompilation-catalog`; it was rechecked and intentionally not duplicated.

## Dedupe decisions and held leads

`dedupe-audit.json` records the 3,411/55 read-only catalog counts, exact-collision results, reviewed same-game overlaps, and zero proposed status patches. `exclusions.json` records the source and reason for each held/excluded lead. Among notable checks:

- Tourist Trophy, Smash64DS, and CTRPad already have rows whose status agrees with their latest useful maintainer evidence.
- The PSX Toy Story 2 repo is in the sibling `game-recomp` staged manifest; Pokémon Colosseum is in `data/research/discovery-2026-10-16/next-wave/`.
- Recompendium and recomp.fyi already have directory entries.
- The PSP/VCS Mac recompilation link currently returns 404 from GitHub's API; the Sims 2, Jade Empire, and Tatsunoko vs. Capcom Mac experiments had social/video claims but no public repository or durable source/build documentation found in this pass. They remain leads, not catalog entries.
- A Dino Crisis modern-controls repo is an enhancement mod, not a project type tracked by this catalog. An MW2 private-decompilation rumor lacked an identifiable public project source.

## Validation and limits

All six remote image links were checked as repository assets without downloading them. Every JSON file in this folder parses. The project, language-evidence, and directory importers were run against a scratch copy of `data/catalog.sqlite`: they inserted 12 projects, 12 language records, and 3 directory rows; SQLite integrity was `ok` and `PRAGMA foreign_key_check` returned no violations. The live database and tracked files were not changed.

This is a time-bounded public-web scan, not a guarantee of exhaustive coverage. X/Twitter direct browsing was blocked by the locked Mac; login-gated/private posts, unindexed sites, unpublished projects, and projects without primary public evidence may be missed. No source code or media were copied into the research set.
