#!/bin/bash
cd ansi_czech
cargo build --release
cd ..
cd watcher
cargo build --release
cd ..
watcher/target/release/watcher
