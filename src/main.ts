import { createApp } from "vue";
import "@/styles.css";
import App from "./App.vue";
import { createPinia } from 'pinia'
import commonFunc from '@/util/commonFunc'
import router from '@/router'

createApp(App)
    .use(createPinia())
    .use(commonFunc)
    .use(router)
    .mount("#app");
