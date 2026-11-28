FROM quay.io/hummingbird/rust@sha256:5170db358eb542ce8b8eb622a6bcbba6578086c647eebf216c3eb6100547ee5b AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
