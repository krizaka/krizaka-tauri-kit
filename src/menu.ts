import type { MenuSpec } from "@krizaka/app-desktop";
import { acceleratorOf } from "@krizaka/ui/app-shell";

import type { Messages } from "./i18n";

/** The product's shortcuts: the same map the page's `useShortcuts` would answer, turned into the menu's accelerators. */
export const SHORTCUTS = { addFolder: "mod+o" } as const;

/** The native menu bar in the app's words (krizaka-ui ADR-002). A product adds its menus here. */
export function menuSpec(menu: Messages["menu"]): MenuSpec {
  const { file, addFolder, ...labels } = menu;
  return {
    labels,
    menus: [{ title: file, items: [{ id: "folders.add", label: addFolder, accelerator: acceleratorOf(SHORTCUTS.addFolder) }] }],
  };
}
