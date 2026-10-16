# Project language evidence coverage

Verified on 2026-10-15 from official repository language endpoints and project-owned source hosts. This batch fills the remaining project rows that had a legacy language label but no linked evidence.

| Partition | Records |
| --- | ---: |
| A–D | 161 |
| E–H | 85 |
| I–L | 33 |
| M–P | 205 |
| Q–T | 170 |
| U–Z and numeric IDs | 73 |
| **Total** | **727** |

The partitions cover every eligible project ID exactly once. The import records predominant language and the full available distribution for 707 projects; 20 are explicitly unknown with a source-linked reason. Four repositories expose language percentages rather than byte counts, so their published ratios are retained and identified as percentages in their evidence text. Full byte maps from official host APIs are preserved for the other known-language projects. A `predominant_language` of null clears the catalog filter value rather than retaining an unsupported label.

Validation checked exact coverage against the current SQLite set with no language evidence, unique project IDs, HTTPS evidence URLs, verification dates, language/map consistency, and nonnegative map values. An isolated database-copy import accepted all 727 records, preserved the previous 563 evidence rows unchanged, produced 1,290 evidence rows total, and passed SQLite integrity checks. The live catalog is imported by `scripts/build.sh` from this manifest.
