/** 固件信息 */
export interface FirmwareInfo {
  file_path: string;
  file_format: string;
  total_size: number;
  start_address: number | null;
  end_address: number | null;
  entry_point: number | null;
  segment_count: number;
}
