#!/bin/bash -x
echo converting pictures...

type fd      >/dev/null 2>&1 || { echo >&2 "automatic picture conversion requires fd-find";     exit 1; }
type convert >/dev/null 2>&1 || { echo >&2 "automatic picture conversion requires imagemagick"; exit 1; }

cd img/src
fd -e jpg -e png -e gif -x convert '{}' '../out/{/.}.eps'
