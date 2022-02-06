#!/bin/sh

YEAR=2016

if [ $# -le 0 ]
then 
	echo usage: $0 session=12345 [day]
	echo Where session is the session cookie from your browser.
	echo and where [day] is an optional day.
	exit 1
fi

COOKIE="$1"
if [ $# -gt 1 ]
then
	DAY=$2
else
	DAY="$(date +%1d)"
fi

HERE="$(dirname $(readlink -f $0))"
DATADIR="$HERE/data"
mkdir -p $DATADIR
[ -f "$DATADIR/day$DAY.txt" ] || curl -H "cookie: $COOKIE" https://adventofcode.com/$YEAR/day/$DAY/input -o "$DATADIR/day$DAY.txt"
