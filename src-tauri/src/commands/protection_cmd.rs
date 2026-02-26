use tauri::{AppHandle, State};

use crate::error::{AppError, AppResult};
use crate::flash::protection;
use crate::protocol::bootloader::{Bootloader, BootloaderState};
use crate::serial::manager::SerialManager;

/// 开启写保护
#[tauri::command]
pub fn enable_write_protection(
    app: AppHandle,
    serial: State<'_, SerialManager>,
    bootloader: State<'_, Bootloader>,
    sectors: Vec<u8>,
) -> AppResult<()> {
    if bootloader.state() != BootloaderState::Connected {
        return Err(AppError::NotConnected);
    }
    protection::enable_write_protect(&app, &serial, &sectors)?;
    // 写保护后设备复位，标记断开
    bootloader.disconnect();
    Ok(())
}

/// 关闭写保护
#[tauri::command]
pub fn disable_write_protection(
    app: AppHandle,
    serial: State<'_, SerialManager>,
    bootloader: State<'_, Bootloader>,
) -> AppResult<()> {
    if bootloader.state() != BootloaderState::Connected {
        return Err(AppError::NotConnected);
    }
    protection::disable_write_protect(&app, &serial)?;
    bootloader.disconnect();
    Ok(())
}

/// 开启读保护
#[tauri::command]
pub fn enable_readout_protection(
    app: AppHandle,
    serial: State<'_, SerialManager>,
    bootloader: State<'_, Bootloader>,
) -> AppResult<()> {
    if bootloader.state() != BootloaderState::Connected {
        return Err(AppError::NotConnected);
    }
    protection::enable_readout_protect(&app, &serial)?;
    bootloader.disconnect();
    Ok(())
}

/// 关闭读保护（会触发全片擦除）
#[tauri::command]
pub fn disable_readout_protection(
    app: AppHandle,
    serial: State<'_, SerialManager>,
    bootloader: State<'_, Bootloader>,
) -> AppResult<()> {
    if bootloader.state() != BootloaderState::Connected {
        return Err(AppError::NotConnected);
    }
    protection::disable_readout_protect(&app, &serial)?;
    bootloader.disconnect();
    Ok(())
}
