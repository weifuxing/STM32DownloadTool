<script setup lang="ts">
import { computed } from "vue";
import { NButton, NCheckbox } from "naive-ui";
import { useChipStore } from "@/stores/chip";
import { useFirmwareStore } from "@/stores/firmware";
import { useOperationStore } from "@/stores/operation";
import * as flashApi from "@/api/flash";

const chip = useChipStore();
const firmware = useFirmwareStore();
const operation = useOperationStore();

const isReady = computed(
  () => chip.state === "Connected" && !operation.running
);
const canDownload = computed(() => isReady.value && firmware.loaded);

async function handleDownload() {
  try {
    await operation.download();
    operation.addLog("success", "下载操作完成");
  } catch (e: any) {
    operation.addLog("error", `下载失败: ${e}`);
  }
}

async function handleErase() {
  try {
    await operation.erase();
    operation.addLog("success", "擦除操作完成");
  } catch (e: any) {
    operation.addLog("error", `擦除失败: ${e}`);
  }
}

async function handleVerify() {
  try {
    await operation.verify();
    operation.addLog("success", "校验操作完成");
  } catch (e: any) {
    operation.addLog("error", `校验失败: ${e}`);
  }
}

async function handleGo() {
  const addr = firmware.info?.entry_point ?? firmware.info?.start_address;
  if (addr === null || addr === undefined) {
    operation.addLog("error", "无有效跳转地址");
    return;
  }
  try {
    await flashApi.goToAddress(addr);
    operation.addLog("info", `已跳转到 0x${addr.toString(16).toUpperCase()}`);
  } catch (e: any) {
    operation.addLog("error", `跳转失败: ${e}`);
  }
}

async function handleCancel() {
  await operation.cancel();
  operation.addLog("warning", "操作已取消");
}
</script>

<template>
  <div class="panel operation-panel">
    <div class="panel-header">操作</div>
    <div class="panel-body">
      <div class="button-grid">
        <NButton
          type="primary"
          size="small"
          :disabled="!canDownload"
          @click="handleDownload"
        >
          下载
        </NButton>
        <NButton
          type="warning"
          size="small"
          :disabled="!isReady"
          @click="handleErase"
        >
          擦除
        </NButton>
        <NButton
          type="info"
          size="small"
          :disabled="!canDownload"
          @click="handleVerify"
        >
          校验
        </NButton>
        <NButton
          size="small"
          :disabled="!isReady || !firmware.loaded"
          @click="handleGo"
        >
          跳转
        </NButton>
      </div>

      <div class="options">
        <NCheckbox v-model:checked="operation.doVerify" size="small">
          写入后校验
        </NCheckbox>
        <NCheckbox v-model:checked="operation.doGo" size="small">
          完成后跳转
        </NCheckbox>
      </div>

      <NButton
        v-if="operation.running"
        type="error"
        size="small"
        block
        @click="handleCancel"
      >
        取消操作
      </NButton>
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

.operation-panel {
  width: 200px;
  flex-shrink: 0;
}

.panel-header {
  padding: 7px 12px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  color: var(--text-secondary);
  border-bottom: 1px solid var(--border-color);
}

.panel-body {
  padding: 10px 12px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.button-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 5px;
}

.options {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 12px;
}

.protocol-select .label {
  color: var(--text-secondary);
}
</style>
