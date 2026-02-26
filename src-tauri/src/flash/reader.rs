use tauri::{AppHandle, Emitter};

use crate::error::AppResult;
use crate::flash::downloader::{CancelToken, LogEvent, ProgressEvent};
use crate::protocol::commands::cmd_read_memory;
use crate::protocol::constants::*;
use crate::serial::manager::SerialManager;

/// 读取 Flash 内容
pub fn read_flash_region(
    app: &AppHandle,
    serial: &SerialManager,
    start_address: u32,
    length: usize,
    cancel: &CancelToken,
) -> AppResult<Vec<u8>> {
    let mut data = Vec::with_capacity(length);
    let total_blocks = (length + MAX_READ_BLOCK - 1) / MAX_READ_BLOCK;
    let mut offset = 0usize;

    let _ = app.emit(
        "flash-log",
        LogEvent {
            level: "info".to_string(),
            message: format!(
                "开始读取 Flash: 0x{:08X} - 0x{:08X} ({} 字节)",
                start_address,
                start_address + length as u32,
                length
            ),
            timestamp: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
        },
    );

    let start_time = std::time::Instant::now();

    while offset < length {
        cancel.check()?;

        let remaining = length - offset;
        let block_len = remaining.min(MAX_READ_BLOCK);
        let addr = start_address + offset as u32;

        let block = cmd_read_memory(serial, addr, block_len)?;
        data.extend_from_slice(&block);
        offset += block_len;

        let block_index = offset / MAX_READ_BLOCK;
        let elapsed = start_time.elapsed().as_secs_f64();
        let speed = if elapsed > 0.0 {
            offset as f64 / elapsed / 1024.0
        } else {
            0.0
        };

        let _ = app.emit(
            "flash-progress",
            ProgressEvent {
                stage: "读取".to_string(),
                progress: (block_index as f64 / total_blocks as f64) * 100.0,
                current: block_index,
                total: total_blocks,
                speed,
            },
        );
    }

    let _ = app.emit(
        "flash-log",
        LogEvent {
            level: "success".to_string(),
            message: format!("读取完成: {} 字节", data.len()),
            timestamp: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
        },
    );

    Ok(data)
}

/// 导出为 BIN 文件
pub fn export_bin(data: &[u8], path: &str) -> AppResult<()> {
    std::fs::write(path, data)
        .map_err(|e| crate::error::AppError::Io(format!("导出 BIN 失败: {}", e)))?;
    Ok(())
}

/// 导出为 Intel HEX 文件
pub fn export_hex(data: &[u8], start_address: u32, path: &str) -> AppResult<()> {
    let mut hex_content = String::new();

    // Extended Linear Address Record
    let upper = (start_address >> 16) as u16;
    let upper_bytes = [(upper >> 8) as u8, (upper & 0xFF) as u8];
    let checksum = 0u8
        .wrapping_add(0x02)
        .wrapping_add(0x00)
        .wrapping_add(0x00)
        .wrapping_add(0x04)
        .wrapping_add(upper_bytes[0])
        .wrapping_add(upper_bytes[1]);
    hex_content.push_str(&format!(
        ":02000004{:02X}{:02X}{:02X}\n",
        upper_bytes[0],
        upper_bytes[1],
        (!checksum).wrapping_add(1)
    ));

    // Data Records (每行 16 字节)
    let base_offset = (start_address & 0xFFFF) as u16;
    for (i, chunk) in data.chunks(16).enumerate() {
        let offset = base_offset.wrapping_add((i * 16) as u16);
        let len = chunk.len() as u8;
        let offset_high = (offset >> 8) as u8;
        let offset_low = (offset & 0xFF) as u8;

        let mut sum = 0u8;
        sum = sum.wrapping_add(len);
        sum = sum.wrapping_add(offset_high);
        sum = sum.wrapping_add(offset_low);
        sum = sum.wrapping_add(0x00); // record type

        let mut data_hex = String::new();
        for &b in chunk {
            data_hex.push_str(&format!("{:02X}", b));
            sum = sum.wrapping_add(b);
        }

        let checksum = (!sum).wrapping_add(1);
        hex_content.push_str(&format!(
            ":{:02X}{:02X}{:02X}00{}{:02X}\n",
            len, offset_high, offset_low, data_hex, checksum
        ));
    }

    // EOF Record
    hex_content.push_str(":00000001FF\n");

    std::fs::write(path, hex_content)
        .map_err(|e| crate::error::AppError::Io(format!("导出 HEX 失败: {}", e)))?;

    Ok(())
}
