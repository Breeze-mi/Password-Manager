<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { Button, Input } from "@/components/ui";
import { useAuthStore } from "@/stores";
import { useToast } from "@/composables/useToast";

const router = useRouter();
const authStore = useAuthStore();
const { showToast } = useToast();

const password = ref("");
const isLoading = ref(false);
const error = ref("");

async function handleUnlock() {
  if (!password.value) {
    error.value = "请输入主密码";
    return;
  }

  error.value = "";
  isLoading.value = true;

  try {
    const isValid = await authStore.verifyPassword(password.value);
    if (isValid) {
      showToast("解锁成功", "success");
      router.replace({ name: "main" });
    } else {
      error.value = "密码错误，请重试";
      password.value = "";
    }
  } catch (e) {
    error.value = "验证失败，请重试";
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <div
    class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 px-4"
  >
    <div class="w-full max-w-md">
      <!-- Logo and Title -->
      <div class="text-center mb-8">
        <div class="text-6xl mb-4">🔐</div>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-gray-100">
          One-Password
        </h1>
        <p class="mt-2 text-gray-600 dark:text-gray-400">请输入主密码解锁</p>
      </div>

      <!-- Unlock Form -->
      <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-8">
        <form @submit.prevent="handleUnlock" class="space-y-6">
          <Input
            v-model="password"
            type="password"
            label="主密码"
            placeholder="请输入主密码"
            :error="error"
            autofocus
          />

          <Button
            type="submit"
            variant="primary"
            size="lg"
            class="w-full"
            :loading="isLoading"
          >
            解锁
          </Button>
        </form>
      </div>
    </div>
  </div>
</template>
