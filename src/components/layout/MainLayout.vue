<script setup lang="ts">
import { computed } from "vue";
import Sidebar from "./Sidebar.vue";
import AppHeader from "./AppHeader.vue";
import EntryList from "@/components/entry/EntryList.vue";
import EntryDetail from "@/components/entry/EntryDetail.vue";
import { useEntriesStore } from "@/stores";

const entriesStore = useEntriesStore();

const showDetail = computed(() => entriesStore.selectedEntry !== undefined);
</script>

<template>
  <div class="h-screen flex bg-gray-100 dark:bg-gray-900">
    <!-- Sidebar -->
    <Sidebar />

    <!-- Main Content -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <!-- Header -->
      <AppHeader />

      <!-- Content Area -->
      <div class="flex-1 flex overflow-hidden">
        <!-- Entry List -->
        <div
          class="w-80 border-r border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 overflow-y-auto"
        >
          <EntryList />
        </div>

        <!-- Entry Detail -->
        <div class="flex-1 bg-gray-50 dark:bg-gray-900 overflow-y-auto">
          <EntryDetail v-if="showDetail" />
          <div
            v-else
            class="h-full flex items-center justify-center text-gray-400"
          >
            <div class="text-center">
              <div class="text-5xl mb-4">📋</div>
              <p>选择一个条目查看详情</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
