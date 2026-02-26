use std::sync::Mutex;
use std::time::Duration;

use serde::Serialize;
use serialport::SerialPort;

use crate::error::{AppError, AppResult};
use crate::serial::config::SerialConfig;

/// 串口端口信息，序列化后传递给前端
#[derive(Debug, Clone, Serialize)]
pub struct PortInfo {
    pub name: String,
    pub description: String,
}

/// 串口管理器，使用 Mutex 保证线程安全
pub struct SerialManager {
    port: Mutex<Option<Box<dyn SerialPort>>>,
    port_name: Mutex<String>,
}

impl SerialManager {
    pub fn new() -> Self {
        Self {
            port: Mutex::new(None),
            port_name: Mutex::new(String::new()),
        }
    }

    /// 列举系统可用串口
    pub fn list_ports() -> AppResult<Vec<PortInfo>> {
        let ports = serialport::available_ports().map_err(|e| AppError::Serial(e.to_string()))?;
        Ok(ports
            .into_iter()
            .map(|p| {
                let description = match &p.port_type {
                    serialport::SerialPortType::UsbPort(usb) => {
                        format!(
                            "{} ({})",
                            usb.product.as_deref().unwrap_or("USB Serial"),
                            usb.manufacturer.as_deref().unwrap_or("Unknown")
                        )
                    }
                    serialport::SerialPortType::PciPort => "PCI Serial".to_string(),
                    serialport::SerialPortType::BluetoothPort => "Bluetooth Serial".to_string(),
                    _ => "Serial Port".to_string(),
                };
                PortInfo {
                    name: p.port_name,
                    description,
                }
            })
            .collect())
    }

    /// 打开串口
    pub fn open(&self, port_name: &str, config: &SerialConfig) -> AppResult<()> {
        // 先关闭已打开的串口
        self.close()?;

        let port = serialport::new(port_name, config.baud_rate)
            .data_bits(config.data_bits_enum())
            .parity(config.parity_enum())
            .stop_bits(config.stop_bits_enum())
            .timeout(Duration::from_millis(config.timeout_ms))
            .open()
            .map_err(|e| AppError::Serial(format!("无法打开串口 {}: {}", port_name, e)))?;

        *self.port.lock().unwrap() = Some(port);
        *self.port_name.lock().unwrap() = port_name.to_string();
        Ok(())
    }

    /// 关闭串口
    pub fn close(&self) -> AppResult<()> {
        let mut port = self.port.lock().unwrap();
        if port.is_some() {
            *port = None;
            *self.port_name.lock().unwrap() = String::new();
        }
        Ok(())
    }

    /// 检查串口是否已打开
    pub fn is_open(&self) -> bool {
        self.port.lock().unwrap().is_some()
    }

    /// 获取当前打开的串口名称
    pub fn port_name(&self) -> String {
        self.port_name.lock().unwrap().clone()
    }

    /// 写入数据
    pub fn write(&self, data: &[u8]) -> AppResult<()> {
        let mut port = self.port.lock().unwrap();
        let port = port.as_mut().ok_or(AppError::NotConnected)?;
        port.write_all(data)?;
        port.flush()?;
        Ok(())
    }

    /// 读取指定长度数据
    pub fn read_exact(&self, len: usize) -> AppResult<Vec<u8>> {
        let mut port = self.port.lock().unwrap();
        let port = port.as_mut().ok_or(AppError::NotConnected)?;
        let mut buf = vec![0u8; len];
        port.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// 读取单个字节
    pub fn read_byte(&self) -> AppResult<u8> {
        let data = self.read_exact(1)?;
        Ok(data[0])
    }

    /// 读取可用数据（非阻塞）
    pub fn read_available(&self) -> AppResult<Vec<u8>> {
        let mut port = self.port.lock().unwrap();
        let port = port.as_mut().ok_or(AppError::NotConnected)?;
        let available = port.bytes_to_read().unwrap_or(0) as usize;
        if available == 0 {
            return Ok(vec![]);
        }
        let mut buf = vec![0u8; available];
        let n = port.read(&mut buf).unwrap_or(0);
        buf.truncate(n);
        Ok(buf)
    }

    /// 清空输入输出缓冲区
    pub fn flush(&self) -> AppResult<()> {
        let mut port = self.port.lock().unwrap();
        if let Some(port) = port.as_mut() {
            port.clear(serialport::ClearBuffer::All)
                .map_err(|e| AppError::Serial(e.to_string()))?;
        }
        Ok(())
    }

    /// 设置超时
    pub fn set_timeout(&self, timeout_ms: u64) -> AppResult<()> {
        let mut port = self.port.lock().unwrap();
        let port = port.as_mut().ok_or(AppError::NotConnected)?;
        port.set_timeout(Duration::from_millis(timeout_ms))
            .map_err(|e| AppError::Serial(e.to_string()))?;
        Ok(())
    }

    /// 设置 DTR 信号电平
    pub fn set_dtr(&self, level: bool) -> AppResult<()> {
        let mut port = self.port.lock().unwrap();
        let port = port.as_mut().ok_or(AppError::NotConnected)?;
        port.write_data_terminal_ready(level)
            .map_err(|e| AppError::Serial(format!("设置 DTR 失败: {}", e)))?;
        Ok(())
    }

    /// 设置 RTS 信号电平
    pub fn set_rts(&self, level: bool) -> AppResult<()> {
        let mut port = self.port.lock().unwrap();
        let port = port.as_mut().ok_or(AppError::NotConnected)?;
        port.write_request_to_send(level)
            .map_err(|e| AppError::Serial(format!("设置 RTS 失败: {}", e)))?;
        Ok(())
    }

    /// 通过 DTR/RTS 信号控制 STM32 进入 ISP Bootloader 模式
    ///
    /// 支持两种硬件接线模式:
    ///
    /// **模式 A — "rts_reset"（RTS 复位，DTR 控制 BOOT0）:**
    ///   RTS HIGH → NRST = LOW（复位）
    ///   DTR LOW  → BOOT0 = HIGH（进入 Bootloader）
    ///
    /// **模式 B — "dtr_reset"（DTR 复位，RTS 控制 BOOT0）:**
    ///   DTR LOW  → NRST = LOW（复位）
    ///   RTS HIGH → BOOT0 = HIGH（进入 Bootloader）
    pub fn enter_isp_mode(&self, mode: &str) -> AppResult<()> {
        match mode {
            "rts_reset" => {
                // 模式 A: RTS 复位, DTR 控制 BOOT0
                self.set_dtr(false)?;   // BOOT0 = HIGH
                self.set_rts(true)?;    // NRST = LOW（复位）
                std::thread::sleep(Duration::from_millis(100));
                self.set_rts(false)?;   // NRST = HIGH（释放复位）
                std::thread::sleep(Duration::from_millis(50));
            }
            "dtr_reset" => {
                // 模式 B: DTR 复位, RTS 控制 BOOT0
                self.set_rts(true)?;    // BOOT0 = HIGH
                self.set_dtr(false)?;   // NRST = LOW（复位）
                std::thread::sleep(Duration::from_millis(100));
                self.set_dtr(true)?;    // NRST = HIGH（释放复位）
                std::thread::sleep(Duration::from_millis(50));
            }
            _ => {
                // "none" — 不自动控制，用户手动进入 ISP
                return Ok(());
            }
        }

        // 清空缓冲区（复位期间可能有杂波）
        self.flush()?;
        Ok(())
    }

    /// 释放 DTR/RTS 信号，让 STM32 正常运行用户程序
    pub fn exit_isp_mode(&self, mode: &str) -> AppResult<()> {
        match mode {
            "rts_reset" => {
                self.set_dtr(true)?;    // BOOT0 = LOW（正常启动）
                self.set_rts(true)?;    // NRST = LOW（复位）
                std::thread::sleep(Duration::from_millis(100));
                self.set_rts(false)?;   // NRST = HIGH（释放复位）
            }
            "dtr_reset" => {
                self.set_rts(false)?;   // BOOT0 = LOW（正常启动）
                self.set_dtr(false)?;   // NRST = LOW（复位）
                std::thread::sleep(Duration::from_millis(100));
                self.set_dtr(true)?;    // NRST = HIGH（释放复位）
            }
            _ => return Ok(()),
        }
        std::thread::sleep(Duration::from_millis(50));
        Ok(())
    }
}
