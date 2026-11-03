FROM quay.io/hummingbird/rust@sha256:3940bcd47612f953ee1af2cea4978074bea4f803b4a7da173efdd751b7532364 AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
