import { createApp } from "vue";
import { createPinia } from "pinia";
import QuickAccess from "./QuickAccess.vue";
import "../assets/styles/main.css";

const app = createApp(QuickAccess);
const pinia = createPinia();

app.use(pinia);
app.mount("#app");
