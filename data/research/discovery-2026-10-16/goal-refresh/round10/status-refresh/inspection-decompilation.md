## diablo-devilution-source-reconstruction — Devilution: Diablo Source Reconstruction
- Stars 9004; repo pushed 2025-09-15T02:01:36Z; archived False.
- Catalog status: Source reconstruction project; current maintenance moved to DevilutionX
- Latest release: 0.10.0 (2019-07-21T22:46:18Z) (pre-release); latest tag: works2.
- Latest default-branch commit: 2025-09-15T02:01:35Z 9f01757243 — Remove dead link
- README: 1: [![Build Status](https://circleci.com/gh/diasurgical/devilution.svg?style=svg)](https://circleci.com/gh/diasurgical/devilution) | 2: [![Build Status](https://www.travis-ci.com/diasurgical/devilution.svg?branch=master)](https://www.travis-ci.com/diasurgical/devilution) | 3: [![Build status](https://ci.appveyor.com/api/projects/status/ce9wnf46gqqk6prp?svg=true)](https://ci.appveyor.com/project/AJenbo/devilution) | 4: [![Downloads](https://img.shields.io/github/downloads/diasurgical/devilution/total.svg)](https://github.com/diasurgical/devilution/releases) | 12: **Note**, Devilution requires an original copy of `diabdat.mpq`. None of the Diablo 1 game assets are provided by this project. To get a legitimate copy of the game assets, please refer to the [GoG release of Diablo 1](https://www.gog.com/game/diablo). | 15: While most titles from Blizzard receive years of love and support, Diablo stayed in the shadows. Abandoned in favor of a sequel, it remained full of bugs and unfinished potential. The game was last patched in 2001 before being discontinued altogether, a problem I wanted to fix... | 17: Diablo's development team moved on to Diablo II while passing the source code down to **Synergistic Software** for Hellfire. Less known however is that it was also given to **Climax Studios** to create a PlayStation port. Now Sony has long been known for letting things slide; ... | 22: Having the source code makes Diablo much easier to update and maintain. For years mod-makers had to rely on tedious code editing and memory injection. A few even went further and reversed most or all of the game. The problem is that they rarely shared their work. Usually being...

## ilspy-dotnet-decompiler — ILSpy
- Stars 26666; repo pushed 2026-10-16T09:24:17Z; archived False.
- Catalog status: Active; v11.1 was released 2026-09-23 and source changes continued through 2026-10-14.
- Latest release: v11.1 (2026-09-23T06:43:43Z) (regular); latest tag: v11.1.
- Latest default-branch commit: 2026-10-14T15:01:39Z dad8a25ed2 — Bump ossf/scorecard-action from 2.4.4 to 2.5.0 (#4243)
- README: 1: # ILSpy [![NuGet](https://img.shields.io/nuget/v/ICSharpCode.Decompiler.svg)](https://nuget.org/packages/ICSharpCode.Decompiler) ![Build ILSpy](https://github.com/icsharpcode/ILSpy/workflows/Build%20ILSpy/badge.svg?branch=master) [![Mastodon](https://img.shields.io/badge/dynam... | 3: ILSpy is the open-source, cross-platform .NET assembly browser and decompiler. The desktop UI runs on Windows, Linux, and macOS (built on [Avalonia](https://avaloniaui.net/)). | 5: Download: [latest release](https://github.com/icsharpcode/ILSpy/releases) | [latest CI build (master)](https://github.com/icsharpcode/ILSpy/actions?query=workflow%3A%22Build+ILSpy%22+branch%3Amaster+is%3Asuccess+event%3Apush) | [Microsoft Store (RTM versions only)](https://app... | 10: Aside from the cross-platform desktop UI ILSpy (downloadable via Releases, see also [plugins](https://github.com/icsharpcode/ILSpy/wiki/Plugins)), the following other frontends are available: | 12: * Visual Studio 2022/2026 ship with decompilation support for F12 enabled by default (using our engines v8.2 and v9.1 respectively). | 15: * Our dotnet tool for Linux/Mac/Windows - check out [ILSpyCmd](ICSharpCode.ILSpyCmd) in this repository | [dotnet tool install](https://www.nuget.org/packages/ilspycmd/) | 16: * [C# for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=ms-dotnettools.csharp) ships with decompilation support as well. To enable, activate the setting "Enable Decompilation Support". | 18: * Our Linux/Mac/Windows [PowerShell cmdlets](ICSharpCode.Decompiler.PowerShell) in this repository

## jadx-android-bytecode-decompiler — JADX
- Stars 50993; repo pushed 2026-10-14T22:10:04Z; archived False.
- Catalog status: Active; v1.5.6 was released 2026-07-10 and source changes continued through 2026-10-14. The maintainers warn that decompilation can be incomplete and produce errors.
- Latest release: v1.5.6 (2026-07-10T19:07:35Z) (regular); latest tag: v1.5.6.
- Latest default-branch commit: 2026-10-14T22:10:03Z c4c45fde27 — feat(gui): docking support using ModernDocking (PR #3012)
- README: 16: Command-line and GUI tools for producing Java source code from Android Dex and APK files | 20: > Check the [Troubleshooting guide](https://github.com/skylot/jadx/wiki/Troubleshooting-Q&A#decompilation-issues) for workarounds. | 22: **Main features:** | 25: - decode `AndroidManifest.xml` and other resources from `resources.arsc` | 28: **jadx-gui features:** | 37: See these features in action here: [jadx-gui features overview](https://github.com/skylot/jadx/wiki/jadx-gui-features-overview) | 41: ### Download | 42: - release

## lego-island-decomp — LEGO Island Decompilation
- Stars 3491; repo pushed 2026-09-18T23:29:27Z; archived False.
- Catalog status: Complete decompilation for English version 1.1, per README
- Latest release: continuous (2026-09-18T23:29:27Z) (regular); latest tag: continuous.
- Latest default-branch commit: 2026-09-18T23:16:23Z 8119bd0136 — Revert PDB changes (#1806)
- README: 5: This is a complete decompilation of LEGO Island (Version 1.1, English). It aims to be as accurate as possible, matching the recompiled instructions to the original machine code as much as possible. The goal is to provide a workable codebase that can be modified, improved, and ... | 7: > **Note:** This repository is for decompilation only and its code is true to the original release. It will not compile for targets other than 32-bit Windows. For a modern adaptation of the LEGO Island codebase with native compatibility for all major platforms and the Web, see... | 9: ## Status | 15: | `CONFIG.EXE` | [CONFIGPROGRESS.HTML](https://isledecomp.github.io/isle/CONFIGPROGRESS.HTML) | [CONFIG target result](https://isledecomp.github.io/isle/report.html#targets) | | 16: | `ISLE.EXE` | [ISLEPROGRESS.HTML](https://isledecomp.github.io/isle/ISLEPROGRESS.HTML) | [ISLE target result](https://isledecomp.github.io/isle/report.html#targets) | | 17: | `LEGO1.DLL` | [LEGO1PROGRESS.HTML](https://isledecomp.github.io/isle/LEGO1PROGRESS.HTML) | [LEGO1 target result](https://isledecomp.github.io/isle/report.html#targets) | | 20: The ReproBit report records the cold rebuild that reproduced all three | 24: `CONFIG.EXE`, `ISLE.EXE`, and `LEGO1.DLL` are completely decompiled. The

## oot-decomp — Ocarina of Time Decompilation
- Stars 5593; repo pushed 2026-10-14T18:19:57Z; archived False.
- Catalog status: Complete; repository still notes work in progress
- Latest release: none (n/a); latest tag: none.
- Latest default-branch commit: 2026-10-14T18:19:57Z 8a3634ca77 — [cc0 oot] EnKusa (#2819)
- README: 3: [![Build Status][gha-badge]][gha] [![Decompilation Progress][progress-badge]][progress] [![Contributors][contributors-badge]][contributors] [![Discord Channel][discord-badge]][discord] | 6: [gha-badge]: https://img.shields.io/github/actions/workflow/status/zeldaret/oot/ci.yml | 8: [progress]: https://zelda.deco.mp/games/oot | 9: [progress-badge]: https://img.shields.io/endpoint?url=https://zelda.deco.mp/assets/csv/progress-oot-shield.json | 20: This repository is a work in progress, and while it can be used to make certain changes, it's still | 21: constantly evolving. If you use it for modding purposes in its current state, please be aware that | 28: It builds the following versions: | 30: | Name | Build timestamp | Description | MD5 hash of input ROM(s) |

## opengoal-jak-and-daxter — OpenGOAL: Jak and Daxter Native PC Port
- Stars 3575; repo pushed 2026-10-16T01:27:52Z; archived False.
- Catalog status: Active; build and play workflow documented
- Latest release: v0.3.9 (2026-10-15T23:15:46Z) (regular); latest tag: v0.3.9.
- Latest default-branch commit: 2026-10-16T01:27:51Z 32c3cf7397 — github: add AI policy, contributing gidelines, and a PR template that all xref each other (#4447)
- README: 16: > Our repositories on GitHub are for development of the project and tracking active issues. Most of the information you will find here pertains to setting up the project for development purposes and is not relevant to the end-user. | 25: > **Do not use this decompilation project without the use of your own legally purchased copy of the game.** OpenGOAL does not include any assets from the original games, so you must provide your own legitimately obtained PS2 copy of the game. OpenGOAL supports every retail PAL... | 28: - [Current Status](#current-status) | 33: - [Building and Running the Game](#building-and-running-the-game) | 35: - [Build the Game (Running the Compiler)](#build-the-game-running-the-compiler) | 36: - [Run the Game](#run-the-game) | 38: - [Running the Game Without Auto-Booting](#running-the-game-without-auto-booting) | 52: - try to match things from the original game and development as possible. For example, the original GOAL compiler supported live modification of code while the game is running, so we do the same, even though it's not required for just porting the game.

## pokecrystal — Pokémon Crystal Disassembly
- Stars 2528; repo pushed 2026-10-06T17:56:31Z; archived False.
- Catalog status: Complete
- Latest release: none (n/a); latest tag: none.
- Latest default-branch commit: 2026-10-06T17:48:07Z 3bc8daa417 — Update hardware.inc to 6.0.0
- README: 1: # Pokémon Crystal [![Build Status][ci-badge]][ci] | 5: It builds the following ROMs: | 7: - Pokemon - Crystal Version (UE) (V1.0) [C][!].gbc `sha1: f4cd194bdee0d04ca4eac29e09b8e4e9d818c133` | 8: - Pokemon - Crystal Version (UE) (V1.1) [C][!].gbc `sha1: f2f52230b536214ef7c9924f483392993e226cfb` | 9: - Pokemon - Crystal Version (A) [C][!].gbc `sha1: a0fc810f1d4e124434f7be2c989ab5b5892ddf36`

## pokeemerald — Pokémon Emerald Decompilation
- Stars 3597; repo pushed 2026-10-01T19:02:13Z; archived False.
- Catalog status: In progress
- Latest release: none (n/a); latest tag: none.
- Latest default-branch commit: 2026-10-01T19:02:13Z 731ad5bfd6 — Report SaveBlock1/SaveBlock2/PokemonStorage usage (#2364)
- README: 5: It builds the following ROM:

## pokered — Pokémon Red and Blue Disassembly
- Stars 4994; repo pushed 2026-10-06T17:56:05Z; archived False.
- Catalog status: Complete
- Latest release: none (n/a); latest tag: none.
- Latest default-branch commit: 2026-10-06T17:44:05Z af51989971 — Update hardware.inc to 6.0.0
- README: 1: # Pokémon Red and Blue [![Build Status][ci-badge]][ci] | 5: It builds the following ROMs: | 9: - BLUEMONS.GB (debug build) `sha1: 5b1456177671b79b263c614ea0e7cc9ac542e9c4`

## pycdc-python-bytecode-decompiler — Decompyle++ (pycdc)
- Stars 4651; repo pushed 2026-04-07T05:58:13Z; archived False.
- Catalog status: Public source remains active; last push checked 2026-04-07; no tagged release is listed. The README says broad version coverage is a project aim but does not publish a compatibility matrix, so bytecode-version support is uncertain.
- Latest release: none (n/a); latest tag: none.
- Latest default-branch commit: 2026-04-07T05:58:13Z b428976097 — Merge pull request #599 from daiche330-png/wip/python311-exception-opcodes-20260405
- README: 7: support byte-code from any version of Python. | 16: ## Building Decompyle++ | 18: * The following options can be passed to CMake to control debug features: | 22: | `-DCMAKE_BUILD_TYPE=Debug` | Produce debugging symbols | | 26: * Build the generated project or makefile | 27: * For projects (e.g. MSVC), open the generated project file and build it | 28: * For makefiles, just run `make` | 29: * To run tests (on \*nix or MSYS), run `make check JOBS=4` (optional

## reko-binary-decompiler — Reko
- Stars 2618; repo pushed 2026-10-14T19:41:59Z; archived False.
- Catalog status: Active; version 0.12.5 was released 2026-09-29 and source changes continued through 2026-10-14. The supported-binaries matrix gives architecture and format maturity tiers; not every listed target has stable decompilation.
- Latest release: version-0.12.5 (2026-09-29T00:11:28Z) (regular); latest tag: version-0.12.5.
- Latest default-branch commit: 2026-10-14T19:41:22Z 63c497aef0 — Fix: slnx file was incorrect.
- README: 4: [![GitHub workflow status](https://github.com/uxmal/reko/workflows/Reko/badge.svg)](https://github.com/uxmal/reko/actions/workflows/reko.yml) | 13: ends to help it achieve its goals. A command-line and a Windows GUI | 34: Reko has the ambition of supporting decompilation of various | 36: intervention. For a complete list, see the | 37: [supported binaries](https://github.com/uxmal/reko/wiki/Supported-binaries) | 45: ## Downloading Reko | 47: Official releases are published every few months on [Github](https://github.com/uxmal/reko/releases) | 49: Users who can't or won't build Reko themselves can download the output

## sm64-decomp — Super Mario 64 Decompilation
- Stars 8885; repo pushed 2024-02-04T10:26:16Z; archived False.
- Catalog status: Complete
- Latest release: irix2 (2019-08-25T06:36:31Z) (regular); latest tag: irix2.
- Latest default-branch commit: 2023-08-17T13:21:59Z 9921382a68 — ique hotfix
- README: 3: - This repo contains a full decompilation of Super Mario 64 of the following releases: Japan (jp), North America (us), Europe (eu), Shindou (sh) and iQue Player (cn). | 4: - Naming and documentation of the source code and data structures are in progress. | 6: It builds the following ROMs: | 14: This repo does not include all assets necessary for compiling the ROMs. | 19: 1. Install prerequisites: `sudo apt install -y binutils-mips-linux-gnu build-essential git pkgconf python3` | 20: 2. Clone the repo from within Linux: `git clone https://github.com/n64decomp/sm64.git` | 21: 3. Place a Super Mario 64 ROM called `baserom.<VERSION>.z64` into the project folder for asset extraction, where `VERSION` can be `jp`, `us`, `eu`, `sh`, or `cn`. | 22: 4. Run `make` to build. Specify the version through `make VERSION=<VERSION>`. Add `-j4` to improve build speed (hardware dependent).
