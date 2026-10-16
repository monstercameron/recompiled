# Roadmap

Milestones are sequential and gated. Each milestone ends with documentation,
tests, and a manual verification pass before the next one starts.

```mermaid
flowchart LR
    M0[M0<br/>Bootstrap] --> M1[M1<br/>Behavioral analysis]
    M1 --> M2[M2<br/>Architecture spike]
    M2 --> M3[M3<br/>Viewer MVP]
    M3 --> M4[M4<br/>Albums]
    M4 --> M5[M5<br/>Maintenance]
```

## M0 — Bootstrap

Deliverables:

- Git repository, license (GPL-3.0-or-later), `.gitignore` that excludes
  proprietary binaries and scratch analysis output.
- Cargo workspace skeleton: `pinacoteca-app`, `pinacoteca-core`,
  `pinacoteca-decoders`, `pinacoteca-db`.
- Documentation set: vision, scope, roadmap, architecture, clean-room protocol.
- CI: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`.
- Tooling for M1: REA installed and registered, Ghidra available.

Exit criteria: workspace builds, CI green, docs reviewed.

## M1 — Behavioral analysis of the reference application

Goal: produce behavioral specifications for the in-scope features, without
writing implementation code.

Deliverables (in `docs/analysis/`):

- `viewer-behavior.md` — grid, fullscreen viewer, zoom/pan, slideshow, keyboard.
- `album-model.md` — how albums, tags, ratings and folder views behave.
- `maintenance-operations.md` — each edit operation: inputs, effect, parameters,
  ordering, reversibility.
- `thumbnail-pipeline.md` — observed thumbnail sizes, generation and caching
  behavior.
- `library-format-notes.md` — neutral notes on the reference library/database
  format (for potential future migration; not implemented in this milestone).

Out of scope for analysis: import, export, sharing, face recognition.

Exit criteria: specifications are sufficient for an implementer to build M3–M5
without consulting the binary.

## M2 — Architecture spike

Goal: settle the technical foundation with measurements, not assumptions.

Deliverables:

- Decoder benchmark on a real photo set (pure-Rust vs optional native decoders).
- GPU rendering pipeline prototype: mip/pyramid strategy for instant zoom.
- Thumbnail cache design and background worker prototype.
- Database schema v1 (catalog, albums, tags, ratings, edit stacks).
- Architecture Decision Records (ADRs) under `docs/adr/`.

Exit criteria: documented decisions with benchmark evidence; performance budgets
recorded in `docs/architecture.md`.

## M3 — Viewer MVP

- Folder scan and incremental indexing.
- Thumbnail grid (dense, dark, incremental) with keyboard navigation.
- Fullscreen viewer with GPU zoom/pan, EXIF orientation, fit/actual-size.
- Slideshow.

Exit criteria: opens a large real folder with an acceptable time-to-first-thumbnail
and smooth navigation.

## M4 — Albums and organization

- Virtual albums, add/remove via drag-and-drop and keyboard.
- Folder-as-album browsing.
- Tags and ratings.
- Smart albums (saved queries).

Exit criteria: organization completes without touching files on disk.

## M5 — Photo maintenance

- Rotate, crop, straighten.
- Auto-fix, brightness, contrast, saturation, warmth.
- Red-eye removal.
- Sharpen and effect presets.
- Non-destructive edit stack with undo/redo.

Exit criteria: all operations round-trip through the edit stack; originals are
byte-identical before and after (verified by test).

## Post-M5 candidates (not committed)

- Optional RAW / HEIC decoder packs.
- Migration of an existing third-party library.
- Additional format support based on user demand.
