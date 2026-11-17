//! Checked IO law homes and an open Strict client of their generic statement.
//! Promise class: durable checked-identity and explicit-import invariant.

use std::path::PathBuf;

use ken_elaborator::modules::{PRELUDE_FLOOR_NAMES, PRELUDE_PACKAGE_HOMES};
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, Term};

const OWNER: &str = "Capability.System.IO";
const CLAIM: &str = "theorem client_advance_start (span : BufferSpan) (count : TransferCount) \
    : write_all_advance_start_prop span count = \
    write_all_advance_start span count\n";

fn strict(source: &str) -> Result<ElabEnv, ElabError> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages");
    let mut env = ElabEnv::new()?;
    env.elaborate_isolated_file_strict_v1(&[root], source)?;
    Ok(env)
}

fn spine(mut term: &Term) -> (&Term, Vec<&Term>) {
    let mut args = Vec::new();
    while let Term::App(function, argument) = term {
        args.push(argument.as_ref());
        term = function;
    }
    args.reverse();
    (term, args)
}

/// MEASURED: both law names import from their sole IO home at the original
/// pre-source IDs, and an open theorem's checked statement uses the imported
/// proposition while its body applies the imported proof. CLAIMED: these
/// homes deliver a usable generic law, not merely two selectable names.
/// THE GAP: this checks the generic checked proposition and proof reference,
/// not physical write-loop execution; the existing runtime gates cover that.
#[test]
fn imported_advance_start_law_proves_its_generic_statement_at_old_ids() {
    let prelude = ElabEnv::new().expect("checked pre-source law IDs");
    let source =
        format!("import {OWNER} (write_all_advance_start_prop, write_all_advance_start)\n{CLAIM}");
    let client = strict(&source).expect("imported law proves a generic client theorem");
    for name in ["write_all_advance_start_prop", "write_all_advance_start"] {
        let owners = PRELUDE_PACKAGE_HOMES
            .iter()
            .filter_map(|(owner, member)| (*member == name).then_some(*owner))
            .collect::<Vec<_>>();
        assert_eq!(owners, [OWNER], "{name}: one IO home");
        assert!(!PRELUDE_FLOOR_NAMES.contains(&name), "{name}: not ambient");
        assert_eq!(
            client.globals[name], prelude.globals[name],
            "{name}: original ID"
        );
    }
    let Some(Decl::Transparent { ty, body, .. }) =
        client.env.lookup(client.globals["client_advance_start"])
    else {
        panic!("the open client theorem must be checked and transparent")
    };
    let Term::Pi(_, first) = ty else {
        panic!("span binder missing: {ty:?}")
    };
    let Term::Pi(_, conclusion) = first.as_ref() else {
        panic!("count binder missing: {first:?}")
    };
    let (claim_head, claim_args) = spine(conclusion);
    assert!(
        matches!(claim_head, Term::Const { id, .. }
        if *id == prelude.globals["write_all_advance_start_prop"]),
        "the generic statement must use the imported pre-source proposition: {conclusion:?}"
    );
    assert_eq!(
        claim_args,
        [&Term::var(1), &Term::var(0)],
        "open span/count claim"
    );
    let Term::Lam(_, first) = body else {
        panic!("span proof binder missing: {body:?}")
    };
    let Term::Lam(_, proof) = first.as_ref() else {
        panic!("count proof binder missing: {first:?}")
    };
    let (proof_head, proof_args) = spine(proof);
    assert!(
        matches!(proof_head, Term::Const { id, .. }
        if *id == prelude.globals["write_all_advance_start"]),
        "the proof must actually apply the imported pre-source law: {proof:?}"
    );
    assert_eq!(
        proof_args,
        [&Term::var(1), &Term::var(0)],
        "open span/count proof"
    );
}

/// MEASURED: dropping only the proposition import, then only the proof import,
/// from the same generic client rejects at the missing spelling. CLAIMED:
/// neither homed identity is available through a Strict ambient path.
/// THE GAP: an unbound result alone cannot show the law was usable; the
/// positive open-theorem row independently checks both IDs and their use.
#[test]
fn unimported_advance_start_proposition_and_proof_refuse_at_each_name() {
    for (import, missing) in [
        ("write_all_advance_start", "write_all_advance_start_prop"),
        ("write_all_advance_start_prop", "write_all_advance_start"),
    ] {
        let source = format!("import {OWNER} ({import})\n{CLAIM}");
        match strict(&source) {
            Err(ElabError::UnboundName { name, span }) => {
                assert_eq!(name, missing);
                let start = source
                    .rfind(missing)
                    .expect("the missing law is in the theorem");
                assert_eq!(
                    (span.start, span.end),
                    (start, start + missing.len()),
                    "refusal must land on the missing proposition or proof token"
                );
            }
            Err(error) => panic!("{missing}: wrong refusal: {error:?}"),
            Ok(_) => panic!("{missing}: missing import was accepted"),
        }
    }
}
