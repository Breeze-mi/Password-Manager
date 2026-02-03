import { ref, onMounted, onUnmounted, watch } from "vue";
import { useAuthStore } from "@/stores/auth";
import { useSettingsStore } from "@/stores/settings";

export function useAutoLock() {
  const authStore = useAuthStore();
  const settingsStore = useSettingsStore();
  const remainingSeconds = ref(0);

  let checkIntervalId: number | null = null;
  let lastThrottleTime = 0;
  const THROTTLE_INTERVAL = 1000; // 1 second throttle

  // Activity events to monitor
  const activityEvents = [
    "mousedown",
    "mousemove",
    "keydown",
    "scroll",
    "touchstart",
    "click",
  ];

  // Throttled activity handler
  function handleActivity() {
    const now = Date.now();
    if (now - lastThrottleTime >= THROTTLE_INTERVAL) {
      lastThrottleTime = now;
      authStore.updateLastActivity();
    }
  }

  // Check if auto-lock timeout has been reached
  function checkTimeout() {
    const autoLockMinutes = settingsStore.settings.autoLockMinutes;

    // 0 means never auto-lock
    if (autoLockMinutes === 0 || !authStore.isUnlocked) {
      remainingSeconds.value = 0;
      return;
    }

    const elapsed = Date.now() - authStore.lastActivityTime;
    const timeout = autoLockMinutes * 60 * 1000;
    const remaining = timeout - elapsed;

    if (remaining <= 0) {
      // Time's up, lock the app
      authStore.lock();
      remainingSeconds.value = 0;
    } else {
      // Update remaining seconds for UI
      remainingSeconds.value = Math.ceil(remaining / 1000);
    }
  }

  // Start monitoring
  function startMonitoring() {
    // Add activity event listeners
    activityEvents.forEach((event) => {
      window.addEventListener(event, handleActivity, { passive: true });
    });

    // Start interval to check timeout every second
    checkIntervalId = window.setInterval(checkTimeout, 1000);
  }

  // Stop monitoring
  function stopMonitoring() {
    // Remove activity event listeners
    activityEvents.forEach((event) => {
      window.removeEventListener(event, handleActivity);
    });

    // Clear interval
    if (checkIntervalId !== null) {
      clearInterval(checkIntervalId);
      checkIntervalId = null;
    }
  }

  // Watch for unlock state changes
  watch(
    () => authStore.isUnlocked,
    (isUnlocked) => {
      if (isUnlocked) {
        // Reset activity time when unlocked
        authStore.updateLastActivity();
        startMonitoring();
      } else {
        stopMonitoring();
      }
    }
  );

  // Watch for settings changes
  watch(
    () => settingsStore.settings.autoLockMinutes,
    () => {
      // Reset activity time when settings change
      if (authStore.isUnlocked) {
        authStore.updateLastActivity();
      }
    }
  );

  // Setup and cleanup
  onMounted(() => {
    if (authStore.isUnlocked) {
      startMonitoring();
    }
  });

  onUnmounted(() => {
    stopMonitoring();
  });

  return {
    remainingSeconds,
  };
}
