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
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
```

`cargo xtask` wraps the repo's `xtask/` crate (`fmt`, `clippy`, `test`, `doc`,
`check`, `gate`); `ci.sh` is the gate entry. The crate builds a `cdylib` +
`rlib`; `cargo test` links the `rlib` statically.

```sh
cargo build        # target/debug/libhnl_ext_kit.so
HANDL_EXT_DIR=$PWD/target/debug ../host-cli/target/debug/hnl gen-grammar
```

Sibling-repo deps resolve from sibling checkouts via the `[patch]` tables;
without them the `branch = "main"` git sources are used.

## License

MIT.
