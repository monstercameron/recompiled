# Round 10 reconciliation audit

Verified 36 unique project additions and 3 separate directory tools against the live catalog snapshot on 2026-10-16.

## Outcome

- Project additions: **36**; all have required importer fields, at least one HTTPS primary repository source, unique IDs, unique exact repository-root/target pairs, and no canonical DB collisions.
- Language evidence: **36** rows; **28** nonempty largest-byte-bucket assignments, **7** exact empty GitHub maps, and one GitLab percentage-only map left unknown. Full exact byte maps are in `project-language-evidence.json`.
- Project kinds: API Translation 2, Decompilation 2, Emulator 4, Fan Localization 10, Recompilation 7, Reimplementation 11.
- Language counts: Assembly 1, C 13, C++ 5, PowerShell 1, Python 3, Rust 3, TypeScript 2, Unknown 8.
- Directory additions: 3/3 net new (Kuriimu2, ScummTR, Telltale Workbench); validated separately with `import-directory`.
- Live database before validation: 3,288 projects, 1,954 game keys, 11,206 source rows; hash 7b228a34ea26bad44510944a24b087b6a32298941b0a0e68beb0698274d3bc8e.
- Scratch import of projects, exact language evidence, and directory entries passed; SQLite integrity is `ok`, foreign-key issues are 0, and the live DB hash stayed unchanged. The four emulator additions import as `target_type: Software`. Scratch totals: 3,324 projects, 1,971 games, 11,414 sources, 3,297 language-evidence rows, and 36 directory items.

## Holds and exclusions

- **hold** lazuli-gamecube: README says active development stopped on 2026-09-16 while GitHub reports a later 2026-10-12 push and nightly release; reconcile this status before treating it as maintained.
- **hold** openprimus-wpm-primus-espresso-machine-firmware: Current README describes research plus a 3-page simulator; real sensor/actuator mapping and brew-control loop are incomplete, so no working replacement firmware is established.
- **exclude** hl2-xbox-static-recomp: The primary README explicitly says the build is not playable and stops before the game world is handed off.
- **exclude** abuse-sdl2-port: Primary README identifies this as a fork of the Xenoveritas SDL2 port; older 2016 release, no independent reconstruction or distinct target is established.
- **hold** microproject-microsoft-project-desktop-reconstruction: Active release exists, but the project is still a ProjectLibre-derived mixed legacy/clean-room codebase and explicitly says its Microsoft Project compatibility goal is not achieved; method and target boundary need clearer evidence.
- **hold** pinacoteca-picasa-clean-room-photo-manager: README/roadmap remain at repository bootstrap and behavioral-analysis stages; no usable viewer or shipped feature was verified.
- **hold** armored-core-mobile-mission-steam-en: README says patch is only distributed in a Discord; no public patch artifact/version is linked, so current availability is not independently verifiable.
- **hold** armored-core-mobile-2-steam-en: README says patch is only distributed in a Discord; no public patch artifact/version is linked, so current availability is not independently verifiable.

The staged search/reconciliation records contain 102 additional duplicate, related-project, or held-lead dispositions; the full per-row evidence and rationale are in `audit.json`.

## Target-key coverage

The 23 Game rows resolve to 23 distinct game keys: 6 already exist in the live game table, 17 are new, and none matches an exact key in the round10 fact-profile or Wikipedia-link packets. Existing equivalent keys were reused for Crash Team Racing, Ace Combat 04, DELTARUNE, Doom RPG, Dinosaur Planet, and San Francisco Rush 2049. All four standalone emulators are classified as Software and retain their supported system names in the legacy `game` field.

## Revalidation lanes

Status, category, and language revalidation packets refer to projects already in the current catalog and are not repeated as project additions. Game facts, Wikipedia links, and project media remain separate import lanes.
