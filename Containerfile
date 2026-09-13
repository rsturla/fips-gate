FROM quay.io/hummingbird/rust@sha256:48cdbed14d6945956d8d58578a3c3d3b836db5df7fbbe28cfce98d0323fe5c7c AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
