#!/bin/bash
echo updating...

#set -xg PATH /usr/heirloom/bin/ucb/ $PATH

indxbib refer_dbs/refer
indxbib refer_dbs/dark_souls_literatura
indxbib refer_dbs/ai

export PATH=./troff/bin

env PATH="/usr/heirloom/bin/ucb:$PATH" cat $(basename -- "$1" .me).me \
	| env PATH="/usr/heirloom/bin/ucb:$PATH" refer -p "refer_dbs/ai" -p "refer_dbs/refer" -p "refer_dbs/dark_souls_literatura" \
	| env PATH="/usr/heirloom/bin/ucb:$PATH" pic \
	| env PATH="/usr/heirloom/bin/ucb:$PATH" eqn \
	| env PATH="/usr/heirloom/bin/ucb:$PATH" troff -dpaper=a4 -me - \
	| ps2pdf - > $(basename -- "$1" .me).pdf

sh -c "kill -s SIGHUP mupdf; exit 2>&1 > /dev/null"

