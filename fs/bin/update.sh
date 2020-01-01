#!/bin/bash
echo updating...

#indxbib refer_dbs/refer
#indxbib refer_dbs/dark_souls_literatura
#indxbib refer_dbs/ai

export PATH=./fs/bin:./fs/ucb/:$PATH

if ! [[ -v $TMAC ]]; then
	export TMAC=me
fi

#	| refer -p "refer_dbs/ai" -p "refer_dbs/refer" -p "refer_dbs/dark_souls_literatura" \
cat $(basename -- "$1" .$TMAC).$TMAC \
	| pic \
	| eqn \
	| troff -dpaper=a4 "-$TMAC" - \
	| dpost \
	| ps2pdf - > $(basename -- "$1" .$TMAC).pdf

sh -c "kill -s HUP $(pidof mupdf); exit 2>&1 > /dev/null"

