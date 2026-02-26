import { invoke } from "@tauri-apps/api/core";

export async function downloadFirmware(
  doVerify: boolean,
  doGo: boolean
): Promise<void> {
  return invoke("download_firmware", { doVerify, doGo });
}

export async function eraseFlash(): Promise<void> {
  return invoke("erase_flash");
}

export async function verifyFlash(): Promise<void> {
  return invoke("verify_flash");
}

export async function readFlash(
  startAddress: number,
  length: number
): Promise<number[]> {
  return invoke<number[]>("read_flash", { startAddress, length });
}

export async function exportFlash(
  startAddress: number,
  length: number,
  filePath: string,
  format: string
): Promise<void> {
  return invoke("export_flash", { startAddress, length, filePath, format });
}

export async function goToAddress(address: number): Promise<void> {
  return invoke("go_to_address", { address });
}

export async function cancelOperation(): Promise<void> {
  return invoke("cancel_operation");
}
