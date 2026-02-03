<script setup lang="ts">
import { computed } from "vue";
import { StarIcon, TrashIcon } from "@heroicons/vue/24/outline";
import { StarIcon as StarSolidIcon } from "@heroicons/vue/24/solid";
import type { Entry } from "@/types";

const props = defineProps<{
  entry: Entry;
  selected: boolean;
}>();

defineEmits<{
  click: [];
  toggleFavorite: [];
  delete: [];
}>();

const initials = computed(() => {
  if (!props.entry.title) return "?";
  return props.entry.title.slice(0, 2).toUpperCase();
});

const displayUrl = computed(() => {
  if (!props.entry.url) return "";
  try {
    const url = new URL(props.entry.url);
    return url.hostname;
  } catch {
    return props.entry.url;
  }
});
</script>

<template>
  <div
    @click="$emit('click')"
    class="p-4 cursor-pointer border-b border-gray-100 dark:border-gray-700 transition-all duration-200 group relative hover:translate-x-1"
    :class="
      selected
        ? 'bg-primary-50 dark:bg-primary-900/30'
        : 'hover:bg-gray-50 dark:hover:bg-gray-700/50'
    "
  >
    <div class="flex items-center space-x-3">
      <!-- Avatar with hover scale -->
      <div
        class="w-10 h-10 rounded-lg bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 flex items-center justify-center font-semibold text-sm flex-shrink-0 transition-transform duration-200 group-hover:scale-105"
      >
        {{ initials }}
      </div>

      <!-- Content -->
      <div class="flex-1 min-w-0">
        <div class="flex items-center space-x-2">
          <h3
            class="font-medium text-gray-900 dark:text-gray-100 truncate"
          >
            {{ entry.title }}
          </h3>
          <button
            @click.stop="$emit('toggleFavorite')"
            class="flex-shrink-0 p-0.5 rounded hover:bg-gray-100 dark:hover:bg-gray-600 transition-all duration-200 star-button"
          >
            <StarSolidIcon
              v-if="entry.isFavorite"
              class="w-4 h-4 text-yellow-500"
            />
            <StarIcon v-else class="w-4 h-4 text-gray-400" />
          </button>
        </div>
        <p class="text-sm text-gray-500 dark:text-gray-400 truncate">
          {{ entry.username || displayUrl || "无用户名" }}
        </p>
      </div>

      <!-- Delete Button (visible on hover) -->
      <button
        @click.stop="$emit('delete')"
        class="flex-shrink-0 p-1.5 rounded-lg text-gray-400 hover:text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 transition-all duration-200 opacity-0 group-hover:opacity-100"
        title="删除"
      >
        <TrashIcon class="w-4 h-4" />
      </button>
    </div>
  </div>
</template>

<style scoped>
/* Star button click animation */
@keyframes bounce-star {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.3);
  }
}

.star-button:active {
  animation: bounce-star 0.3s ease;
}
</style>
