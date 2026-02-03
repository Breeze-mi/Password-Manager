<script setup lang="ts">
import { computed, ref } from "vue";
import { EyeIcon, EyeSlashIcon } from "@heroicons/vue/24/outline";

interface Props {
  modelValue?: string;
  type?: "text" | "password" | "email" | "url";
  placeholder?: string;
  disabled?: boolean;
  error?: string;
  label?: string;
  showPasswordToggle?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: "",
  type: "text",
  placeholder: "",
  disabled: false,
  error: "",
  label: "",
  showPasswordToggle: false,
});

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
}>();

const showPassword = ref(false);

const inputType = computed(() => {
  if (props.type === "password" && showPassword.value) {
    return "text";
  }
  return props.type;
});

const inputClasses = computed(() => {
  const base = "w-full px-3 py-2 rounded-lg border bg-white text-gray-900 focus:outline-none focus:ring-2 focus:border-transparent transition-colors duration-200 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100";
  const errorClass = props.error ? "border-red-500 focus:ring-red-500" : "border-gray-300 focus:ring-primary-500";
  const paddingRight = props.showPasswordToggle && props.type === "password" ? "pr-10" : "";

  return `${base} ${errorClass} ${paddingRight}`;
});

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement;
  emit("update:modelValue", target.value);
};

const togglePassword = () => {
  showPassword.value = !showPassword.value;
};
</script>

<template>
  <div class="w-full">
    <label v-if="props.label" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
      {{ props.label }}
    </label>
    <div class="relative">
      <input
        :type="inputType"
        :value="props.modelValue"
        :placeholder="props.placeholder"
        :disabled="props.disabled"
        @input="handleInput"
        :class="inputClasses"
      />
      <button
        v-if="props.showPasswordToggle && props.type === 'password'"
        type="button"
        @click="togglePassword"
        class="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
      >
        <EyeSlashIcon v-if="showPassword" class="w-5 h-5" />
        <EyeIcon v-else class="w-5 h-5" />
      </button>
    </div>
    <p v-if="props.error" class="mt-1 text-sm text-red-500">
      {{ props.error }}
    </p>
  </div>
</template>
