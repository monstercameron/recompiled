# Round 13 localization search notes

Verified 2026-10-16. Search paths: GitHub repository search and direct maintainer repositories/releases; Romhack Plaza and Romhack.ing project trackers; focused Reddit posts used only as discovery/maintainer announcements. The existing SQLite catalog and Round 12 localization manifests were read-only.

## Included

Nine game-specific Fan Localization rows are in `project-additions.json`. They cover Japanese → Simplified Chinese/English patches for PS3, Dreamcast, PlayStation, PC, Game Boy, PS Vita, PSP, Super Famicom, and Casio Loopy. The Typing of the Dead II entry is deliberately partial: only Level 1 menu and attract-mode text is translated. Torneko is a revised English-script/graphics update with both Japanese and older English patch inputs. Nisekoi’s maintainer release links a pre-patched package in addition to xdelta/rePatch methods; this is stated explicitly in its row.

The language map in `programming-languages.json` is the exact GitHub REST Languages API byte count. It is not a translation-language signal. Empty API maps remain unknown.

## Held

`exclusions.json` records the Romanian Flower, Sun and Rain PS2 patch (tracker says complete, but no patch file is currently exposed), I Want a Room in Loopy Town (no primary maintainer page recovered), Air Management ’96 (secondary discovery only), and the disavowed DS English Flower, Sun and Rain project.

## Game profiles

Every included game key has quick facts in `game-profiles.json`: developer, publisher, genre, original platform(s), and release year when the cited source identifies them; explicit Unknown values are used where the reviewed source does not identify a fact. The fan patch target is recorded separately in each project row’s platform/summary. `wikipedia-links.json` uses direct articles where verified and exact-title English Wikipedia search URLs otherwise.

## Evidence snapshots

GitHub repository metadata, README, release metadata, root file listing, and Languages API payloads are saved in `../evidence-snapshots/<owner>__<repo>/`. Textual LICENSE files were saved for the custom Gundam license and MIT projects; no game assets were downloaded.
