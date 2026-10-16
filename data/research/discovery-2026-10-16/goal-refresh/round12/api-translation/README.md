# API and runtime compatibility research — round 12

Checked on 2026-10-16 against the 3,411-project SQLite catalog, generated research manifests, and work staging manifests. This focused pass searched for distinct Direct3D/Vulkan/Metal, Wine/Linux/Android, and console/API compatibility projects. Known graphics/runtime layers and prior research proposals were deduplicated rather than repeated.

## Staged addition

One distinct runtime is proposed in project-additions.json: Wine Mono, the Wine-specific implementation of .NET Framework 4.8.1 and earlier for Windows managed applications running inside Wine. It packages a modified Framework Mono runtime, class libraries, and related components and integrates with Wine's built-in mscoree.dll. It is a managed-framework reimplementation, not a Windows system API translator. The catalog's existing mono-dotnet-runtime-legacy-upstream row tracks the general legacy Mono runtime and its own source line; it is not the separately released Wine Mono package.

WineHQ's current Wine-user release tag is wine-mono-11.3.0 (2026-08-17). wine-mono-11.2.1 (2026-09-04) is explicitly a Proton bug-fix release and directs Wine users to 11.3.0. The latest canonical source commit on 2026-10-14 only added a GitHub pull-request policy template, so the status description does not imply that this housekeeping commit changed runtime compatibility.

The canonical source is WineHQ GitLab. Wine Mono's README identifies the GitHub repository as an official mirror; its main commit SHA matched GitLab's at review time. The exact GitHub Languages API bytes are recorded in programming-languages.json. C is the largest aggregate bucket, but the result includes substantial upstream Mono/runtime and bundled component source and does not isolate Wine-specific work. The project COPYING file documents multiple component licenses; GitHub's repository-level classifier is NOASSERTION, so this import does not claim one repository-wide SPDX license.

No eligible direct project-linked image or video URL was found, so there is no media record. Media, if added later, should remain URI-only and directly linked to Wine Mono's own source/release page.

## Validation and exclusions

- The addition and language manifests parse and match the catalog importer shapes.
- Exact project ID and canonical URL are absent from SQLite and were absent from data/research/ and work/ before this lane was staged.
- The related generic Mono runtime was reviewed and retained as a separate project because its source target and release line differ.
- The sampled Direct3D/Vulkan/Metal/Wine layers, including D8VK, DXMT, D9MT, MTLD3D, DXVK, VKD3D-Proton, MoltenVK, Dozen, Zink, LSW, Weave, and ZLUDA, are already cataloged.
- evidence.json records the canonical GitLab metadata, exact source revision identity of the official mirror, current release distinction, language map, and license evidence. exclusions.json records the screened nearby projects and dispositions.
- scripts/catalog.py importer validation is run against a disposable SQLite copy only; this lane does not write to data/catalog.sqlite or tracked site files.
