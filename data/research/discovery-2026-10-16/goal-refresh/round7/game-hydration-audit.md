# Round 7 game-profile hydration

This batch hydrates all 188 game keys introduced in the round6 project expansion. The combined manifest contains 894 source-linked facts and one direct English Wikipedia article or precise English Wikipedia search URL per key. Profiles describe base-game facts where the edition-specific source is not definitive; they do not infer build, serial, hash, update, or region details.

All profile and Wikipedia manifests passed exact batch-key coverage, HTTPS/source, required evidence, date-verification, URL-host, and duplicate checks. The combined import was tested against a disposable copy of the 2,656-project catalog: 894 facts and 188 Wikipedia links were added, SQLite integrity passed, and no foreign-key errors were present.

Batch notes follow.

## Part 01

# Game hydration notes — continuation part 01

Verified 2026-10-16. Profiles use only facts supported by the cited English Wikipedia page or primary developer/publisher material. Edition-specific serials, ROM revisions, and build numbers are not inferred.

- The four `.hack` keys share the English series article because no standalone volume articles were found. It explicitly names all four titles and supplies PlayStation 2, action-RPG, developer, publisher, and North American release-date data. It does not verify the assigned SLUS identifiers.
- **2Xtreme**: the game article supports general PlayStation metadata and a 1996 year, but not the assigned SCUS serial.
- **Abuse**: the article supports general game metadata and regional publishing; its release history discusses both original MS-DOS release and later ports.
- **Age of Empires II: Definitive Edition (Mac edition)**: the game article supports core game metadata, and the official Age of Empires announcement verifies native macOS availability on Steam from May 28, 2026. No Mac-specific developer credit beyond the port partnership was added.
- **Alex vs Bus: The Race**: no dedicated English Wikipedia article was found, so the link uses an exact-title search fallback. Facts cite the developer’s GitHub README. The repository describes a current C port of an earlier Java game and identifies Android and PC support; no release date was stated there.
- **Aliens versus Predator Gold**: the game article covers the Gold Edition as a 2000 compilation of the base game and Millennium Expansion Pack; edition-specific serials were not found.
- **Alley Cat**: the article supports the IBM PC and PCjr ports in 1984, but gives no DOS build or disk identifier.
- **Another World / Out of This World**: the article identifies *Out of This World* as the North American title and supports the general original release metadata; no exact build identifiers are covered.
- **Ape Escape**: the article supports the PlayStation game and North American release date but does not verify SCUS-94423.
- **Bloody Roar II**: the article gives regional title names, the arcade release and 1999 PlayStation port, but does not verify any listed disc serial or the experimental Japan/Asia identifier. Its developer/publisher credits describe the arcade game.
- **Championship Manager 01/02**: the article supports the Windows and Mac release months, publisher, developer, and football-management genre. It does not identify PC version 3.9.68 or confirm the Mac edition’s PowerPC architecture.
- **Chibi-Robo!**: the game article supports GameCube metadata and broad regional release years, but not the assigned PAL/USA disc serials.

All remaining Wikipedia entries are direct English articles; the `.hack` link points to the series article rather than a volume-level page, as noted above.

---

## Part 02

# Game hydration, part 02

Verified on 2026-10-16. The two JSON manifests cover all 21 exact keys in `game-key-batches/part-02.json`, once each. Every profile has 2–5 source-linked facts; date facts include a `date_granularity`. Wikipedia links use direct English articles where available and exact-title search URLs for the two titles without a standalone English article. EA’s official Steam listings were used for Red Alert 2/Yuri’s Revenge and Tiberian Sun/Firestorm credits and expansion context; a verified SEGA trailer was used for Conduit 2 credits and Wii platform.

## Article and metadata limits

- **DHINAMEPTOR 1.00 by ニシキガツオ:** No standalone English Wikipedia article was found. Its creator-hosted page supports the title, shooting-game label, endless gameplay description, and the creator’s listed v1.00. The page does not establish a platform or release date, so neither is asserted. The Wikipedia link is an exact-title search fallback.
- **Deep 3D: Submarine Odyssey (Android release):** No standalone English Wikipedia article was found. The English Fishlabs article lists *DEEP: Submarine Odyssey* as a 2007 J2ME/Symbian game, and does not verify an Android release. The profile records only that supported metadata; it does not treat the catalog’s Android label as verified. The Wikipedia link is an exact-title search fallback.
- **Diner Dash (2004 Windows PC version):** The article identifies Windows, developer, publisher, genre, and series, but its PC release listing says 2005 while its category labels the game 2003. Since the article does not substantiate the key’s 2004 date, no release-date fact is included.
- The Red Alert 2/Yuri’s Revenge key combines a main game and expansion, so the Wikipedia link points to the Red Alert 2 article while the expansion fact cites the separate Yuri’s Revenge article. Both DOOM keys use the Doom (1993 video game) article. The idTech collection key links to the id Tech 4 article, which names the listed games and their developers.

The cited articles do not establish the key-specific GOG v1.3 packaging, compatible-IWAD set or doomgeneric target, Diablo Japan disc serial, Dark Cloud disc/review builds, Disgaea North American disc serial or update 1.40, or Dragon Age Xbox title ID. Those edition/build identifiers were not turned into facts. The common game metadata remains sourced to the relevant article.

---

## Part 03

# Part 03 game hydration notes

The staged profile manifest contains 21 exact keys, each once, with 3–5 source-linked facts. The Wikipedia manifest also contains all 21 exact keys once. Direct English Wikipedia articles are linked where available; exact-title Special:Search URLs are used for Elfen Lied Fighting, Endless Online, Exploding Lips, Galaxy on Fire 3D, and Gravity Defied.

- **Elfen Lied Fighting:** No English Wikipedia article surfaced. The source is a maintainer's preservation-port repository, which describes the original as a fan-made PC fighting game and records its original-game credit as ねくぱっち.
- **Endless Online:** No English Wikipedia article surfaced. The official site supports the MMORPG and pixel-isometric description, provides a Windows client, and documents the 0.28 and 0.3.x version lines. The staged facts avoid assigning a historical developer where the official page does not establish that for these versions.
- **Exploding Lips:** No English Wikipedia article surfaced. The official GODD Games page says first release was January 4, 2000, which conflicts with the assigned key's 1999 label and third-party catalog listings. The fact records the developer-page date and the conflict is preserved here; the page mentions the GODD engine but does not establish that “G.O.D.D.” is an alternate game title.
- **Galaxy on Fire 3D:** No English Wikipedia article surfaced. A Fishlabs retrospective documents the iOS App Store release in March 2009 and device-specific Android versions for Xperia Play (April 2011), Tegra-2 devices (June 2011), and Samsung Galaxy S II/S III (August 2012). These are device-specific release facts, not one universal Android launch date.
- **Gravity Defied:** No English Wikipedia article surfaced. The available history/handbook documents the original J2ME game, Codebrew Software, motorcycle trial-racing genre, and early-April 2005 full release.
- **Heavy Weapon Deluxe:** The direct link points to the English Wikipedia article for the base title “Heavy Weapon.” Its facts support the shared game's developer, publisher, genre, Windows availability, and 2005 year; the article does not describe Deluxe-specific packaging or build details.
- **Edition boundaries:** EVE Burst Error's facts describe the underlying game, not the Korean patch. Fallout 2 and Half-Life facts describe the cited originals; no Community Edition implementation details or ROM/build-specific claims are inferred.

---

## Part 04

# Game hydration, continuation part 04

Staged 21 profile rows with 100 source-linked facts and 21 Wikipedia link rows. The catalog was inspected read-only before staging: every assigned exact key exists, has zero `game_facts`, and currently has no `wikipedia_url`.

The facts deliberately describe the underlying named games or editions documented in the cited source. They do not claim the supplied serials, PC executables, patch versions, fan-translation builds, or Minecraft TU19/v1.6.0560.0 details.

Exceptions and scope notes:

- **Ignition:** the English article is a short stub and gives a 1996 game release; the key identifies a 1997 3dfx/Glide executable. No release-date or Glide/build-specific fact is staged.
- **Inazuma Eleven GO Strikers 2013:** no standalone English Wikipedia article was found; the franchise article lists the title, while the exact-title Special:Search fallback is staged. The product facts come from Level-5’s official Japanese product specification.
- **In Search of Dr. Riptide:** a matching English article exists, but it carries a citation-needed notice. Only the article’s explicit infobox/lead metadata is staged.
- **Marathon:** the key is title-ambiguous after the 2026 same-name game appeared. The adjacent Marathon 2 and Marathon Infinity keys indicate the original trilogy context, so the facts and direct Wikipedia link are limited to the 1994 game article; this context choice is noted for review.
- **Mafia: The City of Lost Heaven:** the English article is titled “Mafia” and covers the original 2002 Windows game. It does not substantiate the key’s version 1.2 detail.
- **Minecraft: Java Edition / Xbox 360 Edition:** Wikipedia’s Minecraft article covers both editions and supports the general metadata. No version-specific claim is made about the named Xbox 360 title update or build number.
- **Mizzurna Falls:** the article covers the Japanese PlayStation game, but not the catalog key’s serial or the Cirosan translation build.
- **Motor City Online:** the article’s infobox gives October 31, 2001 while its prose says October 29, 2001. The conflicting date is omitted.
- **Heroes III Complete** and **Marble Blast Gold:** the English articles are under the underlying game titles and include sections for the named PC/Complete editions; edition-specific facts are included only where those sections explicitly state them.

All fact source URLs use HTTPS, all fact records carry `verified_on: 2026-10-16`, and every date fact includes a supported precision (`day` or `year`).

---

## Part 05

# Part 05 game hydration notes

- All 21 assigned keys resolve to an English Wikipedia page; `New Star Soccer 5` uses the section about NSS5 in the broader `New Star Soccer` article. Profile facts cite the article that supports them, with the Steam store listing used for exact NSS5 product metadata.
- `N.O.V.A. 3`’s article gives inconsistent mobile/Android release dates in its lead and infobox; no release-date fact is recorded for the Android key.
- The source pages do not verify the catalog-specific identifiers/build details in these keys: NCAA Football 10 PSP product code `ULUS10440`; Blackmoor Manor’s particular GOG `GAME.EXE`; the Black Edition install being exactly patch 1.3; the Underground 2 x86 build; Parasite Eve’s two-disc packaging; Persona 2 disc code or Gemini translation version; PlayStation All-Stars product code `BCUS98472`; PES 2013 PC version 1.0; Project I.G.I. executable SHA-1; or a Quake II shareware-demo-specific build/distribution. Facts are limited to supported base-game/platform or edition context.
- For the Most Wanted 2005 Black Edition key, the article independently documents the Windows Black Edition and the Windows 1.3 patch/date; it does not establish that the catalog’s specific image is that exact patch installation.
- For NSS5, Wikipedia says the fifth version launched in August 2011; the Steam store listing separately displays September 27, 2012 as its Steam release date. The profile uses the Wikipedia game-version launch month and does not treat the Steam listing date as the original game launch.

---

## Part 06

# Game hydration part 06

The staged profiles cover the 21 assigned keys exactly once, with 2–5 sourced facts per key. Version labels, product codes, disc revisions, and patch labels are not treated as proof of edition-specific facts; the profiles stay at the game or engine level unless a source explicitly documents the detail.

## Article and identity notes

- `Quest (BBC Micro)` has no dedicated English Wikipedia page under the exact title. The exact-title search fallback is paired with developer Tony Oakden’s first-person 1988 article, which identifies the arcade-adventure genre and BBC Micro/Electron builds.
- `Radio Helicopter` is the European alias of the Wii game also titled `MiniCopter: Adventure Flight` / `Puchi Copter Wii: Adventure Flight`. No standalone English article was found; metadata uses Nintendo’s official Wii product page for the Japanese release and does not claim unverified regional details for the European disc.
- The `Rockman X7` key names a Korean Windows release and unofficial patch. The English Wikipedia article covers the base game, not the Korean release or patch, so only base-game facts are recorded.
- The `SCP: Containment Breach v1.3.11` article identifies 1.3.11 as the latest release version. The SOCOM article does not document the assigned `SCUS-97275` disc revision, and the SSX article does not document `ULUS-10042`; no fact claims those specific media revisions.
- `Saru! Get You! SaruSaru Daisakusen` has no dedicated English Wikipedia article. The exact-title search fallback is paired with developer h.a.n.d.’s official product listing. It supports game-level PSP, publisher, developer, and Japan release metadata; it does not establish details unique to the assigned dump revision.
- `Scrabble (Magpie)` refers to a computer Scrabble engine and its Sega Genesis/Mega Drive 68000 port, not a separately documented retail board-game adaptation. No dedicated Wikipedia article was found; the profile cites the Magpie and port repositories.
- `ShiinaRio visual novels (multi-title engine target)` is an engine target, not one standalone game. No dedicated Wikipedia article was found; facts describe the engine context using the OpenShiina repository’s project description.
- Both Sonic CD keys use the `Sonic CD` article. Its rereleases section documents the 2011 remaster, developer, engine, supported platforms, and platform-specific release dates; it does not establish provenance for the particular asset files named in the key.
- `Sonic CD Yuusei Demo (v0.02 / 1204 prototype)` has no dedicated English Wikipedia article. The exact-title search fallback accompanies facts from the Hidden Palace prototype record, which explicitly gives the build name, date, Sega CD system, and genre.

All `verified_on`/`verified_at` values are `2026-10-16`. Search fallbacks in `wikipedia-links-part06.json` are exact-title English Wikipedia searches; all other entries point directly to the identified English article.

---

## Part 07

# Game hydration part 07 notes

Checked on 2026-10-16. All 21 assigned keys have sourced facts and a Wikipedia link entry. Facts describe the underlying games or editions only where the cited article supports the claim; the title IDs, revisions, patch levels, resource formats, and other build-specific qualifiers in the catalog keys are not independently asserted.

- `Soul Sacrifice Delta` has no separate English Wikipedia page found. The profile cites the Delta section of the `Soul Sacrifice` article for its release date and the PlayStation Vita games list for Delta-specific genre, platform, developer, and publisher credits.
- `Super Mario Eclipse` has no standalone Wikipedia article. Its link points to the unofficial Mario media list, which identifies it as a Super Mario Sunshine modification. The project README is cited for its source-code and asset credits.
- The two `Star Trek: Voyager – Elite Force` keys distinguish the article's single-player mode and Holomatch multiplayer mode; both are modes of the same game.
- `Touhou 4: Lotus Land Story` is covered by the Lotus Land Story entry in `Touhou Project games`, with general genre classification from the Touhou Project article; no standalone English article was found.
- The supplied qualifiers for the Sonic Unleashed update/region, Soul Sacrifice Delta v1.30, Shadows of the Empire Rev. 2, System Shock resource compatibility, Tekken 6 language configuration, Tekken Tag Tournament 2 update v16, Tenshi no Inai 12-gatsu GB18030 resources, and Test Drive Unlimited MC 1.66A are not documented by the cited game articles. No claims about those specifics are included.
- The Terraria Xbox 360 edition is covered in the console-port subsection of the main Terraria article. The Xbox 360 regional launch date for Terraria and the GameCube/Xbox release month for The Two Towers are recorded at the granularity the cited articles provide.

---

## Part 08

# Part 08 game hydration notes

All 15 exact catalog keys are covered. Each profile contains five or fewer source-backed facts. Wikipedia URLs are direct articles where a matching page exists; exact-title search URLs are used for the combined Touhou 6–8 and Ultima VII keys and for the game without a dedicated English Wikipedia article (Tag Force 5). Konami’s official product page supplies the Tag Force 5 PSP facts.

- **Touhou 6–8:** one catalog key covers three titles. The cited game-list article supplies common Windows-era developer/publisher context and per-title platform, genre, and release dates.
- **Touhou 9:** the article’s table and prose differ on the exact August release day, so the profile records only 2005. The supplied `1.50a` version label is not asserted as a supported game build.
- **Tux Racer:** the article covers both the original open-source game and a later proprietary commercial release. Facts distinguish the initial Linux release from the Sunspire commercial release; the key does not specify which edition.
- **ULTRAKILL:** the article supports Early Access metadata; no full-release date is asserted.
- **Ultima VII:** the key combines *The Black Gate* and *Serpent Isle*. Facts identify the two articles separately and record their shared MS-DOS platform plus the sequel relationship.
- **Vampire Survivors:** facts describe the general game and documented platform/engine history, without claiming a particular executable or engine build.
- **Warcraft II:** the key combines the base game and its expansion. Dates and platform evidence distinguish *Tides of Darkness* from *Beyond the Dark Portal*.
- **Way of the Samurai 2:** the article supports the game and a Japan-only PSP rerelease; it does not verify the NTSC-U serial, MIPS R5900 target, or disc build.
- **Wing Commander:** the article describes the Windows 95 *Kilrathi Saga* re-release and its three included games. It does not verify the key’s partial DOS-data support statement or a specific runtime.
- **Yu-Gi-Oh! 5D’s Tag Force 5:** Konami’s page supplies title, PSP, genre, and the Japanese date. The European disc serial `ULES-01474` is not verified by the cited page.
- **Zombies Ate My Neighbors:** the article supports the SNES platform and broad title facts, but not the specific US cartridge or ROM build.

No edition-, serial-, CPU-, or ROM-specific claim was added where the sources did not establish it.

---

## Part 09

# Part 09 game profile and Wikipedia audit

Verified on 2026-10-16 for the exact 16 `game_key` values in `game-key-batches/part-09.json`. A read-only SQLite check confirmed that all 16 keys already exist, every one has `wikipedia_url = NULL`, and each has zero `game_facts`. The profile manifest contains only these 16 keys because the catalog importer replaces all facts for every key included in a profile manifest. No other game profiles or data were modified.

## Wikipedia coverage

Nine keys point to direct English articles that identify the same underlying game or adaptation: Another Century's Episode 3: The Final; Jet Impulse; the Jaleco Famicom adaptation of Maniac Mansion; Mr. Driller Drill Land; Pokémon Crystal Version (article title “Pokémon Crystal”); Return of Double Dragon / Super Double Dragon (article covers the Japanese Return version); Super Street Fighter II Turbo Revival (article covers the GBA Revival port); The King of Fighters: Battle de Paradise; and YU-NO. The seven remaining titles lack a dedicated English article surfaced in the search pass, so the manifest uses exact-title Wikipedia search URLs for Drag'n Wash, Mobile Golf, My Merry May, Samurai Kid, Shounen Ashibe: Nepal Daibouken no Maki, Simple 2000 Series Ultimate Vol. 17: Taisen! Bakudan Poi Poi, and Super Robot Pinball.

## Sourcing and conservative scope

For directly covered games, profile facts cite the corresponding English article. For less-documented games, primary product pages are used where available: [Nintendo's Mobile Golf page](https://www.nintendo.co.jp/n02/dmg/bgoj/index.html), [MAGES.' My Merry May with be page](https://game.mages.co.jp/mmmwithbe/), [Koei's Samurai Kid page](https://www.gamecity.ne.jp/products/products/ee/Rlsamurai_k.htm), [D3 Publisher's product catalogue](https://www.d3p.co.jp/cs/?pg=15&pl=2), [Jupiter's Super Robot Pinball page](https://www.jupiter.co.jp/e/product/game/gb/suparobo.php), and the [GatorDragonGames itch.io listing](https://gatordragongames.itch.io/dragnwash). The Shounen Ashibe entry has no standalone English article; its Famicom platform, date, and Takara credit are limited to the Japanese Famicom-catalog record with the exact title, TFC-8A product code, Takara copyright, and linked manual. No developer is proposed for this title. For My Merry May, only the 2002 year and series/first-installment description are included; no original-platform or edition-specific claim is inferred from the PS2 localization target. Drag'n Wash is matched to the official GatorDragonGames storefront entry through the shared title and developer; its source page spells its display name “DragN'Wash.”

All date facts carry `date_granularity` (`day` for exact dates, `year` when the official My Merry May press release gives only the year). Unconfirmed developer credits, regional releases, and edition-specific changes were left out. No remote media were downloaded and no media manifest was needed.

## Validation

The two JSON files are shaped for `import-game-profiles` and `import-wikipedia-links`. The profile manifest covers all 16 unique exact keys, with 2–5 facts per game and a nonempty source URL, evidence, and `verified_on: 2026-10-16` on every fact. Every date has an explicit `date_granularity`. The Wikipedia manifest covers the same 16 keys exactly once; all URLs target `en.wikipedia.org`. Nine URLs point to dedicated English articles and seven are exact-title search fallbacks. These manifests were validated locally but were not imported.

---

## Part 10

# Part 10 game hydration notes

- All 10 exact keys from `game-key-batches/part-10.json` now have one profile (five facts each) and one direct English Wikipedia article URL. The existing catalog was checked read-only on 2026-10-16: all 10 keys were present, each had zero `game_facts`, and each had a null Wikipedia URL.
- Sources are the exact English Wikipedia game articles, including the dedicated pages for *Angry Birds Journey* and *Dead Space (mobile game)*. `Ys I` is represented by the article *Ys I: Ancient Ys Vanished*, which discusses a later DotEmu Android release; it does not identify the cataloged app version.
- Profile facts stay within the game/article scope. They do not assert the cataloged Android build numbers, architecture, Unity version, IL2CPP status, the Space Channel 5 supported GDI revision, or the exact Ys app version. The Dead Space Xperia Play control description is included because the article specifically documents it.
- Platform-specific release dates are included only where the article gives a platform-specific date: LEGO Star Wars: The Complete Saga Android, Mass Effect Infiltrator Android, Modern Combat 3 Android, Real Racing 3 iOS/Android, and Space Channel 5 North American Dreamcast. These are game/platform release dates, not dates for the exact cataloged builds/revisions.
- N.O.V.A. 3 has no release-date fact because its article's lead and infobox give inconsistent Android/iOS timing. The profile uses unambiguous article facts only.
- The importer shape is `{"verified_at": "YYYY-MM-DD", "profiles": [{"game_key": exact_key, "facts": [{"field", "value", "source_url", "evidence", "verified_on", optional "date_granularity"}]}]}` and Wikipedia links use `{"entries": [{"game_key", "wikipedia_url", "wikipedia_title"}]}`. No live database import or tracked-file change was made.
