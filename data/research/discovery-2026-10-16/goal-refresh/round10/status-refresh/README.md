# Round 10 project status refresh

This reviewed packet refreshes the catalog review date and current maintainer-source citations for 60 existing projects. Fourteen records receive precise status corrections; two platform lists, two summaries, and one source-availability value are also corrected. The status notes distinguish releases from tags and activity timestamps, and limit playability claims to the versions and test scope reported by maintainers.

`project-updates-reviewed.json` is imported by `scripts/build.sh`. `decisions-and-holds.md` explains selection, per-project decisions, and evidence boundaries. `github-repo-metadata.json` holds direct repository API snapshots; the category inspection notes summarize the source checks. The raw README bodies are not copied into the repository; the importer manifest links to each reviewed primary source.

The full update manifest passed against a temporary copy of the 3,288-project catalog: all 60 IDs were known, project count was preserved, and SQLite integrity and foreign-key checks passed.
