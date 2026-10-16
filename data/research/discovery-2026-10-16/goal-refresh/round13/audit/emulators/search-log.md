# Additional emulator / FPGA-core discovery — 2026-10-16

## Scope and baseline

This continuation pass used the authoritative SQLite snapshot of 3,462 project rows and 534 rows with `kind=Emulator`. The database was opened read-only. It also compared against project-addition manifests under `/Users/earlcameron/Desktop/recompiled/work` and the current Documents/Codex work area, including the prior round-13 emulator batch.

## Search channels and query log

- Web/GitHub search: `site:github.com/*/* emulator 2026 retro console emulator new GitHub`; `site:github.com/MiSTer-devel MiSTer core 2026 FPGA emulator`; `GitHub emulator released 2025/2026 open source`; `Tang Nano 20K FPGA core emulator`.
- Web/GitHub search: `site:github.com/MiSTer-devel/ MiSTer FPGA Core 2026`; `site:github.com MiSTer core FPGA README new 2026`; `site:github.com Tang Nano 20K FPGA core emulator retro`; `site:github.com FPGA core emulator retro 2025 2026 console open source`.
- Web/GitHub search for other fresh software-emulator leads: GameTank, NES Go, Hemulator, RustyNES, MiSTeryNano, MSXnano, GBTang, NESTang, C64Nano, and hybrid PICO-8.
- Primary GitHub inventory: MiSTer-devel organization repository-list snapshot retrieved 2026-10-16 at `https://api.github.com/orgs/MiSTer-devel/repos?per_page=100&page=N` (407 repositories). Repository pages, repository-root README/LICENSE text, Releases pages, and exact `/languages` API endpoints were checked for selected candidates.

## Inclusion rules

Selected only actual HDL/FPGA cores or emulators, with source under the maintained non-fork MiSTer-devel organization roots. Core families supporting several titles are `target_type=Software`; single-game/one-franchise cores (including WIP Alpha Mission and Salamander/Life Force) are `target_type=Game`. The target-system string is unique within the batch. README caveats, playable-title limits, no-formal-release state, and missing license files are called out rather than hidden. All staged rows have a direct repository page, README source where present, metadata provenance, release page, and exact GitHub Languages API map.

## Candidate set

20 candidates: 15 multi-game system/board cores classified Software and 5 currently game-specific cores classified Game. The strongest documentation comes from the current core READMEs: e.g., Irem M92, Taito F2, Sega System 1, MCR1/2/3, Sega G80V, Sega VCO, Sega VIC Z80, and the dedicated game cores. `Arcade-Deco16` and `SNK Triple Z80` are included with explicit WIP/accuracy caveats.

## Exclusions and dedupe

`exclusions.json` records exact catalog roots, duplicate target platforms, forked repos, prior staged roots, and the empty `Arcade-CV1k` repository. The repository-root audit reports zero matches against both the 3,462-row SQLite catalog and scanned prior project manifests; batch target-system keys are unique. Related generic MAME coverage is not treated as an exact system-core duplicate because these rows identify dedicated FPGA cores and their own hardware/platform scope.

## Outputs

- `new-projects.json` — importer-shaped candidate rows.
- `github-byte-language-evidence.json` — exact language-byte maps from GitHub Languages API (20/20 HTTP 200).
- `candidate-source-evidence.json` — fact-level scope/status/platform/license/release evidence and target-type rationale.
- `repo-snapshots/` — organization metadata, current primary README/license text, root-file evidence, Releases-page status, and the exact language map for each candidate.
- `exclusions.json`, `deduplication-audit.json`, and `validation-report.json`.
