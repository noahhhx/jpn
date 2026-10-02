#!/usr/bin/env bash
# Extract the itinerary doc into diffable text plus its images.
#
# Usage: extract.sh <docx> <out-dir>
#
# Writes:
#   <out-dir>/doc.md      markdown with <img> tags pointing at the extracted images
#   <out-dir>/doc.txt     normalised plain text, for diffing against the baseline
#   <out-dir>/media/      every image in the doc, as imageN.png
#   <out-dir>/images.txt  each doc image's size and the site image with the same size, if any
set -euo pipefail

docx=$(realpath "$1")
out=$(realpath -m "$2")
repo=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
mkdir -p "$out"

nix shell nixpkgs#pandoc nixpkgs#imagemagick -c bash -s "$docx" "$out" "$repo" <<'EOF'
set -euo pipefail
docx=$1 out=$2 repo=$3

pandoc "$docx" -t gfm --wrap=none --extract-media="$out" -o "$out/doc.md"

# Plain text with formatting noise removed, so a diff only shows real edits:
# image markers, superscript ordinals, curly quotes, indentation and blank lines.
pandoc "$docx" -t plain --wrap=none |
  sed -e 's/\[\]//g' -e 's/\^(\([a-z]*\))/\1/g' \
      -e "s/[‘’]/'/g" -e 's/[“”]/"/g' \
      -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' |
  grep -v '^$' > "$out/doc.txt"

declare -A site
while read -r name size; do
  site[$size]+="$name "
done < <(identify -format '%f %wx%h\n' "$repo"/public/img/*.webp 2>/dev/null)

: > "$out/images.txt"
# Image names depend on what exported the doc (image12.png, rId34.jpeg, ...).
for img in $(grep -o "$out/media/[^\")]*" "$out/doc.md" | awk '!seen[$0]++'); do
  size=$(identify -format '%wx%h' "$img")
  echo "$(basename "$img") $size -> ${site[$size]:-NO SIZE MATCH}" >> "$out/images.txt"
done
EOF
