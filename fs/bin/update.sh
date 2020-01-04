#!/bin/bash
echo updating...

# this is how you add a new reference database
#mkey refer_dbs/refer                 | inv refer_dbs/refer
#mkey refer_dbs/dark_souls_literatura | inv refer_dbs/dark_souls_literatura
#mkey refer_dbs/ai                    | inv refer_dbs/ai
#indxbib refer_dbs/refer
#indxbib refer_dbs/dark_souls_literatura
#indxbib refer_dbs/ai

export PATH=./fs/bin:./fs/ucb/:$PATH

if [[ -z "$TMAC" ]]; then
	export TMAC=me
fi

refer -v

cat $(basename -- "$1" .$TMAC).$TMAC \
	| pic \
	| eqn \
	| refer -p "refer" \
	| troff -dpaper=a4 "-$TMAC" - \
	| dpost \
	| ps2pdf - > $(basename -- "$1" .$TMAC).pdf


pids=$(pidof mupdf)

if [ "$pids" != "" ]; then
	echo $pids
	kill -s HUP $pids
else
	echo "mupdf is not running"
fi

