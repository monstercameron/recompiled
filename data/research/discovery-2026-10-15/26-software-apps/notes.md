# Software application reconstructions (2026-10-15)

This lane searched GitHub and the open web for non-game desktop and system software source restorations, decompilations, recompilations, and application ports. Repository READMEs and maintainer-authored posts were used to verify targets, methods, and status; search indexes and community discussion were treated only as leads. The existing SQLite catalog was checked by exact project URL and target/title before staging additions. Nothing in this lane was imported into SQLite or generated docs.

## New records

- **Acornsoft VIEW** is a productivity application rather than a game. Its maintainer describes three related deliverables: a symbolized 6502 assembly reconstruction that rebuilds the B3.0 ROM bit-for-bit, a CP/M-65 adaptation, and a portable C version. The source README states that printing and some platform-specific commands are missing and flags the original copyright as unresolved. The row is classified as `Decompilation` because it includes direct instruction-level reconstruction; the C port is described as a separate deliverable, not mislabeled as original high-level source.
- **Xbox Dashboard build 3944** is Microsoft system software, not a game. The project translates the dashboard's original x86 machine code to C and supplies a Windows host runtime. Its README is unusually explicit that it has not yet drawn the dashboard UI: it reaches its own clear operation but submits zero draw calls, then returns to firmware after a flash verification failure. The row is classified as `Recompilation` and does not claim a usable dashboard.
- **Theseus / UIX Desktop** reconstructs the same general Xbox system-shell target using a separate reverse-engineered C++ codebase. It runs as a drop-in dashboard on modded hardware and as a multi-platform desktop launcher/media center. It is classified as `Reimplementation`, not as a second copy of the build-3944 recompilation. Its last upstream commit is dated 2026-08-16; the README documents the current Xbox and desktop feature/platform scope.

## Existing metadata patch

`microsoft-bob-recomp` now records the current Windows/macOS/Linux host scope and more carefully distinguishes the documented real startup/database milestones from the still-missing rendered UI. Its repository's license covers only its own tools/runtime; the original binaries, data, and generated lifted code remain outside that grant.

## Leads excluded from this batch

- **Microsoft Word for Windows 1.1a x64 port (`jmarshall23/msword`)** appeared in search-indexed GitHub pages and recent news, but direct GitHub page and API requests returned 404 during this review. The upstream Computer History Museum source release remains available, but it is not evidence that the separate x64 port repository is presently available. It was therefore not added as a current project; recheck if the maintainer republishes it.
- **The Electronic Whole Earth Catalog (`sp00nznet/wholeearth`)** is a HyperCard content disc, not a decompilation/recompilation of a distinct application binary. Its own README says there is no Broderbund executable to recompile and points to HyperCard as the actual application target.
- **`macrecomp` and `pcrecomp`** are general recompilation toolchains, not application targets. This lane was explicitly limited to software projects rather than generic reverse-engineering/developer tools.
- Projects that merely appeared in search results, had no primary repository/status evidence, or were actually games were not staged.

The reviewed project set is not a claim that every historical application's reconstruction has been found; it is a source-backed incremental batch and should be continued with app-family searches (Macintosh, Win16, UNIX productivity software, and office/multimedia applications).
