use std::collections::BTreeSet;
use std::fs;

use ken_elaborator::{modules, parser, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};
use tempfile::tempdir;

const NEEDS_PREMISE: &str = "const ac0_need : Int requires Equal Int 0 0 = 0";
const BAD_CALLER: &str = "const ac0_bad : Bool = ac0_need";
const SPACE_DECL_AFTER_HOLE: &str = r#"
space S {
  mut cell : Int = 0
  proc call () : Bool visits [S] = ac0_space_callee 0 0
}
"#;

fn trusted_ids(env: &ElabEnv) -> BTreeSet<GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

fn added_ids(before: &BTreeSet<GlobalId>, after: &BTreeSet<GlobalId>) -> BTreeSet<GlobalId> {
    after.difference(before).copied().collect()
}

fn assert_type_mismatch(error: ElabError) {
    assert!(
        matches!(
            error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "fixture must fail after the requires hole is made: {error:?}"
    );
}

/// MEASURED: the exact trusted-base delta after the direct API's TypeMismatch.
/// CLAIMED: a failed declaration leaves no unreported obligation and the next
/// successful declaration owns the reused ID.
/// THE GAP: this row exercises the direct `ElabEnv` route; the other shared
/// environment entry points have separate rows below.
#[test]
fn failed_reused_elabenv_declaration_rolls_back_and_reuses_its_id() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_decl_v1(NEEDS_PREMISE)
        .expect("the callee declaration itself is checked");
    let before = trusted_ids(&env);
    let first_after_mark = env.env.next_global_id();

    let error = env
        .elaborate_decl_v1(BAD_CALLER)
        .expect_err("an Int result cannot check against the caller's Bool result");
    let after = trusted_ids(&env);
    assert_eq!(added_ids(&before, &after), BTreeSet::new());
    assert_type_mismatch(error);

    assert!(
        !env.globals.contains_key("ac0_bad"),
        "the failed declaration must not leave a spelling binding"
    );
    let recovered = env
        .elaborate_decl_v1("const ac0_after : Bool = True")
        .expect("the environment remains reusable");
    assert_eq!(recovered.def_id, first_after_mark);
    let true_id = env.globals["True"];
    assert!(matches!(
        env.env.lookup(recovered.def_id),
        Some(Decl::Transparent { body, .. })
            if *body == Term::constructor(true_id, Vec::new())
    ));
    let aliases: Vec<_> = env
        .globals
        .iter()
        .filter_map(|(name, id)| (*id == recovered.def_id).then_some(name.as_str()))
        .collect();
    assert_eq!(aliases, ["ac0_after"]);
}

/// MEASURED: the exact trusted-base delta from `expand_and_elaborate`'s Err.
/// CLAIMED: this reusable batch API rolls back the failed declaration.
/// THE GAP: an empty delta alone is vacuous; the exact kernel TypeMismatch
/// and the direct API's hole-producing control establish that the fixture
/// reaches the post-hole failure path.
#[test]
fn failed_expand_and_elaborate_declaration_has_exact_zero_delta() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_decl_v1(NEEDS_PREMISE)
        .expect("the callee declaration itself is checked");
    let declarations = parser::parse_decls(BAD_CALLER).expect("fixture parses");
    let before = trusted_ids(&env);

    let result = modules::expand_and_elaborate(&mut env, &declarations);
    let after = trusted_ids(&env);
    assert_eq!(added_ids(&before, &after), BTreeSet::new());
    assert_type_mismatch(result.expect_err("the checked caller has the wrong result type"));
}

/// MEASURED: the exact trusted-base delta from the roots-loaded entry's Err.
/// CLAIMED: `load_unit` leaves no orphan assumption after the failed member.
/// THE GAP: the loaded file contains a successful callee first, so the Err and
/// delta are scoped to the later failed declaration rather than parse failure.
#[test]
fn failed_roots_loaded_declaration_has_exact_zero_delta() {
    let root = tempdir().expect("temporary catalog root");
    fs::write(
        root.path().join("Entry.ken"),
        format!("{NEEDS_PREMISE}\n{BAD_CALLER}\n"),
    )
    .expect("write the roots-loader fixture");
    let mut env = ElabEnv::new().expect("base environment");
    let before = trusted_ids(&env);

    let result = env.elaborate_module_from_roots(&[root.path().to_path_buf()], "Entry");
    let after = trusted_ids(&env);
    assert_eq!(added_ids(&before, &after), BTreeSet::new());
    assert_type_mismatch(result.expect_err("the entry's caller has the wrong result type"));
}

/// Promise class: durable rollback invariant (AC-1 and AC-3).
/// MEASURED: a real SpaceDecl calls a requires-bearing function with false
/// arguments, returns the exact kernel TypeMismatch, and leaves no new trusted
/// IDs after its post-hole failure.
/// CLAIMED: the SpaceDecl rollback seam removes obligations created before its
/// later operation error while preserving the caller-visible failure.
/// THE GAP: the failed call must reach the real SpaceDecl production wrapper;
/// deleting only that wrapper is the AC-3 mutation for this row.
#[test]
fn failed_space_decl_after_premise_hole_has_exact_zero_delta() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_decl_v1(
        r#"fn ac0_space_callee (x : Int) (y : Int) : Int requires Not (Equal Int x y) = x"#,
    )
    .expect("the requires-bearing callee is checked");

    let before = trusted_ids(&env);
    let declarations =
        parser::parse_decls(SPACE_DECL_AFTER_HOLE).expect("the space fixture parses");
    let result = modules::expand_and_elaborate(&mut env, &declarations);
    let after = trusted_ids(&env);
    assert_eq!(added_ids(&before, &after), BTreeSet::new());
    assert_type_mismatch(result.expect_err("the SpaceDecl operation has an Int/Bool mismatch"));
}

/// MEASURED: the exact trusted-base delta from an expression that allocates
/// an `Axiom` postulate before application rejects it.
/// CLAIMED: failed standalone expressions do not leave an unreported opaque.
/// THE GAP: the same expression must reach the postulate-producing check path;
/// the positive Axiom control below verifies that path remains live.
#[test]
fn failed_standalone_axiom_expression_rolls_back_its_postulate() {
    let mut env = ElabEnv::new().expect("base environment");
    let before = trusted_ids(&env);

    let error = env
        .elaborate_expr("ac0_expr_bad", "(Axiom : Top) Proved")
        .expect_err("a proof of Top is not a function");
    let after = trusted_ids(&env);
    assert_eq!(added_ids(&before, &after), BTreeSet::new());
    assert!(matches!(error, ElabError::NotAFunction { .. }), "{error:?}");
}

/// MEASURED: kernel `trusted_base()` additions by identity, compared with
/// reported obligation IDs or the exact `Opaque` returned by each AX-2 path.
/// CLAIMED: successful reported holes and explicit Axiom forms retain trust.
/// THE GAP: a no-addition result could pass vacuously, so each control requires
/// and inspects its positive opaque witness.
#[test]
fn successful_reported_hole_and_axiom_controls_keep_their_trust() {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_decl_v1(NEEDS_PREMISE)
        .expect("the callee declaration itself is checked");
    let before = trusted_ids(&env);
    let result = env
        .elaborate_decl_v1("const ac0_good : Int = ac0_need")
        .expect("the caller reports its open requires proof");
    let after = trusted_ids(&env);
    let reported: BTreeSet<_> = result
        .obligations
        .iter()
        .map(|obligation| obligation.hole_id)
        .collect();
    assert!(
        !reported.is_empty(),
        "the positive control must make a hole"
    );
    assert_eq!(added_ids(&before, &after), reported);

    let before = trusted_ids(&env);
    let explicit = env
        .elaborate_decl_v1("axiom ac0_explicit : Top")
        .expect("explicit axiom declaration");
    let after = trusted_ids(&env);
    let explicit_added = added_ids(&before, &after);
    assert_eq!(explicit_added.len(), 1);
    let explicit_id = *explicit_added.iter().next().expect("one axiom postulate");
    assert!(matches!(
        env.env.lookup(explicit_id),
        Some(Decl::Opaque { name, .. }) if name == "ac0_explicit"
    ));

    let before = trusted_ids(&env);
    env.elaborate_expr("ac0_standalone_axiom", "Axiom : Top")
        .expect("standalone Axiom expression");
    let after = trusted_ids(&env);
    let standalone = added_ids(&before, &after);
    assert_eq!(standalone.len(), 1);
    let standalone_id = *standalone.iter().next().expect("one opaque entry");
    assert!(matches!(
        env.env.lookup(standalone_id),
        Some(Decl::Opaque { name, .. }) if name == "ac0_standalone_axiom"
    ));
}
