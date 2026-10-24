FROM quay.io/hummingbird/rust@sha256:bb05ad7a32ed23e8d57d1a5d36f875b8b12b82df616d816466bb7fa6cfff8297 AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
