<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { Modal, Input, Button, Select } from "@/components/ui";
import { useEntriesStore, useGroupsStore } from "@/stores";
import { useToast } from "@/composables/useToast";
import type { Entry, CreateEntryDto, UpdateEntryDto } from "@/types";

const props = defineProps<{
  show: boolean;
  entry?: Entry;
}>();

const emit = defineEmits<{
  close: [];
  saved: [];
}>();

const entriesStore = useEntriesStore();
const groupsStore = useGroupsStore();
const { showToast } = useToast();

const isEditing = computed(() => !!props.entry);
const title = computed(() => (isEditing.value ? "编辑条目" : "新建条目"));

const form = ref({
  title: "",
  groupId: null as string | null,
  url: "",
  username: "",
  password: "",
  notes: "",
});

const isSaving = ref(false);
const errors = ref<Record<string, string>>({});

const groupOptions = computed(() => [
  { value: null, label: "未分组", icon: "📁" },
  ...groupsStore.sortedGroups.map((g) => ({
    value: g.id,
    label: g.name,
    icon: g.icon,
  })),
]);

// Reset form when modal opens
watch(
  () => props.show,
  (show) => {
    if (show) {
      if (props.entry) {
        form.value = {
          title: props.entry.title,
          groupId: props.entry.groupId,
          url: props.entry.url || "",
          username: props.entry.username || "",
          password: props.entry.password || "",
          notes: props.entry.notes || "",
        };
      } else {
        form.value = {
          title: "",
          groupId: groupsStore.selectedGroupId,
          url: "",
          username: "",
          password: "",
          notes: "",
        };
      }
      errors.value = {};
    }
  }
);

function validate(): boolean {
  errors.value = {};

  if (!form.value.title.trim()) {
    errors.value.title = "标题不能为空";
    return false;
  }

  return true;
}

async function handleSubmit() {
  if (!validate()) return;

  isSaving.value = true;
  try {
    if (isEditing.value && props.entry) {
      const dto: UpdateEntryDto = {
        title: form.value.title,
        groupId: form.value.groupId,
        url: form.value.url || undefined,
        username: form.value.username || undefined,
        password: form.value.password || undefined,
        notes: form.value.notes || undefined,
      };
      await entriesStore.updateEntry(props.entry.id, dto);
      showToast("条目已更新", "success");
    } else {
      const dto: CreateEntryDto = {
        title: form.value.title,
        groupId: form.value.groupId,
        url: form.value.url || undefined,
        username: form.value.username || undefined,
        password: form.value.password || undefined,
        notes: form.value.notes || undefined,
      };
      await entriesStore.createEntry(dto);
      showToast("条目已创建", "success");
    }
    emit("saved");
  } catch {
    showToast(isEditing.value ? "更新失败" : "创建失败", "error");
  } finally {
    isSaving.value = false;
  }
}
</script>

<template>
  <Modal :show="show" :title="title" size="lg" @close="$emit('close')">
    <form @submit.prevent="handleSubmit" class="space-y-4">
      <Input
        v-model="form.title"
        label="标题"
        placeholder="例如：GitHub 账号"
        :error="errors.title"
        required
      />

      <Select
        v-model="form.groupId"
        label="分组"
        :options="groupOptions"
      />

      <Input
        v-model="form.url"
        type="url"
        label="网址"
        placeholder="https://example.com"
      />

      <Input
        v-model="form.username"
        label="用户名"
        placeholder="用户名或邮箱"
      />

      <Input
        v-model="form.password"
        type="password"
        label="密码"
        placeholder="输入密码"
      />

      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          备注
        </label>
        <textarea
          v-model="form.notes"
          rows="3"
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent resize-none"
          placeholder="添加备注..."
        />
      </div>
    </form>

    <template #footer>
      <div class="flex justify-end space-x-3">
        <Button variant="secondary" @click="$emit('close')">取消</Button>
        <Button variant="primary" :loading="isSaving" @click="handleSubmit">
          {{ isEditing ? "保存" : "创建" }}
        </Button>
      </div>
    </template>
  </Modal>
</template>
