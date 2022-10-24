#!/bin/sh
docker run --platform linux/x86_64 --rm -v $CARGO_MANIFEST_DIR:$CARGO_MANIFEST_DIR \
    qemu \
    sh -c "cd $CARGO_MANIFEST_DIR;
    qemu-system-gnuarmeclipse \
        -cpu cortex-m4 \
        -machine STM32F4-Discovery \
        -nographic \
        -semihosting-config enable=on,target=native \
        -kernel $1"