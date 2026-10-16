# Long-tail compatibility and reverse-engineering projects

Reviewed on 2026-10-15. This batch focuses on gaps outside the main Nintendo/console-decomp and high-star recompilation searches: N-Gage static recompilation, Game Boy Advance and 3DS projects, Sega Saturn reverse engineering, multi-title vintage-computer runtimes, and clean-room operating-system reimplementation.

## Evidence and inclusion rules

- Discovery used web search, GitHub project/topic pages, and searches prompted by platform-specific decompilation community lists. Every staged claim was checked against a maintainer repository, README, progress log, release page, or official GitHub API endpoint. Search pages were discovery leads only.
- Repository language metadata was checked through each project's GitHub Languages API. Unknown language values are left blank; repository creation time was not used to infer `project_started`.
- Reimplementation, decompilation, and static recompilation are kept separate. A project described as a source port or rewritten game behavior is classified as `Reimplementation`; only work explicitly described as a matching source/binary reconstruction is `Decompilation`; ARMv4 instructions translated ahead of runtime into C/native code are `Recompilation`.
- Statuses retain each maintainer's maturity boundary. Early boot, a blank frame, a verified format, or a partial campaign are not described as playable when the maintainer does not make that claim.
- All original game files, firmware, or assets are user-provided where the project requires them. This batch stores only project/source URLs and does not copy media or game files.
- IDs, repository/game pairs, and target names were compared with `data/catalog.sqlite`, all JSON research manifests under `data/research/`, and all JSON follow-up batches under `work/research-followup/`. No SQLite or generated `docs/catalog.json` changes were made.

## Staged projects

`new-projects.json` contains twelve records:

1. **ALIS** — reverse-engineered Silmarils virtual-machine runtime for multiple old game releases. The README includes a title-by-original-version compatibility table and calls out unknown/non-working combinations.
2. **Azel / Panzer Dragoon Saga** — a Sega Saturn reverse-engineering effort with an early semi-playable Windows section; not a matching decompilation.
3. **OpenCaptive / Captive** — C/SDL3 reimplementation, with a playable DOS-based runtime and an explicit current gameplay-verification boundary.
4. **OpenCaptive / Liberation: Captive 2** — separate game target in the same repository; format/data and city-system slices are reported, while the full campaign is not complete. These two rows share one repository and should count once in repository-star rankings.
5. **ccleste / Celeste Classic** — manually translated PICO-8 Lua gameplay in C, with PC/3DS and experimental web builds; upstream is archived.
6. **NGageRecomp** — generic N-Gage ARMv4 static-recompiler framework; deliberately represented as Software, with incomplete game bring-up and the proprietary IDA/Ghidra analysis-toolchain caveat retained.
7. **SonicN N-Gage** — game-specific proof target built on NGageRecomp; the repo remains pre-first-frame/playable, and the framework's loop proof currently renders blank/loading output.
8. **Snakes N-Gage** — second framework target; all engine functions lift and compile, but application/menu/gameplay bring-up is incomplete.
9. **OpenClaw / Captain Claw** — from-scratch C++ game reimplementation with Windows/Linux/macOS, Android, and Emscripten targets. The source README documents a playable web build and says Android runs; GitHub API shows the last push was 2022-10-24, so present maintenance is not implied.
10. **Rayman 3 Readvanced** — GBA/N-Gage behavior recreated in C# / MonoGame; the maintainer says no builds are available yet and only Windows x64 is currently supported.
11. **Sonic Advance 3** — a separate active GBA matching-decomp repository. The catalog already has the Sonic Advance 1/2 repository, which calls Sonic Advance 3 a future target; this independently maintained repository is a distinct title and was not in SQLite or any reviewed project batch.
12. **System 7 Portable Reimplementation** — clean-room, x86-bootable System 7-style OS; only small test 68K apps are verified, not real Apple applications. ARM64 has QEMU evidence and PowerPC remains experimental.

## Relevant candidates not staged

- `Project12x/sm64-port` was excluded: its README says the Saturn branch currently verifies a hello/hardware-test bootstrap and does not yet compile or run Super Mario 64 game code. It is not yet a game port.
- `sylverb/ccleste-go` was not staged separately: it is a fork of the already staged `lemon32767/ccleste`; the checked README/source tree did not establish a distinct Game & Watch Retro-go target beyond the repository description. Revisit if the fork publishes target-specific build/runtime evidence.
- `EKA2L1` is already present in the emulator-additions research batch; it was not duplicated here.
- `OpenJKDF2` and Frogger's Adventures: Temple of the Frog have existing catalog rows; no duplicate additions were staged.
- The Pokémon TCG 2 decomp/disassembly is already represented in reviewed research and was not copied into this batch.

## Maintainer source pages

- [ALIS](https://github.com/maestun/alis)
- [Azel](https://github.com/yaz0r/Azel)
- [OpenCaptive](https://github.com/yeager/OpenCaptive)
- [ccleste](https://github.com/lemon32767/ccleste)
- [NGageRecomp](https://github.com/sp00nznet/ngagerecomp), [SonicN](https://github.com/sp00nznet/sonicn-ngage), and [Snakes](https://github.com/sp00nznet/snakes-ngage)
- [OpenClaw](https://github.com/pjasicek/OpenClaw)
- [Rayman 3 Readvanced](https://github.com/RayCarrot/Rayman3Readvanced)
- [Sonic Advance 3](https://github.com/SAT-R/sa3)
- [System 7](https://github.com/Kelsidavis/System7)
