<script setup lang="ts">
import { useOperationStore } from "@/stores/operation";

const operation = useOperationStore();
</script>

<template>
  <div class="status-bar">
    <div class="status-left">
      <span v-if="operation.running" class="status-running">
        ● {{ operation.stage }}中...
      </span>
      <span v-else class="status-idle">● 就绪</span>
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
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  height: 24px;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
  font-size: 12px;
  color: var(--text-secondary);
}

.status-running {
  color: var(--accent-warning);
}

.status-idle {
  color: var(--accent-primary);
}
</style>
