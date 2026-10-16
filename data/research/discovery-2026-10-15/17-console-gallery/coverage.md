# Console gallery Commons coverage audit

Audit date: 2026-10-15. The reviewed source is `data/research/console-gallery.json`. This work only adds research artifacts under this discovery folder and a trial copy under `work/console-gallery-trial-20261015`; it does not change the source manifest, existing gallery assets, catalog, or SQLite database.

## Coverage

The manifest contains 18 maker-logo records, 2 source-logo records, and 56 console/add-on photos. Three maker logos already exist locally and their SHA-256 values match the manifest. No other referenced asset exists at its expected local path.

| Asset group | Manifest records | Cached | Missing |
| --- | ---: | ---: | ---: |
| Maker logos | 18 | 3 | 15 |
| Source logos | 2 | 0 | 2 |
| Console and add-on photos | 56 | 0 | 56 |
| **Total references** | **76** | **3** | **73** |

The 76 references point to 72 unique Commons file pages because several maker entries reuse the same logo file. All 72 pages resolved through Commons `imageinfo`; none were missing. Every manifest image URL points to the matching file page after normalizing URI percent-encoding. The audit output records the resolved page title, canonical original URL, file size, Commons SHA-1, license label/URL, creator, and image description for every reference in `commons-audit.json`.

All 56 system records use the `wikimedia-commons` source-logo ID. The GitHub mark is present in the source-logo table but is not referenced by these console records.

## Corrections in the validated copy

The three flagged console-photo URLs contain literal parentheses, while Commons returns those path characters percent-encoded. They identify the same file after URI normalization, but the validated copy uses Commons' canonical form:

| Record | Commons page | Validated original URL |
| --- | --- | --- |
| `32x` | [Sega MegaDrive MegaCD 32X (26017594293).jpg](https://commons.wikimedia.org/wiki/File:Sega_MegaDrive_MegaCD_32X_%2826017594293%29.jpg) | `https://upload.wikimedia.org/wikipedia/commons/5/5d/Sega_MegaDrive_MegaCD_32X_%2826017594293%29.jpg` |
| `atari-5200` | [Atari 5200 (7973364982).jpg](https://commons.wikimedia.org/wiki/File:Atari_5200_%287973364982%29.jpg) | `https://upload.wikimedia.org/wikipedia/commons/0/0c/Atari_5200_%287973364982%29.jpg` |
| `fairchild-channel-f` | [Fairchild (7973361770).jpg](https://commons.wikimedia.org/wiki/File:Fairchild_%287973361770%29.jpg) | `https://upload.wikimedia.org/wikipedia/commons/c/cc/Fairchild_%287973361770%29.jpg` |

The GitHub source image URL itself matches Commons. Its file metadata names GitHub as creator and identifies the Octicons mark as MIT licensed. The original manifest linked the current Open Source Initiative MIT page; Commons metadata exposes the older `http://opensource.org/licenses/mit-license.php` URI. The validated copy uses its HTTPS form, which returns successfully and redirects to the current official MIT license page. Its attribution now follows the Commons creator/license metadata more directly.

The resulting separate manifest is `console-gallery-validated.json`. It retains all records and asset paths, changing only the three canonical photo URLs and GitHub source-logo license metadata. The cache script's `--dry-run` accepted this manifest and confirmed that the three existing files still match their recorded hashes; it made no file or database changes.

## Cache script and bounded trial

`scripts/cache-console-gallery-assets.py` downloads one file at a time, validates the response host, size, image signature, declared MIME type, and (for SVG) a restricted XML tree. It installs files atomically without replacing an unrecognized existing file, records SHA-256, pauses 3 seconds after each successful cache, and stops on HTTP 429 or 503 while reporting `Retry-After`. This is a conservative rate policy; it deliberately does not retry a throttled request. A metadata pass can stay small: the 72 distinct file pages were checked with two Commons API requests of 40 and 32 titles, `maxlag=5`, and a two-second pause between requests.

Two limits matter before using it for a full run:

- The script does not query Commons to prove that each image URL belongs to its stated file page. Keep the metadata audit as a prerequisite and use the corrected copy.
- A `--manifest` path alone does not isolate a run. Asset paths and cache state are still rooted at the script's own repository. The trial therefore used a copied script and reduced manifest under `work/console-gallery-trial-20261015`.

The CLI trial copy stopped safely on `maker-sega`: Commons' `SEGA_logo.svg` begins with a standard SVG DOCTYPE, and the current parser rejects every DOCTYPE before parsing. It published no file. This is a blocker for a complete logo run until SVG validation can accept harmless external-DTD declarations without fetching external resources or allowing entity expansion; do not disable the safety check or rewrite source bytes to bypass it.

A bounded single-image trial then used the script's existing `download()` helper on the GameCube console photo. The Commons description identifies a console, controller, and memory card; visual inspection confirmed the downloaded sample contains no game art. The result is isolated at `work/console-gallery-trial-20261015/docs/assets/consoles/gamecube.jpg` and is not in `docs/assets`.

| Check | Result |
| --- | --- |
| Commons file page | [Gamecube-console.jpg](https://commons.wikimedia.org/wiki/File:Gamecube-console.jpg) |
| Creator and license | Evan-Amos; public domain |
| Download size and format | 365,539 bytes; JPEG |
| Commons SHA-1 | `b435235a33374124dd7156b52d120c7d6a786eb5` |
| Download SHA-1 | `b435235a33374124dd7156b52d120c7d6a786eb5` (exact match) |
| Download SHA-256 | `4b83bb47355728fe68399eee22ec72fbe49037954db7cd863b6e792d3aa86117` |

This confirms byte-identical caching for a permitted raster image. It does not establish that all 56 candidates are visually free of incidental game artwork; only the GameCube sample received visual review. Keep image review and the SVG DTD handling issue in scope before a full cache run.
