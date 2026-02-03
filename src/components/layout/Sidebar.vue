<script setup lang="ts">
import { ref, computed } from "vue";
import {
  FolderIcon,
  StarIcon,
  Cog6ToothIcon,
  PlusIcon,
  LockClosedIcon,
} from "@heroicons/vue/24/outline";
import { StarIcon as StarSolidIcon } from "@heroicons/vue/24/solid";
import { Button, Modal, Input } from "@/components/ui";
import { useGroupsStore, useEntriesStore, useAuthStore } from "@/stores";
import { useToast } from "@/composables/useToast";
import GroupItem from "@/components/group/GroupItem.vue";
import SettingsDialog from "@/components/settings/SettingsDialog.vue";

const groupsStore = useGroupsStore();
const entriesStore = useEntriesStore();
const authStore = useAuthStore();
const { showToast } = useToast();

// View state
const currentView = ref<"all" | "favorites">("all");

// Create group modal
const showCreateModal = ref(false);
const newGroupName = ref("");
const newGroupIcon = ref("📁");
const isCreating = ref(false);

// Settings dialog
const showSettings = ref(false);

const totalCount = computed(() => entriesStore.totalCount);
const favoriteCount = computed(() => entriesStore.totalFavoriteCount);

function handleViewAll() {
  currentView.value = "all";
  groupsStore.selectGroup(null);
  entriesStore.setShowFavoritesOnly(false);
  entriesStore.fetchEntries();
}

function handleViewFavorites() {
  currentView.value = "favorites";
  groupsStore.selectGroup(null);
  entriesStore.setShowFavoritesOnly(true);
  entriesStore.fetchEntries(null, true);
}

function handleSelectGroup(groupId: string) {
  currentView.value = "all";
  groupsStore.selectGroup(groupId);
  entriesStore.setShowFavoritesOnly(false);
  entriesStore.fetchEntries(groupId);
}

async function handleCreateGroup() {
  if (!newGroupName.value.trim()) return;

  isCreating.value = true;
  try {
    await groupsStore.createGroup(newGroupName.value, newGroupIcon.value);
    showToast("分组创建成功", "success");
    showCreateModal.value = false;
    newGroupName.value = "";
    newGroupIcon.value = "📁";
  } catch {
    showToast("创建分组失败", "error");
  } finally {
    isCreating.value = false;
  }
}

function handleLock() {
  authStore.lock();
}

const iconOptions = ["📁", "🏢", "🏠", "🏦", "🎮", "🛒", "📧", "🌐", "🔧", "💼"];
</script>

<template>
  <aside
    class="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col"
  >
    <!-- App Title -->
    <div class="p-4 border-b border-gray-200 dark:border-gray-700">
      <div class="flex items-center space-x-2">
        <span class="text-2xl">🔐</span>
        <h1 class="text-lg font-bold text-gray-900 dark:text-gray-100">
          One-Password
        </h1>
      </div>
    </div>

    <!-- Navigation -->
    <nav class="flex-1 p-4 space-y-1 overflow-y-auto">
      <!-- All Items -->
      <button
        @click="handleViewAll"
        class="w-full flex items-center justify-between px-3 py-2 rounded-lg transition-colors"
        :class="
          currentView === 'all' && !groupsStore.selectedGroupId
            ? 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300'
            : 'hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300'
        "
      >
        <div class="flex items-center space-x-3">
          <FolderIcon class="w-5 h-5" />
          <span>全部</span>
        </div>
        <span class="text-sm text-gray-500 dark:text-gray-400">{{
          totalCount
        }}</span>
      </button>

      <!-- Favorites -->
      <button
        @click="handleViewFavorites"
        class="w-full flex items-center justify-between px-3 py-2 rounded-lg transition-colors"
        :class="
          currentView === 'favorites'
            ? 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300'
            : 'hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300'
        "
      >
        <div class="flex items-center space-x-3">
          <StarSolidIcon
            v-if="currentView === 'favorites'"
            class="w-5 h-5 text-yellow-500"
          />
          <StarIcon v-else class="w-5 h-5" />
          <span>收藏</span>
        </div>
        <span class="text-sm text-gray-500 dark:text-gray-400">{{
          favoriteCount
        }}</span>
      </button>

      <!-- Divider -->
      <div class="pt-4 pb-2">
        <div
          class="flex items-center justify-between text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider"
        >
          <span>分组</span>
          <button
            @click="showCreateModal = true"
            class="p-1 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
          >
            <PlusIcon class="w-4 h-4" />
          </button>
        </div>
      </div>

      <!-- Groups -->
      <div class="space-y-1">
        <GroupItem
          v-for="group in groupsStore.sortedGroups"
          :key="group.id"
          :group="group"
          :selected="groupsStore.selectedGroupId === group.id"
          :count="groupsStore.getEntryCount(group.id)"
          @click="handleSelectGroup(group.id)"
        />
      </div>
    </nav>

    <!-- Bottom Actions -->
    <div class="p-4 border-t border-gray-200 dark:border-gray-700 space-y-2">
      <Button
        variant="ghost"
        size="sm"
        class="w-full justify-start"
        @click="showSettings = true"
      >
        <Cog6ToothIcon class="w-5 h-5 mr-2" />
        设置
      </Button>
      <Button
        variant="ghost"
        size="sm"
        class="w-full justify-start text-red-600 hover:text-red-700 hover:bg-red-50 dark:hover:bg-red-900/20"
        @click="handleLock"
      >
        <LockClosedIcon class="w-5 h-5 mr-2" />
        锁定
      </Button>
    </div>
  </aside>

  <!-- Create Group Modal -->
  <Modal :show="showCreateModal" title="新建分组" @close="showCreateModal = false">
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          图标
        </label>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="icon in iconOptions"
            :key="icon"
            @click="newGroupIcon = icon"
            class="w-10 h-10 flex items-center justify-center text-xl rounded-lg border-2 transition-colors"
            :class="
              newGroupIcon === icon
                ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/30'
                : 'border-gray-200 dark:border-gray-600 hover:border-gray-300'
            "
          >
            {{ icon }}
          </button>
        </div>
      </div>

      <Input
        v-model="newGroupName"
        label="分组名称"
        placeholder="请输入分组名称"
      />
    </div>

    <template #footer>
      <div class="flex justify-end space-x-3">
        <Button variant="secondary" @click="showCreateModal = false">
          取消
        </Button>
        <Button
          variant="primary"
          :loading="isCreating"
          :disabled="!newGroupName.trim()"
          @click="handleCreateGroup"
        >
          创建
        </Button>
      </div>
    </template>
  </Modal>

  <!-- Settings Dialog -->
  <SettingsDialog :show="showSettings" @close="showSettings = false" />
</template>
