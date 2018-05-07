#!/bin/bash
cd ansi_czech; cargo build --release; cd ..
watch -n 1 "ansi_czech/target/release/ansi_czech ambivalence.me | pdfroff -me - > ambivalence.pdf"

