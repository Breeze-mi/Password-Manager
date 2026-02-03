import { ref } from "vue";
import type { Toast, ToastType } from "@/types";

const toasts = ref<Toast[]>([]);

let toastId = 0;

export function useToast() {
  const addToast = (type: ToastType, message: string, duration = 3000) => {
    const id = `toast-${++toastId}`;
    const toast: Toast = { id, type, message, duration };

    toasts.value.push(toast);

    if (duration > 0) {
      setTimeout(() => {
        removeToast(id);
      }, duration);
    }

    return id;
  };

  const removeToast = (id: string) => {
    const index = toasts.value.findIndex((t) => t.id === id);
    if (index !== -1) {
      toasts.value.splice(index, 1);
    }
  };

  const showToast = (message: string, type: ToastType = "info", duration?: number) =>
    addToast(type, message, duration);

  const success = (message: string, duration?: number) => addToast("success", message, duration);
  const error = (message: string, duration?: number) => addToast("error", message, duration);
  const warning = (message: string, duration?: number) => addToast("warning", message, duration);
  const info = (message: string, duration?: number) => addToast("info", message, duration);

  return {
    toasts,
    addToast,
    removeToast,
    showToast,
    success,
    error,
    warning,
    info,
  };
}
