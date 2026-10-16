# Continuation-round13 game metadata, batch B

Verified 2026-10-16 against the live catalog before staging and re-read the live `games` and `game_facts` rows at validation time. The six exact keys existed in `games`; none had acquired any facts by the final read. The profiles therefore contain 37 newly sourced facts across all six keys, including one neutral `collection_context` fact for the multi-title visual-novel scope. No facts are assigned to unsupported or ambiguous titles.

## Wikipedia decisions

- Direct article: `Persona 2: Innocent Sin` → `https://en.wikipedia.org/wiki/Persona_2:_Innocent_Sin` (article distinguishes the Japan-only PlayStation original from the 2011 PSP re-release).
- Exact-title search fallback: `Selected Japanese visual novels from the 1990s` → `https://en.wikipedia.org/wiki/Special:Search?search=Selected+Japanese+visual+novels+from+the+1990s` (a broad multi-title scope, not one game).
- Exact-title search fallback: `Streets of Rage X (OpenBOR fan game; Android release)` → `https://en.wikipedia.org/wiki/Special:Search?search=Streets+of+Rage+X` (no dedicated article for this fan game verified).
- Direct article: `WWF WrestleMania: The Arcade Game (Arcade, Wolf Unit REV 1.30)` → `https://en.wikipedia.org/wiki/WWF_WrestleMania:_The_Arcade_Game` (covers the arcade and home versions; the arcade hardware is Midway Wolf Unit, but REV 1.30 is not specified in the article).
- Exact-title search fallback: `Yoiyami Dancers` → `https://en.wikipedia.org/wiki/Special:Search?search=Yoiyami+Dancers` (no dedicated English article verified).
- Direct article: `Yoshi’s Island: Super Mario Advance 3 (USA, Game Boy Advance)` → `https://en.wikipedia.org/wiki/Yoshi%27s_Island` (the article has a separate GBA port subsection and regional release dates; this is not a claim that the USA ROM revision is separately covered).

## Source and edition notes

`Persona 2: Innocent Sin` is the PAL PSP release ULES01557: Ghostlight's official product record supplies the genre, developer, and 4 November 2011 PAL release date, and Ghostlight's release announcement supports the PAL PSP publisher attribution. The Wikipedia article is used for original PlayStation/PSP relationship and Persona-series facts.

The visual-novel key is an umbrella scope. The SuperSakura maintainer README describes an engine for selected Japanese games from the 1990s and explicitly says original copyrighted games/data are not distributed. No title-level developer, date, publisher, or platform is assigned.

`Streets of Rage X` facts are limited to the OpenBOR community resource and its creator record: Kratus is credited as the creator/author; the resource is the Windows/Android fan remake and describes its continuation relationship and content. No release date or publisher is asserted.

`WWF WrestleMania: The Arcade Game` facts are restricted to the Midway arcade edition; the article lists home versions separately and differentiates their developers and publishers. The catalog's Wolf Unit REV 1.30 revision is not claimed to be independently confirmed by the article.

`Yoiyami Dancers` is described from the official Steam product page: full displayed title, danmaku rhythm-action genre, developer and publisher credits, Windows requirements, 28 September 2022 Steam release, and Touhou fan-game status.

`Yoshi’s Island: Super Mario Advance 3` is the Game Boy Advance port, not the Super NES original. The Wikipedia article identifies Nintendo R&D2 as GBA developer and separates regional GBA release dates; Nintendo's official product listing confirms the GBA title. The key's exact USA ROM build/revision is not attributed to the article.

## Validation

Both manifests parse and satisfy current importer schema. Profile keys are unique, all facts have HTTPS sources, evidence, and `verified_on: 2026-10-16`; every date fact has a day/year granularity. Wikipedia entries use only direct English Wikipedia article URLs or exact-title `Special:Search` URLs. At final read all six targets had zero live facts, so no existing rows needed merging; the manifests nevertheless contain complete replacements for each key. Both importers passed on a fresh disposable copy of `data/catalog.sqlite`: 37 facts and six links imported, `integrity_check` returned `ok`, and `foreign_key_check` returned no errors. The canonical database hash was unchanged and the disposable copy was removed.
