# Round 10 status refresh — decisions and holds

Checked 2026-10-16 against the live SQLite catalog opened read-only and the maintainers’ current repository README, release endpoint, and latest commit endpoint. This packet contains 60 unique existing project IDs: 12 Decompilation, 20 Recompilation, 12 Reimplementation, and 16 API Translation. Eight repository candidates flagged by the preceding catalog audit (RingOut, NFS Most Wanted Mac, DOAXVB-RE, NWiiRecomp, Tomodachi Life, Def Jam FFNY, Fukami, and Snap64) are included because they are existing catalog rows that need current primary-source checks.

GitHub star counts served only as an initial sampling signal. The initial GraphQL snapshot covered 1,964 of 2,114 unique catalog repositories in these four kinds; three query chunks failed on repositories GitHub could not resolve. Every final sample repository was then checked directly through the GitHub repository API, and every final README, release list, and latest-commit request succeeded. The selected star count and API URL are preserved in `github-repo-metadata.json` and in each project’s source records. Stars did not support status, compatibility, or completion claims.

The patch refreshes `updated` to the catalog review date for all 60 actually reviewed rows and appends current, exact README/release/commit/API source records. Fourteen rows receive status corrections or clarifications; two also receive summary changes, two platform changes, and one source-availability correction. For the other 46, the status/platform/method/summary were checked and remain materially accurate.

## Holds and evidence boundaries

- No sampled project is held from the patch: all 60 IDs exist in the live catalog and all selected public repositories responded to the current source checks.
- The ZeldaRET Ocarina of Time progress URL linked from the README returned HTTP 403 when opened directly and yielded no readable progress rows in the web view. The Ocarina status is therefore based only on its own README, which explicitly says the project is a work in progress. No percentage, completion claim, or external tracker finding is used.
- Def Jam FFNY has a `v0.9.2` tag but the latest published GitHub release remains `v0.9.1`. The patch labels v0.9.2 as a tag only and does not claim it was published as a release.
- DOAXVB-RE’s maintainer ledger reports full playability for a local v0.4 Beta build. Its current public v0.6.0 Beta RC1 has no full play session documented; macOS and Linux checks reach settings/save-select menus. The patch does not transfer v0.4’s playability claim to RC1.
- OpenRA’s newest release entry is `playtest-20261011`, marked prerelease; the last stable release remains `release-20250330`. PS2Recomp v0.4 and WinObjC 0.2.180220 are also prereleases. The patch preserves those labels.
- Amethyst’s repository API `pushed_at` is 2026-10-15, while GitHub’s latest-commit endpoint lists a default-branch commit dated 2026-09-21. These are recorded separately as repository push activity and latest listed commit; neither is used as a compatibility or project-status claim.
- A repository push or latest commit is activity evidence only. Status wording comes from maintainer README/status/release evidence. Full playability and completion statements are limited to the specific version and tested scope named by the maintainer.

## Project decisions

Activity fields below are kept separate: “repository pushed” is GitHub’s repository metadata timestamp; “latest commit listed” is the commit endpoint result. Release dates and status wording are separate evidence.

### `jadx-android-bytecode-decompiler` — JADX
- Sample: Decompilation / Software; 50,994 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; v1.5.6 was released 2026-07-10 and source changes continued through 2026-10-14. The maintainers warn that decompilation can be incomplete and produce errors.
- Latest GitHub release entry: `v1.5.6` (2026-07-10).
- Activity: repository pushed 2026-10-14; latest commit listed 2026-10-14 `c4c45fde27` — feat(gui): docking support using ModernDocking (PR #3012).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `ilspy-dotnet-decompiler` — ILSpy
- Sample: Decompilation / Software; 26,666 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; v11.1 was released 2026-09-23 and source changes continued through 2026-10-14.
- Latest GitHub release entry: `v11.1` (2026-09-23).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-14 `dad8a25ed2` — Bump ossf/scorecard-action from 2.4.4 to 2.5.0 (#4243).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `diablo-devilution-source-reconstruction` — Devilution: Diablo Source Reconstruction
- Sample: Decompilation / Game; 9,004 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Source reconstruction project; current maintenance moved to DevilutionX
- Latest GitHub release entry: `0.10.0` (2019-07-21; prerelease).
- Activity: repository pushed 2025-09-15; latest commit listed 2025-09-15 `9f01757243` — Remove dead link.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `sm64-decomp` — Super Mario 64 Decompilation
- Sample: Decompilation / Game; 8,885 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Made “Complete” precise: the README calls the decompilation full for five named regional releases while saying documentation/naming work continues.
- Reviewed status evidence: The README calls this a full decompilation for Japan, North America, Europe, Shindou, and iQue releases; naming and source documentation remain in progress.
- Latest GitHub release entry: `irix2` (2019-08-25).
- Activity: repository pushed 2024-02-04; latest commit listed 2023-08-17 `9921382a68` — ique hotfix.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `oot-decomp` — Ocarina of Time Decompilation
- Sample: Decompilation / Game; 5,593 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Removed the contradictory “Complete” status. The current README explicitly says WIP and identifies continuing modding/shiftability limits; the linked external progress tracker was not used.
- Reviewed status evidence: Work in progress. The README says the repository is a WIP and warns that the codebase is still evolving and some ROM sections may not yet be shiftable for modding. It lists supported ROM build targets and says the project is not a PC port.
- Latest GitHub release entry: No GitHub release entry listed.
- Activity: repository pushed 2026-10-14; latest commit listed 2026-10-14 `8a3634ca77` — [cc0 oot] EnKusa (#2819).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `pokered` — Pokémon Red and Blue Disassembly
- Sample: Decompilation / Game; 4,994 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Replaced the unsupported “Complete” label with the concrete ROM/debug/patch outputs the current README documents; no gameplay or completion claim is attributed to maintainers.
- Reviewed status evidence: The README identifies this as a Pokémon Red and Blue disassembly and lists U.S. ROM build outputs, a debug build, and patches. It does not state a project-completion or gameplay-compatibility milestone.
- Latest GitHub release entry: No GitHub release entry listed.
- Activity: repository pushed 2026-10-06; latest commit listed 2026-10-06 `af51989971` — Update hardware.inc to 6.0.0.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `pycdc-python-bytecode-decompiler` — Decompyle++ (pycdc)
- Sample: Decompilation / Software; 4,651 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Removed “active” as a project-status claim because the README offers no maturity statement. Kept the absent release and absent version matrix facts.
- Reviewed status evidence: No GitHub release is listed. The README describes pycdc and pycdas for Python bytecode decompilation and disassembly, but does not publish a supported-bytecode-version matrix or a maturity claim; the repository is not archived.
- Latest GitHub release entry: No GitHub release entry listed.
- Activity: repository pushed 2026-04-07; latest commit listed 2026-04-07 `b428976097` — Merge pull request #599 from daiche330-png/wip/python311-exception-opcodes-20260405.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `pokeemerald` — Pokémon Emerald Decompilation
- Sample: Decompilation / Game; 3,597 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: In progress
- Latest GitHub release entry: No GitHub release entry listed.
- Activity: repository pushed 2026-10-01; latest commit listed 2026-10-01 `731ad5bfd6` — Report SaveBlock1/SaveBlock2/PokemonStorage usage (#2364).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `opengoal-jak-and-daxter` — OpenGOAL: Jak and Daxter Native PC Port
- Sample: Decompilation / Game; 3,575 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; build and play workflow documented
- Latest GitHub release entry: `v0.3.9` (2026-10-15).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-16 `32c3cf7397` — github: add AI policy, contributing gidelines, and a PR template that all xref each other (#4447).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `lego-island-decomp` — LEGO Island Decompilation
- Sample: Decompilation / Game; 3,491 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Complete decompilation for English version 1.1, per README
- Latest GitHub release entry: `continuous` (2026-09-18).
- Activity: repository pushed 2026-09-18; latest commit listed 2026-09-18 `8119bd0136` — Revert PDB changes (#1806).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `reko-binary-decompiler` — Reko
- Sample: Decompilation / Software; 2,618 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; version 0.12.5 was released 2026-09-29 and source changes continued through 2026-10-14. The supported-binaries matrix gives architecture and format maturity tiers; not every listed target has stable decompilation.
- Latest GitHub release entry: `version-0.12.5` (2026-09-29).
- Activity: repository pushed 2026-10-14; latest commit listed 2026-10-14 `63c497aef0` — Fix: slnx file was incorrect..
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `pokecrystal` — Pokémon Crystal Disassembly
- Sample: Decompilation / Game; 2,528 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Replaced the unsupported “Complete” label with the concrete ROM variants the current README says the disassembly builds; no gameplay or completion claim is attributed to maintainers.
- Reviewed status evidence: The README identifies this as a Pokémon Crystal disassembly and lists build outputs for U.S. versions 1.0 and 1.1, the Australian release, and patches. It does not state a project-completion or gameplay-compatibility milestone.
- Latest GitHub release entry: No GitHub release entry listed.
- Activity: repository pushed 2026-10-06; latest commit listed 2026-10-06 `3bc8daa417` — Update hardware.inc to 6.0.0.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `zelda64recomp` — Zelda 64: Recompiled
- Sample: Recompilation / Game; 7,437 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Released; Ocarina of Time support in development
- Latest GitHub release entry: `v1.2.2` (2025-08-27).
- Activity: repository pushed 2026-09-25; latest commit listed 2026-09-25 `b65c482ed6` — Formatting changes in README (#731).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `unleashed-recompiled` — Unleashed Recompiled
- Sample: Recompilation / Game; 5,193 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Released
- Latest GitHub release entry: `v1.0.3` (2025-04-03).
- Activity: repository pushed 2026-06-29; latest commit listed 2026-06-29 `cf829a9eca` — Update username in credits (#1770).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `ps2recomp-static-recompiler-framework` — PS2Recomp
- Sample: Recompilation / Software; 3,523 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Marked v0.4 as a prerelease and preserved the README’s experimental, per-game status framing; no broad compatibility is inferred.
- Reviewed status evidence: Experimental. The latest GitHub release is v0.4, published as a prerelease on 2026-04-05. The README describes static recompilation of PS2 ELF binaries to C++; it does not establish broad game compatibility.
- Latest GitHub release entry: `v0.4` (2026-04-05; prerelease).
- Activity: repository pushed 2026-10-14; latest commit listed 2026-10-14 `eb8688b2fd` — Emulate PS2 float semantics for EE FPU results and DIV.S/RSQRT.S (#272).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `banjo-recompiled` — Banjo: Recompiled
- Sample: Recompilation / Game; 2,046 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Latest release v1.0.2 (2026-07-31). The README requires the North American 1.0 Nintendo 64 game image and lists macOS 13.0+; no game assets are included.
- Latest GitHub release entry: `v1.0.2` (2026-07-31).
- Activity: repository pushed 2026-09-25; latest commit listed 2026-09-25 `dbd52154bc` — Update formatting in README (#319).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `skate3-recomp` — Skate 3 Recomp
- Sample: Recompilation / Game; 1,635 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Latest release v2.0.2 (2026-07-24). The README says Windows is most tested, Linux next, and the macOS ARM build is experimental and more prone to issues.
- Latest GitHub release entry: `v2.0.2` (2026-07-24).
- Activity: repository pushed 2026-07-24; latest commit listed 2026-07-24 `f6e0ae87fd` — render: declare copy-source usage on the readback source textures.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `soulcalibur-ii-gamecube-recomp` — Soulcalibur II GameCube Recompilation (RingOut)
- Sample: Recompilation / Game; 162 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Fully playable per README; networking remains experimental
- Latest GitHub release entry: `v1.6.9` (2026-10-15).
- Activity: repository pushed 2026-10-15; latest commit listed 2026-10-15 `6aa0179f8e` — Release 1.6.9: opens fullscreen by default, faster US disc, pause menu polish.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `nfs-most-wanted-pc-macos-recomp` — Need for Speed: Most Wanted (2005 PC) — Apple Silicon Recompilation
- Sample: Recompilation / Game; 87 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: v3.0.3 released 2026-10-15; the README reports testing at 4K/60 on an M5 Pro MacBook Pro.
- Latest GitHub release entry: `v3.0.3` (2026-10-15).
- Activity: repository pushed 2026-10-15; latest commit listed 2026-10-15 `0c7e4f57e3` — 3.0.3: Auto resolution reads your screen, MacBook Pro sizes below the notch (#10, Simonxdd), 120 Hz on the screen the game is on, and a guard for the start-up crash in #8.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `doaxvb-xbox-recomp` — Dead or Alive Xtreme Beach Volleyball Recompilation
- Sample: Recompilation / Game; 166 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Updated the version-specific playability status from the maintainer’s public ledger and added current macOS/Linux build platforms. v0.4 test evidence does not transfer to the unplayed v0.6.0 RC1 build.
- Reviewed status evidence: The maintainer's public status page reports v0.4 Beta playable through every stage, the Casino, Hotel and Shops, save/reload, and a vacation rollover; 60 FPS is reported, while days 11–13 are inferred from shared code. The latest public build is v0.6.0 Beta RC1 (pre-release; re-uploaded 2026-10-10); a full RC1 play session is not reported. macOS and Linux smoke runs reach the save-select/settings menus only. Full offline-route acceptance and original lighting/camera fidelity remain open.
- Latest GitHub release entry: `v0.6.0-rc1` (2026-10-09; prerelease).
- Activity: repository pushed 2026-10-14; latest commit listed 2026-10-14 `e282a3fc15` — Merge pull request #125 from NoRain211/codex/metadata-upkeep.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `nintendo-wii-gc-recomp-toolkit` — NWiiRecomp GameCube/Wii Static Recompilation Toolkit
- Sample: Recompilation / Software; 31 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Refined the generic test bring-up wording to the README’s specific latest Mario Party 7 boot milestone and kept the no-general-compatibility caveat.
- Reviewed status evidence: Work in progress. The README reports architecture bring-up on Need for Speed: Hot Pursuit 2, Mario Party 7, and Silent Hill: Shattered Memories; its latest update says Mario Party 7 boots past the Nintendo and Hudson logos to the real-time 3D intro sequence. These project-reported tests do not establish broad compatibility for other GameCube/Wii titles.
- Latest GitHub release entry: No GitHub release entry listed.
- Activity: repository pushed 2026-08-07; latest commit listed 2026-08-07 `595f176f1d` — Update README.md.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `tomodachi-ltd-switch-recomp` — Tomodachi Life: Living the Dream Recompilation
- Sample: Recompilation / Game; 34 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Updated platform, summary, and status to match the current README’s explicit Suyu-derived runtime dependency and Linux/Windows maturity. Removed any implication that the present build is an emulator-free native port.
- Reviewed status evidence: Experimental static recompilation. The README says audio, video, the opening, and the start of the game have been tested; an end-to-end installer run on a fresh clone started the game. Testing is limited to one machine, and parts of the game may remain unplayed. The current executable still depends on Suyu-derived runtime components for graphics, audio, and Switch services, so it is not an emulator-free native port. Linux instructions are tested on Arch/EndeavourOS, Fedora is reported working, and Windows 10/11 is experimental. It requires the user’s game NSP v1.0.0 and keys; no GitHub releases are listed.
- Latest GitHub release entry: No GitHub release entry listed.
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-16 `2226e51a32` — docs: remove emulator-free claim from illustrations.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `def-jam-fight-for-ny-xbox-recomp` — Def Jam: Fight for NY Xbox Recompilation
- Sample: Recompilation / Game; 57 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Added the current README and tested-route caveat, and clarified that v0.9.2 is a repository tag without a matching release entry; v0.9.1 remains the latest published release.
- Reviewed status evidence: Latest published GitHub release is v0.9.1 (2026-10-15); a v0.9.2 repository tag exists but has no matching GitHub release entry. The README calls the port playable and describes menus, fights, and tested Story routes; the maintainer does not claim a complete Story campaign. The README requires a user-supplied supported U.S. Xbox dump and documents Windows, macOS, Linux, and Steam Deck builds.
- Latest GitHub release entry: `v0.9.1` (2026-10-15).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-16 `5b0251a6c9` — docs: agents prune merged branches; GitHub deletes remote heads on merge (#33).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `ridge-racer-v-ps2-recomp` — Ridge Racer V Recompilation (Fukami)
- Sample: Recompilation / Game; 63 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: v1.1.0 release; README documents 60 fps gameplay and desktop builds.
- Latest GitHub release entry: `v1.1.0` (2026-10-14).
- Activity: repository pushed 2026-10-14; latest commit listed 2026-10-14 `2ecb2810dd` — Merge pull request #4 from danisandoval/claude/modest-faraday-lngowl.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `snap64-recomp` — Snap64 Recomp
- Sample: Recompilation / Game; 109 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Latest release v1.1.4 (2026-10-11). The README accepts only the US Nintendo 64 cartridge image; it lists macOS 14+ but does not specify a Mac CPU architecture.
- Latest GitHub release entry: `v1.1.4` (2026-10-11).
- Activity: repository pushed 2026-10-15; latest commit listed 2026-10-15 `a2198cd55c` — The docs point to the published mods.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `mario-kart-wii-wiicompiled-pc` — Mario Kart Wii WiiCompiled PC Port
- Sample: Recompilation / Game; 1,161 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Playable development build; the maintainer says gameplay should be playable and ghost runs match original physics, while compatibility, rendering, networking, and performance remain under active work. Windows 10/11 and macOS 12+ are listed; only a clean PAL RMCP01 dump is accepted.
- Latest GitHub release entry: `v0.2.34` (2026-10-08).
- Activity: repository pushed 2026-10-09; latest commit listed 2026-10-09 `bd219e38a3` — fix: drive compile progress from ninja's [done/total] counter (#285).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `link-awakening-switch-recomp` — Link's Awakening Switch Recompilation
- Sample: Recompilation / Game; 1,003 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Beta; latest release beta_4 (2026-10-15). The README identifies macOS as Apple silicon only with Metal/MetalFX, Windows/Linux as x86-64 Vulkan builds, and Linux aarch64 as built but untested on real hardware.
- Latest GitHub release entry: `beta_4` (2026-10-15).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-15 `6244eb9d4d` — Linux smoketest: a real setup first, and no false failures from pipefail.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `god-of-war-ii-ps2-web-recomp` — God of War II PS2 Web Recompilation
- Sample: Recompilation / Game; 964 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Menus, cutscenes and combat are reported working in measured scenes; the README does not claim a complete campaign playthrough.
- Latest GitHub release entry: No GitHub release entry listed.
- Activity: repository pushed 2026-10-15; latest commit listed 2026-10-15 `c628ae8432` — Scripted presses can push the left stick.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `castlevania-sotn-psx-symphony-recomp` — Symphony of the Night PSX Recompilation
- Sample: Recompilation / Game; 896 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Open beta; owner warns that serious game-breaking bugs may occur and says stable v1.0 has not been released. Latest checked release v0.5.1b (2026-08-19).
- Latest GitHub release entry: `v0.5.1b` (2026-08-19).
- Activity: repository pushed 2026-09-27; latest commit listed 2026-09-27 `c94a9096ba` — Update windows_initial_build.bat.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `goldeneye-007-xbox360-recomp` — GoldenEye 007 (Xbox 360/XBLA) Recompilation
- Sample: Recompilation / Game; 895 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Work in progress; latest release is Goldeneye1.2.4. This targets the Xbox 360 remaster and is separate from N64 GoldenEye projects.
- Latest GitHub release entry: `Goldeneye1.2.4` (2026-06-18).
- Activity: repository pushed 2026-06-18; latest commit listed 2026-06-18 `fdee4d1f75` — Mouse fixes and keybind fixes.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `wind-waker-hd-recompilation` — The Wind Waker HD Recompilation
- Sample: Recompilation / Game; 696 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Latest release v0.3.0. Native builds are documented for Windows, Apple Silicon macOS, Linux, and Android arm64. Accepts Wii U USA or Europe version 0 (disc or eShop, without the update) and checks the user-supplied dump before use. The Switch port is a separate community project.
- Latest GitHub release entry: `v0.3.0` (2026-10-15).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-15 `f10b56621e` — Changelog: v0.3.0.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `donkey-kong-64-pc-recomp` — Donkey Kong 64 Recompiled (PC)
- Sample: Recompilation / Game; 510 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: v1.0.3 release; the project documents PC builds, but does not make a complete-playthrough claim.
- Latest GitHub release entry: `v1.0.3` (2026-09-30).
- Activity: repository pushed 2026-09-30; latest commit listed 2026-09-30 `3b0c36d3fb` — Fix some minor macro issues.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `reactos-windows-nt-os-reimplementation` — ReactOS
- Sample: Reimplementation / Software; 18,222 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: The project says its current focus is Windows Server 2003 compatibility. The README links Alpha-stage releases and warns about serious problems and possible disk corruption.
- Latest GitHub release entry: `0.4.16-release` (2026-08-29).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-07-15 `7772194f8f` — [RAPPS] Rework certificate pinning.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `openra` — OpenRA
- Sample: Reimplementation / Game; 17,578 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Separated the last stable release from the newer public playtest prerelease. The prerelease is not described as a stable release.
- Reviewed status evidence: The README describes OpenRA as a libre RTS engine for early Westwood classics and lists Windows, Linux, and macOS hosts. The latest stable release is release-20250330 (2025-03-30); a public playtest-20261011 prerelease was published 2026-10-11.
- Latest GitHub release entry: `playtest-20261011` (2026-10-11; prerelease).
- Activity: repository pushed 2026-10-11; latest commit listed 2026-10-11 `e180e3225f` — Check for success of the code signing..
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `openrct2` — OpenRCT2
- Sample: Reimplementation / Game; 16,442 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; latest release v0.5.6 was published 2026-10-10, with macOS builds and official macOS CMake instructions. Android is also an official APK build; the guide specifies Android 7.0/API 24+, manual installation and Data/ObjData copied from an original Windows RCT2 or RCT Classic installation. Phone controls are not optimized; no ABI list or device test matrix is published.
- Latest GitHub release entry: `v0.5.6` (2026-10-10).
- Activity: repository pushed 2026-10-14; latest commit listed 2026-10-14 `2107821cbe` — Fix #27286: Headless servers do not write chat logs (#27287).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `freerdp-rdp-protocol-reimplementation` — FreeRDP (Microsoft Remote Desktop Protocol implementation)
- Sample: Reimplementation / Software; 13,839 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; FreeRDP 3.32.2 was released on 2026-10-12 and the repository received source updates through 2026-10-15. This is the FreeRDP release number; upstream does not identify one single RDP wire-protocol version as its target.
- Latest GitHub release entry: `3.32.2` (2026-10-12).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-16 `00bade26ef` — Merge pull request #13724 from congzhangzh/emscripten-build-fixes.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `webamp-winamp-2-browser-reimplementation` — Webamp
- Sample: Reimplementation / Software; 11,298 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: The project changelog identifies v2.3.1 as the current Webamp release. The README documents current Edge, Firefox, Safari, and Chrome support and says Internet Explorer is unsupported. The skin documentation states that classic Winamp 2 skins are rendered pixel-for-pixel as in Winamp 2.9; the project does not claim compatibility with Winamp binary plugins or the entire desktop product.
- Latest GitHub release entry: No GitHub release entry listed.
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-16 `00c7349985` — Open the event dashboard in production, behind a token.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `devilutionx` — DevilutionX
- Sample: Reimplementation / Game; 9,869 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; latest stable release v1.5.5 (2025-10-30). The README lists a macOS x86_64 build workflow, and repository development continued through 2026-10-15. Android APKs and iOS IPA builds are published; users must supply Diablo/Hellfire MPQ files. Android’s in-app data importer requires Android 7.1.1+, while the optional Inno Setup Extractor method requires Android 8+; neither helper floor is stated as the app minimum. The iOS deployment target is 16.3.
- Latest GitHub release entry: `1.5.5` (2025-10-30).
- Activity: repository pushed 2026-10-15; latest commit listed 2026-10-15 `94433061f8` — Free pointer returned by SDL_GetDisplays().
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `openmw` — OpenMW
- Sample: Reimplementation / Game; 6,634 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; latest tagged release is openmw-0.51.0 (2026-06-19). The current development README identifies 0.52.0 and lists macOS 14.8+; that minimum is for the development branch.
- Latest GitHub release entry: `openmw-0.51.0` (2026-06-19).
- Activity: repository pushed 2026-10-15; latest commit listed 2026-10-15 `9ec458174f` — Merge branch 'fix/shoot-animation-gap' into 'master'.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `heroes3-vcmi-mobile` — VCMI: Heroes of Might and Magic III mobile engine
- Sample: Reimplementation / Game; 5,949 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Android 5.0+ is required; Google Play, F-Droid, GitHub APKs, and unstable daily builds are documented. F-Droid currently offers arm64 and x86_64 variants. The iOS guide supports iOS 12+ and TestFlight or sideload installation, and says iOS 10/11 require a source build. Original Heroes III data is required. The docs do not list Android device models.
- Latest GitHub release entry: `1.7.5` (2026-08-15).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-16 `ffff9bba18` — Merge pull request #8011 from tonicmuroq/upstream-pr/scroll-spell-crossover.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `jspaint-classic-microsoft-paint-reimplementation` — JS Paint
- Sample: Reimplementation / Software; 7,896 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: The maintainer describes JS Paint as a pixel-perfect remake of classic MS Paint and says it recreates its tools and menus to a high degree of fidelity. The README says some tools remain incomplete; full clipboard support in the web app requires a browser with the Async Clipboard API for images (the README's Chrome 76+ note is historical). The Electron build is cross-platform, but its README warns that Electron is out of date and automatic updates are not implemented. The repository's latest push checked on 2026-10-15 was 2026-07-19.
- Latest GitHub release entry: `v1.1.0` (2026-04-10).
- Activity: repository pushed 2026-07-19; latest commit listed 2026-04-10 `53be67ab8c` — Enable macOS build.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `julius-caesar3-android` — Julius: Caesar III Android reimplementation
- Sample: Reimplementation / Game; 3,396 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Android installation is available through Google Play, with a separate source-build guide. The game requires Caesar III files patched to 1.0.1.0; the Android guide documents full touch support, limited mouse support, and Samsung DeX compatibility. The project does not state a minimum Android API/CPU ABI or a device-model test matrix.
- Latest GitHub release entry: `v1.8.0` (2025-07-31).
- Activity: repository pushed 2026-09-12; latest commit listed 2026-09-12 `34d1ecd54b` — Fix undo corruption when merged house becomes vacant.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `uxplay-airplay-receiver-reimplementation` — UxPlay (AirPlay mirroring and audio receiver)
- Sample: Reimplementation / Software; 3,216 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active continuation at FDH2/UxPlay; the latest tagged release is v1.73.7 (2026-09-04), while the current README labels 1.74 as experimental. It supports AirPlay mirroring and audio, using the AirPlay 2 legacy protocol; the README says some features are missing and AirPlay 2 multi-room audio is unsupported. The project source history cites RPiPlay-derived components, so this is not presented as a clean-room implementation.
- Latest GitHub release entry: `v1.73.7` (2026-09-04).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-16 `4e95a9ec01` — keepAliveLowPower plist item is boolean  (from @danieldhaiti ).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `heroes2-fheroes2-android` — fheroes2: Heroes of Might and Magic II Android port
- Sample: Reimplementation / Game; 3,506 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: The Android guide specifies a minimum of Android 5.1 and documents the APK, full-screen mode, and touch interaction. Players can import original Heroes II data or use the demo. The guide does not list CPU ABIs or specific tested devices.
- Latest GitHub release entry: `1.1.17` (2026-06-30).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-16 `e397b1faa5` — Update translation files (#11060).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `proton` — Proton
- Sample: API Translation / Software; 33,064 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; latest listed stable branch release Proton 11.0-2e was published 2026-10-09, alongside the Experimental channel. WineD3D can be selected as an OpenGL backend in place of Vulkan-based DXVK for supported Direct3D versions.
- Latest GitHub release entry: `proton-11.0-2e` (2026-10-09).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-09-23 `bc896217ff` — proton: Copy some data in appcache instead of creating symbolic link..
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `winlator-android-windows-compatibility` — Winlator: Windows Applications on Android
- Sample: API Translation / Software; 19,481 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: The latest stable release found is v11.2.0 (2026-08-19), with an Android APK. The repository was pushed on 2026-09-22 and remains active. The project documents Wine with Box86/Box64 plus Mesa/Turnip and Direct3D translation components; it does not claim that all Windows applications or games work.
- Latest GitHub release entry: `v11.2.0` (2026-08-19).
- Activity: repository pushed 2026-09-22; latest commit listed 2026-09-22 `b6b2259158` — Update README.md.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `wgpu-webgpu-native-graphics-api` — wgpu
- Sample: API Translation / Software; 18,306 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; release v30.0.1 was published 2026-08-22, and the trunk branch's latest recorded commit was 2026-10-14. The README marks Vulkan, Metal, and D3D12 as first-class native backends, with OpenGL downlevel/best-effort and portability-layer requirements on some platforms.
- Latest GitHub release entry: `v30.0.1` (2026-08-22).
- Activity: repository pushed 2026-10-14; latest commit listed 2026-10-14 `14328f4991` — Validate mesh draws in render bundles against the multiview mask (#10496).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `dxvk` — DXVK
- Sample: API Translation / Software; 18,301 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; DXVK 3.1.1 was released 2026-09-15. Current 3.x driver requirements and Wine winevulkan requirements are listed in the maintainer wiki.
- Latest GitHub release entry: `v3.1.1` (2026-09-15).
- Activity: repository pushed 2026-10-15; latest commit listed 2026-10-15 `e8c80a8d1c` — [d3d9] Don't enable point scaling with weird projection matrices.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `ge-proton-linux-windows-compatibility` — GE-Proton
- Sample: API Translation / Software; 15,173 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; GE-Proton11-9 was released 2026-10-14. The maintainer publishes architecture-specific x86-64 and AArch64 builds. It is a community Proton/Wine build with independent patches and bundled components; the maintainer warns that launching non-Steam games outside Steam is supported only through umu.
- Latest GitHub release entry: `GE-Proton11-9` (2026-10-14).
- Activity: repository pushed 2026-10-14; latest commit listed 2026-10-14 `01bfbf67b3` — GE-Proton11-9 Released.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `whisky-macos-wine-wrapper` — Whisky
- Sample: API Translation / Software; 15,089 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Archived by its owner on 2025-05-11 and no longer actively maintained. The latest release is v2.3.5 (2025-04-05); the README lists Apple silicon and macOS Sonoma 14+.
- Latest GitHub release entry: `v2.3.5` (2025-04-05).
- Activity: repository pushed 2025-05-11; latest commit listed 2025-04-09 `fd5480a76b` — Maintenance Notice.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `zluda-cuda-on-amd` — ZLUDA
- Sample: API Translation / Software; 15,082 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active hobby project with experimental pre-release builds; v7-preview.15 was published 2026-10-08. The maintainer said commercial funding ended and development continues as a weekend project (2026-06-29). Workload coverage varies: the maintainer reports Blender and llama.cpp support, early PhysX support, and ongoing PyTorch work.
- Latest GitHub release entry: `v7-preview.15` (2026-10-08; prerelease).
- Activity: repository pushed 2026-10-12; latest commit listed 2026-10-08 `86ec0a93cc` — Support the sink symbol _ in place of vector operands (#683).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `darling-macos-linux-compatibility` — Darling
- Sample: API Translation / Software; 13,474 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; latest release v0.1.20260608 was published 2026-06-09 UTC, and the default branch had commits through 2026-09-06. The README reports many command-line tools functional, GUI support in active development, and an initial Metal backend powered by Vulkan translation.
- Latest GitHub release entry: `v0.1.20260608` (2026-06-09).
- Activity: repository pushed 2026-10-12; latest commit listed 2026-09-06 `60ba801dec` — Merge pull request #1779 from jlucfarias/fix-hitoolbox-build.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `gamenative-android-pc-games` — GameNative
- Sample: API Translation / Software; 11,350 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: The latest release found is v1.3.0 (2026-10-14), with a standard Android APK and a separate legacy XR APK. The maintainer says the project is early: not every game runs, and some titles need configuration changes. The Android build configuration lists arm64-v8a modern variants and arm64-v8a/armeabi-v7a legacy variants.
- Latest GitHub release entry: `v1.3.0` (2026-10-14).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-16 `358117b5e2` — Hide .DepotDownloader and .DownloadInfo from guest directory listings (#2156).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `microsoft-winobjc-ios-api-windows-bridge` — Windows Bridge for iOS (WinObjC)
- Sample: API Translation / Software; 6,221 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Corrected the false claim that no GitHub release exists, and recorded the actual 2018 prerelease. Archive state remains supported by GitHub repository metadata.
- Reviewed status evidence: Archived; GitHub lists prerelease 0.2.180220 (2018-02-21). The README describes an Objective-C development environment and selected iOS API support for Windows UWP apps; its setup instructions target Windows 10 build 10586+ and Visual Studio 2017.
- Latest GitHub release entry: `0.2.180220` (2018-02-21; prerelease).
- Activity: repository pushed 2022-11-28; latest commit listed 2022-11-28 `94f6b5bfdc` — Auto merge mandatory file pr.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `moltenvk` — MoltenVK
- Sample: API Translation / Software; 5,886 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; v1.4.2 was released 2026-07-24. Release notes include 2026 Metal/SPIRV-Cross updates and macOS GPU fixes; Khronos’ Apple-silicon conformance tracker records v1.4.2 testing on macOS 26.5.2. The README also lists macOS, iOS, tvOS, visionOS, simulator and Mac Catalyst targets, with macOS 12 and iOS/tvOS 15 deployment floors; no specific device-test matrix is stated.
- Latest GitHub release entry: `v1.4.2` (2026-07-24).
- Activity: repository pushed 2026-10-13; latest commit listed 2026-10-13 `5189f8de86` — Merge pull request #2832 from KhronosGroup/add-contributing-file.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `angle` — ANGLE
- Sample: API Translation / Software; 4,100 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active; README lists OpenGL ES 2.0–3.2 backend coverage
- Latest GitHub release entry: No GitHub release entry listed.
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-16 `ea2608deb1` — Skip packed encoder uniform test on Linux/NVIDIA.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `vkd3d-proton` — vkd3d-proton
- Sample: API Translation / Software; 3,005 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Active Proton graphics component; latest listed release v3.0.1 was published 2026-05-06.
- Latest GitHub release entry: `v3.0.1` (2026-05-06).
- Activity: repository pushed 2026-10-14; latest commit listed 2026-10-13 `0bb64c56c8` — dxil-spirv: Update submodule..
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `god-of-war-iii-remastered-ps4-api-translation` — God of War III Remastered PS4 PC API-Translation Port
- Sample: API Translation / Game; 2,691 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Replaced the now-false statement that no v2.4.0 release was listed. The v2.4.0 release is dated 2026-10-16 and the README still labels Linux AppImage experimental.
- Reviewed status evidence: The Windows v2.4.0 release was published 2026-10-16 and includes beta ultrawide resolutions; the release also provides a Linux AppImage, which the README labels experimental. The README reports one completed playthrough on a specific test setup and documents issues on other setups, so compatibility remains hardware-specific.
- Latest GitHub release entry: `windows-v2.4.0` (2026-10-16).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-16 `7180a7bafa` — Chore(Release): bumped version to 2.4.0 (#460).
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `amethyst-minecraft-java-android` — Amethyst: Minecraft Java Edition for Android
- Sample: API Translation / Game; 2,297 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Corrected** — Removed source-activity wording from status; latest release and README compatibility scope stay distinct from push/commit timestamps.
- Reviewed status evidence: Latest published Android release is 1.1.7 (2026-07-31). The README says the launcher runs Minecraft Java Edition versions from early beta through snapshot 26.3 and supports Forge and Fabric. It publishes no device test matrix or universal performance claim.
- Latest GitHub release entry: `1.1.7` (2026-07-31).
- Activity: repository pushed 2026-10-15; latest commit listed 2026-09-21 `330c6eae31` — fix(SDL/Controller): Incorrect numbers copy pasted.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).

### `dxmt` — DXMT
- Sample: API Translation / Software; 1,231 GitHub stars on the 2026-10-16 repository API check (sampling signal only).
- Decision: **Kept** — current maintainer README and release evidence support the existing factual status; only the review date and current source records are refreshed.
- Reviewed status evidence: Experimental macOS/Wine implementation of Direct3D 10/11; v0.80 (2026-04-23) remains the latest upstream release listed on 2026-10-16, with v0.81 still described as planned. Maintainer commits through 2026-10-12 include D3D11 Map handling and D3D12 feature-query fixes. The issue tracker continues to document compatibility and performance bugs, so release and source activity do not imply broad compatibility.
- Latest GitHub release entry: `v0.80` (2026-04-23).
- Activity: repository pushed 2026-10-16; latest commit listed 2026-10-16 `50102f994b` — fix(nvngx): allow resources stored as `void *` to be read as D3D resources.
- Source links: recorded per project in `project-updates-reviewed.json` (README, release or no-release page, exact latest commit, repository API; plus direct status/validation documents where used).
