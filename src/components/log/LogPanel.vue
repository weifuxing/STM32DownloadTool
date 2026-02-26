<script setup lang="ts">
import { ref, watch, nextTick } from "vue";
import { NButton } from "naive-ui";
import { useOperationStore } from "@/stores/operation";

const operation = useOperationStore();
const logContainer = ref<HTMLDivElement | null>(null);
const autoScroll = ref(true);

/** 日志级别对应的颜色 */
function levelColor(level: string): string {
  switch (level) {
    case "success":
      return "#0f9b58";
    case "warning":
      return "#f39c12";
    case "error":
      return "#e74c3c";
    default:
      return "#a0a0b0";
  }
}

/** 日志级别标签 */
function levelTag(level: string): string {
  switch (level) {
    case "success":
      return "OK";
    case "warning":
      return "WARN";
    case "error":
      return "ERR";
    default:
      return "INFO";
  }
}

// 自动滚动到底部
watch(
  () => operation.logs.length,
  async () => {
    if (autoScroll.value) {
      await nextTick();
      if (logContainer.value) {
        logContainer.value.scrollTop = logContainer.value.scrollHeight;
      }
    }
  }
);

function handleScroll() {
  if (!logContainer.value) return;
  const { scrollTop, scrollHeight, clientHeight } = logContainer.value;
  autoScroll.value = scrollHeight - scrollTop - clientHeight < 30;
}

function exportLogs() {
  const content = operation.logs
    .map((l) => `[${l.timestamp}] [${levelTag(l.level)}] ${l.message}`)
    .join("\n");
  const blob = new Blob([content], { type: "text/plain" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `stm32-log-${new Date().toISOString().slice(0, 19).replace(/:/g, "-")}.txt`;
  a.click();
  URL.revokeObjectURL(url);
}
</script>

<template>
  <div class="panel log-panel">
    <div class="panel-header">
      <span>日志</span>
      <div class="log-actions">
        <NButton size="tiny" quaternary @click="exportLogs">导出</NButton>
        <NButton size="tiny" quaternary @click="operation.clearLogs()">
          清除
        </NButton>
      </div>
    </div>
    <div
      class="log-content"
      ref="logContainer"
      @scroll="handleScroll"
    >
      <div
        v-for="entry in operation.logs"
        :key="entry.id"
        class="log-entry"
      >
        <span class="log-time">{{ entry.timestamp }}</span>
        <span
          class="log-level"
          :style="{ color: levelColor(entry.level) }"
        >
          [{{ levelTag(entry.level) }}]
        </span>
        <span class="log-message">{{ entry.message }}</span>
      </div>
      <div v-if="operation.logs.length === 0" class="log-empty">
        暂无日志
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
  display: flex;
  flex-direction: column;
  height: 100%;
}

.log-panel .panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 12px;
  font-size: 13px;
  font-weight: 600;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.log-actions {
  display: flex;
  gap: 2px;
}

.log-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px 12px;
  font-family: "Cascadia Code", "Consolas", "Monaco", monospace;
  font-size: 12px;
  line-height: 1.6;
}

.log-entry {
  display: flex;
  gap: 8px;
  white-space: nowrap;
}

.log-time {
  color: var(--text-secondary);
  flex-shrink: 0;
}

.log-level {
  flex-shrink: 0;
  font-weight: 600;
  width: 44px;
}

.log-message {
  color: var(--text-primary);
  white-space: pre-wrap;
  word-break: break-all;
}

.log-empty {
  color: var(--text-secondary);
  text-align: center;
  padding: 24px;
  font-family: inherit;
}
</style>
