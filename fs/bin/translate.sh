#/bin/bash

cat -  \
	| sed 's:\\\[:\\*[:g' \
	| sed 's:\\\]:\\*]:g' \
	| sed 's:\\(:\\*(:g' \
	| sed 's:\\):\\*):g'
