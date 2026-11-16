//! Checked-identity and strict-import boundary for Process pipe operations.
//! Spec 38 §1.11.1–§1.11.2; no new prelude declaration or trusted-base entry.

use std::path::PathBuf;

use ken_elaborator::modules::{PRELUDE_FLOOR_NAMES, PRELUDE_PACKAGE_HOMES};
use ken_elaborator::{ElabEnv, ElabError};

const HOME: &str = "Capability.Process.Child";
const NAMES: [&str; 3] = ["write_pipe", "close_pipe_write", "close_pipe_read"];

fn strict(source: &str) -> Result<ElabEnv, ElabError> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    let mut env = ElabEnv::new()?;
    env.elaborate_isolated_file_strict_v1(&[root], source)?;
    Ok(env)
}

/// Promise class: durable invariant. MEASURED: a Strict client imports all
/// three pipe operations through their one home, and each selected checked
/// identity equals its pre-source ID without extra trust beyond home loading.
/// CLAIMED: the exports home existing operations, not lookalike declarations.
/// THE GAP: this checks identity and import reachability, not runtime effects;
/// the interpreter row drives write, early close, and wait independently.
#[test]
fn strict_imports_bind_three_pipe_operations_to_existing_checked_ids() {
    let baseline = ElabEnv::new().expect("checked prelude");
    let home = strict(&format!("import {HOME}\n")).expect("load the Child home");
    let imported = strict(&format!(
        "import {HOME} (write_pipe, close_pipe_write, close_pipe_read)\n\
         export write_pipe, close_pipe_write, close_pipe_read\n"
    ))
    .expect("three explicit checked imports");
    for name in NAMES {
        let homes = PRELUDE_PACKAGE_HOMES
            .iter()
            .filter_map(|(owner, member)| (*member == name).then_some(*owner))
            .collect::<Vec<_>>();
        assert_eq!(homes, [HOME], "{name}: exactly one explicit home");
        assert!(!PRELUDE_FLOOR_NAMES.contains(&name), "{name}: not ambient");
        assert_eq!(
            imported.globals[name], baseline.globals[name],
            "{name}: old ID"
        );
    }
    assert_eq!(
        imported.env.trusted_base(),
        home.env.trusted_base(),
        "imports add no trust beyond the same home"
    );
}

/// Promise class: durable invariant. MEASURED: each of the three identical
/// Strict clients refuses at its chosen bare operation when its import is
/// removed. CLAIMED: the public source path requires an explicit package
/// import. THE GAP: this rules out an accidental floor binding, but does not
/// by itself establish that the selected ID is the original pre-source ID.
#[test]
fn unimported_pipe_operations_refuse_each_bare_name_in_strict() {
    for name in NAMES {
        let client = format!("export {name}\n");
        strict(&format!("import {HOME} ({name})\n{client}"))
            .unwrap_or_else(|err| panic!("{name}: positive twin refused: {err:?}"));
        match strict(&client) {
            Err(ElabError::UnboundName { name: missing, .. }) => assert_eq!(missing, name),
            Err(err) => panic!("{name}: wrong missing-import refusal: {err:?}"),
            Ok(_) => panic!("{name}: import was not required"),
        }
    }
}
