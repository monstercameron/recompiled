# Pinacoteca

**A fast, clean-room reimplementation of the photo viewing, album organization and
photo maintenance experience that Picasa offered.**

Pinacoteca is a native desktop application focused on one thing: browsing and
organizing a local photo library with excellent performance and a pleasant,
immediate workflow. It is written in Rust and renders through the GPU.

> **Pinacoteca is an independent, clean-room reimplementation inspired by Picasa.
> It is not affiliated with, endorsed by, or derived from Google. It contains no
> Google source code or binaries.** See [Legal & clean-room](#legal--clean-room).

---

## Why

Picasa was discontinued in 2016. For many people it remains the fastest and most
comfortable way to browse, organize and lightly edit a local photo collection:
an instant thumbnail grid, a fullscreen viewer that keeps up with the keyboard, and
non-destructive tweaks that never touch the original files.

Pinacoteca rebuilds that experience from the ground up — modern toolchain, modern
formats, GPU-accelerated rendering — while keeping the original photos untouched
and the library local and private.

## Goals

- **Performance first.** Large folders open incrementally. Scrolling, zooming and
  panning stay smooth because image decoding and rendering happen off the critical
  path and on the GPU.
- **Great to use.** Keyboard-driven, low-friction navigation that respects the
  user's attention. The photo, not the chrome, is the interface.
- **Non-destructive by design.** Editing operations are stored as an edit stack in
  a local database. Original files are never modified.
- **Local and private.** No accounts, no uploads, no telemetry. The library lives
  on your machine.

## Scope

See [docs/scope.md](docs/scope.md) for the full in-scope / out-of-scope tables.

**In scope (initial milestones)**

- Image viewing: thumbnail grid and fullscreen viewer with zoom, pan and slideshow.
- Album organization: virtual albums, tags, ratings, smart albums, folder views.
- Photo maintenance: rotate, crop, straighten, auto-fix, red-eye removal,
  brightness / contrast / saturation / warmth, sharpen and basic effects.

**Explicitly out of scope (for now)**

- Import from cameras, devices or other applications.
- Export, sharing and upload to any online service.
- Facial recognition and face grouping.
- Cloud synchronization and backup.

These areas are considered dated or obsolete for the project's purpose and are
intentionally excluded from the initial roadmap.

## Status

Early development. The project is currently in its bootstrap and analysis phase —
see [docs/roadmap.md](docs/roadmap.md) for the milestone plan and
[docs/architecture.md](docs/architecture.md) for the technical design.

| Milestone | Description | State |
|-----------|-------------|-------|
| M0 | Project bootstrap: repository, docs, CI, tooling | In progress |
| M1 | Behavioral analysis of the reference application | Pending |
| M2 | Architecture spike: decoders, GPU pipeline, benchmarks | Pending |
| M3 | Viewer MVP: grid + fullscreen viewer | Pending |
| M4 | Albums and organization | Pending |
| M5 | Photo maintenance (non-destructive edits) | Pending |

## Technology

- **Language:** Rust
- **UI:** `egui` / `eframe` on a `wgpu` backend, with custom GPU image rendering
- **Decoding:** pluggable decoder layer (pure-Rust by default; optional RAW / HEIC)
- **Storage:** SQLite for catalog, albums, tags and edit stacks
- **Thumbnails:** content-addressed on-disk cache with a background worker pool

Details, benchmarks and the reasoning behind these choices live in
[docs/architecture.md](docs/architecture.md).

## Building

```sh
# Requires a recent stable Rust toolchain (see rust-version in Cargo.toml).
cargo build --release
cargo run -p pinacoteca
```

Run the full check suite:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Documentation

| Document | Contents |
|----------|----------|
| [docs/vision.md](docs/vision.md) | Product vision and guiding principles |
| [docs/scope.md](docs/scope.md) | In-scope and out-of-scope feature tables |
| [docs/roadmap.md](docs/roadmap.md) | Milestones and delivery order |
| [docs/architecture.md](docs/architecture.md) | Technical architecture and decisions |
| [docs/cleanroom-protocol.md](docs/cleanroom-protocol.md) | How clean-room analysis is conducted |
| [docs/analysis/](docs/analysis/) | Behavioral findings from the reference application |

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) and the
[clean-room protocol](docs/cleanroom-protocol.md) before opening a pull request.
The clean-room rules are a hard requirement, not a suggestion.

## Legal & clean-room

Pinacoteca is an independent reimplementation. It is not derived from Google
source code, and no Google code, binaries or proprietary assets are included in
this repository or its releases.

The reference application (Picasa) is proprietary software owned by Google. It is
used here strictly as a **behavioral reference** under a clean-room process: its
observable behavior is documented, and the implementation is written from those
behavioral specifications alone. Original photo files are never modified.

This project is not affiliated with, endorsed by, or sponsored by Google. "Picasa"
is a trademark of Google LLC and is used only descriptively to state what this
project is inspired by.

## License

Licensed under the [GNU General Public License v3.0](LICENSE)
(`GPL-3.0-or-later`).
