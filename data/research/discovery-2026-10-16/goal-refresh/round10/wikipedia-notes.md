# Wikipedia gap review — 2026-10-16

The live SQLite query found **124** rows in `games` whose `wikipedia_url` was empty. The batch in `wikipedia-links.json` covers each exact `game_key` once and leaves every key spelling intact.

## Decisions

- **42 direct article links** point to an English Wikipedia page that identifies the exact game or a named version/expansion. Notes in the manifest and audit explain title aliases and version coverage.
- **82 exact-title search fallbacks** retain the full catalog key in Wikipedia’s search URL because no dedicated, unambiguous article was verified. The importer only requires an HTTPS `en.wikipedia.org` URL; the Rust catalog test accepts these URLs.
- **No keys are held and none lack a defensible reference under the requested fallback policy.** Rows marked `exact_title_search_fallback` are references to a search for the exact title, not claims that a matching article exists. No fuzzy page matches were added.

Alice’s Cottage II, the test’s first failing key, maps to the project **Alice’s Cottage II English Translation**, an English localization for the PC-98. No exact English Wikipedia game article was verified, so the entry searches the exact catalog title. The candidate audit records that project title and URL.

## Prior-manifest reconciliation

I recursively scanned JSON manifests under `data/research/` and `work/research-2026-10-16/` and compared exact game keys against the live blanks. The only existing URL proposal for these 124 keys is the Deep 3D exact-title search in `work/research-2026-10-16/goal-refresh/continuation-next-additions/game-hydration/wikipedia-links-part02.json`; the importer-ready batch carries that URL and title through unchanged. The round-9 `hydration-audit.json` has null Wikipedia fields for several matching targets, which are not proposed links. The new manifest has no repeated game keys.

## Ambiguities recorded

The audit marks exact-title searches for several common collision cases: Alice’s Cottage II has no verified dedicated page; `Dangan` resolves to a Chinese archival term; `Ganbare Goemon 2` resolves to a different game article; the `Growlanser VI` title resolves to the series page; `Macross Ultimate Frontier` resolves to its predecessor’s article; the Pocket Mirror title resolves to a different later game; the Sora no Otoshimono title resolves to a manga article; the unqualified Addams Family target refers to a TurboGrafx-CD game that Wikipedia distinguishes from Ocean’s separate platform game; and `Ys.` does not identify an installment safely. Each receives its exact catalog-title search URL rather than an unsupported direct match.

## Validation

The importer is hard-wired to the repository database path, so I ran the real `scripts/catalog.py` importer function with its database path redirected in memory to a temporary SQLite file. That disposable database was seeded with the 124 live keys and then imported this manifest. The import completed for all 124 rows, and a follow-up query confirmed 124 populated `en.wikipedia.org` URLs, no blank URLs, and no duplicate game keys. The canonical `data/catalog.sqlite`, `docs/catalog.json`, tracked files, and build outputs were not modified.
