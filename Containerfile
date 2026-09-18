FROM quay.io/hummingbird/rust@sha256:d582cda57eb0b9388677f8c3f7247b7b90139157380ca525f7ab21679c85ebca AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
