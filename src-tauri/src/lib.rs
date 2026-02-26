pub mod commands;
pub mod error;
pub mod firmware;
pub mod flash;
pub mod protocol;
pub mod serial;

use commands::firmware_cmd::FirmwareManager;
use commands::flash_cmd::CancelState;
use protocol::bootloader::Bootloader;
use serial::manager::SerialManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;
            Ok(())
        })
        .manage(SerialManager::new())
        .manage(Bootloader::new())
        .manage(FirmwareManager::new())
        .manage(CancelState::new())
        .invoke_handler(tauri::generate_handler![
            // 串口命令
            commands::serial_cmd::list_serial_ports,
            commands::serial_cmd::open_serial_port,
            commands::serial_cmd::close_serial_port,
            commands::serial_cmd::get_serial_status,
            // 协议命令
            commands::protocol_cmd::connect_bootloader,
            commands::protocol_cmd::disconnect_bootloader,
            commands::protocol_cmd::get_bootloader_state,
            commands::protocol_cmd::get_chip_info,
            // 固件命令
            commands::firmware_cmd::load_firmware,
            commands::firmware_cmd::get_firmware_info,
            commands::firmware_cmd::clear_firmware,
            // Flash 操作命令
            commands::flash_cmd::download_firmware,
            commands::flash_cmd::erase_flash,
            commands::flash_cmd::verify_flash,
            commands::flash_cmd::read_flash,
            commands::flash_cmd::export_flash,
            commands::flash_cmd::go_to_address,
            commands::flash_cmd::cancel_operation,
            // 保护命令
            commands::protection_cmd::enable_write_protection,
            commands::protection_cmd::disable_write_protection,
            commands::protection_cmd::enable_readout_protection,
            commands::protection_cmd::disable_readout_protection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
