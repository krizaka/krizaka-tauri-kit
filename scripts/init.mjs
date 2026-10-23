#!/usr/bin/env node
// Starts a product from this template: node scripts/init.mjs <slug> "<Name>"
//   slug  lowercase, letters and digits (`yomizaka`): the scheme, the bundle id com.krizaka.<slug>, the domain <slug>.com
//   Name  how the product is written (`Yomizaka`)
// Replaces `__slug__` and `__Name__` in every text file of the repository, then removes nothing: git shows what changed.
import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const SLUG = /^[a-z][a-z0-9]{2,23}$/;

export function validate(slug, name) {
  if (!SLUG.test(slug ?? "")) throw new Error("slug: lowercase letters and digits, 3 to 24 characters, starting with a letter");
  if (!name || name.trim() !== name || name.length > 40) throw new Error("name: the product's name, at most 40 characters");
}

export function fill(text, slug, name) {
  return text.replaceAll("__slug__", slug).replaceAll("__Name__", name);
}

function textFiles() {
  return execFileSync("git", ["ls-files"], { encoding: "utf8" })
    .split("\n")
    .filter((f) => f && !/\.(png|jpe?g|ico|icns|woff2?|ttf|lock)$/.test(f) && !f.startsWith("LICENSE") && f !== "scripts/init.mjs" && f !== "scripts/init.test.mjs" && f !== "pnpm-lock.yaml");
}

if (import.meta.url === pathToFileURL(process.argv[1] ?? "").href) {
  const [slug, name] = process.argv.slice(2);
  try {
    validate(slug, name);
  } catch (error) {
    console.error(`init: ${error.message}`);
    process.exit(2);
  }
  for (const file of textFiles()) {
    const before = readFileSync(file, "utf8");
    const after = fill(before, slug, name);
    if (after !== before) writeFileSync(file, after);
  }
  console.log(`init: ${name} (${slug}) — com.krizaka.${slug}; now run the package manager's install, then the checks.`);
}
