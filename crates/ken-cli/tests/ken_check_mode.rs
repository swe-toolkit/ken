//! FR-3 (`docs/program/wp/ds-1-findings-remediation.md`) — `ken check`, a
//! library check-mode: elaborate + verify fences, never drive IO. Exercises
//! the real `ken` CLI binary as a subprocess (soundness-inert black-box).

use std::path::PathBuf;
use std::process::Command;

fn ken_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_ken"))
}

fn write_fixture(name: &str, contents: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let path = dir.join(name);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create fixture parent");
    }
    std::fs::write(&path, contents).expect("write fixture");
    path
}

fn write_catalog_fixture(label: &str, entry: &str, dependency: &str) -> PathBuf {
    let relative_root = format!("{label}/catalog/packages");
    write_fixture(&format!("{relative_root}/Dependency.ken.md"), dependency);
    write_fixture(&format!("{relative_root}/Entry.ken.md"), entry)
}

const PURE_LIBRARY_KEN_MD: &str = r#"A pure-library entry, no IO `main`.

```ken
fn notBool (b : Bool) : Bool = match b { True |-> False ; False |-> True }

fn stagedBool (b : Bool) : Bool = let first = notBool b; second = notBool first in second
```

```ken example
const notTrue : Bool = notBool True
```
"#;

const FAILING_FENCE_KEN_MD: &str = r#"An entry whose `ken reject` fence is stale.

```ken
fn notBool (b : Bool) : Bool = match b { True |-> False ; False |-> True }
```

```ken reject
-- This is claimed to fail but actually elaborates fine — a stale negative.
fn notBool2 (b : Bool) : Bool = match b { True |-> False ; False |-> True }
```
"#;

const IO_KEN_MD: &str = r#"A runnable program.

```ken
program capabilities FS APartial
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial) : HostIO APartial ExitCode visits [Console] = host_program APartial (print_line "check mode ran main? no.")
```
"#;

const CATALOG_DEPENDENCY: &str = r#"# Dependency

```ken
pub const value : Bool = True
```
"#;

const CATALOG_ENTRY: &str = r#"# Entry

```ken
import Dependency
const answer : Bool = Dependency.value
```
"#;

// ken check on a genuine pure-library file: exits 0, never attempts IO (no
// stdout produced — `print_line` is never reached because IO is never
// driven; this file doesn't even have an IO `main`).
#[test]
fn check_pure_library_file_exits_zero_no_io() {
    let path = write_fixture("check_pure_library.ken.md", PURE_LIBRARY_KEN_MD);

    let output = Command::new(ken_bin())
        .arg("check")
        .arg(&path)
        .output()
        .expect("spawn ken check");

    assert!(
        output.status.success(),
        "ken check on a pure-library file must exit 0; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stdout.is_empty(),
        "ken check must never drive IO (no stdout expected): {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

/// Durable invariant: a real literate catalog entry retains its check verdict
/// and checked-fence scope when addressed through the roots loader.
#[test]
fn check_existing_catalog_entry_through_roots_loader() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages/Core/Logic/Transport.ken.md");
    let output = Command::new(ken_bin())
        .arg("check")
        .arg(&path)
        .output()
        .expect("spawn existing catalog ken check");
    assert!(
        output.status.success(),
        "an existing literate catalog entry and its checked fences must retain their verdict; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Durable invariant: a catalog-addressed CLI entry follows import edges; an
/// isolated-file implementation cannot satisfy this fixture.
#[test]
fn check_catalog_entry_resolves_import_through_roots_loader() {
    let path = write_catalog_fixture("loader_entry_import", CATALOG_ENTRY, CATALOG_DEPENDENCY);

    let output = Command::new(ken_bin())
        .arg("check")
        .arg(&path)
        .output()
        .expect("spawn catalog ken check");

    assert!(
        output.status.success(),
        "an imported catalog dependency must resolve through the roots loader; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Durable invariant: roots loading does not erase the entry document's
/// negative-example obligation.
#[test]
fn stale_reject_in_catalog_entry_is_still_refused_after_roots_loading() {
    let entry = format!("{CATALOG_ENTRY}\n```ken reject\nconst stale : Bool = True\n```\n");
    let path = write_catalog_fixture("loader_entry_stale_reject", &entry, CATALOG_DEPENDENCY);

    let output = Command::new(ken_bin())
        .arg("check")
        .arg(&path)
        .output()
        .expect("spawn catalog ken check");
    assert!(
        !output.status.success(),
        "the entry's stale reject must fail"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("negative example is stale"),
        "the entry's existing reject-fence diagnostic must be retained: {stderr}"
    );
}

/// Durable invariant: roots loading does not erase the entry document's
/// positive-example obligation.
#[test]
fn failing_example_in_catalog_entry_is_still_refused_after_roots_loading() {
    let entry = format!("{CATALOG_ENTRY}\n```ken example\nconst broken : Nat = True\n```\n");
    let path = write_catalog_fixture("loader_entry_failing_example", &entry, CATALOG_DEPENDENCY);

    let output = Command::new(ken_bin())
        .arg("check")
        .arg(&path)
        .output()
        .expect("spawn catalog ken check");
    assert!(
        !output.status.success(),
        "the entry's failing example must fail"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("'ken example' block failed"),
        "the entry's existing example-fence diagnostic must be retained"
    );
}

/// Durable invariant: document-check roles belong to the selected entry, not
/// to a dependency's exported module interface.
#[test]
fn stale_reject_in_imported_dependency_is_not_run() {
    let dependency =
        format!("{CATALOG_DEPENDENCY}\n```ken reject\nconst stale : Bool = True\n```\n");
    let path = write_catalog_fixture("loader_dependency_stale_reject", CATALOG_ENTRY, &dependency);

    let output = Command::new(ken_bin())
        .arg("check")
        .arg(&path)
        .output()
        .expect("spawn catalog ken check");
    assert!(
        output.status.success(),
        "loading a dependency must not execute its document-check roles; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Durable invariant: paths with no derivable catalog module address retain
/// the pre-existing isolated-file dispatch.
#[test]
fn non_catalog_lowercase_path_keeps_isolated_fallback() {
    let path = write_fixture("non_catalog_lowercase.ken.md", PURE_LIBRARY_KEN_MD);
    let output = Command::new(ken_bin())
        .arg("check")
        .arg(&path)
        .output()
        .expect("spawn non-catalog ken check");
    assert!(
        output.status.success(),
        "a path with no catalog/packages ancestor must retain isolated checking; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// ken check on a file with a failing fence: exits 1, via the identical
// elaborate_ken_md_file Err path ken run's own front half uses.
#[test]
fn check_file_with_failing_fence_exits_one_same_error_path() {
    let path = write_fixture("check_failing_fence.ken.md", FAILING_FENCE_KEN_MD);

    let output = Command::new(ken_bin())
        .arg("check")
        .arg(&path)
        .output()
        .expect("spawn ken check");

    assert!(
        !output.status.success(),
        "ken check must reject a stale ken-reject fence"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("ken check:") && stderr.contains("elaboration error"),
        "ken check's error must go through the shared elaboration-error path: {stderr}"
    );
}

// ken check on a genuinely runnable (IO-shaped) file also succeeds — check
// doesn't care whether the last decl is IO-shaped, only that everything
// elaborates and fences behave. Never runs the IO (no stdout).
#[test]
fn check_io_shaped_file_also_exits_zero_without_running_io() {
    let path = write_fixture("check_io_shaped.ken.md", IO_KEN_MD);

    let output = Command::new(ken_bin())
        .arg("check")
        .arg(&path)
        .output()
        .expect("spawn ken check");

    assert!(
        output.status.success(),
        "ken check on an IO-shaped file must still exit 0; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stdout.is_empty(),
        "ken check must never drive IO, even on an IO-shaped file: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

// `ken run` stays strict: a pure-library file has no named entrypoint and
// fails without auto-detect fallthrough to check mode.
#[test]
fn ken_run_rejects_a_pure_library_without_named_main() {
    let path = write_fixture("run_pure_library.ken.md", PURE_LIBRARY_KEN_MD);

    let output = Command::new(ken_bin())
        .arg("run")
        .arg(&path)
        .output()
        .expect("spawn ken run");

    assert!(
        !output.status.success(),
        "ken run on a pure-library file must fail strictly"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing entrypoint 'main'"),
        "ken run must name the missing entrypoint: {stderr}"
    );
}

// Regression: ken run on a genuine IO-shaped file is byte-for-byte
// unchanged — still drives IO and produces the program's real output.
#[test]
fn ken_run_still_drives_io_on_an_io_shaped_file_unchanged() {
    let path = write_fixture("run_io_shaped.ken.md", IO_KEN_MD);

    let output = Command::new(ken_bin())
        .arg("run")
        .arg(&path)
        .output()
        .expect("spawn ken run");

    assert!(
        output.status.success(),
        "ken run on an IO-shaped file must still succeed; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "check mode ran main? no.\n",
        "ken run must still actually execute the IO program"
    );
}

// A missing argument to ken check gets its own subcommand's usage message,
// not a borrowed "ken run" one.
#[test]
fn check_missing_argument_uses_own_subcommand_name() {
    let output = Command::new(ken_bin())
        .arg("check")
        .output()
        .expect("spawn ken check");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("ken check: missing <file> argument"),
        "stderr should name the check subcommand, not run: {stderr}"
    );
}
