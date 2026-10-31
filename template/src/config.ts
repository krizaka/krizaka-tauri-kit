import { browserStorage, createApiClient, createAppDiscovery, createSessionStore } from "@krizaka/app-client";
import { fetch as tauriFetch } from "@tauri-apps/plugin-http";

import { keychain } from "./desktop";
import { currentLocale } from "./i18n";

/**
 * The Mac app's wiring, once, as on the phone (ADR-004): the backend comes from the signed manifest on krizaka.com
 * (ADR-003), the session lives in the Keychain, the API client is @krizaka/app-client's.
 */
export const PRODUCT = "__slug__";
export const APP_VERSION = import.meta.env.VITE_APP_VERSION ?? "0.1.0";
// @fixed: a call that waits longer is a Mac that has lost its network
const API_TIMEOUT_MS = 30_000;
/** Until a verified manifest answers: the address of the dev deployment. */
const BAKED = { apiBase: "https://dev.__slug__.com", webBase: "https://dev.__slug__.com" } as const;

export const session = createSessionStore({ storage: keychain, key: "__slug__.session" });

export const discovery = createAppDiscovery({
  product: PRODUCT,
  hostSuffix: "__slug__.com",
  appVersion: APP_VERSION,
  baked: BAKED,
  // The signed manifest is public: the page's storage keeps it; the Keychain keeps the session only.
  storage: browserStorage(() => window.localStorage),
  env: { url: import.meta.env.VITE_APP_URL, environment: import.meta.env.VITE_APP_ENV },
});

export const api = createApiClient({
  baseUrl: () => discovery.getService(PRODUCT).apiBase,
  session,
  fetch: (input, init) => tauriFetch(input, init),
  language: currentLocale,
  productName: "__Name__",
  timeoutMs: API_TIMEOUT_MS,
});
