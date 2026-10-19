//! Recursive unindexed constructor equations lack checked evidence until
//! LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW (spec 22 §3, 39 §2(8)).

use ken_elaborator::{ElabEnv, ObligationKind};
use ken_kernel::{check, Context, Decl, GlobalId, Term};

fn contains_equation_on_family(term: &Term, family: GlobalId) -> bool {
    if let Term::Eq(carrier, _, _) = term {
        let mut head = carrier.as_ref();
        while let Term::App(function, _) = head {
            head = function;
        }
        if matches!(head, Term::IndFormer { id, .. } if *id == family) {
            return true;
        }
    }
    term.children()
        .iter()
        .any(|child| contains_equation_on_family(child, family))
}

fn list_match_motive(term: &Term, list: GlobalId) -> Option<&Term> {
    if let Term::Elim { fam, motive, .. } = term {
        if *fam == list {
            return Some(motive);
        }
    }
    term.children()
        .iter()
        .find_map(|child| list_match_motive(child, list))
}

/// Transition sentinel: until LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW installs
/// real recursive equations, the Nil leaf's open hole has no List equation,
/// and the checked `xs` motive is the plain subset Σ. A new checked recursive
/// equation may change both observations but must never invent a proof.
/// MEASURED: production `sort`'s emitted Nil obligation and checked Elim.
/// CLAIMED: only term-evidenced facts are used by the leaf hole and motive.
/// THE GAP: a condition accidentally installed at a different site remains
/// outside this row; the mutation controls test both these production sites.
#[test]
fn recursive_sort_nil_goal_omits_unwitnessed_list_eq_and_uses_plain_sigma_motive() {
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_decl(
        "fn insert (a : Type) (leq : a → a → Bool) (x : a) (xs : List a) : List a = \
         match xs { Nil |-> Cons a x (Nil a) ; Cons h t |-> match leq x h { \
         True |-> Cons a x (Cons a h t) ; False |-> Cons a h (insert a leq x t) } }",
    )
    .expect("checked insertion helper");
    let result = env
        .elaborate_decl_v1(
            "fn sort (a : Type) (leq : a → a → Bool) (xs : List a) : \
             { ys : List a | And (is_sorted a leq ys) (Perm a ys xs) } = \
             match xs { Nil |-> Nil a ; Cons h t |-> insert a leq h (sort a leq t) }",
        )
        .expect("recursive sort with two open result refinements");
    assert_eq!(
        result.obligations.len(),
        2,
        "Nil and Cons are independent leaves"
    );
    let nil = &result.obligations[0];
    assert!(matches!(nil.kind, ObligationKind::RefinementIntroduction));
    assert!(
        env.is_open_hole(nil.hole_id),
        "Nil cannot prove Perm Nil xs without evidence"
    );
    let list = env.globals["List"];
    assert!(
        !contains_equation_on_family(&nil.goal_closed, list),
        "Nil cannot assume the missing Eq (List a) xs (Nil a): {:?}",
        nil.goal_closed
    );
    let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(result.def_id) else {
        panic!("sort remains a checked transparent definition")
    };
    check(&env.env, &Context::new(), body, ty).expect("kernel checks the emitted sort");
    let motive = list_match_motive(body, list).expect("the xs match uses the List eliminator");
    let motive_body = match motive {
        Term::Ascript(function, _) => function.as_ref(),
        other => other,
    };
    let Term::Lam(_, result) = motive_body else {
        panic!("the xs motive abstracts over its scrutinee: {motive:?}")
    };
    assert!(
        matches!(result.as_ref(), Term::Sigma(..)),
        "a recursive unindexed split uses the plain checked Σ motive, not an Eq convoy: {motive:?}"
    );
}
