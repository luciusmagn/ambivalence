#!/bin/bash
if [[ -v $TMAC ]]; then
	fs/bin/watcher $(basename $@ $TMAC)
else
	fs/bin/watcher $(basename $@ .me)
fi
