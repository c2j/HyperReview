import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css"; // 必须在这里导入，Vite 才会通过 PostCSS 处理它
import App from "./App";
import { LanguageProvider } from "./i18n";
import { LoadingProvider } from "./context/LoadingContext";

const rootElement = document.getElementById("root");
if (!rootElement) {
  throw new Error("Could not find root element to mount to");
}

const root = ReactDOM.createRoot(rootElement);
root.render(
  <React.StrictMode>
    <LoadingProvider>
      <LanguageProvider>
        <App />
      </LanguageProvider>
    </LoadingProvider>
  </React.StrictMode>,
);
