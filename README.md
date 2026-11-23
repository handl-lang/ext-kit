# ext-kit

`hnl-ext-kit` — `hnl gen-grammar` + `hnl kit` as a dynamic `hnl` extension.

An `ext-*` crate: kit tooling packaged as a `libhnl_ext_kit.{so,dylib,dll}`
shared library for the dumb `hnl` host. The host discovers it via
`--ext-dir`, `$HANDL_EXT_DIR`, `~/.handl/ext/`, or a dev `target/debug/` next
to `hnl`.

## Commands

```text
hnl gen-grammar [--kit <name>] [output_path]   — TextMate grammar from the canonical kit
hnl kit <add|remove|info> [args...]            — kit management (KitManagerPlugin)
```

`gen-grammar` renders the assembled canonical `Language` through
`handl-host-tooling`'s grammar generator; the old global `--kit` flag is
accepted (and ignored — the canonical kit is always assembled). `kit` runs
the `kit` capability of `handl-plugin-ext-kit` inside a `DefaultHost`.
Object responses print nothing; errors go to stderr with exit 2.

## Development

```bash
cargo run --manifest-path xtask/Cargo.toml -- gate   # fmt → clippy → test → doc
```

`xtask/` provides the gate tasks (`fmt`, `clippy`, `test`, `doc`, `check`,
`gate`); `workflows/gate.yaml` is the per-push gate — the `handl-gates` bundle's rust campaign (fmt → clippy → test → doc), run by bupha/the fleet runner via `peregrin campaign run`. The crate
builds a `cdylib` + `rlib`; `cargo test` links the `rlib` statically.

```sh
cargo build        # target/debug/libhnl_ext_kit.so
HANDL_EXT_DIR=$PWD/target/debug ../host-cli/target/debug/hnl gen-grammar
```

Deps are git deps on `branch = "main"`; locally the workspace-root
`.cargo/config.toml` patches them to sibling checkouts (dev-only, never
committed).

## License

MIT.
