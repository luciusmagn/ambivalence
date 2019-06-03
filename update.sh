#!/bin/bash
echo updating...
indxbib refer
indxbib dark_souls_literatura
ascii_czech/target/release/ascii_czech $1.me | refer -prefer |  ascii_czech/target/release/ascii_czech - | pic | eqn | pdfroff -dpaper=a4 -P-pa4 -me - > $1.pdf; fish --command "kill -s SIGHUP mupdf; exit"
