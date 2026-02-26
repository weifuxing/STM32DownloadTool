use crate::error::AppResult;
use crate::firmware::memory_image::MemoryImage;
use crate::protocol::constants::DEFAULT_FLASH_ADDRESS;

/// 加载 BIN 文件，默认起始地址 0x08000000
pub fn load_bin(data: &[u8], start_address: Option<u32>) -> AppResult<MemoryImage> {
    let address = start_address.unwrap_or(DEFAULT_FLASH_ADDRESS);
    let mut image = MemoryImage::new();
    image.add_data(address, data);
    // BIN 文件默认入口点为起始地址
    image.entry_point = Some(address);
    Ok(image)
}
