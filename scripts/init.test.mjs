import assert from "node:assert/strict";
import { test } from "node:test";

import { fill, validate } from "./init.mjs";

test("a slug is lowercase letters and digits, a name is the product's own", () => {
  validate("yomizaka", "Yomizaka");
  validate("akinaza", "Akinaza");
  for (const slug of ["", "Yomi", "yo", "yomi-zaka", "1yomi", "x".repeat(25), undefined]) assert.throws(() => validate(slug, "Name"), /slug/);
  assert.throws(() => validate("yomizaka", " Yomizaka"), /name/);
  assert.throws(() => validate("yomizaka", ""), /name/);
});

test("fills every placeholder, as many times as it appears", () => {
  assert.equal(fill("com.krizaka.__slug__ / __Name__ / __Name__", "akinaza", "Akinaza"), "com.krizaka.akinaza / Akinaza / Akinaza");
});
