# Batch A: factless game metadata continuation

Reviewed 2026-10-16 against `/Users/earlcameron/Desktop/recompiled/data/catalog.sqlite`. The six requested exact keys all existed and had zero `game_facts` rows. Each already had the direct English Wikipedia URL listed in `wikipedia-link-audit.json`; no canonical URL change is proposed. The fact manifest contains 28 facts across six profiles. No live catalog database or tracked file was modified.

## Sources and scope

Facts cite the RCGS institutional game collection; Millennium Kitchen’s official works page and dated archive; Rockstar’s official support, launch, and Google Play pages; the official Bokujō Monogatari series page and Natsume publisher announcement; the archived contemporary Activision advertisement hosted by CGW Museum; and Sony’s official PlayStation Store page. Every fact has a direct HTTPS source URL, evidence note, and `verified_on` date. Date fields use only source-supported precision.

## Wikipedia reconciliation

`wikipedia-links-reviewed.json` contains the six existing direct article URLs solely as a no-op importer-shape check. The corresponding rows already held the same URL and page title before staging, so no link was overwritten or improved. The direct mapping and identity check are in `wikipedia-link-audit.json`. Advanced Daisenryaku currently maps to the Daisenryaku series article, which contains a separately titled section and row for “Advanced Daisenryaku: Deutsch Dengeki Sakusen”; the URL is retained. The other five pages are title-specific articles, and Bully’s article explicitly covers Anniversary Edition and its mobile release.

## Holds and caveats

- HyperBlade’s Activision ad supports the PC platforms and team/publisher information and states “The Drome Opens Summer ’96.” That is preserved as an advertised window only; no actual retail date is asserted. The key’s `HYPERX.EXE v1.0.0.1` identifier was not added as a fact because the exact patch association found in this pass was only in a secondary archive.
- Mister Mosquito’s official PlayStation Store page supports the PS2 origin, game description, level count, and the 2024 PS4/PS5 conversion date. No 2001 original release day or developer credit is staged because the primary/authoritative sources found did not establish those details.
- Bully’s exact Android v1.4.311 update note is supported by Rockstar Support. Rockstar also documents an earlier 64-bit stability update (v1.4.277, March 25, 2025); this batch uses the exact v1.4.311 fix and does not claim an unverified ABI requirement for that version.
- Advanced Daisenryaku’s institutional record gives year precision (1991) and attributes that date to Mega Drive Perfect Catalog; no exact day is asserted.

## Files

- `game-facts-reviewed.json` — importer-shaped additive profiles.
- `wikipedia-links-reviewed.json` — six unchanged current links, for scratch validation only.
- `wikipedia-link-audit.json` — mapping and direct-article review notes.
- `live-state-before.json` — pre-staging exact-key counts, link values, and database hash.
- `validation.json` — disposable-copy importer results (written after validation).
