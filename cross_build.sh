#!/usr/bin/env bash

docker run --rm -it -v $PWD:/app \
    $USER/tauri-cross:armv7-unknown-linux-gnueabihf bash \
    -c "yarn tauri build --target armv7-unknown-linux-gnueabihf"