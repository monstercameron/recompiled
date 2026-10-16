# Graphics/API translation discovery (2026-10-15)

This lane reviewed the existing 33 `API Translation` rows in `catalog.sqlite`, searched maintainer/vendor documentation for missing general-purpose graphics layers, and checked additions against catalog IDs, canonical project URLs, and target APIs. Claims are linked to project maintainers, vendors, or Khronos/Mesa documentation. `updated` is the review date; `project_started` is left null unless a source states when development began. New projects are staged only; no database, generated docs, or build outputs were touched.

## New project candidates

The nine records in `new-projects.json` each represent a distinct API implementation or compatibility layer, rather than a game-specific port or whole-system emulator:

- **GL4ES** translates desktop OpenGL 1.5/2.1 calls to OpenGL ES 1.1/2.0 on embedded/mobile systems.
- **Mesa Gallium D3D12** provides OpenGL 3.3 on top of Direct3D 12, including WSL/WSLg graphics acceleration.
- **KosmicKrisp** implements Vulkan over Metal on Apple Silicon; Khronos reports Vulkan 1.4 conformance on 2026-10-01. It is a separate Mesa driver/API direction from MoltenVK.
- **DXGL** wraps legacy DirectDraw/Direct3D-era calls over OpenGL. The active 0.5.28 release and the nonfunctional 0.6.0-dev branch caveat are both called out.
- **cnc-ddraw** reimplements the DirectDraw API with GDI, OpenGL, and D3D9 outputs for software-rendered 2D games. Its README explicitly excludes Direct3D-rendered games.
- **dgVoodoo2** is a proprietary Windows wrapper for legacy Glide/DirectX APIs over D3D11/12 or WARP.
- **nGlide** is a proprietary Windows Glide wrapper over Direct3D or Vulkan; its developer’s first-release announcement supplies a 2009 date, but that date is recorded as the first public release, not the project start.
- **Gallium Nine Standalone** connects Wine’s D3D9 interface directly to Mesa’s Gallium Nine frontend. The record describes its Mesa/GPU dependencies and latest listed release rather than calling it a D3D9-to-OpenGL translator.
- **Valve ToGL** is a Source-engine D3D9-subset-to-OpenGL layer. Its public source is an unsupported dump, only a limited feature subset is documented, and Valve says it may not build by itself.

The two Mesa driver entries share a monorepo URL, but have different project targets and directions: OpenGL→D3D12 versus Vulkan→Metal. They are not duplicate records.

## Existing record boundaries and corrections

The update patch makes API direction and maturity more precise for 29 existing rows. It refreshes dated releases for d3d8to9, DXMT, DXVK, MoltenVK, Proton, VKD3D-Proton, Wine upstream vkd3d, and other active projects. It also marks **KytyPS5** as `Emulator`, since its own repository describes a console emulator and the Vulkan shader recompiler is only one part of that system.

The PS4 records for **bbport**, **inFAMOUS: Second Son**, and **The Last Guardian** are game-specific runtimes/ports that contain graphics translation; their summaries are updated to make the title-specific scope visible. **bbport** currently derives its graphics path from shadPS4-style PM4 command processing and describes direct GNM→Vulkan translation as future work. **bbhost** likewise has a game-specific hybrid graphics path. They should not be presented as reusable general-purpose console graphics API layers.

**AnyPS5** remains a binary-relinking/system-library compatibility project with Vulkan GPU work; conversion of a binary is not evidence that a game runs. **PortPS5** remains pre-1.0 and its dated PRD says no gate title had met its run criteria. **WinDurango** is a partial Xbox One ERA/UWP API compatibility project, not a full hardware emulator. The patch clarifies those scopes.

**Wine** and **Proton** are broad Windows compatibility stacks. **WineD3D**, **DXVK**, **vkd3d**, and **D3DMetal/DXMT** are distinct graphics components/backends, and the updates preserve those relationships. **Rosetta 2** was not updated in this lane: it translates CPU instruction sets rather than graphics or application APIs.

## Excluded leads

- **SwiftShader** is a CPU-based graphics API implementation/software renderer, not an API-to-API translator.
- **DDrawCompat** focuses on fixing DirectDraw behavior without translating it to a newer graphics API.
- **VirGL** and **gfxstream** are virtualized guest graphics paths; they sit too close to VM/emulator graphics plumbing for this lane’s “general graphics API translation” scope.
- Game-specific ports and console emulators that happen to use Vulkan are not added as general-purpose translation projects.

All candidate rows were checked against the SQLite catalog by ID and project URL. This staged handoff contains 9 new records and 29 metadata updates; it is not imported into SQLite.
