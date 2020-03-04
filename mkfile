PATH = fs/bin:fs/share/refer/:fs/ucb/:fs/ucblib/reftools/:$PATH
<|echo -n "TROFF_PATH = $(realpath fs)"

tmac=me

TMAC=$tmac
TARGET_BASE=`basename $doc .pdf`

`test -n "$run" -o -n "$phony" && echo "$TARGET_BASE.pdf:V" || echo "$TARGET_BASE"`: $TARGET_BASE.$tmac \
  fs/bin/wendy \
  fs/bin/yacc \
  fs/ucb/troff \
  fs/bin/refer \
  fs/bin/ascii_czech \
  fs/ucblib/doctools/hyphen/hyph_cs_CZ.dic \
  fs/ucblib/doctools/tmac/om
	echo $tmac $TARGET_BASE
	[[ -n "$run" ]] \
	&& wendy -m 264 \
		-f img/src \
		-e convert_images.sh \
	|| convert_images.sh &
	[[ -n "$run" ]] \
	&& wendy -m 264 \
		-f $TARGET_BASE.$tmac \
		-f tmac \
		-f fs/bin/update.sh \
		-f refer_dbs/refer \
		-f refer_dbs/ai \
		-f refer_dbs/dark_souls_literatura \
		-e update.sh $TARGET_BASE \
	|| update.sh $TARGET_BASE

fs/ucblib/doctools/tmac/om: fs/ucb/troff
	cp src-git/mom-2.4-4/om.tmac fs/ucblib/doctools/tmac/om

fs/ucblib/doctools/hyphen/hyph_cs_CZ.dic: fs/ucb/troff
	cp src-git/hyph_cs_CZ.dic fs/ucblib/doctools/hyphen/hyph_cs_CZ.dic

fs/bin/ascii_czech: fs/ucblib/doctools/hyphen/hyph_cs_CZ.dic `find src-git/ascii_czech/ -iname '*.rs'`
	cd src-git/ascii_czech/
	cargo build --release
	cp target/release/ascii_czech $TROFF_PATH/bin/

fs/bin/refer: `find src-git/utroff-refer/ -iname '*.c' -o -iname '*.h'` fs/ucb/troff
	echo "Building utroff refer"
	cd src-git/utroff-refer
	make -j4 PREFIX=$TROFF_PATH
	make install PREFIX=$TROFF_PATH
	cd $TROFF_PATH
	cp -r ucblib/reftools/papers share/refer/

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

