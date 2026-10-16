# GitHub, GitLab, and Codeberg long-tail discovery

Reviewed 2026-10-15. This batch captures new or recently active project repositories outside the already-staged high-level API-translation, emulator, localization, and social-discovery categories. It includes 47 project records, two status refreshes, and no copied code, game data, screenshots, or other media. External asset references remain URLs only.

## Search and verification

Search used GitHub CLI repository search sorted by recent activity for `decompilation`, `recompilation`, `reimplementation`, and `native port` with `pushed:>2026-08-01`, followed by direct repository metadata, README, release, and language-endpoint checks. Focused GitLab API searches covered `decompilation`, `recompilation`, `recomp`, and `game port`; candidate READMEs were fetched from the project repositories and language endpoints. Codeberg’s public repository-search API was also queried for `decomp`; it returned no recent game/software reconstruction projects in scope.

GitHub’s pushed timestamp was treated as a discovery signal only. Status statements in the manifest come from maintainers’ READMEs, progress reports, or release pages. For the Atari ST repository, each game has a separate catalog row because the README identifies six separate game targets and reports their progress independently. The repository’s current language endpoint lists Python as its largest byte total, while its game reconstruction uses C; this distinction is preserved in the description.

The record source set includes the maintainer repository page and the repository language breakdown. Where a source exposes a stronger project-specific progress report or release page, that link is included as well. All projects use `first_found: 2026-10-15`; `project_started` remains unknown unless the primary source explicitly states a start date. The project-status patches use only fields supported by the catalog updater.

## Dedupe and exclusions

The 47 staged IDs and canonical repository URLs were compared with `data/catalog.sqlite` and JSON project manifests under `data/research/` and `work/` at staging time. No ID or repository URL collisions were found. The six Atari entries intentionally share one repository URL but identify six distinct games. There are no duplicate IDs. The two metadata updates reference existing catalog IDs `flow-ps3-recomp` and `fable-the-lost-chapters-pc-decomp`; neither project update was already present in the reviewed discovery update files.

Other candidates were excluded or treated as duplicates:

- FIFA Street PC (`samuelit0davila/fifa-street-pc-port`) and the Saints Row Xbox-to-Linux static recomp (`HttpAnimations/saints-row-recomp`) already have catalog rows, so they were not duplicated. The separate Saints Row matching-decompilation repository is included. The final pass also added the Dragon Ball GT: Final Bout PS1 recomp, Rival Schools Evolution Disc recomp, Prince of Persia: The Sands of Time Xbox recomp, Army of Two: The 40th Day proof-of-concept recomp, and Final Fantasy Tactics native-port lab after checking their canonical project URLs.
- `Caesar0007/NFSHS-PSX-decomp`, `Megami-Decomps/dds-decomp`, `BuffJesus/FableDecomp`, and `mstan/SuperMetroidRecomp` already exist in the live catalog. Fable is refreshed in the status-update manifest instead of added again.
- `Moddimation/Yasiki` explicitly calls itself outdated and points to a newer Luigi’s Mansion project; the current `SomeJakeGuy/lm-decomp` repository is used.
- The GitLab Conker repository was excluded: its README says it is analysis-only and its source directory is empty, so it does not yet establish a decompilation source build.
- GitLab “ecoscan” Perfect Dark / GoldenEye mirrors with deletion-scheduled project names were excluded as unstable duplicate mirrors rather than independent maintained projects.
- Other generic decompilers, source-port frameworks, repository directories, and compatibility tools were not listed as game projects unless a repository identified a concrete target and documented project-specific behavior.

No SQLite import, generated catalog edit, or site build was performed. Parent integration should rerun the duplicate audit immediately before import because other research lanes may finish staging after this check.
