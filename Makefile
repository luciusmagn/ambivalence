export PATH := fs/bin:fs/ucb/:$(PATH)

TROFF_PATH = $(shell realpath fs)

ifndef tmac
tmac = me
endif

fs/ucb/troff: src-git/h-doctools/cfg.mk
	echo "Building heirloom troff..."
	echo ${TROFF_PATH}
	cd src-git/h-doctools; ./configure  > /dev/null
	cd src-git/h-doctools; make -j4 PREFIX=${TROFF_PATH}     > /dev/null
	cd src-git/h-doctools; make install PREFIX=${TROFF_PATH}     > /dev/null
	fish --command 'cp -r fonts/base35/**.{afm,pfb} fs/ucblib/doctools/font/devps/'

fs/bin/yacc: src-git/byacc/*.c
	echo "Building Berkeley yacc"
	cd src-git/byacc; ./configure --prefix="${TROFF_PATH}" > /dev/null
	cd src-git/byacc; make -j4                                > /dev/null
	cd src-git/byacc; make install                            > /dev/null

fs/bin/watcher: watcher/src/**.rs
	cd watcher; cargo build --release
	-mv watcher/target/release/watcher fs/bin/watcher

export TMAC := ${tmac}

# rip lol

%.pdf: %.${tmac} fs/bin/watcher fs/bin/yacc fs/ucb/troff
	echo $@ $<
ifdef run
	-fs/bin/run.sh $(basename $@ .pdf)

else
	fs/bin/update.sh $(basename $@ .pdf)
endif

