# Additional API translation discovery (2026-10-15)

This lane expanded the graphics/API search beyond the previous `06-graphics-api` batch. It searched maintainer GitHub repositories and release pages for legacy Direct3D-to-Vulkan layers, Windows-game graphics components used on Android, and distinct OpenGL-to-mobile/Metal implementations. Candidates were checked against SQLite and the generated `docs/catalog.json` by stable ID, title, and canonical repository URL. Nine unique, source-backed additions are staged in `new-projects.json`; neither SQLite nor generated site data was changed.

## New records

- **D7VK** routes legacy Direct3D 3/5/6/7 immediate-mode work through a D3D9/Vulkan backend under Wine. D3D7 is the principal target; D3D6/5/3 are less mature and retained-mode D3D is unsupported. The maintainer's v2.3 release is dated 2026-09-29.
- **DXVK-Sarek** maintains a DXVK-derived Direct3D 3–11 implementation for Vulkan 1.1/1.2 hardware below current upstream requirements. It documents Android/ARM compatibility-front-end use, but its maintainer warns of degraded rendering and does not promise mobile GPU support.
- **VEGAS** is an Adreno-focused DXVK/GPLAsync fork used as a graphics layer in Wine-based Android game frontends. Its release repository contains separate DXVK-derived and Sarek build lines, so the status records dates and versions without treating them as a single sequential version stream.
- **DXVK-Remix** is NVIDIA's distinct RTX Remix runtime fork. Its own SDK documentation describes a Direct3D 9 translation component that reconstructs scene information for the Remix renderer; a separate bridge supports 32-bit clients talking to the 64-bit runtime. It is more specific than the already cataloged general-purpose DXVK row and is useful for tracking classic-game remastering/modding.
- **Remix Plus** is a separate community fork of DXVK-Remix with substantive Remix SDK/plugin additions (batched geometry/lights, game-state injection, UI access, and VRAM controls). It shares the base D3D9-to-Vulkan translation path with NVIDIA's runtime; the record makes that relationship explicit rather than presenting it as a new graphics backend.
- **MobileGL** is an unfinished multi-backend desktop OpenGL implementation. The maintainer explicitly labels it in development, names OpenGL 4.2 Core as the current target, and says no prebuilt releases are available. Its documented macOS path uses Vulkan/MoltenVK.
- **MobileGlues** adapts desktop OpenGL/GLSL to Android OpenGL ES for Minecraft Java. The code repository is LGPL-2.1; the companion release repo publishes V2.0.0 and compatibility information for supported Minecraft renderer mods.
- **Mithril-Wrapper** translates OpenGL 3.3/EGL to Vulkan 1.2 and then Metal through MoltenVK for iOS/iPadOS launchers. It is a work in progress; its README explicitly identifies EGL sync/image calls that are only shadow implementations. The old repository is archived and points to the current successor, which is used as the canonical project URL.
- **MGL (openglonmetal)** is a separate, partial macOS implementation of OpenGL over Metal. Its README explicitly calls out missing functions and non-conformance; the record keeps those limits visible and does not imply general application compatibility.

## Existing-record recheck

Primary GitHub release endpoints were checked for **Proton** (latest listed `proton-11.0-2e`, 2026-10-09), **DXVK** (`v3.1.1`, 2026-09-15), and **vkd3d-proton** (`v3.0.1`, 2026-05-06). Those versions match the current catalog metadata, so this lane does not stage redundant updates. Earlier graphics/API documentation in `06-graphics-api/metadata-updates.json` already covers the relevant base projects and API-direction clarifications.

## Excluded leads and boundaries

- **prosperNG** is a fork that compares identical to its parent `mattias800/prosper` at the checked branch; it has no independent project history to catalog as a second layer.
- **MetalANGLE** is an older ANGLE fork whose maintainer says Metal backend work moved into official ANGLE and the fork is no longer actively developed; ANGLE is already cataloged.
- **AppGL** was found through a self-promotional Reddit post linking to a GitHub Pages site, but the site was inaccessible to direct verification and no public source repository/release record was corroborated. Its performance claims were also self-reported. It is not staged as a verified project.
- **VulkanMobile** is a Minecraft mod that replaces its renderer with a Vulkan renderer, rather than a general API translation layer; it belongs in a renderer/mod category if separately reviewed.
- **XPS5X/KytyPS5 and other console emulators** were excluded from this lane even when their renderers use Vulkan; emulation is outside the assigned API-layer scope.
- Other DXVK/Remix forks without a distinct, source-documented API, compatibility target, or substantive modding feature set were not added; this lane does not enumerate every fork or binary repack.

## Verification

- All nine IDs and canonical project URLs were queried against `data/catalog.sqlite` and searched in `docs/catalog.json`; no collisions were found.
- All project rows have direct HTTPS source links and the exact API direction, current evidenced maturity/status, source availability, and predominant implementation language where GitHub language data was available.
- Dates for project start are blank because no reviewed maintainer source states a reliable start date. `first_found` records this discovery date instead.
- Import check: `python3 scripts/catalog.py import data/research/discovery-2026-10-15/24-api-layers/new-projects.json --dry-run` is not supported by the importer, so the manifest is validated with JSON/schema/dedup checks only; do not import it as part of this lane.
