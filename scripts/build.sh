#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
if [[ -f data/research/emulators.json ]]; then
  python3 scripts/catalog.py import-emulators data/research/emulators.json
fi
if [[ -f data/research/github-discovery-additions.json ]]; then
  python3 scripts/catalog.py import data/research/github-discovery-additions.json
fi
if [[ -f data/research/console-phone-projects.json ]]; then
  python3 scripts/catalog.py import data/research/console-phone-projects.json
fi
if [[ -f data/research/console-console-projects.json ]]; then
  python3 scripts/catalog.py import data/research/console-console-projects.json
fi
if [[ -f data/research/mobile-port-projects.json ]]; then
  python3 scripts/catalog.py import data/research/mobile-port-projects.json
fi
if [[ -f data/research/software-app-projects.json ]]; then
  python3 scripts/catalog.py import data/research/software-app-projects.json
fi
if [[ -f data/research/verified-additions-2026-10.json ]]; then
  python3 scripts/catalog.py import data/research/verified-additions-2026-10.json
fi
if [[ -f data/research/project-research-reviewed-2026-10.json ]]; then
  python3 scripts/catalog.py import data/research/project-research-reviewed-2026-10.json
fi
if [[ -f data/research/discovery-2026-10-15/project-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-15/project-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-15/project-updates-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-15/project-updates-reviewed.json
fi
if [[ -f data/research/media.json ]]; then
  python3 scripts/catalog.py import-media data/research/media.json
fi
if [[ -f data/research/mobile-port-game-profiles.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/mobile-port-game-profiles.json
fi
if [[ -f data/research/discovery-2026-10-15/15-game-metadata/game-facts.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-15/15-game-metadata/game-facts.json
fi
if [[ -f data/research/discovery-2026-10-15/48-game-profile-hydration/game-facts.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-15/48-game-profile-hydration/game-facts.json
fi
if [[ -f data/research/wikipedia-links.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/wikipedia-links.json
fi
if [[ -f data/research/wikipedia-links-additions-2026-10.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/wikipedia-links-additions-2026-10.json
fi
if [[ -f data/research/discovery-2026-10-15/15-game-metadata/wikipedia-links.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-15/15-game-metadata/wikipedia-links.json
fi
if [[ -f data/research/discovery-2026-10-15/new-game-wikipedia-links.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-15/new-game-wikipedia-links.json
fi
if [[ -f data/research/discovery-2026-10-15/46-wikipedia-current-gaps/wikipedia-links.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-15/46-wikipedia-current-gaps/wikipedia-links.json
fi
if [[ -f data/research/directory.json ]]; then
  python3 scripts/catalog.py import-directory data/research/directory.json
fi
if [[ -f data/research/discovery-2026-10-15/18-tools-resources/directory-additions.json ]]; then
  python3 scripts/catalog.py import-directory data/research/discovery-2026-10-15/18-tools-resources/directory-additions.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/directory-additions.json ]]; then
  python3 scripts/catalog.py import-directory data/research/discovery-2026-10-16/next-wave/directory-additions.json
fi
if [[ -f data/research/programming-languages.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/programming-languages.json
fi
if [[ -f data/research/programming-language-additions-2026-10.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/programming-language-additions-2026-10.json
fi
if [[ -f data/research/project-language-evidence-2026-10.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/project-language-evidence-2026-10.json
fi
if [[ -f data/research/discovery-2026-10-15/programming-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-15/programming-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-15/47-programming-language-evidence/programming-languages.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-15/47-programming-language-evidence/programming-languages.json
fi
if [[ -f data/research/discovery-2026-10-16/project-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/project-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/project-updates-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/project-updates-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/programming-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/programming-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/media-reviewed.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/media-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/project-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/next-wave/project-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/project-updates-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/next-wave/project-updates-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/project-additions-round2-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/next-wave/project-additions-round2-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/project-updates-round2-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/next-wave/project-updates-round2-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/project-additions-round3-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/next-wave/project-additions-round3-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/activity-dates-round2.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/next-wave/activity-dates-round2.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/game-facts-round2.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/next-wave/game-facts-round2.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/programming-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/next-wave/programming-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/programming-languages-round3.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/next-wave/programming-languages-round3.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/media-reviewed.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/next-wave/media-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/media-round3.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/next-wave/media-round3.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/wikipedia-links-reviewed.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/next-wave/wikipedia-links-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/wikipedia-links-round2.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/next-wave/wikipedia-links-round2.json
fi
if [[ -f data/research/discovery-2026-10-16/next-wave/wikipedia-links-round3.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/next-wave/wikipedia-links-round3.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/project-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/project-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/project-additions-round2-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/project-additions-round2-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/wikipedia-links-reviewed.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/wikipedia-links-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/project-updates-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/project-updates-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/project-status-updates-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/project-status-updates-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/project-status-updates-round2.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/project-status-updates-round2.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/project-updates-round2-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/project-updates-round2-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/project-language-evidence-round4.json && -f data/research/discovery-2026-10-16/goal-refresh/project-language-evidence-new.json ]]; then
  python3 scripts/catalog.py import-programming-languages \
    data/research/discovery-2026-10-16/goal-refresh/project-language-evidence-round4.json \
    data/research/discovery-2026-10-16/goal-refresh/project-language-evidence-new.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/project-language-evidence-round2.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/project-language-evidence-round2.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/project-media-reviewed.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/project-media-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/project-media-round2.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/project-media-round2.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/game-facts-round2.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/goal-refresh/game-facts-round2.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/wikipedia-links-round2.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/wikipedia-links-round2.json
fi
if [[ -f data/research/console-gallery.json ]]; then
  if python3 - <<'PY'
import json
from pathlib import Path

manifest = json.loads(Path("data/research/console-gallery.json").read_text(encoding="utf-8"))
rows = []
for maker in manifest.get("makers", []):
    rows.append(maker.get("logo_asset_path", ""))
for source in manifest.get("source_logos", []):
    rows.append(source.get("logo_asset_path", ""))
for item in manifest.get("items", []):
    rows.append(item.get("asset_path", ""))
missing = [path for path in rows if not path or not (Path("docs") / path).is_file()]
if missing:
    print(f"Console gallery is not imported: {len(missing)} of {len(rows)} local image files are not cached yet.")
    raise SystemExit(1)
PY
  then
    python3 scripts/generate-console-gallery-previews.py --check
    python3 scripts/catalog.py import-console-gallery data/research/console-gallery.json
  else
    echo "Skipping console-gallery import; its local image set is incomplete."
  fi
fi
python3 scripts/catalog.py import-featured-projects data/research/featured-projects.json
python3 scripts/catalog.py import-process-resources data/research/process-resources.json
if [[ -f data/research/discovery-2026-10-15/18-tools-resources/resource-updates.json ]]; then
  python3 scripts/catalog.py import-process-resources data/research/discovery-2026-10-15/18-tools-resources/resource-updates.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round4/project-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round4/project-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round4/programming-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round4/programming-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round4/project-updates-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/round4/project-updates-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round4/game-facts-reviewed.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/goal-refresh/round4/game-facts-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round4/project-media-reviewed.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/round4/project-media-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round4/wikipedia-links-reviewed.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/round4/wikipedia-links-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round5/project-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round5/project-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round5/programming-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round5/programming-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round5/project-updates-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/round5/project-updates-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round5/wikipedia-links-reviewed.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/round5/wikipedia-links-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round6/project-updates-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/round6/project-updates-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round6/project-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round6/project-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round6/fan-localization-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round6/fan-localization-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round6/current-port-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round6/current-port-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round6/software-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round6/software-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round6/programming-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round6/programming-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round6/programming-languages-new-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round6/programming-languages-new-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round6/fan-localization-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round6/fan-localization-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round6/current-port-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round6/current-port-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round6/software-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round6/software-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round7/project-metadata-updates.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/round7/project-metadata-updates.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round7/media-manifest.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/round7/media-manifest.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round7/game-facts-reviewed.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/goal-refresh/round7/game-facts-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round7/wikipedia-links-reviewed.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/round7/wikipedia-links-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round7/emulator-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import-emulators data/research/discovery-2026-10-16/goal-refresh/round7/emulator-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round7/programming-languages-emulators-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round7/programming-languages-emulators-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round8/project-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round8/project-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round8/programming-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round8/programming-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round9/project-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round9/project-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round9/project-updates-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/round9/project-updates-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round9/programming-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round9/programming-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round9/game-facts-missing-reviewed.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/goal-refresh/round9/game-facts-missing-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round9/game-facts-additive-reviewed.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/goal-refresh/round9/game-facts-additive-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round9/wikipedia-links-reviewed.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/round9/wikipedia-links-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round9/project-media-reviewed.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/round9/project-media-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round9/directory-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import-directory data/research/discovery-2026-10-16/goal-refresh/round9/directory-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round10/reconciliation/project-additions.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round10/reconciliation/project-additions.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round10/reconciliation/directory-additions.json ]]; then
  python3 scripts/catalog.py import-directory data/research/discovery-2026-10-16/goal-refresh/round10/reconciliation/directory-additions.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round10/wikipedia-links-reviewed.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/round10/wikipedia-links-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round10/wikipedia-direct-improvements-reviewed.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/round10/wikipedia-direct-improvements-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round10/new-game-wikipedia/wikipedia-links.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/round10/new-game-wikipedia/wikipedia-links.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round10/programming-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round10/programming-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round10/reconciliation/project-language-evidence.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round10/reconciliation/project-language-evidence.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round10/game-profiles/A-L/game-facts-reviewed.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/goal-refresh/round10/game-profiles/A-L/game-facts-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round10/game-profiles/M-Z/game-facts-mz.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/goal-refresh/round10/game-profiles/M-Z/game-facts-mz.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round10/status-refresh/project-updates-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/round10/status-refresh/project-updates-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round10/project-media/media.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/round10/project-media/media.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round11/project-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round11/project-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round11/project-updates-reviewed.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/round11/project-updates-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round11/programming-languages-reviewed.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round11/programming-languages-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round11/media-reviewed.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/round11/media-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round11/game-facts-reviewed.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/goal-refresh/round11/game-facts-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round11/wikipedia-links-reviewed.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/round11/wikipedia-links-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round11/directory-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import-directory data/research/discovery-2026-10-16/goal-refresh/round11/directory-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/api-translation/project-additions.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round12/api-translation/project-additions.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/console-ports/project-additions.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round12/console-ports/project-additions.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/game-recomp/new-projects.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round12/game-recomp/new-projects.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/software-reconstruction/project-additions.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round12/software-reconstruction/project-additions.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/social-latest/project-additions-reviewed.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round12/social-latest/project-additions-reviewed.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/api-translation/programming-languages.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round12/api-translation/programming-languages.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/console-ports/programming-languages.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round12/console-ports/programming-languages.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/game-recomp/programming-languages.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round12/game-recomp/programming-languages.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/software-reconstruction/programming-languages.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round12/software-reconstruction/programming-languages.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/social-latest/programming-languages.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round12/social-latest/programming-languages.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/game-recomp/project-updates.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/round12/game-recomp/project-updates.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/console-ports/media.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/round12/console-ports/media.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/game-recomp/media-updates.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/round12/game-recomp/media-updates.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/software-reconstruction/media.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/round12/software-reconstruction/media.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/game-hydration/game-facts-a.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/goal-refresh/round12/game-hydration/game-facts-a.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/game-hydration/game-facts-b.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/goal-refresh/round12/game-hydration/game-facts-b.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/game-recomp/game-profiles.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/goal-refresh/round12/game-recomp/game-profiles.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/game-recomp/wikipedia-links.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/round12/game-recomp/wikipedia-links.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/social-latest/directory-additions.json ]]; then
  python3 scripts/catalog.py import-directory data/research/discovery-2026-10-16/goal-refresh/round12/social-latest/directory-additions.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/mobile-ports/new-projects.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round12/mobile-ports/new-projects.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/localization/project-additions.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round12/localization/project-additions.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/earthsiege-followup/project-additions.json ]]; then
  python3 scripts/catalog.py import data/research/discovery-2026-10-16/goal-refresh/round12/earthsiege-followup/project-additions.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/mobile-ports/programming-languages.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round12/mobile-ports/programming-languages.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/localization/programming-languages.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round12/localization/programming-languages.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/earthsiege-followup/programming-languages.json ]]; then
  python3 scripts/catalog.py import-programming-languages data/research/discovery-2026-10-16/goal-refresh/round12/earthsiege-followup/programming-languages.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/mobile-ports/project-updates.json ]]; then
  python3 scripts/catalog.py import-project-updates data/research/discovery-2026-10-16/goal-refresh/round12/mobile-ports/project-updates.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/mobile-ports/media.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/round12/mobile-ports/media.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/mobile-ports/existing-project-media.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/round12/mobile-ports/existing-project-media.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/localization/media.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/round12/localization/media.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/earthsiege-followup/media.json ]]; then
  python3 scripts/catalog.py import-media data/research/discovery-2026-10-16/goal-refresh/round12/earthsiege-followup/media.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/earthsiege-followup/game-profiles.json ]]; then
  python3 scripts/catalog.py import-game-profiles data/research/discovery-2026-10-16/goal-refresh/round12/earthsiege-followup/game-profiles.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/earthsiege-followup/wikipedia-links.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/round12/earthsiege-followup/wikipedia-links.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/wikipedia-links-gap-completion.json ]]; then
  python3 scripts/catalog.py import-wikipedia-links data/research/discovery-2026-10-16/goal-refresh/round12/wikipedia-links-gap-completion.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/localization/directory-additions.json ]]; then
  python3 scripts/catalog.py import-directory data/research/discovery-2026-10-16/goal-refresh/round12/localization/directory-additions.json
fi
if [[ -f data/research/discovery-2026-10-16/goal-refresh/round12/discovery-resources.json ]]; then
  python3 scripts/catalog.py import-directory data/research/discovery-2026-10-16/goal-refresh/round12/discovery-resources.json
fi
python3 scripts/catalog.py export
cargo build --locked --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir docs/pkg target/wasm32-unknown-unknown/release/recompiled.wasm
python3 - <<'PY'
import hashlib
import re
from pathlib import Path

index = Path("docs/index.html")
stylesheet = Path("docs/style.css")
digest = hashlib.sha256(stylesheet.read_bytes()).hexdigest()[:12]
html = index.read_text(encoding="utf-8")
pattern = re.compile(r'(<link rel="stylesheet" href=")\./style\.css(?:\?v=[^"]*)?(">)')
html, replacements = pattern.subn(
    lambda match: f'{match.group(1)}./style.css?v={digest}{match.group(2)}',
    html,
    count=1,
)
if replacements != 1:
    raise SystemExit("Could not find the stylesheet link to version in docs/index.html")
index.write_text(html, encoding="utf-8")
PY
