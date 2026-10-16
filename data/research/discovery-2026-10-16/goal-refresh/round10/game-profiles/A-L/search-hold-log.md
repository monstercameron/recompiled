# A-L game profile search and hold log

Reviewed 2026-10-16 against the live `/Users/earlcameron/Desktop/recompiled/data/catalog.sqlite` catalog. The A-L zero-fact set contained 59 exact keys. This work stages 24 profiles and 106 sourced facts, plus 9 directly verified Wikipedia improvements. No canonical database or tracked files were edited.

## Review method

Each staged fact has an HTTPS source and evidence note in `game-facts-reviewed.json`. Source preference was publisher/developer pages, official stores and press notices, Nintendo/Bohemia/Sega product information, MAME software-list metadata, SMS Power, the PC Engine Software Bible, RCGS collection records, and specialist product catalogs. Date fields use only supported day, month, or year precision.

## Profiles staged

- **Arma: Cold War Assault - Remastered (originally Operation Flashpoint: Cold War Crisis)** — 5 facts.
- **Atlach-Nacha (1997 original and 2000 re-release)** — 3 facts.
- **Call of Duty: Modern Warfare 2 (2009 PC/Steam game data)** — 4 facts.
- **Captain Tsubasa Vol. II: Super Striker PLUS** — 4 facts.
- **Dangan** — 4 facts.
- **Dangan GB2** — 5 facts.
- **Darius Alpha** — 4 facts.
- **Deadeus** — 4 facts.
- **Draglade 2** — 5 facts.
- **Dragon Ball Z: The Legend** — 5 facts.
- **EverQuest Online Adventures: Frontiers** — 4 facts.
- **Evolution of JiangHu (大衍江湖)** — 5 facts.
- **Fray Shugyou Hen** — 5 facts.
- **Fullmetal Alchemist 3: The Girl Who Succeeds God** — 3 facts.
- **Fushigi no Dungeon: Fūrai no Shiren 2 — Oni Shūrai! Shiren-jō! (Nintendo 64 Japan, NUS-NSIJ-JPN revision 0)** — 5 facts.
- **Ganbare Goemon 2** — 5 facts.
- **Ganbare! Daiku no Gen-san** — 4 facts.
- **Gun Frontier Arcade Gears Vol. 2** — 5 facts.
- **Gundam Assault Survive** — 5 facts.
- **High Seas Havoc** — 3 facts.
- **Hollow Knight: Voidheart Edition** — 5 facts.
- **In Falsus** — 5 facts.
- **Kyuutenkai** — 4 facts.
- **Luigi’s Mansion (Nintendo GameCube; USA `GLME01` rev. 0 and Europe `GLMP01`)** — 5 facts.

## Holds and caveats

- **Alice's Cottage II** — Multi-title fandisc/compilation. Available sources establish the localization package and constituent mini-games, but not two safe, authoritative facts for one game.
- **Biohazard 2 November 6, 1996 prototype (Resident Evil 1.5)** — Dedicated English article verified, but no authoritative archival source supporting the prototype-specific date/build facts was found in this pass.
- **DELTARUNE Chapter 5** — Chapter-specific facts were held because its release details are time-sensitive and still require stable official sourcing. The English Wikipedia Deltarune article has a dedicated Chapter 5 section and release row, so it is proposed as a direct link improvement.
- **DS Telejan** — Project verifies the arcade target; no authoritative publisher/manual/archive record with two useful facts was verified. MobyGames was found but not used as a fact source.
- **DX-Ball 1.07** — The exact v1.07 reconstruction key is not supported by the Longbow material found (that source covered DX-Ball 2); no authoritative source for this version was verified.
- **Death Note: Successors to L** — Translation team confirms the Nintendo DS patch target, but no publisher/manual/archive record with two facts was located.
- **Deep 3D: Submarine Odyssey (Android release)** — The project targets a Switch port of Android libraries/assets; the original Android edition metadata was not established by an authoritative source.
- **Densetsu no Ogre Battle Gaiden: Zenobia no Ouji** — The project establishes the Neo Geo Pocket Color translation target, but an authoritative game/package record with two facts was not verified.
- **Dragon Slayer I** — The English Game Boy games list contains this title, but no publisher/manual/archive record was verified for the Game Boy version.
- **Fallout 3 / Fallout: New Vegas engine branch** — Two-game engine-branch grouping rather than one game; no single-game profile or Wikipedia link assigned.
- **Germs Nerawareta Machi** — The translation project verifies the PlayStation title, but no authoritative publisher/manual/archive game record was found.
- **Gluk the Thunder Warrior** — The translation project verifies the NES title, but no authoritative publisher/manual/archive game record was found.
- **Grand Theft Auto V WebAssembly build 8b0b5899ed** — The key identifies a community WebAssembly build rather than a distinct game edition. Wikipedia links to GTA V itself; build facts were not profiled.
- **Growlanser VI: Precarious World** — The series article lists the installment, but no primary/archival source supporting two installment-specific facts was verified.
- **Hell Fighter** — The translation project verifies a game target, but general search is title-ambiguous and no authoritative game-specific source was found.
- **Innocent Tour** — The translation project verifies a PC-98 target, but no publisher/manual/archive record with two useful facts was found.
- **Isseki Hatchou: Kore 1ppon de 8shurui!** — The localization project identifies a Game Boy Advance title, but an authoritative game record was not verified.
- **JULY** — The Dreamcast game identity is supported by the project and the Dreamcast article, but an authoritative product record sufficient for two facts was not found.
- **Kettou Beast Wars: Beast Senshi Saikyou Ketteisen** — Secondary game databases were found, but no publisher/manual/institutional archive source was verified.
- **Legacy Direct3D 3/5/6/7 applications and games (multi-title)** — Multi-title legacy API/software group, not one game; no profile or Wikipedia game link assigned.

Additional profile caveats:

- **Atlach-Nacha:** AliceSoft dates its original inclusion to the December 1997 ALICE no Yakata 4-5-6 release and says it was later sold standalone. The key’s 2000 standalone date is not asserted.
- **Fushigi no Dungeon: Fūrai no Shiren 2:** RCGS metadata and summary disagree on the September day; only 2000 is staged.
- **Dragon Ball Z: The Legend:** RCGS describes the PlayStation/Saturn title and credits Tose/Bandai; Sega supports the Saturn date. No other regional date is inferred.
- **Gundam Assault Survive:** the official January 2010 announcement phrases March 18 as the planned release date; that status is preserved in the fact value.
- **High Seas Havoc:** MAME’s software list gives the U.S. cartridge year as 1993 and publisher as Data East; no launch day is inferred.
- **Luigi’s Mansion:** Nintendo UK’s 03/05/2002 is normalized as 2002-05-03 for the UK page only.
- **Darius Alpha:** specialist PC Engine archival reference supports date/platform/genre/format; no production-count claim is staged.

## Wikipedia decisions

- `wikipedia-links-reviewed.json` contains 9 direct article improvements and no exact-title search fallbacks. Eight replace search URLs in the 124-entry round-10 manifest already imported into the live catalog; Hollow Knight replaces an earlier exact-title search URL. The file records each prior URL so the delta can be reviewed.
- The 24 profiles were reconciled against current live Wikipedia URLs. Existing direct links were left alone for Arma: Cold War Assault, Call of Duty: Modern Warfare 2, Darius Alpha, Dragon Ball Z: The Legend, EverQuest Online Adventures: Frontiers, Shiren 2, High Seas Havoc, and Luigi’s Mansion.
- Direct improvements are: Resident Evil 1.5; Captain Tsubasa Vol. II: Super Striker; DELTARUNE Chapter 5; Draglade 2; Fray Shugyou Hen; Fullmetal Alchemist 3; Ganbare! Daiku no Gen-san; Gun Frontier Arcade Gears Vol. 2; and Hollow Knight: Voidheart Edition. Each target article has an exact title, matching platform/release/edition context, or a dedicated section identifying the exact game. No direct links already present were duplicated.
- Search fallbacks remain for titles whose English pages were absent, ambiguous, unrelated redirects, or only described a different title: Atlach-Nacha, Dangan, Dangan GB2, Deadeus, Evolution of JiangHu, Ganbare Goemon 2, Gundam Assault Survive, In Falsus, and Kyuutenkai. The Fallout engine-branch grouping and Legacy Direct3D multi-title group are not individual games.

## Full zero-fact key set

| Exact catalog key | Result |
|---|---|
| Alice's Cottage II | Held — see caveat above |
| Arma: Cold War Assault - Remastered (originally Operation Flashpoint: Cold War Crisis) | Profile staged |
| Atlach-Nacha (1997 original and 2000 re-release) | Profile staged |
| Biohazard 2 November 6, 1996 prototype (Resident Evil 1.5) | Held — see caveat above |
| Call of Duty: Modern Warfare 2 (2009 PC/Steam game data) | Profile staged |
| Captain Tsubasa Vol. II: Super Striker PLUS | Profile staged |
| DELTARUNE Chapter 5 | Held — see caveat above |
| DS Telejan | Held — see caveat above |
| DX-Ball 1.07 | Held — see caveat above |
| Dangan | Profile staged |
| Dangan GB2 | Profile staged |
| Darius Alpha | Profile staged |
| Deadeus | Profile staged |
| Death Note: Successors to L | Held — see caveat above |
| Deep 3D: Submarine Odyssey (Android release) | Held — see caveat above |
| Densetsu no Ogre Battle Gaiden: Zenobia no Ouji | Held — see caveat above |
| Draglade 2 | Profile staged |
| Dragon Ball Z: The Legend | Profile staged |
| Dragon Slayer I | Held — see caveat above |
| EverQuest Online Adventures: Frontiers | Profile staged |
| Evolution of JiangHu (大衍江湖) | Profile staged |
| Fallout 3 / Fallout: New Vegas engine branch | Held — see caveat above |
| Fray Shugyou Hen | Profile staged |
| Fullmetal Alchemist 3: The Girl Who Succeeds God | Profile staged |
| Fushigi no Dungeon: Fūrai no Shiren 2 — Oni Shūrai! Shiren-jō! (Nintendo 64 Japan, NUS-NSIJ-JPN revision 0) | Profile staged |
| Game Boy/Game Boy Color | No profile; existing Wikipedia value retained |
| Game Boy/Game Boy Color/Game Boy Advance | No profile; existing Wikipedia value retained |
| Game Boy/Game Boy Color/Super Game Boy | No profile; existing Wikipedia value retained |
| Games and software for Commodore 8-bit computers | No profile; existing Wikipedia value retained |
| Games and software for emulated IBM PC compatibles | No profile; existing Wikipedia value retained |
| Games for Nintendo handheld systems | No profile; existing Wikipedia value retained |
| Games for supported systems | No profile; existing Wikipedia value retained |
| Ganbare Goemon 2 | Profile staged |
| Ganbare! Daiku no Gen-san | Profile staged |
| General-purpose computer systems, including x86 PCs for DOS and Windows games | No profile; existing Wikipedia value retained |
| General-purpose x86 PCs for DOS, Windows, and other guest software | No profile; existing Wikipedia value retained |
| Genesis / Mega Drive; Sega CD / Mega CD; 32X including CD 32X; Game Gear; SC-3000 / SG-1000; Sega Pico; partial Master System backwards compatibility | No profile; existing Wikipedia value retained |
| Genesis / Mega Drive; Sega CD / Mega CD; 32X; SG-1000; Master System / Mark III; Game Gear | No profile; existing Wikipedia value retained |
| Germs Nerawareta Machi | Held — see caveat above |
| Gluk the Thunder Warrior | Held — see caveat above |
| Go Go Ackman | No profile; existing Wikipedia value retained |
| Grand Theft Auto V WebAssembly build 8b0b5899ed | Held — see caveat above |
| Growlanser VI: Precarious World | Held — see caveat above |
| Gun Frontier Arcade Gears Vol. 2 | Profile staged |
| Gundam Assault Survive | Profile staged |
| Hell Fighter | Held — see caveat above |
| High Seas Havoc | Profile staged |
| Hollow Knight: Voidheart Edition | Profile staged |
| IBM PC, XT, PCjr, and Tandy 1000 software and games | No profile; existing Wikipedia value retained |
| In Falsus | Profile staged |
| Innocent Tour | Held — see caveat above |
| Isseki Hatchou: Kore 1ppon de 8shurui! | Held — see caveat above |
| JULY | Held — see caveat above |
| Kettou Beast Wars: Beast Senshi Saikyou Ketteisen | Held — see caveat above |
| Kyuutenkai | Profile staged |
| Laserdisc arcade games and Singe interactive-video games | No profile; existing Wikipedia value retained |
| Legacy Direct3D 3/5/6/7 applications and games (multi-title) | Held — see caveat above |
| Legacy Direct3D 8 games (multi-title) | No profile; existing Wikipedia value retained |
| Luigi’s Mansion (Nintendo GameCube; USA `GLME01` rev. 0 and Europe `GLMP01`) | Profile staged |
