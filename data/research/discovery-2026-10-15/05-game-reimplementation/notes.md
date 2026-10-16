# Game-specific reimplementations research — 2026-10-15

This lane adds 12 game-specific projects with public maintainer repositories and verifies 23 existing Reimplementation records. Research focused on each project’s own repository, README, release information, and linked upstream source projects. Every addition names the original game-data requirement where the project does not distribute commercial assets. Status language is deliberately scoped to what maintainers document; early prototypes are labeled as such.

## Taxonomy decisions

The catalog separates new engine/game implementations from matching decompilations. A native project that uses recovered source code from a matching decompilation is classified as `Decompilation`, including downstream platform ports. Accordingly, the metadata updates reclassify OpenCMR2, Metaforce and its EctoPad integration, Ship of Harkinian / 2Ship platform ports, sm64coopNX, OpenGOAL, and SpaceCadetPinball. These projects are not clean-room reimplementations merely because their output is a native executable.

The 12 additions are engine/game reimplementations or reverse-engineered engine replacements. They are not emulators or mods. OpenFodder is retained as a source port because it implements Cannon Fodder’s engine natively; The Force Engine is an independently reconstructed Jedi-engine replacement. The OpenLara record notes platform ports with varying support instead of asserting that every documented target is equally maintained.

No `target_type` corrections were supported by this review; all reviewed entries remained game projects. Three existing language omissions were filled where repository language evidence was available (EctoPad, Aleph One, and the Zelda3 Android integration). OpenMW Android’s implementation language was left untouched because its repository is primarily a packaging/wrapper fork and the API language statistic is not a reliable description of the upstream engine.

## Exclusions and limits

OpenStunts was not added here because its README describes disassembly/decompilation-derived code, so it belongs in a decompilation review. OpenNefia’s README indicates that portions are ported from original HSP source; its source lineage should be resolved before putting it in a clean-room reimplementation lane. RustyZoo was omitted because the maintainer says it is pre-alpha and not yet playable. No project was added solely to meet a target count.

Metadata is staged for review only. The catalog database, generated docs, and build outputs were not edited.
