#!/bin/bash

set -e

for d in cargo ld objcopy nasm; do
    which $d >/dev/null || (echo "Can't find $d, needed to build"; exit 1)
done

set -x

cargo build --release

cp ../target/release/libqrrust.rlib .

OBJFILE=$(ar t libqrrust.rlib | grep qrrust)

ar x libqrrust.rlib $OBJFILE
mv $OBJFILE qrrust.o

objdump -dr qrrust.o
echo

ld --gc-sections -e _start -T script.ld -o payload qrrust.o
objcopy -j combined -O binary payload payload.bin

ENTRY=$(nm -f posix payload | grep '^_start ' | awk '{print $3}')
nasm -f bin -o qrrust -D entry=0x$ENTRY header.s

chmod +x qrrust
hd qrrust
wc -c qrrust
