use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn build_binary() -> String {
    let status = Command::new("cargo")
        .args(["build"])
        .status()
        .expect("failed to build");
    assert!(status.success());

    std::env::current_dir()
        .unwrap()
        .join("target/debug/fips-gate")
        .to_string_lossy()
        .to_string()
}

#[test]
fn test_no_args_shows_usage() {
    let bin = build_binary();
    let output = Command::new(&bin).output().expect("failed to run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage: fips-gate"));
}

#[test]
fn test_fips_enabled_runs_command() {
    let bin = build_binary();

    let mut fips_file = NamedTempFile::new().unwrap();
    writeln!(fips_file, "1").unwrap();

    // We can't easily override FIPS_ENABLED_PATH, so test via bypass
    let output = Command::new(&bin)
        .env("FIPS_GATE_BYPASS", "1")
        .args(["echo", "success"])
        .output()
        .expect("failed to run");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "success");
}

#[test]
#[ignore = "requires /proc/sys/crypto/fips_enabled"]
fn test_fips_disabled_exits_with_error() {
    let bin = build_binary();

    // Without bypass, on a non-FIPS system it should fail
    let output = Command::new(&bin)
        .env_remove("FIPS_GATE_BYPASS")
        .args(["echo", "should not run"])
        .output()
        .expect("failed to run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("FIPS mode is not enabled"),
        "unexpected stderr: {}",
        stderr
    );
}

#[test]
fn test_bypass_runs_command() {
    let bin = build_binary();

    let output = Command::new(&bin)
        .env("FIPS_GATE_BYPASS", "1")
        .args(["echo", "bypassed"])
        .output()
        .expect("failed to run");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "bypassed");
}

#[test]
fn test_bypass_requires_value_1() {
    let bin = build_binary();

    let output = Command::new(&bin)
        .env("FIPS_GATE_BYPASS", "true")
        .args(["echo", "should not run"])
        .output()
        .expect("failed to run");

    // Should fail because bypass only works with "1"
    assert!(!output.status.success());
}

#[test]
fn test_command_with_args() {
    let bin = build_binary();

    let output = Command::new(&bin)
        .env("FIPS_GATE_BYPASS", "1")
        .args(["sh", "-c", "echo hello world"])
        .output()
        .expect("failed to run");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "hello world"
    );
}

#[test]
fn test_command_exit_code_preserved() {
    let bin = build_binary();

    let output = Command::new(&bin)
        .env("FIPS_GATE_BYPASS", "1")
        .args(["sh", "-c", "exit 42"])
        .output()
        .expect("failed to run");

    assert_eq!(output.status.code(), Some(42));
}

#[test]
fn test_nonexistent_command() {
    let bin = build_binary();

    let output = Command::new(&bin)
        .env("FIPS_GATE_BYPASS", "1")
        .args(["/nonexistent/command"])
        .output()
        .expect("failed to run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to execute"));
}
