FROM quay.io/hummingbird/rust:latest-builder

# Install rust-src for -Z build-std
RUN dnf install -y rust-src && dnf clean all

WORKDIR /build
COPY Cargo.toml Cargo.lock .
COPY src src

# Build with -Z build-std for minimal binary size (~60KB vs ~300KB)
# RUSTC_BOOTSTRAP=1 enables unstable features on stable Rust
# This technique is used by Android, Firefox, and Chromium
ENV RUSTC_BOOTSTRAP=1
ENV RUSTFLAGS="-Zunstable-options -Cpanic=immediate-abort"

RUN cargo build \
    -Z build-std=core,std,alloc \
    --target x86_64-unknown-linux-gnu \
    --release

FROM scratch
COPY --from=0 /build/target/x86_64-unknown-linux-gnu/release/fips-gate /fips-gate
