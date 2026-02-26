/// 帧编码与校验和工具

/// 计算 XOR 校验和（所有字节异或）
pub fn xor_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &b| acc ^ b)
}

/// 编码命令帧：命令字节 + 补码
pub fn encode_command(cmd: u8) -> [u8; 2] {
    [cmd, !cmd]
}

/// 编码地址帧：4 字节大端地址 + XOR 校验和
pub fn encode_address(addr: u32) -> [u8; 5] {
    let bytes = addr.to_be_bytes();
    let checksum = xor_checksum(&bytes);
    [bytes[0], bytes[1], bytes[2], bytes[3], checksum]
}

/// 编码写入数据帧：N(长度-1) + data + 校验和
/// 校验和 = N ^ data[0] ^ data[1] ^ ... ^ data[n]
pub fn encode_write_data(data: &[u8]) -> Vec<u8> {
    let n = (data.len() - 1) as u8;
    let mut checksum = n;
    for &b in data {
        checksum ^= b;
    }
    let mut frame = Vec::with_capacity(data.len() + 2);
    frame.push(n);
    frame.extend_from_slice(data);
    frame.push(checksum);
    frame
}

/// 编码擦除页码帧（标准擦除，CMD 0x43）
/// pages 为空时执行全片擦除 (0xFF + 0x00)
pub fn encode_erase_pages(pages: &[u8]) -> Vec<u8> {
    if pages.is_empty() {
        // 全片擦除
        return vec![0xFF, 0x00];
    }
    let n = (pages.len() - 1) as u8;
    let mut checksum = n;
    for &p in pages {
        checksum ^= p;
    }
    let mut frame = Vec::with_capacity(pages.len() + 2);
    frame.push(n);
    frame.extend_from_slice(pages);
    frame.push(checksum);
    frame
}

/// 编码扩展擦除帧（Extended Erase，CMD 0x44）
/// special: 全片擦除 0xFFFF / Bank1 0xFFFE / Bank2 0xFFFD
pub fn encode_extended_erase_special(special: u16) -> Vec<u8> {
    let high = (special >> 8) as u8;
    let low = (special & 0xFF) as u8;
    let checksum = high ^ low;
    vec![high, low, checksum]
}

/// 编码扩展擦除帧（按页擦除）
pub fn encode_extended_erase_pages(pages: &[u16]) -> Vec<u8> {
    let n = (pages.len() - 1) as u16;
    let n_high = (n >> 8) as u8;
    let n_low = (n & 0xFF) as u8;
    let mut checksum = n_high ^ n_low;

    let mut frame = Vec::with_capacity(pages.len() * 2 + 3);
    frame.push(n_high);
    frame.push(n_low);
    for &page in pages {
        let ph = (page >> 8) as u8;
        let pl = (page & 0xFF) as u8;
        checksum ^= ph;
        checksum ^= pl;
        frame.push(ph);
        frame.push(pl);
    }
    frame.push(checksum);
    frame
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_checksum() {
        assert_eq!(xor_checksum(&[0x00, 0x01, 0x02, 0x03]), 0x00);
        assert_eq!(xor_checksum(&[0xFF]), 0xFF);
    }

    #[test]
    fn test_encode_command() {
        assert_eq!(encode_command(0x00), [0x00, 0xFF]);
        assert_eq!(encode_command(0x11), [0x11, 0xEE]);
    }

    #[test]
    fn test_encode_address() {
        let addr = encode_address(0x08000000);
        assert_eq!(addr[0..4], [0x08, 0x00, 0x00, 0x00]);
        assert_eq!(addr[4], 0x08); // XOR checksum
    }

    #[test]
    fn test_encode_write_data() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let frame = encode_write_data(&data);
        assert_eq!(frame[0], 3); // N = len-1
        assert_eq!(&frame[1..5], &[0x01, 0x02, 0x03, 0x04]);
        // checksum = 3 ^ 1 ^ 2 ^ 3 ^ 4 = 7
        assert_eq!(frame[5], 3 ^ 0x01 ^ 0x02 ^ 0x03 ^ 0x04);
    }
}
