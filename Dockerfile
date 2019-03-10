FROM clux/muslrust:stable AS build

WORKDIR /build
COPY Cargo.lock Cargo.toml ./
COPY cli/Cargo.toml ./cli/
COPY lib/Cargo.toml ./lib/
RUN cargo fetch -v --locked

COPY cli/src ./cli/src
COPY lib/src ./lib/src
RUN cargo build --release -v --locked --all

FROM alpine:3.9 AS misc
WORKDIR /build
ARG ARCH=amd64
ARG OS=linux
RUN set -euo pipefail && \
    apk add --no-cache ca-certificates; \
    # upx installation
    wget https://github.com/upx/upx/releases/download/v3.95/upx-3.95-${ARCH}_${OS}.tar.xz; \
    tar xvf upx-3.95-${ARCH}_${OS}.tar.xz; \
    mv upx-3.95-${ARCH}_${OS}/upx /usr/local/bin/; \
    rm -r upx-3.95-${ARCH}_${OS} upx-3.95-${ARCH}_${OS}.tar.xz; \
    # ghr installation
    wget https://github.com/tcnksm/ghr/releases/download/v0.12.0/ghr_v0.12.0_${OS}_${ARCH}.tar.gz; \
    tar xvf ghr_v0.12.0_${OS}_${ARCH}.tar.gz; \
    mkdir -p /tmp/bin/; \
    mv ghr_v0.12.0_${OS}_${ARCH}/ghr /usr/local/bin/; \
    rm -r ghr_v0.12.0_${OS}_${ARCH} ghr_v0.12.0_${OS}_${ARCH}.tar.gz; \
    :
COPY --from=build /build/target/x86_64-unknown-linux-musl/release/smv ./smv_${ARCH}_${OS}
RUN upx --best ./smv_${ARCH}_${OS}

FROM scratch AS release
WORKDIR /app
ARG ARCH=amd64
ARG OS=linux
COPY --from=misc /build/smv_${ARCH}_${OS} ./smv
CMD ./smv
