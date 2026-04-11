<script setup lang="ts">
import { useFirmwareStore } from "@/stores/firmware";
import { useOperationStore } from "@/stores/operation";
import { NButton, NInput } from "naive-ui";
import { open } from "@tauri-apps/plugin-dialog";
import { computed } from "vue";

const firmware = useFirmwareStore();
const operation = useOperationStore();

const canOperate = computed(() => !operation.running);

async function selectFile() {
  const selected = await open({
    multiple: false,
    filters: [
      { name: "固件文件", extensions: ["hex", "bin", "ihex"] },
      { name: "所有文件", extensions: ["*"] },
    ],
  });

  if (selected) {
    try {
      await firmware.load(selected);
      operation.addLog(
        "info",
        `已加载固件: ${firmware.fileName} (${firmware.formatSize(firmware.info!.total_size)})`
      );
    } catch (e: any) {
      operation.addLog("error", `加载固件失败: ${e}`);
    }
  }
}
</script>

<template>
  <div class="panel">
    <div class="panel-header">固件文件</div>
    <div class="panel-body">
      <div class="form-row">
        <NButton
          size="small"
          type="primary"
          @click="selectFile"
          :disabled="!canOperate"
          :loading="firmware.loading"
        >
          选择文件
        </NButton>
        <NButton
          v-if="firmware.loaded"
          size="small"
          quaternary
          @click="firmware.clear()"
          :disabled="!canOperate"
        >
          清除
        </NButton>
      </div>

      <template v-if="firmware.loaded && firmware.info">
        <div class="file-info">
          <div class="info-item">
            <span class="info-label">文件</span>
            <span class="info-value" :title="firmware.info.file_path">
              {{ firmware.fileName }}
            </span>
          </div>
          <div class="info-item">
            <span class="info-label">格式</span>
            <span class="info-value">{{ firmware.info.file_format }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">大小</span>
            <span class="info-value">
              {{ firmware.formatSize(firmware.info.total_size) }}
            </span>
          </div>
          <div class="info-item">
            <span class="info-label">地址</span>
            <span class="info-value">
              {{ firmware.formatAddress(firmware.info.start_address) }} -
              {{ firmware.formatAddress(firmware.info.end_address) }}
            </span>
          </div>
          <div class="info-item" v-if="firmware.info.entry_point !== null">
            <span class="info-label">入口</span>
            <span class="info-value">
              {{ firmware.formatAddress(firmware.info.entry_point) }}
            </span>
          </div>
        </div>
      </template>

      <template v-else>
        <div class="no-file">
          <span>未加载固件文件</span>
        </div>

        <div class="form-row">
          <label class="small-label">BIN 起始地址</label>
          <NInput
            v-model:value="firmware.binStartAddress"
            size="small"
            placeholder="0x08000000"
            style="width: 140px"
          />
        </div>
      </template>
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
  gap: 7px;
}

.form-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.file-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 8px;
  background: var(--bg-primary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
}

.info-item {
  display: flex;
  gap: 8px;
  font-size: 12px;
}

.info-label {
  width: 32px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.info-value {
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.no-file {
  padding: 14px;
  text-align: center;
  font-size: 12px;
  color: var(--text-secondary);
}

.small-label {
  font-size: 12px;
  color: var(--text-secondary);
  white-space: nowrap;
}
</style>
