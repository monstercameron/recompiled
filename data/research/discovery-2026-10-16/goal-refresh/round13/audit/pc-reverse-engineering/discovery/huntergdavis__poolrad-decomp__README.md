# Pool of Radiance DOS decompilation

This repository is for reconstructing the supplied DOS v1.3 Pool of Radiance
engine as portable C. The immediate deliverable is the
[roadmap for review](ROADMAP.md). Two parallel lanes are defined in
[WORKSTREAMS.md](WORKSTREAMS.md): Claude recovers DOS code and ECL behavior;
a Codex worker builds the portable C data layer. The primary session reviews
and integrates their pull requests.

The owner's original game files live under `resources/`, which Git ignores.
Do not commit game binaries, saves, extracted assets, or generated game text.
The [Mac research snapshot](docs/prior-research/macmaps/README.md) records
relevant findings and two DAX/GEO reference classes from the neighboring
`poolrad-macmaps` project with their source revision and checksums.
An [isolated audit of the DOS Decode Kit](docs/FABLE_AUDIT.md) records the
verified overlay inventory and corrections to published format claims.

DOS gameplay comes first. Once it works, the roadmap calls for optional Amiga
portraits and sprites, followed by C64 music and sound. A Nintendo DS port is
a later project.

The PM's [DOS oracle setup](docs/oracle/README.md) prepares isolated local
runs for behavior comparisons. It keeps every original file and capture under
ignored `resources/`.

The [local evidence dashboard](tools/dashboard/README.md) shows the function
triage queue, commit-bound build results, and scoped DOS/C comparisons. Its
review tags and generated reports stay local; identity evidence is kept
separate from strict full-C credit.
