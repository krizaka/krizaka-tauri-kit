# krizaka-tauri-kit — agent contract

> Governance of **`krizaka/krizaka-tauri-kit`**. Decision: [krizaka/.github ADR-005](https://github.com/krizaka/.github/blob/main/docs/adr/ADR-005-tauri-app-kit.md).

## What belongs here

- The security-sensitive desktop code every Krizaka Mac app needs the same way: folder scope, files inside a scope, the
  navigation guard. Pure Rust, no `tauri` dependency, so it is tested on any machine.
- Not here: a product's commands, its sidecar, its messages, its brand, the Tauri glue (windows, menu bar, deep links).
  Those are the product's, wired from `template-tauri-app`.

## Rules

- **A read goes through `ScopeStore::resolve`.** Never a path from the UI used directly.
- No product word, no network, no environment read in this crate: values arrive as parameters.
- Every refusal has a test: `..`, a symlink out of a grant, a never-grantable folder, a look-alike host.
- A change to what may be granted or to the guard's rules is breaking: apps in the wild depend on it.

## Done

`cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` green; a tag `vX.Y.Z` per release, products pin the tag.
