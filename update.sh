#!/bin/bash
echo updating...
ascii_czech/target/release/ascii_czech ambivalence.me | pdfroff -me - > ambivalence.pdf

