<script setup lang="ts">
import { onMounted, onUnmounted, watch } from "vue";
import { useRouter } from "vue-router";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { Toast } from "@/components/ui";
import { useToast } from "@/composables/useToast";
import { useAutoLock } from "@/composables/useAutoLock";
import { useAuthStore, useSettingsStore } from "@/stores";

const router = useRouter();
const authStore = useAuthStore();
const settingsStore = useSettingsStore();
const { toasts, removeToast } = useToast();

// Initialize auto-lock monitoring
useAutoLock();

let unlistenLock: UnlistenFn | null = null;

// Initialize app on mount
onMounted(async () => {
  // Initialize theme watcher
  settingsStore.initThemeWatcher();

  // Listen for lock event from system tray
  unlistenLock = await listen("lock-app", () => {
    authStore.lock();
  });

  // Check if app is initialized (has master password)
  const isInitialized = await authStore.checkInitialized();

  if (!isInitialized) {
    router.replace({ name: "setup" });
  } else if (!authStore.isUnlocked) {
    router.replace({ name: "unlock" });
  }
});

// Cleanup on unmount
onUnmounted(() => {
  if (unlistenLock) {
    unlistenLock();
  }
});

// Watch for lock state changes
watch(
  () => authStore.isUnlocked,
  (isUnlocked) => {
    if (!isUnlocked && authStore.isInitialized) {
      router.replace({ name: "unlock" });
    }
  }
);

// Navigation guard
router.beforeEach((to, _from, next) => {
  const requiresAuth = to.meta.requiresAuth;

  if (requiresAuth && !authStore.isUnlocked) {
    if (!authStore.isInitialized) {
      next({ name: "setup" });
    } else {
      next({ name: "unlock" });
    }
  } else {
    next();
  }
});
</script>

<template>
  <RouterView />
  <Toast :toasts="toasts" @remove="removeToast" />
</template>
