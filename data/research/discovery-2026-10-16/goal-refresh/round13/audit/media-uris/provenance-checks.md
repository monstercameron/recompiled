# Round 13 media URI provenance and checks

Checked 2026-10-16 against the live SQLite catalog and the linked maintainer README pages. The manifest contains 25 URI-only media references for 25 cataloged projects; 24 are image URIs and one is a YouTube video URL. No media files were downloaded or copied.

## Checks

- Every project ID exists in `data/catalog.sqlite` and had zero current `project_media` rows when this manifest was prepared.
- Every media URL uses HTTPS, is linked from the stated maintainer README, and is unique against all current catalog media URLs and within this batch.
- `kind` follows the linked resource: GitHub-hosted image/GIF/WebP files are `image`; the YouTube watch URL is `video`.
- Image `preview_url` equals the same directly linked image URI. The video preview is omitted because the README does not directly link a thumbnail URI.
- HTTP HEAD status and content type are recorded below; HEAD requests did not retrieve media bodies.

## Records

### 1. Pokémon SoulSilver Single-Screen Static Recompilation (ssrecomp) (`pokemon-soulsilver-single-screen-static-recomp`)

- Kind and label: `image` — Pokémon SoulSilver route 29 in widescreen
- Media URI: https://raw.githubusercontent.com/ClickingMaster/ssrecomp/main/docs/screenshots/route-widescreen.png
- Maintainer source: https://github.com/ClickingMaster/ssrecomp/blob/main/README.md
- README evidence (line 34): `| ![Route 29 in widescreen](docs/screenshots/route-widescreen.png) | ![Violet City in widescreen](docs/screenshots/town-widescreen.png) |`
- HEAD check: 200 OK; Content-Type `image/png`

### 2. Power Dozer (Blast Corps Recompiled) (`blast-corps-power-dozer-pc-recomp`)

- Kind and label: `image` — Player bulldozer demolishing a structure in Simian Acres, running in Power Dozer
- Media URI: https://raw.githubusercontent.com/djultra64/Power-Dozer/main/assets/screenshots/simian-acres-gameplay.png
- Maintainer source: https://github.com/djultra64/Power-Dozer/blob/main/README.md
- README evidence (line 14): `<img src="assets/screenshots/simian-acres-gameplay.png" alt="Player bulldozer demolishing a structure in Simian Acres, running in Power Dozer" width="820">`
- HEAD check: 200 OK; Content-Type `image/png`

### 3. X-Men Legends Original Xbox Windows Recompilation (`x-men-legends-original-xbox-windows-recomp`)

- Kind and label: `image` — Wolverine in X-Men Legends level-one gameplay at 1080p
- Media URI: https://raw.githubusercontent.com/GTTeancum/OpenXML1xbox/main/docs/images/gameplay.png
- Maintainer source: https://github.com/GTTeancum/OpenXML1xbox/blob/main/README.md
- README evidence (line 19): `<td><img src="docs/images/gameplay.png" alt="Wolverine in level-one gameplay at 1080p" width="480"><br>1080p gameplay</td>`
- HEAD check: 200 OK; Content-Type `image/png`

### 4. Midnight Club: Los Angeles Complete Edition Recompilation (holdmysocks) (`midnight-club-la-holdmysocks-recomp`)

- Kind and label: `image` — Midnight Club: Los Angeles reflection fix, shown at night and at noon
- Media URI: https://raw.githubusercontent.com/holdmysocks/mcla-recomp/main/docs/images/reflection-fix-before-after.webp
- Maintainer source: https://github.com/holdmysocks/mcla-recomp/blob/main/README.md
- README evidence (line 22): `![The same spot before and after the fix, at night and at noon](docs/images/reflection-fix-before-after.webp)`
- HEAD check: 200 OK; Content-Type `image/webp`

### 5. WWE SmackDown vs. Raw 2010 Xbox 360 Native PC Recompilation (`wwe-smackdown-vs-raw-2010-xbox360-recomp`)

- Kind and label: `image` — CM Punk hits the GO2SLEEP on Randy Orton in WWE SmackDown vs. Raw 2010
- Media URI: https://raw.githubusercontent.com/nefariousjosiah/WWE-SVR2010-RECOMP/main/docs/showcase/gts.gif
- Maintainer source: https://github.com/nefariousjosiah/WWE-SVR2010-RECOMP/blob/main/README.md
- README evidence (line 33): `<img src="docs/showcase/gts.gif" width="480" alt="CM Punk hits the GO2SLEEP on Randy Orton">`
- HEAD check: 200 OK; Content-Type `application/octet-stream`

### 6. Guitar Hero III PS3 Windows Static Recompilation (`guitar-hero-iii-ps3-windows-static-recomp`)

- Kind and label: `image` — Guitar Hero III running natively on Windows: Slow Ride in the Backyard
- Media URI: https://raw.githubusercontent.com/sp00nznet/gh3/master/docs/media/hero.gif
- Maintainer source: https://github.com/sp00nznet/gh3/blob/master/README.md
- README evidence (line 6): `![Guitar Hero III running natively on Windows: Slow Ride in the Backyard](docs/media/hero.gif)`
- HEAD check: 200 OK; Content-Type `image/gif`

### 7. Mario Paint SNES Static Recompilation (`mario-paint-snes-recomp`)

- Kind and label: `image` — Mario Paint canvas in the native recompilation
- Media URI: https://raw.githubusercontent.com/sp00nznet/mariopaint/main/docs/screenshots/canvas-timed.png
- Maintainer source: https://github.com/sp00nznet/mariopaint/blob/main/README.md
- README evidence (line 53): `| ![title](docs/screenshots/title-timed.png) | ![canvas](docs/screenshots/canvas-timed.png) | ![tool](docs/screenshots/pencil-timed.png) |`
- HEAD check: 200 OK; Content-Type `image/png`

### 8. Command & Conquer: Tiberian Sun — Firestorm PC Static Recompilation (`command-conquer-tiberian-sun-firestorm-pc-static-recomp`)

- Kind and label: `image` — Tiberian Sun: Firestorm skirmish
- Media URI: https://raw.githubusercontent.com/sp00nznet/tiberiansun-recomp/main/docs/screenshots/skirmish.png
- Maintainer source: https://github.com/sp00nznet/tiberiansun-recomp/blob/main/README.md
- README evidence (line 86): `| ![Skirmish setup](docs/screenshots/skirmish-setup.png) | ![Skirmish](docs/screenshots/skirmish.png) |`
- HEAD check: 200 OK; Content-Type `image/png`

### 9. Burnout 2: Point of Impact Xbox Static Recompilation (`burnout-2-point-of-impact-xbox-static-recomp`)

- Kind and label: `image` — Burnout 2: Point of Impact first driving lesson
- Media URI: https://raw.githubusercontent.com/timftw21/burnout2-recompilation/main/screenshots/driving-lesson.jpg
- Maintainer source: https://github.com/timftw21/burnout2-recompilation/blob/main/README.md
- README evidence (line 37): `<img src="https://raw.githubusercontent.com/timftw21/burnout2-recompilation/main/screenshots/driving-lesson.jpg" alt="First driving lesson" width="49%">`
- HEAD check: 200 OK; Content-Type `image/jpeg`

### 10. Dead or Alive 3 Native Android Recompilation (`dead-or-alive-3-android-recomp`)

- Kind and label: `image` — Dead or Alive 3 combat on an RG477M while reading an ISO
- Media URI: https://raw.githubusercontent.com/Wannamingo/DOA3/main/docs/images/screenshots/gameplay.png
- Maintainer source: https://github.com/Wannamingo/DOA3/blob/main/README.md
- README evidence (line 66): `![Бой DOA3 на RG477M при чтении ISO](docs/images/screenshots/gameplay.png)`
- HEAD check: 200 OK; Content-Type `image/png`

### 11. The Binding of Isaac (Flash) Recompiled for Android (`binding-of-isaac-flash-recomp-android`)

- Kind and label: `image` — The Binding of Isaac recompilation comparing 30 fps and 60 fps in slow motion
- Media URI: https://raw.githubusercontent.com/whylurking/IsaacRecomp/main/docs/media/compare_60fps_slowmo.gif
- Maintainer source: https://github.com/whylurking/IsaacRecomp/blob/main/README.md
- README evidence (line 72): `<img src="docs/media/compare_60fps_slowmo.gif" alt="30 fps vs 60 fps (slow motion)" width="100%"><br>`
- HEAD check: 200 OK; Content-Type `image/gif`

### 12. Imabikisō PS3 Windows Static Recompilation (`imabikiso-ps3-windows-static-recomp`)

- Kind and label: `image` — Imabikisō English title screen with New Game and Continue
- Media URI: https://raw.githubusercontent.com/yasindevs/imabikisou-pc/main/docs/screenshots/01-english-menu.png
- Maintainer source: https://github.com/yasindevs/imabikisou-pc/blob/main/README.md
- README evidence (line 31): `| [![Title screen with New Game and Continue in English](docs/screenshots/01-english-menu.png)](docs/screenshots/01-english-menu.png) | [![Opening scene displaying Why me in English](docs/screenshots/02-prologue-why-me.p`
- HEAD check: 200 OK; Content-Type `image/png`

### 13. UTP: Unreal Tournament 99 for iOS/iPadOS (`utp-ut99-ios`)

- Kind and label: `image` — Unreal Tournament 99 gameplay in UTP with the Flak Cannon visible
- Media URI: https://raw.githubusercontent.com/chrissotraidis/utp/main/docs/readme/utp-gameplay-flak-cannon.png
- Maintainer source: https://github.com/chrissotraidis/utp/blob/main/README.md
- README evidence (line 18): `![Unreal Tournament 99 gameplay in UTP with the Flak Cannon visible](docs/readme/utp-gameplay-flak-cannon.png)`
- HEAD check: 200 OK; Content-Type `image/png`

### 14. Ignition PC-to-PS Vita Port (`ignition-pc-vita-native-port`)

- Kind and label: `image` — Ignition running in the PS Vita LiveArea, photographed on a real console
- Media URI: https://raw.githubusercontent.com/Zeusthenen/Ignition-Vita/main/docs/images/livearea.jpg
- Maintainer source: https://github.com/Zeusthenen/Ignition-Vita/blob/main/README.md
- README evidence (line 24): `<img src="docs/images/livearea.jpg" alt="Ignition Vita in the PS Vita LiveArea (real console photo)" width="420"><br>`
- HEAD check: 200 OK; Content-Type `image/jpeg`

### 15. GalaxyQuest — Super Mario Galaxy VR Native Port (`super-mario-galaxy-galaxyquest-vr-port`)

- Kind and label: `image` — Mario on a pirate ship in GalaxyQuest’s diorama view
- Media URI: https://raw.githubusercontent.com/bigmak94/GalaxyQuest/main/docs/images/diorama.jpg
- Maintainer source: https://github.com/bigmak94/GalaxyQuest/blob/main/README.md
- README evidence (line 26): `![Mario on a pirate ship in the diorama view](docs/images/diorama.jpg)`
- HEAD check: 200 OK; Content-Type `image/jpeg`

### 16. Fallout II Community Edition PS4 Port (`fallout-2-ce-ps4-native-port`)

- Kind and label: `image` — Fallout II Community Edition RobCo boot menu with mod selection
- Media URI: https://raw.githubusercontent.com/jaca772/fallout2-ce-ps4/main/docs/images/boot_menu.png
- Maintainer source: https://github.com/jaca772/fallout2-ce-ps4/blob/main/README.md
- README evidence (line 13): `<img src="docs/images/boot_menu.png" alt="RobCo boot menu with mod selection" width="640"><br>`
- HEAD check: 200 OK; Content-Type `image/png`

### 17. Bounce Vita (`bounce-vita-port`)

- Kind and label: `image` — Bounce Vita first-level gameplay
- Media URI: https://raw.githubusercontent.com/M-Essa11/Bounce-Vita/main/docs/screenshots/vita/level-1.png
- Maintainer source: https://github.com/M-Essa11/Bounce-Vita/blob/main/README.md
- README evidence (line 13): `![Bounce Vita first level](docs/screenshots/vita/level-1.png)`
- HEAD check: 200 OK; Content-Type `image/png`

### 18. IW4 Pocket: Modern Warfare 2 Native iOS Port (`iw4-pocket-modern-warfare-2-ios`)

- Kind and label: `image` — IW4 Pocket demo running on an iPhone 13 Pro Max
- Media URI: https://raw.githubusercontent.com/MarkusSela/IW4-Pocket/main/media/demo.gif
- Maintainer source: https://github.com/MarkusSela/IW4-Pocket/blob/main/README.md
- README evidence (line 4): `<img src="media/demo.gif" alt="IW4 Pocket demo, iPhone 13 Pro Max">`
- HEAD check: 200 OK; Content-Type `image/gif`

### 19. BattleTanx: Global Assault Recompiled (`battletanx-global-assault-n64-pc-recomp`)

- Kind and label: `video` — BattleTanx: Global Assault (1999 N64) recompiled and ported demo
- Media URI: https://www.youtube.com/watch?v=M_FX5GtOARQ
- Maintainer source: https://github.com/wootbeer/btgarecomp/blob/main/README.md
- README evidence (line 3): `https://www.youtube.com/watch?v=M_FX5GtOARQ`
- HEAD check: 200 OK; Content-Type `text/html; charset=utf-8`

### 20. SolidCraft (Fusion-style parametric CAD application) (`solidcraft-fusion-style-parametric-cad-reimplementation`)

- Kind and label: `image` — SolidCraft 0.8 demo modeling a Raspberry Pi 4 case
- Media URI: https://raw.githubusercontent.com/bherbruck/solidcraft/main/docs/screenshots/demo-0.8.gif
- Maintainer source: https://github.com/bherbruck/solidcraft/blob/main/README.md
- README evidence (line 9): `![SolidCraft 0.8: model a Raspberry Pi 4 case from an empty design: a sketch dimensioned with parameters, extrude, fillets and shell, a rim rounded in one pick, standoffs patterned on the board's mounting holes, port cut`
- HEAD check: 200 OK; Content-Type `image/gif`

### 21. Lunar Modulator (M-VAVE FM-1 firmware preview) (`fm1-lunar-modulator-preview-firmware`)

- Kind and label: `image` — Lunar Modulator browser view of the to-scale FM-1 panel with a chord held
- Media URI: https://raw.githubusercontent.com/ip2k/lunar-modulator/main/assets/screenshots/virtual-fm1.png
- Maintainer source: https://github.com/ip2k/lunar-modulator/blob/main/README.md
- README evidence (line 13): `![Lunar Modulator in the browser: the to-scale FM-1 panel with a chord held on Macro](assets/screenshots/virtual-fm1.png)`
- HEAD check: 200 OK; Content-Type `image/png`

### 22. KOWSI KWS-X1 Standalone Firmware (`kowsi-kws-x1-standalone-firmware-reimplementation`)

- Kind and label: `image` — KOWSI KWS-X1 ripple screen beside a DSO Shell oscilloscope
- Media URI: https://raw.githubusercontent.com/kralonur/kws-x1-standalone-rs/main/pictures/ripple_scope_comparison.webp
- Maintainer source: https://github.com/kralonur/kws-x1-standalone-rs/blob/main/README.md
- README evidence (line 447): `<img src="pictures/ripple_scope_comparison.webp" width="600" alt="Ripple screen next to a DSO Shell oscilloscope, showing similar downward steps in bus ripple">`
- HEAD check: 200 OK; Content-Type `image/webp`

### 23. HP-41 Calculator Emulator (`hp41-calculator-emulator`)

- Kind and label: `image` — HP-41C/CV/CX calculator interface on macOS
- Media URI: https://raw.githubusercontent.com/talent-factory/hp41-calculator-emulator/develop/docs/screenshots/hp41-gui-v3.x.png
- Maintainer source: https://github.com/talent-factory/hp41-calculator-emulator/blob/develop/README.md
- README evidence (line 8): `<img src="docs/screenshots/hp41-gui-v3.x.png" alt="HP-41C/CV/CX GUI on macOS" width="320">`
- HEAD check: 200 OK; Content-Type `image/png`

### 24. FigCraft (Figma reimplementation) (`figcraft-figma`)

- Kind and label: `image` — FigCraft dark theme editing the Ridgeline hiking app
- Media URI: https://raw.githubusercontent.com/storytold/figcraft/main/docs/images/shot-1-hero.png
- Maintainer source: https://github.com/storytold/figcraft/blob/main/README.md
- README evidence (line 38): `<img src="docs/images/shot-1-hero.png" alt="FigCraft in its dark theme editing the Ridgeline hiking app: three phone screens (Home in light mode, a trail detail page and Home in dark mode) on the canvas, the Featured tra`
- HEAD check: 200 OK; Content-Type `image/png`

### 25. HyperCard 1.2.2 Static Recompilation (`hypercard-122-static-recompilation`)

- Kind and label: `image` — Whole Earth Catalog WHOLE SYSTEMS card drawn by HyperCard 1.2.2 recompiled to C
- Media URI: https://raw.githubusercontent.com/sp00nznet/macrecomp/main/docs/img/hypercard-card.png
- Maintainer source: https://github.com/sp00nznet/macrecomp/blob/main/README.md
- README evidence (line 161): `![The Electronic Whole Earth Catalog's WHOLE SYSTEMS card, drawn by HyperCard 1.2.2 recompiled to C](docs/img/hypercard-card.png)`
- HEAD check: 200 OK; Content-Type `image/png`
