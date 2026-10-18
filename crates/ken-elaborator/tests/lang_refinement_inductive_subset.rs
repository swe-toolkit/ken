//! Checked subset-Σ types in data constructors and record fields (21 §2).

use ken_elaborator::{ElabEnv, ObligationKind};
use ken_kernel::Term;

fn has_projected_previous(term: &Term) -> bool {
    if let Term::Proj1(previous) = term {
        let previous = match previous.as_ref() {
            Term::Ascript(value, _) => value.as_ref(),
            other => other,
        };
        if matches!(previous, Term::Var(1)) {
            return true;
        }
    }
    term.children().into_iter().any(has_projected_previous)
}

/// Promise class: durable dependent-constructor scope invariant.
/// MEASURED: the second constructor argument is a kernel Σ referencing its
/// preceding field, and a concrete introduction is kernel-admitted.
/// CLAIMED: checked field telescopes use the same de Bruijn binder scope as
/// final inductive admission. THE GAP: field order matters; the source's
/// second predicate mentions the first field rather than a closed literal.
#[test]
fn dependent_constructor_field_retains_its_prior_binder() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl(
        "data DepBox : Type where { MkDepBox : (x : Int) → (r : { y : Int | Equal Int y x }) → DepBox }",
    ).expect("dependent constructor");
    let (family, index) = env.env.constructor(env.globals["MkDepBox"])
        .expect("checked constructor");
    let Term::Sigma(_, predicate) = &family.constructors[index].args[1] else {
        panic!("the second field must be a subset Σ");
    };
    assert!(predicate.children().into_iter().any(|child| matches!(child, Term::Var(1))),
        "the second predicate must mention the prior field");
    env.elaborate_decl("const dep : DepBox = MkDepBox 5 5")
        .expect("the checked value and evidence inhabit the constructor field");
}

/// Promise class: durable dependent-record introduction invariant.
/// MEASURED: the second record field is a Σ whose predicate projects the
/// carrier of its preceding refined field; the record literal is admitted.
/// CLAIMED: a previous Σ field is never treated as an Int by conversion.
/// THE GAP: the predicate must require Int to force the Proj1 coercion.
#[test]
fn dependent_record_field_reads_the_carrier_of_prior_subset() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl(
        "record DepRecord { a : { x : Int | Equal Int x 5 }, b : { y : Int | Equal Int y a } }",
    ).expect("dependent record with a refined prior field");
    let (_, body) = env.env.transparent_body(env.globals["DepRecord"])
        .expect("record alias has a checked Σ body");
    let Term::Sigma(_, rest) = body else { panic!("record has a first field") };
    let Term::Sigma(second, _) = rest.as_ref() else { panic!("record has a second field") };
    let Term::Sigma(_, predicate) = second.as_ref() else {
        panic!("second field must retain its subset Σ")
    };
    assert!(has_projected_previous(predicate),
        "the predicate must read Proj1 of the earlier subset field: {predicate:?}");
    let value = env.elaborate_decl_v1(
        "const record_value : DepRecord = { a = 5, b = 5 }",
    ).expect("the record literal checks");
    assert!(value.obligations.iter().all(|goal| {
        !matches!(goal.kind, ObligationKind::RefinementIntroduction)
            || !env.is_open_hole(goal.hole_id)
    }), "both equalities have checked certificates");
}
