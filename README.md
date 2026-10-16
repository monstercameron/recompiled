# Recompiled

[![GitHub Pages](https://github.com/monstercameron/recompiled/actions/workflows/pages.yml/badge.svg?branch=main)](https://github.com/monstercameron/recompiled/actions/workflows/pages.yml) [Open the live catalog](https://monstercameron.github.io/recompiled/)

Recompiled is a source-linked field guide to taking ownership of software: recovering code, statically recompiling binaries, rebuilding applications, translating APIs across platforms, and preserving community changes. Games are one part of the index; desktop applications and other software targets belong here too. Entries explain how projects work, where their source lives, which platforms they support, and how mature they are.

The catalog is maintained in SQLite at `data/catalog.sqlite`. Reviewed research records live in `data/research/`; `scripts/catalog.py` exports the browser-ready `docs/catalog.json`. The interface, filters, detail views, and localization are written in Rust and compiled to WebAssembly. GitHub Pages publishes the static site from `docs/`.

## Explore the site

Use the catalog to search and filter projects by target type, project type, source and target platform, method, status, language, discovery date, and other metadata. Project details link to their repositories, releases, source evidence, and media on the original host. The site also includes game profiles, project spotlights, learning resources, and a console photo gallery.

Project images and videos are stored as source URLs and loaded from their original hosts. The gallery is the exception: it caches console photographs only when their Commons file pages document reuse terms, while preserving attribution and license information. The catalog does not include ROMs, commercial application files, or original binaries.

## Run locally

Install Python 3 and the stable Rust toolchain, then add the WebAssembly target and the matching `wasm-bindgen` command-line tool:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.129 --locked
```

Build and serve the GitHub Pages directory:

```sh
./scripts/build.sh
python3 -m http.server 8000 --directory docs
```

Open <http://localhost:8000/>. Use an HTTP server so the browser can fetch the JSON catalog and WebAssembly module.

For a quicker edit loop after importing or exporting catalog data, serve `docs/` with the same Python command. Rebuild the WASM package with `./scripts/build.sh` whenever Rust source changes.

Before submitting a change, run `cargo fmt -- --check`, `cargo test --locked --lib`, `./scripts/build.sh`, and `git diff --check`.

## Contribute catalog records

Read [CONTRIBUTING.md](CONTRIBUTING.md) before adding or updating a project. It describes the record format, source checks, categories, duplicate checks, and validation commands. [AGENTS.md](AGENTS.md) has the broader research and maintenance workflow, including how to discover new projects and refresh status.

The short version is to verify claims on a project's own repository, release page, or website; add a reviewed JSON record under `data/research/`; import it into SQLite; export the catalog; and run the checks. Preserve an existing project ID when refreshing that project. Do not edit `docs/catalog.json` by hand.

## GitHub Pages

GitHub Actions builds the Rust/WebAssembly application, validates the catalog tooling, runs the Rust tests, and deploys `docs/` to [the live catalog](https://monstercameron.github.io/recompiled/) on every push to `main`. You can also start a deployment from the [Actions tab](https://github.com/monstercameron/recompiled/actions/workflows/pages.yml) with **Run workflow**.

## License

The website code is available under the MIT License in [LICENSE](LICENSE). The catalog metadata is released under [CC0 1.0](licenses/catalog-CC0.txt). Linked projects, media, and gallery photographs retain their own licenses and attribution requirements; see each project's source links and the console-gallery records.
