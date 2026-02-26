use tauri::State;

use crate::error::AppResult;
use crate::protocol::bootloader::{Bootloader, BootloaderState, ChipInfo};
use crate::serial::manager::SerialManager;

/// 连接到 bootloader
#[tauri::command]
pub fn connect_bootloader(
    serial: State<'_, SerialManager>,
    bootloader: State<'_, Bootloader>,
    isp_mode: String,
) -> AppResult<ChipInfo> {
    bootloader.connect(&serial, &isp_mode)
}

/// 断开 bootloader
#[tauri::command]
pub fn disconnect_bootloader(bootloader: State<'_, Bootloader>) -> AppResult<()> {
    bootloader.disconnect();
    Ok(())
}

/// 获取 bootloader 状态
#[tauri::command]
pub fn get_bootloader_state(bootloader: State<'_, Bootloader>) -> BootloaderState {
    bootloader.state()
}

/// 获取芯片信息
#[tauri::command]
pub fn get_chip_info(bootloader: State<'_, Bootloader>) -> Option<ChipInfo> {
    bootloader.chip_info()
}
