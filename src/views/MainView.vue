<script setup lang="ts">
import { onMounted } from "vue";
import MainLayout from "@/components/layout/MainLayout.vue";
import { useGroupsStore, useEntriesStore, useSettingsStore } from "@/stores";

const groupsStore = useGroupsStore();
const entriesStore = useEntriesStore();
const settingsStore = useSettingsStore();

onMounted(async () => {
  // Load initial data
  await Promise.all([
    groupsStore.fetchGroups(),
    entriesStore.fetchEntries(),
    entriesStore.fetchTotalCounts(),
    settingsStore.fetchSettings(),
  ]);
});
</script>

<template>
  <MainLayout />
</template>
