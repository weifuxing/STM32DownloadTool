/** Flash 进度事件 */
export interface ProgressEvent {
  stage: string;
  progress: number;
  current: number;
  total: number;
  speed: number;
}

/** Flash 日志事件 */
export interface LogEvent {
  level: "info" | "success" | "warning" | "error";
  message: string;
  timestamp: string;
}
