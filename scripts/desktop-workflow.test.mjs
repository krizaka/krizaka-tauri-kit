import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const workflow = readFileSync(new URL("../.github/workflows/desktop.yml", import.meta.url), "utf8");

test("the desktop workflow passes a signing secret only on a tag push, never on a pull request", () => {
  const lines = workflow.split("\n").filter((l) => /^\s+APPLE_\w+:/.test(l));
  assert.ok(lines.length >= 7, "the seven Apple secrets are passed");
  for (const l of lines) assert.match(l, /github\.event_name == 'push' && secrets\.APPLE_\w+ \|\| ''/, l);
});
