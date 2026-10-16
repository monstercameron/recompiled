# Round 13 media URI research notes

This review stages 25 direct media references for 25 cataloged projects that had no `project_media` rows at the time of research. Each media URI is linked by that project's maintainer README on the default or selected project branch. Relative README asset paths were resolved to their `raw.githubusercontent.com` URI; the source README is retained in `source_url` and the evidence line is listed in `provenance-checks.md`.

The batch contains 24 image resources (including animated GIFs and WebP files) and one direct YouTube watch URL. The `kind` values follow the actual linked resource. Image `preview_url` points to the same image URI. The YouTube preview was left empty because the maintainer README does not link a thumbnail URI.

All 25 media URLs returned HTTP 200 to a HEAD request on 2026-10-16. HEAD requests did not retrieve response bodies. GitHub reported `application/octet-stream` for one `.gif` (`wwe-smackdown-vs-raw-2010-xbox360-recomp`); its README links the `.gif` directly and the URL returned 200. The manifest was checked against the current SQLite catalog for project existence, empty media lists, HTTPS source URLs, and duplicate media URLs. No database import or asset download was performed.

To import later, from the catalog repository run:

```sh
python3 scripts/catalog.py import-media /Users/earlcameron/Desktop/recompiled/work/research-2026-10-16/continuation-round13/media-uris/media.json
```

The manifest is staged only; `data/catalog.sqlite` and tracked catalog files were not edited.
