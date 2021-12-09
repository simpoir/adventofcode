#!/bin/sh

YEAR=2021

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
DATADIR="$HERE/data/day$DAY"
mkdir -p $DATADIR
touch $DATADIR/expected
touch $DATADIR/sample
[ -f "$DATADIR/input" ] || curl -H "cookie: $COOKIE" https://adventofcode.com/$YEAR/day/$DAY/input -o $DATADIR/input
tmpout=$(mktemp)
curl -H "cookie: $COOKIE" https://adventofcode.com/$YEAR/day/$DAY -o $tmpout
# We're totally guessing at the format here.
xmllint --html --xpath '(//pre/code)[1]' $tmpout 2>/dev/null | sed -z -e 's:</*\w*>::g' -e 's/&gt;/>/g' -e 's/\n$//' > $DATADIR/sample
xmllint --html --xpath '(//article[1]//code/em|//article[1]//em/code)[last()]/text()
	|(//article[2]//code/em|//article[2]//em/code)[last()]/text()' \
		$tmpout 2>/dev/null | sed -z -e 's/\n$//' > $DATADIR/expected
rm $tmpout
