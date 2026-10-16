# Validation

- `media.json` parses and follows the `import-media` shape: **29 project groups, 29 media rows**.
- All 29 IDs exist in `data/catalog.sqlite`; they are unique, and the current database has **zero `project_media` rows for every selected ID**.
- Each project has unique media URLs; the manifest has no duplicate media URL across groups. Required fields and URL fields pass the importer’s HTTPS/schema rules.
- `project_media` uses the expected `image`/`video` kind check and unique `(project_id, url)` constraint.
- URL reachability was checked with HTTP `HEAD` only (no media body downloaded or cached): 59 unique URL-field values returned HTTP 200; the Dolphin primary blog source returned 403 to the shell client. The official blog page and its embedded Homebrew Channel showcase were independently resolved in the web reader; the linked YouTube video and preview returned HTTP 200.
- `import-media` deletes each selected project's existing media group before inserting. The all-zero pre-import check means no current group is overwritten; recheck immediately before importing if the database may change.
- Project-kind coverage: `{"API Translation": 6, "Decompilation": 4, "Emulator": 6, "Fan Localization": 4, "Recompilation": 4, "Reimplementation": 5}`.

Selected IDs: `sm64-decomp`, `oot-decomp`, `pokeemerald`, `reko-binary-decompiler`, `f-zero-snes-static-recomp`, `diddy-kong-racing-pc-recomp`, `gba-recomp-static-toolkit`, `star-fox-64-static-recomp`, `droiddeck-android-pc-games`, `moltenmtl-metal-api-over-vulkan`, `metalloid-d3d12-metal-api-translation`, `mac-gaming-uncle`, `inputfusion-gamepad-api-translation`, `crossover`, `openmw`, `daggerfall-unity`, `openloco`, `black-white-2001-openblack-engine`, `the-force-engine-dark-forces`, `xemu-original-xbox`, `pcsx2`, `melonds`, `desmume`, `dolphin`, `amiberry`, `mother3-en`, `tales-of-phantasia-fve-psp-english`, `shining-force-iii-translation`, `time-travelers-3ds-en`.
