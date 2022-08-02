#!/bin/sh

HERE="$(dirname $0)"
last="$(ls -1 "$HERE/src/days" | sort -r | head -n1)"
next="$(( 1 + $(echo "${last#day}" | cut -d. -f1) ))"
cp "$HERE/day.rs" "$HERE/src/days/day$next.rs"
