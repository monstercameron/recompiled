# Catalog maintenance guide

This repository is a source-linked catalog about taking ownership of software through decompilation, recompilation, API translation, reimplementation, ports, and community preservation. Games are one part of the catalog; desktop applications and other software targets are in scope. Keep factual project metadata and source URLs in SQLite and reviewed research manifests; the Rust/WASM UI reads the generated JSON in `docs/`.

## Source of truth and build

- `data/catalog.sqlite` is the persistent catalog database.
- `data/research/*.json` contains reviewed, attributable imports and research snapshots.
- `docs/catalog.json` is generated from SQLite. Never edit it by hand.
- `docs/pkg/` and the WASM package are build outputs. Do not hand-edit them.
- `./scripts/build.sh` imports emulator additions, reviewed GitHub, console-phone, console-to-console, console/PC-to-mobile, and software application reconstruction batches, the consolidated reviewed project research, URI-only project media, sourced game-profile facts and Wikipedia links, project language evidence, the featured showcase, process resources, and the cached console gallery when its assets are present; it exports SQLite to `docs/catalog.json`, builds the Rust/WASM module, and writes its web bindings under `docs/pkg/`.
- For a local preview, run `./scripts/build.sh` and then `python3 -m http.server 8000 --directory docs`; open `http://localhost:8000/`.
- GitHub Pages publishes the `docs/` directory through `.github/workflows/pages.yml`.

The build script does not automatically import every research manifest. Import other reviewed project data explicitly, then rebuild:

```sh
python3 scripts/catalog.py import data/research/new-projects.json
python3 scripts/catalog.py export
./scripts/build.sh
```

Use the matching specialized importer for non-project tables:

```sh
python3 scripts/catalog.py import-emulators data/research/emulators.json
python3 scripts/catalog.py import-media data/research/media.json
python3 scripts/catalog.py import-links data/research/asset-links.json
python3 scripts/catalog.py import-game-profiles data/research/game-profiles.json
python3 scripts/catalog.py import-wikipedia-links data/research/wikipedia-links.json
python3 scripts/catalog.py import-programming-languages data/research/programming-languages.json
python3 scripts/catalog.py import-directory data/research/directory.json
python3 scripts/catalog.py import-featured-projects data/research/featured-projects.json
python3 scripts/catalog.py import-process-resources data/research/process-resources.json
python3 scripts/catalog.py import-console-gallery data/research/console-gallery.json
```

For the photo gallery, review each Wikimedia Commons file page and record its author, exact reuse license, attribution, and direct original-image URI in `data/research/console-gallery.json`. Cache only images with reviewed reuse terms by running `python3 scripts/cache-console-gallery-assets.py --manifest data/research/console-gallery.json`; run with `--dry-run` first. The script writes byte-identical originals under `docs/assets/`, records hashes, permits only Commons file-page provenance, and stops on rate limits so a later run can resume. Generate lightweight display copies with `python3 scripts/generate-console-gallery-previews.py`; this preserves originals and writes 760px JPEG previews that the gallery uses. The build checks that every cached console photo has a preview. Rebuild after caching so SQLite and the Rust/WASM gallery render the local asset paths and linked credits. Do not cache game art, game code, ROMs, or other unlicensed assets.

Run `python3 scripts/catalog.py --help` for the current command list. After changing Rust or catalog structure, run `cargo fmt -- --check`, `cargo test --locked --lib`, and `./scripts/build.sh`; also run `git diff --check`.

## Finding new projects

Use several discovery paths so the index does not only reflect one tracker or one platform:

1. Search GitHub for terms such as `decomp`, `decompilation`, `recomp`, `recompilation`, `static recompilation`, `application recompilation`, `binary lift`, `source port`, `game reimplementation`, `Metal Vulkan`, and platform/API pairs. Example:

   ```sh
   gh search repos 'static recompilation' --limit 100 --sort updated --order desc \
     --json name,owner,description,url,stargazersCount,language,updatedAt,isArchived,isFork
   ```

2. Use project trackers, maintainer blogs, release pages, official forums, X, Reddit (especially focused communities such as `r/decomps` and `r/emulation`), and videos as discovery leads. Search results and community posts are pointers, not proof of project status or implementation details.
3. Open the candidate's own repository, official project site, README, releases, compatibility list, or maintainer post. Confirm the target application, game, or system; set `target_type` to `Software` for application/software records and `Game` for game records. Verify project kind, actual method, current state, source availability, and supported host/target platforms from those sources.
4. Check the existing SQLite catalog by project URL, repository identity, and game before proposing a new row. One repository may cover multiple games; use one game record per title only when the evidence identifies separate targets. Merge aliases and duplicate discoveries under one canonical project record.
5. Keep exclusions and ambiguous cases in research notes rather than presenting them as verified projects. Do not mistake a general-purpose tool, tutorial, fork, mod, compatibility layer, VM, emulator, or source port for a game-specific recompilation.

GitHub CLI can quickly verify public repository metadata and recent activity:

```sh
gh api repos/OWNER/REPO --jq '{full_name,description,archived,fork,pushed_at,updated_at,stargazers_count,language,html_url}'
gh api 'repos/OWNER/REPO/releases?per_page=1' --jq '.[0] | {tag_name,published_at,html_url}'
gh api repos/OWNER/REPO/readme --jq .html_url
```

Treat `pushed_at` and `updated_at` as activity signals only. A recent commit can be a dependency bump, a dormant project can still be usable, and a release date does not prove compatibility. For a status refresh, read the latest maintainer-authored release notes, README/status page, compatibility matrix, and relevant issue or announcement; record the date checked in the research manifest. Use clear wording such as “active,” “archived,” “alpha,” “boots menus,” or “playable” only when a primary source supports it. If the latest evidence is thin or contradictory, say that status is unclear.

## Evidence and catalog rules

- Every project row needs a stable unique `id`, `title`, `game`, `platform`, `kind`, `status`, `summary`, `project_url`, and at least one HTTPS `sources` URL with a useful label.
- `target_type` is `Game` by default and must be `Software` for an application/software record; the required `game` field stores the target name for legacy compatibility.
- `kind` must match a supported project category in `scripts/catalog.py`: `Decompilation`, `Recompilation`, `API Translation`, `Fan Localization`, `Reimplementation`, or `Emulator`. The old value `Translation` is migrated to `Fan Localization`; do not use it for graphics/API compatibility work.
- Describe the method accurately. Annotated assembly disassembly is not a matching decompilation; a source port is not automatically a recompilation; an emulator or VM is not an API translation. A Metal-to-Vulkan runtime may be an API translation even when it is not a game-specific port.
- Link directly to the project repository/site and add separate source links for status, supported versions/platforms, and releases when available. Social posts can document an announcement, demo, or community activity, but link to the maintainer's own project evidence for durable claims.
- Record `project_started` only when a reliable source states the date. `first_found` is when this catalog discovered the record; it is not the project's start date. Leave unknown dates blank rather than estimating.
- For project/game media, store original HTTPS URLs plus the page that links to them; do not copy those media files locally. Console-gallery photos are the explicit exception: only use Commons images whose file page documents a reuse license, keep original bytes unchanged, and retain creator, license, attribution, file-page URL, and original image URL alongside each cached file. Do not copy game code, ROMs, or other unlicensed game assets.
- Popularity values are dated snapshots. For GitHub rankings, query each actual repository (`gh api repos/OWNER/REPO`) and retain that repository's API URL as evidence; do not rank by keyword-search order or counts from a generic topic page. Cross-platform views, likes, votes, and stars are different signals and are not directly comparable.
- Keep summaries neutral and distinguish verified behavior from project goals. Do not turn a maintainer's experimental claim into a compatibility guarantee.

When adding records, save the reviewed manifest under `data/research/`, confirm its JSON parses and IDs/URLs do not duplicate the catalog, import it, export the catalog, run Rust checks, rebuild the site, and inspect the local page at desktop and mobile widths. See `CONTRIBUTING.md` for the project record example and the step-by-step add/update workflow.
