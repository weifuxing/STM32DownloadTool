<script setup lang="ts">
import { useOperationStore } from "@/stores/operation";
import { computed } from "vue";

const operation = useOperationStore();

const progressPercent = computed(() => Math.min(100, Math.max(0, operation.progress)));
const isActive = computed(() => operation.running || operation.progress > 0);
</script>

<template>
  <div class="progress-container" v-if="isActive">
    <div class="progress-header">
      <span class="progress-stage">{{ operation.stage || "就绪" }}</span>
      <span class="progress-percent">{{ progressPercent.toFixed(1) }}%</span>
    </div>
    <div class="progress-bar-track">
      <div
        class="progress-bar-fill"
        :style="{ width: progressPercent + '%' }"
        :class="{ active: operation.running }"
      ></div>
    </div>
    <div class="progress-footer" v-if="operation.running">
      <span>{{ operation.current }} / {{ operation.total }}</span>
      <span v-if="operation.speed > 0">
        {{ operation.speed.toFixed(1) }} KB/s
      </span>
    </div>
  </div>
</template>

<style scoped>
.progress-container {
  padding: 0 2px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
  margin-bottom: 4px;
}

.progress-stage {
  color: var(--text-secondary);
  font-weight: 500;
}

.progress-percent {
  color: var(--accent-primary);
  font-weight: 600;
  font-variant-numeric: tabular-nums;
}

.progress-bar-track {
  height: 3px;
  background: var(--bg-secondary);
  border-radius: 2px;
  overflow: hidden;
}

.progress-bar-fill {
  height: 100%;
  background: var(--accent-primary);
  border-radius: 2px;
  transition: width 0.3s ease;
}

.progress-bar-fill.active {
  background: linear-gradient(
    90deg,
    var(--accent-primary),
    var(--accent-hover)
  );
}

.progress-footer {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
  color: var(--text-secondary);
  margin-top: 2px;
  font-variant-numeric: tabular-nums;
}
</style>
