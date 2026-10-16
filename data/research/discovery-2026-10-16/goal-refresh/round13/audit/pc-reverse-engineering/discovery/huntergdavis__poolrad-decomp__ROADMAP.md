# Pool of Radiance DOS source reconstruction: roadmap for review

**Status:** in progress, 2026-10-12. **Primary target:** the supplied DOS v1.3
installation. **End state:** maintainable modern C source that runs the complete
DOS game from owner-supplied data on supported host backends. A future Nintendo
DS port is an architectural constraint, not work scheduled in this repository.

**Checkpoint:** bounded DAX/RLE, GEO, ECL operand/text, DOS EGA, WALLDEF,
and DOS character record/name readers are merged (PRs #4, #7, #10, #13, #17,
#19, #23). The corrected overlay map, candidate call graph, Ghidra pilot, ECL
contracts, recovered C helpers, and a resident called-target inventory are
merged (PRs #6, #11, #15, #20, #25, #27). The resident audit finds 86 distinct
non-SYSTEM called targets, 84 mapped to exact prologues under empirical shifts;
the game versus library split remains open.
The read-only DOS world-save reader identifies the fixed ten-block frame.
The C-native slot-A adapter copies all ten world-save transfer spans. A bounded
host model loads the six matching character records in saved order and applies
the verified unit `0x9B:+0x01AF` record updates (PRs #193/#194). The adapter still
stops explicitly at the untranslated key-dispatch route. Sidecars, map blocks,
other post-load effects, and executing movement remain open.
Sixty-two game-specific routines are fully translated and behavior-checked
across the overlay and resident lanes (PRs #63/#69 added twenty bounded unit
`0x38` routines, PR #85 two more walk entries, PR #65 a view setter, PR #108 two ECL compare helpers, PRs #124/#126/#128 unit `0x3BD` cell, nibble and field lookups, PRs #130/#132's unit `0x7F` divide-by-five helpers, PR #136 unit `0x25` entry 19 party movement, PR #138 unit `0x68:+0x068D` Up preflight, PR #145 unit `0x68:+0x05AB` party-record scan, PR #149 unit `0x68:+0x05EE` party-list search, PR #152 unit `0x68:+0x0648` record clear, PR #150 four unit `0x3C6` character-slot helpers, PRs #158/#159/#162's unit `0x68` party-byte, map-update and cell-field-clear helpers, PR #194 unit `0x9B:+0x01AF` record updates, and PRs #70/#79 resident target reset/selection).
The PR #65 script-state initializer and PR #72 block loader remain
path-complete models. The separately translated `0x464`
bitmap release helper in PR #64 has unresolved game/library ownership and is
outside that count. Three earlier handler/callee models have unresolved dependencies
or heap semantics, two string routines have
path-complete C translations with a provisional runtime-string cap, and five have
extracted C cores with unresolved engine side effects. The DOS oracle reaches
six-person exploration from a local save; a state-gated one-step move passes.
The reviewed unit `0x9E` entry-8 dice helper uses the byte-verified SYSTEM
seed recurrence with caller-supplied state. The checkpoint is **62 full-C game
routines** (53 overlay, nine resident).
The [native saved one-Up RANDOM watch](docs/re/SLOT_A_ONE_UP_NATIVE_RANDOM_SEED.md)
now measures the authentic first script RNG seed `3140872353 → 3238563622`,
range 14, result 10, and window-2 destination word `16 → 10`.
The merged [native exactly-one-Up write watch](docs/re/SLOT_A_ONE_UP_NATIVE_WRITE_WATCH.md) measured window-0 `+0x018E: 0→1`
and `+0x0190: 5→5`, with numeric screen state `15,4 N 10:50 → 14,4 N 10:51`.
The first observed value change was unit `0x7D` entry 1 `+0x045D`, reached
through unit `0x68:+0x07ED` ([issue #3](https://github.com/huntergdavis/poolrad-decomp/issues/3));
same-value stores are not excluded. C-native save/load/one-Up parity remains open under [issue #3](https://github.com/huntergdavis/poolrad-decomp/issues/3#issuecomment-6117868640). No native playable route exists.
The [native resident-dispatch replay](docs/re/SLOT_A_ONE_UP_NATIVE_RESIDENT_DISPATCH.md)
now measures the reached `RES_08AE:00DC` call with mode 1, active word 100,
and argument 10. It records the selected `+1742(9)` branch and the active
tail through `+006C(10)`, `+17AE(0)`, `+1742(0)`, and `+13B1`, returning to
the coordinate writer. The callees' complete effects and C-native parity
remain open. The active next steps include
[measuring the game-function denominator](https://github.com/huntergdavis/poolrad-decomp/issues/24)
using the merged [overlay audit](https://github.com/huntergdavis/poolrad-decomp/pull/32)
and [resident callable audit](https://github.com/huntergdavis/poolrad-decomp/pull/33).
The regenerated overlay classifier records 114 verified identities, including
the twenty-two reviewed unit `0x38` entries, PR #65's view setter, unit
`0x3BD` entries 6, 7, 4, 2 and 3 from PRs #106/#114/#124/#126/#128, PRs #130/#132's unit `0x7F` entries 10 and 11, PR #108's two compare helpers, PR #136's unit `0x25` entry 19, PR #138's unit `0x68:+0x068D` preflight, PR #145's unit `0x68:+0x05AB` scan, and PR #149's unit `0x68:+0x05EE` list search, PR #152's `0x68:+0x0648` record clear, PR #150's unit `0x3C6` entries 12–15, PRs #158/#159's unit `0x68:+0x0236` party test and `+0x0109` map updater, PR #162's `+0x0000` field clear, and PR #194's unit `0x9B:+0x01AF` record update. With
ten verified resident identities, **124** is the reviewed evidence-backed lower bound, not the full
denominator. The [audited candidate-set range](docs/re/FUNCTION_DENOMINATOR_RANGE.md)
is 124–818, or 124–809 if nine likely linked CRT routines are excluded;
neither ceiling is a whole-game upper bound or a completion denominator. Work continues
alongside [DOS gameplay replay](https://github.com/huntergdavis/poolrad-decomp/issues/3)
and further overlay and resident translations.

The target is a faithful, source-level reconstruction of observable game
behavior. The original Turbo Pascal source cannot be recovered verbatim from
machine code. A build that merely emulates `START.EXE` and `GAME.OVR`, or a
playable remake with invented rules and encounters, would not meet this goal.
The floppy-sized installation is a tractable target for systematic decompilation.
The important distinction for planning is between recovering control flow and
producing a C program that matches game behavior. We will measure the latter
from a representative sample instead of assuming a calendar duration from disk
size.

## Baseline and reusable work

- Local input: `START.EXE` is 65,231 bytes, `GAME.OVR` is 206,105 bytes and
  begins with `FBOV`. A read-only local check validated all 1,245 blocks in
  113 `.DAX` archives, including 29 distinct GEO records of 1,026 bytes each.
  The binary's strings include `version 1.3`. The
  [isolated audit](docs/FABLE_AUDIT.md) records SHA-256 hashes and independently
  matches the public kit's structural fingerprints. The kit publishes no
  hashes, so its addresses remain hypotheses until checked against this build.
- The audit finds **34 overlay units**, **404 exported entry slots**, and
  **673 Pascal-style prologue hits in unit code** (675 in the whole file).
  These counts overlap and include unverified candidates; they cannot be
  summed into a function total. Unit payloads account for 205,893 of
  206,105 overlay bytes, including 8,442 bytes of fixups. This gives us a
  concrete, automatable inventory problem. The
  [function-count evidence](docs/re/OVERLAY_FUNCTION_INVENTORY.md) identifies
  what remains before we can use an exact game-function denominator.
- The actual ECL dispatcher handles **61 opcodes**, while the public kit
  documents about 40 and misnames several. Its playable browser build does not
  execute ECL scripts. The audit also corrects its GEO event-plane, ECL header,
  wallset, and image-header interpretations. Use the kit for verified address
  leads, and [Gold Box Explorer](https://github.com/bsimser/Gold-Box-Explorer)
  for independently checked format semantics.
- [The DOS Decode Kit](https://github.com/DrEvil-TitaniumHelix/dos-decode-kit)
  publishes extraction, overlay, disassembly, and ECL research
  specifically for Pool of Radiance. Its
  [findings](https://github.com/DrEvil-TitaniumHelix/dos-decode-kit/blob/master/games/pool-of-radiance/docs/FINDINGS_pool_of_radiance.md)
  and [ECL notes](https://github.com/DrEvil-TitaniumHelix/dos-decode-kit/blob/master/games/pool-of-radiance/docs/ECL_VM_DECODED.md)
  are the first leads to reproduce. Its authors explicitly call the browser
  game a **reconstruction**, and its
  [combat implementation](https://github.com/DrEvil-TitaniumHelix/dos-decode-kit/blob/master/games/pool-of-radiance/reproduction/combat.js)
  includes simplified rules and generated encounters. Its own findings also
  mark some map interpretation as unconfirmed. We will cite individual facts
  only after checking them against this DOS build. The repository calls its
  tools and documents MIT-licensed, but its
  [license text](https://github.com/DrEvil-TitaniumHelix/dos-decode-kit/blob/master/LICENSE)
  abbreviates the standard terms. Use its findings as research and clarify
  permission before copying its code.
- [Gold Box Explorer](https://github.com/bsimser/Gold-Box-Explorer) provides
  MIT-licensed DAX/GEO format code. The neighboring Mac mapper already adapted
  its decoder into a [checked Java reader](docs/prior-research/macmaps/code/DaxReader.java)
  and [GEO model](docs/prior-research/macmaps/code/GeoMap.java), and
  independently studied maps, events, party state, and combat. Eleven relevant
  notes and those two reference classes are preserved unchanged in
  [the Mac research snapshot](docs/prior-research/macmaps/README.md). Shared
  data formats are useful leads; Mac v1.1 memory addresses and save layout are
  not DOS v1.3 facts.
- [Blackthorn's machine-code disassembly discussion](https://forums.goldbox.games/index.php?topic=4605.0)
  may provide mechanics cross-checks. The
  [Apple II JavaScript project](https://github.com/stlhood/pool-of-radiance-js)
  is work in progress for different machine code and disk media. We found no
  published complete, faithful, portable C decompilation of this DOS release
  in this search; that is a search result, not proof none exists.

All `resources/` content is Git ignored. Research reports, tests, and code may
be committed; original executables, disks, saves, ROMs, extracted text, and
art stay local. Retain upstream notices with any adapted MIT-licensed code.

## Proposed system boundary

```text
Owner's DOS data files ──> C format readers ──> C game core ──> platform API
                                             |                 |
                                             |                 +─ desktop backend
                                             +─ ECL VM            (future: DS)
```

The core owns game state, rules, ECL execution, random numbers, and saves. The
platform API supplies files, input, pixels, sound, and time. Use fixed-width
types, explicit little-endian reads, bounded buffers, and an injected random
seed and clock so runs can be replayed. The core should have no DOS interrupts,
host windowing calls, or dependency on the original executable at runtime.

## Accelerated decompilation method

1. **Generate an overlay loader and symbol ledger.** Parse the MZ resident
   image, all 34 `FBOV` unit descriptors, entry stubs, and per-unit fixup
   tables. Give each routine a stable `unit:offset` identity and resolve near
   and far calls through applied fixups. Export the map and call graph as
   reviewable metadata. The kit's simple segment formula leaves many call
   edges unresolved; apply the real fixups before judging function coverage.
2. **Batch decompile 16-bit x86.** Load reconstructed units into Ghidra with
   their correct 16-bit address spaces and calling conventions. Label Turbo
   Pascal runtime helpers, overlay thunks, known globals, record layouts, and
   ECL handlers; re-run analysis after types and call targets improve. Compare
   selected functions with a second decompiler or manual disassembly. Ghidra's
   [MZ importer](https://github.com/NationalSecurityAgency/ghidra/blob/master/Ghidra/Features/Base/src/main/help/help/topics/ImporterPlugin/importer.htm)
   does not by itself describe this custom `FBOV` overlay format, so the loader
   is part of the work. Decompiled C is evidence, not a compilable source tree;
   [Reko's project discussion](https://github.com/uxmal/reko/discussions/1286)
   makes the same limitation explicit for DOS binaries.
3. **Translate by subsystem.** First make a small portable C compatibility
   layer for 16-bit integer wrapping, Pascal strings/sets, far pointers where
   behavior needs them, and deterministic random/time inputs. Translate the
   recovered ECL VM and data readers, then movement, party, combat, menus, and
   saves. Preserve `unit:offset` provenance in source comments or a manifest.
   The 29 area scripts are data for one VM, not 29 separate engines.
4. **Use the DOS game as an oracle.** Capture fixed input sequences and game
   state from the original under a DOS emulator, then replay them against the
   C build. Compare state transitions, text/event IDs, combat outcomes, and
   save bytes where meaningful. This turns ambiguous pseudocode into specific
   questions. Keep original media and captures under ignored `resources/`.

**Feasibility gate before a delivery estimate:** In a short pilot, recover the
full unit/entry inventory and fixup-aware call graph, then decompile three
representative routines: a leaf, an ECL handler, and a stateful gameplay
routine. Record recovered calls, unresolved indirect edges, data types,
manual corrections, and time to obtain behaviorally checked C for each. Use
that measured throughput and a runtime/game-code classification to estimate
the remaining work. The pilot can expose a fast path; it can also reveal a
compiler or overlay obstacle early. No total-port timeline is credible until
this gate has results.

## Milestones and exit gates

| Stage | Work | Evidence required to leave the stage |
| --- | --- | --- |
| **0. Freeze the DOS oracle** | Hash every supplied input; identify the executable revision; boot an untouched copy under DOS emulation; record repeatable menu, movement, event, combat, and save/load traces. Keep fixtures and captures local. | A second person can replay the same scenarios and identify the exact game files used. |
| **1. Recover the data contract** | Reproduce the DAX directory/RLE reader in C; enumerate and validate every local archive and record; decode GEO, ECL, art, characters, items, and saves one family at a time. Compare the Mac and public decoders only where their bytes agree. | Every supported record has bounded parsing and measured counts/sizes; malformed inputs fail safely; maps and art match original views on selected samples. |
| **2. Map and decompile the program** | Run the feasibility gate above, separate Turbo Pascal runtime from game code, then batch decompile the overlay and resident image. Build an address-to-function ledger with confidence and supporting bytes, calls, strings, or live traces. | All executable regions and overlay entries are inventoried; fixup-aware calls and unresolved edges are reported; each function needed for the first playable route has a reviewed behavioral contract. Unknown behavior remains labeled unknown. |
| **3. First native route** | Implement the C state model, menus, input, map movement, one real ECL route, one real battle, and save/load through a desktop backend. Use original DOS data at runtime. | A deterministic New Phlan-to-Slums style route produces the same significant positions, prompts, party changes, encounter outcome, and reload state as the DOS oracle. No placeholder encounter or invented rule passes this gate. |
| **4. Complete DOS parity** | Finish event opcodes and text, all area and wilderness travel, character creation and advancement, items, spells, combat rules and AI, services, quests, ending, and DOS art/audio behavior. Resolve differences with focused disassembly or original-game experiments. | A full campaign playthrough and a regression set spanning alternate choices, battles, spells, transitions, and save/load agree with the DOS game on meaningful state and outcomes. Document intentional fixes separately from fidelity gaps. |
| **5. Portable C release** | Remove host assumptions; build and exercise desktop backends on Linux, Windows, and macOS. Choose a source license, review third-party notices, and publish reproducible build instructions and an owner-supplied-data setup path. | The same game-data set loads and the same replay tests pass on each supported desktop platform without `START.EXE` or `GAME.OVR` executing. |
| **6. Optional Amiga art** | Decode the supplied Amiga disks' portraits and sprites, map their IDs/frames to DOS assets, and add a selectable visual set. Preserve palette and animation intent. | Every image used by the completed DOS game has an audited mapping or documented fallback; game logic and saves remain compatible. |
| **7. Optional C64 audio** | Obtain owner-supplied C64 media, recover music and sound cues, map them to game events, and add an audio backend or documented conversion path. | Music and effects trigger at the intended events without changing game state; the data and any player library have clear redistribution terms. |

The supplied Amiga files are local ZIP archives. `resources/C64/` is currently
empty, so stage 7 needs source media. The Nintendo DS port can start after
stage 5, with memory, graphics, input, and audio budgets assessed against the
portable core; it remains a separate project.

## Working rules

1. Record the exact binary version, file offset, test scenario, and confidence
   for every reverse-engineered claim. Treat a plausible decompiler rendering
   as a hypothesis until behavior or independent byte evidence supports it.
2. Port small, proven format routines from prior work with attribution. Write
   gameplay code from verified contracts and compare it with the DOS oracle.
3. Keep a behavior-difference ledger and replay tests. Prioritize the first
   playable route, then widen coverage; do not declare the game complete from
   asset extraction or a short demo.
4. Keep private media and generated derivatives under ignored paths. Review
   commits for embedded game text or binary data before publication.

**Decision for review:** use DOS v1.3 as the reference behavior, preserve
observed quirks until understood, and make any bug fixes explicit and testable.
This keeps the initial fidelity target unambiguous while leaving room for
later quality-of-life options.
