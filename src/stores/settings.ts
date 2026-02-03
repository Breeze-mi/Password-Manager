import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Settings } from "@/types";

export const useSettingsStore = defineStore("settings", () => {
  // State
  const settings = ref<Settings>({
    autoLockMinutes: 5,
    clearClipboardSeconds: 30,
    theme: "system",
  });
  const isLoading = ref(false);

  // Actions
  async function fetchSettings() {
    isLoading.value = true;
    try {
      settings.value = await invoke<Settings>("get_settings");
      applyTheme(settings.value.theme);
    } catch (error) {
      console.error("Failed to fetch settings:", error);
      throw error;
    } finally {
      isLoading.value = false;
    }
  }

  async function updateSettings(newSettings: Partial<Settings>) {
    try {
      const updated = { ...settings.value, ...newSettings };
      await invoke("update_settings", { settings: updated });
      settings.value = updated;

      if (newSettings.theme) {
        localStorage.setItem("one-password-theme", newSettings.theme);
        applyTheme(newSettings.theme);
      }
    } catch (error) {
      console.error("Failed to update settings:", error);
      throw error;
    }
  }

  function applyTheme(theme: Settings["theme"]) {
    const root = document.documentElement;

    // Add transition class
    root.classList.add("theme-transition");

    // Apply theme
    const isDark =
      theme === "dark" ||
      (theme === "system" &&
        window.matchMedia("(prefers-color-scheme: dark)").matches);

    root.classList.toggle("dark", isDark);

    // Remove transition class after animation completes
    setTimeout(() => {
      root.classList.remove("theme-transition");
    }, 300);
  }

  // Watch for system theme changes
  function initThemeWatcher() {
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    mediaQuery.addEventListener("change", () => {
      if (settings.value.theme === "system") {
        applyTheme("system");
      }
    });
  }

  return {
    // State
    settings,
    isLoading,
    // Actions
    fetchSettings,
    updateSettings,
    applyTheme,
    initThemeWatcher,
  };
});
