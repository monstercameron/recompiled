# M–Z game-profile hydration research log

Checked 2026-10-16 against the live `data/catalog.sqlite` snapshot in read-only mode (3,288 projects; 1,954 game keys).

Coverage found 115 M–Z game keys with no `game_facts` rows. There were no non-letter-leading zero-fact keys in the current database. This staged batch contains 33 profiles / 220 facts, 51 broad system/runtime/multi-title holds, and 31 title-specific fact holds. It also stages 76 previously missing Wikipedia URLs (34 direct/article-entry links and 42 exact-title search fallbacks).

## Research decisions

Each fact is separately linked to the direct English Wikipedia article or exact-title article entry that states it. The fact evidence records edition and regional caveats where relevant. Date granularity is day, month, or year according to the source; no dates were estimated.

Direct article matches that need special scope care include the Famicom/NES editions of *Maniac Mansion*, *Kickle Cubicle/Meikyujima*, *Ys I*, and *Ys II*; the two *Pokémon* version pairs; the PC-FX edition of *Welcome to Pia Carrot!!*; the X360/PS3 *NHL: Legacy Edition* section of the *NHL 16* article; and the FDS version of *Zanac*. Their caveats are in the fact evidence and link notes.

Exact title search fallbacks are not page matches. They preserve a useful Wikipedia search for a known exact project target while avoiding an incorrect nearby article. In particular, the search for *Macross Ultimate Frontier* is not linked to *Macross Ace Frontier*, and *Mashed: Fully Loaded (PC, 2004)* is held because the article’s edition/platform chronology conflicts with the key.

## No-fact keys held as broad target families

No title-specific game facts were added for these system, emulator, graphics/API, multi-title, or custom runtime targets:

- `Master System; Game Gear` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Metal-rendered applications and games (multi-title)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Minecraft Java Edition 26.2-snapshot-1 renderer` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Minecraft: Java Edition (Android, through compatible launchers)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Minecraft: Java Edition and other OpenGL 3.3 clients on iOS/iPadOS (through EGL-based launchers)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `MS-DOS games and IBM PC-compatible software` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `MSX home computers, including MSX2 and later variants` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Multiple Xbox One ERA games` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `NEC PC-9801 / PC-98 computers and games` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `NES` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `NES/Famicom` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `NES/Famicom/Famicom Disk System/Dendy` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Nintendo 3DS` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Nintendo 64` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Nintendo 64/Nintendo 64 Disk Drive` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Nintendo DS` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Nintendo DS/DSi` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Nintendo GameCube/Wii` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Nintendo Switch 2 software (prototype)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Nintendo Switch and Switch 2 software (prototype)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Nintendo Switch games` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Nintendo Switch games and homebrew` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Nintendo Switch homebrew and games` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `OpenGL applications/games (multi-title)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `OpenGL ES applications and games (multi-title)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Original Xbox and Sega Chihiro software` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `PlayStation 2 software` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `RTX Remix-compatible Direct3D 9 games and plugin integrations (multi-title)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Sega Mark III; Master System; Game Gear; SG-1000; Othello Multivision` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Sega Model 3 arcade games` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Sega Saturn` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Selected Japanese visual novels from the 1990s` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `SG-1000; Mark III; Master System; Game Gear; Genesis / Mega Drive; Sega CD / Mega CD; preliminary Sega Pico` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `SG-1000; SC-3000; Master System; Game Gear; Genesis / Mega Drive; Sega CD / Mega CD; 32X; Sega Pico` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `SG-1000; SC-3000; SF-7000; Mark III; Master System; Game Gear; ColecoVision; Othello Multivision` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `SG-1000; SC-3000; SF-7000; Master System; Game Gear; Genesis / Mega Drive; Sega CD; 32X; Sega Pico` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Sinclair computers and related 8-bit systems, including ZX Spectrum, ZX80/81, QL, and Z88` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `SNES/Super Famicom` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `SNES/Super Famicom; Super Game Boy support` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Vulkan applications/games targeting Apple platforms` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Wii U games and homebrew` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Wii U games, applications, and homebrew` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Windows Direct3D 12 games (multi-title)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Windows Direct3D 3/5/6/7/8/9/10/11 games (multi-title)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Windows Direct3D games (multi-title)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Windows Direct3D games on Android (multi-title)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Windows Direct3D games on macOS (multi-title)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Windows games (multi-title)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `Windows games/apps (multi-title)` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `x86 Linux applications/games on ARM64 Linux` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.
- `x86-64 Linux applications/games on non-x86 Linux` — target is a platform, emulator/runtime, API layer, or multi-title scope rather than one game.

## No-fact game keys held for source or identity review

- `Macross Ultimate Frontier` — The only close English article match found was Macross Ace Frontier, a different PSP game; kept as a title-search link and held from factual import.
- `Marchen Veil` — No exact English game article with usable facts was verified; exact-title search fallback only.
- `Mashed: Fully Loaded (PC, 2004)` — The key combines Fully Loaded with PC/2004, while the article describes Fully Loaded as a later Europe release and distinguishes it from the 2004 PC original; edition/platform mismatch held.
- `Minecraft Classic client behavior (2009-era; the project also partially supports some pre-0.30 features)` — This is a client-behavior/version scope rather than a separately identified game edition; no exact version-level article profile was verified.
- `Mushishi: Amefuru Sato` — No exact English game article with usable facts was verified; exact-title search fallback only.
- `Naruto RPG 2: Chidori vs. Rasengan / Naruto: Path of the Avenger` — The key combines Japanese and English names across reported platform/release differences; a same-title English page was not verified.
- `Nekketsu Oyako` — No dedicated English article with usable game facts was verified; exact-title search fallback only.
- `Plants vs. Zombies: Scared / PvZ Horror Edition` — No authoritative identity or game article distinguishing this title from the official Plants vs. Zombies game was verified.
- `Pocket Mirror (2016 RPG Maker VX Ace release; not Goldenertraum)` — No dedicated English Wikipedia game article with usable facts was verified for the specified 2016 release.
- `Popolocrois Monogatari II` — The closest article found covers the broader PopoloCrois property and does not identify this exact game edition.
- `Rance 4.1: Save the Medicine Plant` — No exact English article or authoritative fact source was verified for this individual title.
- `Rance 4.2: Angel Army` — No exact English article or authoritative fact source was verified for this individual title.
- `Rance II: The Rebellious Maidens` — No exact English article or authoritative fact source was verified for this individual title.
- `Rance: The Quest for Hikari` — No exact English article or authoritative fact source was verified for this individual title.
- `Reco Love: Gold Beach` — No exact English article with facts for this individual edition was verified.
- `Relapse: Resistance (2009 iPhone game)` — No dedicated English Wikipedia article or authoritative source for this 2009 iPhone title was verified.
- `Retroid` — The key did not resolve to a dedicated English article for the Game Boy title; exact-title search fallback only.
- `SD Gundam World: Gachapon Senshi 5` — No dedicated English article with version-specific facts was verified.
- `Shin Bokura no Taiyou: Gyakushuu no Sabata (Boktai 3)` — No dedicated English article with facts for the third Boktai game was verified; series-level matches do not identify this title precisely.
- `Son Son II` — Only a PC Engine release-list entry was found, without an article supporting a useful fact profile.
- `Sora no Otoshimono Forte: Dreamy Season` — No dedicated English article or authoritative fact source was verified for this licensed game.
- `Sunsoft Grand Prix` — No dedicated English article or authoritative fact source was verified for this Game Boy title.
- `Suzumiya Haruhi no Chokuretsu (The Series of Haruhi Suzumiya)` — The closest English article match is about the Haruhi franchise and does not verify this particular game.
- `Taxi 2 - The Game` — The film franchise page does not provide reliable game-specific facts for this release.
- `The Addams Family` — The project target is the TurboGrafx-CD title, but the direct game article primarily covers other Addams Family games; exact-title search fallback only.
- `TimeBall` — No exact English article with usable facts was verified for this PC Engine game.
- `Tongari Boushi to Mahou no Machi` — No exact English article with usable facts was verified for this Japanese game.
- `Touhou 10.5: Scarlet Weather Rhapsody` — A series-level result does not identify this exact installment; no dedicated English game article with usable facts was verified.
- `Undertale Green` — Fan game with no exact English Wikipedia article or authoritative fact profile verified.
- `UnNetHack 6 (based on UnNetHack 6.0.15-dev)` — The English article for NetHack describes a different game; the UnNetHack derivative/version is held.
- `Wonder Trek` — No exact English article with usable facts was verified for this Game Boy title.

## Profiled keys

- `Maniac Mansion` — 6 sourced facts.
- `Medabots: Rokusho Version` — 5 sourced facts.
- `MediEvil II (USA, SCUS-94564)` — 7 sourced facts.
- `Meikyujima / Labyrinth Island` — 5 sourced facts.
- `Need for Speed: Most Wanted (2005, PlayStation 2)` — 7 sourced facts.
- `NHL 11 Legacy Edition` — 7 sourced facts.
- `P.T. (CUSA01127; user-supplied PS4 game data)` — 7 sourced facts.
- `Pharaoh Gold with Cleopatra expansion (GOG release, patch 2.1; Pharaoh.exe 2.1.0.0, built 2000-08-28)` — 7 sourced facts.
- `Pokémon Emerald (Game Boy Advance; upstream US base ROM SHA-1 f3ae088181bf583e55daf962a92bb46f4f1d07b7)` — 7 sourced facts.
- `Pokémon Puzzle League (Nintendo 64 North American ROM; SHA-1 8173866fc8c7652abd44c48efcab85441c6806a1)` — 7 sourced facts.
- `Pokémon SoulSilver (US Nintendo DS retail ROM)` — 8 sourced facts.
- `Pokémon Sword (Nintendo Switch, build 562)` — 7 sourced facts.
- `Pu·Li·Ru·La Arcade Gears Vol. 1` — 6 sourced facts.
- `Ratchet: Deadlocked` — 7 sourced facts.
- `Sakura Wars 2` — 7 sourced facts.
- `Scramble Spirits` — 6 sourced facts.
- `Solar Striker` — 6 sourced facts.
- `Spyro the Dragon (Japan)` — 6 sourced facts.
- `Star Ocean: Till the End of Time` — 7 sourced facts.
- `Strike Gunner S.T.G.` — 6 sourced facts.
- `Super Breakout` — 7 sourced facts.
- `Super Mario 64 (Nintendo 64; README documents the U.S. ROM build)` — 7 sourced facts.
- `Syndicate` — 7 sourced facts.
- `The Simpsons: Tapped Out (Android arm64-v8a client, 4.69.x)` — 8 sourced facts.
- `Tomba! (USA PlayStation disc, SCUS-94236)` — 6 sourced facts.
- `Torchlight (Xbox 360/XBLA version 1.0.140.0)` — 7 sourced facts.
- `Welcome to Pia Carrot!!` — 7 sourced facts.
- `Wipeout XL (USA, SCUS-94351)` — 7 sourced facts.
- `Youchien Senki Madara` — 6 sourced facts.
- `Ys 2` — 6 sourced facts.
- `Ys.` — 7 sourced facts.
- `YU-NO: A Girl That Chants Love at the Bound of This World` — 7 sourced facts.
- `Zanac AI` — 7 sourced facts.

## Wikipedia-only title checks

For targets that already had factual profiles but lacked a Wikipedia URL, this batch adds only the exact direct page/entry or an exact-title search fallback. Existing live links were not replaced. Profile manifests remain separate from the Wikipedia-link manifest.

## Import validation

Both JSON manifests passed duplicate-key checks and the current `scripts/catalog.py` importer schema. All 33 profile keys were present in `games` with zero prior fact rows; all 76 Wikipedia-link keys were present with blank prior Wikipedia URLs. On a fresh disposable copy of `data/catalog.sqlite`, the profile and link importers loaded 220 facts and all 76 links; SQLite integrity and foreign-key checks passed. The live database remained unchanged (3,288 projects, 1,954 game keys, 9,196 fact rows; SHA-256 unchanged). The disposable database was removed after validation.
