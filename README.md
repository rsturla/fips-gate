# fips-gate

A minimal container entrypoint that enforces FIPS mode. If the host kernel has FIPS mode enabled, the specified command runs. Otherwise, it exits with an error.
