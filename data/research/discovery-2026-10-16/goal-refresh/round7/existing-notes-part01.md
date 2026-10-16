# Existing game hydration, part 01

Research manifest scope: the 11 exact `game_key` values in `part-01.json`. At the source catalog snapshot (`/Users/earlcameron/Desktop/recompiled/data/catalog.sqlite`), each key existed in `games`, mapped to at least one `target_type='Game'` project, and had zero `game_facts` rows. The profile manifest contains each exact key once; all 85 facts have an HTTPS source, evidence note, and `verified_on: 2026-10-16`. Every `regional_release_date` fact carries a `day`, `month`, or `year` granularity.

## Evidence choices and limits

- **2048** uses its English article for the browser original, creator, puzzle genre, web release date, and open-source context. No publisher is claimed for the original web release.
- **Block Blast!** uses its dedicated English article for the game profile and launch year, plus Apple's official App Store listing for the developer. The available sources identify the mobile platforms and developer; they do not provide a separate publisher credit.
- **Bounce** uses the English article's “Bounce (2001)” subsection. The facts refer to the original Nokia title, not later Rovio-developed series entries.
- **Call of Duty: Roads to Victory** uses its exact English article. The release fact is the North American PSP date; the profile separately notes later Vita compatibility.
- **Command & Conquer / Red Alert / Dune 2000** is a grouped game key used by OpenRA. The manifest therefore has repeated `original_platform`, `developer`, `publisher`, and `regional_release_date` fields, each labeled with its constituent title. Those facts use the individual English game articles. The `genre` and OpenRA relationship use OpenRA's official about page. This avoids merging distinct publishers, platforms, or release dates into one unsupported value.
- **Dragon Ball 3: Gokuu Den** uses a Japanese Famicom catalog record for title, role-playing genre, system, TOSE, Bandai, and the exact Japan date. Arcade History supports the short story context. No English Wikipedia game article was found.
- **Evo's Space Adventures** is the European PlayStation port of *Space Station Silicon Valley*. The English parent article explicitly names that port and credits Runecraft; MobyGames' release record supplies the PlayStation publisher. The original N64 release remains explicit context so the PS1 edition is not confused with the original platform.
- **Fairy Tail: Madoushi** uses Nintendo's Japanese DS product page for the exact title, system, publisher/manufacturer, genre, and date. GameFAQs credits A.I. as developer; Rakuten's Japanese listing verifies product code `NTR-P-BH6J`.
- **Freedoom** has no single original hardware platform because it is game data distributed as IWAD files for Doom-compatible source ports. The manifest records the available platforms and makes that execution model explicit from the project's official manual. The English *Doom modding* page has a dedicated Freedoom section.
- **Slam Dunk: Yonkyo Gekitotsu!!** uses the Japanese game catalog for its Japanese title spelling, platform, Bandai attribution, and exact date; the matching `SHVC-UX` GameFAQs release record supplies TOSE and the basketball simulation classification. The English Wikipedia query is an exact-title fallback.
- **Guimo** uses RGB Classic Games for the DOS/platform profile and exact v2.2 shareware date. The version-specific date is December 18, 1997; no English Wikipedia article for the exact DOS version was found.

## Wikipedia link review

Direct English pages were verified for 2048, Block Blast!, Bounce, Roads to Victory, Space Station Silicon Valley (which covers the Evo PS1 port), and the Freedoom section of *Doom modding*. Exact-title search fallbacks are used for the grouped OpenRA key, Dragon Ball 3, Fairy Tail: Madoushi, Slam Dunk: Yonkyo Gekitotsu!!, and Guimo v2.2. These replace the stale Block Blast and Evo search entries with direct pages while carrying forward the already direct links for 2048, Bounce, and Roads to Victory.

The staged manifests are importer-shaped. The checks below run only against a temporary copy of the catalog; the authoritative SQLite database is not modified.
