use serde::Serialize;

use crate::error::{AppError, AppResult};
use crate::protocol::constants::*;
use crate::protocol::frame::*;
use crate::serial::manager::SerialManager;

/// Get 命令响应
#[derive(Debug, Clone, Serialize)]
pub struct GetResponse {
    pub bootloader_version: u8,
    pub supported_commands: Vec<u8>,
}

/// Get Version 命令响应
#[derive(Debug, Clone, Serialize)]
pub struct GetVersionResponse {
    pub version: u8,
    pub option1: u8,
    pub option2: u8,
}

/// Get ID 命令响应
#[derive(Debug, Clone, Serialize)]
pub struct GetIdResponse {
    pub pid: u16,
    pub chip_name: String,
}

/// 等待 ACK 响应
fn wait_ack(serial: &SerialManager) -> AppResult<()> {
    let byte = serial.read_byte()?;
    match byte {
        ACK => Ok(()),
        NACK => Err(AppError::Nack),
        other => Err(AppError::Protocol(format!(
            "期望 ACK(0x79) 或 NACK(0x1F)，收到 0x{:02X}",
            other
        ))),
    }
}

/// 同步命令：发送 0x7F，等待 ACK
pub fn cmd_sync(serial: &SerialManager) -> AppResult<()> {
    serial.set_timeout(TIMEOUT_SYNC)?;
    serial.flush()?;
    serial.write(&[SYNC_BYTE])?;
    wait_ack(serial)?;
    serial.set_timeout(TIMEOUT_ACK)?;
    Ok(())
}

/// Get 命令 (0x00)：获取 bootloader 版本和支持的命令列表
pub fn cmd_get(serial: &SerialManager) -> AppResult<GetResponse> {
    serial.write(&encode_command(CMD_GET))?;
    wait_ack(serial)?;

    // 读取字节数 N
    let n = serial.read_byte()? as usize;
    // 读取 bootloader 版本
    let version = serial.read_byte()?;
    // 读取 N 个支持的命令码
    let commands = serial.read_exact(n)?;
    // 等待结束 ACK
    wait_ack(serial)?;

    Ok(GetResponse {
        bootloader_version: version,
        supported_commands: commands,
    })
}

/// Get Version 命令 (0x01)
pub fn cmd_get_version(serial: &SerialManager) -> AppResult<GetVersionResponse> {
    serial.write(&encode_command(CMD_GET_VERSION))?;
    wait_ack(serial)?;

    let version = serial.read_byte()?;
    let option1 = serial.read_byte()?;
    let option2 = serial.read_byte()?;
    wait_ack(serial)?;

    Ok(GetVersionResponse {
        version,
        option1,
        option2,
    })
}

/// Get ID 命令 (0x02)：获取芯片 Product ID
pub fn cmd_get_id(serial: &SerialManager) -> AppResult<GetIdResponse> {
    serial.write(&encode_command(CMD_GET_ID))?;
    wait_ack(serial)?;

    // N = 接下来的字节数 - 1
    let n = serial.read_byte()? as usize;
    let id_bytes = serial.read_exact(n + 1)?;
    wait_ack(serial)?;

    let pid = if id_bytes.len() >= 2 {
        ((id_bytes[0] as u16) << 8) | (id_bytes[1] as u16)
    } else {
        id_bytes[0] as u16
    };

    Ok(GetIdResponse {
        pid,
        chip_name: chip_name_from_pid(pid).to_string(),
    })
}

/// Read Memory 命令 (0x11)：从指定地址读取数据
pub fn cmd_read_memory(serial: &SerialManager, addr: u32, len: usize) -> AppResult<Vec<u8>> {
    if len == 0 || len > MAX_READ_BLOCK {
        return Err(AppError::Protocol(format!(
            "读取长度必须在 1..={} 之间，当前 {}",
            MAX_READ_BLOCK, len
        )));
    }

    serial.set_timeout(TIMEOUT_READ)?;
    serial.write(&encode_command(CMD_READ_MEMORY))?;
    wait_ack(serial)?;

    // 发送地址
    serial.write(&encode_address(addr))?;
    wait_ack(serial)?;

    // 发送读取长度: N-1 + checksum
    let n = (len - 1) as u8;
    serial.write(&[n, !n])?;
    wait_ack(serial)?;

    // 读取数据
    let data = serial.read_exact(len)?;
    serial.set_timeout(TIMEOUT_ACK)?;
    Ok(data)
}

/// Go 命令 (0x21)：跳转到指定地址执行
pub fn cmd_go(serial: &SerialManager, addr: u32) -> AppResult<()> {
    serial.write(&encode_command(CMD_GO))?;
    wait_ack(serial)?;

    serial.write(&encode_address(addr))?;
    wait_ack(serial)?;

    Ok(())
}

/// Write Memory 命令 (0x31)：向指定地址写入数据（最大 256 字节）
pub fn cmd_write_memory(serial: &SerialManager, addr: u32, data: &[u8]) -> AppResult<()> {
    if data.is_empty() || data.len() > MAX_WRITE_BLOCK {
        return Err(AppError::Protocol(format!(
            "写入长度必须在 1..={} 之间，当前 {}",
            MAX_WRITE_BLOCK,
            data.len()
        )));
    }

    serial.set_timeout(TIMEOUT_WRITE)?;
    serial.write(&encode_command(CMD_WRITE_MEMORY))?;
    wait_ack(serial)?;

    // 发送地址
    serial.write(&encode_address(addr))?;
    wait_ack(serial)?;

    // 数据需要 4 字节对齐，不足补 0xFF
    let mut aligned_data = data.to_vec();
    while aligned_data.len() % 4 != 0 {
        aligned_data.push(0xFF);
    }

    // 发送数据帧
    serial.write(&encode_write_data(&aligned_data))?;
    wait_ack(serial)?;

    serial.set_timeout(TIMEOUT_ACK)?;
    Ok(())
}

/// Erase 命令 (0x43)：标准擦除
pub fn cmd_erase(serial: &SerialManager, pages: &[u8]) -> AppResult<()> {
    serial.set_timeout(TIMEOUT_ERASE)?;
    serial.write(&encode_command(CMD_ERASE))?;
    wait_ack(serial)?;

    serial.write(&encode_erase_pages(pages))?;
    wait_ack(serial)?;

    serial.set_timeout(TIMEOUT_ACK)?;
    Ok(())
}

/// Extended Erase 命令 (0x44)：全片擦除
pub fn cmd_extended_erase_global(serial: &SerialManager) -> AppResult<()> {
    serial.set_timeout(TIMEOUT_ERASE)?;
    serial.write(&encode_command(CMD_EXTENDED_ERASE))?;
    wait_ack(serial)?;

    serial.write(&encode_extended_erase_special(ERASE_GLOBAL))?;
    wait_ack(serial)?;

    serial.set_timeout(TIMEOUT_ACK)?;
    Ok(())
}

/// Extended Erase 命令 (0x44)：按页擦除
pub fn cmd_extended_erase_pages(serial: &SerialManager, pages: &[u16]) -> AppResult<()> {
    serial.set_timeout(TIMEOUT_ERASE)?;
    serial.write(&encode_command(CMD_EXTENDED_ERASE))?;
    wait_ack(serial)?;

    serial.write(&encode_extended_erase_pages(pages))?;
    wait_ack(serial)?;

    serial.set_timeout(TIMEOUT_ACK)?;
    Ok(())
}

/// Write Protect 命令 (0x63)
pub fn cmd_write_protect(serial: &SerialManager, sectors: &[u8]) -> AppResult<()> {
    serial.set_timeout(TIMEOUT_PROTECT)?;
    serial.write(&encode_command(CMD_WRITE_PROTECT))?;
    wait_ack(serial)?;

    // N-1 + sectors + checksum
    let n = (sectors.len() - 1) as u8;
    let mut checksum = n;
    for &s in sectors {
        checksum ^= s;
    }
    let mut frame = Vec::with_capacity(sectors.len() + 2);
    frame.push(n);
    frame.extend_from_slice(sectors);
    frame.push(checksum);
    serial.write(&frame)?;
    wait_ack(serial)?;

    serial.set_timeout(TIMEOUT_ACK)?;
    Ok(())
}

/// Write Unprotect 命令 (0x73)
pub fn cmd_write_unprotect(serial: &SerialManager) -> AppResult<()> {
    serial.set_timeout(TIMEOUT_PROTECT)?;
    serial.write(&encode_command(CMD_WRITE_UNPROTECT))?;
    wait_ack(serial)?;
    // 等待第二个 ACK（设备执行解保护并复位）
    wait_ack(serial)?;
    serial.set_timeout(TIMEOUT_ACK)?;
    Ok(())
}

/// Readout Protect 命令 (0x82)
pub fn cmd_readout_protect(serial: &SerialManager) -> AppResult<()> {
    serial.set_timeout(TIMEOUT_PROTECT)?;
    serial.write(&encode_command(CMD_READOUT_PROTECT))?;
    wait_ack(serial)?;
    wait_ack(serial)?;
    serial.set_timeout(TIMEOUT_ACK)?;
    Ok(())
}

/// Readout Unprotect 命令 (0x92)
pub fn cmd_readout_unprotect(serial: &SerialManager) -> AppResult<()> {
    serial.set_timeout(TIMEOUT_PROTECT)?;
    serial.write(&encode_command(CMD_READOUT_UNPROTECT))?;
    wait_ack(serial)?;
    // 解除读保护会触发全片擦除
    wait_ack(serial)?;
    serial.set_timeout(TIMEOUT_ACK)?;
    Ok(())
}
