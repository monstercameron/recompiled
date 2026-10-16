# Mac compatibility and translation research — 2026-10-15

This lane adds six software-level Mac compatibility projects and stages 25 evidence-backed metadata updates. The catalog snapshot had 847 project records. All discoveries and updates use maintainer repositories, release pages, GitHub repository metadata, or Apple/CodeWeavers documentation; X and Reddit were used only as leads.

The new compatibility frontends and wrappers are explicitly `target_type: Software` and describe their multi-title Windows software target. Their summaries distinguish the wrapper/manager from the Wine, CPU, and graphics translation components it composes. The Mac Gaming Uncle entry also distinguishes its own app/source from the separately obtained Apple D3DMetal component.

## New projects

- Hadron: early Apple silicon stack using native arm64 Wine, FEX for x86/x86-64 guest code, and graphics translation to Metal.
- Sikarugir: the Kegworks URL redirects to the canonical Sikarugir repository; recorded once under that canonical project.
- Whisky: original Apple silicon Wine wrapper; archived in 2025 and recorded with that status.
- Scotch: recent Apple silicon Wine frontend with separately selected graphics backends and a documented tested matrix.
- Cosmos: Intel and Apple silicon macOS Wine/Steam frontend.
- Mac Gaming Uncle: Apple silicon research preview that composes Wine, Rosetta 2, D3DMetal and fallback graphics layers.

The Game Porting Toolkit is not added as a second project because the catalog already represents D3DMetal as its graphics component. Generic launchers and installation helpers were left out where their own sources did not establish a distinct compatibility implementation or stack.

## Existing-record updates

The 25 updates cover current Apple graphics/compatibility layers and Mac-targeted game ports/reimplementations. Notable corrections include:

- DXMT advanced to its v0.80 release and current issue/activity evidence; MoltenVK to v1.4.2.
- KytyPS5's macOS path was narrowed to its documented x86-64 build under Rosetta 2, with Vulkan through MoltenVK.
- Tenkawa's only tested host was clarified as Apple silicon macOS; Linux/Windows/Steam Deck remain untested.
- Hybrid Heaven's unsupported macOS listing was removed because its current release matrix lists Windows, Linux and Android.
- Other game entries gained the documented Mac CPU/OS minimum, renderer, source revision, or latest release where the README or release page stated it. Unknown Mac architectures remain unspecified.

## Files and scope

- `new-projects.json`: `{updated, projects}` envelope; six full importer-shaped software records.
- `metadata-updates.json`: `{updated, updates}` envelope; each update has `id`, `fields`, `sources`, and `evidence_note`.
- The SQLite database, generated docs, and build outputs were not changed or imported.
