FROM quay.io/hummingbird/rust@sha256:ee55e4d2fbcf677ff4144cf665e79596f2d767c6f65653731bd2c3c9fff04174 AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
