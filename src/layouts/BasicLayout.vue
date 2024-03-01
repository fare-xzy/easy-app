<template>
  <a-layout class="layout" style="height: 100%;">
    <a-layout-header>
      <a-menu theme="dark" mode="horizontal">
        <a-menu-item v-for='(r, index) in visibleMenuItems' :key="index" @click="jump(r.path)">{{ r.meta.title
        }}</a-menu-item>
      </a-menu>
    </a-layout-header>
    <a-layout-content>
      <RouterView />
    </a-layout-content>
    <a-layout-footer style="text-align: center">
      Easy-app ©2024 Created by Fare丶梦
    </a-layout-footer>
  </a-layout>
</template>
<script lang="ts" setup>
import { useRouter } from 'vue-router'
const router = useRouter()
console.log(router.getRoutes())

const visibleMenuItems = computed(() => {
  return router.getRoutes().filter(item => item.meta.showMenu);
});
// 跳转
const jump = (path: string): void => {
  router.push(path)
}
</script>
<style scoped></style>
