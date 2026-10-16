# Round 12 catalog research

Verified manifests in this directory add 51 projects, preserve 52 GitHub language audits (including one refresh for an existing project), apply four evidence-backed project updates, add 34 remote media URIs, fill 76 previously empty game profiles with 434 sourced facts, add 25 Wikipedia links, and add seven tools or discovery resources.

| Lane | Projects | Language audits | Other imports |
| --- | ---: | ---: | --- |
| API translation | 1 | 1 | — |
| Console ports | 13 | 13 | 4 media URIs |
| Game recompilation | 6 | 6 | 3 project updates; 7 media URIs; 5 game profiles / 24 facts; 5 Wikipedia links |
| Software reconstruction | 7 | 7 | 2 media URIs |
| Recent community leads | 12 | 12 | 6 inline media URIs; 3 directory entries |
| Game metadata hydration | — | — | 70 additive profiles / 405 sourced facts across batches A and B |
| Console-to-mobile ports | 7 | 8 | 1 project update; 12 media URIs across new and existing projects |
| Fan localizations | 4 | 4 | 2 media URIs; 3 translation-tool directory entries |
| HERCULAN / Earthsiege 2 | 1 | 1 | 1 media URI; 1 game profile / 5 facts; 1 Wikipedia link |
| Discovery index | — | — | 1 directory entry |

The aggregate language-audit total includes 51 new projects and an audit refresh for the existing Pokémon Emerald Android project. Media is stored as remote HTTPS URIs with source pages; no media or game files are copied into the repository. The Awesome Decomp Games website is listed only as a discovery lead: its dataset is not imported, and project claims still require direct maintainer-source verification.

Wikipedia coverage includes 19 additional catalog gaps in `wikipedia-links-gap-completion.json` (17 direct article links and two exact-title searches), alongside the five game-recompilation references and the separate Earthsiege 2 article.

All candidate projects were deduplicated against the catalog before import. Each project record links to its maintainer repository or official project page and records its method, target, status, platform, source availability, and relevant limitations. Search leads without adequate primary evidence are held in the lane audits.

## Lane evidence

- `api-translation/` — primary repositories, method evidence, and exact GitHub Languages API totals.
- `console-ports/` — target and method evidence, candidate audit, language totals, and remote media.
- `game-recomp/` — maintainer sources, GitHub search and repository audits, status updates, game profiles, Wikipedia links, media, and held related projects.
- `software-reconstruction/` — software-target scope, maintainer evidence, language totals, and URL-only media.
- `social-latest/` — community discovery leads reconciled to maintainer repositories; see `network-followups.md` for live X/Reddit follow-up and deduplication decisions.
- `mobile-ports/` — Android and iOS target research, existing-project update, media URL checks, exact language maps, search coverage, and exclusions.
- `localization/` — project and language-pair evidence, exact programming-language maps, remote video references, translation tools, held leads, and dedupe audit.
- `earthsiege-followup/` — HERCULAN maintainer README, release and license evidence, game profile, language map, Wikipedia reference, and one remote screenshot URI.
- `wikipedia-links-gap-completion.json` — direct and exact-title Wikipedia references for 19 previously uncovered game keys.
- `discovery-resources.json` — source evidence for the discovery index.

The import sequence is recorded in `scripts/build.sh` so a normal build recreates the database-backed catalog export and WASM site from these reviewed manifests.
