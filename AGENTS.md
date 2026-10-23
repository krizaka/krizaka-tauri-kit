# __Name__ desktop — agent contract

> Started from `krizaka/template-tauri-app`. Decision: [ADR-005](https://github.com/krizaka/.github/blob/main/docs/adr/ADR-005-tauri-app-kit.md).

- A read goes through `Access::resolve` of `krizaka-tauri-kit`; never a path from the page used directly. No `fs`, shell or opener capability.
- The page is built from `@krizaka/ui` blocks, never hand-built; words live in messages (French first); the URL (the window's hash) is the state.
- A fix to the scope, the drop gate or the guard goes to the kit, with its test, then is used here.
- Test first: `pnpm check` (types, UI tests, Rust tests) before a push.
