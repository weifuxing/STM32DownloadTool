import { invoke } from "@tauri-apps/api/core";
import type { PortInfo, SerialConfig, SerialStatus } from "@/types/serial";

export async function listSerialPorts(): Promise<PortInfo[]> {
  return invoke<PortInfo[]>("list_serial_ports");
}

export async function openSerialPort(
  portName: string,
  config: SerialConfig
): Promise<void> {
  return invoke("open_serial_port", { portName, config });
}

export async function closeSerialPort(): Promise<void> {
  return invoke("close_serial_port");
}

export async function getSerialStatus(): Promise<SerialStatus> {
  return invoke<SerialStatus>("get_serial_status");
}
