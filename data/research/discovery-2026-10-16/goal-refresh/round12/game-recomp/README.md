# Round 12 — game recompilation research

Reviewed 2026-10-16. This lane stages importer-ready project records only; it does not edit the live database, tracked catalog files, or local media. Project/game screenshots and videos are represented by source-linked URLs only.

## Staged manifests

- `new-projects.json`: 6 distinct game projects, each qualified using a maintainer README, exact project/release evidence, and source-availability notes.
- `programming-languages.json`: exact GitHub Languages API byte maps for all 6 additions.
- `wikipedia-links.json`: 5 direct Wikipedia entries (the Darkness entry is intentionally omitted because both its catalog game keys already have article links).
- `game-profiles.json`: 24 sourced game facts across 5 new game keys; none of these five keys currently has profile facts in SQLite.
- `media-updates.json`: 4 source-linked remote media URLs for 3 existing catalog projects. `new-projects.json` also supplies 3 project-specific SSX 3 screenshot URLs. No media was downloaded.
- `project-updates.json`: 3 evidence-backed patches to existing projects.
- `dedupe-audit.json`: additions, existing-record review, and holds.

Import the project manifest first; the other specialized importers can then resolve its project/game keys. This folder is ignored staging work. Do not add its contents to the production import chain without review.

## Additions

| Project | Scope | Evidence-supported state |
|---|---|---|
| [koenhendriks/SSX3-recomp](https://github.com/koenhendriks/SSX3-recomp) | SSX 3, Xbox PAL → Windows x64 | No release is listed. Maintainer reports whole-game playability, all peaks/events, Conquer the Mountain, 2-player split-screen, 60 FPS, a two-hour soak and 48/48 scripted boots reaching a race. The record attributes these claims to the README rather than presenting them as independent results. |
| [alexbeavs-ps1-ports/rollcage-stage-ii-recomp](https://github.com/alexbeavs-ps1-ports/rollcage-stage-ii-recomp) | Rollcage Stage II, USA PS1 SLUS-00867 → Windows; Linux/macOS packages | v0.1.0 release candidate. Exact Windows package has maintainer-reported clean-extract setup and headless start; a private build was reported playable in an earlier operator tally. Native Linux/macOS setup/gameplay remain unverified. Bring-your-own-disc and BIOS. |
| [Capridog/SpaceRanger](https://github.com/Capridog/SpaceRanger) | Toy Story 2, USA N64 → Windows | v0.1.0 pre-release. Maintainer reports start-to-finish playability and discloses effects/audio issues and testing mainly on one PC. |
| [SomeoneIsWorking/psx-toy-story-2](https://github.com/SomeoneIsWorking/psx-toy-story-2) | Toy Story 2, USA PS1 SLUS_008.93 → Linux x86-64 | Separate PS1 build from SpaceRanger’s N64 static recomp. Maintainer reports the title, menus, level select, LEVEL01 and Andy’s Room running through the Lightrec runtime executor; native scene producers/front-end widescreen are incomplete. Asset-free test gates do not establish gameplay compatibility. |
| [TefMeister/the-darkness-vr](https://github.com/TefMeister/the-darkness-vr) | The Darkness, Xbox 360 → PC; stereo/VR development | Independent project from existing community Windows recompilation rows. README says the project’s own PC recomp runs in real-time 3D and one test presented the same moment to both stereo eyes; headset integration is future work. Host OS is unspecified, and this is not a supported product. |
| [xzhuofan/CryRecomp](https://github.com/xzhuofan/CryRecomp) | Tears of the Kingdom, Switch AArch64 recompilation research | Explicitly non-playable research. README reports parsed/linked modules, a measured relocation batch, and recompiled code reaching `nnMain`; native code generation and application runtime are planned. |

## Existing-row audit

The seven repositories supplied in the initial parent lead set (GoldenEye Metal, Black, Conker, Wave Race 64, JSRF, DKC3, and Road Trip Adventure) already have exact repository rows. No duplicate additions were made. Their current source-backed project states were checked; the only status-text refinement was the DKC3 bring-up summary. Media was added for GoldenEye Metal, Conker, and Wave Race 64 because their current READMEs link specific maintainer-provided media not attached to those records.

The subsequent six status leads were also checked. Burnout 3 Switch now documents native Linux x86-64 setup in addition to Switch and is patched with an explicit Linux-playtesting caveat. Geometry Wars 2 has a v0.3.1 Windows release dated 2026-10-16; its native Linux/Steam Deck build remains experimental, with the README reporting one real-hardware Steam Machine test. Third Age, SSBU, Mario Kart 8 Deluxe, and Utopia had no further verified change that warranted a content patch beyond the current catalog record; they remain listed in the audit file.

## Deduplication and holds

Exact GitHub project URLs and existing IDs were checked against the 3,411-row SQLite catalog. Game/repo checks also covered prior round-12 discovery material, including the social-latest evidence file. The earlier Toy Story 2 PS1 lead was verified as absent from SQLite and staged as its own project; it is not a duplicate of the N64 SpaceRanger record. Both target the same game but different original consoles and separate implementations.

- `MattSG/mm3-recomp`: held from a second record because the README describes merging with and extending the already-cataloged Lakshya PC-native project lineage; no evidence found for an independently scoped same-title deliverable.
- `YYOzcan/Tobiichi-Port`: held as a duplicate/downstream God of War implementation because its README credits and depends on the already-cataloged KIexster recomplication work.
- `Alexbeav/psxrecomp-ports`, `recomp-ui`, trackers, and catalog/awesome-list repos: not game-specific deliverables; the Rollcage title is staged from its own project repository.
- Existing exact repo matches from the search sweep (including Kula World, Saru Get You, JSRF Windows, and Super Dodge Ball) were not re-added.

## Source and claims policy

Repository README/release pages are the ownership and status evidence. Wikipedia is used only for game profile facts and direct links. GitHub Languages API byte maps are exact repository-level counts and can reflect generators/tooling. Generated game code, ROMs, discs, BIOS images and artwork are not copied into this work folder. Status statements about gameplay are explicitly identified as maintainer reports.
