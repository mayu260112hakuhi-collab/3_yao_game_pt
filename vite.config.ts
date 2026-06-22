import { defineConfig } from "vite";

export default defineConfig(async () => ({
  appType: "mpa",
  server: {
    port: 1420,
    strictPort: true,
  },
  build: {
    rollupOptions: {
      input: {
        index: "index.html",
        editor: "editor.html",
        settings: "settings.html",
        newProject: "new-project.html",
      },
    },
  },
}));