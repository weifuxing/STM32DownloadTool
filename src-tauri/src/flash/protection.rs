use tauri::{AppHandle, Emitter};

use crate::error::AppResult;
use crate::flash::downloader::LogEvent;
use crate::protocol::commands::*;
use crate::serial::manager::SerialManager;

/// 开启写保护
pub fn enable_write_protect(
    app: &AppHandle,
    serial: &SerialManager,
    sectors: &[u8],
) -> AppResult<()> {
    let _ = app.emit(
        "flash-log",
        LogEvent {
            level: "info".to_string(),
            message: format!("开启写保护: {} 个扇区", sectors.len()),
            timestamp: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
        },
    );

    cmd_write_protect(serial, sectors)?;

    let _ = app.emit(
        "flash-log",
        LogEvent {
            level: "success".to_string(),
            message: "写保护已开启（设备将自动复位）".to_string(),
            timestamp: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
        },
    );

    Ok(())
}

/// 关闭写保护
pub fn disable_write_protect(app: &AppHandle, serial: &SerialManager) -> AppResult<()> {
    let _ = app.emit(
        "flash-log",
        LogEvent {
            level: "warning".to_string(),
            message: "关闭写保护（设备将自动复位）...".to_string(),
            timestamp: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
        },
    );

    cmd_write_unprotect(serial)?;

    let _ = app.emit(
        "flash-log",
        LogEvent {
            level: "success".to_string(),
            message: "写保护已关闭".to_string(),
            timestamp: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
        },
    );

    Ok(())
}

/// 开启读保护
pub fn enable_readout_protect(app: &AppHandle, serial: &SerialManager) -> AppResult<()> {
    let _ = app.emit(
        "flash-log",
        LogEvent {
            level: "warning".to_string(),
            message: "开启读保护（设备将自动复位）...".to_string(),
            timestamp: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
        },
    );

    cmd_readout_protect(serial)?;

    let _ = app.emit(
        "flash-log",
        LogEvent {
            level: "success".to_string(),
            message: "读保护已开启".to_string(),
            timestamp: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
        },
    );

    Ok(())
}

/// 关闭读保护（会触发全片擦除！）
pub fn disable_readout_protect(app: &AppHandle, serial: &SerialManager) -> AppResult<()> {
    let _ = app.emit(
        "flash-log",
        LogEvent {
            level: "error".to_string(),
            message: "关闭读保护（将触发全片擦除！设备将自动复位）...".to_string(),
            timestamp: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
        },
    );

    cmd_readout_unprotect(serial)?;

    let _ = app.emit(
        "flash-log",
        LogEvent {
            level: "success".to_string(),
            message: "读保护已关闭（Flash 已被擦除）".to_string(),
            timestamp: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
        },
    );

    Ok(())
}
