/** @type {import('tailwindcss').Config} */
export default {
  content: ["./frontend/**/*.{js,ts,jsx,tsx,vue}"],
  theme: {
    extend: {
      colors: {
        editor: {
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
  plugins: [],
};
