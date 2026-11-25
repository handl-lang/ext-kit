# ext-kit

`hnl-ext-kit` — `hnl gen-grammar` + `hnl kit` as a dynamic `hnl`
extension: kit tooling packaged as a `libhnl_ext_kit.{so,dylib,dll}`
cdylib the dumb `hnl` host discovers at runtime.

## Layout

| path | role |
| --- | --- |
| `src/` | extension ABI exports + `gen-grammar`/`kit` command wiring |
| `xtask/` | vendored gate crate (`fmt`, `clippy`, `test`, `doc`, `gate`) |
| `workflows/gate.yaml` | per-push gate — `handl-gates` rust campaign via bupha |

## Contract

- Commands: `hnl gen-grammar [--kit <name>] [output_path]` (TextMate
  grammar from the canonical kit, rendered through
  `handl-host-tooling`'s grammar generator — the old global `--kit` flag
  is accepted and ignored) and `hnl kit <add|remove|info> [args...]`
  (`KitManagerPlugin` capability inside a `DefaultHost`).
- Object responses print nothing; errors go to stderr with exit 2.
- Builds `cdylib` + `rlib`; `cargo test` links the `rlib` statically.

## Verification

```bash
cargo run --manifest-path xtask/Cargo.toml -- gate   # fmt → clippy → test → doc
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
