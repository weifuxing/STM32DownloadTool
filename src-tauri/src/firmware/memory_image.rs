use serde::Serialize;

use crate::protocol::constants::MAX_WRITE_BLOCK;

/// 单个内存段（连续的地址+数据区域）
#[derive(Debug, Clone, Serialize)]
pub struct MemorySegment {
    pub address: u32,
    pub data: Vec<u8>,
}

impl MemorySegment {
    pub fn end_address(&self) -> u32 {
        self.address + self.data.len() as u32
    }
}

/// 统一内存映像，支持多段、合并、分块
#[derive(Debug, Clone, Serialize)]
pub struct MemoryImage {
    pub segments: Vec<MemorySegment>,
    pub entry_point: Option<u32>,
}

impl MemoryImage {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            entry_point: None,
        }
    }

    /// 添加数据到指定地址（自动合并相邻段）
    pub fn add_data(&mut self, address: u32, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        // 尝试找到可合并的段
        for seg in &mut self.segments {
            // 数据紧接在现有段之后
            if seg.end_address() == address {
                seg.data.extend_from_slice(data);
                return;
            }
            // 数据在现有段范围内（覆盖写入）
            if address >= seg.address && address < seg.end_address() {
                let offset = (address - seg.address) as usize;
                let end = offset + data.len();
                if end > seg.data.len() {
                    seg.data.resize(end, 0xFF);
                }
                seg.data[offset..offset + data.len()].copy_from_slice(data);
                return;
            }
        }

        // 无法合并，创建新段
        self.segments.push(MemorySegment {
            address,
            data: data.to_vec(),
        });
    }

    /// 合并相邻或重叠的段
    pub fn merge_segments(&mut self) {
        if self.segments.len() < 2 {
            return;
        }

        self.segments.sort_by_key(|s| s.address);

        let mut merged: Vec<MemorySegment> = Vec::new();
        for seg in self.segments.drain(..) {
            if let Some(last) = merged.last_mut() {
                if seg.address <= last.end_address() {
                    // 有重叠或相邻
                    let overlap_end = std::cmp::max(last.end_address(), seg.end_address());
                    let new_len = (overlap_end - last.address) as usize;
                    if new_len > last.data.len() {
                        last.data.resize(new_len, 0xFF);
                    }
                    let offset = (seg.address - last.address) as usize;
                    last.data[offset..offset + seg.data.len()].copy_from_slice(&seg.data);
                } else {
                    merged.push(seg);
                }
            } else {
                merged.push(seg);
            }
        }

        self.segments = merged;
    }

    /// 将内存映像按指定块大小分块（用于写入）
    pub fn chunks(&self, block_size: usize) -> Vec<(u32, Vec<u8>)> {
        let block_size = if block_size == 0 {
            MAX_WRITE_BLOCK
        } else {
            block_size
        };

        let mut result = Vec::new();
        for seg in &self.segments {
            let mut offset = 0usize;
            while offset < seg.data.len() {
                let remaining = seg.data.len() - offset;
                let chunk_len = remaining.min(block_size);
                let addr = seg.address + offset as u32;
                result.push((addr, seg.data[offset..offset + chunk_len].to_vec()));
                offset += chunk_len;
            }
        }
        result
    }

    /// 总数据大小
    pub fn total_size(&self) -> usize {
        self.segments.iter().map(|s| s.data.len()).sum()
    }

    /// 起始地址
    pub fn start_address(&self) -> Option<u32> {
        self.segments.first().map(|s| s.address)
    }

    /// 结束地址
    pub fn end_address(&self) -> Option<u32> {
        self.segments.last().map(|s| s.end_address())
    }

    /// 导出为连续二进制数据（从最低地址到最高地址，空隙填充 0xFF）
    pub fn to_binary(&self) -> Option<(u32, Vec<u8>)> {
        if self.segments.is_empty() {
            return None;
        }

        let start = self.start_address()?;
        let end = self.end_address()?;
        let total_len = (end - start) as usize;
        let mut binary = vec![0xFF; total_len];

        for seg in &self.segments {
            let offset = (seg.address - start) as usize;
            binary[offset..offset + seg.data.len()].copy_from_slice(&seg.data);
        }

        Some((start, binary))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_data_and_merge() {
        let mut img = MemoryImage::new();
        img.add_data(0x08000000, &[1, 2, 3, 4]);
        img.add_data(0x08000004, &[5, 6, 7, 8]);
        assert_eq!(img.segments.len(), 1);
        assert_eq!(img.segments[0].data, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_chunks() {
        let mut img = MemoryImage::new();
        img.add_data(0x08000000, &vec![0xAA; 512]);
        let chunks = img.chunks(256);
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].0, 0x08000000);
        assert_eq!(chunks[1].0, 0x08000100);
    }

    #[test]
    fn test_total_size() {
        let mut img = MemoryImage::new();
        img.add_data(0x08000000, &[1, 2, 3]);
        img.add_data(0x08001000, &[4, 5]);
        assert_eq!(img.total_size(), 5);
    }
}
