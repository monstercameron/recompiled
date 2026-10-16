# Emulator discovery notes — 2026-10-15

This focused lane reviewed the 178 existing `Emulator` rows, the earlier `11-emulators` and `27-emulators` discovery lanes, generated `docs/catalog.json`, the central reviewed project-addition/update manifests, and other staged manifests. Candidate repository roots and target identities were checked against those sources before inclusion. No catalog database, generated docs, central manifest, or import was changed.

The additions cover gaps in Japanese home computers (PC-6001 and Sharp MZ-700/800/1500), Apple Lisa, a modern Tamagotchi Paradise handheld, a CPS-1 arcade project, new NES/SNES source projects, and two recent CPU/full-system emulation research projects. The console, handheld, and arcade catalog already contains broad coverage, so this batch adds discrete repositories rather than another generic frontend or aggregator.

| Project | Target | Release/activity checked | Language | Scope note |
|---|---|---|---|---|
| PC6001VX | NEC PC-6001 family | v4.5.1, 2026-10-02 | C++ with C/assembly components | Original ROMs normally needed; compatible-ROM mode has documented limits. Linux/Unix-like and Android are maintainer targets; Windows binaries are also published. |
| mz800emu | Sharp MZ-700/800/1500 | v2.0.6, 2026-10-14; source push 2026-10-15 | C and C++ | README lists Linux/Windows; BSD is not regularly tested. |
| LisaEm | Apple Lisa | README version 1.2.7 RC4 (2022-04-01); source push 2024-07-24; no GitHub release | C and C++ | Current maintenance, supported models, and host matrix remain unclear; source and build documentation are public. |
| Capybara | Tamagotchi Paradise / Sonix SNC73410 | v0.2.2, 2026-08-31; source push 2026-09-10 | Rust | Requires a dump from the user's own device. README says four editions were tested and two remain untested. |
| sfemu | CPS-1, Street Fighter I/II configurations | v0.1.0, 2026-08-29; source push 2026-08-29 | Rust | Experimental, limited title scope; later maintenance and host matrix are unclear. |
| RustyNES | NES/Famicom, FDS, Vs./PlayChoice-10 | v3.2.0, 2026-10-15 | Rust | Maintainer discloses LLM-assisted development. README's version badge lags the current GitHub release; the release endpoint supplied the version recorded here. |
| RustySNES | SNES/Super Famicom | v1.31.1, 2026-08-05; source push 2026-08-20 | Rust | Maintainer discloses heavily AI-assisted development; accuracy and compatibility claims remain attributed to its own docs. |
| Sem86 | x86 full-system PC | Source push 2026-08-21; no GitHub release | Rust (primary) | Research project, explicitly not ready for production; documented hosts are Linux and Android. |
| rax | x86-64, Arm, Hexagon, RISC-V, eBPF/SBPF | v0.1.2, 2026-09-16; source push 2026-10-15 | Rust | Active research project; documented execution paths and limits are summarized in the row, not generalized to full ISA or OS compatibility. |

The two metadata updates only fill `updated` dates on 3Beans and Panda3DS. Their existing status text already carried the same source-activity dates, and their system, language, and compatibility fields were left as-is.

Candidates excluded from the new-project list included `trs80gp`, because the current project page distributes binaries but no public source repository; and `Emux`, whose public source was last pushed in 2021 and did not provide a current gap-filling target. The catalog already has wide 3DS, Switch, PlayStation, Xbox, arcade, and established home-computer coverage, so duplicate frontends, forks without a distinct documented target, and generic emulator utilities were not added.

Dates in the rows are snapshots checked on 2026-10-15. GitHub push dates are activity signals; they are not presented as compatibility claims. Project start dates remain blank unless a reliable source explicitly stated them. LisaEm, Sem86, and sfemu retain explicit unknown release/maintenance or support boundaries. No ROM, firmware, BIOS, key, or game media was copied into the catalog.
