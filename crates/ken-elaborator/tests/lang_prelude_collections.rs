//! LANG-PRELUDE-COLLECTIONS -- `map`/`fold`/`zip`/`filter` are reachable
//! from a bare prelude environment (`37 §9`, WS-L), with no test-local
//! redeclaration. `spec/30-surface/37-strings-collections.md §9` requires
//! these "in the surface/elaborator + prelude"; before this WP they were
//! declared only inside `tests/l3a_acceptance.rs`'s `setup_combinators`, so
//! a program that merely imports the prelude had a `List` type and no
//! operation over it.

use ken_elaborator::ElabEnv;
use ken_kernel::{whnf, Context, GlobalId, Term};

/// AC-1 -- the combinators are reachable from a bare prelude env. `env` gets
/// no `elaborate_decl` of `map`/`fold`/`zip`/`filter` themselves; the single
/// declaration below both applies all four and composes their outputs into
/// each other (zip's `Prod` output feeds map's projection, map's `List Nat`
/// feeds filter, filter's result feeds fold), so a well-typed elaboration is
/// only possible if every one of the four already exists in the bare
/// prelude. This fails today for all four -- before this WP, `ElabEnv::new()`
/// has no `map`/`fold`/`zip`/`filter` global at all.
#[test]
fn ac1_prelude_combinators_reachable_from_bare_env() {
    let mut env = ElabEnv::new().expect("base env");
    env.elaborate_decl(
        "const uses_all_four_combinators : Nat = \
         fold Nat Nat (\\h acc. Suc acc) Zero \
           (filter Nat (\\n. match n { Zero |-> True ; Suc m |-> False }) \
             (map (Prod Nat Nat) Nat (\\p. match p { MkProd x y |-> x }) \
               (zip Nat Nat (Cons Nat Zero (Nil Nat)) (Cons Nat Zero (Nil Nat)))))",
    )
    .expect(
        "a declaration applying map/fold/zip/filter must elaborate against \
         a bare prelude env with no test-local combinator declaration",
    );
}

/// Peel a fully-applied `App` spine to its head `GlobalId` (constructor or
/// const) and its arguments in application order -- mirrors
/// `l3a_acceptance.rs::peel_app_head`, extended to also collect args so the
/// concrete `List` structure below can be walked and compared.
fn peel_app(term: &Term) -> (Option<GlobalId>, Vec<Term>) {
    let mut cur = term;
    let mut args = Vec::new();
    loop {
        match cur {
            Term::App(f, a) => {
                args.push((**a).clone());
                cur = f;
            }
            Term::Constructor { id, .. } | Term::Const { id, .. } => {
                args.reverse();
                return (Some(*id), args);
            }
            _ => {
                args.reverse();
                return (None, args);
            }
        }
    }
}

/// Walk a `List Nat` value (already reduced to whnf) all the way down its
/// `Cons`/`Nil` spine, whnf-reducing each successive tail, and return the
/// `Nat`-typed head terms in order. Panics if the spine is not a `List Nat`
/// shape (only `Cons`/`Nil` heads, each `Cons` applied to exactly the type
/// arg + head + tail).
fn whnf_list_elements(env: &ElabEnv, nil_id: GlobalId, cons_id: GlobalId, term: &Term) -> Vec<Term> {
    let mut elements = Vec::new();
    let mut cur = whnf(&env.env, &Context::new(), term);
    loop {
        let (head, args) = peel_app(&cur);
        match head {
            Some(id) if id == nil_id => return elements,
            Some(id) if id == cons_id => {
                assert_eq!(
                    args.len(),
                    3,
                    "Cons must be applied to exactly (type, head, tail); got {args:?}"
                );
                elements.push(args[1].clone());
                cur = whnf(&env.env, &Context::new(), &args[2]);
            }
            other => panic!("expected a List Nat spine of Cons/Nil, got head {other:?} in {cur:?}"),
        }
    }
}

/// AC-2 -- `filter` computes, not merely elaborates. `is_zero` rejects two of
/// the three elements of `[Zero, Suc Zero, Suc (Suc Zero)]`, so a `filter`
/// that type-checked but silently returned its input unchanged (the failure
/// this AC exists to catch) would produce a 3-element list here, not the
/// 1-element `[Zero]` this test asserts.
#[test]
fn ac2_filter_computes_and_rejects_at_least_one_element() {
    let mut env = ElabEnv::new().expect("base env");
    let nil_id = env.globals["Nil"];
    let cons_id = env.globals["Cons"];
    let zero_id = env.globals["Zero"];

    let (term, _ty) = env
        .elaborate_expr(
            "ac2_filter_computes_and_rejects_at_least_one_element",
            "filter Nat (\\n. match n { Zero |-> True ; Suc m |-> False }) \
             (Cons Nat Zero (Cons Nat (Suc Zero) (Cons Nat (Suc (Suc Zero)) (Nil Nat))))",
        )
        .expect("filter application over a concrete Nat list elaborates");

    let elements = whnf_list_elements(&env, nil_id, cons_id, &term);
    assert_eq!(
        elements.len(),
        1,
        "filter over [Zero, Suc Zero, Suc (Suc Zero)] with is_zero must keep only Zero; got {elements:?}"
    );
    let (kept_head, kept_args) = peel_app(&whnf(&env.env, &Context::new(), &elements[0]));
    assert_eq!(
        kept_head,
        Some(zero_id),
        "the single surviving element must be Zero; got {kept_head:?} {kept_args:?}"
    );
    assert!(kept_args.is_empty(), "Zero takes no arguments; got {kept_args:?}");
}

/// AC-5 -- the trusted base does not grow. `sort`'s `is_sorted ∧ Perm`
/// obligation is deliberately excluded from the prelude (it would enter as
/// an undischarged postulate, `elab.rs:53-57`); this asserts the boundary
/// mechanism directly rather than a raw before/after count (a frozen size
/// is a snapshot that a later, unrelated prelude addition would have to
/// keep updating, and a coincidental +1/-1 elsewhere could mask a real
/// regression here). `map`/`fold`/`zip`/`filter` are ordinary `fn`
/// declarations with real bodies -- the only way any of them could grow
/// `trusted_base()` is if elaboration left one of them as an undischarged
/// postulate/obligation instead of a transparent definition, which is
/// exactly what this checks per name, mirroring the existing
/// `!env.env.trusted_base().contains(&unfold_id)` idiom
/// `fuel_bounded_unfold_produces_finite_prefix` already uses for
/// `unfoldUpTo`.
#[test]
fn ac5_new_combinators_add_zero_trusted_base_entries() {
    let env = ElabEnv::new().expect("base env");
    let trusted = env.env.trusted_base();
    for name in ["map", "fold", "zip", "filter"] {
        let id = env.globals[name];
        assert!(
            !trusted.contains(&id),
            "{name} must be a transparent definition, not a trusted-base postulate"
        );
    }
}
