# Game-specific decompilation, recompilation, and native-port sweep — 2026-10-16

This targeted pass looked for current game-specific decompilation, static recompilation, native ports, and API-translation projects on desktop, Switch, and ARM Linux handhelds. I read the catalog's AGENTS.md and checked the SQLite catalog read-only by repository URL, repository identity, and game title, then checked JSON manifests under the catalog's data/research tree and work/research-2026-10-16. The staged records below are based on project-maintainer READMEs/releases and GitHub repository, release, and Languages APIs. GitHub activity dates are reported as pushed_at snapshots; unknown project start dates remain blank.

## Staged additions

project-additions.json contains ten project records; programming-languages.json contains official GitHub Languages API byte totals for the same ten repositories. All ten projects have public source and a project release with a platform artifact. Original game data stays user-supplied; the records state version requirements and caveats rather than implying bundled rights or full compatibility.

**Space Channel 5 Native** is a Dreamcast-to-desktop static recompilation. The maintainer says SH-4 game code is translated ahead of time into native modules, with Flycast providing device, graphics, and audio components. Preview v0.1.0-preview.2 supports only the hash-validated USA Dreamcast GDI revision. Windows gameplay/regression testing is documented; Linux and macOS have build/runtime checks but still need hands-on gameplay testing. The project README and release include Windows, Linux x64/ARM64, and macOS artifacts. This is the only decompilation/recompilation-focused new project in this sweep.

**Five EapRules Android-to-Linux ARM ports** load original Android game libraries through a Bionic/JNI compatibility layer on Linux ARM handhelds, without an Android runtime or emulator. The README statuses make the maturity distinctions explicit:
- **Modern Combat 3: Fallen Nation** targets Android v1.1.7g and has a hardware-playable campaign; a pause-menu mission-restart crash remains.
- **N.O.V.A. 3** targets Android v1.0.7 and has a maintainer-reported playable campaign on R36S/ArkOS, plus a community RG35XXH/Knulli report.
- **Mass Effect Infiltrator** targets the v1.0.58 gamepad build. Its qemu/llvmpipe harness reaches M7/7 and the maintainer confirmed textures on an R36S; audio, camera controls, saves, and a full play-through still need device testing.
- **Dead Space Mobile** targets Xperia Play Android v1.1.33. It has similar harness and R36S texture evidence; audio, camera controls, saves, and a full play-through still need device testing. The README explicitly excludes a separate ETC1 data variant that currently renders black models.
- **Real Racing 3** targets Android v2.7.0 ROW and is reported playable with front-end, 3D scene, gamepad, and sound. The maintainer reports about 15–20 FPS on RK3326-class handhelds and says the approximately 2.4 GB legacy data is no longer downloadable from the origin.

These source repos are GPL-3.0 and their PortMaster releases omit EA/Gameloft game binaries and assets. Each record names the required Android build and reflects whether a hardware play-through is established.

**Four Android-on-Switch projects** translate or wrap the original game binaries in a small Android-compatible environment for Nintendo Switch homebrew:
- **LEGO Star Wars: The Complete Saga** targets v2.0.2.02/build 20202; its 1.0.3 NRO changelog reports audio and controller fixes. Users extract libTTapp.so and game files from their copy.
- **Ys Chronicles 1** loads the original arm64-v8a app's shared libraries using a custom ELF loader, Bionic shims, and a fake JNI layer. The exact app version is not stated. Release v0.2 fixes memory, text, and dialog-interaction issues; the README still documents occasional diagonal movement on fast direction changes.
- **Subway Surfers** targets Android v3.66.1 ARM64, Unity 2022.3 IL2CPP. Release 1.0.3 reports docked-display and menu/shop navigation fixes; the README documents binary validation and asset repacking without claiming whole-game completion.
- **Angry Birds Journey** targets Android ARM64 v3.8.4, Unity 2022.3.67f2. The initial NRO release is 1.0.0; the README documents version validation and asset repacking but gives no explicit gameplay or completion claim.

These Switch repositories are MIT-licensed wrapper source (with any separately licensed vendored component noted on the LEGO record). They do not distribute proprietary game binaries or assets. Release status and functional claims are bounded to each README/changelog; no record claims full-game completion.

## Deduplication and held-out leads

No staged repository URL, normalized GitHub repository identity, or proposed ID duplicates a catalog or earlier work-manifest record. The catalog already has **N.O.V.A. 3 NX** from markaurel13/nova3-nx, an Android-game wrapper targeting Switch. EapRules/nova3-native-arm is a separate repository and target: Linux ARM handhelds/PortMaster. It is recorded separately as a distinct implementation for a different target platform, not as an alias or duplicate discovery.

**RuneCast** (LobotomyDC/RuneCast) was found as a C99 port/refactor of the public RuneScape Classic Java client for Dreamcast/IRIX, but its maintainer says it does not run on stock Dreamcast hardware yet. I left it out of additions because current target-hardware operation is not established. **MC2BPegasus-Vita** does not name a precise Android donor version and has limited status evidence. **mc3-vita** is a Vita wrapper candidate with its last source push in 2024, so it was not selected for this current-project pass. Previously cataloged projects such as The Impossible Game Vita, the TeamGDB Dragon Ball Z PSP port, and MHP3rd HD remain deduped rather than restaged.

## Language evidence and validation

The programming-language manifest records the full byte breakdown returned by each repository's official GitHub Languages API on 2026-10-16. C is the largest detected category for Space Channel 5 and the four Switch wrappers; C++ is largest for all five EapRules ports. Counts include repository support and build files, not only the core game loader.

The project manifest has ten unique IDs and ten canonical repository URLs; the language manifest has one matching record per project. Each project has a primary README, release, metadata, and language evidence URL. No SQLite write, tracked-file edit, import, export, build, or database mutation was made.

