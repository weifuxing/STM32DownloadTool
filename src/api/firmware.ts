import { invoke } from "@tauri-apps/api/core";
import type { FirmwareInfo } from "@/types/firmware";

export async function loadFirmware(
  filePath: string,
  binStartAddress?: number
): Promise<FirmwareInfo> {
  return invoke<FirmwareInfo>("load_firmware", {
    filePath,
    binStartAddress: binStartAddress ?? null,
  });
}

export async function getFirmwareInfo(): Promise<FirmwareInfo | null> {
  return invoke<FirmwareInfo | null>("get_firmware_info");
}

export async function clearFirmware(): Promise<void> {
  return invoke("clear_firmware");
}
