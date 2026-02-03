<script setup lang="ts">
import { ref, computed, watch } from "vue";
import {
  ClipboardDocumentIcon,
  CheckIcon,
  EyeIcon,
  EyeSlashIcon,
  PencilIcon,
  TrashIcon,
  StarIcon,
  ArrowTopRightOnSquareIcon,
} from "@heroicons/vue/24/outline";
import { StarIcon as StarSolidIcon } from "@heroicons/vue/24/solid";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { openUrl as openExternal } from "@tauri-apps/plugin-opener";
import { Button, Modal } from "@/components/ui";
import { useEntriesStore, useGroupsStore, useSettingsStore } from "@/stores";
import { useToast } from "@/composables/useToast";
import EntryForm from "./EntryForm.vue";

const entriesStore = useEntriesStore();
const groupsStore = useGroupsStore();
const settingsStore = useSettingsStore();
const { showToast } = useToast();

const showPassword = ref(false);
const showEditModal = ref(false);
const showDeleteConfirm = ref(false);
const isDeleting = ref(false);
const copiedField = ref<string | null>(null);

const entry = computed(() => entriesStore.selectedEntry);

const groupName = computed(() => {
  if (!entry.value?.groupId) return "未分组";
  const group = groupsStore.groups.find((g) => g.id === entry.value!.groupId);
  return group ? `${group.icon} ${group.name}` : "未分组";
});

// Reset password visibility when entry changes
watch(
  () => entry.value?.id,
  () => {
    showPassword.value = false;
  }
);

async function copyToClipboard(text: string, label: string, fieldId: string) {
  try {
    await writeText(text);
    showToast(`${label}已复制到剪贴板`, "success");

    // Show checkmark animation
    copiedField.value = fieldId;
    setTimeout(() => {
      copiedField.value = null;
    }, 2000);

    // Auto-clear clipboard
    const clearSeconds = settingsStore.settings.clearClipboardSeconds;
    if (clearSeconds > 0) {
      setTimeout(async () => {
        await writeText("");
      }, clearSeconds * 1000);
    }
  } catch {
    showToast("复制失败", "error");
  }
}

async function openUrl() {
  if (!entry.value?.url) return;
  try {
    await openExternal(entry.value.url);
  } catch {
    showToast("无法打开链接", "error");
  }
}

async function handleToggleFavorite() {
  if (!entry.value) return;
  await entriesStore.toggleFavorite(entry.value.id);
}

async function handleDelete() {
  if (!entry.value) return;
  isDeleting.value = true;
  try {
    await entriesStore.deleteEntry(entry.value.id);
    groupsStore.fetchEntryCounts();
    showToast("条目已删除", "success");
    showDeleteConfirm.value = false;
  } catch {
    showToast("删除失败", "error");
  } finally {
    isDeleting.value = false;
  }
}

function handleUpdated() {
  showEditModal.value = false;
  entriesStore.fetchEntries(groupsStore.selectedGroupId);
}
</script>

<template>
  <div v-if="entry" class="p-6">
    <!-- Header -->
    <div class="flex items-start justify-between mb-6">
      <div class="flex items-center space-x-4">
        <div
          class="w-16 h-16 rounded-xl bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 flex items-center justify-center font-bold text-2xl"
        >
          {{ entry.title.slice(0, 2).toUpperCase() }}
        </div>
        <div>
          <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100">
            {{ entry.title }}
          </h2>
          <p class="text-sm text-gray-500 dark:text-gray-400">{{ groupName }}</p>
        </div>
      </div>
      <div class="flex items-center space-x-2">
        <Button
          variant="ghost"
          size="sm"
          @click="handleToggleFavorite"
          :title="entry.isFavorite ? '取消收藏' : '添加收藏'"
        >
          <StarSolidIcon
            v-if="entry.isFavorite"
            class="w-5 h-5 text-yellow-500"
          />
          <StarIcon v-else class="w-5 h-5" />
        </Button>
        <Button variant="ghost" size="sm" @click="showEditModal = true">
          <PencilIcon class="w-5 h-5" />
        </Button>
        <Button
          variant="ghost"
          size="sm"
          class="text-red-500 hover:text-red-600"
          @click="showDeleteConfirm = true"
        >
          <TrashIcon class="w-5 h-5" />
        </Button>
      </div>
    </div>

    <!-- Fields -->
    <div class="space-y-4">
      <!-- URL -->
      <div
        v-if="entry.url"
        class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
      >
        <label class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
          网址
        </label>
        <div class="flex items-center justify-between mt-1">
          <a
            :href="entry.url"
            @click.prevent="openUrl"
            class="text-primary-600 dark:text-primary-400 hover:underline truncate"
          >
            {{ entry.url }}
          </a>
          <div class="flex items-center space-x-1 ml-2">
            <Button
              variant="ghost"
              size="sm"
              @click="copyToClipboard(entry.url!, '网址', 'url')"
              class="copy-button"
            >
              <CheckIcon v-if="copiedField === 'url'" class="w-4 h-4 text-green-500" />
              <ClipboardDocumentIcon v-else class="w-4 h-4" />
            </Button>
            <Button variant="ghost" size="sm" @click="openUrl">
              <ArrowTopRightOnSquareIcon class="w-4 h-4" />
            </Button>
          </div>
        </div>
      </div>

      <!-- Username -->
      <div
        v-if="entry.username"
        class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
      >
        <label class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
          用户名
        </label>
        <div class="flex items-center justify-between mt-1">
          <span class="text-gray-900 dark:text-gray-100">{{
            entry.username
          }}</span>
          <Button
            variant="ghost"
            size="sm"
            @click="copyToClipboard(entry.username!, '用户名', 'username')"
            class="copy-button"
          >
            <CheckIcon v-if="copiedField === 'username'" class="w-4 h-4 text-green-500" />
            <ClipboardDocumentIcon v-else class="w-4 h-4" />
          </Button>
        </div>
      </div>

      <!-- Password -->
      <div
        v-if="entry.password"
        class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
      >
        <label class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
          密码
        </label>
        <div class="flex items-center justify-between mt-1">
          <span class="text-gray-900 dark:text-gray-100 font-mono">
            {{ showPassword ? entry.password : "••••••••••••" }}
          </span>
          <div class="flex items-center space-x-1">
            <Button
              variant="ghost"
              size="sm"
              @click="showPassword = !showPassword"
            >
              <EyeSlashIcon v-if="showPassword" class="w-4 h-4" />
              <EyeIcon v-else class="w-4 h-4" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              @click="copyToClipboard(entry.password!, '密码', 'password')"
              class="copy-button"
            >
              <CheckIcon v-if="copiedField === 'password'" class="w-4 h-4 text-green-500" />
              <ClipboardDocumentIcon v-else class="w-4 h-4" />
            </Button>
          </div>
        </div>
      </div>

      <!-- Notes -->
      <div
        v-if="entry.notes"
        class="bg-white dark:bg-gray-800 rounded-lg p-4 border border-gray-200 dark:border-gray-700"
      >
        <label class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
          备注
        </label>
        <p class="mt-1 text-gray-900 dark:text-gray-100 whitespace-pre-wrap">
          {{ entry.notes }}
        </p>
      </div>

      <!-- Metadata -->
      <div class="text-xs text-gray-400 dark:text-gray-500 pt-4">
        <p>创建时间：{{ new Date(entry.createdAt * 1000).toLocaleString() }}</p>
        <p>更新时间：{{ new Date(entry.updatedAt * 1000).toLocaleString() }}</p>
      </div>
    </div>
  </div>

  <!-- Edit Modal -->
  <EntryForm
    :show="showEditModal"
    :entry="entry"
    @close="showEditModal = false"
    @saved="handleUpdated"
  />

  <!-- Delete Confirmation -->
  <Modal
    :show="showDeleteConfirm"
    title="确认删除"
    size="sm"
    @close="showDeleteConfirm = false"
  >
    <p class="text-gray-600 dark:text-gray-400">
      确定要删除"{{ entry?.title }}"吗？此操作无法撤销。
    </p>
    <template #footer>
      <div class="flex justify-end space-x-3">
        <Button variant="secondary" @click="showDeleteConfirm = false">
          取消
        </Button>
        <Button variant="danger" :loading="isDeleting" @click="handleDelete">
          删除
        </Button>
      </div>
    </template>
  </Modal>
</template>

<style scoped>
/* Copy button animation */
.copy-button {
  transition: transform 0.3s ease;
}

.copy-button:active {
  animation: icon-switch 0.3s ease;
}

@keyframes icon-switch {
  0% {
    transform: scale(1) rotate(0deg);
  }
  50% {
    transform: scale(1.2) rotate(180deg);
  }
  100% {
    transform: scale(1) rotate(360deg);
  }
}
</style>
