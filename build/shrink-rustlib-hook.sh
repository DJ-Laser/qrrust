#!/bin/bash

shrinkRustlibHook() {
    echo "Executing shrinkRustlibHook"

    local prevDir=$(pwd)
    mkdir -p $out/bin

    local binaryName=$pname
    local libraryName=lib${binaryName}.rlib

    local tmpDir=$(mktemp -d)

    cp $out/target/x86_64-unknown-linux-gnu/release/$libraryName $tmpDir
    cd $tmpDir

    local OBJFILE=$(ar t $libraryName | grep $binaryName)
    ar x $libraryName $OBJFILE

    ld --gc-sections -e _start -T @linkScript@ -o payload.out $OBJFILE
    objcopy -j combined -j .got -j .got.plt -O binary payload.out payload.bin

    local ENTRY=$(nm -f posix payload.out | grep '^_start ' | awk '{print $3}')
    nasm -f bin -o $binaryName -D entry=0x$ENTRY @headerAsm@

    chmod +x $binaryName

    cd $out
    cp $tmpDir/$binaryName bin/
    rm -r $tmpDir

    cd $prevDir
    rm -r $out/target

    echo "Finished shrinkRustlibHook"
}

if [ -z "${postInstall-}" ]; then
    postInstallHooks+=(shrinkRustlibHook)
fi
