#!/bin/bash
echo updating...
ansi_czech/target/release/ansi_czech ambivalence.me | pdfroff -me - > ambivalence.pdf

