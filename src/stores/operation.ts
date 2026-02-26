import { defineStore } from "pinia";
import { ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import type { ProgressEvent, LogEvent } from "@/types/events";
import * as flashApi from "@/api/flash";

export interface LogEntry extends LogEvent {
  id: number;
}

export const useOperationStore = defineStore("operation", () => {
  const running = ref(false);
  const stage = ref("");
  const progress = ref(0);
  const speed = ref(0);
  const current = ref(0);
  const total = ref(0);
  const logs = ref<LogEntry[]>([]);
  const doVerify = ref(true);
  const doGo = ref(true);

  let logId = 0;
  let unlistenProgress: (() => void) | null = null;
  let unlistenLog: (() => void) | null = null;

  /** 初始化事件监听 */
  async function initListeners() {
    unlistenProgress = await listen<ProgressEvent>(
      "flash-progress",
      (event) => {
        stage.value = event.payload.stage;
        progress.value = event.payload.progress;
        speed.value = event.payload.speed;
        current.value = event.payload.current;
        total.value = event.payload.total;
      }
    );

    unlistenLog = await listen<LogEvent>("flash-log", (event) => {
      logs.value.push({
        ...event.payload,
        id: ++logId,
      });
    });
  }

  /** 清理事件监听 */
  function cleanupListeners() {
    unlistenProgress?.();
    unlistenLog?.();
    unlistenProgress = null;
    unlistenLog = null;
  }

  /** 添加前端日志 */
  function addLog(level: LogEvent["level"], message: string) {
    const now = new Date();
    const timestamp = `${now.getHours().toString().padStart(2, "0")}:${now
      .getMinutes()
      .toString()
      .padStart(2, "0")}:${now.getSeconds().toString().padStart(2, "0")}.${now
      .getMilliseconds()
      .toString()
      .padStart(3, "0")}`;
    logs.value.push({ level, message, timestamp, id: ++logId });
  }

  /** 清除日志 */
  function clearLogs() {
    logs.value = [];
  }

  /** 重置进度 */
  function resetProgress() {
    stage.value = "";
    progress.value = 0;
    speed.value = 0;
    current.value = 0;
    total.value = 0;
  }

  /** 下载固件 */
  async function download() {
    running.value = true;
    resetProgress();
    try {
      await flashApi.downloadFirmware(doVerify.value, doGo.value);
    } finally {
      running.value = false;
    }
  }

  /** 仅擦除 */
  async function erase() {
    running.value = true;
    resetProgress();
    try {
      await flashApi.eraseFlash();
    } finally {
      running.value = false;
    }
  }

  /** 仅校验 */
  async function verify() {
    running.value = true;
    resetProgress();
    try {
      await flashApi.verifyFlash();
    } finally {
      running.value = false;
    }
  }

  /** 取消操作 */
  async function cancel() {
    await flashApi.cancelOperation();
  }

  return {
    running,
    stage,
    progress,
    speed,
    current,
    total,
    logs,
    doVerify,
    doGo,
    initListeners,
    cleanupListeners,
    addLog,
    clearLogs,
    resetProgress,
    download,
    erase,
    verify,
    cancel,
  };
});
