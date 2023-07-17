FROM debian:bullseye-slim
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install --assume-yes --no-install-recommends \
        gnupg2 \
        ca-certificates \
        g++-arm-linux-gnueabihf \
        libc6-dev-armhf-cross \
        libclang-3.9-dev \
        build-essential \
        pkg-config \
        curl \
        wget \
        file

RUN dpkg --add-architecture armhf && \
    apt-get update && apt-get install --assume-yes --no-install-recommends \
        libwebkit2gtk-4.0-dev:armhf \
        libssl-dev:armhf \
        libgtk-3-dev:armhf \
        libayatana-appindicator3-dev:armhf \
        librsvg2-dev:armhf \
        libatk1.0-dev:armhf \
        libgdk-pixbuf-2.0-dev:armhf \
        libcairo2-dev:armhf \
        libpango1.0-dev:armhf \
        libgtk-3-dev:armhf \
        libsoup2.4-dev:armhf

# Install NodeJs and Yarn
RUN curl -fsSL https://deb.nodesource.com/setup_19.x | bash - &&\
    curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add &&\
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list &&\
    apt-get update && apt-get install --assume-yes --no-install-recommends \
        nodejs \
        yarn

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --target armv7-unknown-linux-gnueabihf

# Set envs
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
    CC_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc \
    CXX_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++ \
    BINDGEN_EXTRA_CLANG_ARGS_armv7_unknown_linux_gnueabihf="--sysroot=/usr/arm-linux-gnueabihf" \
    PKG_CONFIG_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig/:${PKG_CONFIG_PATH}" \
    PKG_CONFIG_ALLOW_CROSS=1 \
    PATH="/root/.cargo/bin:$PATH"
WORKDIR /app