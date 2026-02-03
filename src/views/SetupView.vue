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
const confirmPassword = ref("");
const isLoading = ref(false);
const errors = ref({
  password: "",
  confirmPassword: "",
});

function validateForm(): boolean {
  errors.value = { password: "", confirmPassword: "" };

  if (password.value.length < 6) {
    errors.value.password = "密码长度至少为 6 位";
    return false;
  }

  if (password.value !== confirmPassword.value) {
    errors.value.confirmPassword = "两次输入的密码不一致";
    return false;
  }

  return true;
}

async function handleSubmit() {
  if (!validateForm()) return;

  isLoading.value = true;
  try {
    await authStore.setupPassword(password.value);
    showToast("主密码设置成功！", "success");
    router.replace({ name: "main" });
  } catch (error) {
    showToast("设置密码失败，请重试", "error");
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
        <p class="mt-2 text-gray-600 dark:text-gray-400">设置您的主密码</p>
      </div>

      <!-- Setup Form -->
      <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-8">
        <form @submit.prevent="handleSubmit" class="space-y-6">
          <Input
            v-model="password"
            type="password"
            label="主密码"
            placeholder="请输入主密码（至少6位）"
            :error="errors.password"
          />

          <Input
            v-model="confirmPassword"
            type="password"
            label="确认密码"
            placeholder="请再次输入主密码"
            :error="errors.confirmPassword"
          />

          <div class="pt-2">
            <Button
              type="submit"
              variant="primary"
              size="lg"
              class="w-full"
              :loading="isLoading"
            >
              创建密码库
            </Button>
          </div>
        </form>

        <div class="mt-6 p-4 bg-amber-50 dark:bg-amber-900/20 rounded-lg">
          <p class="text-sm text-amber-800 dark:text-amber-200">
            <strong>⚠️ 重要提示：</strong>
            请牢记您的主密码。主密码无法找回，忘记密码将导致无法访问已保存的数据。
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
