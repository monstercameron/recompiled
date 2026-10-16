# Dark Nebula 1/2 Recompilied

Native arm64 iOS preservation ports of the original Dark Nebula HD episodes.
**On October 15, 2026, the user confirmed that both Episode One and Episode Two
are 100% complete and requested GitHub publication.** This records user
acceptance; it does not invent automated or per-level test results.

Both apps build from **`port/DarkNebula2.xcodeproj`**:

| Scheme | Included content |
| --- | --- |
| `DarkNebula1` | 11 Original and 12 Extended levels, including terminal sequences: 23 total |
| `DarkNebula2` | 20 Original and 12 Extended levels: 32 total |

Only the original packs ship. All six authored custom courses and Episode Two's
TEST LEVEL playground were removed at the user's request. Original progression
is preserved; selections of removed packs fall back to Original.

The selected compatibility fixes include UI-only safe areas with full-screen
backgrounds/gameplay, 120 Hz presentation interpolation on supported displays,
Episode Two's shadow/audio/button-contact fixes, and Episode One's relaunch,
viewport, speed-zone and camera/entry-motion fixes. Rendering remains separate
from the original simulation, physics, animation-event and audio clocks.

## Build and run

Keep the complete workspace layout: the project references recovered/generated
sources, original-derived Data and genuine Unity runtime libraries outside
`port/`. If using a source-only publication snapshot, restore its documented
external build artifacts first. Original game and third-party engine payloads
are not relicensed by this repository.

1. Open `port/DarkNebula2.xcodeproj` in Xcode.
2. Select `DarkNebula1` or `DarkNebula2`, then an iPhone or available Simulator.
3. For your own device, select your Apple Development Team under **Signing &
   Capabilities**. Automatic signing is enabled; signing credentials are not
   included.
4. Build and Run.

For unsigned arm64 build checks from the workspace root:

```sh
xcodebuild -project port/DarkNebula2.xcodeproj \
  -scheme DarkNebula1 -configuration Release -sdk iphoneos \
  -destination 'generic/platform=iOS' -derivedDataPath build/DarkNebula1 \
  CODE_SIGNING_ALLOWED=NO build

xcodebuild -project port/DarkNebula2.xcodeproj \
  -scheme DarkNebula2 -configuration Release -sdk iphoneos \
  -destination 'generic/platform=iOS' -derivedDataPath build/DarkNebula2 \
  CODE_SIGNING_ALLOWED=NO build
```

These commands produce unsigned `.app` bundles under each DerivedData directory's
`Build/Products/Release-iphoneos/`. Use Xcode signing to run on a physical device.

## Preservation and evidence

Original IPAs and extracted source evidence remain immutable: 191 Episode One
files and 134 Episode Two files. Recheck the local preservation baseline with:

```sh
python3 tools/game_archive_preservation.py verify --game-key episode-one
python3 tools/preservation.py verify
```

The iOS packages' retained managed metadata, original ARM Mono AOT code and
serialized assets underpin the reconstruction. A companion Android IL reference
assisted Episode Two recovery; original iOS evidence remained authoritative.
The ports use the genuine Unity 4.6.2 arm64 runtime and IL2CPP toolchain, with
explicit asset-layout migration and documented compatibility changes.

- [Scope, dated changes and current user acceptance](SPECIFICATION.md)
- [Original-only packs, build and menu evidence](reports/original-packs-only.md)
- [Episode One camera and entry-motion correction](reports/episode-one-motion-smoothing-01.md)
- [Managed recovery method](reports/managed-source-reconstruction.md)
- [Original asset recovery and migration](reports/asset-migration-feasibility.md)
- [Verification ledger](analysis/verification.json)

Historical reports retain their original findings, failures and test boundaries.
Their dated “pending” or “untested” statements describe the evidence available
when written; they do not override the October 15 user acceptance. Individual
level verification remains with the user, and no unperformed instrumented test
is marked passed.
