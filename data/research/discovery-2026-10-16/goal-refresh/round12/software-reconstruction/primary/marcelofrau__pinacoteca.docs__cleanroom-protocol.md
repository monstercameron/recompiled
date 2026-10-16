# Clean-room protocol

Pinacoteca is a clean-room reimplementation. This document defines the rules that
keep it clean. They are a hard requirement: a contribution that violates them
cannot be merged, regardless of quality.

## Why this matters

The reference application (Picasa) is proprietary software owned by Google.
Reimplementing its *behavior* is legitimate; copying its *code* is not. A
clean-room process makes the difference explicit and auditable: behavior is
observed and specified, and implementation is written from the specification.

## The two roles

In a traditional clean room, two separate people are involved. In this project the
same contributor may hold both roles, but the **artifact boundary** is what keeps
the process clean: analysis produces specifications, and implementation consumes
only specifications.

| Role | Produces | May look at |
|------|----------|-------------|
| Analyst | Behavioral specifications in `docs/analysis/` | The reference application and its disassembly |
| Implementer | Source code in `crates/` | Behavioral specifications and public standards only |

## Rules

### Analysts

1. Findings are recorded in `docs/analysis/` as **observable behavior**:
   inputs, outputs, user-visible effects, formats, timings, edge cases.
2. Specifications describe **what** the application does and **why it is
   desirable**, never **how the original code did it**.
3. Copies of decompiled or disassembled code, original source, proprietary assets
   (icons, fonts, resource files) must **not** be committed to this repository.
   Raw tool output belongs in the ignored `analysis-scratch/` directory.
4. No file from the reference installation is redistributed. The installer and
   extracted runtime under `binaries/` are local-only and git-ignored.
5. Format specifications derived by analysis are written as neutral descriptions
   (fields, types, semantics), suitable for an independent implementation.

### Implementers

1. Do **not** consult disassembly, decompiled output, or original binaries while
   writing implementation code.
2. Implement from `docs/analysis/` specifications and from public standards
   (e.g. published file-format specifications, EXIF, ICC).
3. Never link against, bundle, or load proprietary Google components.
4. When a specification is incomplete, ask the analyst to extend it. Do not fill
   the gap by reading the binary.
5. Keep provenance: implementation commits reference the specification they
   realize where applicable.

## Provenance of files in this repository

| Path | Provenance |
|------|------------|
| `crates/**` | Original work, GPL-3.0-or-later |
| `docs/analysis/**` | Original behavioral observations and neutral specifications |
| `binaries/**` | Proprietary, local-only, git-ignored, never redistributed |
| `analysis-scratch/**` | Local scratch, git-ignored |

## Trademarks and naming

"Picasa" is a Google trademark. It is used descriptively to state what this
project reimplements. The product name is **Pinacoteca**. No Google branding,
logo or trade dress is reproduced in the application or its documentation.
