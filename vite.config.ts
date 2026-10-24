/// <reference types="vitest/config" />
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Tauri の開発サーバー (tauri.conf.json の devUrl) に合わせて, ポートを固定する.
export default defineConfig({
  plugins: [react()],
  // src を `@/` で参照する (テストから `@/lib/...` のように使う)
  resolve: {
    alias: { "@": "/src" },
  },
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
    setupFiles: "./tests/setup.ts",
    include: ["tests/**/*.test.{ts,tsx}"],
    coverage: {
      provider: "v8",
      include: ["src/**/*.{ts,tsx}"],
      // エントリポイントは, 画面に描くだけなので, 除く.
      exclude: ["src/main.tsx", "src/vite-env.d.ts"],
      // 命令・分岐・関数・行のすべてで 100% を保つ (下回ると失敗する)
      thresholds: {
        100: true,
      },
    },
  },
});
