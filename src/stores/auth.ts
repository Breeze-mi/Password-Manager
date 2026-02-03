import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

export const useAuthStore = defineStore("auth", () => {
  // State
  const isInitialized = ref(false);
  const isUnlocked = ref(false);
  const lastActivityTime = ref(Date.now());

  // Getters
  const isLocked = computed(() => !isUnlocked.value);

  // Actions
  async function checkInitialized() {
    try {
      isInitialized.value = await invoke<boolean>("check_initialized");
      return isInitialized.value;
    } catch (error) {
      console.error("Failed to check initialization:", error);
      return false;
    }
  }

  async function setupPassword(password: string) {
    try {
      await invoke("setup_password", { password });
      isInitialized.value = true;
      isUnlocked.value = true;
      updateLastActivity();
      return true;
    } catch (error) {
      console.error("Failed to setup password:", error);
      throw error;
    }
  }

  async function verifyPassword(password: string) {
    try {
      const isValid = await invoke<boolean>("verify_password", { password });
      if (isValid) {
        isUnlocked.value = true;
        updateLastActivity();
      }
      return isValid;
    } catch (error) {
      console.error("Failed to verify password:", error);
      return false;
    }
  }

  async function changePassword(oldPassword: string, newPassword: string) {
    try {
      await invoke("change_password", { oldPassword, newPassword });
      return true;
    } catch (error) {
      console.error("Failed to change password:", error);
      throw error;
    }
  }

  function lock() {
    isUnlocked.value = false;
  }

  function updateLastActivity() {
    lastActivityTime.value = Date.now();
  }

  return {
    // State
    isInitialized,
    isUnlocked,
    lastActivityTime,
    // Getters
    isLocked,
    // Actions
    checkInitialized,
    setupPassword,
    verifyPassword,
    changePassword,
    lock,
    updateLastActivity,
  };
});
