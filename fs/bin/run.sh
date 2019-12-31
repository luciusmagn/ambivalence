#!/bin/bash
cd ascii_czech
cargo build --release
cd ..
cd watcher
cargo build --release
cd ..
watcher/target/release/watcher $(basename $@ .me)
