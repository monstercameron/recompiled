# Fan localization round 2 — 2026-10-15

Reviewed the current SQLite snapshot of 847 projects, including all 80 rows with kind `Fan Localization`. Staged 10 new project candidates after checking IDs, project URLs, and exact game titles against the catalog. Staged 80 source-backed metadata updates for the current 80 Fan Localization IDs: 45 from the current translation metadata audit, 13 from the recently verified fan-localization batch, and 22 from a follow-up audit of the remaining records. No game files, ROMs, patched ISOs/CIAs, or other game assets were copied or added.

## New candidates

The candidate set covers console, handheld, PC visual novel, and Switch projects. `new-projects.json` records output/source language, base build, patch format and distribution, maintainer status, compatibility limits, and patch-only/legal notes for each:

- Project Metafalica 2.0 for Ar tonelico II (English retranslation; four PS2 editions; xdelta).
- Our Life: Beginnings & Always Japanese patch (v1.20; game v1.7.1 PC/Android; `.rpa` ZIP; game data excluded; developer permission stated by maintainer).
- XOXO Droplets Japanese patch (v1.00; game v1.30; `.rpa`; game data excluded).
- New Love Plus+ English patch (v1.0.0-rc3; Japanese 3DS title ID and hash-gated patching; incomplete translation and limited device testing called out).
- Dragon Quest Monsters: Joker 3 Professional fixed English build (v2.21; exact Japanese base and 1.3 update; two xdelta patches; output CIAs are not distributed).
- Tomodachi Collection English patch (v1.0; exact Japanese CCUJ 1.1 ROM fingerprint; xdelta; no ROM distributed).
- Saber's Super Heroine Chronicle English patch (v3.1; PS3 BLJS10244/input hash; xdelta installer; no game data; incomplete text and translation-license caveat noted).
- Puyo Puyo Tetris 3DS English localization (build 1.1.1 / DLC 0.2.10; Japanese v1.2.0 base; xdelta/LayeredFS; the release also lists a pre-patched DLC CIA, which is flagged rather than described as patch-only).
- Higurashi Hou Switch Simplified Chinese patch (v2.2 for game 2.0.0/2.0.2 and separate v1.1 line for older builds; LayeredFS).
- Mobile Suit Gundam: Battlefield Record U.C. 0081 Simplified Chinese patch (formal package dated 2026-10-07; BLJS-10050 with official 1.02 resources; personal noncommercial use and no patch-file redistribution; test limitations noted).

## Existing metadata updates

The 45-row translation audit adds source language/output language where verified, patch or release version, target build/revision, patch format, platform instructions, download-page state, project status, and license details when the source supports them. The 13 recent rows capture language pair, platform, build/version, patch format and distribution from their maintainer pages/readmes. The 22 follow-up rows add the remaining current projects, including exact source-disc hashes, game/build compatibility, patch installer or delta formats, distribution rules, route/translation coverage, and release caveats. Update IDs are unique and all 80 exist in the current catalog.

The recent Mumble, Pokémon Azoth, Mushoku Tensei, Plants vs. Zombies Fusion, Fire Emblem Warriors, Summer Time Rendering, Xiaomi PC Manager, and Cavalry items retain their release and compatibility qualifications. The source manifest distinguishes a patch's required base build from translation source language, calls out rolling releases and untested ports, and identifies software UI patches separately from game patches.

The follow-up audit captured maintainer-source evidence for all 22 rows that lacked updates in the first pass. Where a maintainer page does not state an exact value, the update records that gap rather than guessing: examples include missing patch versions/formats for Brothers Conflict, Inazuma Eleven GO Galaxy, and Kamaitachi no Yoru; missing exact game builds for Tokeru Fuuka to Shirousagi, Valkyria Chronicles, and the Yakuza Remastered patches; and the Katana ZERO patch’s lack of a separate version number despite an exact tested game build.

Where an existing project has no public exact build, release date, patch format, or current download endpoint, the evidence notes preserve that gap rather than inferring one from an English-region baseline or repository activity. Dates are exact only where a maintainer stated a day; the New Love Plus+ release candidate's date is recorded only to the month because that is the precision in its release note.

## Sources and legal handling

Evidence links are project-owned maintainer sites, GitHub repositories/releases, itch.io maintainer pages, and patch readmes. GitHub topic pages were used only as discovery leads and are not cited as proof. Every proposed project has a primary project source. When maintainers require the user's own dump, provide only a difference/mod, restrict redistribution, exclude the game, or include a pre-patched add-on, that fact is retained in `caveats` or `legal_patch_only_details`.
