# ext-kit — `hnl gen-grammar` + `hnl kit` as a dynamic `hnl` extension

Kit tooling packaged as a `libhnl_ext_kit.{so,dylib,dll}` shared library for
the dumb `hnl` host (ADR-0149). The host discovers it via `--ext-dir`,
`$HANDL_EXT_DIR`, `~/.handl/ext/`, or a dev `target/debug/` next to `hnl`.

## Commands

```text
hnl gen-grammar [--kit <name>] [output_path]   — TextMate grammar from the canonical kit
hnl kit <add|remove|info> [args...]            — kit management (KitManagerPlugin)
```

`gen-grammar` renders the assembled canonical `Language` through
`handl-host-tooling`'s grammar generator; the old global `--kit` flag is
accepted (and ignored, as before — the canonical kit is always assembled).
`kit` runs the `kit` capability of `handl-plugin-ext-kit` inside a
`DefaultHost` — the same destination the old host's `cli` fallback routed
unknown commands to. As before, object responses print nothing; errors go
to stderr with exit 2.

## Dev

```sh
cargo build        # target/debug/libhnl_ext_kit.so
cargo test         # unit tests link the rlib statically
HANDL_EXT_DIR=$PWD/target/debug ../host-cli/target/debug/hnl gen-grammar
```

Sibling-repo deps resolve from sibling checkouts via the `[patch]` tables
(ADR-0128 §Resolution); without them the `branch = "main"` git sources are used.
