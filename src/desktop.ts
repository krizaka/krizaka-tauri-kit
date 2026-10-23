import { createFolders, createUpdates, followLinks, keychainStorage, setNativeMenu, type TauriBridge } from "@krizaka/app-desktop";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

/** The native side (tauri-plugin-krizaka), typed by @krizaka/app-desktop. The page has no fs, no shell, no opener. */
export const bridge: TauriBridge = { invoke, listen };
export const folders = createFolders(bridge);
export const updates = createUpdates(bridge);
export const keychain = keychainStorage(bridge);
export { followLinks, setNativeMenu };
