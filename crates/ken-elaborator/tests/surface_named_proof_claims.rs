//! SURF named proof-claim machinery.
//!
//! Pins `spec/30-surface/33-declarations.md` §8 and the conformance seed:
//! `prop`, standalone `theorem`, and attached `proof <name> for <subject>` are
//! ordinary checked proof terms over the existing proof lane. Attached proofs
//! resolve by canonical `subject::proof_name`, never by a bare proof name.

use ken_elaborator::{ElabEnv, ElabError};

fn mk_env() -> ElabEnv {
    ElabEnv::new().expect("base env construction failed")
}

fn elaborate_ok(src: &str) -> ElabEnv {
    let mut env = mk_env();
    env.elaborate_file(src)
        .unwrap_or_else(|e| panic!("source should elaborate: {e}"));
    env
}

fn elaborate_err(src: &str) -> ElabError {
    let mut env = mk_env();
    env.elaborate_file(src)
        .expect_err("source should be rejected")
}

#[test]
fn prop_seed_shape_elaborates_without_trusted_base_growth() {
    let mut env = mk_env();
    let before = env.env.trusted_base();

    env.elaborate_file(
        r#"
        prop HasProof (a : Type) : Omega where {
          intro : HasProof a
        }
        "#,
    )
    .expect("seed prop elaborates");

    assert!(env.globals.contains_key("HasProof"));
    assert!(env.globals.contains_key("HasProof.intro"));
    assert_eq!(
        before,
        env.env.trusted_base(),
        "prop seed elaboration must not grow the trusted base"
    );
}

#[test]
fn local_prop_intro_resolves_through_the_family_name() {
    let env = elaborate_ok(
        r#"
        prop HasProof (a : Type) : Omega where {
          intro : HasProof a
        }
        theorem consume (a : Type) : HasProof a = HasProof.intro a
        "#,
    );

    assert!(env.globals.contains_key("HasProof"));
    assert!(env.globals.contains_key("HasProof.intro"));
    assert!(env.globals.contains_key("consume"));
}

#[test]
fn standalone_theorem_is_checked_in_ordinary_namespace() {
    let env = elaborate_ok(
        r#"
        theorem int_self (x : Int) : Equal Int x x = Refl
        theorem consume (x : Int) : Equal Int x x = int_self x
        "#,
    );

    assert!(env.globals.contains_key("int_self"));
    assert!(env.globals.contains_key("consume"));
}

#[test]
fn attached_proof_canonical_path_and_selector_both_resolve() {
    let env = elaborate_ok(
        r#"
        fn id (x : Int) : Int = x
        proof id_self for id (x : Int) : Equal Int (id x) x = Refl
        theorem consume_path (x : Int) : Equal Int (id x) x = id::id_self x
        theorem consume_selector (x : Int) : Equal Int (id x) x =
          (proof id_self for id) x
        "#,
    );

    assert!(env.globals.contains_key("id"));
    assert!(env.globals.contains_key("id::id_self"));
    assert!(!env.globals.contains_key("id_self"));
    assert!(env.globals.contains_key("consume_path"));
    assert!(env.globals.contains_key("consume_selector"));
}

#[test]
fn bare_attached_proof_name_is_not_in_ordinary_namespace() {
    let err = elaborate_err(
        r#"
        fn id (x : Int) : Int = x
        proof id_self for id (x : Int) : Equal Int (id x) x = Refl
        theorem bad (x : Int) : Equal Int (id x) x = id_self x
        "#,
    );

    match err {
        ElabError::UnboundName { name, .. } | ElabError::UnresolvedCon { name, .. } => {
            assert_eq!(name, "id_self");
        }
        other => panic!("expected bare attached proof name rejection, got {other:?}"),
    }
}

#[test]
fn non_proof_theorem_result_is_rejected() {
    let err = elaborate_err("theorem not_proof (x : Int) : Int = x");

    match err {
        ElabError::TypeMismatch { reason, .. } => {
            assert!(
                reason.contains("Omega"),
                "diagnostic should name the proof sort, got {reason}"
            );
        }
        other => panic!("expected TypeMismatch for non-proof theorem, got {other:?}"),
    }
}

#[test]
fn attached_proof_subject_telescope_must_match() {
    let err = elaborate_err(
        r#"
        fn id (x : Int) : Int = x
        proof wrong for id (x : Bool) : Equal Bool x x = Refl
        "#,
    );

    match err {
        ElabError::TypeMismatch { reason, .. } => {
            assert!(
                reason.contains("subject")
                    || reason.contains("telescope")
                    || reason.contains("parameter"),
                "diagnostic should name the subject signature, got {reason}"
            );
        }
        other => panic!("expected subject telescope TypeMismatch, got {other:?}"),
    }
}

#[test]
fn attached_proof_can_depend_on_same_subject_attached_proof() {
    let env = elaborate_ok(
        r#"
        fn id (x : Int) : Int = x
        proof p1 for id (x : Int) : Equal Int (id x) x = Refl
        proof p2 for id (x : Int) : Equal Int (id x) x = id::p1 x
        "#,
    );

    assert!(env.globals.contains_key("id::p1"));
    assert!(env.globals.contains_key("id::p2"));
}

#[test]
fn duplicate_attached_proof_name_on_same_subject_is_rejected() {
    let err = elaborate_err(
        r#"
        fn id (x : Int) : Int = x
        proof p for id (x : Int) : Equal Int (id x) x = Refl
        proof p for id (x : Int) : Equal Int (id x) x = Refl
        "#,
    );

    match err {
        ElabError::DuplicateDefinition { name, .. } => {
            assert_eq!(name, "id::p", "duplicate proof rejection must name the collision");
        }
        other => panic!("expected duplicate attached-proof rejection, got {other:?}"),
    }
}

#[test]
fn helper_theorem_can_bridge_same_subject_attached_proofs() {
    let env = elaborate_ok(
        r#"
        fn id (x : Int) : Int = x
        proof p1 for id (x : Int) : Equal Int (id x) x = Refl
        theorem helper (x : Int) : Equal Int (id x) x = id::p1 x
        proof p2 for id (x : Int) : Equal Int (id x) x = helper x
        "#,
    );

    assert!(env.globals.contains_key("id::p1"));
    assert!(env.globals.contains_key("helper"));
    assert!(env.globals.contains_key("id::p2"));
}

#[test]
fn module_selective_import_exposes_canonical_attached_proof_only() {
    let env = elaborate_ok(
        r#"
        module M {
          pub fn id (x : Int) : Int = x
          pub proof id_self for id (x : Int) : Equal Int (id x) x = Refl
        }
        import M (id)
        theorem consume (x : Int) : Equal Int (id x) x = id::id_self x
        "#,
    );

    assert!(env.globals.contains_key("M.id::id_self"));
    assert!(!env.globals.contains_key("id_self"));
    assert!(env.globals.contains_key("consume"));
}

#[test]
fn module_selective_import_exposes_prop_intro_without_bare_intro() {
    let env = elaborate_ok(
        r#"
        module M {
          pub prop HasProof (a : Type) : Omega where {
            intro : HasProof a
          }
        }
        import M (HasProof)
        theorem consume (a : Type) : HasProof a = HasProof.intro a
        "#,
    );

    assert!(env.globals.contains_key("M.HasProof"));
    assert!(env.globals.contains_key("M.HasProof.intro"));
    assert!(env.globals.contains_key("consume"));
}

#[test]
fn qualified_module_import_exposes_prop_intro_through_family_path() {
    let env = elaborate_ok(
        r#"
        module M {
          pub prop HasProof (a : Type) : Omega where {
            intro : HasProof a
          }
        }
        import M
        theorem consume (a : Type) : M.HasProof a = M.HasProof.intro a
        "#,
    );

    assert!(env.globals.contains_key("M.HasProof"));
    assert!(env.globals.contains_key("M.HasProof.intro"));
    assert!(env.globals.contains_key("consume"));
}

// Durable invariant: only the declared intro of a publicly exported prop
// follows the qualified family path, including an alias or facade rename.
#[test]
fn qualified_prop_intro_follows_alias_and_reexported_family_identity() {
    let env = elaborate_ok(
        r#"
        module M {
          pub prop HasProof (a : Type) : Omega where {
            intro : HasProof a
          }
        }
        module Facade { export M (HasProof as Claim) }
        module InScope { import M (HasProof) export HasProof as Proof }
        import M as Alias
        import Facade as F
        import InScope
        theorem through_alias (a : Type) : Alias.HasProof a = Alias.HasProof.intro a
        theorem through_facade (a : Type) : F.Claim a = F.Claim.intro a
        theorem through_in_scope (a : Type) : InScope.Proof a = InScope.Proof.intro a
        "#,
    );
    assert!(env.globals.contains_key("through_alias"));
    assert!(env.globals.contains_key("through_facade"));
    assert!(env.globals.contains_key("through_in_scope"));
    assert!(!env.globals.contains_key("Facade.Claim.intro"));
}

// Durable invariant: an exported family grants only its actually declared
// intro helpers; selective import does not grant an unrelated module prefix.
#[test]
fn qualified_prop_intro_rejects_private_unknown_and_unimported_paths() {
    let source = r#"
        module M {
          pub prop HasProof (a : Type) : Omega where {
            intro : HasProof a
          }
          prop Hidden (a : Type) : Omega where {
            intro : Hidden a
          }
        }
    "#;
    for (imports, family, expression, missing) in [
        (
            "import M",
            "M.HasProof",
            "M.HasProof.missing",
            "M.HasProof.missing",
        ),
        ("import M", "M.HasProof", "M.Hidden.intro", "M.Hidden.intro"),
        (
            "import M (HasProof)",
            "HasProof",
            "M.HasProof.intro",
            "M.HasProof.intro",
        ),
        (
            "import M as Alias",
            "Alias.HasProof",
            "M.HasProof.intro",
            "M.HasProof.intro",
        ),
    ] {
        // Keep the theorem's type well-bound in each scope; refusal must
        // identify the attempted selector, not a missing family type.
        let src = format!(
            "{source}\nmodule Client {{ {imports} \
             theorem bad (a : Type) : {family} a = {expression} a }}"
        );
        let mut env = mk_env();
        match env.elaborate_file(&src) {
            Err(ElabError::UnboundName { name, .. }) => assert_eq!(name, missing),
            Err(other) => panic!("expected rejected intro path {missing}, got {other:?}"),
            Ok(_) => panic!("unexpectedly accepted intro path {expression}"),
        }
    }
}

#[test]
fn public_attached_proof_requires_public_subject() {
    let err = elaborate_err(
        r#"
        module M {
          fn id (x : Int) : Int = x
          pub proof id_self for id (x : Int) : Equal Int (id x) x = Refl
        }
        "#,
    );

    match err {
        ElabError::UnboundName { name, .. } => {
            assert_eq!(name, "id");
        }
        other => panic!("expected public proof/private subject rejection, got {other:?}"),
    }
}

#[test]
fn prop_constructor_shape_outside_seed_subset_rejects() {
    let err = elaborate_err(
        r#"
        prop Bad (a : Type) : Omega where {
          intro : a -> Bad a
        }
        "#,
    );

    match err {
        ElabError::TypeMismatch { reason, .. } => {
            assert!(
                reason.contains("seed") || reason.contains("intro"),
                "diagnostic should name the bounded prop subset, got {reason}"
            );
        }
        other => panic!("expected bounded prop-shape rejection, got {other:?}"),
    }
}
