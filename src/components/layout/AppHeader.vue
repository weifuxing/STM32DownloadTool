<script setup lang="ts">
import { ref, onMounted } from "vue";
import { NButton, NModal, NCard, NProgress, NSpace, NTag } from "naive-ui";
import { useUpdater } from "@/composables/useUpdater";
import { getVersion } from "@tauri-apps/api/app";

const {
  error,
  progress,
  newVersion,
  releaseNotes,
  updateAvailable,
  checking,
  downloading,
  checkForUpdate,
  downloadAndInstall,
  reset,
} = useUpdater();

const showDialog = ref(false);
const currentVersion = ref("");

/** 点击检查更新 */
async function handleCheckUpdate() {
  const hasUpdate = await checkForUpdate();
  if (hasUpdate) {
    showDialog.value = true;
  }
}

/** 确认下载并安装 */
async function handleConfirmUpdate() {
  await downloadAndInstall();
}

/** 关闭弹窗 */
function handleCloseDialog() {
  if (!downloading.value) {
    showDialog.value = false;
    reset();
  }
}

// 启动时静默检查一次
onMounted(async () => {
  currentVersion.value = await getVersion();
  const hasUpdate = await checkForUpdate();
  if (hasUpdate) {
    showDialog.value = true;
  }
});
</script>

<template>
  <div class="app-header">
    <div class="header-title">
      <span class="header-icon">?</span>
      <span>STM32 ISP Download Tool</span>
    </div>

    <div class="header-actions">
      <!-- 检查更新按钮 -->
      <NButton
        size="tiny"
        :type="updateAvailable ? 'warning' : 'default'"
        :loading="checking"
        quaternary
        class="update-btn"
        @click="handleCheckUpdate"
      >
        {{ checking ? '检查中...' : updateAvailable ? '有新版本' : '检查更新' }}
      </NButton>

      <span class="header-version">v{{ currentVersion }}</span>
    </div>

    <!-- 更新弹窗 -->
    <NModal
      v-model:show="showDialog"
      :mask-closable="!downloading"
      :close-on-esc="!downloading"
    >
      <NCard
        title="发现新版本"
        :bordered="false"
        size="small"
        class="update-dialog"
        role="dialog"
        :closable="!downloading"
        @close="handleCloseDialog"
      >
        <NSpace vertical :size="12">
          <div class="update-version">
            <span>新版本：</span>
            <NTag type="success" size="small">{{ newVersion }}</NTag>
          </div>

          <!-- 更新说明 -->
          <div v-if="releaseNotes" class="update-notes">
            <div class="notes-label">更新说明：</div>
            <div class="notes-content">{{ releaseNotes }}</div>
          </div>

          <!-- 下载进度 -->
          <div v-if="downloading">
            <NProgress
              type="line"
              :percentage="progress"
              :show-indicator="true"
              status="info"
            />
            <div class="progress-text">正在下载更新... {{ progress }}%</div>
          </div>

          <!-- 错误信息 -->
          <div v-if="error" class="update-error">{{ error }}</div>

          <!-- 操作按钮 -->
          <NSpace justify="end" v-if="!downloading">
            <NButton size="small" @click="handleCloseDialog">稍后再说</NButton>
            <NButton size="small" type="primary" @click="handleConfirmUpdate">
              立即更新
            </NButton>
          </NSpace>
        </NSpace>
      </NCard>
    </NModal>
  </div>
</template>

<style scoped>
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 14px;
  height: 38px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  -webkit-app-region: drag;
}

.header-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  font-weight: 600;
  letter-spacing: 0.01em;
  color: var(--text-primary);
}

.header-icon {
  font-size: 15px;
  opacity: 0.85;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  -webkit-app-region: no-drag;
}

.header-version {
  font-size: 11px;
  color: var(--text-secondary);
}

.update-btn {
  font-size: 11px;
}

.update-dialog {
  width: 400px;
}

.update-version {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
}

.notes-label {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.notes-content {
  font-size: 12px;
  line-height: 1.5;
  padding: 8px;
  background: var(--bg-primary);
  border-radius: var(--radius-sm);
  max-height: 200px;
  overflow-y: auto;
  white-space: pre-wrap;
}

.progress-text {
  font-size: 11px;
  color: var(--text-secondary);
  margin-top: 4px;
  text-align: center;
}

.update-error {
  color: var(--accent-danger);
  font-size: 12px;
}
</style>
