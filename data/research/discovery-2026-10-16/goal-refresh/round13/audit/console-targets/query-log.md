# Console target research log — round 13

Checked 2026-10-16 against `/Users/earlcameron/Desktop/recompiled/data/catalog.sqlite`. Only untracked staging files were written under this directory; SQLite and tracked repository files were left untouched.

Search paths used:

- GitHub CLI repository search: `PS2Recomp`, sorted by recently updated. This surfaced Ultraman Fighting Evolution 3 and several PS2 projects; exact repository URLs were compared with SQLite and prior batches.
- GitHub CLI repository search: `PS2 static recompilation`, `game decompilation`, `console game recompilation`, plus platform-specific `PS2 matching decomp`, `PlayStation byte matching decompilation`, `Nintendo DS decompilation game`, `GameCube matching decompilation`, and `GBA matching decompilation`.
- Web search: `site:github.com "work-in-progress decompilation" "PlayStation 2" game`, `site:github.com "matching decompilation" "PlayStation 2"`, and targeted searches for PS2 recompilations and cross-console ports.
- Primary-source review: each proposed project's GitHub README (and license/build/status pages when available), GitHub repository metadata API, and GitHub Languages API.

Verified candidates staged: 4. The new repository identities are absent from SQLite. The Street Fighter III: 3rd Strike game already appears in SQLite through a Wii U port and an Xbox decompilation; the proposed record is the separate PS2 decompilation repository. Other candidates in this batch do not overlap current game/project records.

Existing target/repository leads excluded as exact SQLite identities include SwitchWakerHD, SwitchWaker, the Super Mario 64 Zune HD port, GoldenEye Vita, Sonic Advance 2 Switch, Here Comes the Pain, Devil May Cry PS2, and Shadow the Hedgehog PS2. The reviewed `03-game-recomp` and prior console-port staging manifests had no URL overlaps with the four staged candidates. Held and excluded leads, including the evidence gap for Tales of Legendia, are recorded in `exclusions.json`.
