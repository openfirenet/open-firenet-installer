//! Looks at the board plugged into the computer before anything is written to it: which chip it is, how much flash it
//! has, and what its partition table says. From that the installer tells a new board from one that already runs Open
//! Firenet, and preselects "first installation" or "update" (issue #29). Nothing is written.

use anyhow::{anyhow, Result};
use serde::Serialize;

use crate::cli_i18n::CliLang;

/// Where the partition table lives and how much of it is read.
const TABLE_OFFSET: u32 = 0x8000;
const TABLE_SIZE: u32 = 0xC00;
/// The size of an update slot in the layout the releases are built for ("Minimal SPIFFS": two slots of 1.9 MB).
pub const NEEDED_SLOT: u32 = 0x1E_0000;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Partition {
    pub label: String,
    pub kind: u8,
    pub subtype: u8,
    pub offset: u32,
    pub size: u32,
}

/// What the board is, for the choice of the flashing mode.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BoardState {
    /// Not an ESP32-S3: the firmware cannot run on it.
    WrongChip,
    /// Less than 4 MB of flash: the firmware does not fit.
    FlashTooSmall,
    /// No partition table: a new or erased board. Needs the first installation.
    Blank,
    /// A partition table without two update slots large enough (Arduino's default layout, another firmware...).
    /// Needs the first installation, which rewrites the table.
    OtherLayout,
    /// The layout of the releases, with an application in place: an update keeps the settings.
    Ready,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoardReport {
    pub state: BoardState,
    pub chip: String,
    pub flash_bytes: u32,
    pub partitions: Vec<Partition>,
}

/// The entries of a partition table image (32 bytes each, magic 0xAA 0x50), up to the end marker.
pub fn parse_partition_table(data: &[u8]) -> Vec<Partition> {
    let mut out = Vec::new();
    for entry in data.chunks_exact(32) {
        if entry[0] != 0xAA || entry[1] != 0x50 {
            break; // 0xEB 0xEB = MD5 line, 0xFF = end of the table or erased flash
        }
        let word = |at: usize| u32::from_le_bytes([entry[at], entry[at + 1], entry[at + 2], entry[at + 3]]);
        let label: Vec<u8> = entry[12..28].iter().copied().take_while(|b| *b != 0).collect();
        out.push(Partition {
            label: String::from_utf8_lossy(&label).into_owned(),
            kind: entry[2],
            subtype: entry[3],
            offset: word(4),
            size: word(8),
        });
    }
    out
}

/// The state of a board from what was read on it. `app_present`: the first update slot starts with an application
/// image.
pub fn classify(is_esp32s3: bool, flash_bytes: u32, partitions: &[Partition], app_present: bool) -> BoardState {
    if !is_esp32s3 {
        return BoardState::WrongChip;
    }
    if flash_bytes < 4 * 1024 * 1024 {
        return BoardState::FlashTooSmall;
    }
    if partitions.is_empty() {
        return BoardState::Blank;
    }
    // Application partitions (type 0) of the OTA kind (subtypes 0x10..0x1F).
    let slots: Vec<&Partition> = partitions.iter().filter(|p| p.kind == 0 && (0x10..0x20).contains(&p.subtype)).collect();
    let layout_ok = slots.len() >= 2 && slots.iter().all(|p| p.size >= NEEDED_SLOT) && slots.iter().any(|p| p.offset == 0x1_0000);
    if layout_ok && app_present {
        BoardState::Ready
    } else {
        BoardState::OtherLayout
    }
}

/// Connects to the board on this port and reads its chip, its flash size, its partition table and the first bytes
/// of its application. The board is restarted into its bootloader for this, and left to restart normally.
pub fn probe(port_name: &str, lang: CliLang) -> Result<BoardReport> {
    use espflash::connection::reset::{ResetAfterOperation, ResetBeforeOperation};
    use espflash::flasher::Flasher;
    use espflash::targets::Chip;
    use serialport::{SerialPortType, UsbPortInfo};

    let unknown = UsbPortInfo { vid: 0, pid: 0, serial_number: None, manufacturer: None, product: None };
    let port_info = serialport::available_ports()
        .ok()
        .and_then(|ports| ports.into_iter().find(|p| p.port_name == port_name))
        .and_then(|p| match p.port_type {
            SerialPortType::UsbPort(info) => Some(info),
            _ => None,
        })
        .unwrap_or(unknown);
    let serial = serialport::new(port_name, 115_200)
        .flow_control(serialport::FlowControl::None)
        .open_native()
        .map_err(|e| anyhow!(lang.cannot_open_port(port_name, &e.to_string())))?;
    let before = if port_info.pid == 0x1001 { ResetBeforeOperation::UsbReset } else { ResetBeforeOperation::DefaultReset };

    // No chip is imposed: the point is to learn which one it is.
    let mut flasher = Flasher::connect(serial, port_info, Some(115_200), true, false, false, None, ResetAfterOperation::HardReset, before)
        .map_err(|e| anyhow!(lang.bootloader_connect_failed(&format!("{:?}", e))))?;
    let info = flasher.device_info().map_err(|e| anyhow!(lang.bootloader_connect_failed(&format!("{:?}", e))))?;
    let is_s3 = info.chip == Chip::Esp32s3;
    let flash_bytes = info.flash_size.size();

    let mut read = |offset: u32, size: u32| -> Result<Vec<u8>> {
        let path = std::env::temp_dir().join(format!("open-firenet-probe-{}-{:x}.bin", std::process::id(), offset));
        let result = flasher.read_flash(offset, size, 0x1000, 64, path.clone());
        let data = std::fs::read(&path).unwrap_or_default();
        let _ = std::fs::remove_file(&path);
        result.map_err(|e| anyhow!(lang.bootloader_connect_failed(&format!("{:?}", e))))?;
        Ok(data)
    };
    let (partitions, app_present) = if is_s3 && flash_bytes >= 4 * 1024 * 1024 {
        let table = parse_partition_table(&read(TABLE_OFFSET, TABLE_SIZE)?);
        // An application image starts with the byte 0xE9.
        let app = read(0x1_0000, 0x1000).map(|d| d.first() == Some(&0xE9)).unwrap_or(false);
        (table, app)
    } else {
        (Vec::new(), false)
    };
    Ok(BoardReport { state: classify(is_s3, flash_bytes, &partitions, app_present), chip: format!("{}", info.chip), flash_bytes, partitions })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(kind: u8, subtype: u8, offset: u32, size: u32, label: &str) -> Vec<u8> {
        let mut e = vec![0xAA, 0x50, kind, subtype];
        e.extend_from_slice(&offset.to_le_bytes());
        e.extend_from_slice(&size.to_le_bytes());
        let mut name = label.as_bytes().to_vec();
        name.resize(16, 0);
        e.extend_from_slice(&name);
        e.extend_from_slice(&[0; 4]);
        e
    }
    fn table(entries: &[Vec<u8>]) -> Vec<u8> {
        let mut t: Vec<u8> = entries.concat();
        t.resize(0xC00, 0xFF);
        t
    }
    /// The layout of the releases ("Minimal SPIFFS").
    fn ours() -> Vec<u8> {
        table(&[
            entry(1, 2, 0x9000, 0x5000, "nvs"),
            entry(1, 0, 0xE000, 0x2000, "otadata"),
            entry(0, 0x10, 0x1_0000, 0x1E_0000, "app0"),
            entry(0, 0x11, 0x1F_0000, 0x1E_0000, "app1"),
            entry(1, 0x82, 0x3D_0000, 0x2_0000, "spiffs"),
        ])
    }
    /// Arduino's default layout: two slots of 1,310,720 bytes.
    fn arduino_default() -> Vec<u8> {
        table(&[
            entry(1, 2, 0x9000, 0x5000, "nvs"),
            entry(1, 0, 0xE000, 0x2000, "otadata"),
            entry(0, 0x10, 0x1_0000, 0x14_0000, "app0"),
            entry(0, 0x11, 0x15_0000, 0x14_0000, "app1"),
            entry(1, 0x82, 0x29_0000, 0x16_0000, "spiffs"),
        ])
    }
    const MB4: u32 = 4 * 1024 * 1024;

    #[test]
    fn the_table_is_read() {
        let parts = parse_partition_table(&ours());
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[2], Partition { label: "app0".into(), kind: 0, subtype: 0x10, offset: 0x1_0000, size: 0x1E_0000 });
        assert!(parse_partition_table(&vec![0xFF; 0xC00]).is_empty()); // erased flash
        assert!(parse_partition_table(&[]).is_empty());
    }

    #[test]
    fn a_new_board_needs_the_first_installation() {
        assert_eq!(classify(true, MB4, &[], false), BoardState::Blank);
    }

    #[test]
    fn a_board_with_the_release_layout_and_an_application_can_be_updated() {
        assert_eq!(classify(true, MB4, &parse_partition_table(&ours()), true), BoardState::Ready);
        // The right table but no application: an update would not be enough.
        assert_eq!(classify(true, MB4, &parse_partition_table(&ours()), false), BoardState::OtherLayout);
    }

    #[test]
    fn arduino_s_default_layout_needs_the_first_installation() {
        assert_eq!(classify(true, MB4, &parse_partition_table(&arduino_default()), true), BoardState::OtherLayout);
        // A layout without any update slot (the bridge then reports ota_slot_bytes = 0).
        let no_ota = parse_partition_table(&table(&[entry(1, 2, 0x9000, 0x5000, "nvs"), entry(0, 0, 0x1_0000, 0x30_0000, "factory")]));
        assert_eq!(classify(true, MB4, &no_ota, true), BoardState::OtherLayout);
    }

    #[test]
    fn a_wrong_chip_or_a_small_flash_is_refused() {
        assert_eq!(classify(false, MB4, &parse_partition_table(&ours()), true), BoardState::WrongChip);
        assert_eq!(classify(true, 2 * 1024 * 1024, &[], false), BoardState::FlashTooSmall);
    }
}
