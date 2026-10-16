# October 16, 2026 next-wave catalog review

This batch consolidates 313 source-checked additions, 11 updates to existing records, 314 programming-language evidence records, 5 URI-only media links across 2 projects, and English Wikipedia links for previously unlinked game targets.

Research was split across platform and method lanes. Each project row contains its own primary project/status sources; language rows use repository language byte maps when a public source endpoint exists and preserve explicit unknowns otherwise. Media remains remote by URI; no images were copied into the repository. Exact-title Wikipedia search pages are used only when an article could not be safely matched from the candidate title.

The HyperCard 1.2.2 static-recompilation target shares a general repository URL with the already-catalogued macrecomp toolkit; it is kept as a distinct target record because its target is independently documented. The IBM System/370 browser emulator describes a Rust implementation but publishes its JavaScript/UI and compiled WebAssembly rather than Rust source; the source-availability value says so. A proposed Pokémon Black repository remapping is deliberately held because evidence does not establish that it is the same project lineage as the catalogued record.

The batch passed the repository's actual project, update, language, media, and Wikipedia importers against a disposable copy of the current catalog. SQLite integrity and foreign-key checks passed.

## Reconciled follow-on batch

`project-additions-round2-reviewed.json` adds 298 more source-linked targets after ID/repository reconciliation against the live catalog. The batch spans all six catalog types and includes software targets, emulators, API translations, mobile ports, console-to-console ports, and localizations. Five repository URLs are intentionally reused for separately documented games or platform builds; the exact target decisions are in `reconciliation-decisions-round2.json`. `project-updates-round2-reviewed.json` corrects the existing Metal2Vulkan target, alpha status, and license evidence. The importer test grew an isolated catalog from 1,887 to 2,185 projects with SQLite integrity intact.

`game-facts-round2.json` adds 3,123 source-linked observations across 631 game profiles. `wikipedia-links-round2.json` adds 188 direct article links or clearly labeled title-search fallbacks. Some combined profile keys intentionally retain multiple edition-specific observations for fields such as platforms, publishers, and release dates; each observation carries its own source and context. Where evidence does not establish project-start dates, they remain blank.

`activity-dates-round2.json` fills 223 blank project activity dates and advances 10 older values to a later source-backed repository update date. The remaining 39 older candidates and 28 already-matching values from the 300-row audit were not applied; `activity-dates-round2-audit.json` records the hold decisions.

Forty-two project additions still lack an evidenced predominant language in their project row; language evidence is being reconciled separately rather than inferred from the target or README prose.

## Verified third batch

`project-additions-round3-reviewed.json` adds 15 more deduplicated software targets, Android/mobile ports, a console-to-console port, and a graphics-driver port. `programming-languages-round3.json` records the official GitHub language totals for all 15. `media-round3.json` adds two remote screenshot URIs for Gravity Rush 2; the site stores those source URLs without copying the images locally. `wikipedia-links-round3.json` links the four new game keys to their matching English articles, including the original Flash release of The Binding of Isaac.
