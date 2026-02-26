use serde::Serialize;
use tauri::State;

use crate::error::AppResult;
use crate::serial::config::SerialConfig;
use crate::serial::manager::{PortInfo, SerialManager};

/// 串口状态信息
#[derive(Debug, Clone, Serialize)]
pub struct SerialStatus {
    pub connected: bool,
    pub port_name: String,
}

/// 列举可用串口
#[tauri::command]
pub fn list_serial_ports() -> AppResult<Vec<PortInfo>> {
    SerialManager::list_ports()
}

/// 打开串口
#[tauri::command]
pub fn open_serial_port(
    manager: State<'_, SerialManager>,
    port_name: String,
    config: SerialConfig,
) -> AppResult<()> {
    manager.open(&port_name, &config)
}

/// 关闭串口
#[tauri::command]
pub fn close_serial_port(manager: State<'_, SerialManager>) -> AppResult<()> {
    manager.close()
}

/// 获取串口状态
#[tauri::command]
pub fn get_serial_status(manager: State<'_, SerialManager>) -> SerialStatus {
    SerialStatus {
        connected: manager.is_open(),
        port_name: manager.port_name(),
    }
}
