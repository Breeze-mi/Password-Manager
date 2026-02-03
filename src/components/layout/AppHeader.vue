<script setup lang="ts">
import { ref, watch } from "vue";
import {
  MagnifyingGlassIcon,
  PlusIcon,
  XMarkIcon,
} from "@heroicons/vue/24/outline";
import { Button } from "@/components/ui";
import { useEntriesStore, useGroupsStore } from "@/stores";
import EntryForm from "@/components/entry/EntryForm.vue";

const entriesStore = useEntriesStore();
const groupsStore = useGroupsStore();

const showSearch = ref(false);
const searchInput = ref("");
const showCreateEntry = ref(false);

// Debounce search
let searchTimeout: ReturnType<typeof setTimeout> | null = null;
watch(searchInput, (value) => {
  if (searchTimeout) clearTimeout(searchTimeout);
  searchTimeout = setTimeout(() => {
    entriesStore.setSearchKeyword(value);
    entriesStore.fetchEntries(groupsStore.selectedGroupId);
  }, 300);
});

function toggleSearch() {
  showSearch.value = !showSearch.value;
  if (!showSearch.value) {
    searchInput.value = "";
    entriesStore.setSearchKeyword("");
  }
}

function handleEntryCreated() {
  showCreateEntry.value = false;
  entriesStore.fetchEntries(groupsStore.selectedGroupId);
  groupsStore.fetchEntryCounts();
}
</script>

<template>
  <header
    class="h-14 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between px-4"
  >
    <!-- Left: Title / Search -->
    <div class="flex-1 flex items-center">
      <div v-if="showSearch" class="flex items-center w-full max-w-md">
        <div class="relative w-full">
          <MagnifyingGlassIcon
            class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400"
          />
          <input
            v-model="searchInput"
            type="text"
            placeholder="搜索条目..."
            class="w-full pl-10 pr-10 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent"
            autofocus
          />
          <button
            @click="toggleSearch"
            class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
          >
            <XMarkIcon class="w-5 h-5" />
          </button>
        </div>
      </div>
      <div v-else>
        <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
          {{
            groupsStore.selectedGroup?.name ||
            (entriesStore.showFavoritesOnly ? "收藏" : "全部条目")
          }}
        </h2>
      </div>
    </div>

    <!-- Right: Actions -->
    <div class="flex items-center space-x-2">
      <Button v-if="!showSearch" variant="ghost" size="sm" @click="toggleSearch">
        <MagnifyingGlassIcon class="w-5 h-5" />
      </Button>
      <Button variant="primary" size="sm" @click="showCreateEntry = true">
        <PlusIcon class="w-5 h-5 mr-1" />
        新建
      </Button>
    </div>
  </header>

  <!-- Create Entry Modal -->
  <EntryForm
    :show="showCreateEntry"
    @close="showCreateEntry = false"
    @saved="handleEntryCreated"
  />
</template>
