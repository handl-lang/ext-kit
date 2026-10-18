//! `hnl-ext-kit` — the `gen-grammar` + `kit` extension for `hnl` (ADR-0149).
//!
//! Two surfaces live here because both are *kit tooling*: `gen-grammar`
//! renders the loaded kit's `Language` as a TextMate grammar
//! (`handl-host-tooling`), and `kit` is the kit-management capability
//! (`handl-plugin-ext-kit`'s `KitManagerPlugin` — add/remove/info, the same
//! `cli`-capability target the built-in host's unknown-command fallback
//! routed to).
//!
//! This crate is the *packaging*: it exports the ADR-0149 ABI
//! (`handl_ext_manifest` / `handl_ext_dispatch` / `handl_ext_free`) and
//! implements the command surfaces the built-in `gen-grammar` arm and the
//! `cli` fallback's `kit` route had before the host went dumb.

use std::path::{Path, PathBuf};

use handl_ext::{ExtRequest, ExtResponse, ExtensionManifest};
use handl_kit_canonical::Kit;
use handl_platform::{CapabilityRequest, DefaultHost, PlatformError};

handl_ext::export!(
    ExtensionManifest::new("kit")
        .version(env!("CARGO_PKG_VERSION"))
        .command("gen-grammar", "gen-grammar [--kit <name>] [output_path]")
        .command("kit", "kit <add|remove|info> [args...]"),
    dispatch
);

/// ABI dispatch: `gen-grammar` generates a TextMate grammar; `kit` runs
/// the kit-management capability.
fn dispatch(cmd: &str, req: &ExtRequest) -> ExtResponse {
    match cmd {
        "gen-grammar" => gen_grammar(&req.args, Path::new(&req.cwd)),
        "kit" => kit_command(&req.args),
        other => ExtResponse::error(2, format!("kit: unknown command `{other}`\n")),
    }
}

// ---------------------------------------------------------------------------
// `gen-grammar [--kit <name>] [output_path]`
// ---------------------------------------------------------------------------

/// Output accumulator: the extension renders one `ExtResponse` instead of
/// printing, so stdout/stderr collect here and the code is returned.
#[derive(Default)]
struct Out {
    stdout: String,
    stderr: String,
}

impl Out {
    fn err(&mut self, msg: impl std::fmt::Display) {
        self.stderr.push_str(&format!("{msg}\n"));
    }
}

/// `hnl gen-grammar [output_path]` — the built-in arm took the kit name
/// from the host's global `--kit` flag and then ignored it (the canonical
/// kit is always assembled); the extension accepts `--kit <name>` as a
/// command-local flag and ignores it the same way. Extra positionals past
/// the output path are ignored, matching the old `command_args.first()`.
fn gen_grammar(args: &[String], cwd: &Path) -> ExtResponse {
    let mut out = Out::default();
    let mut output_path: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--kit" => {
                i += 1;
                if i >= args.len() {
                    out.err("gen-grammar: --kit requires a value");
                    return code(2, out);
                }
            }
            "--" => {}
            s if s.starts_with("--") => {
                out.err(format!("gen-grammar: unknown flag `{s}`"));
                return code(2, out);
            }
            s => {
                if output_path.is_none() {
                    output_path = Some(s.to_string());
                }
            }
        }
        i += 1;
    }

    let kit = match Kit::new() {
        Ok(k) => k,
        Err(e) => {
            out.err(format!("gen-grammar: {e}"));
            return code(1, out);
        }
    };
    let rules = handl_host_tooling::grammar_rules_from_language(kit.runtime().lang());
    let grammar =
        handl_host_tooling::generate_textmate(&rules, kit.scope_name(), &kit.file_extensions());

    let json = match serde_json::to_string_pretty(&grammar) {
        Ok(s) => s,
        Err(e) => {
            out.err(format!("gen-grammar: {e}"));
            return code(1, out);
        }
    };

    match output_path {
        Some(path) => {
            let disk = resolve(cwd, &path);
            if let Err(e) = std::fs::write(&disk, &json) {
                out.err(format!("gen-grammar: failed to write {path}: {e}"));
                return code(1, out);
            }
            out.err(format!("Generated TextMate grammar: {path}"));
        }
        None => {
            out.stdout.push_str(&json);
            out.stdout.push('\n');
        }
    }
    code(0, out)
}

// ---------------------------------------------------------------------------
// `kit <add|remove|info> [args...]`
// ---------------------------------------------------------------------------

/// Route `kit` args through the `kit` capability — the same destination the
/// old `cli` fallback dispatched to (payload `{args}`; `KitManagerPlugin`
/// reads `args[0]` as the action). The old `cli_dispatch` printed only a
/// string response; object results (`{"added": …}`) produced no output —
/// preserved here.
fn kit_command(args: &[String]) -> ExtResponse {
    let mut out = Out::default();

    let mut host = DefaultHost::new();
    host.register_plugin(handl_plugin_ext_kit::KitManagerPlugin::new());
    if let Err(e) = host.load() {
        out.err(format!("composition error: {e}"));
        return code(2, out);
    }
    if let Err(e) = host.resolve_composition() {
        out.err(format!("composition error: {e}"));
        return code(2, out);
    }

    let resp = host.run(&CapabilityRequest {
        capability: "kit".into(),
        payload: serde_json::json!({ "args": args }),
    });

    match resp {
        Ok(r) => {
            if let Some(s) = r.0.as_str() {
                out.out_str(s);
            }
            code(0, out)
        }
        Err(e) => {
            out.err(format!("kit: {e}"));
            code(2, out)
        }
    }
}

impl Out {
    fn out_str(&mut self, msg: impl std::fmt::Display) {
        self.stdout.push_str(&format!("{msg}\n"));
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Resolve `path` (possibly relative) against the request's `cwd`.
fn resolve(cwd: &Path, path: &str) -> PathBuf {
    let p = Path::new(path);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        cwd.join(p)
    }
}

fn code(c: i32, out: Out) -> ExtResponse {
    ExtResponse {
        code: c,
        stdout: out.stdout,
        stderr: out.stderr,
    }
}

/// Convert a [`PlatformError`]-shaped error into a response (unused — kept
/// for parity with sibling extension templates).
#[allow(dead_code)]
fn platform_err(e: PlatformError) -> ExtResponse {
    ExtResponse::error(2, format!("kit: {e}\n"))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn gen_grammar_unknown_flag_is_usage_error() {
        let res = gen_grammar(&["--bogus".into()], Path::new("."));
        assert_eq!(res.code, 2);
        assert!(res.stderr.contains("gen-grammar: unknown flag `--bogus`"));
    }

    #[test]
    fn gen_grammar_to_stdout_emits_textmate() {
        let res = gen_grammar(&[], Path::new("."));
        assert_eq!(res.code, 0, "stderr: {}", res.stderr);
        assert!(res.stdout.contains("\"scopeName\""));
    }

    #[test]
    fn kit_info_runs_the_capability() {
        let res = kit_command(&["info".into()]);
        assert_eq!(res.code, 0, "stderr: {}", res.stderr);
    }

    #[test]
    fn kit_missing_action_is_an_error() {
        let res = kit_command(&[]);
        assert_eq!(res.code, 2);
    }

    /// The `export!`-generated manifest symbol, called directly.
    #[test]
    fn manifest_declares_gen_grammar_and_kit() {
        let raw = handl_ext_manifest();
        assert!(!raw.is_null());
        let json = unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned();
        unsafe { handl_ext_free(raw) };
        let manifest: ExtensionManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(manifest.name, "kit");
        let names: Vec<&str> = manifest.commands.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["gen-grammar", "kit"]);
    }
}
