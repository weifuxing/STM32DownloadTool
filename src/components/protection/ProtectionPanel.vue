<script setup lang="ts">
import { computed } from "vue";
import { NButton, NPopconfirm } from "naive-ui";
import { useChipStore } from "@/stores/chip";
import { useOperationStore } from "@/stores/operation";
import * as protectionApi from "@/api/protection";

const chip = useChipStore();
const operation = useOperationStore();

const isReady = computed(
  () => chip.state === "Connected" && !operation.running
);

async function handleWriteUnprotect() {
  try {
    await protectionApi.disableWriteProtection();
    operation.addLog("warning", "写保护已关闭（设备已复位，需重新连接）");
    chip.setDisconnected();
  } catch (e: any) {
    operation.addLog("error", `关闭写保护失败: ${e}`);
  }
}

async function handleReadoutUnprotect() {
  try {
    await protectionApi.disableReadoutProtection();
    operation.addLog(
      "warning",
      "读保护已关闭（Flash 已擦除，设备已复位，需重新连接）"
    );
    chip.setDisconnected();
  } catch (e: any) {
    operation.addLog("error", `关闭读保护失败: ${e}`);
  }
}

async function handleReadoutProtect() {
  try {
    await protectionApi.enableReadoutProtection();
    operation.addLog("warning", "读保护已开启（设备已复位，需重新连接）");
    chip.setDisconnected();
  } catch (e: any) {
    operation.addLog("error", `开启读保护失败: ${e}`);
  }
}
</script>

<template>
  <div class="panel">
    <div class="panel-header">保护管理</div>
    <div class="panel-body">
      <div class="protection-row">
        <span class="prot-label">写保护</span>
        <NPopconfirm @positive-click="handleWriteUnprotect">
          <template #trigger>
            <NButton
              size="tiny"
              type="warning"
              :disabled="!isReady"
              quaternary
            >
              关闭写保护
            </NButton>
          </template>
          确定关闭写保护？设备将自动复位。
        </NPopconfirm>
      </div>

      <div class="protection-row">
        <span class="prot-label">读保护</span>
        <div class="prot-actions">
          <NPopconfirm @positive-click="handleReadoutProtect">
            <template #trigger>
              <NButton
                size="tiny"
                type="info"
                :disabled="!isReady"
                quaternary
              >
                开启
              </NButton>
            </template>
            确定开启读保护？
          </NPopconfirm>
          <NPopconfirm @positive-click="handleReadoutUnprotect">
            <template #trigger>
              <NButton
                size="tiny"
                type="error"
                :disabled="!isReady"
                quaternary
              >
                关闭
              </NButton>
            </template>
            ⚠️ 关闭读保护将触发全片擦除！确定继续？
          </NPopconfirm>
        </div>
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
  gap: 8px;
}

.protection-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 12px;
}

.prot-label {
  color: var(--text-secondary);
}

.prot-actions {
  display: flex;
  gap: 4px;
}
</style>
