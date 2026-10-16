# Localization repository-language evidence

Checked all 18 staged project IDs against the authoritative catalog at `/Users/earlcameron/Desktop/recompiled/data/catalog.sqlite` on 2026-10-16. The live catalog had 2,625 project rows when checked, and all 18 candidate IDs were absent. No database writes were made.

`programming-languages.json` has one importer-compatible record per candidate. Ten GitHub repository URLs were queried at the official `/languages` REST endpoint; six returned language byte counts and four returned `{}`. The remaining eight non-GitHub project URLs have an explicit unknown reason because no GitHub repository can be queried from their staged project URL.

For each non-empty API response, the predominant value is the language with the largest exact byte count. Counts are repository-wide and may represent tools, generated material, or content for other projects. Shounen Ashibe points into `BlackPaladin2022/Translation-Patches`, and Super Street Fighter II Turbo Revival points into `sewave/translations`; their API totals cover those full repositories, not the selected subdirectory. The other eight GitHub project URLs point at repository roots.

Unauthenticated requests first hit the shared GitHub API rate limit; the recorded results were then fetched successfully with authenticated `gh api` calls to the same official endpoints. This file is staged for import after the corresponding project rows exist; no import or tracked-file edit was performed.
