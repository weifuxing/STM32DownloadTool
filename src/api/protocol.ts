import { invoke } from "@tauri-apps/api/core";
import type { BootloaderState, ChipInfo } from "@/types/protocol";

export async function connectBootloader(ispMode: string): Promise<ChipInfo> {
  return invoke<ChipInfo>("connect_bootloader", { ispMode });
}

export async function disconnectBootloader(): Promise<void> {
  return invoke("disconnect_bootloader");
}

export async function getBootloaderState(): Promise<BootloaderState> {
  return invoke<BootloaderState>("get_bootloader_state");
}

export async function getChipInfo(): Promise<ChipInfo | null> {
  return invoke<ChipInfo | null>("get_chip_info");
}
