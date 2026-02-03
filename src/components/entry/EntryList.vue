<script setup lang="ts">
import { ref } from "vue";
import { useEntriesStore, useGroupsStore } from "@/stores";
import { useToast } from "@/composables/useToast";
import { Modal, Button } from "@/components/ui";
import EntryCard from "./EntryCard.vue";

const entriesStore = useEntriesStore();
const groupsStore = useGroupsStore();
const { showToast } = useToast();

const showDeleteConfirm = ref(false);
const entryToDelete = ref<{ id: string; title: string } | null>(null);
const isDeleting = ref(false);

async function handleToggleFavorite(id: string) {
  await entriesStore.toggleFavorite(id);
}

function confirmDelete(id: string, title: string) {
  entryToDelete.value = { id, title };
  showDeleteConfirm.value = true;
}

async function handleDelete() {
  if (!entryToDelete.value) return;

  isDeleting.value = true;
  try {
    await entriesStore.deleteEntry(entryToDelete.value.id);
    await groupsStore.fetchEntryCounts();
    showToast("条目已删除", "success");
    showDeleteConfirm.value = false;
    entryToDelete.value = null;
  } catch {
    showToast("删除失败", "error");
  } finally {
    isDeleting.value = false;
  }
}

function cancelDelete() {
  showDeleteConfirm.value = false;
  entryToDelete.value = null;
}
</script>

<template>
  <div class="h-full">
    <!-- Loading State with Skeleton -->
    <div v-if="entriesStore.isLoading" class="space-y-2 p-4">
      <div
        v-for="i in 5"
        :key="i"
        class="animate-pulse bg-gray-200 dark:bg-gray-700 rounded-lg p-4 h-20"
      >
        <div class="h-4 bg-gray-300 dark:bg-gray-600 rounded w-3/4 mb-2"></div>
        <div class="h-3 bg-gray-300 dark:bg-gray-600 rounded w-1/2"></div>
      </div>
    </div>

    <!-- Empty State -->
    <div
      v-else-if="entriesStore.filteredEntries.length === 0"
      class="flex flex-col items-center justify-center h-full text-gray-400 dark:text-gray-500 p-8"
    >
      <div class="text-5xl mb-4">📭</div>
      <p class="text-center">
        {{
          entriesStore.searchKeyword
            ? "没有找到匹配的条目"
            : entriesStore.showFavoritesOnly
              ? "还没有收藏的条目"
              : "还没有添加任何条目"
        }}
      </p>
    </div>

    <!-- Entry List with Transition -->
    <TransitionGroup
      v-else
      name="list"
      tag="div"
      class="space-y-2"
    >
      <EntryCard
        v-for="entry in entriesStore.filteredEntries"
        :key="entry.id"
        :entry="entry"
        :selected="entriesStore.selectedEntryId === entry.id"
        @click="entriesStore.selectEntry(entry.id)"
        @toggle-favorite="handleToggleFavorite(entry.id)"
        @delete="confirmDelete(entry.id, entry.title)"
      />
    </TransitionGroup>

    <!-- Delete Confirmation Modal -->
    <Modal
      :show="showDeleteConfirm"
      title="确认删除"
      size="sm"
      @close="cancelDelete"
    >
      <p class="text-gray-600 dark:text-gray-400">
        确定要删除"{{ entryToDelete?.title }}"吗？此操作无法撤销。
      </p>
      <template #footer>
        <div class="flex justify-end space-x-3">
          <Button variant="secondary" @click="cancelDelete">取消</Button>
          <Button variant="danger" :loading="isDeleting" @click="handleDelete">
            删除
          </Button>
        </div>
      </template>
    </Modal>
  </div>
</template>

<style scoped>
/* List transition animations */
.list-enter-active,
.list-leave-active {
  transition: all 0.3s ease;
}

.list-enter-from {
  opacity: 0;
  transform: translateX(-30px);
}

.list-leave-to {
  opacity: 0;
  transform: translateX(30px);
}

.list-move {
  transition: transform 0.3s ease;
}
</style>
