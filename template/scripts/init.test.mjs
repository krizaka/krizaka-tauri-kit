import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";

import { BUNDLE_PLACEHOLDER, fill, validate } from "./init.mjs";

test("a slug is lowercase letters and digits, a name is the product's own", () => {
  validate("yomizaka", "Yomizaka");
  validate("akinaza", "Akinaza");
  for (const slug of ["", "Yomi", "yo", "yomi-zaka", "1yomi", "x".repeat(25), undefined])
    assert.throws(() => validate(slug, "Name"), /slug/);
  assert.throws(() => validate("yomizaka", " Yomizaka"), /name/);
  assert.throws(() => validate("yomizaka", ""), /name/);
});

test("fills every placeholder, as many times as it appears", () => {
  assert.equal(
    fill("com.krizaka.__slug__ / __Name__ / __Name__", "akinaza", "Akinaza"),
    "com.krizaka.akinaza / Akinaza / Akinaza",
  );
});

test("the committed bundle identifier is valid for Tauri and the scaffolder replaces it", () => {
  const conf = readFileSync(new URL("../src-tauri/tauri.conf.json", import.meta.url), "utf8");
  const { identifier } = JSON.parse(conf);
  assert.equal(identifier, BUNDLE_PLACEHOLDER);
  assert.match(identifier, /^[A-Za-z0-9-]+(\.[A-Za-z0-9-]+)+$/, "Tauri allows only alphanumerics, hyphens and dots");
  const filled = JSON.parse(fill(conf, "akinaza", "Akinaza"));
  assert.equal(filled.identifier, "com.krizaka.akinaza.desktop");
});

test("the stylesheet imports a brand the tokens package exports, never a placeholder (CI was red on main)", () => {
  const css = readFileSync(new URL("../src/styles.css", import.meta.url), "utf8");
  const brands = [...css.matchAll(/@krizaka\/tokens\/brands\/([\w-]+)\.css/g)].map((m) => m[1]);
  assert.ok(brands.length > 0);
  for (const brand of brands) assert.ok(["krizaka", "scoped", "akinaza", "orazaka", "yomizaka"].includes(brand), brand);
});
