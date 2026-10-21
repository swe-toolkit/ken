//! Demand-only recursive constructor equations, without reading a method IH
//! (LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW; spec 22 §3, 34 §3.3).

use ken_elaborator::{ElabEnv, ObligationKind};
use ken_kernel::{check, subst::shift, Context, Decl, GlobalId, Term};

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

fn list_match_elim(term: &Term, list: GlobalId) -> Option<&Term> {
    if matches!(term, Term::Elim { fam, .. } if *fam == list) {
        return Some(term);
    }
    term.children()
        .iter()
        .find_map(|child| list_match_elim(child, list))
}

fn contains_const(term: &Term, id: GlobalId) -> bool {
    matches!(term, Term::Const { id: found, .. } if *found == id)
        || term
            .children()
            .iter()
            .any(|child| contains_const(child, id))
}

/// Promise class: durable invariant (spec 22 §3 and 34 §3.3).
/// MEASURED: the checked recursive `sort` Nil obligation includes its List
/// constructor equation, the motive carries it, and the Cons method still
/// calls the SCT-admitted global without using its method IH. CLAIMED: a
/// demanded case-only convoy changes branch evidence, not recursion. THE GAP:
/// this does not establish behavior for a consuming method IH; AC-4 excludes it.
#[test]
fn recursive_sort_nil_goal_carries_checked_list_eq_without_consuming_ih() {
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
        contains_equation_on_family(&nil.goal_closed, list),
        "Nil needs its checked Eq (List a) xs (Nil a): {:?}",
        nil.goal_closed
    );
    let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(result.def_id) else {
        panic!("sort remains a checked transparent definition")
    };
    check(&env.env, &Context::new(), body, ty).expect("kernel checks the emitted sort");
    let Term::Elim {
        motive, methods, ..
    } = list_match_elim(body, list).expect("xs uses the List eliminator")
    else {
        unreachable!()
    };
    let motive_body = match motive.as_ref() {
        Term::Ascript(function, _) => function.as_ref(),
        other => other,
    };
    let Term::Lam(_, motive_result) = motive_body else {
        panic!("the xs motive abstracts over its scrutinee: {motive:?}")
    };
    assert!(
        matches!(motive_result.as_ref(), Term::Pi(domain, _) if contains_equation_on_family(domain, list)),
        "the checked motive must carry the List constructor equation: {motive:?}"
    );
    let cons = &methods[1];
    assert!(
        contains_const(cons, result.def_id),
        "Cons retains the direct sort call"
    );
    let Term::Lam(_, cons) = cons else {
        panic!("Cons binds head")
    };
    let Term::Lam(_, cons) = cons.as_ref() else {
        panic!("Cons binds tail")
    };
    let Term::Lam(_, cons) = cons.as_ref() else {
        panic!("Cons binds the unused method IH")
    };
    assert_eq!(
        shift(&shift(cons, -1, 0), 1, 0),
        **cons,
        "Cons must not consume its method IH"
    );
}
