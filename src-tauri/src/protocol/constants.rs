/// AN3155 协议常量定义

// 同步字节
pub const SYNC_BYTE: u8 = 0x7F;

// 应答
pub const ACK: u8 = 0x79;
pub const NACK: u8 = 0x1F;

// Bootloader 命令码
pub const CMD_GET: u8 = 0x00;
pub const CMD_GET_VERSION: u8 = 0x01;
pub const CMD_GET_ID: u8 = 0x02;
pub const CMD_READ_MEMORY: u8 = 0x11;
pub const CMD_GO: u8 = 0x21;
pub const CMD_WRITE_MEMORY: u8 = 0x31;
pub const CMD_ERASE: u8 = 0x43;
pub const CMD_EXTENDED_ERASE: u8 = 0x44;
pub const CMD_WRITE_PROTECT: u8 = 0x63;
pub const CMD_WRITE_UNPROTECT: u8 = 0x73;
pub const CMD_READOUT_PROTECT: u8 = 0x82;
pub const CMD_READOUT_UNPROTECT: u8 = 0x92;

// 擦除特殊页码（Extended Erase）
pub const ERASE_GLOBAL: u16 = 0xFFFF;
pub const ERASE_BANK1: u16 = 0xFFFE;
pub const ERASE_BANK2: u16 = 0xFFFD;

// 超时（毫秒）
pub const TIMEOUT_SYNC: u64 = 3000;
pub const TIMEOUT_ACK: u64 = 1000;
pub const TIMEOUT_ERASE: u64 = 30000;
pub const TIMEOUT_WRITE: u64 = 2000;
pub const TIMEOUT_READ: u64 = 2000;
pub const TIMEOUT_PROTECT: u64 = 10000;

// 最大单次写入块大小
pub const MAX_WRITE_BLOCK: usize = 256;

// 最大单次读取大小
pub const MAX_READ_BLOCK: usize = 256;

// STM32 默认 Flash 起始地址
pub const DEFAULT_FLASH_ADDRESS: u32 = 0x0800_0000;

/// 根据芯片 PID 查找芯片名称
pub fn chip_name_from_pid(pid: u16) -> &'static str {
    match pid {
        0x410 => "STM32F10xxx Medium-density",
        0x411 => "STM32F2xx",
        0x412 => "STM32F10xxx Low-density",
        0x413 => "STM32F40x/41x",
        0x414 => "STM32F10xxx High-density",
        0x415 => "STM32L47x/48x",
        0x416 => "STM32L1xx Medium-density",
        0x417 => "STM32L0xx Cat.3",
        0x418 => "STM32F105xx/107xx",
        0x419 => "STM32F42x/43x",
        0x420 => "STM32F10xxx Medium-density VL",
        0x421 => "STM32F446xx",
        0x422 => "STM32F30x/31x/32x",
        0x423 => "STM32F401xB/C",
        0x425 => "STM32L0xx Cat.2",
        0x427 => "STM32L1xx Medium-density Plus",
        0x428 => "STM32F10xxx High-density VL",
        0x429 => "STM32L1xx Cat.2",
        0x430 => "STM32F10xxx XL-density",
        0x431 => "STM32F411xx",
        0x432 => "STM32F37x",
        0x433 => "STM32F401xD/E",
        0x434 => "STM32F469/479",
        0x435 => "STM32L43x/44x",
        0x436 => "STM32L1xx High-density",
        0x437 => "STM32L1xx Medium-density Plus",
        0x438 => "STM32F334",
        0x439 => "STM32F30x/31x/32x",
        0x440 => "STM32F05x",
        0x441 => "STM32F41x",
        0x442 => "STM32F030x8",
        0x443 => "STM32F09x",
        0x444 => "STM32F03xx",
        0x445 => "STM32F04x",
        0x446 => "STM32F303xD/E",
        0x447 => "STM32L07x/08x",
        0x448 => "STM32F07x",
        0x449 => "STM32F74x/75x",
        0x450 => "STM32H74x/75x",
        0x451 => "STM32F76x/77x",
        0x452 => "STM32F72x/73x",
        0x457 => "STM32L01x/02x",
        0x458 => "STM32F410",
        0x460 => "STM32G07x/08x",
        0x461 => "STM32L496x/4A6x",
        0x462 => "STM32L45x/46x",
        0x463 => "STM32F413/423",
        0x464 => "STM32L41x/42x",
        0x466 => "STM32G03x/04x",
        0x467 => "STM32G0B1/C1",
        0x468 => "STM32G43x/44x",
        0x469 => "STM32G47x/48x",
        0x470 => "STM32L4Rx/4Sx",
        0x471 => "STM32L4P5/Q5",
        0x472 => "STM32L5x",
        0x473 => "STM32U5x",
        0x479 => "STM32G49x/4Ax",
        0x480 => "STM32H7A3/B3/B0",
        0x483 => "STM32H72x/73x",
        _ => "Unknown STM32",
    }
}
