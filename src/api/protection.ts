import { invoke } from "@tauri-apps/api/core";

export async function enableWriteProtection(
  sectors: number[]
): Promise<void> {
  return invoke("enable_write_protection", { sectors });
}

export async function disableWriteProtection(): Promise<void> {
  return invoke("disable_write_protection");
}

export async function enableReadoutProtection(): Promise<void> {
  return invoke("enable_readout_protection");
}

export async function disableReadoutProtection(): Promise<void> {
  return invoke("disable_readout_protection");
}
