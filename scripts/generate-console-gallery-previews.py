#!/usr/bin/env python3
"""Create small JPEG display copies while preserving cached Commons originals."""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_MANIFEST = ROOT / "data/research/console-gallery.json"
DEFAULT_EDGE = 760
DEFAULT_QUALITY = 74


def preview_path(asset_path: str) -> Path:
    prefix = "assets/consoles/"
    if not asset_path.startswith(prefix) or ".." in Path(asset_path).parts:
        raise ValueError(f"Unsafe console-gallery asset path: {asset_path!r}")
    relative = Path(asset_path[len(prefix) :])
    if not relative.suffix or relative.suffix.lower() not in {".jpg", ".jpeg", ".png"}:
        raise ValueError(f"Unsupported console-gallery image type: {asset_path!r}")
    return ROOT / "docs/assets/console-previews" / relative.with_suffix(".jpg")


def create_preview(source: Path, destination: Path, edge: int, quality: int) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    temporary = destination.with_name(f".{destination.stem}.tmp.jpg")
    sips = shutil.which("sips")
    magick = shutil.which("magick")
    convert = shutil.which("convert")

    if sips:
        command = [
            sips,
            "-s",
            "format",
            "jpeg",
            "-s",
            "formatOptions",
            str(quality),
            "-Z",
            str(edge),
            str(source),
            "--out",
            str(temporary),
        ]
    elif magick or convert:
        command = [
            magick or convert,
            str(source),
            "-auto-orient",
            "-resize",
            f"{edge}x{edge}>",
            "-strip",
            "-quality",
            str(quality),
            f"jpeg:{temporary}",
        ]
    else:
        raise RuntimeError("Install ImageMagick or run this script on macOS, which provides sips.")

    try:
        subprocess.run(command, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
        if not temporary.is_file() or temporary.stat().st_size == 0:
            raise RuntimeError(f"Preview tool produced no output for {source.relative_to(ROOT)}")
        temporary.replace(destination)
    finally:
        temporary.unlink(missing_ok=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--max-edge", type=int, default=DEFAULT_EDGE)
    parser.add_argument("--quality", type=int, default=DEFAULT_QUALITY)
    parser.add_argument("--check", action="store_true", help="verify all previews exist without generating them")
    args = parser.parse_args()

    if args.max_edge < 320 or args.max_edge > 2400:
        parser.error("--max-edge must be between 320 and 2400 pixels")
    if args.quality < 40 or args.quality > 95:
        parser.error("--quality must be between 40 and 95")

    manifest_path = args.manifest if args.manifest.is_absolute() else ROOT / args.manifest
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    items = manifest.get("items")
    if not isinstance(items, list) or not items:
        raise ValueError("Console gallery manifest has no items")

    rows: dict[Path, Path] = {}
    for item in items:
        asset_path = item.get("asset_path", "")
        destination = preview_path(asset_path)
        source = ROOT / "docs" / asset_path
        if not source.is_file():
            raise FileNotFoundError(f"Cached original is missing: {source}")
        previous = rows.get(destination)
        if previous and previous != source:
            raise ValueError(f"Two gallery assets map to the same preview: {previous} and {source}")
        rows[destination] = source

    missing = [destination for destination in rows if not destination.is_file()]
    if args.check:
        if missing:
            print(f"Missing {len(missing)} of {len(rows)} console-gallery previews.", file=sys.stderr)
            print("Run: python3 scripts/generate-console-gallery-previews.py", file=sys.stderr)
            return 1
        print(f"Verified {len(rows)} console-gallery previews.")
        return 0

    before = sum(source.stat().st_size for source in rows.values())
    for destination, source in rows.items():
        create_preview(source, destination, args.max_edge, args.quality)
    after = sum(destination.stat().st_size for destination in rows)
    print(
        f"Generated {len(rows)} JPEG previews ({args.max_edge}px max edge, quality {args.quality}): "
        f"{before / 1024 / 1024:.1f} MiB originals → {after / 1024 / 1024:.1f} MiB previews."
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, RuntimeError, json.JSONDecodeError) as error:
        print(f"Error: {error}", file=sys.stderr)
        raise SystemExit(1) from error
