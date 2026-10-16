# Game profile metadata hydration

Verified from the per-partition research snapshots dated 2026-10-15 and 2026-10-16. This reviewed import merges facts for 638 exact game keys across the alphabetic catalog.

- Raw researched facts: 3,573.
- Researched facts matching existing SQLite claims: 303; these remain at their original positions and keep their original `verified_on` values.
- Newly added sourced facts: 3,260.
- Existing fact rows copied forward for the profiles being refreshed: 303; their source, claim, granularity, and verification date are preserved.
- New profiles covered: 568; all exact keys exist in `games`.
- Profile keys with no metadata before this import: 568.
- Ten additional duplicate exact claims within incoming snapshots were collapsed. The repeated X-COM facts are included once.

Every added fact has a direct HTTPS source and claim-specific evidence. New claims receive the verification date from their research partition. The T supplement has no batch date because it carries old undated facts; its new claims use the 2026-10-15 verification date documented in its companion research notes. Existing rows without a verification date remain undated. The manifest intentionally has no top-level `verified_at`, so the importer cannot assign a batch date to those carried-forward facts.

This lane is imported after the earlier game-metadata lane in `scripts/build.sh`; it includes all existing facts for each game key it replaces, so the replacement importer does not discard prior claims. Ten existing profiles outside this alphabetic batch are left untouched.
