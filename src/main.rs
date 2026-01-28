use std::env;
use std::fs;
use std::os::unix::process::CommandExt;
use std::process::{self, Command};

const FIPS_ENABLED_PATH: &str = "/proc/sys/crypto/fips_enabled";
const KERNEL_RELEASE_PATH: &str = "/proc/sys/kernel/osrelease";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: fips-gate <command> [args...]");
        process::exit(1);
    }

    if should_bypass() {
        exec_command(&args);
    }

    match check_fips(FIPS_ENABLED_PATH) {
        FipsStatus::Enabled => exec_command(&args),
        FipsStatus::Disabled(value) => {
            eprintln!(
                "FIPS mode is not enabled on this system (fips_enabled={}).",
                value
            );
            print_fips_help();
            process::exit(1);
        }
        FipsStatus::Unavailable(err) => {
            eprintln!("Unable to determine FIPS mode status: {}", err);
            print_fips_help();
            process::exit(1);
        }
    }
}

fn print_fips_help() {
    eprintln!();
    eprintln!("This container requires FIPS 140 mode to be enabled on the host.");

    let distro = detect_distro();
    match distro {
        Distro::Rhel10 => {
            eprintln!("See https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/10/html/security_hardening/switching-rhel-to-fips-mode_security-hardening");
        }
        Distro::Rhel9 => {
            eprintln!("See https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/9/html/security_hardening/switching-rhel-to-fips-mode_security-hardening");
        }
        Distro::Rhel8 => {
            eprintln!("See https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/8/html/security_hardening/switching-rhel-to-fips-mode_security-hardening");
        }
        Distro::Unknown => {
            eprintln!("See your distribution's documentation for enabling FIPS mode.");
        }
    }
}

#[derive(Debug, PartialEq)]
enum Distro {
    Rhel10,
    Rhel9,
    Rhel8,
    Unknown,
}

fn detect_distro() -> Distro {
    let release = match fs::read_to_string(KERNEL_RELEASE_PATH) {
        Ok(r) => r,
        Err(_) => return Distro::Unknown,
    };
    parse_distro(&release)
}

fn parse_distro(release: &str) -> Distro {
    if release.contains(".el10") {
        Distro::Rhel10
    } else if release.contains(".el9") {
        Distro::Rhel9
    } else if release.contains(".el8") {
        Distro::Rhel8
    } else {
        Distro::Unknown
    }
}

#[derive(Debug, PartialEq)]
enum FipsStatus {
    Enabled,
    Disabled(String),
    Unavailable(String),
}

fn should_bypass() -> bool {
    env::var("FIPS_GATE_BYPASS")
        .map(|v| v == "1")
        .unwrap_or(false)
}

fn check_fips(path: &str) -> FipsStatus {
    match fs::read_to_string(path) {
        Ok(content) => {
            let value = content.trim();
            if value == "1" {
                FipsStatus::Enabled
            } else {
                FipsStatus::Disabled(value.to_string())
            }
        }
        Err(e) => FipsStatus::Unavailable(e.to_string()),
    }
}

fn exec_command(args: &[String]) -> ! {
    let err = Command::new(&args[0]).args(&args[1..]).exec();
    eprintln!("Failed to execute '{}': {}", args[0], err);
    process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_check_fips_enabled() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "1").unwrap();
        assert_eq!(
            check_fips(file.path().to_str().unwrap()),
            FipsStatus::Enabled
        );
    }

    #[test]
    fn test_check_fips_enabled_no_newline() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "1").unwrap();
        assert_eq!(
            check_fips(file.path().to_str().unwrap()),
            FipsStatus::Enabled
        );
    }

    #[test]
    fn test_check_fips_disabled() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "0").unwrap();
        assert_eq!(
            check_fips(file.path().to_str().unwrap()),
            FipsStatus::Disabled("0".to_string())
        );
    }

    #[test]
    fn test_check_fips_unavailable() {
        let status = check_fips("/nonexistent/path/fips_enabled");
        assert!(matches!(status, FipsStatus::Unavailable(_)));
    }

    #[test]
    fn test_check_fips_unexpected_value() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "2").unwrap();
        assert_eq!(
            check_fips(file.path().to_str().unwrap()),
            FipsStatus::Disabled("2".to_string())
        );
    }

    #[test]
    fn test_should_bypass_not_set() {
        env::remove_var("FIPS_GATE_BYPASS");
        assert!(!should_bypass());
    }

    #[test]
    fn test_should_bypass_set_to_1() {
        env::set_var("FIPS_GATE_BYPASS", "1");
        assert!(should_bypass());
        env::remove_var("FIPS_GATE_BYPASS");
    }

    #[test]
    fn test_should_bypass_set_to_other() {
        env::set_var("FIPS_GATE_BYPASS", "true");
        assert!(!should_bypass());
        env::remove_var("FIPS_GATE_BYPASS");
    }

    #[test]
    fn test_parse_distro_rhel10() {
        assert_eq!(parse_distro("6.12.0-55.el10.x86_64"), Distro::Rhel10);
    }

    #[test]
    fn test_parse_distro_rhel9() {
        assert_eq!(parse_distro("5.14.0-362.24.1.el9_3.x86_64"), Distro::Rhel9);
    }

    #[test]
    fn test_parse_distro_rhel8() {
        assert_eq!(parse_distro("4.18.0-513.11.1.el8_9.x86_64"), Distro::Rhel8);
    }

    #[test]
    fn test_parse_distro_unknown() {
        assert_eq!(parse_distro("6.1.0-custom"), Distro::Unknown);
        assert_eq!(parse_distro("6.18.5-200.fc43.x86_64"), Distro::Unknown);
        assert_eq!(parse_distro("5.15.0-91-ubuntu"), Distro::Unknown);
    }
}
