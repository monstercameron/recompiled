# Round 13 API translation candidates

Checked 2026-10-16. This staging lane contains two repository-verified software projects that were absent from SQLite and prior staged research. It does not modify the catalog database or tracked files.

`reims-vgpu` is a QEMU virtual GPU for macOS guests. It is included as an API-translation candidate because its host implementation replaces `ParavirtualizedGraphics.framework` and executes the Apple guest driver’s paravirtual GPU command stream through native Metal or Vulkan. The scope is narrow: it is a guest-device command bridge, not general application-level Metal compatibility. README status is Beta; the release API has no tags.

`metal-api-emulator` is an independent source-level Metal-shaped API prototype with Vulkan and native Metal providers. It is included as a narrow API-translation experiment, not a binary compatibility runtime. The README explicitly disclaims a `Metal.framework` ABI and lists substantial unimplemented features. Its README describes a local experimental reims adapter; that integration is not upstream-approved.

The exact official GitHub Languages API byte maps, repository metadata, current commits, release checks, license evidence, source links, and duplicate checks are in the adjacent JSON files. No media was found or downloaded.

`metal2vulkan` is already cataloged as `metal2vulkan`; no new project row or status patch is staged. The current row already says Alpha, notes that workspace manifest version 0.2.0 has no published GitHub release and that v0.1.0 (2026-07-22) remains latest, and describes the documented API subset, Vulkan 1.2 requirement, typed refusals, and behavioral-compatibility limits.

## Validation

All JSON files in this directory parse. Project IDs and exact canonical repository URLs were checked against the 3,462-row SQLite catalog and prior research text. `canonical_database_modified` is false.
