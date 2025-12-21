export default {
  plugins: {
    "postcss-import": {},
    "tailwindcss/nesting": {},
    tailwindcss: {},
    "postcss-preset-env": {
      stage: 3,
      features: {
        "custom-properties": true, // 确保 CSS 变量被正确处理
      },
    },
    "postcss-color-functional-notation": {}, // 将空格语法转为逗号语法
    autoprefixer: {},
  },
};
