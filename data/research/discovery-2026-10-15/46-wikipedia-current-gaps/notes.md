# Current null Wikipedia-link audit

Generated: 2026-10-16 03:54 UTC

Read-only source: `/Users/earlcameron/Desktop/recompiled/data/catalog.sqlite`. The catalog contained **40** rows with `games.wikipedia_url IS NULL`. The `projects.game = games.game_key` join was one-to-one for all 40 rows. The 33 rows classified `target_type = Game` are staged in [`wikipedia-links.json`](wikipedia-links.json); the seven `target_type = Software` records below are excluded because they identify compatibility layers, clients, or multi-title software targets rather than individual game titles.

## Staged link counts

- **31 direct English Wikipedia article URLs**. Page title/slug is recorded as `wikipedia_title`; specific ports, editions, and target identity are explained in `match_note` where needed.
- **2 exact-title search URLs**. No clear dedicated English game article was found for these titles, so `wikipedia_title` is `null`, `match_type` is `exact_title_search`, and `match_note` explains the ambiguity.
- **33 total staged entries**, all checked against the exact current `game_key` and verified to have a null current `wikipedia_url`.

## Excluded software targets

- `Classic Direct3D 9 games compatible with RTX Remix (multi-title)` → `nvidia-dxvk-remix` (DXVK-Remix (NVIDIA RTX Remix Runtime))
- `Legacy Direct3D 3/5/6/7 applications and games (multi-title)` → `d7vk` (D7VK)
- `Minecraft: Java Edition (Android, through compatible launchers)` → `mobileglues` (MobileGlues)
- `Minecraft: Java Edition and other OpenGL 3.3 clients on iOS/iPadOS (through EGL-based launchers)` → `mithril-wrapper` (Mithril-Wrapper)
- `RTX Remix-compatible Direct3D 9 games and plugin integrations (multi-title)` → `dxvk-remix-plus` (Remix Plus (DXVK-Remix community fork))
- `Windows Direct3D 3/5/6/7/8/9/10/11 games (multi-title)` → `dxvk-sarek` (DXVK-Sarek)
- `Windows Direct3D games on Android (multi-title)` → `vegas-adreno-dxvk` (VEGAS)

These seven entries cover DXVK/RTX Remix compatibility, Direct3D translation layers, and Minecraft/OpenGL launcher compatibility. They are not individual game pages, so assigning a game article link would misrepresent their catalog target.

## Match notes

The manifest's direct article links are to the canonical English article titles listed in `wikipedia_title`. Most are exact game articles. The identified related-title mappings are:

- `BIT.TRIP COMPLETE (Wii)` → `Bit.Trip Complete` (article's canonical styling).
- Both `Crazy Taxi` release keys → `Crazy Taxi (video game)`, whose article covers PSN and XBLA ports.
- `Dragon Quest IX: Sentinels of the Starry Skies` → `Dragon Quest IX` (shortened canonical title).
- `Micropolis` → `SimCity (1989 video game)` because Micropolis is the GPL release/renamed version of the original game.
- `MOTHER 3` → `Mother 3`, whose article discusses the unofficial English fan translation.
- `Pokémon: Let's Go, Eevee!` → shared article `Pokémon: Let's Go, Pikachu! and Let's Go, Eevee!`.
- `Resident Evil Code: Veronica X` → parent article `Resident Evil – Code: Veronica`, with a section covering Code: Veronica X and the PS2 version.
- `Shadow of the Colossus` → its article, which includes the PlayStation 3 HD remaster.
- `The Lost World: Jurassic Park` arcade release → the dedicated arcade-game article.
- `The Simpsons Arcade Game` → `The Simpsons (video game)`, whose article documents PSN and XBLA ports.

The two fallback searches are `Go Go Ackman (Super Famicom Japan)` (the available Go! Go! Ackman article is primarily about the manga/franchise, with only brief coverage of game adaptations) and `X JAPAN Virtual Shock 001 (Japan, GS-9023 V1.000)` (no dedicated English game article was identified; the title occurs in X Japan discography material). Each search URL's query is the exact catalog `game_key`.

Live check: all 31 direct URLs opened successfully on English Wikipedia and the displayed page headings matched the recorded canonical `wikipedia_title` values. Both `Special:Search` URLs opened with the full exact `game_key` query; Go Go Ackman surfaced only the franchise article and lists, while X JAPAN Virtual Shock returned no matching result.

The importer at `scripts/catalog.py::import_wikipedia_links` accepts an object with an `entries` list and requires `game_key` plus `wikipedia_url`; it permits extra metadata fields and updates `wikipedia_title` from the corresponding field. This staged manifest does not edit the SQLite database, documentation, tracked files, or any existing manifest.
