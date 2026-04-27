FROM quay.io/hummingbird/rust@sha256:9d6682db07075c28856a3a4123e4942d12fa9badd72b17a410818669fa504f92 AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

RUN cargo build --release

# Binary only - expects glibc from the target container
FROM scratch
COPY --from=builder /build/target/release/fips-gate /fips-gate
ENTRYPOINT ["/fips-gate"]
