# Lane 10 — reverse-engineering and recompiler tools

Research checked 2026-10-15. This lane staged 33 new rows: 11 decompilation or build-matching tools, 19 static recompilation toolchains/frameworks, and 3 binary or embedded-system emulation frameworks. Every proposed row targets Software; target-specific game integrations remain in separate catalog rows where applicable.

## Verification and deduplication

Reviewed each candidate's own GitHub README, release page when available, repository metadata (activity, language, and license), and any linked supported-target/status matrix. Checked candidate repository URLs against the current `projects` and `directory_items` tables and all current discovery manifests before staging. Candidates that had already appeared in the catalog—such as NGageRecomp—were excluded. No current catalog project needed a metadata correction in this lane, so `metadata-updates.json` is deliberately empty.

The dated `updated` value in each row records the catalog check date. `first_found` is the date this discovery lane found the record, not the project's start date. No start dates are asserted. The programming-language value uses the repository's primary language reported by GitHub; for mixed-language repositories it is not a complete language inventory.

## Scope boundaries and caveats

- The project taxonomy has no dedicated reverse-engineering-helper or ISA-translation kind. Assembly comparison and source permutation helpers are placed under Decompilation with helper-specific subtypes; process-level CPU translation (Box86) is placed under Emulator and explicitly labeled as user-mode rather than full-system emulation.
- Recompiler frameworks are listed as Software targets because they are reusable toolchains. Their release/game compatibility belongs to the separate title repositories they consume; framework milestones are not represented as whole-library compatibility claims.
- RetDec remains public and receives basic maintenance, but its own README warns that it is in limited-maintenance mode and has very limited feature development.
- pycdc has no tagged release and does not publish a supported-bytecode matrix; the row explicitly marks its compatibility range uncertain. uncompyle6 documents bytecode input through Python 3.8.
- PS1Recomp reports an experimental Crash Bandicoot 1 boot to the main loop. psprecomp reports generated code can compile/link/execute but no game renders yet. vitarecomp reports HLE remains in progress. DolRecomp calls Wii U support work in progress. PortableKit explicitly says its extracted framework is incomplete.
- WiiKit, ndskit, 3dokit, Saturn-recomp, and other reusable toolkits cite separate game-port repositories. The toolkit records describe the shared toolchain only; individual port records should be independently deduplicated and status-checked.
- Tutorials, how-to pages, and generic documentation are not project rows. If selected as useful reading later, they belong in the process-resources table with their own resource importer.

## Exclusions

- Existing catalog or directory entries: Ghidra, decomp.me, objdiff, splat, decomp-toolkit, N64Recomp, XenonRecomp, XboxRecomp, PS4Recomp, ReXGlue, REA, NGageRecomp, Box64, QEMU, and existing emulator/framework rows.
- Rizin/Cutter: active RE/disassembly front ends, but not standalone source decompilers or binary recompiler toolchains matching this lane's project categories.
- Apktool: resource decoder and smali disassembler/rebuilder; it does not reconstruct Java source.
- PSXRecomp/RetroPortingToolKit: PolyForm Noncommercial terms do not meet the open-source inclusion rule.
- Projects without a verifiable open-source license were excluded.
- Generic tutorials and documentation were excluded from the projects manifest because the catalog classifies those as process resources, not projects.

## Staged files

- `new-projects.json`: 33 fully hydrated project rows with official evidence links.
- `metadata-updates.json`: empty update list; none of the candidate repositories matched an existing catalog project requiring factual correction.
