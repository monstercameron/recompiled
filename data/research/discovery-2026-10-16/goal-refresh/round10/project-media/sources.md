# Round 10 project media: provenance and quality notes

This is an additive manifest for **29 catalog projects that had no `project_media` rows** when checked against `data/catalog.sqlite` on 2026-10-16. Every entry is a URI only. No media file was downloaded, cached, or copied into the repository. All image records use their original source URL as the preview; YouTube videos use the public YouTube thumbnail URL where available. `PCSX2` uses its official WebM URL without a separate preview.

The `source_url` field points to the project-controlled README/site that links to the asset when available. For externally hosted tutorial/demo videos, the video page itself is the provenance page. Labels identify external videos and the cases where the visual is tooling or feature art rather than a gameplay screenshot.

| Catalog ID | Kind | Media source / quality note |
|---|---|---|
| `sm64-decomp` | Decompilation | Third-party setup and first-code-changes tutorial, explicitly titled for the SM64 decomp project. Useful tutorial; not project-controlled. |
| `oot-decomp` | Decompilation | First-party README-linked screenshot of `asm-differ`, a development tool bundled under the OoT decomp repository. It is tooling UI rather than game footage. |
| `pokeemerald` | Decompilation | Third-party tutorial specifically demonstrating setup and compilation of the pokeemerald decomp project. |
| `reko-binary-decompiler` | Decompilation | First-party README-linked screenshot of Reko's byte-map view loading an ARM executable; direct maintainer repository asset. |
| `f-zero-snes-static-recomp` | Recompilation | First-party README-linked BS F-Zero Deluxe Forest III race screenshot in 21:9. |
| `diddy-kong-racing-pc-recomp` | Recompilation | First-party README-embedded project screenshot hosted in GitHub user attachments. |
| `gba-recomp-static-toolkit` | Recompilation | First-party README-linked WarioWare: Twisted! example for the GBA recompilation toolkit. |
| `star-fox-64-static-recomp` | Recompilation | Third-party Star Fox 64 Recompiled playthrough; exact project demonstration, but not project-controlled. |
| `droiddeck-android-pc-games` | API Translation | First-party README-linked animated GIF demonstrating Android launch into Steam Big Picture; stored as an image so the site uses its image preview path. |
| `moltenmtl-metal-api-over-vulkan` | API Translation | First-party README-linked ray-traced cube example rendered through MoltenMTL. |
| `metalloid-d3d12-metal-api-translation` | API Translation | First-party README-linked performance screenshot, using the project's DX12 branch asset. |
| `mac-gaming-uncle` | API Translation | First-party README-linked screenshot of the project's home screen. |
| `inputfusion-gamepad-api-translation` | API Translation | First-party README-linked Steam Input to XInput mapping diagram. |
| `crossover` | API Translation | Official CodeWeavers product page embeds this CrossOver download/free-trial tutorial; project-controlled page and video. |
| `openmw` | Reimplementation | Official OpenMW media-gallery screenshot; page describes long-distance rendering. |
| `daggerfall-unity` | Reimplementation | First-party README-embedded game screenshot hosted by GitHub user attachments; README alt text is generic. |
| `openloco` | Reimplementation | First-party README-embedded gameplay screenshot. |
| `black-white-2001-openblack-engine` | Reimplementation | First-party GitHub project page embeds two screenshots; the selected direct asset is linked there. |
| `the-force-engine-dark-forces` | Reimplementation | First-party repository example preview image from the WIP level-editor sample. This is editor-example media, not a general gameplay screenshot. |
| `xemu-original-xbox` | Emulator | Official xemu homepage screenshot carousel; direct image URL. |
| `pcsx2` | Emulator | Official PCSX2 homepage splash video; direct project-hosted WebM. |
| `melonds` | Emulator | Official project-site screenshot carousel image; the carousel has no descriptive alt text, so the label stays generic. |
| `desmume` | Emulator | Third-party DeSmuME setup/configuration tutorial, clearly titled for the emulator; not project-controlled. |
| `dolphin` | Emulator | Official Dolphin progress report embeds the Homebrew Channel showcase video; the manifest stores the linked YouTube video and thumbnail. |
| `amiberry` | Emulator | First-party README-linked shader/bezel screenshot from the Amiberry repository. |
| `mother3-en` | Fan Localization | First-party translation-project preview gallery's main-text screenshot; the page identifies its preview section as text screenshots. |
| `tales-of-phantasia-fve-psp-english` | Fan Localization | First-party patch README-embedded image; README alt text is generic, so label avoids claims about its exact scene. |
| `shining-force-iii-translation` | Fan Localization | Third-party patching/emulation tutorial explicitly about the Shining Force III Translation Project. |
| `time-travelers-3ds-en` | Fan Localization | Official Fan Translators International project-page feature image. This is project art/logo rather than a gameplay screenshot. |

## Import behavior

`scripts/catalog.py import-media` replaces each project's complete media group by deleting all rows for that ID before inserting the manifest entries. These IDs were rechecked as having zero existing media rows before staging, so this manifest cannot replace an existing group in the current database. Recheck zero-row status immediately before import if another catalog operation may add media in the meantime.
