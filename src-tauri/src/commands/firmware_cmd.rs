use std::sync::Mutex;

use serde::Serialize;

use crate::error::{AppError, AppResult};
use crate::firmware::bin_loader;
use crate::firmware::hex_parser;
use crate::firmware::memory_image::MemoryImage;

/// 固件管理器，持有当前加载的固件映像
pub struct FirmwareManager {
    image: Mutex<Option<MemoryImage>>,
    file_path: Mutex<String>,
    file_format: Mutex<String>,
}

impl FirmwareManager {
    pub fn new() -> Self {
        Self {
            image: Mutex::new(None),
            file_path: Mutex::new(String::new()),
            file_format: Mutex::new(String::new()),
        }
    }

    pub fn image(&self) -> Option<MemoryImage> {
        self.image.lock().unwrap().clone()
    }

    pub fn set_image(&self, image: MemoryImage, path: &str, format: &str) {
        *self.image.lock().unwrap() = Some(image);
        *self.file_path.lock().unwrap() = path.to_string();
        *self.file_format.lock().unwrap() = format.to_string();
    }

    pub fn clear(&self) {
        *self.image.lock().unwrap() = None;
        *self.file_path.lock().unwrap() = String::new();
        *self.file_format.lock().unwrap() = String::new();
    }
}

/// 固件信息
#[derive(Debug, Clone, Serialize)]
pub struct FirmwareInfo {
    pub file_path: String,
    pub file_format: String,
    pub total_size: usize,
    pub start_address: Option<u32>,
    pub end_address: Option<u32>,
    pub entry_point: Option<u32>,
    pub segment_count: usize,
}

/// 加载固件文件（自动检测格式）
#[tauri::command]
pub fn load_firmware(
    firmware: tauri::State<'_, FirmwareManager>,
    file_path: String,
    bin_start_address: Option<u32>,
) -> AppResult<FirmwareInfo> {
    let content = std::fs::read(&file_path)
        .map_err(|e| AppError::Firmware(format!("无法读取文件: {}", e)))?;

    let ext = file_path
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase();

    let (image, format) = match ext.as_str() {
        "hex" | "ihex" => {
            let text = String::from_utf8(content)
                .map_err(|e| AppError::Firmware(format!("文件编码错误: {}", e)))?;
            (hex_parser::parse_hex(&text)?, "Intel HEX")
        }
        "bin" => (bin_loader::load_bin(&content, bin_start_address)?, "Binary"),
        _ => {
            // 尝试自动检测：如果以 ':' 开头则为 HEX
            let text = String::from_utf8(content.clone());
            if let Ok(text) = text {
                if text.trim_start().starts_with(':') {
                    (hex_parser::parse_hex(&text)?, "Intel HEX")
                } else {
                    (bin_loader::load_bin(&content, bin_start_address)?, "Binary")
                }
            } else {
                (bin_loader::load_bin(&content, bin_start_address)?, "Binary")
            }
        }
    };

    let info = FirmwareInfo {
        file_path: file_path.clone(),
        file_format: format.to_string(),
        total_size: image.total_size(),
        start_address: image.start_address(),
        end_address: image.end_address(),
        entry_point: image.entry_point,
        segment_count: image.segments.len(),
    };

    firmware.set_image(image, &file_path, format);

    Ok(info)
}

/// 获取当前固件信息
#[tauri::command]
pub fn get_firmware_info(firmware: tauri::State<'_, FirmwareManager>) -> Option<FirmwareInfo> {
    let image = firmware.image()?;
    let path = firmware.file_path.lock().unwrap().clone();
    let format = firmware.file_format.lock().unwrap().clone();

    Some(FirmwareInfo {
        file_path: path,
        file_format: format,
        total_size: image.total_size(),
        start_address: image.start_address(),
        end_address: image.end_address(),
        entry_point: image.entry_point,
        segment_count: image.segments.len(),
    })
}

/// 清除已加载的固件
#[tauri::command]
pub fn clear_firmware(firmware: tauri::State<'_, FirmwareManager>) -> AppResult<()> {
    firmware.clear();
    Ok(())
}
