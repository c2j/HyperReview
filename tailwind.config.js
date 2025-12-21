/** @type {import('tailwindcss').Config} */
export default {
  content: ["./frontend/**/*.{js,ts,jsx,tsx,vue}"],
  theme: {
    extend: {
      colors: {
        editor: {
          // 基础颜色
          bg: "#1e1e1e",
          fg: "#cccccc",
          line: "#2d2d2d",
          selection: "#264f78",
          sidebar: "#252526",
          accent: "#007acc",
          error: "#f14c4c",
          warning: "#ff8800",
          info: "#409eff",
          success: "#5cc46b",
          modified: "#e2c08d",

          // --- 针对 Chrome 86 的透明度预定义 ---
          // 这里的命名对应你代码里的 /10, /30, /50 写法
          // 我们直接写死具体的 rgba 格式（逗号分隔），确保老浏览器认识
          "error-10": "rgba(241, 76, 76, 0.1)",
          "error-30": "rgba(241, 76, 76, 0.3)",
          "error-50": "rgba(241, 76, 76, 0.5)",

          "warning-10": "rgba(255, 136, 0, 0.1)",
          "warning-50": "rgba(255, 136, 0, 0.5)",

          "info-10": "rgba(64, 158, 255, 0.1)",
          "info-50": "rgba(64, 158, 255, 0.5)",

          "selection-30": "rgba(38, 79, 120, 0.3)",
          "selection-50": "rgba(38, 79, 120, 0.5)",

          "line-10": "rgba(45, 45, 45, 0.1)",
          "line-50": "rgba(45, 45, 45, 0.5)",
        },
      },
      fontFamily: {
        mono: [
          "JetBrains Mono",
          "ui-monospace",
          "SFMono-Regular",
          "Menlo",
          "Monaco",
          "Consolas",
          "Liberation Mono",
          "Courier New",
          "monospace",
        ],
      },
    },
  },
  // 关键：禁用某些会导致现代 CSS 语法的核心插件功能（可选，如果上述配置已生效则不选）
  corePlugins: {
    // 如果你发现即使写了 rgba 还是有问题，可以尝试开启兼容模式
  },
  plugins: [],
};
