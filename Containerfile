FROM quay.io/hummingbird/rust@sha256:5242ee29bb4ed200e42a73d33e75e1f8e2aa137709b85ae3d2a59eea740d358e AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
