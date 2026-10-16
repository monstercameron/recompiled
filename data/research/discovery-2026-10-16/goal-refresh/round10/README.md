# Round 10 catalog research

Round 10 begins with a source-checked Wikipedia reference pass for every game key that lacked one after round 9.

## Wikipedia references

- `wikipedia-links-reviewed.json` fills 124 missing references: 42 direct English Wikipedia articles and 82 exact-title Wikipedia search links.
- `wikipedia-candidate-audit.csv` records the exact catalog key, project context, match decision, and ambiguity notes for every entry. No fuzzy title matches were used.
- `wikipedia-notes.md` records source review and validation details.
- Alice’s Cottage II has no verified standalone English Wikipedia article; it uses an exact-title search link, keeping the PC-98 translation target discoverable without conflating it with other AliceSoft games.

The batch was tested with the real importer against a disposable database. The build imports these references and exports them into `docs/catalog.json`.

## Programming-language evidence

- `programming-languages-reviewed.json` records 264 previously blank project-language entries: 133 source-backed predominant-language assignments and 131 explicit unknowns. The unknown rows retain their evidence and reason so the filter does not guess from a project's translated language or target platform.
- GitHub, GitLab, Codeberg, and project-owned documentation are used where available. Repository byte maps remain distinct from GitLab percentage maps, and each assignment names its evidence URL and verification date.
- One single-file Python patcher was not treated as proof of a translation project's predominant language.
- The combined manifest passed the real importer against a disposable copy of the full catalog; 264 evidence rows were written, integrity and foreign-key checks passed, and all target project language fields were blank before import.
