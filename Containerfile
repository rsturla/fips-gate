FROM quay.io/hummingbird/rust@sha256:89d155adc85ac9025d9a9fa583008352e1b6357d765bca8e2e542e6173ea0354 AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
