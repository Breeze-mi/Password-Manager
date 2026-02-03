<script setup lang="ts">
import type { Group } from "@/types";

defineProps<{
  group: Group;
  selected: boolean;
  count: number;
}>();

defineEmits<{
  click: [];
}>();
</script>

<template>
  <button
    @click="$emit('click')"
    class="w-full flex items-center justify-between px-3 py-2 rounded-lg transition-colors"
    :class="
      selected
        ? 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300'
        : 'hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300'
    "
  >
    <div class="flex items-center space-x-3">
      <span class="text-lg">{{ group.icon }}</span>
      <span class="truncate">{{ group.name }}</span>
    </div>
    <span class="text-sm text-gray-500 dark:text-gray-400">{{ count }}</span>
  </button>
</template>
