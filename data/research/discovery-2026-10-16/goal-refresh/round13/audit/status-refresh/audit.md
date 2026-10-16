# Existing-project status refresh audit — 2026-10-16

## Scope and method

This packet checks 19 existing catalog projects surfaced by recent Reddit, GitHub-search, recompilation-news, and community-tracker leads. Social posts and registries were discovery signals only. Project facts were checked against the canonical GitHub repository API, default-branch README, GitHub Releases API/release pages, and GitHub Languages API. The canonical database was read through SQLite URI `mode=ro`; the staged snapshot contains the candidate rows and their current source/language evidence.

The live database was `/Users/earlcameron/Desktop/recompiled/data/catalog.sqlite`, with 3,462 project rows, metadata date `2026-10-16`, and SHA-256 `23ca4afe398ac57b79498017fff7e2a349276c9319d5e9a256c4cd68c1e2b7a9`. The hash and count matched before and after this audit. No database or tracked file was edited.

The proposed importer manifests are deliberately deltas. The existing P.T., KytyPS5, and Horizon summaries were kept. Fields were added to the project patch only where a current fact was stale or useful source-availability wording was supported by primary evidence. The existing GitHub language-byte maps were retained unless the current API response differed.

## Proposed changes

`project-updates.json` contains four source-backed patches:

- **P.T. (`pt-ps4-pc-reimplementation`)** — advances verification date and clarifies the source-availability field from “public source repository” to the evidenced MIT port code, release assets for Windows/Linux/macOS, and the user-supplied PS4 data boundary. Its current status already names v1.0.5 and the hotfix, so the status is held.
- **KytyPS5 (`kytyps5`)** — advances verification date and details GPL-2.0 C++ source, the current Windows/Linux/macOS archives, the Rosetta 2 macOS qualification, and the no-game/system-software boundary. Its existing status already names the exact current release and retains the experimental macOS caveat.
- **WineMetalGL (`winemetalgl-opengl-metal`)** — advances verification date and records merged PR #34’s targeted GLSL line-continuation fix and its reported 28 affected Godot shader variants, 61/61 host tests, and 259/259 GPU tests. The status preserves that this is a narrow fix and that compatibility claims remain bounded by the README’s test corpus and unsupported-feature list.
- **Horizon (`horizon-minecraft-java-reimplementation`)** — advances verification date and resolves the current release title to v2026.10.16.03 (published 2026-10-16 05:11:41 UTC; commit `f188dfe`). Release notes list chat-camera input and trial chamber/vault/trial-spawner fixes, plus a sprint-jumping report the maintainers could not reproduce. The status preserves the README’s 281-harness/2026-10-13 audit caveat and explicitly says the README still labels v2026.10.13.1 as the latest release. The correct summary and existing MIT source-availability field are held.

`programming-languages.json` contains three refreshed maps: KytyPS5, WineMetalGL, and Horizon. They are the only candidates whose current GitHub Languages API maps differ from the 2026-10-15 SQLite evidence. P.T.’s map and the other 15 candidates’ maps match the live API and are held. The exact Horizon response is in `evidence/github/horizon-minecraft-java-reimplementation.languages.json` and records Rust 19,085,901; Python 3,799,666; Java 3,074,284; HTML 67,149; JavaScript 43,222; WGSL 32,426; CSS 28,348; Shell 13,509; Dockerfile 2,003; Nix 554 bytes. Rust remains the largest measured bucket.

## No-change and held decisions

`candidate-review.json` gives a per-project upstream release/push check, exact prior row date, decision, language-map comparison, rationale, and snapshot paths for all 19 candidates. The remaining candidates were not patched because their current rows already reflect the newest useful primary-source facts, or because newer repository activity did not establish a newer supported-platform or compatibility claim:

- OpenCMR2 already records v1.1.5, its fixes, per-platform test coverage, and untested-play caveats.
- Viva Piñata: Trouble in Paradise already separates the native-renderer work from documented host support and does not infer full-campaign completion; release 1.13.0 is latest in the captured feed.
- D9MT has no GitHub release, its latest push is 2026-07-15, and the README says GTA IV is the only tested game.
- mtld3d already records v0.13.0 (2026-10-13), the README’s title-specific test list, and the limit on extrapolating compatibility.
- TF2MT already records v0.3.5 (2026-10-14) and retains the repository About-text conflict rather than treating it as implementation evidence.
- x87sidecar already records v1.9.0 (2026-10-13), macOS test coverage and its specific x87-instruction scope.
- Tourist Trophy already matches the README snapshot at commit `2935e55bc607` (22,501/31,593 functions; 71.2%; 32.3% of code bytes; 86.1% clean-source criterion) and the 2026-10-15 push.
- Smash64DS already matches the README’s dated 2026-10-02 status snapshot and records the 2026-10-13 push; that push supplies no newer compatibility claim.
- CTRPad already records source/build-recipe-only release v0.2.2 (2026-10-05), hardware tests and open acceptance work; repository activity ends on Oct 5.
- JSRF, BLACK, DK64, Blue Dragon, MW2 and Link’s Awakening already name their latest captured release/nightly and retain their project README limits on playability and tested coverage. The per-row details and release timestamps are in `candidate-review.json` and the snapshots.

The Horizon discovery source is the exact Reddit permalink `https://www.reddit.com/r/decomps/comments/1x71xxf/minecraft_java_263_was_just_decompiled_and/`; it is not used as a catalog fact source. Primary Horizon evidence is the saved repository README, release API, exact release endpoint and Languages API snapshots. The same primary-source rule applies to all other candidates.

## Staged files and validation

- `project-updates.json` — importer-compatible metadata field patches.
- `programming-languages.json` — importer-compatible exact byte maps.
- `catalog-candidate-snapshot.json` — SQLite read-only truth for all 19 current rows, their sources, and prior language evidence.
- `github-primary-snapshot-index.json` — repository, README, language and release API index with lead crosswalk.
- `evidence/github/` — exact GitHub API response JSON and decoded README snapshots for all 19 candidates.
- `candidate-review.json` — changed/held/unchanged rationale by project.
- `horizon-evidence-sha256.json` — checksums for the six Horizon primary-source snapshots.
- `scratch-validation/` — isolated importer copy, results and final row checks.

All proposed manifests are to be validated against a disposable copy of the catalog. The production database is intentionally untouched. The latest observed database integrity check returned no foreign-key violations.
