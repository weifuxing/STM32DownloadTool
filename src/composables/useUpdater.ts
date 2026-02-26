import { ref, computed } from "vue";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { useOperationStore } from "@/stores/operation";

type UpdateStatus = "idle" | "checking" | "downloading" | "installing" | "ready";

/** 应用自动更新逻辑封装 */
export function useUpdater() {
  const status = ref<UpdateStatus>("idle");
  const error = ref<string | null>(null);
  const progress = ref(0);
  const newVersion = ref("");
  const releaseNotes = ref("");
  const downloadedBytes = ref(0);
  const totalBytes = ref(0);

  const operationStore = useOperationStore();

  let pendingUpdate: Update | null = null;

  const updateAvailable = computed(() => pendingUpdate !== null && status.value === "idle");
  const checking = computed(() => status.value === "checking");
  const downloading = computed(() => status.value === "downloading");
  const installing = computed(() => status.value === "installing");

  /** 检查是否有可用更新 */
  async function checkForUpdate(): Promise<boolean> {
    if (status.value === "checking" || status.value === "downloading") {
      return false;
    }

    status.value = "checking";
    error.value = null;
    pendingUpdate = null;
    operationStore.addLog("info", "开始检查更新...");

    try {
      const update = await check();

      if (update) {
        pendingUpdate = update;
        newVersion.value = update.version;
        releaseNotes.value = update.body ?? "";
        status.value = "idle";
        operationStore.addLog("success", `发现新版本: v${update.version}`);
        return true;
      }

      status.value = "idle";
      operationStore.addLog("info", "当前已是最新版本");
      return false;
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      status.value = "idle";
      operationStore.addLog("error", `检查更新失败: ${error.value}`);
      return false;
    }
  }

  /** 下载并安装更新，完成后自动重启 */
  async function downloadAndInstall(): Promise<void> {
    if (!pendingUpdate) {
      error.value = "没有可用的更新";
      return;
    }

    status.value = "downloading";
    error.value = null;
    progress.value = 0;
    downloadedBytes.value = 0;
    totalBytes.value = 0;
    operationStore.addLog("info", `开始下载更新 v${newVersion.value}...`);

    try {
      await pendingUpdate.downloadAndInstall((event) => {
        if (event.event === "Started" && event.data.contentLength) {
          totalBytes.value = event.data.contentLength;
          operationStore.addLog("info", `更新包大小: ${(totalBytes.value / 1024 / 1024).toFixed(2)} MB`);
        } else if (event.event === "Progress") {
          downloadedBytes.value += event.data.chunkLength;
          if (totalBytes.value > 0) {
            progress.value = Math.round(
              (downloadedBytes.value / totalBytes.value) * 100
            );
          }
        } else if (event.event === "Finished") {
          progress.value = 100;
          operationStore.addLog("success", "下载完成，准备安装重启...");
        }
      });

      status.value = "ready";
      // 安装完成，自动重启
      await relaunch();
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      status.value = "idle";
      operationStore.addLog("error", `更新失败: ${error.value}`);
    }
  }

  /** 重置状态 */
  function reset() {
    status.value = "idle";
    error.value = null;
    progress.value = 0;
    newVersion.value = "";
    releaseNotes.value = "";
    downloadedBytes.value = 0;
    totalBytes.value = 0;
    pendingUpdate = null;
  }

  return {
    // 状态
    status,
    error,
    progress,
    newVersion,
    releaseNotes,
    downloadedBytes,
    totalBytes,
    // 计算属性
    updateAvailable,
    checking,
    downloading,
    installing,
    // 方法
    checkForUpdate,
    downloadAndInstall,
    reset,
  };
}
