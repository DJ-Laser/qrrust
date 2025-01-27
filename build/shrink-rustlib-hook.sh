#!/bin/bash

shrinkRustlibHook() {
    echo "Executing shrinkRustlibHook"

    local prevDir=$(pwd)
    mkdir -p $out/bin
    mkdir -p $out/qr

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

    local zippedBinaryName=${binaryName}-zipped
    cp @unzipScript@ $zippedBinaryName
    ls -la $zippedBinaryName
    chmod 755 $zippedBinaryName
    gzip -9c $binaryName >> $zippedBinaryName
    chmod +x $zippedBinaryName

    local qrDataName=${binaryName}-qrdata
    local qrCodeName=${binaryName}.png
    echo "data:application/octet-stream;base64," > $qrDataName
    base64 $zippedBinaryName >> $qrDataName
    qrencode -r $qrDataName -o $qrCodeName

    cp $binaryName $out/bin/
    cp $zippedBinaryName $out/bin/
    cp $qrCodeName $out/qr

    cd $prevDir
    rm -r $tmpDir`1`

    rm -r $out/target

    echo "Finished shrinkRustlibHook"
}

if [ -z "${postInstall-}" ]; then
    postInstallHooks+=(shrinkRustlibHook)
fi
