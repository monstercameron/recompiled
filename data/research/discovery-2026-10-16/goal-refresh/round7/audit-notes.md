# Metadata hydration audit

Audited the repository SQLite catalog (2,673 projects at validation time; full schema). The initial validation was performed on a disposable copy; the checked-in catalog was not modified by that review.

`project-metadata-updates.json` is importer-shaped for `scripts/catalog.py:import_project_updates`. It contains 53 unique project IDs with no no-op field values: the nine remaining round6 `source_available=true` values persisted as text `1`, 43 current generic `source_available="Repository"` rows whose public GitHub API metadata supports a more useful statement, and one Block Blast existing-row hydration. Of the 43 GitHub rows, 19 have explicit SPDX identifiers and 24 have no SPDX-identified license metadata. Two GitLab rows remain held because their pages redirected to sign-in and the API request returned 404 in this review.

An earlier 53-row source-availability proposal was a no-op against SQLite and had no ID overlap with this patch; it was excluded.

The nine Boolean rows already have correct status, target platform, predominant language and review date (`2026-10-16`) in the round6 records, so those values are not duplicated. Their source-availability values now describe actual source/releases and license metadata.

Block Blast was already in the catalog despite appearing in a staged `new-projects.json` file. Its latest published release remains v1.3 (2026-07-27); the current README changelog has a v1.4 entry but GitHub has no v1.4 release. The README supplies a specific 3DS/2DS device list, CIA and 3DSX install routes, zlib licensing, and five screenshots. It states the gallery is PC-rendered, so media labels preserve that caveat. The five image URIs are pinned to reviewed commit `6d8c7d6e67b696ddcba462256ff6d56da853632d` and each returned HTTP 200 with `image/png`.

Shizuku and Oni are not duplicated because the sibling localization manifest already contains source-backed metadata updates. Shizuku’s platform is already specific and GitHub reports no language bytes. Oni’s existing Python language and platform evidence are already present; the sibling patch’s v1.0 URL points to a 404 `ps1trans` path and should be corrected to `https://github.com/chivien99999/oni-zero-fukkatsu-english/releases/tag/v1.0`.

A standalone license column does not exist in the project schema, so explicit license details remain in `source_available` with first-party evidence citations.

## Validation

The update and media manifests were applied with the repository importer to a disposable copy of the authoritative SQLite database. The validation report records baseline row count, all patched values and source rows, five media rows, `PRAGMA integrity_check`, `PRAGMA foreign_key_check`, and live-database hashes. The disposable copy was deleted after validation.
