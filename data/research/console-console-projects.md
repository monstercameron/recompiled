# Console-to-console project review

Reviewed on 2026-10-15. The companion JSON manifest contains 85 game-specific catalog rows sourced from the projects' own repositories. It covers Dreamcast, PlayStation Vita, Nintendo 3DS, Wii U, Switch, PlayStation 3/4/5, PSP, Wii, and the original Xbox.

This batch includes native ports built from decompilation work, static recompilations, and native reimplementations. A repository that supports more than one game has a separate row for each game; one repository that supports several target consoles for the same game remains a single row. Where a project documents only a build target, the status says so and does not imply that the game is playable. Hardware testing and performance claims are included only when the maintainer reports them.

The batch excludes emulators, PC-only source ports, and ambiguous console-source claims. In particular, the Wii U Sonic Mania lead was not imported because its README did not pin the source build to a specific console release. WipEout for Dreamcast was also left out because the Dreamcast port derives from a PC reimplementation, not directly from the PlayStation game. A console target documented without compatibility or gameplay evidence is marked accordingly.

Import and rebuild with:

```sh
python3 scripts/catalog.py import data/research/console-console-projects.json
python3 scripts/catalog.py export
./scripts/build.sh
```

The build script imports this manifest automatically. Each entry links to its project repository or the target-specific branch, where source revisions, build instructions, supported game data, and current limitations can be checked.
