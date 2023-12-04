#!/bin/sh

HERE="$(dirname "$0")"
last="$(ls -1 "$HERE/src/days" | cut -c4- | sort -rn | head -n1)"
test -z "$last" && last=0
next="$(( 1 + $(echo "${last#day}" | cut -d. -f1) ))"
cp "$HERE/day.rs" "$HERE/src/days/day$next.rs"
