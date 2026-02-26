use serde::Serialize;

use crate::error::{AppError, AppResult};
use crate::protocol::commands::*;
use crate::protocol::constants::*;
use crate::serial::manager::SerialManager;

/// Bootloader 连接状态
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum BootloaderState {
    Disconnected,
    Syncing,
    Connected,
    Busy,
}

/// Bootloader 芯片信息
#[derive(Debug, Clone, Serialize)]
pub struct ChipInfo {
    pub bootloader_version: String,
    pub pid: u16,
    pub chip_name: String,
    pub supported_commands: Vec<u8>,
}

/// Bootloader 协议管理器
pub struct Bootloader {
    state: std::sync::Mutex<BootloaderState>,
    chip_info: std::sync::Mutex<Option<ChipInfo>>,
}

impl Bootloader {
    pub fn new() -> Self {
        Self {
            state: std::sync::Mutex::new(BootloaderState::Disconnected),
            chip_info: std::sync::Mutex::new(None),
        }
    }

    /// 获取当前状态
    pub fn state(&self) -> BootloaderState {
        self.state.lock().unwrap().clone()
    }

    /// 获取芯片信息
    pub fn chip_info(&self) -> Option<ChipInfo> {
        self.chip_info.lock().unwrap().clone()
    }

    /// 检查是否支持指定命令
    pub fn supports_command(&self, cmd: u8) -> bool {
        self.chip_info
            .lock()
            .unwrap()
            .as_ref()
            .map(|info| info.supported_commands.contains(&cmd))
            .unwrap_or(false)
    }

    /// 是否使用 Extended Erase（0x44）
    pub fn uses_extended_erase(&self) -> bool {
        self.supports_command(CMD_EXTENDED_ERASE)
    }

    /// 连接到 bootloader：进入 ISP → 同步 → 获取信息
    ///
    /// isp_mode: "rts_reset" | "dtr_reset" | "none"
    pub fn connect(&self, serial: &SerialManager, isp_mode: &str) -> AppResult<ChipInfo> {
        if !serial.is_open() {
            return Err(AppError::NotConnected);
        }

        *self.state.lock().unwrap() = BootloaderState::Syncing;

        // 通过 DTR/RTS 自动复位进入 ISP Bootloader
        if let Err(e) = serial.enter_isp_mode(isp_mode) {
            log::warn!("DTR/RTS 进入 ISP 模式失败（可忽略，手动模式可继续）: {}", e);
        }

        // 同步：最多重试 3 次
        let mut last_err = None;
        for attempt in 1..=3 {
            match cmd_sync(serial) {
                Ok(()) => {
                    last_err = None;
                    break;
                }
                Err(e) => {
                    log::warn!("同步尝试 {}/3 失败: {}", attempt, e);
                    last_err = Some(e);
                    // 重试前短暂延时
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    let _ = serial.flush();
                }
            }
        }
        if let Some(e) = last_err {
            *self.state.lock().unwrap() = BootloaderState::Disconnected;
            return Err(AppError::Protocol(format!("同步失败（已重试 3 次）: {}", e)));
        }

        // Get 命令
        let get_resp = match cmd_get(serial) {
            Ok(r) => r,
            Err(e) => {
                *self.state.lock().unwrap() = BootloaderState::Disconnected;
                return Err(AppError::Protocol(format!("Get 命令失败: {}", e)));
            }
        };

        // Get ID 命令
        let id_resp = match cmd_get_id(serial) {
            Ok(r) => r,
            Err(e) => {
                *self.state.lock().unwrap() = BootloaderState::Disconnected;
                return Err(AppError::Protocol(format!("Get ID 命令失败: {}", e)));
            }
        };

        let info = ChipInfo {
            bootloader_version: format!(
                "{}.{}",
                get_resp.bootloader_version >> 4,
                get_resp.bootloader_version & 0x0F
            ),
            pid: id_resp.pid,
            chip_name: id_resp.chip_name,
            supported_commands: get_resp.supported_commands,
        };

        *self.chip_info.lock().unwrap() = Some(info.clone());
        *self.state.lock().unwrap() = BootloaderState::Connected;

        Ok(info)
    }

    /// 断开连接
    pub fn disconnect(&self) {
        *self.state.lock().unwrap() = BootloaderState::Disconnected;
        *self.chip_info.lock().unwrap() = None;
    }

    /// 设置忙碌状态
    pub fn set_busy(&self) {
        *self.state.lock().unwrap() = BootloaderState::Busy;
    }

    /// 恢复已连接状态
    pub fn set_connected(&self) {
        *self.state.lock().unwrap() = BootloaderState::Connected;
    }
}
