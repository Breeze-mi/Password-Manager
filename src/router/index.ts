import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "main",
      component: () => import("@/views/MainView.vue"),
      meta: { requiresAuth: true },
    },
    {
      path: "/setup",
      name: "setup",
      component: () => import("@/views/SetupView.vue"),
      meta: { requiresAuth: false },
    },
    {
      path: "/unlock",
      name: "unlock",
      component: () => import("@/views/UnlockView.vue"),
      meta: { requiresAuth: false },
    },
  ],
});

// Navigation guard will be added when auth store is ready
export default router;
