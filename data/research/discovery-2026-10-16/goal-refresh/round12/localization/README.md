# Round 12 localization research

Verified 2026-10-16 against the read-only 3,450-project SQLite catalog, current maintainer repository pages and releases, and exact GitHub Languages API endpoints. No database import or tracked-file edit was performed.

## Staged candidates

Four game-localization project rows are in `project-additions.json`:

- **Persona 2: Innocent Sin PSP French** — English → French, public v0.2.8 beta; system/menu coverage remains incomplete.
- **Pokémon Unbound French** — English → French, public BPS v2.1.179; only the French output is proposed.
- **Yoiyami Dancers Simplified Chinese** — Japanese → Simplified Chinese, public Steam v1.4.2 patch released 2026-10-16.
- **Harvest Moon: Back to Nature Brazilian Portuguese** — English → Brazilian Portuguese, active WIP; public source/tools and substantial progress, but no patch release yet.

Three tools are in `directory-additions.json`: Hoshi2Star, Game Translate, and the Pokémon Unbound translation toolkit. Game Translate converts existing Simplified Chinese game localization to Traditional Chinese; it is not a Proton-like platform/API translation layer. Two maintainer-linked YouTube videos and their direct thumbnail URLs are in `media.json`; no assets were downloaded.

Exact programming-language byte maps for the four project IDs are in `programming-languages.json`. That field describes source repository code only, not the human language pair. Yoiyami Dancers has an empty GitHub Languages API map, so its programming language remains unknown.

## Dedupe and held leads

`dedupe-audit.json` records repository/target checks against SQLite and research JSON under `data/research` and `work/research-2026-10-16`. Persona 2 has an existing PS1 recompilation row, a separate project and method from its PSP French patch. Pokémon Unbound has German, Italian, and Indonesian localization rows; the French target is absent. Proposed IDs and canonical URLs were not present.

`held-leads.json` records an alternate German Unbound patch that overlaps an existing game/language target, The Guild 1 Remake Russian patch whose source language is not disclosed, and a GitHub Desktop Simplified Chinese patch whose source locale is not disclosed. Harvest Moon is included as a WIP with no patch release.

## Validation

The JSON is validated for required catalog fields, HTTPS URLs, IDs, distinct project URLs, catalog IDs/URLs, and exact language maps. The importer and live SQLite database were not run or changed. Parent review should precede importing the staged manifests.
