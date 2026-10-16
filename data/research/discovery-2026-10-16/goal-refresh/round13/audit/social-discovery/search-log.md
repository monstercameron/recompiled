# Social and tracker discovery review

Checked 2026-10-16. Social posts and community trackers were treated as discovery leads, not as project proof. New project rows below are supported by maintainer-owned repositories, READMEs, release notes, licenses, repository metadata and exact GitHub Languages API responses. No project datasets were copied from discovery trackers.

## Discovery leads and decisions

- [r/decomps RecompBoard thread](https://www.reddit.com/r/decomps/comments/1x607hp/recompboard_decomps_recomps_and_ports_in_one/) was reviewed as a source-finding method only. It points at recomp.fyi, already present as `recomp-fyi`; the thread's records were not copied.
- [r/decomps decomp.dev sunset discussion](https://www.reddit.com/r/decomps/comments/1x66xrd/decompdev_is_finished/) led to two distinct successor/alternative sites, [decompilati.onl](https://decompilati.onl/) and [ByteMatch](https://bytematch.dev/). Their own repositories and live sites were checked; both are entered only as directory pointers, not as project datasets.
- [PCMR LATAM's X post](https://x.com/PCMR_LATAM/status/2110983153418641846) links its [native PC port/recomp/decomp article](https://www.pcmrace.com/2026/10/06/lista-pc-port-nativo-decomp-recomp/) and self-reports 300+ new or updated rows, including more precise required-file/version details. The X page text loaded, but the article returned HTTP 403/Cloudflare. The article is kept as a qualified discovery pointer with the limitation stated; no list entries or count are independently asserted.
- [r/decomps Ultra Street Fighter IV post](https://www.reddit.com/r/decomps/comments/1ws249o/ultra_street_fighter_iv_running_natively_on_pc/) linked a repository that returned GitHub 404; excluded because no maintainer source could be verified.
- [Rayman Origins Android/native discussion](https://www.reddit.com/r/emulation/comments/1wpyfv4/rayman_origins_recompiled_from_xbox_360_to_native/) was checked against `XDanfr/RaymanOriginsRecomp`; that canonical project is already in the database with Android and Apple Silicon details, so no duplicate row or update was staged.
- A YouTube/community tutorial scan did not yield a new process resource with defensible top-five/popularity evidence. No process-resource manifest is included because that specialized importer requires an exact five-resource roster.

## Maintainer-source-confirmed project additions

- `sp00nznet/sfrush` is a new, distinct San Francisco Rush: Extreme Racing (1997 N64) project, not the existing San Francisco Rush 2049 records. Its README calls it static recompilation via N64Recomp and reports 1,014 functions/356,001 generated C lines, but still marks runtime wiring and title-screen boot as future work. The project README, MIT-scope LICENSE, GitHub metadata, empty releases endpoint and exact language map were snapshotted.
- `atsb/NakedAVP` is a distinct modern SDL3 source port of Aliens versus Predator Gold, separate from the existing 3DS project. Maintainer README and release `1.3.1` (2026-10-16) support a source-port/reimplementation record; repository LICENSE restrictions and external game-data requirement are preserved. Exact repository metadata and language map were snapshotted.

## Tracker review

- [Decompendium](https://decompendium.com/) is a byte-matching decompilation index grouped by game with progress from each project's CI; the official README and live catalog were checked.
- [Recomp](https://recompsoftware.com/) is a broader live tracker for decompilation, recompilation, clean-room and preservation work; its homepage was checked on 2026-10-16. `recomp.fyi` remains a separate existing item.
- [decompilati.onl](https://decompilati.onl/) is a narrower decompilation-progress tracker. Its own README documents project-owner/CI reports; the live homepage listed three projects during this check.
- [ByteMatch](https://bytematch.dev/) is an open-source tracker for matching decomp projects using objdiff reports. Its README documents pages, history, API, badges, PR comments and decomp.dev history migration.
- `decomp.dev` itself was not added again; its existing catalog row is an archive pointer and the site's sunset notice says new projects cannot register.

## Scope and dedupe

SQLite and all tracked files were left untouched. Proposed project IDs and canonical GitHub URLs were checked against the current catalog and visible Round 13 manifests. The two project rows have no exact URL/ID duplicates; the San Francisco Rush title is distinct from Rush 2049 and Rush 2049 Recompiled. NakedAVP shares the AvP Gold game target with the separate 3DS port but is a distinct canonical repository and host-platform project. New tracker URLs/IDs do not collide with existing directory rows.
