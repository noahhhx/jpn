---
name: sync-itinerary
description: Update the site from the latest version of the Japan 2026 itinerary Word doc. Use when the user says the doc ("Japan 2026.docx") has changed and the site should reflect it.
---

# Sync the site with the itinerary doc

The site's content is hand-written in `src/itinerary.rs` from the trip doc `Japan 2026.docx`. The doc sits untracked in the repo root. The site isn't a transcript of the doc: it rewrites it into tidy blocks (`place`, `Schedule`, `List`, `Note`, `Links`, `image`, see `src/trip.rs`) in British English. So syncing means finding what changed in the doc and editing the matching blocks by hand.

## 1. Extract the doc

```sh
S=<scratchpad dir>
.claude/skills/sync-itinerary/extract.sh "Japan 2026.docx" "$S/doc"
```

This runs pandoc and imagemagick through `nix shell`, since neither is installed. The first run downloads them.

## 2. Find what changed

The baseline is `.itinerary-sync/baseline.txt`, the doc as of the last sync. It's gitignored, so it only exists on the machine that last ran this skill.

- **Baseline exists:** run `diff .itinerary-sync/baseline.txt "$S/doc/doc.txt"`. Every hunk is a real edit. Read the matching part of `$S/doc/doc.md` for links and formatting.
- **No baseline:** read `$S/doc/doc.md` in full and compare it day by day against `src/itinerary.rs`. Expect the wording to differ everywhere. Only report facts that differ or are missing: times, places, bookings, codes, items, links.

## 3. Find new images

`$S/doc/images.txt` lists each doc image and any site image with the same pixel size.

- **A size match:** almost always the same image already on the site.
- **`NO SIZE MATCH`:** view the image with Read. It might be new, a re-cropped version of a site image, or something the site turned into text on purpose.

The site's image conventions:

- **Images it keeps:** maps, QR codes, train and ticket screenshots, timetables and price tables.
- **Turned into text instead:** booking confirmations (hotel, parking, flights, insurance) and personal app screenshots (e.g. Mytrip). The dates, times and details go into a `place` note or a `Schedule`.
- **New images:** convert to `.webp` in `public/img/` with a descriptive kebab-case name. Use lossless for QR codes:
  ```sh
  nix shell nixpkgs#imagemagick -c magick in.png -define webp:lossless=true public/img/name-qr.webp
  nix shell nixpkgs#imagemagick -c magick in.png -quality 90 public/img/name.webp
  ```
- **Updated screenshots:** if the doc has a newer version of an image already on the site (e.g. a booking now confirmed), overwrite the existing file and keep its name.
- **Captions:** state the key facts, e.g. `"Yufuin no Mori 5 · Hakata 14:38 → Yufuin 16:50"`.

## 4. Edit `src/itinerary.rs`

- **Style:** match the surrounding blocks. Keep the site's tone (short, edited sentences, `—` dashes, `¥`/`£`), and don't paste the doc's wording verbatim.
- **Links:** skip email tracking redirects (e.g. `click.sfmail...`). A `place` already links to Google Maps from its address, so prefer one of those.
- **Moved text:** when something moves between days in the doc, move it on the site too.
- **Days and dates:** come from `START` and the order of `DAYS`. Only touch them if the trip dates change.

## 5. Verify and finish

1. Run `cargo test --features ssr`.
2. Save the new baseline: `mkdir -p .itinerary-sync && cp "$S/doc/doc.txt" .itinerary-sync/baseline.txt`.
3. Report to the user:
   - every change, grouped by day
   - judgement calls they should check, such as facts inferred from a screenshot
   - anything in the doc you deliberately left off the site
4. Only commit if they ask.
