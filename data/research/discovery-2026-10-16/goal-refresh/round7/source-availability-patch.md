# Source availability and metadata patch

Checked against the current authoritative catalog on 2026-10-16. The importer batch is `project-metadata-updates.json`; Block Blast media is in `media-manifest.json`. Only values that differ from SQLite are in the importer batch. The separate 53-row fresh-metadata package is excluded because its values already match the current catalog.

## Nine remaining boolean values stored as `1`

These are the nine exact rows still containing literal text `1` in `source_available`.

| Project ID | Current value | Proposed human-readable value | Evidence |
|---|---|---|---|
| `amstrad-pcw-mister-core` | `1` | Public SystemVerilog/Verilog/VHDL source and runnable dated MiSTer bitstreams in the repository. The maintainer README documents two PCW model groups and standard DSK images; GitHub reports no recognized repository license or tagged release. | [Maintainer README: supported PCW models, features, disk formats and usage](https://github.com/MiSTer-devel/Amstrad-PCW_MiSTer/blob/master/README.md); [Official dated MiSTer bitstreams](https://github.com/MiSTer-devel/Amstrad-PCW_MiSTer/tree/master/releases) |
| `arcade-astrocade-mister-core` | `1` | Public VHDL/Verilog/SystemVerilog source plus dated MiSTer RBF and MRA files in the repository. The README says arcade ROMs are not included and must be supplied separately; GitHub reports no recognized repository license or tagged release. | [Maintainer README: core history, ROM requirement and MRA files](https://github.com/MiSTer-devel/Arcade-Astrocade_MiSTer/blob/master/README.txt); [Official MiSTer arcade documentation and supported game families](https://github.com/MiSTer-devel/MkDocs_MiSTer/blob/main/docs/cores/arcade.md) |
| `colecods-ds` | `1` | Public C/C++ source and v11.0 Nintendo DS/DSi release assets. The maintainer permits copying and distribution only for non-commercial use, with attribution and third-party core terms preserved; required BIOS and game files are not bundled. | [Maintainer README: source copyright/distribution terms, BIOS requirements and bundled content](https://github.com/wavemotion-dave/ColecoDS/blob/main/README.md); [Maintainer v11.0 release and assets (2026-08-20)](https://github.com/wavemotion-dave/ColecoDS/releases/tag/11.0) |
| `ep128emu` | `1` | Public C++ source under the GNU GPL v2 (the repository includes COPYING) and tagged release 2.0.11.2. The README documents Windows installer assets and source builds for Linux/POSIX and macOS. | [Maintainer README: source/build availability and host platforms](https://github.com/istvan-v/ep128emu/blob/master/README); [Maintainer COPYING file: GNU GPL version 2](https://github.com/istvan-v/ep128emu/blob/master/COPYING) |
| `gameandwatch-mister-core` | `1` | Public MiSTer RTL, ROM-generator source and generated game packages. The README lists component-specific terms: core RTL/docs MIT, MiSTer glue GPLv3, vendored SDRAM controller MIT, and framework files under their upstream notices; no tagged GitHub release is listed. | [Maintainer README: source/package availability and component-specific licensing table](https://github.com/MiSTer-devel/GameAndWatch_MiSTer/blob/master/README.md); [Maintainer repository and generated ROM package tree](https://github.com/MiSTer-devel/GameAndWatch_MiSTer) |
| `gamecom-mister-core` | `1` | Public Verilog/SystemVerilog source under GPL-2.0 and a dated runnable RBF bitstream in the repository releases directory; no tagged GitHub release is listed. | [Maintainer README: core behavior and source/project notes](https://github.com/MiSTer-devel/GameCom_MiSTer/blob/master/Readme.md); [Maintainer GPL-2.0 license](https://github.com/MiSTer-devel/GameCom_MiSTer/blob/master/LICENSE) |
| `gp32emu` | `1` | Public C/C++/JavaScript source and beta release 1.0. The maintainer README says BIOS, game and commercial assets are not bundled; no repository-wide license was identified in the README or GitHub license metadata. | [Maintainer README: source builds, frontends and bundled-asset policy](https://github.com/gameblabla/gp32emu/blob/main/README.md); [Maintainer beta release 1.0 (2026-06-08)](https://github.com/gameblabla/gp32emu/releases/tag/1.0) |
| `ibm-pc110-mister-core` | `1` | Public GPL-3.0-or-later source for PC110-specific code and downloadable MiSTer-ready release packages; shared CPU/platform RTL keeps its upstream per-file licenses. The README says the public v2026.08.08 package needs a later MiSTer Main build and is not in the official update database; IBM firmware is not included. | [Maintainer README: source licensing, public package and MiSTer Main prerequisite](https://github.com/ahmadexp/PC110_MiSTer/blob/main/README.md); [Maintainer third-party notices: upstream licenses and firmware exclusions](https://github.com/ahmadexp/PC110_MiSTer/blob/main/THIRD_PARTY_NOTICES.md) |
| `yakc-east-german-computers` | `1` | Public MIT-licensed source repository; the README links a WebAssembly/browser demo. The maintainer marks the project abandoned and no tagged GitHub release is listed. | [Maintainer README: project state, systems and browser demo](https://github.com/floooh/yakc/blob/master/README.md); [Maintainer MIT license](https://github.com/floooh/yakc/blob/master/LICENSE) |

## Current generic `Repository` values

The catalog has 45 such rows. Forty-three point to GitHub repositories present in the first-party round6 metadata snapshot and are patched below: 19 have explicit repository-level SPDX values; 24 have no SPDX-identified license metadata. The text reports GitHub metadata and avoids inferring legal terms.

### Explicit SPDX metadata

| Project ID | Current value | Proposed value | Evidence |
|---|---|---|---|
| `battletanx-global-assault-vita-recomp` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as GPL-3.0. | [GitHub API](https://api.github.com/repos/TheGh0stShip/VitaTanxGA) |
| `bloody-roar-ii-ps1-pc-recomp` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as MIT. | [GitHub API](https://api.github.com/repos/novapowers0/BloodyRoar2Recomp) |
| `castlevania-sotn-xbla-switch-recomp` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as MIT. | [GitHub API](https://api.github.com/repos/Surmavanick/castlevania-nx) |
| `chibi-robo-gamecube-windows-static-recomp` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as GPL-3.0. | [GitHub API](https://api.github.com/repos/Armandpro12/chibi-robo-pc) |
| `crash-team-racing-vita-high-octane` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as GPL-3.0. | [GitHub API](https://api.github.com/repos/Rinnegatamante/Crash-Team-Racing-High-Octane) |
| `dragon-age-origins-xbox360-windows-recomp` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as MIT. | [GitHub API](https://api.github.com/repos/96dot/DAORecomp) |
| `final-fantasy-xi-macos-arm64-recomp` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as MIT. | [GitHub API](https://api.github.com/repos/rubymatrix/xi-on-anything) |
| `gears-of-war-3-xbox360-windows-recomp` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as GPL-3.0. | [GitHub API](https://api.github.com/repos/KiltedKernel/gears-of-war-3-pc-port) |
| `ken-griffey-jr-slugfest-n64-linux-recomp` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as GPL-3.0. | [GitHub API](https://api.github.com/repos/CalenCyr/SlugfestRecompiled) |
| `metroid-zero-mission-switch-port` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as GPL-3.0. | [GitHub API](https://api.github.com/repos/JJTapiaXX/Metroid-Zero-Mission-NX) |
| `new-star-soccer-5-source-reconstruction` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as MIT. | [GitHub API](https://api.github.com/repos/pr0sc0per9000/nss5-forge) |
| `nfs-underground-2-pc-macos-arm64-recomp` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as MIT. | [GitHub API](https://api.github.com/repos/TalhaMudo/NFSU2Recomp) |
| `paper-mario-n64-switch-port-paperboat` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as CC0-1.0. | [GitHub API](https://api.github.com/repos/Thorhax/PaperBoat-NX-Modern) |
| `peggle-2-xbox360-switch-recomp` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as GPL-3.0. | [GitHub API](https://api.github.com/repos/ChanseyIsTheBest/peggle2-nx) |
| `saru-saru-daisakusen-psp-recomp` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as MIT. | [GitHub API](https://api.github.com/repos/jackgllghr/bigmission-recomp) |
| `star-fox-64-switch-port-starship` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as CC0-1.0. | [GitHub API](https://api.github.com/repos/HarbourMasters/Starship) |
| `super-mario-64-switch-port-ghostship` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as MIT. | [GitHub API](https://api.github.com/repos/HarbourMasters/Ghostship) |
| `super-mario-sunshine-switch-port` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as MIT. | [GitHub API](https://api.github.com/repos/chasem-dev/sms-switch) |
| `tekken-tag-tournament-2-wiiu-linux-recomp` | `Repository` | Public source repository; GitHub repository metadata identifies the repository-level license as MIT. | [GitHub API](https://api.github.com/repos/Petexy/Tekken-Tag-2-Recompiled) |

### No SPDX-identified license metadata

| Project ID | Current value | Proposed value | Evidence |
|---|---|---|---|
| `another-world-cross-release-source-reconstruction` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/ArqueologiaDigital/another-world-source-reconstruction) |
| `blood-1997-matching-decompilation` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/NBlood/Blood-RE) |
| `blood-mapedit-matching-decompilation` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/NBlood/Blood-RE) |
| `castlevania-sotn-ps1-3ds-recomp` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/randygil/SymphonyRecomp-3DS) |
| `championship-manager-0102-macos-arm64-recomp` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/VictorDMor/cm0102mac) |
| `command-conquer-red-alert-2-pc-recomp` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/sp00nznet/redalert2-recomp) |
| `f-zero-snes-vita-recomp` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/rameauv/FZeroRecomp-vita) |
| `golden-sun-gba-desktop-recomp` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/Shmargus/GSRecomp) |
| `lord-of-the-rings-two-towers-xbox-recomp` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/Gumpolas/TooTawersRecomp) |
| `mario-kart-64-switch-port-spaghettikart` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/HarbourMasters/SpaghettiKart) |
| `mega-man-x-switch-recomp` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/Thorhax/MegaManX-NX) |
| `nfs-carbon-pc-macos-arm64-recomp` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/vasist15-source/nfs-carbon-mac) |
| `nfs-most-wanted-pc-android-recomp` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/codepdbh/nfsmwpc-recompiled) |
| `nfs-most-wanted-pc-macos-recomp` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/elforeign/nfs-most-wanted-mac) |
| `pokemon-crystal-gen2recomped-console` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/UNDERdecoded/Gen2Recomped) |
| `pokemon-gold-gen2recomped-console` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/UNDERdecoded/Gen2Recomped) |
| `pokemon-silver-gen2recomped-console` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/UNDERdecoded/Gen2Recomped) |
| `quest-bbc-micro-byte-exact-source-reconstruction` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/retro-foundry/quest-decomp) |
| `star-wars-shadows-of-the-empire-n64-recomp` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/GTTeancum/SotE-Recomp) |
| `starcraft-64-switch-port` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/gabryboy12-ai/StarCraft64-NX) |
| `super-smash-bros-64-switch-port-battleship` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/KawaiiBunga/BattleShipNX) |
| `super-smash-bros-melee-switch-port` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/KawaiiBunga/Melee-NX) |
| `tekken-6-xbox360-windows-recomp` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/mahmoud-mess/tekken6-recomp) |
| `wave-race-64-switch-port-racewave46` | `Repository` | Public source repository; GitHub reports no SPDX-identified license metadata. | [GitHub API](https://api.github.com/repos/1weell/RaceWave46-Switch) |

Two remaining `Repository` rows point to GitLab (`vkd3d-upstream`, `wine`). They are held because the GitLab pages redirected to sign-in and the API requests returned 404 in this audit; no source availability or license details are inferred.

## Block Blast existing-row correction

Block Blast appears in a staged new-project file but is already catalogued; it is treated as an existing-row hydration. It has current review date `2026-10-16` and predominant language `C`, so those are not changed.

| Field | Current value | Proposed value | Evidence |
|---|---|---|---|
| `status` | Release v1.3 was published 2026-07-27. The maintainer provides CIA and 3DSX builds and documents stylus or button play, automatic saves, and stereoscopic 3D. The README says its screenshots are rendered by the game’s PC drawing code rather than captured from a console, and makes no hardware frame-rate claim. | Latest published GitHub release is v1.3 (2026-07-27). The current README changelog also lists v1.4 save-threading and performance changes, but GitHub does not list a v1.4 release. The README gallery is PC-rendered rather than captured from a console and makes no hardware frame-rate claim. | [README](https://github.com/TheFilipcom4607/BlockBlast-3DS/blob/6d8c7d6e67b696ddcba462256ff6d56da853632d/README.md); [v1.3 release](https://github.com/TheFilipcom4607/BlockBlast-3DS/releases/tag/V1.3) |
| `platform` | Block Blast-style mobile puzzle → Nintendo 3DS homebrew | Nintendo 3DS family: original 3DS, 3DS XL, 2DS, New 3DS and New 2DS; install as CIA via CFW/Luma and FBI or as 3DSX via Homebrew Launcher. | [README](https://github.com/TheFilipcom4607/BlockBlast-3DS/blob/6d8c7d6e67b696ddcba462256ff6d56da853632d/README.md); [v1.3 release](https://github.com/TheFilipcom4607/BlockBlast-3DS/releases/tag/V1.3) |
| `source_available` | Public source repository and CIA/3DSX releases | Public zlib-licensed C source and CIA/3DSX builds. The README says the independent game generates its graphics and audio in code and includes no original Block Blast code or assets. | [README](https://github.com/TheFilipcom4607/BlockBlast-3DS/blob/6d8c7d6e67b696ddcba462256ff6d56da853632d/README.md); [v1.3 release](https://github.com/TheFilipcom4607/BlockBlast-3DS/releases/tag/V1.3) |

The media manifest adds five pinned README screenshots. Every label identifies the image as PC-rendered and not an on-console capture; all five pinned image URIs returned HTTP 200 as `image/png`.
