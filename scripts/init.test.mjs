import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";

import { fill, validate } from "./init.mjs";

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

test("the stylesheet imports a brand the tokens package exports, never a placeholder (CI was red on main)", () => {
  const css = readFileSync(new URL("../src/styles.css", import.meta.url), "utf8");
  const brands = [...css.matchAll(/@krizaka\/tokens\/brands\/([\w-]+)\.css/g)].map((m) => m[1]);
  assert.ok(brands.length > 0);
  for (const brand of brands) assert.ok(["krizaka", "scoped", "akinaza", "orazaka", "yomizaka"].includes(brand), brand);
});
