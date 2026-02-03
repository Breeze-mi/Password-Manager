<script setup lang="ts">
import { computed } from "vue";
import {
  Listbox,
  ListboxButton,
  ListboxOptions,
  ListboxOption,
} from "@headlessui/vue";
import { CheckIcon, ChevronUpDownIcon } from "@heroicons/vue/24/outline";

interface Option {
  value: string | number | null;
  label: string;
  icon?: string;
}

interface Props {
  modelValue: string | number | null;
  options: Option[];
  placeholder?: string;
  disabled?: boolean;
  label?: string;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: null,
  placeholder: "请选择",
  disabled: false,
  label: "",
});

const emit = defineEmits<{
  (e: "update:modelValue", value: string | number | null): void;
}>();

const selectedOption = computed(() => {
  return props.options.find((opt) => opt.value === props.modelValue) || null;
});

const handleChange = (value: string | number | null) => {
  emit("update:modelValue", value);
};
</script>

<template>
  <div class="w-full">
    <label v-if="props.label" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
      {{ props.label }}
    </label>
    <Listbox :model-value="props.modelValue" @update:model-value="handleChange" :disabled="props.disabled">
      <div class="relative">
        <ListboxButton
          class="w-full px-3 py-2 rounded-lg border border-gray-300 bg-white text-gray-900 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-colors duration-200 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100 flex items-center justify-between cursor-pointer"
          :class="{ 'opacity-50 cursor-not-allowed': props.disabled }"
        >
          <span v-if="selectedOption" class="flex items-center gap-2 truncate">
            <span v-if="selectedOption.icon">{{ selectedOption.icon }}</span>
            <span>{{ selectedOption.label }}</span>
          </span>
          <span v-else class="text-gray-400">{{ props.placeholder }}</span>
          <ChevronUpDownIcon class="w-5 h-5 text-gray-400 flex-shrink-0" />
        </ListboxButton>

        <transition
          leave-active-class="transition duration-100 ease-in"
          leave-from-class="opacity-100"
          leave-to-class="opacity-0"
        >
          <ListboxOptions
            class="absolute z-10 mt-1 w-full max-h-60 overflow-auto rounded-lg bg-white dark:bg-gray-800 shadow-lg border border-gray-200 dark:border-gray-700 focus:outline-none"
          >
            <ListboxOption
              v-for="option in props.options"
              :key="option.value ?? 'null'"
              :value="option.value"
              v-slot="{ active, selected }"
              class="cursor-pointer"
            >
              <div
                :class="[
                  'flex items-center justify-between px-3 py-2',
                  active ? 'bg-primary-50 dark:bg-primary-900/30' : '',
                ]"
              >
                <span class="flex items-center gap-2">
                  <span v-if="option.icon">{{ option.icon }}</span>
                  <span :class="{ 'font-medium': selected }">{{ option.label }}</span>
                </span>
                <CheckIcon v-if="selected" class="w-5 h-5 text-primary-600" />
              </div>
            </ListboxOption>
          </ListboxOptions>
        </transition>
      </div>
    </Listbox>
  </div>
</template>
