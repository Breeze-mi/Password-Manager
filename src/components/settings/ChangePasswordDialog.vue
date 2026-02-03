<script setup lang="ts">
import { ref, watch } from "vue";
import { Modal, Input, Button } from "@/components/ui";
import { useAuthStore } from "@/stores";
import { useToast } from "@/composables/useToast";

const props = defineProps<{
  show: boolean;
}>();

const emit = defineEmits<{
  close: [];
}>();

const authStore = useAuthStore();
const { showToast } = useToast();

const form = ref({
  currentPassword: "",
  newPassword: "",
  confirmPassword: "",
});

const errors = ref({
  currentPassword: "",
  newPassword: "",
  confirmPassword: "",
});

const isSaving = ref(false);

// Reset form when modal opens
watch(
  () => props.show,
  (show) => {
    if (show) {
      form.value = {
        currentPassword: "",
        newPassword: "",
        confirmPassword: "",
      };
      errors.value = {
        currentPassword: "",
        newPassword: "",
        confirmPassword: "",
      };
    }
  }
);

function validate(): boolean {
  errors.value = {
    currentPassword: "",
    newPassword: "",
    confirmPassword: "",
  };

  let valid = true;

  if (!form.value.currentPassword) {
    errors.value.currentPassword = "请输入当前密码";
    valid = false;
  }

  if (form.value.newPassword.length < 6) {
    errors.value.newPassword = "新密码长度至少为 6 位";
    valid = false;
  }

  if (form.value.newPassword !== form.value.confirmPassword) {
    errors.value.confirmPassword = "两次输入的密码不一致";
    valid = false;
  }

  return valid;
}

async function handleSubmit() {
  if (!validate()) return;

  isSaving.value = true;
  try {
    await authStore.changePassword(
      form.value.currentPassword,
      form.value.newPassword
    );
    showToast("密码修改成功", "success");
    emit("close");
  } catch (error: any) {
    if (error?.message?.includes("incorrect") || error?.message?.includes("Invalid")) {
      errors.value.currentPassword = "当前密码错误";
    } else {
      showToast("密码修改失败", "error");
    }
  } finally {
    isSaving.value = false;
  }
}
</script>

<template>
  <Modal :show="show" title="修改主密码" size="sm" @close="$emit('close')">
    <form @submit.prevent="handleSubmit" class="space-y-4">
      <Input
        v-model="form.currentPassword"
        type="password"
        label="当前密码"
        placeholder="请输入当前密码"
        :error="errors.currentPassword"
      />

      <Input
        v-model="form.newPassword"
        type="password"
        label="新密码"
        placeholder="请输入新密码（至少6位）"
        :error="errors.newPassword"
      />

      <Input
        v-model="form.confirmPassword"
        type="password"
        label="确认新密码"
        placeholder="请再次输入新密码"
        :error="errors.confirmPassword"
      />
    </form>

    <template #footer>
      <div class="flex justify-end space-x-3">
        <Button variant="secondary" @click="$emit('close')">取消</Button>
        <Button variant="primary" :loading="isSaving" @click="handleSubmit">
          确认修改
        </Button>
      </div>
    </template>
  </Modal>
</template>
