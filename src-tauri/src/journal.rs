//! Journal of what the installer did since it started, kept in memory, to attach to a report.
//!
//! An error shown to the user is the last message of a chain: it does not tell which operation failed (opening the
//! port, writing, flushing, reading) nor what the board had answered before. The journal notes each step of the
//! exchanges with the board and each failure with the system's error code. Nothing secret goes in: the callers
//! pass the Wi-Fi name and passwords through `mask`.
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

const MAX_LINES: usize = 2000;

static START: OnceLock<Instant> = OnceLock::new();
static LINES: Mutex<Vec<String>> = Mutex::new(Vec::new());
static ECHO: OnceLock<bool> = OnceLock::new();

/// Prints each line on the error output as it is noted (command line option `--debug`).
pub fn echo_to_stderr() {
    let _ = ECHO.set(true);
}

/// Adds a line, stamped with the seconds since the program started.
pub fn note(text: impl AsRef<str>) {
    let elapsed = START.get_or_init(Instant::now).elapsed().as_secs_f64();
    let line = format!("[{:9.3}] {}", elapsed, text.as_ref());
    if ECHO.get().copied().unwrap_or(false) {
        eprintln!("{}", line);
    }

    if let Ok(mut lines) = LINES.lock() {
        if lines.len() >= MAX_LINES {
            lines.remove(0);
        }
        lines.push(line);
    }
}

/// Notes the outcome of one input/output operation and hands the result back unchanged.
pub fn io<T>(operation: &str, result: std::io::Result<T>) -> std::io::Result<T> {
    if let Err(e) = &result {
        // A read that times out is the normal way to wait: it would fill the journal.
        if e.kind() != std::io::ErrorKind::TimedOut {
            note(format!("FAILED {}: {} (kind {:?}, os error {:?})", operation, e, e.kind(), e.raw_os_error()));
        }
    }
    result
}

/// Notes how a whole action ended and hands the result back unchanged.
/// The error is written with its causes (`{:#}`), which the message shown to the user leaves out.
pub fn outcome<T, E: std::fmt::Display>(action: &str, result: Result<T, E>) -> Result<T, E> {
    match &result {
        Ok(_) => note(format!("{}: done", action)),
        Err(e) => note(format!("{}: ERROR: {:#}", action, e)),
    }
    result
}

/// `text` with each secret replaced by a mark; empty secrets are ignored.
pub fn mask(text: &str, secrets: &[&str]) -> String {
    let mut out = text.to_string();
    for secret in secrets.iter().filter(|s| !s.is_empty()) {
        out = out.replace(secret, "<masked>");
    }
    out
}

/// What a board sent, on one line: control characters shown as escapes.
pub fn printable(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).escape_debug().to_string()
}

/// The whole journal as a text file: the program, the system, the serial ports seen now, then the lines.
pub fn text(version: &str) -> String {
    let mut out = vec![
        "Open Firenet Installer journal".to_string(),
        format!("Installer version: {}", version),
        format!("System: {} {}", std::env::consts::OS, std::env::consts::ARCH),
        "Masked: Wi-Fi name and passwords.".to_string(),
        String::new(),
        "===== SERIAL PORTS NOW =====".to_string(),
    ];
    match serialport::available_ports() {
        Ok(ports) if ports.is_empty() => out.push("(none)".to_string()),
        Ok(ports) => {
            for p in ports {
                out.push(match p.port_type {
                    serialport::SerialPortType::UsbPort(u) => format!(
                        "{}: USB {:04x}:{:04x} manufacturer={:?} product={:?}",
                        p.port_name, u.vid, u.pid, u.manufacturer.unwrap_or_default(), u.product.unwrap_or_default()
                    ),
                    other => format!("{}: {:?}", p.port_name, other),
                });
            }
        }
        Err(e) => out.push(format!("(could not be listed: {})", e)),
    }
    out.push(String::new());
    out.push("===== JOURNAL (seconds since start) =====".to_string());
    if let Ok(lines) = LINES.lock() {
        out.extend(lines.iter().cloned());
    }
    out.join("\n") + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secrets_are_masked_and_empty_ones_ignored() {
        assert_eq!(mask("SETWIFI:Home:hunter2", &["Home", "hunter2", ""]), "SETWIFI:<masked>:<masked>");
    }

    #[test]
    fn a_failure_is_noted_with_its_operation_and_system_code_and_handed_back() {
        let err: std::io::Result<()> = Err(std::io::Error::from_raw_os_error(22));
        assert!(io("flush COM5", err).is_err());
        let _: std::io::Result<()> = io("read COM5", Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "t")));
        let all = text("0.0.0");
        assert!(all.contains("FAILED flush COM5") && all.contains("os error Some(22)"));
        assert!(!all.contains("read COM5"), "a timeout is not a failure");
    }

    #[test]
    fn what_the_board_sent_stays_on_one_line() {
        assert_eq!(printable(b"[wifi] SETWIFI OK\r\n"), "[wifi] SETWIFI OK\\r\\n");
    }
}
