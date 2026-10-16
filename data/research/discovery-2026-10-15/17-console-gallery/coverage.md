# Console gallery Commons coverage audit

Audit date: 2026-10-15. The reviewed gallery source is `data/research/console-gallery.json`. This follow-up replaced three console photos after visual review. It changed only the gallery manifest, cache-state journal, reviewed Commons evidence, coverage notes, and those three cached images. SQLite, generated `docs/catalog.json`, the cache script, and unrelated assets were not changed.

## Coverage and cache result

The manifest contains 18 maker-logo records, 2 source-logo records, and 56 console/add-on photos. All referenced assets are cached locally. The script verified retained assets by their recorded hashes and fetched the three replacement photographs one at a time with its built-in three-second pause.

| Asset group | Manifest records | Cached | Missing |
| --- | ---: | ---: | ---: |
| Maker logos | 18 | 18 | 0 |
| Source logos | 2 | 2 | 0 |
| Console and add-on photos | 56 | 56 | 0 |
| **Total references** | **76** | **76** | **0** |

The Commons audit now contains 81 evidence records: 78 current records and 3 historical photo records marked `superseded`. The active records cover 74 distinct Commons file pages because multiple maker records reuse the same logo. Every one of the 76 local asset paths matches its manifest SHA-256 and cache-state entry. Each console photo also matches the SHA-1 in the corresponding current Commons `imageinfo` audit record. The cached paths total 184,778,561 bytes. All cache transformation fields state that source bytes were retained unchanged.

The final `--dry-run` verified all 76 manifest entries as already cached and hash-matched. The live replacement run exited successfully; Commons returned no rate limit. The cache-state journal has one verified entry for each of the 76 asset paths.

All 56 system records use the `wikimedia-commons` source-logo ID. The GitHub mark is present in the source-logo table but is not referenced by these console records.

## Source and license corrections

The three visually flagged photos were replaced with hardware-only views that do not show game scenes on a display:

| Record | Commons File page | License | Creator |
| --- | --- | --- | --- |
| `32x` | [Mega Drive 32X (8568455128).jpg](https://commons.wikimedia.org/wiki/File:Mega_Drive_32X_%288568455128%29.jpg) | CC BY 2.0 | Chapuisat from España |
| `nec-pc-engine-cdrom2` | [Super CD-ROM2 with CoreGrafx II (3-4 left view).jpg](https://commons.wikimedia.org/wiki/File:Super_CD-ROM2_with_CoreGrafx_II_%283-4_left_view%29.jpg) | CC BY-SA 2.0 | Bryan Ochalla from Seattle, Washington |
| `xbox-series-x` | [Xbox Series (X).jpg](https://commons.wikimedia.org/wiki/File:Xbox_Series_%28X%29.jpg) | CC BY-SA 4.0 | Der. Bellemer |

The cache stores the exact Commons original bytes. The audit records each replacement's current image URL, Commons `imageinfo` original URL, MIME type, file size, SHA-1, and license metadata. Prior photo evidence remains in the audit, marked superseded, for traceability.

The original Sega maker SVG and 3DO maker SVG were rejected by the cache script because their source files contain a DOCTYPE. Their replacements are separately reviewed Commons raster file pages: [SEGA logo.png](https://commons.wikimedia.org/wiki/File:SEGA_logo.png) and [3DO Company logo.png](https://commons.wikimedia.org/wiki/File:3DO_Company_logo.png). Commons metadata identifies both as public-domain text logos; attribution keeps the source description and trademark notice. The script's SVG active-content protections were not changed.

The GitHub source image URL matches Commons. Its file metadata names GitHub as creator and identifies the Octicons mark as MIT licensed. The validated copy uses Commons' HTTPS form of the MIT license URI. Its attribution follows the Commons creator/license metadata.

## Cache safety and visual review

`scripts/cache-console-gallery-assets.py` downloads one file at a time, validates the response host, size, image signature, declared MIME type, and (for SVG) a restricted XML tree. It installs files atomically without replacing an unrecognized existing file, records SHA-256 before publishing, pauses three seconds after each successful cache, and stops on HTTP 429 or 503 while reporting `Retry-After`. The script was not changed and its DOCTYPE rejection was not weakened.

A visual review of all 56 console/add-on photos found that the three earlier findings are resolved: the 32X photo shows the add-on installed on a Mega Drive, the NEC photo shows the CoreGrafx II and Super CD-ROM² hardware against a plain background, and the Xbox photo shows the console/controller with Xbox-branded packaging. None shows game imagery on a display. The other 53 photos appear to show hardware without visible game artwork. The separate bounded GameCube trial is retained under `work/console-gallery-trial-20261015`; it is not part of `docs/assets/`.
