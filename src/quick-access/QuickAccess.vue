<template>
  <div
    class="h-screen flex flex-col bg-white dark:bg-gray-900 rounded-lg overflow-hidden border border-gray-200 dark:border-gray-700"
    @keydown.esc="closeWindow"
  >
    <!-- Custom title bar -->
    <div
      @mousedown="startDrag"
      class="flex items-center justify-between px-3 py-2 bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 cursor-move"
    >
      <div class="flex items-center gap-2">
        <KeyIcon class="w-4 h-4 text-blue-500" />
        <span class="text-sm font-medium text-gray-700 dark:text-gray-200"
          >Quick Access</span
        >
      </div>
      <div class="flex items-center gap-1" @mousedown.stop>
        <button
          @click="togglePin"
          :class="[
            'p-1.5 rounded hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors',
            isPinned
              ? 'text-blue-500'
              : 'text-gray-400 hover:text-gray-600 dark:hover:text-gray-300',
          ]"
          :title="isPinned ? '取消固定' : '固定窗口'"
        >
          <component
            :is="isPinned ? LockClosedIcon : LockOpenIcon"
            class="w-4 h-4"
          />
        </button>
        <button
          @click="closeWindow"
          class="p-1.5 rounded hover:bg-red-100 dark:hover:bg-red-900/30 text-gray-400 hover:text-red-500 transition-colors"
          title="关闭 (ESC)"
        >
          <XMarkIcon class="w-4 h-4" />
        </button>
      </div>
    </div>

    <!-- Unlock screen -->
    <div v-if="!authStore.isUnlocked" class="flex-1 flex flex-col p-4">
      <div class="flex-1 flex items-center justify-center">
        <div class="w-full max-w-xs">
          <div class="text-center mb-6">
            <LockClosedIcon
              class="w-12 h-12 mx-auto text-gray-400 dark:text-gray-500 mb-2"
            />
            <h2 class="text-lg font-medium text-gray-700 dark:text-gray-200">
              输入主密码
            </h2>
          </div>
          <form @submit.prevent="handleUnlock">
            <input
              ref="passwordInput"
              v-model="password"
              type="password"
              placeholder="主密码"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-3"
              :disabled="isUnlocking"
              autocomplete="current-password"
            />
            <button
              type="submit"
              :disabled="isUnlocking || !password"
              class="w-full py-2 bg-blue-500 hover:bg-blue-600 disabled:bg-gray-300 dark:disabled:bg-gray-700 text-white rounded-lg font-medium transition-colors"
            >
              {{ isUnlocking ? "验证中..." : "解锁" }}
            </button>
            <p
              v-if="unlockError"
              class="mt-2 text-sm text-red-500 text-center"
            >
              {{ unlockError }}
            </p>
          </form>
        </div>
      </div>
    </div>

    <!-- Main content -->
    <template v-else>
      <!-- Search box -->
      <div class="p-3 border-b border-gray-200 dark:border-gray-700">
        <div class="relative">
          <MagnifyingGlassIcon
            class="w-5 h-5 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
          />
          <input
            ref="searchInput"
            v-model="searchKeyword"
            type="text"
            placeholder="搜索密码..."
            class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
            @input="handleSearch"
          />
        </div>
      </div>

      <!-- Entry list -->
      <div class="flex-1 overflow-y-auto">
        <!-- Loading state -->
        <div
          v-if="isLoading"
          class="flex items-center justify-center py-8 text-gray-500"
        >
          <svg
            class="animate-spin h-5 w-5 mr-2"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            ></circle>
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
            ></path>
          </svg>
          加载中...
        </div>

        <!-- Empty state -->
        <div
          v-else-if="filteredEntries.length === 0"
          class="flex flex-col items-center justify-center py-8 text-gray-500"
        >
          <KeyIcon class="w-10 h-10 mb-2 opacity-50" />
          <p class="text-sm">
            {{ searchKeyword ? "未找到匹配的密码" : "暂无密码条目" }}
          </p>
        </div>

        <!-- Entry items -->
        <div v-else class="divide-y divide-gray-100 dark:divide-gray-800">
          <div
            v-for="entry in filteredEntries"
            :key="entry.id"
            class="flex items-center gap-3 px-3 py-2.5 hover:bg-gray-50 dark:hover:bg-gray-800 cursor-pointer group"
            @click="selectEntry(entry)"
          >
            <!-- Icon -->
            <div
              class="flex-shrink-0 w-8 h-8 rounded-lg bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center text-white text-xs font-bold"
            >
              {{ getInitials(entry.title) }}
            </div>

            <!-- Info -->
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-1.5">
                <span
                  class="text-sm font-medium text-gray-900 dark:text-gray-100 truncate"
                  >{{ entry.title }}</span
                >
                <StarIcon
                  v-if="entry.isFavorite"
                  class="w-3.5 h-3.5 text-yellow-400 flex-shrink-0"
                />
              </div>
              <p
                v-if="entry.username"
                class="text-xs text-gray-500 dark:text-gray-400 truncate"
              >
                {{ entry.username }}
              </p>
            </div>

            <!-- Quick actions -->
            <div
              class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
            >
              <button
                @click.stop="copyUsername(entry)"
                :disabled="!entry.username"
                class="p-1.5 rounded hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 disabled:opacity-30 disabled:cursor-not-allowed"
                title="复制用户名"
              >
                <UserIcon class="w-4 h-4" />
              </button>
              <button
                @click.stop="copyPassword(entry)"
                :disabled="!entry.password"
                class="p-1.5 rounded hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 disabled:opacity-30 disabled:cursor-not-allowed"
                title="复制密码"
              >
                <KeyIcon class="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Selected entry detail -->
      <div
        v-if="selectedEntry"
        class="border-t border-gray-200 dark:border-gray-700 p-3 bg-gray-50 dark:bg-gray-800"
      >
        <div class="flex items-center justify-between mb-2">
          <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100">
            {{ selectedEntry.title }}
          </h3>
          <button
            @click="selectedEntry = null"
            class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
          >
            <XMarkIcon class="w-4 h-4" />
          </button>
        </div>

        <div class="space-y-2">
          <!-- Username -->
          <div v-if="selectedEntry.username" class="flex items-center gap-2">
            <span class="text-xs text-gray-500 w-12">用户名</span>
            <span
              class="flex-1 text-sm text-gray-700 dark:text-gray-300 truncate"
              >{{ selectedEntry.username }}</span
            >
            <button
              @click="copyUsername(selectedEntry)"
              class="p-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-400 hover:text-blue-500"
              title="复制"
            >
              <ClipboardDocumentIcon class="w-4 h-4" />
            </button>
          </div>

          <!-- Password -->
          <div v-if="selectedEntry.password" class="flex items-center gap-2">
            <span class="text-xs text-gray-500 w-12">密码</span>
            <span
              class="flex-1 text-sm text-gray-700 dark:text-gray-300 font-mono"
              >{{ showPassword ? selectedEntry.password : "••••••••" }}</span
            >
            <button
              @click="showPassword = !showPassword"
              class="p-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
              :title="showPassword ? '隐藏' : '显示'"
            >
              <component
                :is="showPassword ? EyeSlashIcon : EyeIcon"
                class="w-4 h-4"
              />
            </button>
            <button
              @click="copyPassword(selectedEntry)"
              class="p-1 rounded hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-400 hover:text-blue-500"
              title="复制"
            >
              <ClipboardDocumentIcon class="w-4 h-4" />
            </button>
          </div>

          <!-- URL -->
          <div v-if="selectedEntry.url" class="flex items-center gap-2">
            <span class="text-xs text-gray-500 w-12">网址</span>
            <a
              :href="selectedEntry.url"
              target="_blank"
              class="flex-1 text-sm text-blue-500 hover:text-blue-600 truncate"
              @click.prevent="openUrl(selectedEntry.url)"
            >
              {{ selectedEntry.url }}
            </a>
          </div>
        </div>
      </div>

      <!-- Footer with shortcut hint -->
      <div
        class="px-3 py-1.5 bg-gray-50 dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700"
      >
        <div class="flex items-center justify-between text-xs text-gray-400">
          <span>{{ filteredEntries.length }} 条结果</span>
          <span>ESC 关闭 · Ctrl+Shift+P 唤醒</span>
        </div>
      </div>
    </template>

    <!-- Toast notification -->
    <Transition
      enter-active-class="transition-all duration-200 ease-out"
      leave-active-class="transition-all duration-150 ease-in"
      enter-from-class="opacity-0 translate-y-2"
      leave-to-class="opacity-0 translate-y-2"
    >
      <div
        v-if="toast"
        :class="[
          'fixed bottom-16 left-1/2 -translate-x-1/2 px-4 py-2 rounded-lg text-sm font-medium shadow-lg',
          toast.type === 'success'
            ? 'bg-green-500 text-white'
            : toast.type === 'error'
              ? 'bg-red-500 text-white'
              : 'bg-gray-800 text-white',
        ]"
      >
        {{ toast.message }}
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { openUrl as openExternal } from "@tauri-apps/plugin-opener";
import type { UnlistenFn } from "@tauri-apps/api/event";
import {
  KeyIcon,
  XMarkIcon,
  MagnifyingGlassIcon,
  StarIcon,
  UserIcon,
  ClipboardDocumentIcon,
  EyeIcon,
  EyeSlashIcon,
  LockClosedIcon,
  LockOpenIcon,
} from "@heroicons/vue/24/outline";
import { useAuthStore } from "@/stores/auth";
import type { Entry } from "@/types";

const authStore = useAuthStore();

// Refs
const searchInput = ref<HTMLInputElement | null>(null);
const passwordInput = ref<HTMLInputElement | null>(null);
const searchKeyword = ref("");
const password = ref("");
const isUnlocking = ref(false);
const unlockError = ref("");
const isLoading = ref(false);
const isPinned = ref(false);
const showPassword = ref(false);
const entries = ref<Entry[]>([]);
const selectedEntry = ref<Entry | null>(null);
const toast = ref<{ type: "success" | "error" | "info"; message: string } | null>(null);

// Tauri window reference
const appWindow = getCurrentWindow();

// Window event listeners
let unlistenFocus: UnlistenFn | null = null;
let isDragging = false;

// Start window drag
function startDrag(e: MouseEvent) {
  if (e.buttons === 1 && e.detail < 2) {
    e.preventDefault();
    isDragging = true;
    appWindow.startDragging().finally(() => {
      // Delay resetting the flag to avoid false blur triggers
      setTimeout(() => {
        isDragging = false;
      }, 300);
    });
  }
}

// Computed
const filteredEntries = computed(() => {
  if (!searchKeyword.value.trim()) {
    return entries.value;
  }
  const keyword = searchKeyword.value.toLowerCase();
  return entries.value.filter(
    (e) =>
      e.title.toLowerCase().includes(keyword) ||
      e.username?.toLowerCase().includes(keyword) ||
      e.url?.toLowerCase().includes(keyword)
  );
});

// Methods
function getInitials(title: string): string {
  return title
    .split(/\s+/)
    .map((w) => w[0])
    .join("")
    .toUpperCase()
    .slice(0, 2);
}

async function handleUnlock() {
  if (!password.value) return;

  isUnlocking.value = true;
  unlockError.value = "";

  try {
    const isValid = await authStore.verifyPassword(password.value);
    if (isValid) {
      password.value = "";
      await loadEntries();
      nextTick(() => {
        searchInput.value?.focus();
      });
    } else {
      unlockError.value = "密码错误";
    }
  } catch {
    unlockError.value = "验证失败";
  } finally {
    isUnlocking.value = false;
  }
}

async function loadEntries() {
  isLoading.value = true;
  try {
    entries.value = await invoke<Entry[]>("get_entries", {
      groupId: null,
      search: null,
      favoritesOnly: false,
    });
  } catch (error) {
    console.error("Failed to load entries:", error);
  } finally {
    isLoading.value = false;
  }
}

function handleSearch() {
  // Search is handled by computed property, no debounce needed for quick access
}

function selectEntry(entry: Entry) {
  selectedEntry.value = entry;
  showPassword.value = false;
}

async function copyUsername(entry: Entry) {
  if (!entry.username) return;
  try {
    await writeText(entry.username);
    showToast("success", "用户名已复制");
  } catch {
    showToast("error", "复制失败");
  }
}

async function copyPassword(entry: Entry) {
  if (!entry.password) return;
  try {
    await writeText(entry.password);
    showToast("success", "密码已复制");
  } catch {
    showToast("error", "复制失败");
  }
}

async function openUrl(url: string) {
  try {
    await openExternal(url);
  } catch (error) {
    console.error("Failed to open URL:", error);
  }
}

function togglePin() {
  isPinned.value = !isPinned.value;
}

async function closeWindow() {
  try {
    await appWindow.hide();
  } catch (error) {
    console.error("Failed to close window:", error);
  }
}

function showToast(type: "success" | "error" | "info", message: string) {
  toast.value = { type, message };
  setTimeout(() => {
    toast.value = null;
  }, 2000);
}

// Lifecycle
onMounted(async () => {
  // Check auth state
  await authStore.checkInitialized();

  if (authStore.isUnlocked) {
    await loadEntries();
    nextTick(() => {
      searchInput.value?.focus();
    });
  } else {
    nextTick(() => {
      passwordInput.value?.focus();
    });
  }

  // Listen for Tauri window focus changes
  unlistenFocus = await appWindow.onFocusChanged(async ({ payload: focused }) => {
    if (focused) {
      // Reload entries when window gains focus
      if (authStore.isUnlocked) {
        await loadEntries();
      }
    } else {
      // Auto-hide when window loses focus (unless pinned or dragging)
      if (!isPinned.value && !isDragging) {
        await closeWindow();
      }
    }
  });
});

onUnmounted(() => {
  // Clean up Tauri window event listener
  if (unlistenFocus) {
    unlistenFocus();
  }
});

// Watch for auth state changes
watch(
  () => authStore.isUnlocked,
  async (unlocked) => {
    if (unlocked) {
      await loadEntries();
      nextTick(() => {
        searchInput.value?.focus();
      });
    }
  }
);
</script>
