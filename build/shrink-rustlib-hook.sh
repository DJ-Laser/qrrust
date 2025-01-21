#!/bin/bash

shrinkRustlibHook() {
    echo "Executing shrinkRustlibHook"

    shrinkDir="${releaseDir}-shrink";
    local prevDir=$(pwd)

    mkdir -p $shrinkDir
    cp -r ${releaseDir}/* $shrinkDir/
    cd $shrinkDir

    local binaryName=$pname
    local libraryName=lib${binaryName}.rlib

    local OBJFILE=$(ar t $libraryName | grep $binaryName)
    ar x $libraryName $OBJFILE

    ld --gc-sections -e _start -T @linkScript@ -o payload.out $OBJFILE
    objcopy -j combined -j .got -j .got.plt -O binary payload.out payload.bin

    local ENTRY=$(nm -f posix payload.out | grep '^_start ' | awk '{print $3}')
    nasm -f bin -o $binaryName -D entry=0x$ENTRY @headerAsm@

    chmod +x $binaryName

    cd $prevDir
    cp $shrinkDir/$binaryName $releaseDir/$binaryName

    rm -r $shrinkDir

    bins=${bins}\ $releaseDir/$binaryName

    echo "Finished shrinkRustlibHook"
}

if [ -z "${preInstall-}" ]; then
    preInstallHooks+=(shrinkRustlibHook)
fi
