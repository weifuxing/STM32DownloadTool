/** 串口端口信息 */
export interface PortInfo {
  name: string;
  description: string;
}

/** 串口配置 */
export interface SerialConfig {
  baud_rate: number;
  data_bits: number;
  parity: string;
  stop_bits: number;
  timeout_ms: number;
}

/** 串口状态 */
export interface SerialStatus {
  connected: boolean;
  port_name: string;
}

/** 默认串口配置 (AN3155: 8E1) */
export const DEFAULT_SERIAL_CONFIG: SerialConfig = {
  baud_rate: 230400,
  data_bits: 8,
  parity: "even",
  stop_bits: 1,
  timeout_ms: 1000,
};

/** 可选波特率 */
export const BAUD_RATES = [
  9600, 14400, 19200, 28800, 38400, 57600, 76800, 115200, 230400, 460800,
  921600,
];

/** ISP 进入模式 */
export type IspMode = "rts_reset" | "dtr_reset" | "none";

/** ISP 模式选项 */
export const ISP_MODE_OPTIONS: { label: string; value: IspMode }[] = [
  { label: "RTS 复位 / DTR BOOT0", value: "rts_reset" },
  { label: "DTR 复位 / RTS BOOT0", value: "dtr_reset" },
  { label: "手动（不自动复位）", value: "none" },
];
