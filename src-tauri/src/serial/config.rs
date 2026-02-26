use serde::{Deserialize, Serialize};

/// 串口配置，AN3155 默认要求 8E1（8 数据位、偶校验、1 停止位）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialConfig {
    pub baud_rate: u32,
    pub data_bits: u8,
    pub parity: String,
    pub stop_bits: u8,
    pub timeout_ms: u64,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            baud_rate: 230400,
            data_bits: 8,
            parity: "even".to_string(),
            stop_bits: 1,
            timeout_ms: 1000,
        }
    }
}

impl SerialConfig {
    /// 将 parity 字符串转换为 serialport 枚举
    pub fn parity_enum(&self) -> serialport::Parity {
        match self.parity.to_lowercase().as_str() {
            "none" => serialport::Parity::None,
            "odd" => serialport::Parity::Odd,
            _ => serialport::Parity::Even,
        }
    }

    /// 将 data_bits 转换为 serialport 枚举
    pub fn data_bits_enum(&self) -> serialport::DataBits {
        match self.data_bits {
            5 => serialport::DataBits::Five,
            6 => serialport::DataBits::Six,
            7 => serialport::DataBits::Seven,
            _ => serialport::DataBits::Eight,
        }
    }

    /// 将 stop_bits 转换为 serialport 枚举
    pub fn stop_bits_enum(&self) -> serialport::StopBits {
        match self.stop_bits {
            2 => serialport::StopBits::Two,
            _ => serialport::StopBits::One,
        }
    }
}
