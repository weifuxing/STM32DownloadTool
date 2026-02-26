use crate::error::{AppError, AppResult};
use crate::firmware::memory_image::MemoryImage;

/// 解析 Intel HEX 格式文件内容
pub fn parse_hex(content: &str) -> AppResult<MemoryImage> {
    let mut image = MemoryImage::new();
    let mut base_address: u32 = 0;
    let mut line_num = 0u32;

    for line in content.lines() {
        line_num += 1;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if !line.starts_with(':') {
            return Err(AppError::Firmware(format!(
                "第 {} 行: 缺少起始标记 ':'",
                line_num
            )));
        }

        let hex_str = &line[1..];
        if hex_str.len() < 10 || hex_str.len() % 2 != 0 {
            return Err(AppError::Firmware(format!(
                "第 {} 行: 长度无效",
                line_num
            )));
        }

        // 解析为字节数组
        let bytes = hex_to_bytes(hex_str).map_err(|e| {
            AppError::Firmware(format!("第 {} 行: {}", line_num, e))
        })?;

        // 校验和验证：所有字节之和 mod 256 == 0
        let checksum: u8 = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        if checksum != 0 {
            return Err(AppError::Firmware(format!(
                "第 {} 行: 校验和错误",
                line_num
            )));
        }

        let byte_count = bytes[0] as usize;
        let offset = ((bytes[1] as u16) << 8) | (bytes[2] as u16);
        let record_type = bytes[3];
        let data = &bytes[4..4 + byte_count];

        // 验证数据长度
        if bytes.len() != byte_count + 5 {
            return Err(AppError::Firmware(format!(
                "第 {} 行: 数据长度不匹配",
                line_num
            )));
        }

        match record_type {
            // Type 00: Data Record
            0x00 => {
                let address = base_address + offset as u32;
                image.add_data(address, data);
            }
            // Type 01: End of File
            0x01 => {
                break;
            }
            // Type 02: Extended Segment Address
            0x02 => {
                if data.len() >= 2 {
                    base_address = (((data[0] as u32) << 8) | (data[1] as u32)) << 4;
                }
            }
            // Type 03: Start Segment Address
            0x03 => {
                if data.len() >= 4 {
                    let cs = ((data[0] as u32) << 8) | (data[1] as u32);
                    let ip = ((data[2] as u32) << 8) | (data[3] as u32);
                    image.entry_point = Some((cs << 4) + ip);
                }
            }
            // Type 04: Extended Linear Address
            0x04 => {
                if data.len() >= 2 {
                    base_address = (((data[0] as u32) << 8) | (data[1] as u32)) << 16;
                }
            }
            // Type 05: Start Linear Address
            0x05 => {
                if data.len() >= 4 {
                    image.entry_point = Some(
                        ((data[0] as u32) << 24)
                            | ((data[1] as u32) << 16)
                            | ((data[2] as u32) << 8)
                            | (data[3] as u32),
                    );
                }
            }
            _ => {
                // 忽略未知记录类型
            }
        }
    }

    image.merge_segments();

    if image.segments.is_empty() {
        return Err(AppError::Firmware("HEX 文件不包含有效数据".to_string()));
    }

    Ok(image)
}

/// 将十六进制字符串转换为字节数组
fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i + 2];
        let byte = u8::from_str_radix(byte_str, 16)
            .map_err(|_| format!("无效的十六进制字符: '{}'", byte_str))?;
        bytes.push(byte);
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_hex() {
        let hex_content = "\
:020000040800F2
:1000000000100020ED000008F1000008F300000864
:00000001FF
";
        let image = parse_hex(hex_content).unwrap();
        assert_eq!(image.segments.len(), 1);
        assert_eq!(image.segments[0].address, 0x08000000);
        assert_eq!(image.segments[0].data.len(), 16);
    }

    #[test]
    fn test_checksum_error() {
        let hex_content = ":1000000000100020ED000008F1000008F300000800\n:00000001FF\n";
        assert!(parse_hex(hex_content).is_err());
    }
}
