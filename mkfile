PATH = fs/bin:fs/ucb/:fs/ucblib/reftools/:$PATH
<|echo -n "TROFF_PATH = $(realpath fs)"

tmac=me

TMAC=$tmac
TARGET_BASE=`basename $doc .pdf`

`test -n "$run" -o -n "$phony" && echo "$TARGET_BASE.pdf:V" || echo "$TARGET_BASE"`: $TARGET_BASE.$tmac fs/bin/wendy fs/bin/yacc fs/ucb/troff
	[[ -n "$run" ]] && wendy -m 8 -f $TARGET_BASE.$tmac -e update.sh $TARGET_BASE || update.sh $TARGET_BASE

fs/ucblib/doctools/tmac/om.tmac: fs/ucb/troff
	cp src-git/mom-2.4-4/om.tmac fs/ucblib/doctools/tmac/

fs/bin/wendy: `find src-git/wendy/ -iname '*.c' -o -iname '*.h'`
	echo "Building wendy"
	cd src-git/wendy/
	make
	cp wendy ../../fs/bin/wendy

fs/ucb/troff: `find src-git/h-doctools/ -iname '*.c' -o -iname '*.h'`
	echo "Building heirloom troff..."
	cd src-git/h-doctools;
	./configure
	make -j4 PREFIX=$TROFF_PATH
	make install PREFIX=$TROFF_PATH
	cd ../..
	fish --command 'cp -r fonts/base35/**.{afm,pfb} fs/ucblib/doctools/font/devps/'

fs/bin/yacc: `find src-git/byacc -iname '*.c' -o -iname '*.h'`
	echo "Building Berkeley yacc"
	cd src-git/byacc
	./configure --prefix="${TROFF_PATH}"
	make -j4
	make install


# rip lol
`[[ -n "$run" ]] && echo ".PHONY: $TARGET_BASE.pdf" || true`

.DEFAULT_GOAL: $target
