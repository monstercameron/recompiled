# Tools and process resources review — 2026-10-15

Reviewed the current `directory_items` and five `process_resources` rows in SQLite, their reviewed manifests, importer constraints, and official project/site pages. The staged manifests are proposals only; nothing was imported and the database, generated catalog, and build outputs were not changed.

## Staged additions

`directory-additions.json` contains six non-duplicate additions:

- `splat` fills the N64/PSX/PS2/PSP binary-splitting gap for decompilation and modding. Its README explicitly directs GameCube/Wii projects to decomp-toolkit. GitHub REST API: 368 stars on 2026-10-15.
- `decomp-toolkit` covers GameCube/Wii disc and DOL/ELF analysis, splitting, and linking workflows. GitHub REST API: 311 stars on 2026-10-15.
- `BepInEx` adds a maintained Unity/XNA/.NET game plugin and patching framework, with current official install and plugin-development docs. The docs list Windows, Linux, and macOS support. GitHub REST API: 8,845 stars on 2026-10-15.
- `MoltenVK` adds the Vulkan-over-Metal portability path for Apple platforms. Khronos documentation lists macOS, iOS, tvOS, and visionOS. GitHub REST API: 5,886 stars on 2026-10-15.
- `ANGLE` adds OpenGL ES translation to backends including Vulkan, desktop OpenGL, Direct3D 9, and Direct3D 11. GitHub REST API: 4,099 stars on 2026-10-15. Its GitHub page labels the repository a public mirror of the Chromium-hosted upstream; the README is used for the implementation facts.
- `Nexus Mods` adds a major multi-game mod publishing/discovery site and its Vortex/Collections tools. Its official about page states over 750,000 mods across 2,500+ games when checked on 2026-10-15. This site count is kept distinct from GitHub and YouTube metrics.

Popularity values above came from each repository's official `https://api.github.com/repos/{owner}/{repo}` response (`stargazers_count`) on 2026-10-15. They are dated selection evidence; the directory schema does not store a popularity field.

## Existing directory audit

- Current GitHub project URLs and README paths returned successfully; the repository identities and descriptions for Ghidra, decomp.me, objdiff, N64Recomp, Proton, DXVK, vkd3d-proton, Flips, REA, xboxrecomp, XenonRecomp, ReXGlue, PS4Recomp, and XenosRecomp remain supported by their official project pages.
- The Data Crystal and TCRF pages returned HTTP 403 to direct fetches. This looks like access protection rather than a missing page; the URLs have not redirected or shown a 404. Recheck through a normal browser before changing either URL.
- `xenosrecomp` is accurately described in its evidence as an Xbox 360 shader-microcode-to-HLSL converter with DXIL/SPIR-V paths. Its current category, “Graphics API translation,” can imply whole API/runtime translation; “Shader translation” would be a more precise category when existing entries are next editable.
- Ghidra is a general software reverse-engineering tool. Its description is accurate but could say “compiled software” rather than “compiled game code” to avoid implying game-specific support.
- `process_resources` currently represents four subjects: decompilation, static recompilation, source ports, and API translation. The current five titles' source URLs and topic assignments still match their descriptions.

## Process-resource metrics

`resource-updates.json` includes all five existing resources, with their positions and content retained. It refreshes counts using platform-native sources and exact values, without comparing unlike metrics or changing the ranking:

| Resource | Platform metric before | Official metric checked 2026-10-15 |
| --- | ---: | ---: |
| Nerrel static-recompilation video | YouTube 1,065,571 views / 52,000 likes | 1,105,699 views / 52,455 likes |
| MVG Ocarina of Time decompilation video | YouTube 458,837 views / 22,000 likes | 461,935 views / 22,339 likes |
| N64: Recompiled README | GitHub 8.3k stars (rounded page display) | 8,250 stars |
| Shipwright README | GitHub 5.6k stars (rounded page display) | 5,624 stars |
| DXVK README | GitHub 18.3k stars (rounded page display) | 18,303 stars |

The YouTube counts were read from the official watch pages' video metadata; their watch URLs are retained as `evidence_url`. GitHub counts use the official REST API endpoints as `evidence_url`. This fixes rounded or stale display values without treating views, likes, and stars as interchangeable.

## Missing how-to coverage and schema limit

The catalog has no process-resource topic for modding, and its importer accepts only five total records and the four existing topic keys. I did not force a modding tutorial into an unrelated topic or replace an existing position. A useful official candidate for a future decompilation how-to is ZeldaRET's [Wind Waker decompiling guide](https://github.com/zeldaret/tww/blob/main/docs/decompiling.md); the owning repository had 1,097 stars in the GitHub API on 2026-10-15. A useful official modding how-to candidate is the [BepInEx basic plugin guide](https://docs.bepinex.dev/master/articles/dev_guide/plugin_tutorial/index.html), which walks through setup and a working C# plugin, but the importer cannot label it as modding and the docs page has no page-level popularity metric.

Adding a modding-topic resource or expanding the five-card limit requires a later schema/UI change. No `process-resource-additions.json` was staged under the current importer contract.
