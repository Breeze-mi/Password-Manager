import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Group } from "@/types";

export const useGroupsStore = defineStore("groups", () => {
  // State
  const groups = ref<Group[]>([]);
  const selectedGroupId = ref<string | null>(null);
  const entryCounts = ref<Map<string, number>>(new Map());
  const isLoading = ref(false);

  // Getters
  const selectedGroup = computed(() =>
    groups.value.find((g) => g.id === selectedGroupId.value)
  );

  const sortedGroups = computed(() =>
    [...groups.value].sort((a, b) => a.sortOrder - b.sortOrder)
  );

  // Actions
  async function fetchGroups() {
    isLoading.value = true;
    try {
      groups.value = await invoke<Group[]>("get_groups");
      await fetchEntryCounts();
    } catch (error) {
      console.error("Failed to fetch groups:", error);
      throw error;
    } finally {
      isLoading.value = false;
    }
  }

  async function fetchEntryCounts() {
    try {
      const counts = await invoke<[string | null, number][]>(
        "get_group_entry_counts"
      );
      entryCounts.value = new Map(
        counts.map(([id, count]) => [id ?? "uncategorized", count])
      );
    } catch (error) {
      console.error("Failed to fetch entry counts:", error);
    }
  }

  async function createGroup(name: string, icon?: string) {
    try {
      const group = await invoke<Group>("create_group", { name, icon });
      groups.value.push(group);
      return group;
    } catch (error) {
      console.error("Failed to create group:", error);
      throw error;
    }
  }

  async function updateGroup(id: string, name?: string, icon?: string) {
    try {
      const updated = await invoke<Group>("update_group", { id, name, icon });
      const index = groups.value.findIndex((g) => g.id === id);
      if (index !== -1) {
        groups.value[index] = updated;
      }
      return updated;
    } catch (error) {
      console.error("Failed to update group:", error);
      throw error;
    }
  }

  async function deleteGroup(id: string) {
    try {
      await invoke("delete_group", { id });
      groups.value = groups.value.filter((g) => g.id !== id);
      if (selectedGroupId.value === id) {
        selectedGroupId.value = null;
      }
    } catch (error) {
      console.error("Failed to delete group:", error);
      throw error;
    }
  }

  function selectGroup(id: string | null) {
    selectedGroupId.value = id;
  }

  function getEntryCount(groupId: string | null): number {
    return entryCounts.value.get(groupId ?? "uncategorized") ?? 0;
  }

  return {
    // State
    groups,
    selectedGroupId,
    entryCounts,
    isLoading,
    // Getters
    selectedGroup,
    sortedGroups,
    // Actions
    fetchGroups,
    fetchEntryCounts,
    createGroup,
    updateGroup,
    deleteGroup,
    selectGroup,
    getEntryCount,
  };
});
