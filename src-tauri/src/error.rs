use serde::Serialize;

/// 统一错误类型，覆盖所有可能的错误场景
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("串口错误: {0}")]
    Serial(String),

    #[error("协议错误: {0}")]
    Protocol(String),

    #[error("通信超时: {0}")]
    Timeout(String),

    #[error("设备返回 NACK")]
    Nack,

    #[error("校验和错误: 期望 {expected:#04X}, 实际 {actual:#04X}")]
    Checksum { expected: u8, actual: u8 },

    #[error("固件文件错误: {0}")]
    Firmware(String),

    #[error("Flash 操作错误: {0}")]
    Flash(String),

    #[error("设备未连接")]
    NotConnected,

    #[error("操作已取消")]
    Cancelled,

    #[error("IO 错误: {0}")]
    Io(String),
}

/// 为 Tauri IPC 序列化实现，将错误转换为字符串
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<serialport::Error> for AppError {
    fn from(e: serialport::Error) -> Self {
        AppError::Serial(e.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        if e.kind() == std::io::ErrorKind::TimedOut {
            AppError::Timeout(e.to_string())
        } else {
            AppError::Io(e.to_string())
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
