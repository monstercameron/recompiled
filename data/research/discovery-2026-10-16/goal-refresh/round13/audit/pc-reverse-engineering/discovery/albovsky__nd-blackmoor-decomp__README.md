# Blackmoor reconstruction project

Permanent project: `/Users/albovsky/Projects/nd-blackmoor-decomp`.

Read `outputs/blackmoor-decomp/PROJECT-HANDOFF.md` for the completed slice and the current limits. Read `outputs/blackmoor-decomp/WORK-SLICES.md` to choose a parallel work packet. The buildable source and saved deliverables are in `outputs/blackmoor-decomp/`; original-byte evidence and local tools are in `work/`.

The original GOG app is unchanged. This reconstruction is incomplete and is not yet a playable replacement.

## x86 matching progress (objdiff, ByteMatch)

`objdiff.json` defines one objdiff unit per function of the original `.text` (4,752: the M03 census split plus the two unsplit CRT spans), with categories `game`, `vorbis` and `crt_mfc`. Target objects are written at build time from the local `work/original/GAME.EXE`, one function code part per object. Base objects exist only for functions in the class `tool_assisted_reconstructed_assembly`: accepted in an audited `asm/batch-*-report.json` and unchanged since. They are assembled from `outputs/blackmoor-decomp/asm/*.s` with the pinned GNU as 2.43.1. Every other function counts as unmatched. Objects and the report go under the git-ignored `build/`, and no original bytes are committed. `script/objdiff_units.py` explains the details, including one storage-class change to the base object's data-item labels that keeps objdiff from sizing switch tables as code.

```bash
script/upload-bytematch.sh --no-upload   # regenerate objdiff.json, objects and build/report.json
script/upload-bytematch.sh               # same, then upload to bytematch.dev (version "gog", current HEAD)
```

The upload uses the per-project token in `~/.bytematch-token` and refuses to run while `objdiff.json` or `asm/` has uncommitted changes. After a new batch is accepted, rerun the script and commit the regenerated `objdiff.json`. objdiff-cli 3.8.2 is fetched into `work/tools/objdiff/` if it is missing.
