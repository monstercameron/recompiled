# Goal-refresh round 5

Reviewed on 2026-10-16 against the live 2,291-project SQLite catalog and maintainer-owned repositories. These importer manifests add seven source-backed handheld or PSP projects, repository language-byte evidence for all seven, three release/status refreshes for existing projects, and Wikipedia references for all six new game profiles.

- `project-additions-reviewed.json`: SD Gundam: Dimension War Virtual Boy recompilation; Legend of Doom, 2048, and Block Blast 3DS projects; Bounce Vita; Call of Duty: Roads to Victory PSP web recompilation; and the Nakagawa PSP recompilation toolkit. Status text records release maturity, target hardware, required game data, and validation limits.
- `programming-languages-reviewed.json`: predominant language plus exact byte breakdowns from each official GitHub Languages endpoint.
- `project-updates-reviewed.json`: current release notes for Animal Crossing 3DS, Paper Mario/Star Haven 3DS, and Super Smash Bros. Melee 3DS. Each row links to maintainer README or release evidence; `updated` means reviewed on 2026-10-16, while upstream release dates appear in status text.
- `wikipedia-links-reviewed.json`: English Wikipedia references for all six new game profiles, using direct articles where available and exact-title searches where no dedicated article exists.

The seven new rows were checked by ID, title, game, project URL, and owner/repository against the live catalog and staged research manifests. A disposable-database import dry run applied seven additions, seven language audits, and three status updates with a clean SQLite integrity check and no foreign-key errors.
