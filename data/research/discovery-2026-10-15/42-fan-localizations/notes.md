# Fan localization discovery (2026-10-15)

## Scope and method

Reviewed the catalog's existing Fan Localization records and all staged discovery JSON before selecting additions. Used ROMhack Plaza, Patch Clan, Aeon Genesis, GitHub repositories/releases, and Reddit/X/Bluesky posts as discovery leads. Verified candidates against project-maintainer pages, repositories, release notes, or first-party social posts. Checked release status, target language, supported source build, patch format, and whether a game image is included.

The catalog snapshot checked for this lane contained 1,290 projects, including 106 Fan Localization records. Project IDs, names, games, source URLs, and relevant staged rows were searched across the catalog and research manifests. None of the thirteen staged candidates already appeared in either.

## Staged additions

The manifest contains 13 verified additions. Patch Clan's project pages identify current releases and assets for Yu Yu Hakusho Gaiden, Magic Knight Rayearth 2, Taekwon-Do, Dragon Ball 3: Gokuu Den, and The Gorillaman; direct IPS asset URLs responded successfully when checked on 2026-10-15. ROMhack Plaza release entries from the same maintainer team supply detailed supported-ROM fingerprints and reported test coverage.

Other additions are Shin SD Gundam Gaiden: Knight Gundam Monogatari (BPS, with source build fingerprint and hardware playthrough), Medarot Navi (English and Latin American Spanish IPS/release patcher, explicitly still in final validation and LLM-drafted), Splatterworld (xdelta patches and public source/build repository), Tokimeki Memorial 2 (v1.0 five-disc xdelta release published 2026-10-15), and Super Robot Wars 4 (Aeon Genesis v1.00 patch ZIP; internal patch format and source-ROM checksum are not stated).

Reddit/X/Bluesky searching surfaced three distinct Xenosaga I+II DS localization efforts, each verified separately from its own project source: a complete English v1.0 release (with a later experimental Episode 4 hotfix), a complete Dutch v1.0.17 release, and Vector Translations' separate first-three-chapter BPS beta. The Vector team describes its translation as non-AI and its public account requested Japanese translators; no complete release was found. The English and Dutch projects share the same clean Japanese source image but have separate release repositories and target languages.

## Held-out leads

- Ninja Burai Densetsu, Slam Dunk, Super Robot Wars Link Battler, and Super Robot Wars 64 had Patch Clan beta-page leads, but their direct patch URLs returned 404 and the ROMhack Plaza widgets showed no downloadable files at the time checked. No addition was staged without an independently reachable patch artifact.
- Pokémon Spaceworld 1997 demo appeared in a ROMhack database as a work in progress, but the linked repository was a prototype/disassembly project rather than a verifiable translation patch source; no patch artifact was confirmed.
- Meikyujima Arcade had a database listing with BPS instructions, but no maintainer repository or reachable patch artifact was verified.
- Spike and Suzy, RPG Tsukuru DS+, and Shounen Ashibe were held out because the primary-source material did not establish a usable localization patch/source package and target build clearly enough.
- An earlier solo Xenosaga I+II DS English effort was not added: the author said they stopped after Vector Translations began its project. Ar Tonelico II and Oni Zero were already staged in lanes 12 and 28. Brightis, Nakoruru, and Snatcher metadata work was already present in lane 12, so none was duplicated here.

No unique metadata updates qualified for this lane: the recent Fan Localization updates identified during deduplication were already staged by other lanes.
