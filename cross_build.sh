#!/usr/bin/env bash

docker run --rm -it -v $PWD:/app \
    joaoantoniocardoso/cross-rs:armv7-unknown-linux-gnueabihf-bullseye-slim-tauri bash \
    -c "cd /app && yarn tauri build --target armv7-unknown-linux-gnueabihf"