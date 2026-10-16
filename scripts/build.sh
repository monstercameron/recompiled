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
