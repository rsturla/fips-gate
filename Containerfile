FROM quay.io/hummingbird/rust@sha256:d3d4c93145a07225d3a6a2ff5f2ca95edb8263e65044da5ec9d51471571d215b AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
