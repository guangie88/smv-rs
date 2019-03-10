FROM clux/muslrust:stable as base

WORKDIR /build
COPY Cargo.lock Cargo.toml ./
COPY cli/Cargo.toml ./cli/
COPY lib/Cargo.toml ./lib/
RUN cargo fetch -v --locked

COPY cli/src ./cli/src
COPY lib/src ./lib/src

FROM base as build-release
RUN cargo build --release -v --locked --all

FROM scratch as release
WORKDIR /app
COPY --from=build-release /build/target/x86_64-unknown-linux-musl/release/smv ./
CMD /app/smv

FROM base as build
RUN cargo build -v --locked --all

FROM base as test
RUN rustup component add rustfmt
RUN cargo test --no-run -v --locked --all
