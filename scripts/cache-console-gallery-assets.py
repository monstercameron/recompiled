#!/usr/bin/env python3
"""Cache verified Wikimedia Commons console-gallery images locally.

The reviewed manifest remains the source of attribution. Successful downloads
add SHA-256 and MIME metadata to their manifest rows so later runs can resume
without replacing files that this script cannot identify as its own.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import re
import sys
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request
import xml.etree.ElementTree as ET


ROOT = pathlib.Path(__file__).resolve().parents[1]
DEFAULT_MANIFEST = ROOT / "data" / "research" / "console-gallery.json"
STATE_PATH = ROOT / "data" / "research" / "console-gallery-cache-state.json"
USER_AGENT = "RecompiledCatalogConsoleGallery/1.0 (+https://github.com/monstercameron/recompiled)"
MAX_IMAGE_BYTES = 30 * 1024 * 1024
TIMEOUT_SECONDS = 40

ALLOWED_DIRS = ("assets/consoles/", "assets/maker-logos/", "assets/source-logos/")
ALLOWED_FORMATS = {
    ".jpg": "jpeg",
    ".jpeg": "jpeg",
    ".png": "png",
    ".webp": "webp",
    ".svg": "svg+xml",
}
MIME_FOR_FORMAT = {
    "jpeg": "image/jpeg",
    "png": "image/png",
    "webp": "image/webp",
    "svg+xml": "image/svg+xml",
}


class CacheError(Exception):
    """An unsafe or invalid cache operation."""


def atomic_json(path: pathlib.Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temp_name = tempfile.mkstemp(prefix=f".{path.name}.", suffix=".tmp", dir=path.parent)
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as stream:
            json.dump(value, stream, ensure_ascii=False, indent=2)
            stream.write("\n")
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temp_name, path)
    except Exception:
        try:
            os.unlink(temp_name)
        except FileNotFoundError:
            pass
        raise


def safe_asset_path(value: str, allowed_prefixes: tuple[str, ...]) -> pathlib.Path:
    if not isinstance(value, str) or not value.startswith("assets/"):
        raise CacheError(f"Asset path must start inside docs/assets/: {value!r}")
    relative = pathlib.PurePosixPath(value)
    if relative.is_absolute() or ".." in relative.parts or "\\" in value:
        raise CacheError(f"Unsafe asset path: {value!r}")
    normalized = relative.as_posix()
    if not any(normalized.startswith(prefix) for prefix in allowed_prefixes):
        raise CacheError(f"Asset path is outside its allow-listed directory: {value}")
    suffix = pathlib.PurePosixPath(normalized).suffix.lower()
    if suffix not in ALLOWED_FORMATS:
        raise CacheError(f"Unsupported image extension in asset path: {value}")
    target = (ROOT / "docs" / normalized).resolve()
    try:
        target.relative_to((ROOT / "docs" / "assets").resolve())
    except ValueError as error:
        raise CacheError(f"Asset path escapes docs/assets/: {value}") from error
    return target


def validate_commons_urls(image_url: str, page_url: str, label: str) -> None:
    image = urllib.parse.urlsplit(image_url)
    page = urllib.parse.urlsplit(page_url)
    if image.scheme != "https" or image.hostname != "upload.wikimedia.org" or image.username or image.password:
        raise CacheError(f"{label}: image URL must use HTTPS on upload.wikimedia.org")
    if not image.path.startswith("/wikipedia/commons/"):
        raise CacheError(f"{label}: image URL is outside Wikimedia Commons uploads")
    if image.port not in (None, 443):
        raise CacheError(f"{label}: non-standard upload URL port")
    if page.scheme != "https" or page.hostname != "commons.wikimedia.org" or page.username or page.password:
        raise CacheError(f"{label}: provenance must be a Wikimedia Commons HTTPS page")
    if not page.path.lower().startswith("/wiki/file:"):
        raise CacheError(f"{label}: provenance URL must be a Commons File page")
    if page.port not in (None, 443):
        raise CacheError(f"{label}: non-standard Commons page port")


def sniff_image(path: pathlib.Path, declared_mime: str) -> str:
    size = path.stat().st_size
    if size <= 0 or size > MAX_IMAGE_BYTES:
        raise CacheError(f"Image has an invalid size ({size} bytes); maximum is {MAX_IMAGE_BYTES}")
    with path.open("rb") as stream:
        header = stream.read(32)
        stream.seek(0)
        if header.startswith(b"\xff\xd8\xff"):
            fmt = "jpeg"
        elif header.startswith(b"\x89PNG\r\n\x1a\n"):
            fmt = "png"
        elif len(header) >= 12 and header[:4] == b"RIFF" and header[8:12] == b"WEBP":
            fmt = "webp"
        else:
            prefix = stream.read()
            if b"<!DOCTYPE" in prefix.upper():
                raise CacheError("SVG with a document type declaration is not accepted")
            try:
                root = ET.fromstring(prefix)
            except ET.ParseError:
                root = None
            if root is None or root.tag.split("}")[-1].lower() != "svg":
                raise CacheError("Downloaded content is not a supported image signature")
            validate_svg_tree(root)
            fmt = "svg+xml"

    declared = declared_mime.split(";", 1)[0].strip().lower()
    if declared and declared != "application/octet-stream" and declared != MIME_FOR_FORMAT[fmt]:
        raise CacheError(f"MIME mismatch: server declared {declared}, content is {MIME_FOR_FORMAT[fmt]}")
    return fmt


def validate_svg_tree(root: ET.Element) -> None:
    """Reject active SVG features and external references before publishing."""
    forbidden = {"script", "foreignobject", "iframe", "object", "embed"}
    for element in root.iter():
        tag = element.tag.split("}")[-1].lower()
        if tag in forbidden:
            raise CacheError(f"Unsafe SVG element: {tag}")
        for name, value in element.attrib.items():
            local_name = name.split("}")[-1].lower()
            lowered = value.strip().lower()
            if local_name.startswith("on"):
                raise CacheError("SVG event handlers are not accepted")
            if local_name in {"href", "src"} and lowered and not lowered.startswith("#"):
                raise CacheError("SVG external or embedded references are not accepted")
            if "url(" in lowered:
                for match in re.finditer(r"url\(\s*([^)]*)\)", lowered):
                    reference = match.group(1).strip(" '\"\t\r\n")
                    if not reference.startswith("#"):
                        raise CacheError("SVG styles may not reference external content")
        text = (element.text or "").lower()
        if "@import" in text:
            raise CacheError("SVG styles may not import external content")
        if "url(" in text:
            for match in re.finditer(r"url\(\s*([^)]*)\)", text):
                reference = match.group(1).strip(" '\"\t\r\n")
                if not reference.startswith("#"):
                    raise CacheError("SVG styles may not reference external content")


def file_sha256(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


class CommonsRedirectHandler(urllib.request.HTTPRedirectHandler):
    """Allow redirects only while the request remains on Commons uploads."""

    def redirect_request(self, request, response, code, message, headers, new_url):
        destination = urllib.parse.urlsplit(new_url)
        if destination.scheme != "https" or destination.hostname != "upload.wikimedia.org":
            raise CacheError("Wikimedia redirected the image request outside upload.wikimedia.org")
        return super().redirect_request(request, response, code, message, headers, new_url)


def download(url: str, target: pathlib.Path) -> tuple[str, str, str]:
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT, "Accept": "image/avif,image/webp,image/png,image/jpeg,image/svg+xml,*/*;q=0.5"})
    try:
        opener = urllib.request.build_opener(CommonsRedirectHandler)
        response = opener.open(request, timeout=TIMEOUT_SECONDS)
    except urllib.error.HTTPError as error:
        if error.code == 429 or error.code == 503:
            retry_after = error.headers.get("Retry-After", "not provided")
            raise CacheError(f"Rate limited by Wikimedia (HTTP {error.code}; Retry-After: {retry_after}); stopping without retry") from error
        raise CacheError(f"Image request failed with HTTP {error.code}") from error
    except urllib.error.URLError as error:
        raise CacheError(f"Image request failed: {error.reason}") from error

    with response:
        final = urllib.parse.urlsplit(response.geturl())
        if final.scheme != "https" or final.hostname != "upload.wikimedia.org":
            raise CacheError("Wikimedia redirected the image request outside upload.wikimedia.org")
        status = getattr(response, "status", 200)
        if status == 429 or status == 503:
            retry_after = response.headers.get("Retry-After", "not provided")
            raise CacheError(f"Rate limited by Wikimedia (HTTP {status}; Retry-After: {retry_after}); stopping without retry")
        declared = response.headers.get("Content-Type", "").strip()
        mime = declared.split(";", 1)[0].strip().lower()
        if mime and not (mime.startswith("image/") or mime == "application/octet-stream"):
            raise CacheError(f"Server returned non-image MIME type: {mime}")
        temp_fd, temp_name = tempfile.mkstemp(prefix=".commons-download-", dir=target.parent)
        temp = pathlib.Path(temp_name)
        try:
            total = 0
            with os.fdopen(temp_fd, "wb") as output:
                while True:
                    chunk = response.read(128 * 1024)
                    if not chunk:
                        break
                    total += len(chunk)
                    if total > MAX_IMAGE_BYTES:
                        raise CacheError(f"Image exceeds the {MAX_IMAGE_BYTES} byte size limit")
                    output.write(chunk)
                output.flush()
                os.fsync(output.fileno())
            fmt = sniff_image(temp, mime)
            expected = ALLOWED_FORMATS[target.suffix.lower()]
            if fmt != expected:
                raise CacheError(f"Image content is {fmt}, but asset path ends in {target.suffix}")
            return str(temp), fmt, file_sha256(temp)
        except Exception:
            try:
                temp.unlink()
            except FileNotFoundError:
                pass
            raise


def records(manifest: dict) -> list[tuple[dict, tuple[str, ...], tuple[str, ...], tuple[str, ...]]]:
    result = []
    for collection in ("makers", "source_logos", "items"):
        value = manifest.get(collection)
        if not isinstance(value, list) or not value:
            raise CacheError(f"Manifest needs a non-empty {collection} array")
    for row in manifest["makers"]:
        result.append((row, ("file_url", "logo_image_url"), ("file_page", "logo_page_url"), ("assets/maker-logos/",)))
    for row in manifest["source_logos"]:
        result.append((row, ("file_url", "logo_image_url"), ("file_page", "logo_page_url"), ("assets/source-logos/",)))
    for row in manifest["items"]:
        result.append((row, ("source_image_url",), ("source_page_url",), ("assets/consoles/",)))
    return result


def load_json(path: pathlib.Path, default: object) -> object:
    if not path.exists():
        return default
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise CacheError(f"Cannot read JSON file {path}: {error}") from error


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=pathlib.Path, default=DEFAULT_MANIFEST, help="Reviewed gallery manifest (default: data/research/console-gallery.json)")
    parser.add_argument("--dry-run", action="store_true", help="Validate entries and show actions without network or file writes")
    args = parser.parse_args()
    manifest_path = args.manifest.resolve()
    try:
        manifest = load_json(manifest_path, None)
        if not isinstance(manifest, dict):
            raise CacheError(f"Manifest must be a JSON object: {manifest_path}")
        rows = records(manifest)
        if not rows:
            raise CacheError("Manifest has no makers, source_logos, or items to cache")
        state = load_json(STATE_PATH, {})
        if not isinstance(state, dict):
            raise CacheError(f"Cache state must be a JSON object: {STATE_PATH}")

        targets: set[pathlib.Path] = set()
        actions = []
        resolved_rows = []
        for row, image_fields, page_fields, prefixes in rows:
            if not isinstance(row, dict):
                raise CacheError("Each gallery entry must be an object")
            label = row.get("id", "<unknown>")
            image_field = next((field for field in image_fields if row.get(field)), image_fields[0])
            page_field = next((field for field in page_fields if row.get(field)), page_fields[0])
            asset_field = "logo_asset_path" if image_fields[0] == "file_url" else "asset_path"
            image_url, page_url, asset_path = row.get(image_field), row.get(page_field), row.get(asset_field)
            if not all(isinstance(value, str) and value.strip() for value in (image_url, page_url, asset_path)):
                raise CacheError(f"{label}: missing image URL, Commons file page, or asset path")
            validate_commons_urls(image_url, page_url, label)
            target = safe_asset_path(asset_path, prefixes)
            if target in targets:
                raise CacheError(f"Multiple manifest entries target the same local asset: {asset_path}")
            targets.add(target)
            resolved_rows.append((row, image_field, asset_field, prefixes))
            existing_digest = row.get("cache_sha256")
            state_key = asset_path
            pending_digest = state.get(state_key)
            if target.exists():
                actual_digest = file_sha256(target)
                known_digest = existing_digest or pending_digest
                if not known_digest or actual_digest != known_digest:
                    raise CacheError(f"Refusing to replace or adopt an unrecognized existing file: {asset_path}")
                if existing_digest != actual_digest:
                    row["cache_sha256"] = actual_digest
                    row.setdefault("cache_mime", MIME_FOR_FORMAT[ALLOWED_FORMATS[target.suffix.lower()]])
                    row.setdefault("cache_transformation", "None; source image bytes cached unchanged.")
                    if not args.dry_run:
                        atomic_json(manifest_path, manifest)
                        state[state_key] = actual_digest
                        atomic_json(STATE_PATH, state)
                actions.append((label, asset_path, "already cached; verified SHA-256"))
            else:
                actions.append((label, asset_path, "would download original" if args.dry_run else "download"))

        for label, asset_path, action in actions:
            print(f"{action}: {label} -> {asset_path}")
        if args.dry_run:
            return 0

        # Download one at a time. Any 429/503 ends the run immediately; already
        # completed files stay recorded so a later invocation can resume.
        for row, image_field, asset_field, prefixes in resolved_rows:
            asset_path = row.get(asset_field)
            target = safe_asset_path(asset_path, prefixes)
            if target.exists():
                continue
            target.parent.mkdir(parents=True, exist_ok=True)
            label = row.get("id", "<unknown>")
            temp_name = None
            try:
                temp_name, fmt, digest = download(row[image_field], target)
                # Journal the digest before installing so a crash between the
                # exclusive file creation and manifest update remains resumable.
                state[asset_path] = digest
                atomic_json(STATE_PATH, state)
                temp = pathlib.Path(temp_name)
                try:
                    os.link(temp, target)  # Atomic create; fails if any file exists.
                except FileExistsError as error:
                    raise CacheError(f"File appeared during download; refusing to overwrite: {asset_path}") from error
                temp.unlink()
                temp_name = None
                row["cache_sha256"] = digest
                row["cache_mime"] = MIME_FOR_FORMAT[fmt]
                row["cache_transformation"] = "None; source image bytes cached unchanged."
                atomic_json(manifest_path, manifest)
                print(f"cached: {label} -> {asset_path} ({fmt}, SHA-256 {digest})")
                time.sleep(3.0)
            finally:
                if temp_name:
                    try:
                        pathlib.Path(temp_name).unlink()
                    except FileNotFoundError:
                        pass
        return 0
    except (CacheError, OSError, ValueError, KeyError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
