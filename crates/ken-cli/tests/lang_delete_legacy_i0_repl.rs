//! I0 interactive import at the actual CLI :def dispatch.
//! Promise class: durable invariant: a rooted REPL loads an explicit provider,
//! while a rootless REPL cannot invent one.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn repl(cwd: &Path, commands: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("repl")
        .current_dir(cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(commands.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

/// MEASURED: :def import loads the selected publisher, :list stays empty,
/// and a subsequent :type succeeds. CLAIMED: interactive import works at
/// the user-facing command boundary. THE GAP: the REPL unit test compares
/// the checked id, since textual output alone does not convey identity.
#[test]
fn repl_def_import_reaches_the_home_without_faking_a_definition() {
    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = repl(
        &repo,
        ":def import Capability.Process.Exit (host_pure)\n:list\n:type host_pure\n:quit\n",
    );
    assert!(
        output.contains("imported: import Capability.Process.Exit (host_pure)"),
        "{output}"
    );
    assert!(output.contains("(no definitions in session)"), "{output}");
    assert!(!output.contains("error:"), "{output}");
}

/// MEASURED: with no selected root, a real external import still fails.
/// CLAIMED: ordinary REPL roots are explicit discovery, never ambient
/// discovery from an unrelated previous run.
#[test]
fn repl_outside_catalog_tree_refuses_the_same_import() {
    // tempfile's configured default is under this repository, where an
    // ancestor has a real catalog. Put the negative in a rootless parent.
    let dir = tempfile::Builder::new().tempdir_in("/tmp").unwrap();
    assert!(ken_elaborator::modules::isolated_catalog_root(&dir.path().join("repl.ken")).is_none());
    let output = repl(
        dir.path(),
        ":def import Capability.Process.Exit (host_pure)\n:quit\n",
    );
    assert!(output.contains("error:"), "{output}");
    assert!(
        !output.contains("imported: import Capability.Process.Exit"),
        "{output}"
    );
}
