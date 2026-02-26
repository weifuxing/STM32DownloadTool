import { defineStore } from "pinia";
import { ref } from "vue";
import type { BootloaderState, ChipInfo } from "@/types/protocol";
import * as protocolApi from "@/api/protocol";

export const useChipStore = defineStore("chip", () => {
  const state = ref<BootloaderState>("Disconnected");
  const chipInfo = ref<ChipInfo | null>(null);
  const loading = ref(false);

  async function connect(ispMode: string) {
    loading.value = true;
    state.value = "Syncing";
    try {
      chipInfo.value = await protocolApi.connectBootloader(ispMode);
      state.value = "Connected";
    } catch (e) {
      state.value = "Disconnected";
      chipInfo.value = null;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function disconnect() {
    await protocolApi.disconnectBootloader();
    state.value = "Disconnected";
    chipInfo.value = null;
  }

  function setDisconnected() {
    state.value = "Disconnected";
    chipInfo.value = null;
  }

  return {
    state,
    chipInfo,
    loading,
    connect,
    disconnect,
    setDisconnected,
  };
});
