export PATH := fs/bin:$(PATH)

TROFF_PATH = $(shell realpath fs)

src-git/h-doctools/mk.config: .mk.config
	cp .mk.config src-git/h-doctools/mk.config

fs/bin/troff: src-git/h-doctools/mk.config
	echo "Building heirloom troff..."
	cd src-git/h-doctools; PREFIX=$TROFF_PATH ./configure  > /dev/null
	cd src-git/h-doctools; PREFIX=$TROFF_PATH make -j4     > /dev/null
	cd src-git/h-doctools; cp checknr/checknr \
		col/col \
		eqn/eqn \
		grap/grap \
		mpm/pm \
		pic/pic \
		pickpack/pickpack \
		refer/refer \
		soelim/soelim \
		tbl/tbl \
		troff/troff.d/troff \
		troff/troff.d/troff/ta \
		vgrind/vgrind \
		vgrind/vfontedpr \
		fs/bin

fs/bin/yacc: src-git/byacc/*.c
	echo "Building Berkeley yacc"
	cd src-git/byacc; ./configure --prefix="$TROFF_PATH" > /dev/null
	cd src-git/byacc; make -j4                                > /dev/null
	cd src-git/byacc; make install                            > /dev/null

fs/bin/watcher: watcher/src/**.rs
	cd watcher; cargo build --release
	mv watcher/target/release/watcher fs/bin/watcher

%.pdf: %.me fs/bin/watcher fs/bin/yacc fs/bin/troff
ifdef run
		fs/bin/run.sh %<
else
		fs/bin/update.sh $<
endif

ifdef run
	.PHONY: %.pdf
endif
