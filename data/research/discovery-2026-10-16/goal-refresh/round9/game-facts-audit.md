# Round 9 game-profile hydration audit

Checked 2026-10-16 in read-only mode against the canonical `data/catalog.sqlite` snapshot (SHA-256 `c935fdbbee5ea2f0d20df3d8b37ab671c39c11892fcf6f59f0b744a7e1d7fb63`). Snapshot: 2,673 projects, 1,621 game keys, 8,015 `game_facts` rows; 1,520 keys have facts and 101 have none.

The final target universe combines the prior 188-project base and corrected on-disk incremental manifest (427 project rows). They contain 150 and 221 unique Game keys, with 1 overlap (RoboCop 3). The approved JSON’s root `new_game_keys` property is stale at 288 entries; counts here are derived from the actual `projects` rows (221 unique keys, 201 absent from games). The union has 370 keys: 38 already exist in games and 332 are absent. The base contributes 132 absent keys; the incremental manifest has 201 absent keys, of which 200 are outside the base. The one shared absent key yields 332 distinct missing profiles. All 38 existing keys in this union already have facts. The separately researched 36 live zero-fact gaps were rechecked and remain factless.

The replacement manifest preserves all 168 previously vetted profiles: 132 absent base keys and 36 live zero-fact gaps. It adds 92 of 200 incremental absent keys after title and fact review, for 260 profiles and 1,175 sourced facts. Coverage is 224/332 absent target keys plus 36/36 live zero-fact gaps. The 131 prior-base-only absent profiles remain included; RoboCop 3 is the single shared absent key and is counted once in the base profile set. The 18 old-base keys with live facts are excluded from replacement. They are: Bishoujo Senshi Sailor Moon, Conker's Bad Fur Day, Dead or Alive 3, Dead or Alive Xtreme Beach Volleyball, F-Zero GX, Jak and Daxter: The Precursor Legacy, Paper Mario, Persona (Shin Megami Tensei: Persona), Pokémon Emerald, Prince of Persia (1989 DOS version), Serial Experiments Lain, Silent Hill, Skate 3, Sonic Adventure 2, Sonic the Hedgehog (2006) (Xbox 360 version), Super Mario Galaxy, Super Smash Bros. (Nintendo 64, US and Japanese releases), and Touhou Koumakyou: The Embodiment of Scarlet Devil. Four reviewed full-profile additive bundles preserve all live facts and append only reviewed missing fields: F-Zero GX, Serial Experiments Lain, Silent Hill, and Sonic Adventure 2. The other 14 remain unchanged. Conflicting product scope or reworded facts are held and excluded from the importer manifests.

The optional additive manifest remains separate with 4 complete live profiles and 6 reviewed new facts. It preserves every existing fact in those profiles and adds only missing, source-backed fields.

The Wikipedia manifest contains 216 verified direct English article links. It preserves previously reviewed links, adds links for accepted page-backed profiles, skips existing live links, and contains no search URLs or links outside the reconciled target/gap set. 10 replacement profiles still lack a link entry.

Duplicate article URLs retained for distinct target scopes:
- https://en.wikipedia.org/wiki/Doom_RPG → Doom RPG (J2ME editions), Doom RPG (J2ME game data required).
- https://en.wikipedia.org/wiki/The_Legend_of_Oasis → Legend of Oasis / Story of Thor 2, The Story of Thor 2.
- https://en.wikipedia.org/wiki/Punch-Out!!_(1987_video_game) → Mike Tyson's Punch-Out!!, Punch-Out!!.
- https://en.wikipedia.org/wiki/List_of_Neon_Genesis_Evangelion_video_games → Neon Genesis Evangelion: Typing Project Advanced, Neon Genesis Evangelion: Typing Project E.

The staged lookup held 108 incremental candidates (no direct page found: 69; no usable game infobox: 1; title mismatch or unreviewed alias: 38). Examples of mismatches held include Call of Duty: Modern Warfare 2 → Call of Duty, Dragon Ball Z: The Legend → Dragon Ball Legends, Draglade 2 → Draglade, Macross Ultimate Frontier → Macross Ace Frontier, and UnNetHack 6 → NetHack. Exact-looking results without a usable game infobox (including Dangan, JULY, P.T., The Addams Family, and TimeBall) were also held. Full held list:
- Alice's Cottage II: no direct article page found by the staged Wikipedia lookup.
- Arma: Cold War Assault - Remastered (originally Operation Flashpoint: Cold War Crisis) → Cold War: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Atlach-Nacha (1997 original and 2000 re-release) → Nacha: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Biohazard 2 November 6, 1996 prototype (Resident Evil 1.5): no direct article page found by the staged Wikipedia lookup.
- Call of Duty: Modern Warfare 2 (2009 PC/Steam game data) → Call of Duty: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Captain Tsubasa Vol. II: Super Striker PLUS → Captain Tsubasa: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- DELTARUNE Chapter 5 → Deltarune: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- DS Telejan: no direct article page found by the staged Wikipedia lookup.
- DX-Ball 1.07: no direct article page found by the staged Wikipedia lookup.
- Dangan → Dang'an: title maps to a page, but no game infobox facts were extracted; page scope not verified.
- Dangan GB2: no direct article page found by the staged Wikipedia lookup.
- Darius Alpha: no direct article page found by the staged Wikipedia lookup.
- Deadeus: no direct article page found by the staged Wikipedia lookup.
- Death Note: Successors to L → Death Note: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Densetsu no Ogre Battle Gaiden: Zenobia no Ouji: no direct article page found by the staged Wikipedia lookup.
- Draglade 2 → Draglade: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Dragon Ball Z: The Legend → Dragon Ball Legends: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Dragon Slayer I → Dragon Slayer (song): returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- EverQuest Online Adventures: Frontiers → EverQuest Online Adventures: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Evolution of JiangHu (大衍江湖): no direct article page found by the staged Wikipedia lookup.
- Fallout 3 / Fallout: New Vegas engine branch → Fallout: New Vegas: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Fray Shugyou Hen: no direct article page found by the staged Wikipedia lookup.
- Fullmetal Alchemist 3: The Girl Who Succeeds God → Fullmetal Alchemist: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Fushigi no Dungeon: Fūrai no Shiren 2 — Oni Shūrai! Shiren-jō! (Nintendo 64 Japan, NUS-NSIJ-JPN revision 0): no direct article page found by the staged Wikipedia lookup.
- Ganbare Goemon 2 → Ganbare Goemon: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Ganbare! Daiku no Gen-san: no direct article page found by the staged Wikipedia lookup.
- Germs Nerawareta Machi: no direct article page found by the staged Wikipedia lookup.
- Gluk the Thunder Warrior: no direct article page found by the staged Wikipedia lookup.
- Go Go Ackman → Go! Go! Ackman: matched article but no supported facts remain after scope validation.
- Grand Theft Auto V WebAssembly build 8b0b5899ed → Theft: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Growlanser VI: Precarious World: no direct article page found by the staged Wikipedia lookup.
- Gun Frontier Arcade Gears Vol. 2: no direct article page found by the staged Wikipedia lookup.
- Gundam Assault Survive: no direct article page found by the staged Wikipedia lookup.
- Hell Fighter → The Harlem Hellfighters: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- High Seas Havoc: no direct article page found by the staged Wikipedia lookup.
- In Falsus: no direct article page found by the staged Wikipedia lookup.
- Innocent Tour → An Innocent Man Tour: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Isseki Hatchou: Kore 1ppon de 8shurui!: no direct article page found by the staged Wikipedia lookup.
- JULY → July: title maps to a page, but no game infobox facts were extracted; page scope not verified.
- Kettou Beast Wars: Beast Senshi Saikyou Ketteisen: no direct article page found by the staged Wikipedia lookup.
- Kyuutenkai: no direct article page found by the staged Wikipedia lookup.
- Luigi’s Mansion (Nintendo GameCube; USA `GLME01` rev. 0 and Europe `GLMP01`): no direct article page found by the staged Wikipedia lookup.
- Macross Ultimate Frontier → Macross Ace Frontier: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Maniac Mansion: no direct article page found by the staged Wikipedia lookup.
- Marchen Veil: no direct article page found by the staged Wikipedia lookup.
- Mashed: Fully Loaded (PC, 2004): no direct article page found by the staged Wikipedia lookup.
- Medabots: Rokusho Version → Medabots: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- MediEvil II (USA, SCUS-94564): no direct article page found by the staged Wikipedia lookup.
- Meikyujima / Labyrinth Island: no direct article page found by the staged Wikipedia lookup.
- Minecraft Classic client behavior (2009-era; the project also partially supports some pre-0.30 features): no direct article page found by the staged Wikipedia lookup.
- Minecraft Java Edition 26.2-snapshot-1 renderer: no direct article page found by the staged Wikipedia lookup.
- Mushishi: Amefuru Sato → Mushishi: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- NHL 11 Legacy Edition: no direct article page found by the staged Wikipedia lookup.
- Naruto RPG 2: Chidori vs. Rasengan / Naruto: Path of the Avenger: no direct article page found by the staged Wikipedia lookup.
- Need for Speed: Most Wanted (2005, PlayStation 2): no direct article page found by the staged Wikipedia lookup.
- Nekketsu Oyako: no direct article page found by the staged Wikipedia lookup.
- P.T. (CUSA01127; user-supplied PS4 game data) → PT: title maps to a page, but no game infobox facts were extracted; page scope not verified.
- Pharaoh Gold with Cleopatra expansion (GOG release, patch 2.1; Pharaoh.exe 2.1.0.0, built 2000-08-28): no direct article page found by the staged Wikipedia lookup.
- Plants vs. Zombies: Scared / PvZ Horror Edition: no direct article page found by the staged Wikipedia lookup.
- Pocket Mirror (2016 RPG Maker VX Ace release; not Goldenertraum) → Mirror: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Pokémon Emerald (Game Boy Advance; upstream US base ROM SHA-1 f3ae088181bf583e55daf962a92bb46f4f1d07b7): no direct article page found by the staged Wikipedia lookup.
- Pokémon Puzzle League (Nintendo 64 North American ROM; SHA-1 8173866fc8c7652abd44c48efcab85441c6806a1): no direct article page found by the staged Wikipedia lookup.
- Pokémon SoulSilver (US Nintendo DS retail ROM): no direct article page found by the staged Wikipedia lookup.
- Pokémon Sword (Nintendo Switch, build 562) → Pokémon: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Popolocrois Monogatari II → PopoloCrois: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Pu·Li·Ru·La Arcade Gears Vol. 1: no direct article page found by the staged Wikipedia lookup.
- Rance 4.1: Save the Medicine Plant: no direct article page found by the staged Wikipedia lookup.
- Rance 4.2: Angel Army: no direct article page found by the staged Wikipedia lookup.
- Rance II: The Rebellious Maidens: no direct article page found by the staged Wikipedia lookup.
- Rance: The Quest for Hikari: no direct article page found by the staged Wikipedia lookup.
- Ratchet: Deadlocked: no direct article page found by the staged Wikipedia lookup.
- Reco Love: Gold Beach: no direct article page found by the staged Wikipedia lookup.
- Relapse: Resistance (2009 iPhone game): no direct article page found by the staged Wikipedia lookup.
- Retroid → Retroid Pocket 2: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- SD Gundam World: Gachapon Senshi 5: no direct article page found by the staged Wikipedia lookup.
- Sakura Wars 2 → Sakura Wars: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Scramble Spirits: no direct article page found by the staged Wikipedia lookup.
- Selected Japanese visual novels from the 1990s → Visual novel: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Shin Bokura no Taiyou: Gyakushuu no Sabata (Boktai 3): no direct article page found by the staged Wikipedia lookup.
- Solar Striker: no direct article page found by the staged Wikipedia lookup.
- Son Son II: no direct article page found by the staged Wikipedia lookup.
- Sora no Otoshimono Forte: Dreamy Season: no direct article page found by the staged Wikipedia lookup.
- Spyro the Dragon (Japan) → Spyro: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Star Ocean: Till the End of Time: no direct article page found by the staged Wikipedia lookup.
- Strike Gunner S.T.G.: no direct article page found by the staged Wikipedia lookup.
- Sunsoft Grand Prix: no direct article page found by the staged Wikipedia lookup.
- Super Breakout: no direct article page found by the staged Wikipedia lookup.
- Super Mario 64 (Nintendo 64; README documents the U.S. ROM build): no direct article page found by the staged Wikipedia lookup.
- Suzumiya Haruhi no Chokuretsu (The Series of Haruhi Suzumiya) → Haruhi Suzumiya: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Syndicate → Syndicate: title maps to a page, but no game infobox facts were extracted; page scope not verified.
- Taxi 2 - The Game → Taxi 2: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- The Addams Family → The Addams Family: title maps to a page, but no game infobox facts were extracted; page scope not verified.
- The Simpsons: Tapped Out (Android arm64-v8a client, 4.69.x): no direct article page found by the staged Wikipedia lookup.
- TimeBall → Time ball: title maps to a page, but no game infobox facts were extracted; page scope not verified.
- Tomba! (USA PlayStation disc, SCUS-94236): no direct article page found by the staged Wikipedia lookup.
- Tongari Boushi to Mahou no Machi: no direct article page found by the staged Wikipedia lookup.
- Torchlight (Xbox 360/XBLA version 1.0.140.0): no direct article page found by the staged Wikipedia lookup.
- Touhou 10.5: Scarlet Weather Rhapsody: no direct article page found by the staged Wikipedia lookup.
- UnNetHack 6 (based on UnNetHack 6.0.15-dev) → NetHack: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Undertale Green: no direct article page found by the staged Wikipedia lookup.
- Welcome to Pia Carrot!!: no direct article page found by the staged Wikipedia lookup.
- Wipeout XL (USA, SCUS-94351): no direct article page found by the staged Wikipedia lookup.
- Wonder Trek: no direct article page found by the staged Wikipedia lookup.
- YU-NO: A Girl That Chants Love at the Bound of This World: no direct article page found by the staged Wikipedia lookup.
- Youchien Senki Madara: no direct article page found by the staged Wikipedia lookup.
- Ys 2 → Ys: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.
- Ys. → Ys: title maps to a page, but no game infobox facts were extracted; page scope not verified.
- Zanac AI → Zanac: returned Wikipedia title does not identify the requested game key; fuzzy result withheld.

All imported facts use current UI labels only: canonical_name, collection_context, developer, genre, original_platform, publisher, regional_release_date, series. Auto-extracted release dates were included only if the infobox explicitly labeled a region; otherwise they were held. The tracked profile facts and Wikipedia entries carry per-item source URLs and evidence. Previous source-vetted profile facts are preserved as-is, and unsupported custom fields remain excluded. The held custom field names for a future UI/schema change are `planned_release_year`, `edition_context`, `canonical_name_and_scope`, `company_attribution`, and `engine`; none appear in the importer manifests.

A disposable database copy accepted all 260 replacement profiles, 4 additive profiles, and 216 Wikipedia links. SQLite integrity passed, the foreign-key check returned 0 errors, and all preexisting facts and live Wikipedia URLs were preserved. The tracked importer manifests are `game-facts-missing-reviewed.json`, `game-facts-additive-reviewed.json`, and `wikipedia-links-reviewed.json` in this directory.
