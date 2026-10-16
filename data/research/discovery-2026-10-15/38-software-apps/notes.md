# Lane 38 — software application reconstructions

Checked 2026-10-15. I used GitHub searches, the RecompSoftware applications tracker as a discovery index, and maintainer-owned project/release pages. The tracker and search results were leads only; the staged rows rely on the project repositories and maintainer pages for implementation, platform, version, status, and source claims. These are software application targets rather than games, emulators, generic file-format libraries, or ordinary open-source tools.

## Staged candidates

| Candidate | Target / method | Verified scope and caveats |
|---|---|---|
| FastTracker II Clone | FastTracker II for MS-DOS; portable C/SDL2 clone; `Reimplementation` | Maintainer page lists v2.26 dated 2026-10-15. Windows 7 SP1+ and macOS 10.11+ Intel/Apple Silicon builds are listed; linked Linux amd64 packages are not maintainer-supported and building may depend on distribution/GCC. The README says the XM player is ported from original source with permission and code combines original and new code. |
| ProTracker 2 Clone | ProTracker 2.3D for Amiga; portable C/SDL2 clone; `Reimplementation` | Maintainer page lists v1.92 dated 2026-07-24 and Windows 7 SP1+/macOS 10.11+ builds. The README includes Linux among targets with compile instructions; package availability depends on the distribution. No complete compatibility guarantee is claimed. |
| Webamp | Winamp 2 browser reimplementation; `Reimplementation` | The changelog lists v2.3.1 as current; README documents modern Edge, Firefox, Safari, and Chrome. Skin docs specifically state pixel-level rendering compared with Winamp 2.9. No Winamp binary plug-in or full desktop-product compatibility is claimed. |
| JS Paint | Windows 95/98/XP-era Microsoft Paint interface and behavior; web/Electron remake; `Reimplementation` | README says the editor recreates tools and menus with high fidelity; it also documents unfinished tools, browser clipboard limitations, and an out-of-date Electron version. The Electron app targets macOS, Windows, and Linux. This is an application remake, not a binary-compatible Paint runtime. |

Every staged project includes a maintainer repository, direct project or release/status evidence, and a GitHub API URL for the language breakdown. Language names in the rows follow those primary GitHub language endpoints; where those endpoints show multiple languages, the row identifies the predominant one and notes the other relevant implementation/build languages.

## Dedupe and exclusions

- Checked the four repository URLs, names, and target names against SQLite `projects` and `sources`, `docs/catalog.json`, the central reviewed research manifests, and all same-day staged JSON and notes. No existing record or same-day staged duplicate was found. This lane adds no metadata updates because none of the four projects has an existing catalog row.
- **Mindows 2k** was investigated in response to the lead about Windows 2000 userland on a Windows 11 kernel. The author's [public X timeline](https://x.com/msaraivaf) currently shows a short Explorer/Shell32.dll loading demo and describes “Mindows 2k” as Windows 2000 userland on top of the Windows 11 kernel. A search for the exact name in the [GitHub repository API](https://api.github.com/search/repositories?q=%22Mindows+2k%22) returns no matching repository, and I found no maintained source repo or durable project page. The timeline therefore establishes an experiment/demo only, not a maintainable reimplementation or API-translation record. The discovery follow-up also clarified that components need to be ported and that this does not use WOW64; with no project source or technical record to inspect, it is excluded.
- **SQ8L 64-bit port** appeared as a secondary download/blog lead, but I did not find a maintainer repository or project page that verifies the port's method, source availability, and supported versions. It is excluded pending primary-source evidence.
- Workflow-alike graphics, audio, and office apps from the tracker were not included where their pages described independent replacement products rather than a reconstruction, source port, or compatibility layer. File-format libraries and generic reverse-engineering tools are also outside this lane's scope.

## Validation and boundary

`new-projects.json` parses and has four unique IDs and project URLs. All four rows use `target_type: "Software"` and a supported `kind`. `metadata-updates.json` parses with an empty `updates` list. These are review-only staged files; SQLite, `docs/catalog.json`, and central reviewed manifests were not changed or imported.
