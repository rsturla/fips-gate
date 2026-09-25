FROM quay.io/hummingbird/rust@sha256:741c2e5156dae5ceeb031b3c82c19664ea17c0d59dffd37d14de38f1da1978ea AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
