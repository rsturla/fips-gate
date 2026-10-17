FROM quay.io/hummingbird/rust@sha256:bc28fbc691458652eadf5e8f88d59080beb2d45e1c9169d0274801f051a16544 AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
