<script setup lang="ts">
import { useOperationStore } from "@/stores/operation";

const operation = useOperationStore();
</script>

<template>
  <div class="status-bar">
    <div class="status-left">
      <span v-if="operation.running" class="status-running">
        <span class="dot dot-running"></span>{{ operation.stage }}中...
      </span>
      <span v-else class="status-idle">
        <span class="dot dot-idle"></span>就绪
      </span>
    </div>
    <div class="status-right">
      <span v-if="operation.running && operation.speed > 0">
        {{ operation.speed.toFixed(1) }} KB/s
      </span>
    </div>
  </div>
</template>

<style scoped>
.status-bar {
  display: flex; align-items: center; justify-content: space-between;
  padding: 0 14px; height: 22px;
  background: var(--bg-secondary); border-top: 1px solid var(--border-color);
  font-size: 11px; color: var(--text-secondary); flex-shrink: 0;
}
.status-left, .status-right { display: flex; align-items: center; gap: 5px; }
.dot { display: inline-block; width: 5px; height: 5px; border-radius: 50%; margin-right: 5px; flex-shrink: 0; }
.dot-idle { background: var(--text-dim); }
.dot-running { background: var(--accent-warning); animation: pulse 1s infinite; }
@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.3; } }
.status-running { display: flex; align-items: center; color: var(--accent-warning); }
.status-idle { display: flex; align-items: center; color: var(--text-secondary); }
</style>
