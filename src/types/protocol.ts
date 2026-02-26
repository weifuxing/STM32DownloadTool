/** Bootloader 连接状态 */
export type BootloaderState =
  | "Disconnected"
  | "Syncing"
  | "Connected"
  | "Busy";

/** 芯片信息 */
export interface ChipInfo {
  bootloader_version: string;
  pid: number;
  chip_name: string;
  supported_commands: number[];
}
