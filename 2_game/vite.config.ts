import { defineConfig } from 'vite'

export default defineConfig({
  appType: "mpa",
  server: {
    port: 5173,
    strictPort: true,
  },
  build: {
    outDir: '../dist',
    target: 'esnext'
  },
  // favicon の設定（public/favicon.svg を使用）
  publicDir: 'public',
})
