import { resolve } from 'path'

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
// 自动导入vue中hook reactive ref等
import AutoImport from 'unplugin-auto-import/vite'
//自动导入ui-组件 比如说ant-design-vue  element-plus等
//是此插件无法处理非组件模块，如 message，这种组件需要手动加载：
import Components from 'unplugin-vue-components/vite'
import AntdvResolver from 'antdv-component-resolver'
import { AntDesignVueResolver } from 'unplugin-vue-components/resolvers';
// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue(),
    AutoImport({
      //安装两行后你会发现在组件中不用再导入ref，reactive等
      imports: ['vue', 'vue-router'],
      dts: 'src/auto-import.d.ts',
      //ant-design-vue
      resolvers: [AntdvResolver(), AntDesignVueResolver()]
    }),
    Components({
      dirs: ['src/components'],
      //ant-design-vue   importStyle = false 样式就没了
      resolvers: [
        AntdvResolver(),
        AntDesignVueResolver({ importStyle: false, resolveIcons: true })
      ]
    })
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  resolve: {
    alias: {
      '@': resolve('src')
    }
  }
}));
