import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { FirmwareInfo } from "@/types/firmware";
import * as firmwareApi from "@/api/firmware";

export const useFirmwareStore = defineStore("firmware", () => {
  const info = ref<FirmwareInfo | null>(null);
  const loading = ref(false);
  const binStartAddress = ref("0x08000000");

  const loaded = computed(() => info.value !== null);

  const fileName = computed(() => {
    if (!info.value) return "";
    const parts = info.value.file_path.replace(/\\/g, "/").split("/");
    return parts[parts.length - 1];
  });

  async function load(filePath: string) {
    loading.value = true;
    try {
      const addr = parseInt(binStartAddress.value, 16);
      info.value = await firmwareApi.loadFirmware(
        filePath,
        isNaN(addr) ? undefined : addr
      );
    } finally {
      loading.value = false;
    }
  }

  async function clear() {
    await firmwareApi.clearFirmware();
    info.value = null;
  }

  /** 格式化地址为十六进制字符串 */
  function formatAddress(addr: number | null): string {
    if (addr === null) return "-";
    return "0x" + addr.toString(16).toUpperCase().padStart(8, "0");
  }

  /** 格式化文件大小 */
  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    return `${(bytes / 1024).toFixed(1)} KB`;
  }

  return {
    info,
    loading,
    loaded,
    fileName,
    binStartAddress,
    load,
    clear,
    formatAddress,
    formatSize,
  };
});
