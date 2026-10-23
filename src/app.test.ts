import en from "./i18n/en.json";
import fr from "./i18n/fr.json";
import { menuSpec } from "./menu";

const leaves = (value: unknown, path = ""): string[] =>
  value && typeof value === "object" ? Object.entries(value).flatMap(([key, child]) => leaves(child, path ? `${path}.${key}` : key)) : [path];

describe("the app's words", () => {
  it("exist in French and in English, with the same keys", () => {
    expect(leaves(fr).sort()).toEqual(leaves(en).sort());
  });
});

describe("the native menu", () => {
  it("carries the standard labels and the product's items with their accelerators", () => {
    const spec = menuSpec(fr.menu);
    expect(spec.labels.quit).toBe(fr.menu.quit);
    expect(Object.keys(spec.labels)).not.toContain("file");
    expect(spec.menus).toEqual([{ title: fr.menu.file, items: [{ id: "folders.add", label: fr.menu.addFolder, accelerator: "CmdOrCtrl+O" }] }]);
  });
});
