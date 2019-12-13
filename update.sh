#!/bin/bash
echo updating...
indxbib refer_dbs/refer
indxbib refer_dbs/dark_souls_literatura
indxbib refer_dbs/ai

#cat $1.me \
ascii_czech/target/release/ascii_czech $1.me \
	| refer -p "refer_dbs/ai" -p "refer_dbs/refer" -p "refer_dbs/dark_souls_literatura" \
	| ascii_czech/target/release/ascii_czech - \
	| pic \
	| eqn \
	| groff -dpaper=a4 -P-pa4 -me - \
	| ps2pdf - > $1.pdf

fish --command "kill -s SIGHUP mupdf; exit"

