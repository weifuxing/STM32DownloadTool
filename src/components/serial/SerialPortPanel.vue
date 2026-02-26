<script setup lang="ts">
import { useSerialStore } from "@/stores/serial";
import { useChipStore } from "@/stores/chip";
import { useOperationStore } from "@/stores/operation";
import { BAUD_RATES, ISP_MODE_OPTIONS } from "@/types/serial";
import { NSelect, NButton } from "naive-ui";
import { computed } from "vue";

const serial = useSerialStore();
const chip = useChipStore();
const operation = useOperationStore();

const portOptions = computed(() =>
  serial.ports.map((p) => ({
    label: `${p.name} - ${p.description}`,
    value: p.name,
  }))
);

const baudRateOptions = BAUD_RATES.map((b) => ({
  label: `${b}`,
  value: b,
}));

const isConnected = computed(() => serial.connected);
const isBootloaderConnected = computed(() => chip.state === "Connected");
const canOperate = computed(() => !operation.running);

async function handleConnect() {
  if (isBootloaderConnected.value) {
    // 断开 bootloader + 串口
    await chip.disconnect();
    await serial.disconnect();
    operation.addLog("info", "已断开连接");
  } else if (isConnected.value) {
    // 已打开串口，连接 bootloader
    try {
      await chip.connect(serial.ispMode);
      operation.addLog("success", `已连接: ${chip.chipInfo?.chip_name} (PID: 0x${chip.chipInfo?.pid.toString(16).toUpperCase()})`);
    } catch (e: any) {
      operation.addLog("error", `连接 Bootloader 失败: ${e}`);
      await serial.disconnect();
    }
  } else {
    // 打开串口 + 连接 bootloader
    try {
      await serial.connect();
      operation.addLog("info", `串口 ${serial.selectedPort} 已打开`);
      await chip.connect(serial.ispMode);
      operation.addLog("success", `已连接: ${chip.chipInfo?.chip_name} (PID: 0x${chip.chipInfo?.pid.toString(16).toUpperCase()})`);
    } catch (e: any) {
      operation.addLog("error", `连接失败: ${e}`);
      if (serial.connected) {
        await serial.disconnect();
      }
    }
  }
}

const connectButtonText = computed(() => {
  if (chip.loading) return "连接中...";
  if (isBootloaderConnected.value) return "断开";
  return "连接";
});

const connectButtonType = computed(() => {
  if (isBootloaderConnected.value) return "error";
  return "success";
});
</script>

<template>
  <div class="panel">
    <div class="panel-header">串口配置</div>
    <div class="panel-body">
      <div class="form-row">
        <label>端口</label>
        <div class="port-select-row">
          <NSelect
            v-model:value="serial.selectedPort"
            :options="portOptions"
            placeholder="选择串口"
            :disabled="isBootloaderConnected || !canOperate"
            size="small"
            style="flex: 1"
          />
          <NButton
            size="small"
            @click="serial.refreshPorts()"
            :disabled="isBootloaderConnected"
            quaternary
          >
            刷新
          </NButton>
        </div>
      </div>

      <div class="form-row">
        <label>波特率</label>
        <NSelect
          v-model:value="serial.config.baud_rate"
          :options="baudRateOptions"
          :disabled="isBootloaderConnected || !canOperate"
          size="small"
        />
      </div>

      <div class="form-row">
        <label>参数</label>
        <div class="param-info">8E1 (AN3155)</div>
      </div>

      <div class="form-row">
        <label>ISP</label>
        <NSelect
          v-model:value="serial.ispMode"
          :options="ISP_MODE_OPTIONS"
          :disabled="isBootloaderConnected || !canOperate"
          size="small"
        />
      </div>

      <div class="form-row">
        <NButton
          :type="connectButtonType"
          size="small"
          block
          @click="handleConnect"
          :disabled="(!serial.selectedPort && !isConnected) || !canOperate"
          :loading="chip.loading || serial.loading"
        >
          {{ connectButtonText }}
        </NButton>
      </div>

      <div class="status-indicator">
        <span
          class="status-dot"
          :class="{
            connected: isBootloaderConnected,
            connecting: chip.state === 'Syncing',
          }"
        ></span>
        <span class="status-text">
          {{
            isBootloaderConnected
              ? `已连接 (${serial.selectedPort})`
              : chip.state === "Syncing"
                ? "同步中..."
                : "未连接"
          }}
        </span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.panel {
  background: var(--bg-card);
  border: 1px solid var(--border-color);
  border-radius: var(--radius);
  overflow: hidden;
}

.panel-header {
  padding: 8px 12px;
  font-size: 13px;
  font-weight: 600;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.panel-body {
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.form-row label {
  width: 48px;
  font-size: 12px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.form-row > :not(label) {
  flex: 1;
}

.port-select-row {
  display: flex;
  gap: 4px;
  flex: 1;
}

.param-info {
  font-size: 12px;
  color: var(--text-secondary);
  padding: 4px 8px;
  background: var(--bg-primary);
  border-radius: 4px;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 6px;
  padding-top: 4px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #666;
}

.status-dot.connected {
  background: var(--accent-primary);
  box-shadow: 0 0 6px var(--accent-primary);
}

.status-dot.connecting {
  background: var(--accent-warning);
  animation: pulse 1s infinite;
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.3;
  }
}

.status-text {
  font-size: 12px;
  color: var(--text-secondary);
}
</style>
