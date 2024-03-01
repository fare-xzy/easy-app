import BasicLayout from "@/layouts/BasicLayout.vue";
import Base64 from "@/views/Base64.vue";
import Decrypt from "@/views/Decrypt.vue";

export const constantRouterMap = [
  {
    path: '/',
    component: BasicLayout,
    redirect: '/home/base64',
    meta: { showMenu: false },
    children: [
      {
        path: '/home/base64',
        name: 'base64',
        component: Base64,
        meta: { title: 'Base64转换', showMenu: true }
      },
      {
        path: '/home/decrypt',
        name: 'decrypt',
        component: Decrypt,
        meta: { title: '加密包解密', showMenu: true }
      }
    ]
  }
]
