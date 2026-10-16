# Catalog research import batch — 2026-10-16

This reviewed batch turns the clean reconciliation proposals in `work/research-2026-10-16/staging-reconcile/` into canonical importer inputs. The packet README records original source manifests, deduplication, and exclusions.

- `project-additions-reviewed.json`: 19 unique projects from the audited Mac, console-crossport, and Nintendo batches.
- `project-language-evidence-new.json`: GitHub/API language evidence for those 19 projects.
- `project-language-evidence-round4.json`: 298 audited language records for existing projects that lacked an evidence row; 261 byte-backed/known and 37 explicit unknowns.
- `project-updates-reviewed.json`: 105 conflict-free, non-status field updates.
- `project-status-review.json`: 38 proposed status changes held for independent review before import.
- `project-media-reviewed.json`: 11 HTTPS media links for seven existing project groups; these remain URI-only.
- `wikipedia-links-reviewed.json`: English Wikipedia references for all 10 newly added game targets, with a title-search fallback where the game has no standalone article.

The four fan-localization proposals and nine emulator proposals in the source packet remain on hold for rights/status review. The ambiguous OpenLara duplicate and the God of War launcher integration were excluded.
