import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { resolve } from "path";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],
  css: {
    //postcss: "./postcss.config.js", // 强制指定配置文件
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },

  // Build optimization for production
  build: {
    // Enable source maps for better debugging
    sourcemap: false,

    // Optimize chunk size to improve load times
    chunkSizeWarningLimit: 1000,

    // Rollup options for better code splitting
    rollupOptions: {
      output: {
        // Manual chunks for better caching
        manualChunks: {
          // Vendor libraries
          "vendor-react": ["react", "react-dom"],
          "vendor-ui": ["lucide-react"],
          "vendor-utils": ["zustand"],
        },

        // Improve chunk naming
        chunkFileNames: "assets/[name]-[hash].js",
        entryFileNames: "assets/[name]-[hash].js",
        assetFileNames: "assets/[name]-[hash].[ext]",
      },
    },

    // Minification with esbuild (faster than terser)
    minify: "esbuild",
    // Note: To use terser for more aggressive minification, install it: npm install -D terser
    // minify: 'terser',
    // terserOptions: {
    //   compress: {
    //     drop_console: true,
    //     drop_debugger: true,
    //     passes: 2,
    //   },
    // },
  },

  // Resolve configuration
  resolve: {
    alias: {
      "@": resolve(__dirname, "./frontend"),
      "@components": resolve(__dirname, "./frontend/components"),
      "@api": resolve(__dirname, "./frontend/api"),
      "@hooks": resolve(__dirname, "./frontend/hooks"),
      "@utils": resolve(__dirname, "./frontend/utils"),
      "@types": resolve(__dirname, "./frontend/types"),
      "@store": resolve(__dirname, "./frontend/store"),
    },
  },

  // Optimize dependencies
  optimizeDeps: {
    include: ["react", "react-dom", "lucide-react", "zustand"],
  },
}));
