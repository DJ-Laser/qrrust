#!/bin/bash

shrinkRustlibHook() {
    echo "Executing shrinkRustlibHook"

    local prevDir=$(pwd)
    mkdir -p $out/bin

    local binaryName=$pname
    local libraryName=lib${binaryName}.a

    local tmpDir=$(mktemp -d)

    cp $out/target/x86_64-unknown-linux-gnu/release/$libraryName $tmpDir
    cd $tmpDir

    ld --gc-sections -e _start -T @linkScript@ -o payload.out $libraryName
    objcopy -j combined -j .got -j .got.plt -O binary payload.out payload.bin

    local entryAddress=$(nm -f posix payload.out | grep '^_start ' | awk '{print $3}')
    nasm -f bin -o $binaryName -D entry=0x$entryAddress @headerAsm@

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
