FROM quay.io/hummingbird/rust@sha256:69baac0faf77754d72189f7c764075d02d1dcd3efe3749eda072e6e0592a774a AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
