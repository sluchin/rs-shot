/// <reference types="vitest/config" />
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Tauri の開発サーバー (tauri.conf.json の devUrl) に合わせて, ポートを固定する.
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  test: {
    globals: true,
    environment: "jsdom",
    setupFiles: "./src/tests/setup.ts",
    include: ["src/**/*.test.{ts,tsx}"],
    coverage: {
      provider: "v8",
      include: ["src/**/*.{ts,tsx}"],
      // エントリポイントは, 画面に描くだけなので, 除く.
      exclude: ["src/main.tsx", "src/vite-env.d.ts", "src/tests/**"],
    },
  },
});
