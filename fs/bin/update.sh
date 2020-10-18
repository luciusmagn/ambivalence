#!/bin/bash -x
echo updating...
echo started
# this is how you add a new reference database
#mkey refer_dbs/refer                 | inv refer_dbs/refer
#mkey refer_dbs/dark_souls_literatura | inv refer_dbs/dark_souls_literatura
#mkey refer_dbs/ai                    | inv refer_dbs/ai

#mkey fs/share/refer/Ind | fs/ucblib/reftools/inv fs/share/refer/papers/Ind

mkdir _refer_dbs
mkdir .refer_dbs

/usr/bin/indxbib refer_dbs/ai
/usr/bin/indxbib refer_dbs/refer
/usr/bin/indxbib refer_dbs/niggers
/usr/bin/indxbib refer_dbs/dark_souls_literatura

rm -rf _refer_dbs

if [[ -z "$TMAC" ]]; then
	export TMAC=me
fi

convert_images.sh

fd . refer_dbs/ -x sh -c "cat {} | ascii_czech fuck - > .refer_dbs/{/}"

which chem

soelim $(basename -- "$1" .$TMAC).$TMAC | chem | tee $1.chem

cat $1.chem 2>1 \
	| pic \
	| tbl \
	| eqn \
	| ascii_czech fuck - \
	| /usr/bin/refer \
		-p ".refer_dbs/psychologie" \
		-p ".refer_dbs/niggers" \
		-p ".refer_dbs/refer" \
		-p ".refer_dbs/ai" \
		-p ".refer_dbs/dark_souls_literatura" \
		-p ".refer_dbs/jicha" \
	| ascii_czech unfuck - \
	| if [[ "$DEBUG" = "true" ]]; then tee /dev/stderr; else cat - ; fi \
	| troff \
		-mg \
		-mpictures \
		-dpaper=a4 \
		"-$TMAC" - \
	| dpost \
	| ps2pdf -dPDFA=2 - > $(basename -- "$1" .$TMAC).pdf

rm -rf $1.chem

#	| tee /dev/fd/2 \
#	or
#	| tee /dev/stderr \
# ^ debug pipeline element, insert at random places to inspect stream

pids=$(pidof mupdf)

if [ "$pids" != "" ]; then
	echo $pids
	kill -s HUP $pids
else
	echo "mupdf is not running"
fi
echo end
