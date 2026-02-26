use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::error::{AppError, AppResult};
use crate::firmware::memory_image::MemoryImage;
use crate::protocol::bootloader::Bootloader;
use crate::protocol::commands::*;
use crate::protocol::constants::*;
use crate::serial::manager::SerialManager;

/// 取消令牌
#[derive(Clone)]
pub struct CancelToken {
    cancelled: Arc<AtomicBool>,
}

impl CancelToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    pub fn check(&self) -> AppResult<()> {
        if self.is_cancelled() {
            Err(AppError::Cancelled)
        } else {
            Ok(())
        }
    }
}

/// 进度事件
#[derive(Debug, Clone, Serialize)]
pub struct ProgressEvent {
    pub stage: String,
    pub progress: f64,
    pub current: usize,
    pub total: usize,
    pub speed: f64,
}

/// 日志事件
#[derive(Debug, Clone, Serialize)]
pub struct LogEvent {
    pub level: String,
    pub message: String,
    pub timestamp: String,
}

/// 发送进度事件
fn emit_progress(app: &AppHandle, stage: &str, current: usize, total: usize, speed: f64) {
    let progress = if total > 0 {
        (current as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    let _ = app.emit(
        "flash-progress",
        ProgressEvent {
            stage: stage.to_string(),
            progress,
            current,
            total,
            speed,
        },
    );
}

/// 发送日志事件
fn emit_log(app: &AppHandle, level: &str, message: &str) {
    let _ = app.emit(
        "flash-log",
        LogEvent {
            level: level.to_string(),
            message: message.to_string(),
            timestamp: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
        },
    );
}

/// 擦除 Flash
pub fn erase_flash(
    app: &AppHandle,
    serial: &SerialManager,
    bootloader: &Bootloader,
    cancel: &CancelToken,
) -> AppResult<()> {
    cancel.check()?;
    emit_log(app, "info", "开始擦除 Flash...");
    emit_progress(app, "擦除", 0, 1, 0.0);

    if bootloader.uses_extended_erase() {
        emit_log(app, "info", "使用 Extended Erase (0x44) 全片擦除");
        cmd_extended_erase_global(serial)?;
    } else {
        emit_log(app, "info", "使用 Erase (0x43) 全片擦除");
        cmd_erase(serial, &[])?;
    }

    emit_progress(app, "擦除", 1, 1, 0.0);
    emit_log(app, "success", "Flash 擦除完成");
    Ok(())
}

/// 写入固件到 Flash
pub fn write_flash(
    app: &AppHandle,
    serial: &SerialManager,
    image: &MemoryImage,
    cancel: &CancelToken,
) -> AppResult<()> {
    let chunks = image.chunks(MAX_WRITE_BLOCK);
    let total = chunks.len();
    let total_bytes = image.total_size();

    emit_log(
        app,
        "info",
        &format!(
            "开始写入 Flash: {} 字节, {} 个数据块",
            total_bytes, total
        ),
    );

    let start_time = std::time::Instant::now();

    for (i, (addr, data)) in chunks.iter().enumerate() {
        cancel.check()?;

        cmd_write_memory(serial, *addr, data)?;

        let elapsed = start_time.elapsed().as_secs_f64();
        let bytes_written = (i + 1) * MAX_WRITE_BLOCK;
        let speed = if elapsed > 0.0 {
            bytes_written as f64 / elapsed / 1024.0
        } else {
            0.0
        };

        emit_progress(app, "写入", i + 1, total, speed);
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    let speed = total_bytes as f64 / elapsed / 1024.0;
    emit_log(
        app,
        "success",
        &format!(
            "写入完成: {} 字节, 耗时 {:.1}s, 速度 {:.1} KB/s",
            total_bytes, elapsed, speed
        ),
    );

    Ok(())
}

/// 校验 Flash 内容（读回比较）
pub fn verify_flash(
    app: &AppHandle,
    serial: &SerialManager,
    image: &MemoryImage,
    cancel: &CancelToken,
) -> AppResult<()> {
    let chunks = image.chunks(MAX_READ_BLOCK);
    let total = chunks.len();

    emit_log(app, "info", "开始校验 Flash...");

    let start_time = std::time::Instant::now();

    for (i, (addr, data)) in chunks.iter().enumerate() {
        cancel.check()?;

        let read_data = cmd_read_memory(serial, *addr, data.len())?;

        // 逐字节比较
        for (j, (&expected, &actual)) in data.iter().zip(read_data.iter()).enumerate() {
            if expected != actual {
                let error_addr = addr + j as u32;
                return Err(AppError::Flash(format!(
                    "校验失败: 地址 0x{:08X}, 期望 0x{:02X}, 实际 0x{:02X}",
                    error_addr, expected, actual
                )));
            }
        }

        let elapsed = start_time.elapsed().as_secs_f64();
        let speed = if elapsed > 0.0 {
            ((i + 1) * MAX_READ_BLOCK) as f64 / elapsed / 1024.0
        } else {
            0.0
        };

        emit_progress(app, "校验", i + 1, total, speed);
    }

    emit_log(app, "success", "校验通过");
    Ok(())
}

/// 完整下载流程：擦除 → 写入 → 校验 → 跳转
pub fn download(
    app: &AppHandle,
    serial: &SerialManager,
    bootloader: &Bootloader,
    image: &MemoryImage,
    cancel: &CancelToken,
    do_verify: bool,
    do_go: bool,
) -> AppResult<()> {
    // 擦除
    erase_flash(app, serial, bootloader, cancel)?;

    // 写入
    write_flash(app, serial, image, cancel)?;

    // 校验
    if do_verify {
        verify_flash(app, serial, image, cancel)?;
    }

    // 跳转执行
    if do_go {
        if let Some(entry) = image.entry_point.or(image.start_address()) {
            emit_log(app, "info", &format!("跳转到 0x{:08X}...", entry));
            cmd_go(serial, entry)?;
            emit_log(app, "success", "程序已启动");
        }
    }

    emit_log(app, "success", "下载完成！");
    Ok(())
}
