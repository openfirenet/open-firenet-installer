//! Records what a bridge reports about the stove over a heating cycle, to a text file that can be sent to the
//! project: every change of a discrete value, a sample of the analog ones every 15 seconds, and the notes typed by
//! the user ("the screen shows Cleaning"). Nothing personal goes into the file: no Wi-Fi name, no address.
//!
//! The bridge is only read (`GET /api/state`); nothing is ever sent to the stove.

use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::cli_i18n::CliLang;

/// Values that move all the time: they are sampled every `ANALOG_EVERY`, not logged at each change.
const ANALOG: &[&str] = &[
    "flame", "roomTemp", "boardSensor", "rssi", "idFanMeas", "idFanSet", "augerSet", "augerCurrent", "insertionMotor",
    "insertionCurrent", "airFlaps", "airFlapsTarget", "pressureSensor", "stageCur", "stageCur1", "stageTgt2",
    "serviceMinutes", "serviceCountdown", "kgTillCleaning", "pelletsTotal", "pelletHours", "bakeTemp", "gridMotor",
];
/// The analog values written in each sample line, in this order.
const SAMPLED: &[&str] = &[
    "mainState", "subState", "flame", "roomTemp", "idFanSet", "idFanMeas", "augerSet", "augerCurrent", "insertionMotor",
    "insertionCurrent", "airFlaps", "airFlapsTarget", "pressureSensor", "stageCur", "stageCur1", "stageTgt2",
    "boardSensor", "gridMotor", "serviceMinutes", "pelletsTotal",
];
const ANALOG_EVERY: Duration = Duration::from_secs(15);

/// One reading of the bridge, flattened: the stove's raw values under their names, and a few fields of the state
/// and of the settings under a `~` prefix.
pub type Snapshot = BTreeMap<String, String>;

fn text(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => "-".into(),
        other => other.to_string(),
    }
}

/// The snapshot of an `/api/state` answer. Only values of the stove are kept: nothing about the network.
pub fn snapshot(state: &Value) -> Snapshot {
    let mut snap = Snapshot::new();
    if let Some(raw) = state.get("raw_sensors").and_then(Value::as_object) {
        for (name, value) in raw {
            snap.insert(name.clone(), text(value));
        }
    }
    for (section, keys) in [
        ("stove", &["state", "is_burning", "warning_code", "error_code"][..]),
        ("controls", &["on", "mode", "target_temperature", "power_percent", "eco_mode"][..]),
    ] {
        for key in keys {
            if let Some(value) = state.get(section).and_then(|s| s.get(key)) {
                snap.insert(format!("~{}", key), text(value));
            }
        }
    }
    snap
}

/// The header of the file: what is needed to read the recording, and nothing that identifies the user.
pub fn header(state: &Value, started_utc: &str) -> Vec<String> {
    let at = |section: &str, key: &str| state.get(section).and_then(|s| s.get(key)).map(text).unwrap_or_else(|| "-".into());
    vec![
        "Open Firenet heating cycle recording".to_string(),
        format!("Started: {} UTC", started_utc),
        format!("Bridge firmware: {}", at("device", "version")),
        format!("Stove: {} | firmware {} | build {}", at("stove", "model_name"), at("stove", "mainboard_version"), at("stove", "firmware_build")),
        format!("Protocol variant: {}", state.get("version_frame").map(text).unwrap_or_else(|| "-".into())),
        "Not included: Wi-Fi name, IP and MAC addresses.".to_string(),
        "Lines: +elapsed, then START (every value), CHANGE (old -> new), ANALOG (a sample every 15 s), NOTE (typed by the user).".to_string(),
    ]
}

/// The discrete values that differ between two snapshots, as "name: old -> new".
pub fn changes(before: &Snapshot, after: &Snapshot) -> Vec<String> {
    let mut out = Vec::new();
    for (name, value) in after {
        if ANALOG.contains(&name.as_str()) {
            continue;
        }
        let old = before.get(name).map(String::as_str).unwrap_or("-");
        if old != value {
            out.push(format!("{}: {} -> {}", name, old, value));
        }
    }
    for name in before.keys() {
        if !after.contains_key(name) && !ANALOG.contains(&name.as_str()) {
            out.push(format!("{}: {} -> -", name, before[name]));
        }
    }
    out
}

/// The sample line of the analog values.
pub fn analog_line(snap: &Snapshot) -> String {
    SAMPLED.iter().map(|k| format!("{}={}", k, snap.get(*k).map(String::as_str).unwrap_or("-"))).collect::<Vec<_>>().join(" ")
}

/// "+01:02:03" for a duration since the start of the recording.
pub fn elapsed_stamp(elapsed: Duration) -> String {
    let s = elapsed.as_secs();
    format!("+{:02}:{:02}:{:02}", s / 3600, (s / 60) % 60, s % 60)
}

/// "2026-10-11 09:57:38" for a number of seconds since 1970 (UTC), without a date library.
pub fn utc_text(epoch_secs: u64) -> String {
    let days = (epoch_secs / 86400) as i64;
    let secs = epoch_secs % 86400;
    // Civil date from a day count (Howard Hinnant's algorithm).
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + if month <= 2 { 1 } else { 0 };
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, secs / 3600, (secs / 60) % 60, secs % 60)
}

/// The default file name, from the start time: "open-firenet-cycle-20261011-0957.log".
pub fn default_file_name(epoch_secs: u64) -> String {
    let t = utc_text(epoch_secs);
    format!("open-firenet-cycle-{}{}{}-{}{}.log", &t[0..4], &t[5..7], &t[8..10], &t[11..13], &t[14..16])
}

fn fetch_state(ip: &str) -> Result<Value> {
    let url = format!("http://{}/api/state", ip);
    let resp = ureq::get(&url).timeout(Duration::from_secs(4)).call().map_err(|e| anyhow!(e.to_string()))?;
    // A bridge up to firmware 4.1.0 can put a byte that is not valid UTF-8 in its state: read it leniently.
    let mut bytes = Vec::new();
    std::io::Read::read_to_end(&mut resp.into_reader(), &mut bytes)?;
    Ok(serde_json::from_str(&String::from_utf8_lossy(&bytes))?)
}

/// What a recording reports while it runs.
pub enum Progress {
    /// A line was written to the file.
    Line(String),
    /// A reading of the bridge got no answer (the count so far).
    Missed(u32),
}

/// Records until `duration` has passed or `stop` says so. `notes` carries the lines typed by the user. Returns the
/// number of changes seen.
pub fn record<F>(ip: &str, path: &Path, duration: Duration, interval: Duration, notes: &Receiver<String>,
                 stop: &dyn Fn() -> bool, lang: CliLang, mut on_progress: F) -> Result<u32>
where
    F: FnMut(Progress),
{
    let first = fetch_state(ip).map_err(|e| anyhow!(lang.record_unreachable(ip, &e.to_string())))?;
    let epoch = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let mut file = File::create(path).with_context(|| lang.record_cannot_write(&path.display().to_string()))?;
    for line in header(&first, &utc_text(epoch)) {
        writeln!(file, "{}", line)?;
    }
    writeln!(file)?;

    let started = Instant::now();
    let write = |file: &mut File, text: String, on_progress: &mut F| -> Result<()> {
        let line = format!("{} {}", elapsed_stamp(started.elapsed()), text);
        writeln!(file, "{}", line)?;
        file.flush()?;
        on_progress(Progress::Line(line));
        Ok(())
    };

    let mut last = snapshot(&first);
    write(&mut file, format!("START {}", last.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(" ")), &mut on_progress)?;
    write(&mut file, format!("ANALOG {}", analog_line(&last)), &mut on_progress)?;
    let mut last_sample = Instant::now();
    let mut seen = 0u32;
    let mut missed = 0u32;

    while started.elapsed() < duration && !stop() {
        // Sleep in short steps so that notes and a stop request are taken quickly.
        let wake = Instant::now() + interval;
        while Instant::now() < wake && !stop() {
            while let Ok(note) = notes.try_recv() {
                let note = note.trim();
                if !note.is_empty() {
                    write(&mut file, format!("NOTE {}", note), &mut on_progress)?;
                }
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        let state = match fetch_state(ip) {
            Ok(state) => state,
            Err(_) => {
                missed += 1;
                on_progress(Progress::Missed(missed));
                continue;
            }
        };
        let now = snapshot(&state);
        let diff = changes(&last, &now);
        if !diff.is_empty() {
            seen += diff.len() as u32;
            write(&mut file, format!("CHANGE {}", diff.join(" | ")), &mut on_progress)?;
        }
        if last_sample.elapsed() >= ANALOG_EVERY {
            last_sample = Instant::now();
            write(&mut file, format!("ANALOG {}", analog_line(&now)), &mut on_progress)?;
        }
        last = now;
    }
    write(&mut file, format!("END {} change(s), {} reading(s) without answer", seen, missed), &mut on_progress)?;
    Ok(seen)
}

/// Where a recording goes when no file is given: the current directory.
pub fn default_path() -> PathBuf {
    let epoch = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    PathBuf::from(default_file_name(epoch))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn state(sub: i64, flame: i64) -> Value {
        json!({
            "device": {"version": "4.1.1", "ip": "192.168.1.93", "mac": "AA:BB:CC:DD:EE:FF", "wifi_ssid": "MyHome"},
            "stove": {"state": "heating", "is_burning": true, "warning_code": 0, "error_code": 0, "model_name": "DOMO",
                      "mainboard_version": "2.29", "firmware_build": "58512"},
            "controls": {"on": true, "mode": "comfort", "target_temperature": 20.0, "power_percent": 70, "eco_mode": false},
            "version_frame": "V3",
            "raw_sensors": {"mainState": 4, "subState": sub, "flame": flame, "door": 1},
            "status": {"ssid": "MyHome", "ip": "192.168.1.93"}
        })
    }

    #[test]
    fn a_discrete_change_is_logged_and_an_analog_one_is_not() {
        let before = snapshot(&state(2, 300));
        let after = snapshot(&state(3, 450));
        assert_eq!(changes(&before, &after), vec!["subState: 2 -> 3".to_string()]);
        assert!(changes(&before, &before).is_empty());
    }

    #[test]
    fn a_change_of_state_or_setting_is_logged() {
        let mut later = state(2, 300);
        later["stove"]["state"] = json!("cleaning");
        later["controls"]["target_temperature"] = json!(21.0);
        let diff = changes(&snapshot(&state(2, 300)), &snapshot(&later));
        assert_eq!(diff, vec!["~state: heating -> cleaning".to_string(), "~target_temperature: 20.0 -> 21.0".to_string()]);
    }

    #[test]
    fn nothing_personal_goes_into_the_file() {
        let s = state(2, 300);
        let everything = format!("{}\n{:?}\n{}", header(&s, "2026-10-11 09:57:38").join("\n"), snapshot(&s), analog_line(&snapshot(&s)));
        for secret in ["MyHome", "192.168.1.93", "AA:BB:CC"] {
            assert!(!everything.contains(secret), "{} leaked", secret);
        }
        assert!(everything.contains("DOMO") && everything.contains("2.29"));
    }

    #[test]
    fn the_sample_line_keeps_its_columns() {
        let line = analog_line(&snapshot(&state(3, 450)));
        assert!(line.starts_with("mainState=4 subState=3 flame=450 roomTemp=- "));
    }

    #[test]
    fn stamps_and_dates() {
        assert_eq!(elapsed_stamp(Duration::from_secs(3723)), "+01:02:03");
        assert_eq!(utc_text(0), "1970-01-01 00:00:00");
        assert_eq!(utc_text(1_791_705_458), "2026-10-11 07:57:38");
        assert_eq!(utc_text(951_782_400), "2000-02-29 00:00:00"); // leap day
        assert_eq!(default_file_name(1_791_705_458), "open-firenet-cycle-20261011-0757.log");
    }
}
