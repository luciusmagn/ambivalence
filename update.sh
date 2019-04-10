#!/bin/bash
echo updating...
indxbib refer
ascii_czech/target/release/ascii_czech $1.me | pic | eqn | refer -prefer | pdfroff -dpaper=a4 -P-pa4 -me - > $1.pdf; fish --command "kill -s SIGHUP mupdf; exit"
