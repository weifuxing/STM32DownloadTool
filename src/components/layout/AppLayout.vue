<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import AppHeader from "./AppHeader.vue";
import StatusBar from "./StatusBar.vue";
import SerialPortPanel from "../serial/SerialPortPanel.vue";
import FirmwarePanel from "../firmware/FirmwarePanel.vue";
import OperationPanel from "../operation/OperationPanel.vue";
import ChipInfoPanel from "../chip/ChipInfoPanel.vue";
import ProtectionPanel from "../protection/ProtectionPanel.vue";
import LogPanel from "../log/LogPanel.vue";
import ProgressBar from "../progress/ProgressBar.vue";
import { useOperationStore } from "@/stores/operation";
import { useSerialStore } from "@/stores/serial";

const operation = useOperationStore();
const serial = useSerialStore();

let refreshTimer: ReturnType<typeof setInterval> | null = null;

onMounted(async () => {
  await operation.initListeners();
  await serial.refreshPorts();

  // 每 2 秒自动刷新串口列表（仅未连接时）
  refreshTimer = setInterval(() => {
    if (!serial.connected) {
      serial.refreshPorts();
    }
  }, 2000);
});

onUnmounted(() => {
  operation.cleanupListeners();
  if (refreshTimer) {
    clearInterval(refreshTimer);
    refreshTimer = null;
  }
});
</script>

<template>
  <div class="app-layout">
    <AppHeader />

    <div class="app-content">
      <!-- 上部：串口配置 + 固件文件 -->
      <div class="top-row">
        <SerialPortPanel />
        <FirmwarePanel />
      </div>

      <!-- 中部：操作按钮 + 芯片信息 + 保护管理 -->
      <div class="middle-row">
        <OperationPanel />
        <div class="info-column">
          <ChipInfoPanel />
          <ProtectionPanel />
        </div>
      </div>

      <!-- 进度条 -->
      <ProgressBar />

      <!-- 下部：日志 -->
      <div class="bottom-row">
        <LogPanel />
      </div>
    </div>

    <StatusBar />
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.app-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 12px;
  gap: 12px;
  overflow: hidden;
}

.top-row {
  display: flex;
  gap: 12px;
}

.top-row > * {
  flex: 1;
}

.middle-row {
  display: flex;
  gap: 12px;
}

.info-column {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.bottom-row {
  flex: 1;
  min-height: 120px;
  overflow: hidden;
}
</style>
