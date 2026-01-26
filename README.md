# fips-gate

A minimal container entrypoint that enforces FIPS mode. If the host kernel has FIPS mode enabled, the specified command runs. Otherwise, it exits with an error.

## Usage

```dockerfile
FROM registry.access.redhat.com/ubi9/ubi-minimal

COPY --from=fips-gate /fips-gate /fips-gate
COPY myapp /myapp

ENTRYPOINT ["/fips-gate"]
CMD ["/myapp", "--config", "/etc/myapp.conf"]
```

The container will only run `/myapp` if the host has FIPS mode enabled.

### Command-line

```
fips-gate <command> [args...]
```

Arguments after `fips-gate` are passed directly to `exec()`, replacing the fips-gate process entirely. This means:

- PID 1 in the container becomes your application
- Signals are delivered directly to your application
- Exit codes pass through unchanged

### Environment variables

| Variable | Effect |
|----------|--------|
| `FIPS_GATE_BYPASS=1` | Skip FIPS check and run the command anyway |

Use `FIPS_GATE_BYPASS=1` for development and testing on non-FIPS systems.

## How it works

1. Check if `FIPS_GATE_BYPASS=1` is set → exec the command
2. Read `/proc/sys/crypto/fips_enabled`
3. If contents equal `1` → exec the command
4. Otherwise → print error and exit with code 1

The file `/proc/sys/crypto/fips_enabled` is a kernel-exposed interface on RHEL/Fedora systems that indicates whether FIPS 140-2/140-3 mode was enabled at boot time.

**Note:** fips-gate only verifies that the host kernel is in FIPS mode. It does not verify that the container's userspace libraries (OpenSSL, GnuTLS, NSS) or applications are FIPS-validated or correctly configured. Full FIPS compliance requires both a FIPS-enabled kernel and properly configured userspace components.

## Examples

### FIPS enabled (success)

```console
$ cat /proc/sys/crypto/fips_enabled
1
$ fips-gate /usr/bin/myapp --flag
# myapp runs normally
```

### FIPS disabled (failure)

On a RHEL host with FIPS disabled:

```console
$ cat /proc/sys/crypto/fips_enabled
0
$ podman run myimage
FIPS mode is not enabled on this system (fips_enabled=0).

This container requires FIPS 140 mode to be enabled on the host.
See https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/9/html/security_hardening/switching-rhel-to-fips-mode_security-hardening
```

On a non-RHEL host (e.g., Fedora, Debian):

```console
$ podman run myimage
FIPS mode is not enabled on this system (fips_enabled=0).

This container requires FIPS 140 mode to be enabled on the host.
See your distribution's documentation for enabling FIPS mode.
```

The container exits with code 1 in both cases.

### Bypass for development

```console
$ FIPS_GATE_BYPASS=1 fips-gate /usr/bin/myapp --flag
# myapp runs normally (no output from fips-gate)
```

### Containerfile patterns

**Command in ENTRYPOINT:**

```dockerfile
ENTRYPOINT ["/fips-gate", "/myapp"]
CMD ["--config", "/etc/myapp.conf"]
```

**Command in CMD (more flexible):**

```dockerfile
ENTRYPOINT ["/fips-gate"]
CMD ["/myapp", "--config", "/etc/myapp.conf"]
```

The second pattern allows users to override CMD at runtime without losing the FIPS gate:

```console
$ podman run myimage /bin/sh  # still goes through fips-gate
```

## Building

```console
$ podman build -o ./target/release -t fips-gate .
```

Or with Cargo directly:

```console
$ cargo build --release
```

The binary is at `./target/release/fips-gate`.

## Testing

```console
$ podman run --rm -v $(pwd):/build:Z -w /build quay.io/hummingbird/rust cargo test
```

## License

Apache-2.0
