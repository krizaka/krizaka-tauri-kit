import "./styles.css";

import { ThemeProvider } from "@krizaka/ui/theme";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { App } from "./App";
import { currentLocale, I18nProvider } from "./i18n";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ThemeProvider defaultMode="system">
      <I18nProvider locale={currentLocale()}>
        <App />
      </I18nProvider>
    </ThemeProvider>
  </StrictMode>,
);
