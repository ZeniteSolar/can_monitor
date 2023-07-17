 #!/usr/bin/env bash

docker build \
    -t $USER/tauri-cross:armv7-unknown-linux-gnueabihf \
    --output type=registry \
    --progress=plain \
    docker/armv7-unknown-linux-gnueabihf/