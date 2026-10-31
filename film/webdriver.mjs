// A small W3C client of the WebDriver embedded in a Tauri app (tauri-plugin-webdriver, 127.0.0.1:4445), for the journeys of the
// Mac apps. No dependency. The frames of the film come from its `screenshot()` (see film.mjs): no screen recording, no TCC prompt.
export const WEBDRIVER = "http://127.0.0.1:4445";
const ELEMENT = "element-6066-11e4-a52e-4f735466cecf";

/** The id inside a W3C element reference. */
export const elementId = (reference) => reference?.[ELEMENT] ?? reference?.ELEMENT;

/** A client of the WebDriver server inside the app: sessions, elements, scripts, screenshots. */
export function client(base = WEBDRIVER, fetchImpl = fetch) {
  const call = async (method, path, body) => {
    const res = await fetchImpl(`${base}${path}`, { method, headers: { "content-type": "application/json" }, body: body === undefined ? undefined : JSON.stringify(body) });
    const json = await res.json().catch(() => ({}));
    if (!res.ok) throw new Error(`webdriver ${method} ${path}: HTTP ${res.status} ${json?.value?.message ?? ""}`.trim());
    return json.value;
  };
  return {
    async start() {
      const session = await call("POST", "/session", { capabilities: { alwaysMatch: {} } });
      const sid = session.sessionId;
      const at = (path) => `/session/${sid}${path}`;
      return {
        sid,
        find: async (css) => elementId(await call("POST", at("/element"), { using: "css selector", value: css })),
        exists: async (css) => (await call("POST", at("/elements"), { using: "css selector", value: css })).length > 0,
        click: (id) => call("POST", at(`/element/${id}/click`), {}),
        type: (id, text) => call("POST", at(`/element/${id}/value`), { text }),
        run: (script, ...args) => call("POST", at("/execute/sync"), { script, args }),
        hash: () => call("POST", at("/execute/sync"), { script: "return window.location.hash", args: [] }),
        screenshot: () => call("GET", at("/screenshot")),
        rect: () => call("GET", at("/window/rect")),
        quit: () => call("DELETE", `/session/${sid}`),
      };
    },
  };
}

/** Waits until `check` answers truthy, polling; the error names what was awaited (a stable sentence for the log). */
export async function until(what, check, { timeoutMs, everyMs = 500, sleep = (ms) => new Promise((r) => setTimeout(r, ms)), now = Date.now } = {}) {
  const deadline = now() + timeoutMs;
  let last;
  for (;;) {
    try {
      const value = await check();
      if (value) return value;
    } catch (error) {
      last = error;
    }
    if (now() >= deadline) {
      const detail = last ? ` (${last.message})` : "";
      throw new Error(`timed out waiting for ${what}${detail}`);
    }
    await sleep(everyMs);
  }
}
