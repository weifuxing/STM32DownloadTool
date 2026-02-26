import { defineStore } from "pinia";
import { ref, computed } from "vue";
import type { PortInfo, SerialConfig, IspMode } from "@/types/serial";
import { DEFAULT_SERIAL_CONFIG } from "@/types/serial";
import * as serialApi from "@/api/serial";

export const useSerialStore = defineStore("serial", () => {
  const ports = ref<PortInfo[]>([]);
  const selectedPort = ref("");
  const config = ref<SerialConfig>({ ...DEFAULT_SERIAL_CONFIG });
  const ispMode = ref<IspMode>("rts_reset");
  const connected = ref(false);
  const loading = ref(false);

  const selectedPortInfo = computed(() =>
    ports.value.find((p) => p.name === selectedPort.value)
  );

  async function refreshPorts() {
    try {
      ports.value = await serialApi.listSerialPorts();
      // 如果之前选择的端口不在列表中，清空选择
      if (
        selectedPort.value &&
        !ports.value.find((p) => p.name === selectedPort.value)
      ) {
        selectedPort.value = "";
      }
    } catch (e) {
      console.error("刷新串口列表失败:", e);
    }
  }

  async function connect() {
    if (!selectedPort.value) return;
    loading.value = true;
    try {
      await serialApi.openSerialPort(selectedPort.value, config.value);
      connected.value = true;
    } finally {
      loading.value = false;
    }
  }

  async function disconnect() {
    loading.value = true;
    try {
      await serialApi.closeSerialPort();
      connected.value = false;
    } finally {
      loading.value = false;
    }
  }

  return {
    ports,
    selectedPort,
    config,
    ispMode,
    connected,
    loading,
    selectedPortInfo,
    refreshPorts,
    connect,
    disconnect,
  };
});
