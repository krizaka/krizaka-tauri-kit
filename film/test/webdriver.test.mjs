import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { client, elementId, until } from "../webdriver.mjs";

const reply = (value, ok = true) => ({ ok, status: ok ? 200 : 500, json: async () => ({ value }) });

describe("the WebDriver client of a Tauri app", () => {
  it("reads an element reference in the W3C and the legacy form", () => {
    assert.equal(elementId({ "element-6066-11e4-a52e-4f735466cecf": "a" }), "a");
    assert.equal(elementId({ ELEMENT: "b" }), "b");
  });
  it("opens a session and takes a screenshot of the window", async () => {
    const calls = [];
    const fake = async (url, init) => {
      calls.push(`${init.method} ${url}`);
      return reply(init.method === "POST" ? { sessionId: "s1" } : "cG5n");
    };
    const driver = await client("http://wd", fake).start();
    assert.equal(await driver.screenshot(), "cG5n");
    assert.deepEqual(calls, ["POST http://wd/session", "GET http://wd/session/s1/screenshot"]);
  });
  it("names the HTTP failure", async () => {
    const driver = await client("http://wd", async (_u, init) => (init.method === "POST" && _u.endsWith("/session") ? reply({ sessionId: "s" }) : reply({ message: "boom" }, false))).start();
    await assert.rejects(driver.hash(), /HTTP 500 boom/);
  });
  it("waits until a check is true, and says what it waited for when it is not", async () => {
    let n = 0;
    assert.equal(await until("x", () => ++n === 3, { timeoutMs: 1000, sleep: async () => {} }), true);
    let t = 0;
    await assert.rejects(until("the library", () => false, { timeoutMs: 10, sleep: async () => {}, now: () => (t += 20) }), /timed out waiting for the library/);
  });
});
