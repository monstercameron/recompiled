# Round 11 catalog research

Round 11 combines source-reviewed discovery across GitHub, Reddit, X, project trackers, console-to-console and mobile ports, application decompilation/rebuilds, API translation, emulators, and tool directories. The reviewed manifests below are the import sources wired into `scripts/build.sh`; evidence URLs and language provenance are retained in each record.

- **87 new projects:** 70 game targets and 17 software targets.
- **29 existing-project updates** with source evidence.
- **115 language records** for newly added projects and audited existing projects; unknown language results remain explicit rather than guessed.
- **201 remote media links** (188 images and 13 videos) for 39 projects. Assets are referenced by URL and were not downloaded.
- **58 game profiles** with 377 sourced facts, plus **70 Wikipedia references**. Profiles only fill game keys without prior facts; existing fact-bearing profiles are preserved.
- **19 tool and resource directory entries.**

## Reviewed imports

- `project-additions-reviewed.json` — new project records, with repository/source URLs and discovery metadata.
- `project-updates-reviewed.json` — reviewed status and metadata corrections for existing projects.
- `programming-languages-reviewed.json` — exact language evidence or an explicit unknown where no authoritative language map was available.
- `media-reviewed.json` — original remote media URIs and source pages.
- `game-facts-reviewed.json` — additive profiles for previously unhydrated game keys.
- `wikipedia-links-reviewed.json` — direct English article links or exact-title fallbacks.
- `directory-additions-reviewed.json` — tools and research resources.

All seven manifests are imported by the build before `docs/catalog.json` is exported. Project-status language reflects the available maintainer or repository evidence, and uncertain candidates remain in the research work area rather than being imported.
