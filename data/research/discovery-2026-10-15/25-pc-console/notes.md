# PC-to-console and console-port discovery

Checked: 2026-10-15. This lane searched GitHub repository results and opened project-maintainer repositories, READMEs, release pages, build instructions, and target-specific source. Community lists and social posts were treated as discovery leads only. Each proposed record links primary project evidence. GitHub CLI repository search returned an API rate-limit 403 during this pass, so qualification used the public GitHub project pages and their primary documentation.

The live SQLite catalog contained 897 projects when checked. I compared candidate IDs, canonical project URLs, game titles, and target platforms against SQLite and the generated catalog before writing this batch. The 15 records in `new-projects.json` are candidate rows only. No SQLite file or generated page was edited.

## Included

The batch adds documented console targets that were absent from the live catalog:

- PS Vita: OpenMW, vitaQuake, SDLPoP Vita Enhanced.
- Wii U: ioQuake3-U.
- New Nintendo 3DS: OpenXCOM-3DS and Ballistica.
- Dreamcast: SDLPoP, Celeste Classic, Quake, Quake II, and SkiFree ports.
- Original Xbox: Celeste Classic and OpenTyrian ports.
- Multi-console: OpenLara console targets and Trogdor: Reburninated.

The catalog has no dedicated “source port” category, so these projects use `Reimplementation` where the maintainer describes a source port or engine rewrite. None is labeled a recompilation unless its method is genuinely a binary recompilation. The records distinguish documented builds from tested gameplay and avoid claiming complete campaigns where the maintainers do not.

## Deduplication and held items

- `doom-wii-u-crispy-port`, `jedi-knight-dark-forces-ii-dsi-dreamcast-port`, `moon-child-dreamcast-port`, `sonic-cd-2011-wii-u-port`, and numerous N64/GameCube Dreamcast and handheld ports were already in SQLite with target-specific source links, so they were not repeated.
- `grand-theft-auto-iii-wii-port` and `grand-theft-auto-vice-city-wii-port` were already in SQLite when the final deduplication pass ran. The Vice City Wii repository was discovered during this scan and verified from its README, but its existing row already captures the Wii target and build requirements.
- OpenJKDF2 is already cataloged as a Windows-game reimplementation with DSi and Dreamcast targets; it was not split into a duplicate Dreamcast row.
- `Flowit-Vita` is a valid Vita port, but its source game is an Android app and the desktop build is a separate port; held for the console/mobile discovery lane.
- `Prince of Persia Classic PS Vita` wraps an Android release and its native libraries; held for the mobile-port lane, with its exact compatibility method to be classified there.
- The Xash3D Dreamcast fork exposes a target-specific repository and prerequisites, but the README did not establish a release or successful build/play state in this check; held pending stronger target evidence.
- `nullDC4Wii` and `smsplusdc` are emulator ports rather than game source ports; left for the emulator lane.
- Reports of GTA V running on Switch were not promoted: I did not locate a primary project repository/release proving the claimed build, and available reporting raises unresolved provenance concerns about the source code.
- Rizzo Island's PC/Dreamcast repositories describe an original cross-platform game rather than a reconstruction or port of an existing software target, so it was held outside this batch's preservation scope.

`metadata-updates.json` is intentionally empty. The relevant catalog duplicates already had sourced target, method, source-availability, and status fields; the new console builds are separate project/target records rather than a reason to broaden those existing rows.

## Verification

- Both JSON files parse successfully.
- All 15 records have unique IDs, the required project fields, supported `target_type` and `kind` values, and at least one HTTPS primary-source link.
- None of the candidate IDs or canonical project URLs matched the 897 live SQLite records at validation time.
- `git diff --check` passed for this research directory.

Build availability and playability are not interchangeable. Where a README only provides a build recipe, the record says so and avoids implying real-hardware testing.
