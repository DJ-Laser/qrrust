#!/bin/bash

(tail -n +$((LINENO + 4)) "$0") | gzip -d > qrrust
chmod +x qrrust
./qrrust
exit
