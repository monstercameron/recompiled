# Research notes

## Accepted records

`Fallout: New Vegas` is the Windows-PC 2010 game already represented in SQLite. This new project is distinct from the catalogued `dx9mt` Direct3D 9 translator, `nv-rs`/NVForever engine reimplementations, and the Xbox 360 reNEVADA recompilation: it lifts the full 32-bit Windows PC executable to C for native Apple Silicon. The README reports one recorded hour of scripted gameplay and 99.3% exact matches across its testable leaf-function set. The exact source executable revision and any packaged release are unknown; the repository has no GitHub release. Reported gaps include silent DirectSound, Bink video, controller support, fixed-function rendering, and NVSE mods. Its MIT license covers the project tools, not the game or locally generated code.

`Casualties: Unknown` is a newly keyed PC target. The repo offers two paths and they must remain distinct in the description: the native route runs the original Unity-managed CIL and game data on a Mac ARM64 Unity player after game-specific shader conversion to Metal; the compatibility route runs the unmodified Windows executable through Wine, Rosetta 2, and D3DMetal. The verified native port is the 7.0 Steam demo (65/65 shaders); the Steam full-game listing gives a future planned 2026-11-09 date as of this research date. The full release is not claimed as ported. A maintainer-written README describes the game and build, while the official Steam listing supplies developer, publisher, store genres, Windows requirements, and planned release date. The English Wikipedia article currently has a source-quality warning, so it is linked for article discovery but not used as the sole authority for game facts.

`The Legend of Zelda: Twilight Princess HD` is a new game key distinct from the original `The Legend of Zelda: Twilight Princess`. The target is the Wii U USA disc title 00050000-1019E500, version 81 / Rev 2. The repo translates the PowerPC executable to C and renders through its native Metal layer; it does not emulate the Wii U at runtime. Status is work in progress: the title sequence, new-file opening, and walking in Ordon Village are reported; all 78 disc stages load and run when warped to; a complete playthrough and controller test are not reported. The README says no builds are published. Project code is MPL-2.0, but game files and keys are excluded.

## Canonical game facts

- Fallout: New Vegas already has developer, publisher, genre, Windows PC platform, North American release date, series, and a direct English Wikipedia link in SQLite. No duplicate facts or Wikipedia row are proposed.
- Casualties: Unknown has five facts using Steam and maintainer primary sources. Its full-game date is explicitly labeled planned, not actual. The demo release date is separate evidence and is not substituted for the full-game release.
- Twilight Princess HD has five facts using the Wikipedia article's HD-specific section plus Nintendo's official product page. The direct English Wikipedia link leads to the article whose HD-specific section covers the edition; a separate English article was not found.

## Dedupe and holds

Eight exact Mac-port/recomp repositories are already catalogued: FINAL FANTASY XI, Project I.G.I., Kula World, Jet Set Radio Future, Need for Speed Underground 2, Need for Speed: Most Wanted (PC), Populous: The Beginning, and Def Jam: Fight for NY. Their catalog IDs are recorded in `exclusions.json`.

`craigshaw/FZeroRecomp` targets the same SNES F-Zero desktop recompilation already covered by `f-zero-snes-static-recomp`; it is not added as a second record in this pass. `notmgn/mtld3d` is held because the catalog already has the released `athei/mtld3d` implementation for the same D3D9/Wine/Metal functional target, while the candidate's independence is unclear and its advertised release does not appear in the releases API. `Wilson-Flores/PacmanRecomp` and `leoxcho/nicktoons-racing-recomp` do not supply enough maintainer detail about supported input versions, verified operation, or releases to be strong additions.

No operating-system or Apple Silicon support was inferred from a generic desktop build. Where the maintainer did not identify an executable version or actual release, that absence is stated directly.
