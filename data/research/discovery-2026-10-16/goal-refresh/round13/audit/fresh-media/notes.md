# Fresh media research — 2026-10-16

This batch fills zero or weak media coverage for eight catalog records: Wild 9 Recompiled, Hissatsu!, SEGAGAGA, CRW: Metal Jacket, Urban Reign Recomp, Zelda 64: Recompiled, Banjo: Recompiled, and Unleashed Recompiled. `media.json` follows the existing importer shape (`id` plus `media[]` entries with `kind`, `label`, `url`, `preview_url`, `source_url`, and `discovered_on`).

Media links come from project README galleries, project-linked GitHub pages, or videos uploaded by project maintainers. The screenshot asset bytes were not saved; validation used HTTP HEAD metadata only. YouTube pages were used as the video URIs, and the standard YouTube thumbnails as previews. The repo license applies to code where stated; it does not establish reuse rights for game footage or screenshots. Several translation projects have no repository/media license statement. Treat all linked images and video as remote references and do not rehost them without separate rights confirmation.

| Catalog ID | Coverage added | Source and rights check |
|---|---|---|
| `wild-9-ps1-recompiled` | Two README gameplay screenshots | Repository code is MIT. README gallery labels the screenshots Level 4 and Level 1; game-media license is not separately stated. |
| `hissatsu-saturn-en` | Title, gameplay, and dialogue screenshots | README embeds the assets and credits texture art to Exxistance. No repository or media license was found. |
| `segagaga-dreamcast-en` | Three README gallery screenshots | Used the project lead’s original repository. Images have no alt text; labels identify gallery positions. No repository/media license was found. |
| `crw-metal-jacket-pc98-en` | Original title-screen image | Project README links it and says the screenshot is unlocalized. The README uses HTTP; the matching HTTPS endpoint passed HEAD. No license was found. |
| `urban-reign-ps2-recomp` | Project-authored status map SVG | README links the map; code is GPL-3.0. The README does not separately state the SVG’s media terms. |
| `zelda64recomp` | Release trailer, 1.2 modding trailer, and Steam Deck gyro setup screenshot | Code is GPL-3.0. Trailer uploads are attributed to maintainer Wiseguy; no separate rehosting license is asserted. |
| `banjo-recompiled` | Maintainer release trailer | Code is GPL-3.0. YouTube metadata identifies Wiseguy as uploader; no separate rehosting license is asserted. |
| `unleashed-recompiled` | Maintainer release trailer | Code is GPL-3.0. YouTube metadata identifies lead developer Skyth as uploader; no separate rehosting license is asserted. |

All 15 preview image URIs returned HTTP 200 with an `image/*` content type via HEAD. See [`sources/link-checks.json`](sources/link-checks.json) and [`sources/sources.json`](sources/sources.json) for per-link provenance and license notes. The four YouTube pages were independently identified in YouTube search metadata as creator/maintainer trailers; their thumbnails also passed HEAD checks.

## Second pass: high-signal projects across the catalog

The authoritative read-only catalog contained 3,462 projects, of which 3,217 had no media. I screened the first 100 high-signal empty records (70 Game, 30 Software), prioritizing featured/widely used and recent records. All 100 maintainer GitHub pages returned HTTP 200. Their READMEs contained 15 image references and no video references. I also reviewed 20 linked homepages (16 returned 200, one 308, one 403, and two failed TLS/timeout); the HTML exposed 60 image references and one video reference. No official or maintainer YouTube link was found.

Five projects yielded nine source-linked images that passed HTTP HEAD checks and did not duplicate live database media or other staged manifests. No asset bytes were downloaded or cached. Their source pages and rights notes are in [`sources/sources.json`](sources/sources.json), with HEAD results and filtered README references in [`sources/link-checks.json`](sources/link-checks.json). The complete selection, no-media IDs, scan counts, and exclusion notes are in [`second-pass/audit.json`](second-pass/audit.json).

| Catalog ID | Added media | Provenance and rights note |
|---|---|---|
| `wordperfect-8-unix-linux-compatibility-port` | Four README interface screenshots/demos | No repository or screenshot license identified; README calls it a demonstration copy without granting reuse terms. |
| `reactos-windows-nt-os-reimplementation` | Official-homepage desktop screenshot | README identifies GPL-2.0 for code; no separate screenshot license identified. |
| `ssx-tricky-opentricky-xbox-windows-recomp` | Two official-homepage SSX Tricky gameplay screenshots | Project code is GPL-3.0-only; README says no game media is distributed and does not grant screenshot reuse rights. |
| `ppsspp` | Official-homepage hero image | Project code is GPL-2.0-or-later; no separate hero/game-art license identified. |
| `claude-desktop-zh-cn` | Maintainer README localization showcase | Repository is MIT licensed; no separate product-interface image license identified. |

The other 95 selected records had no usable, link-validated project media for this batch. Among the README references, GitHub user-attachment URLs for DevilutionX, Tokimeki, OpenTricky, and reNEVADA360 returned HTTP 403 to HEAD; two Devilution links pointed into the separate Scalpel project, while MAME and Gitpod references were badges. Media licensing is recorded separately from code licensing throughout, and no reuse rights are inferred.
