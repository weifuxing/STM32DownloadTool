use std::sync::Mutex;

use tauri::{AppHandle, State};

use crate::commands::firmware_cmd::FirmwareManager;
use crate::error::{AppError, AppResult};
use crate::flash::downloader::{self, CancelToken};
use crate::flash::reader;
use crate::protocol::bootloader::Bootloader;
use crate::protocol::commands::cmd_go;
use crate::serial::manager::SerialManager;

/// 全局取消令牌
pub struct CancelState {
    pub token: Mutex<CancelToken>,
}

impl CancelState {
    pub fn new() -> Self {
        Self {
            token: Mutex::new(CancelToken::new()),
        }
    }

    pub fn reset(&self) -> CancelToken {
        let token = CancelToken::new();
        *self.token.lock().unwrap() = token.clone();
        token
    }
}

/// 下载固件（完整流程）
#[tauri::command]
pub async fn download_firmware(
    app: AppHandle,
    serial: State<'_, SerialManager>,
    bootloader: State<'_, Bootloader>,
    firmware: State<'_, FirmwareManager>,
    cancel_state: State<'_, CancelState>,
    do_verify: bool,
    do_go: bool,
) -> AppResult<()> {
    let image = firmware
        .image()
        .ok_or(AppError::Firmware("未加载固件文件".to_string()))?;

    if bootloader.state() != crate::protocol::bootloader::BootloaderState::Connected {
        return Err(AppError::NotConnected);
    }

    let cancel = cancel_state.reset();
    bootloader.set_busy();

    // 在阻塞线程中执行
    let serial_ref = &*serial;
    let bootloader_ref = &*bootloader;
    let result = tokio::task::block_in_place(|| {
        downloader::download(&app, serial_ref, bootloader_ref, &image, &cancel, do_verify, do_go)
    });

    bootloader.set_connected();
    result
}

/// 仅擦除 Flash
#[tauri::command]
pub async fn erase_flash(
    app: AppHandle,
    serial: State<'_, SerialManager>,
    bootloader: State<'_, Bootloader>,
    cancel_state: State<'_, CancelState>,
) -> AppResult<()> {
    if bootloader.state() != crate::protocol::bootloader::BootloaderState::Connected {
        return Err(AppError::NotConnected);
    }

    let cancel = cancel_state.reset();
    bootloader.set_busy();

    let serial_ref = &*serial;
    let bootloader_ref = &*bootloader;
    let result = tokio::task::block_in_place(|| {
        downloader::erase_flash(&app, serial_ref, bootloader_ref, &cancel)
    });

    bootloader.set_connected();
    result
}

/// 仅校验 Flash
#[tauri::command]
pub async fn verify_flash(
    app: AppHandle,
    serial: State<'_, SerialManager>,
    firmware: State<'_, FirmwareManager>,
    bootloader: State<'_, Bootloader>,
    cancel_state: State<'_, CancelState>,
) -> AppResult<()> {
    let image = firmware
        .image()
        .ok_or(AppError::Firmware("未加载固件文件".to_string()))?;

    if bootloader.state() != crate::protocol::bootloader::BootloaderState::Connected {
        return Err(AppError::NotConnected);
    }

    let cancel = cancel_state.reset();
    bootloader.set_busy();

    let serial_ref = &*serial;
    let result = tokio::task::block_in_place(|| {
        downloader::verify_flash(&app, serial_ref, &image, &cancel)
    });

    bootloader.set_connected();
    result
}

/// 读取 Flash 内容
#[tauri::command]
pub async fn read_flash(
    app: AppHandle,
    serial: State<'_, SerialManager>,
    bootloader: State<'_, Bootloader>,
    cancel_state: State<'_, CancelState>,
    start_address: u32,
    length: usize,
) -> AppResult<Vec<u8>> {
    if bootloader.state() != crate::protocol::bootloader::BootloaderState::Connected {
        return Err(AppError::NotConnected);
    }

    let cancel = cancel_state.reset();
    bootloader.set_busy();

    let serial_ref = &*serial;
    let result = tokio::task::block_in_place(|| {
        reader::read_flash_region(&app, serial_ref, start_address, length, &cancel)
    });

    bootloader.set_connected();
    result
}

/// 导出 Flash 内容为文件
#[tauri::command]
pub async fn export_flash(
    app: AppHandle,
    serial: State<'_, SerialManager>,
    bootloader: State<'_, Bootloader>,
    cancel_state: State<'_, CancelState>,
    start_address: u32,
    length: usize,
    file_path: String,
    format: String,
) -> AppResult<()> {
    if bootloader.state() != crate::protocol::bootloader::BootloaderState::Connected {
        return Err(AppError::NotConnected);
    }

    let cancel = cancel_state.reset();
    bootloader.set_busy();

    let serial_ref = &*serial;
    let data = tokio::task::block_in_place(|| {
        reader::read_flash_region(&app, serial_ref, start_address, length, &cancel)
    })?;

    bootloader.set_connected();

    match format.to_lowercase().as_str() {
        "hex" => reader::export_hex(&data, start_address, &file_path)?,
        _ => reader::export_bin(&data, &file_path)?,
    }

    Ok(())
}

/// 跳转到指定地址执行
#[tauri::command]
pub fn go_to_address(
    serial: State<'_, SerialManager>,
    bootloader: State<'_, Bootloader>,
    address: u32,
) -> AppResult<()> {
    if bootloader.state() != crate::protocol::bootloader::BootloaderState::Connected {
        return Err(AppError::NotConnected);
    }
    cmd_go(&serial, address)?;
    Ok(())
}

/// 取消当前操作
#[tauri::command]
pub fn cancel_operation(cancel_state: State<'_, CancelState>) -> AppResult<()> {
    cancel_state.token.lock().unwrap().cancel();
    Ok(())
}
