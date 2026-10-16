# Parallel workstreams

The CTO sets scope and priorities. The primary Codex session coordinates the
project, reviews evidence, and integrates pull requests. Four worker lanes are
defined. When staffed, workers use isolated deep copies and exchange changes
through the public GitHub repository. The currently assigned lanes are one
Claude C worker, one Codex C worker, and one Codex native replay worker, each
in an isolated copy; temporary capacity holds are recorded in issue #3.
Workers use their own branches and open pull requests; the primary
session merges after review.

| Lane | Owner | Delivered; current focus | Owned paths |
| --- | --- | --- | --- |
| A: DOS code recovery | Claude | Thirty-four full overlay translations merged through PR #108; COMPARE, IF and RETURN are bounded models, the latter merged in PR #119. Idle pending PM assignment. | `tools/overlay/`, `docs/re/`, `src/recovered/ecl_*`, `include/poolrad/recovered/ecl_*`, `tests/recovered/test_ecl_*` (no edits to Codex's resident files) |
| B: portable data foundation and resident recovery | Codex worker (p5) | Nine full resident translations merged through PR #79. The entry-2 Up slice and coordinate-writer hook correction merged in PRs #163/#167; the bounded resident `0x8AE:0x00DC` dispatch and `+0x181A` leaf merged in PR #171. The bounded `+0x17AE` table writer merged in PR #176; the native one-Up dispatch branch is now measured. The full unit `0x9E` entry-8 dice helper is reviewed; current focus is the ECL RANDOM runtime-call chain using live seed/result evidence. | `src/formats/`, `include/poolrad/formats/`, `tests/formats/`, `CMakeLists.txt`, `docs/formats/`; resident-specific files under `src/recovered/`, `include/poolrad/recovered/`, `tests/recovered/`, and `docs/re/` (no edits to Claude's ECL helper files or overlay research) |
| C: separate overlay recovery | Claude worker 2 (p6) | Unit `0x68` entry 1 bounded model merged in PR #156, full party-byte and field-clear helpers in PRs #158/#162, bounded roll-gate and tile-step models in PRs #165/#166, and saved-state gameplay loop `0x1C:+0x3387..+0x3507` in PR #169. The reached `+0x3171` script runner already has the lane-A callback model. The AND/OR handler `0x1C:+0x0DEF` and its `0xC04F` load-word source merged in PRs #175/#177. Saved search handlers and the class-1 load hook are merged through PR #192. The slot-A party model and RNG audit merged in PR #193; the complete unit `0x9B:+0x01AF` record updater merged in PR #194. Idle pending the next bounded target. | New uniquely named overlay recovered source, header, test, and evidence files only; coordinate any shared CMake edit. |
| D: native replay evidence | Codex worker 2 (p9) | Native exactly-one-Up minute and coordinate watches merged in PRs #153/#157; saved-game C restore adapter merged in PR #160 with explicit key-dispatch blocker. The native prompt key/extended-flag ABI merged in PR #170. Local unpublished watches now reach saved gameplay, the resident dispatch branch, and the first ECL RANDOM seed/result/store. Current target is the actual post-RANDOM script branch. | Private `resources/` and ignored captures in its own clone; public output only address/hash/count evidence or synthetic replay code. |
| Integration | Primary Codex session | Review PRs, maintain roadmap and issue tracker, establish DOS replay/oracle cases, track the game-function denominator in issue #24, and connect the lanes at the first playable route. | `README.md`, `ROADMAP.md`, `WORKSTREAMS.md`, `.github/`, `tools/overlay/function_evidence.py`, `docs/re/OVERLAY_FUNCTION_INVENTORY.md` |

## Shared rules

1. Use branches named `claude/<topic>` or `codex/<topic>` and open a PR to
   `main`. Put the target DOS executable hashes, input files, measured output,
   and unresolved questions in the PR. Keep changes within the lane's paths
   unless the primary session agrees to an interface change.
2. Coordinate interface changes in GitHub issues or PR comments. Lanes A and C own
   their assigned overlay addresses and behavioral claims; lane B owns portable file-format
   APIs. Every lane cites evidence and distinguishes verified behavior from guesses.
3. Never commit `resources/`, original executables, saves, extracted game
   text, art, audio, or private captures. Tests use synthetic data. A local
   integration audit may read owner-supplied files but must report only counts,
   checksums, and errors. Retain notices when adapting reference code.
4. The primary session checks that branches rebase on current `main`, reviews
   code and reports, runs relevant verification, and merges in dependency
   order. A completed PR updates the roadmap or an issue with what is now
   known and what remains uncertain.

The C parser audits all local DOS DAX archives, the overlay map and Ghidra
feasibility pilot are published, and the ECL, EGA, and WALLDEF readers are merged.
Sixty-six verified game-specific routines have strict full-C translations across the
overlay and resident lanes: fifty-seven overlay and nine resident. PR #64's
separate `0x464` helper has unresolved source ownership.
Three handler/callee models have unresolved dependencies or heap semantics;
PRs #65/#72 add separate path-complete script-state initializer and ECL block-loader models.
Two string routines have path-complete C translations with a provisional runtime
cap, and five more have extracted C cores whose engine side effects remain to be
translated. The regenerated classifier records 118 verified overlay identities,
including PRs #63/#69's twenty unit `0x38` entries, PR #65's view setter,
PRs #106/#114/#124/#126/#128's unit `0x3BD` entries 6/7/4/2/3, PRs #130/#132's unit `0x7F` entries 10/11, PR #108's two compare helpers, PR #136's unit `0x25` entry 19, PR #138's unit `0x68:+0x068D` preflight, PR #145's `0x68:+0x05AB` scan, PR #149's `0x68:+0x05EE` list search, PR #152's `0x68:+0x0648` record clear, PR #150's four unit `0x3C6` slot helpers, PRs #158/#159's `0x68:+0x0236` party test and `+0x0109` map updater, PR #162's `0x68:+0x0000` field clear, PR #194's `0x9B:+0x01AF` record update, PR #276's `0x9B:+0x0255` table leaf, PR #279's `0x9B:+0x0329` field leaf, PR #281's `0x9B:+0x0000` caller, and PR #286's `0x5E:+0x2879` party value,
and the callable ledger records ten resident identities. The combined
evidence-backed floor is therefore 128 game routines. The full game-function
denominator remains open. The merged DOS one-Up watches now identify the first
value-changing writer and the prompt return ABI; the bounded gameplay-loop,
AND/OR, and resident-dispatch C models still have unresolved hooks. The saved
AND/OR source `0xC04F` now resolves to host-supplied DS byte `[0x6AB1]`, whose
live value on that route remains unmeasured. C-native one-Up parity remains open at the key-dispatch, actual search
reachability, conditional AL=13 DIVIDE, and remaining runtime-state
integration gates. The measured first native RANDOM result is AL=10;
the `+0x3171` script runner is already a
merged callback model.

The independently reviewed full-C game dice helper, unit `0x9E` entry 8,
is included in the counts above. A native replay now measures the saved one-Up
`RES_08AE:00DC` branch (mode 1, active word 100, argument 10 and its selected
calls), while the callees' complete effects and C-native parity stay open.
The native ECL RANDOM watch now measures the first saved script RNG seed
`3140872353 → 3238563622`, result 10, and window-2 word `16 → 10`.
