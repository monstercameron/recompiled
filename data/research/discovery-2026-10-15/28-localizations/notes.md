# Fan localization and software UI localization research

Scope: actively maintained or newly released fan/localization patches for games, plus community UI localizations for desktop software. API compatibility layers and graphics/API translation are excluded from this lane; those belong to the API translation research lane. Each record's `language` and `target_languages` mean the output language(s), while `source_language` and `language_pair` describe the source. If a maintainer did not identify the original language, the record says so rather than guessing. Software UI records use `target_type: Software`; the legacy required `game` field contains the application name. `kind: Fan Localization` is used because the catalog currently has no separate software-localization category.

The 12 proposed records are in `new-projects.json`. They cover Oni Zero: Fukkatsu, Khamrai, Twilight Syndrome DS, The Rhapsody of Zephyr Remastered, Gloria Union Android, Digimon Universe: Appli Monsters, FGO Arcade's scooby patch/launcher, Postman Desktop CN Modern, The Forgotten Concluder, Mewgenics, Enkyori Ren'ai / Bond Distance Love, and Claude Desktop Chinese. Each row includes the maintainer/project URL, dated status, output language, platform, patch/distribution details, supported game or client build where documented, caveats, and HTTPS evidence. Unknown details are stated as unknown or omitted; repository activity dates are not described as release dates.

`metadata-updates.json` records three evidence-backed corrections to existing catalog rows: Summer Time Rendering: Another Horizon now has a public v0.13 Switch patch; Dragon Quest XI 3DS has a GitHub 1.0 release dated 2026-10-13 (the maintainer also calls its package v18); and SEGAGAGA should point to the maintainer's ExxistanceDC repository and current v1.1 release. Those same three IDs already have update records in `12-localizations/metadata-updates.json`; merge the stronger/current source evidence by ID instead of applying duplicate updates.

## Dedupe and validation

At the time of this audit, the read-only SQLite catalog had 959 project rows. None of the 12 candidate records matched an existing row by ID, case-insensitive target name, or normalized project URL. A recursive search of the staged JSON manifests found no same-name or same-URL candidate elsewhere. All three metadata-update IDs do exist in the catalog, as expected. Their exact IDs also appeared in lane 12's existing update manifest.

All project source links and project URLs use HTTPS. The additional Gloria Union patch-format source was checked against the live GitHub repository tree and file contents at `docs/PATCH_FORMAT.md`. JSON syntax, required project fields, category values, unique candidate IDs, and metadata-update field names were validated. The manifests are staged for review only; this lane did not edit SQLite or generated documentation and did not copy game, patch, or media files locally.

## Exclusions and boundaries

- Generic translation frameworks, machine-translation engines, and string-extraction utilities were excluded because they are tools rather than a localization of a named target.
- API compatibility projects such as Proton/DXVK-style layers were excluded; they translate graphics or platform APIs, not user-facing language.
- Projects without a public patch/source, a named target, or enough maintainer evidence to establish a real project were excluded.
- Taromaru was left out because its public README describes an older partial/testbed translation and this search did not establish a maintained 2026 release.
- The Portmaster Chinese localization prototype was left out because the issue/repository evidence describes it as not planned or not maintained as a public release.
- Alternate Claude Desktop Chinese repositories were not counted as separate target projects; the selected record is the one with a versioned cross-platform release. Installer privilege and repair-watcher behavior is called out as maintainer-described and was not independently run or audited.
- Postman CN Modern has no tagged release, but was retained because its public repository documents a reproducible install/restore workflow, version support, and recent repository activity. Mewgenics v0.7 is explicitly a prerelease and targets an identified beta game build, which is reflected in its status and caveats.
