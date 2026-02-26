<script setup lang="ts">
import { useChipStore } from "@/stores/chip";
import { NTag } from "naive-ui";
import { computed } from "vue";

const chip = useChipStore();

/** 命令码到命令名称的映射 */
const COMMAND_NAMES: Record<number, string> = {
  0x00: "Get",
  0x01: "GetVer",
  0x02: "GetID",
  0x11: "Read",
  0x21: "Go",
  0x31: "Write",
  0x43: "Erase",
  0x44: "ExtErase",
  0x63: "WP",
  0x73: "WUnP",
  0x82: "RP",
  0x92: "RUnP",
};

const commandTags = computed(() => {
  if (!chip.chipInfo) return [];
  return chip.chipInfo.supported_commands.map((cmd) => ({
    code: cmd,
    name: COMMAND_NAMES[cmd] || `0x${cmd.toString(16).toUpperCase()}`,
  }));
});
</script>

<template>
  <div class="panel">
    <div class="panel-header">芯片信息</div>
    <div class="panel-body">
      <template v-if="chip.chipInfo">
        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">芯片</span>
            <span class="info-value highlight">
              {{ chip.chipInfo.chip_name }}
            </span>
          </div>
          <div class="info-item">
            <span class="info-label">PID</span>
            <span class="info-value">
              0x{{ chip.chipInfo.pid.toString(16).toUpperCase().padStart(4, "0") }}
            </span>
          </div>
          <div class="info-item">
            <span class="info-label">BL 版本</span>
            <span class="info-value">
              v{{ chip.chipInfo.bootloader_version }}
            </span>
          </div>
        </div>
        <div class="commands-section">
          <span class="info-label">支持命令</span>
          <div class="command-tags">
            <NTag
              v-for="cmd in commandTags"
              :key="cmd.code"
              size="small"
              :bordered="false"
              type="info"
            >
              {{ cmd.name }}
            </NTag>
          </div>
        </div>
      </template>
      <template v-else>
        <div class="no-info">未连接设备</div>
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

.info-grid {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.info-item {
  display: flex;
  gap: 8px;
  font-size: 12px;
  align-items: center;
}

.info-label {
  color: var(--text-secondary);
  font-size: 12px;
  min-width: 52px;
}

.info-value {
  color: var(--text-primary);
}

.info-value.highlight {
  color: var(--accent-info);
  font-weight: 600;
}

.commands-section {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.command-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.no-info {
  padding: 12px;
  text-align: center;
  font-size: 12px;
  color: var(--text-secondary);
}
</style>
