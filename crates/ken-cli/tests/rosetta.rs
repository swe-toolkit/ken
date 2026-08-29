//! VAL2 Rosetta pangram — the differential runner (Deliverable 1,
//! `docs/program/wp/VAL2-rosetta-pangram.md`).
//!
//! Globs `examples/rosetta/*/`, runs each `<slug>.ken` through the real
//! `ken` CLI binary (subprocess — soundness-inert, drives the compiler as
//! a black box and diffs text), and checks its declared oracle:
//!
//! - `expected` present  -> must-match: stdout must equal the file exactly,
//!   a mismatch fails this test (reds CI).
//! - `KNOWN-GAP.md` present -> recorded non-blocker: the runner does not
//!   assert anything about `ken run`'s outcome, just notes the dir in the
//!   findings summary printed at the end.
//! - Neither present -> hard error (never a silent skip).
//!
//! `ken run` still elaborates one flat source unit rather than loading a
//! catalog roots closure. Examples that reuse
//! `catalog/packages/Data/Collections/Derived.ken.md` (per the frame's DRY
//! rule) therefore need its provider closure concatenated ahead of their own
//! source. This legacy runner extracts the canonical closure, removes every
//! now-redundant import edge from the flattened provider sources with exact
//! cardinality checks, and orders Transport, Or, OrdResult, Compare, the
//! canonical Nat operations, then Derived.
//!
//! **This concatenation is NOT applied blanket to every example.**
//! Empirically, unconditionally prepending declarations that a given
//! example doesn't need can catastrophically amplify an unrelated
//! `ken-interp` performance characteristic (VAL2 findings routed to
//! Runtime/Architect, `wp/RTP1-interp-sharing`) — the runner only
//! concatenates for the specific dirs confirmed (via a real `ken run`,
//! not just in-process) to both need and tolerate it. See `NEEDS_COLLECTIONS`
//! below; every other dir runs its own `.ken` file as-is.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

const PER_EXAMPLE_TIMEOUT: Duration = Duration::from_secs(90);

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn ken_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_ken"))
}

fn rosetta_dir() -> PathBuf {
    workspace_root().join("examples/rosetta")
}

/// Dirs confirmed (in-process AND via a real `ken run`) to need
/// `catalog/packages/Data/Collections/Derived.ken.md` concatenated ahead of their own source, and to
/// stay within the per-example timeout with it prepended. Do not add a
/// slug here without measuring it first (see the module doc).
const NEEDS_COLLECTIONS: &[&str] = &["palindrome", "closures", "merge-sort", "tree-traversal"];

fn catalog_source(path: &str) -> String {
    let markdown = fs::read_to_string(workspace_root().join(path))
        .unwrap_or_else(|error| panic!("{path} must be readable: {error}"));
    ken_elaborator::literate::extract_ken_md(&markdown)
        .unwrap_or_else(|error| panic!("{path} must extract: {error:?}"))
        .source
}

fn remove_flattened_import(source: &mut String, owner: &str, import: &str) {
    let mut imports = source.match_indices(import);
    let (start, _) = imports
        .next()
        .unwrap_or_else(|| panic!("{owner} must carry import `{import}`"));
    assert!(
        imports.next().is_none(),
        "{owner} must carry exactly one import `{import}`"
    );
    source.replace_range(start..start + import.len(), "");
}

fn flattened_braced_declaration(source: &str, owner: &str, start: &str) -> std::ops::Range<usize> {
    let mut starts = source.match_indices(start);
    let (start_offset, _) = starts
        .next()
        .unwrap_or_else(|| panic!("{owner} must carry declaration `{start}`"));
    assert!(
        starts.next().is_none(),
        "{owner} must carry exactly one declaration `{start}`"
    );

    let body = &source[start_offset..];
    let mut depth = 0usize;
    let mut opened = false;
    for (offset, byte) in body.bytes().enumerate() {
        match byte {
            b'{' => {
                opened = true;
                depth += 1;
            }
            b'}' if opened => {
                depth -= 1;
                if depth == 0 {
                    return start_offset..start_offset + offset + 1;
                }
            }
            _ => {}
        }
    }
    panic!("{owner} declaration `{start}` must close");
}

fn remove_flattened_segment(source: &mut String, owner: &str, start: &str, next: &str) {
    let declaration = flattened_braced_declaration(source, owner, start);
    let mut nexts = source.match_indices(next);
    let (next_offset, _) = nexts
        .next()
        .unwrap_or_else(|| panic!("{owner} must carry following declaration `{next}`"));
    assert!(
        nexts.next().is_none(),
        "{owner} must carry exactly one following declaration `{next}`"
    );
    assert!(
        declaration.end <= next_offset && source[declaration.end..next_offset].trim().is_empty(),
        "{owner}: `{next}` must immediately follow `{start}`"
    );
    source.replace_range(declaration.start..next_offset, "");
}

fn remove_flattened_tail(source: &mut String, owner: &str, start: &str) {
    let declaration = flattened_braced_declaration(source, owner, start);
    assert!(
        source[declaration.end..].trim().is_empty(),
        "{owner} declaration `{start}` must be last"
    );
    source.replace_range(declaration, "");
}

fn collections_prelude() -> String {
    let transport = catalog_source("catalog/packages/Core/Logic/Transport.ken.md");
    let or_source = catalog_source("catalog/packages/Core/Logic/Or.ken.md");
    let ord_result = catalog_source("catalog/packages/Core/Logic/OrdResult.ken.md");
    let mut compare = catalog_source("catalog/packages/Core/Logic/Compare.ken.md");
    let lawful_classes = catalog_source("catalog/packages/Core/Classes/LawfulClasses.ken.md");
    let canonical_bool_ops = ["pub fn bool_leq", "pub fn bool_and"]
        .into_iter()
        .map(|start| {
            let declaration = flattened_braced_declaration(&lawful_classes, "LawfulClasses", start);
            &lawful_classes[declaration]
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mut nat_order = catalog_source("catalog/packages/Data/Numeric/Nat/Order.ken.md");
    let mut collections = catalog_source("catalog/packages/Data/Collections/Derived.ken.md");

    // `ken run` consumes one flat source unit here. Remove every import whose
    // provider source this compatibility runner has flattened immediately
    // above it, failing closed if a provider edge changes shape or cardinality.
    for import in [
        "import Core.Logic.Or (Or, Inl, Inr)",
        "import Core.Logic.OrdResult (OrdResult, Lt, Eq, Gt, ord_eq, ord_lt, ord_gt)",
        "import Core.Logic.Transport (sym)",
    ] {
        remove_flattened_import(&mut compare, "Compare", import);
    }
    for flattened_edge in [
        "import Core.Classes.LawfulClasses (Ord, IsTrue, bool_or, leq_nat)",
        "export Core.Classes.LawfulClasses (Ord, IsTrue, bool_or, leq_nat)",
        "data OrdResult = Lt | Eq | Gt",
    ] {
        remove_flattened_import(&mut nat_order, "Nat.Order", flattened_edge);
    }
    remove_flattened_segment(&mut nat_order, "Nat.Order", "pub fn max", "pub fn sub");
    remove_flattened_tail(&mut nat_order, "Nat.Order", "pub fn compare");

    for import in [
        "import Core.Classes.LawfulClasses (bool_and, bool_leq)",
        "import Core.Logic.Compare (list_compare, list_eq)",
        "import Core.Logic.Or (Or, Inl, Inr)",
        "import Core.Logic.OrdResult (OrdResult, Lt, Eq, Gt, ord_eq, ord_lt, ord_gt)",
        "import Core.Logic.OrdResult\n  (ord_result_leq, ord_result_dispatch2, ord_result_elim, ord_result_elim2)",
        "import Core.Logic.Transport (cong, sym, trans)",
        "import Data.Numeric.Nat.Order (min, sub)",
    ] {
        remove_flattened_import(&mut collections, "Derived", import);
    }

    format!(
        "{transport}\n{or_source}\n{ord_result}\n{compare}\n{canonical_bool_ops}\n{nat_order}\n{collections}"
    )
}

enum Oracle {
    Expected(String),
    KnownGap,
}

fn oracle_for(dir: &Path) -> Oracle {
    let expected_path = dir.join("expected");
    let gap_path = dir.join("KNOWN-GAP.md");
    match (expected_path.exists(), gap_path.exists()) {
        (true, false) => {
            Oracle::Expected(fs::read_to_string(&expected_path).expect("expected must be readable"))
        }
        (false, true) => Oracle::KnownGap,
        (true, true) => panic!(
            "{}: has BOTH `expected` and `KNOWN-GAP.md` — a dir must declare exactly one oracle",
            dir.display()
        ),
        (false, false) => panic!(
            "{}: has NEITHER `expected` nor `KNOWN-GAP.md` — every rosetta dir must declare its \
             oracle (never a silent skip)",
            dir.display()
        ),
    }
}

/// Runs `<slug>.ken` (with the package prelude prepended) through the real
/// `ken` binary as a subprocess, with a generous timeout. Returns
/// `Some((stdout, exit_success))` on completion within the timeout, `None`
/// on timeout (killed).
fn run_example(slug: &str, ken_src_path: &Path, tmp_dir: &Path) -> Option<(String, bool)> {
    let own_src = fs::read_to_string(ken_src_path)
        .unwrap_or_else(|e| panic!("{}: cannot read {}: {}", slug, ken_src_path.display(), e));
    let full_src = if NEEDS_COLLECTIONS.contains(&slug) {
        let body = own_src
            .strip_prefix("program capabilities FS APartial\n")
            .expect("runnable Rosetta source starts with its program header");
        format!(
            "program capabilities FS APartial\n{}\n{}",
            collections_prelude(),
            body
        )
    } else {
        own_src
    };

    let concat_path = tmp_dir.join(format!("{slug}.ken"));
    fs::write(&concat_path, full_src)
        .unwrap_or_else(|e| panic!("{}: cannot write concatenated source: {}", slug, e));

    let mut child = Command::new(ken_bin())
        .arg("run")
        .arg(&concat_path)
        // fs-read-file-lines-flip: `read-file-lines.ken` names its fixture
        // as a workspace-root-relative path (`conformance/fs/fixtures/...`).
        // Cargo runs a test binary with CWD = the crate's own manifest dir
        // (`crates/ken-cli`), not the workspace root, so the spawned `ken`
        // binary must be pinned to the workspace root explicitly — every
        // other example does no filesystem access and is unaffected.
        .current_dir(workspace_root())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("{}: failed to spawn `ken run`: {}", slug, e));

    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait().expect("try_wait failed") {
            let output = child.wait_with_output().expect("wait_with_output failed");
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            return Some((stdout, status.success()));
        }
        if start.elapsed() > PER_EXAMPLE_TIMEOUT {
            let _ = child.kill();
            let _ = child.wait();
            return None;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[test]
fn rosetta_examples_match_their_oracles() {
    let root = rosetta_dir();
    let tmp_dir = std::env::temp_dir().join("ken-rosetta-runner");
    fs::create_dir_all(&tmp_dir).expect("create tmp dir");

    let mut entries: Vec<PathBuf> = fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("cannot read {}: {}", root.display(), e))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    entries.sort();

    assert!(
        !entries.is_empty(),
        "expected at least one dir under {}",
        root.display()
    );

    let mut failures: Vec<String> = Vec::new();
    let mut known_gaps: Vec<String> = Vec::new();
    let mut passed: Vec<String> = Vec::new();

    for dir in &entries {
        let slug = dir.file_name().unwrap().to_string_lossy().into_owned();
        let ken_path = dir.join(format!("{slug}.ken"));
        if !ken_path.exists() {
            failures.push(format!("{slug}: missing {slug}.ken"));
            continue;
        }

        match oracle_for(dir) {
            Oracle::KnownGap => {
                known_gaps.push(slug);
            }
            Oracle::Expected(expected) => match run_example(&slug, &ken_path, &tmp_dir) {
                None => failures.push(format!("{slug}: timed out after {PER_EXAMPLE_TIMEOUT:?}")),
                Some((stdout, success)) => {
                    if !success {
                        failures.push(format!(
                            "{slug}: `ken run` exited non-zero, stdout: {stdout:?}"
                        ));
                    } else if stdout != expected {
                        failures.push(format!(
                            "{slug}: stdout mismatch — expected {expected:?}, got {stdout:?}"
                        ));
                    } else {
                        passed.push(slug);
                    }
                }
            },
        }
    }

    println!("\n=== Rosetta pangram findings summary ===");
    println!("PASS ({}): {:?}", passed.len(), passed);
    println!("KNOWN-GAP ({}): {:?}", known_gaps.len(), known_gaps);
    println!("FAIL ({}): {:?}", failures.len(), failures);

    assert!(
        failures.is_empty(),
        "rosetta examples failed:\n{}",
        failures.join("\n")
    );
}
