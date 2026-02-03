import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Entry, CreateEntryDto, UpdateEntryDto } from "@/types";

export const useEntriesStore = defineStore("entries", () => {
  // State
  const entries = ref<Entry[]>([]);
  const selectedEntryId = ref<string | null>(null);
  const searchKeyword = ref("");
  const showFavoritesOnly = ref(false);
  const isLoading = ref(false);

  // Global counts (independent of current filter)
  const totalCount = ref(0);
  const totalFavoriteCount = ref(0);

  // Getters
  const selectedEntry = computed(() =>
    entries.value.find((e) => e.id === selectedEntryId.value)
  );

  const filteredEntries = computed(() => {
    let result = [...entries.value];

    if (showFavoritesOnly.value) {
      result = result.filter((e) => e.isFavorite);
    }

    if (searchKeyword.value.trim()) {
      const keyword = searchKeyword.value.toLowerCase();
      result = result.filter(
        (e) =>
          e.title.toLowerCase().includes(keyword) ||
          e.username?.toLowerCase().includes(keyword) ||
          e.url?.toLowerCase().includes(keyword) ||
          e.notes?.toLowerCase().includes(keyword)
      );
    }

    return result.sort((a, b) => {
      // Favorites first
      if (a.isFavorite !== b.isFavorite) {
        return a.isFavorite ? -1 : 1;
      }
      // Then by sort order
      return a.sortOrder - b.sortOrder;
    });
  });

  const favoriteEntries = computed(() =>
    entries.value.filter((e) => e.isFavorite)
  );

  // Actions
  async function fetchEntries(groupId?: string | null, favoritesOnly?: boolean) {
    isLoading.value = true;
    try {
      entries.value = await invoke<Entry[]>("get_entries", {
        groupId,
        search: searchKeyword.value || null,
        favoritesOnly: favoritesOnly ?? showFavoritesOnly.value,
      });
    } catch (error) {
      console.error("Failed to fetch entries:", error);
      throw error;
    } finally {
      isLoading.value = false;
    }
  }

  // Fetch global counts (all entries, ignoring current filter)
  async function fetchTotalCounts() {
    try {
      const allEntries = await invoke<Entry[]>("get_entries", {
        groupId: null,
        search: null,
        favoritesOnly: false,
      });
      totalCount.value = allEntries.length;
      totalFavoriteCount.value = allEntries.filter((e) => e.isFavorite).length;
    } catch (error) {
      console.error("Failed to fetch total counts:", error);
    }
  }

  async function createEntry(dto: CreateEntryDto) {
    try {
      const entry = await invoke<Entry>("create_entry", { entry: dto });
      entries.value.push(entry);
      // Update total count
      totalCount.value++;
      return entry;
    } catch (error) {
      console.error("Failed to create entry:", error);
      throw error;
    }
  }

  async function updateEntry(id: string, dto: UpdateEntryDto) {
    try {
      const updated = await invoke<Entry>("update_entry", { id, entry: dto });
      const index = entries.value.findIndex((e) => e.id === id);
      if (index !== -1) {
        entries.value[index] = updated;
      }
      return updated;
    } catch (error) {
      console.error("Failed to update entry:", error);
      throw error;
    }
  }

  async function deleteEntry(id: string) {
    try {
      const entry = entries.value.find((e) => e.id === id);
      await invoke("delete_entry", { id });
      entries.value = entries.value.filter((e) => e.id !== id);
      if (selectedEntryId.value === id) {
        selectedEntryId.value = null;
      }
      // Update total counts
      totalCount.value--;
      if (entry?.isFavorite) {
        totalFavoriteCount.value--;
      }
    } catch (error) {
      console.error("Failed to delete entry:", error);
      throw error;
    }
  }

  async function toggleFavorite(id: string) {
    try {
      const isFavorite = await invoke<boolean>("toggle_favorite", { id });
      const entry = entries.value.find((e) => e.id === id);
      if (entry) {
        entry.isFavorite = isFavorite;
      }
      // Update favorite count
      if (isFavorite) {
        totalFavoriteCount.value++;
      } else {
        totalFavoriteCount.value--;
      }
      return isFavorite;
    } catch (error) {
      console.error("Failed to toggle favorite:", error);
      throw error;
    }
  }

  function selectEntry(id: string | null) {
    selectedEntryId.value = id;
  }

  function setSearchKeyword(keyword: string) {
    searchKeyword.value = keyword;
  }

  function setShowFavoritesOnly(show: boolean) {
    showFavoritesOnly.value = show;
  }

  function clearSelection() {
    selectedEntryId.value = null;
  }

  return {
    // State
    entries,
    selectedEntryId,
    searchKeyword,
    showFavoritesOnly,
    isLoading,
    totalCount,
    totalFavoriteCount,
    // Getters
    selectedEntry,
    filteredEntries,
    favoriteEntries,
    // Actions
    fetchEntries,
    fetchTotalCounts,
    createEntry,
    updateEntry,
    deleteEntry,
    toggleFavorite,
    selectEntry,
    setSearchKeyword,
    setShowFavoritesOnly,
    clearSelection,
  };
});
