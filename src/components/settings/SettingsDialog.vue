<script setup lang="ts">
import { ref } from "vue";
import { Modal, Button, Select } from "@/components/ui";
import { useSettingsStore, useGroupsStore, useEntriesStore } from "@/stores";
import { useToast } from "@/composables/useToast";
import { invoke } from "@tauri-apps/api/core";
import ChangePasswordDialog from "./ChangePasswordDialog.vue";

defineProps<{
  show: boolean;
}>();

const emit = defineEmits<{
  close: [];
}>();

const settingsStore = useSettingsStore();
const groupsStore = useGroupsStore();
const entriesStore = useEntriesStore();
const { showToast } = useToast();

const showChangePassword = ref(false);
const showImportConfirm = ref(false);
const isSaving = ref(false);
const isExporting = ref(false);
const isExportingExcel = ref(false);
const isImporting = ref(false);
const importData = ref("");
const importMergeMode = ref(true);

const localSettings = ref({
  autoLockMinutes: settingsStore.settings.autoLockMinutes,
  clearClipboardSeconds: settingsStore.settings.clearClipboardSeconds,
  theme: settingsStore.settings.theme,
});

const themeOptions = [
  { value: "system", label: "跟随系统", icon: "💻" },
  { value: "light", label: "浅色", icon: "☀️" },
  { value: "dark", label: "深色", icon: "🌙" },
];

const autoLockOptions = [
  { value: 1, label: "1 分钟", icon: "⏱️" },
  { value: 5, label: "5 分钟", icon: "⏱️" },
  { value: 15, label: "15 分钟", icon: "⏱️" },
  { value: 30, label: "30 分钟", icon: "⏱️" },
  { value: 0, label: "从不", icon: "♾️" },
];

const clipboardOptions = [
  { value: 15, label: "15 秒", icon: "📋" },
  { value: 30, label: "30 秒", icon: "📋" },
  { value: 60, label: "60 秒", icon: "📋" },
  { value: 0, label: "从不清除", icon: "📋" },
];

async function handleSave() {
  isSaving.value = true;
  try {
    await settingsStore.updateSettings(localSettings.value);
    showToast("设置已保存", "success");
    emit("close");
  } catch {
    showToast("保存设置失败", "error");
  } finally {
    isSaving.value = false;
  }
}

async function handleExport() {
  isExporting.value = true;
  try {
    const content = await invoke<string>("export_data");
    await invoke("save_export_file", { content });
    showToast("数据导出成功（JSON）", "success");
  } catch (error) {
    if (error !== "用户取消保存") {
      showToast(`导出失败: ${error}`, "error");
    }
  } finally {
    isExporting.value = false;
  }
}

async function handleExportExcel() {
  isExportingExcel.value = true;
  try {
    const content = await invoke<number[]>("export_excel");
    await invoke("save_export_excel_file", { content });
    showToast("数据导出成功（Excel）", "success");
  } catch (error) {
    if (error !== "用户取消保存") {
      showToast(`导出失败: ${error}`, "error");
    }
  } finally {
    isExportingExcel.value = false;
  }
}

async function handleImport() {
  try {
    const data = await invoke<string>("load_import_file");
    importData.value = data;
    showImportConfirm.value = true;
  } catch (error) {
    if (error !== "用户取消导入") {
      showToast(`读取文件失败: ${error}`, "error");
    }
  }
}

async function confirmImport() {
  isImporting.value = true;
  try {
    const result = await invoke<{ groupsImported: number; entriesImported: number }>(
      "import_data",
      {
        jsonData: importData.value,
        mergeMode: importMergeMode.value
      }
    );

    // Refresh data
    await Promise.all([
      groupsStore.fetchGroups(),
      entriesStore.fetchEntries(),
    ]);

    showToast(
      `导入成功: ${result.groupsImported} 个分组, ${result.entriesImported} 个条目`,
      "success"
    );
    showImportConfirm.value = false;
    importData.value = "";
  } catch (error) {
    showToast(`导入失败: ${error}`, "error");
  } finally {
    isImporting.value = false;
  }
}
</script>

<template>
  <Modal :show="show" title="设置" size="md" @close="$emit('close')">
    <div class="space-y-6">
      <!-- Theme -->
      <div>
        <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-3">
          外观
        </h3>
        <Select
          v-model="localSettings.theme"
          label="主题"
          :options="themeOptions"
        />
      </div>

      <!-- Security -->
      <div>
        <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-3">
          安全
        </h3>
        <div class="space-y-4">
          <Select
            v-model="localSettings.autoLockMinutes"
            label="自动锁定"
            :options="autoLockOptions"
          />
          <Select
            v-model="localSettings.clearClipboardSeconds"
            label="剪贴板自动清除"
            :options="clipboardOptions"
          />
          <div>
            <Button
              variant="secondary"
              size="sm"
              @click="showChangePassword = true"
            >
              修改主密码
            </Button>
          </div>
        </div>
      </div>

      <!-- About -->
      <div class="pt-4 border-t border-gray-200 dark:border-gray-700">
        <div class="text-center text-sm text-gray-500 dark:text-gray-400">
          <p class="font-medium text-gray-900 dark:text-gray-100">
            One-Password v0.1.0
          </p>
          <p class="mt-1">本地密码管理器</p>
        </div>
      </div>

      <!-- Data Management -->
      <div class="pt-4 border-t border-gray-200 dark:border-gray-700">
        <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-3">
          数据管理
        </h3>
        <div class="flex space-x-3">
          <Button
            variant="secondary"
            size="sm"
            :loading="isExporting"
            @click="handleExport"
          >
            导出 JSON
          </Button>
          <Button
            variant="secondary"
            size="sm"
            :loading="isExportingExcel"
            @click="handleExportExcel"
          >
            导出 Excel
          </Button>
          <Button
            variant="secondary"
            size="sm"
            @click="handleImport"
          >
            导入数据
          </Button>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-end space-x-3">
        <Button variant="secondary" @click="$emit('close')">取消</Button>
        <Button variant="primary" :loading="isSaving" @click="handleSave">
          保存
        </Button>
      </div>
    </template>
  </Modal>

  <!-- Change Password Dialog -->
  <ChangePasswordDialog
    :show="showChangePassword"
    @close="showChangePassword = false"
  />

  <!-- Import Confirmation Dialog -->
  <Modal
    :show="showImportConfirm"
    title="确认导入"
    size="sm"
    @close="showImportConfirm = false"
  >
    <div class="space-y-4">
      <p class="text-sm text-gray-600 dark:text-gray-400">
        请选择导入模式:
      </p>

      <div class="space-y-2">
        <label class="flex items-start space-x-3 cursor-pointer">
          <input
            type="radio"
            :value="true"
            v-model="importMergeMode"
            class="mt-1"
          />
          <div>
            <div class="font-medium text-gray-900 dark:text-gray-100">
              合并模式（推荐）
            </div>
            <div class="text-xs text-gray-500 dark:text-gray-400">
              保留现有数据，仅导入新数据
            </div>
          </div>
        </label>

        <label class="flex items-start space-x-3 cursor-pointer">
          <input
            type="radio"
            :value="false"
            v-model="importMergeMode"
            class="mt-1"
          />
          <div>
            <div class="font-medium text-gray-900 dark:text-gray-100">
              覆盖模式
            </div>
            <div class="text-xs text-red-500">
              ⚠️ 警告：将删除所有现有数据！
            </div>
          </div>
        </label>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-end space-x-3">
        <Button
          variant="secondary"
          @click="showImportConfirm = false"
        >
          取消
        </Button>
        <Button
          variant="primary"
          :loading="isImporting"
          @click="confirmImport"
        >
          确认导入
        </Button>
      </div>
    </template>
  </Modal>
</template>
