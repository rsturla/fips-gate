FROM quay.io/hummingbird/rust@sha256:2a8055093394cdecc6ab6b70a172e0c3eeff22579aaf0bcd52899153aa40f253 AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
