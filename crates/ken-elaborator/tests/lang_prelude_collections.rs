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
fn whnf_list_elements(
    env: &ElabEnv,
    nil_id: GlobalId,
    cons_id: GlobalId,
    term: &Term,
) -> Vec<Term> {
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
    assert!(
        kept_args.is_empty(),
        "Zero takes no arguments; got {kept_args:?}"
    );
}

/// AC-5 -- each of the four new combinators is a transparent definition,
/// not a trusted-base postulate, by name. (Narrowed by
/// LANG-PRELUDE-ELABORATION-DEPTH D5a: this is a per-name claim about these
/// four identifiers, not the wider claim that the trusted base does not
/// grow under any addition -- an entry can be registered under a name
/// other than the declaration that raised it, `prover.rs:493-501`'s
/// `emit_unknown_hole` registering under the literal `"prover unknown
/// goal"` being the existing example. `d5b_trusted_base_full_enumeration_
/// from_bare_env` below is the full-enumeration control for that wider
/// property.) `sort`'s `is_sorted ∧ Perm` obligation is deliberately
/// excluded from the prelude (it would enter as an undischarged postulate,
/// `elab.rs:53-57`); this asserts the per-name mechanism directly rather
/// than a raw before/after count (a frozen size is a snapshot that a later,
/// unrelated prelude addition would have to keep updating, and a
/// coincidental +1/-1 elsewhere could mask a real regression here).
/// `map`/`fold`/`zip`/`filter` are ordinary `fn` declarations with real
/// bodies -- the only way any of them could grow `trusted_base()` is if
/// elaboration left one of them as an undischarged postulate/obligation
/// instead of a transparent definition, which is exactly what this checks
/// per name, mirroring the existing
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

/// Every `trusted_base()` entry's label, tagged with its `ken_kernel::Decl`
/// kind (LANG-TRUSTED-BASE-LABEL-KIND-TAG D1): `Opaque(name)` for an opaque
/// declaration's own kernel-recorded `name` (`ken_kernel::Decl::Opaque`'s
/// audit label -- what `declare_postulate` was actually called with, which
/// is what `prover.rs`'s "prover unknown goal" and `bytes.rs`'s
/// "BytesRoundTripLaw" both show can diverge from the declaration that
/// raised the obligation), or `Primitive(name)` for a primitive, where
/// `name` is the elaborator's public surface name if one resolves and the
/// literal `"<unregistered>"` otherwise -- some entries are deliberately
/// removed from the public name map after use (`conversions.rs`'s unchecked
/// ABI-scalar narrowing primitives, "not a public API"), so they have no
/// surface name at all, by design rather than by omission, and must still
/// be visible in the enumeration (D2). Tagging by kind is what makes a
/// postulate quietly becoming a primitive under the same spelling visible:
/// before this WP, `Opaque(add_int)` and a same-spelled `Primitive(add_int)`
/// rendered identically as the untagged `"add_int"`. `trusted_base()`'s own
/// filter (`env.rs`) admits only `Opaque` and non-literal `Primitive`
/// declarations, so `Transparent`/`Inductive` are enumerated declarations
/// that can never actually reach this `match`; the arm panics rather than
/// silently mislabeling them, so a regression in that filter reds here
/// instead of producing a wrong tag.
///
/// `by_global_name`'s `HashMap<u32, &String>` is built by `.collect()`ing
/// `env.globals` (`HashMap<String, GlobalId>`), which would be
/// order-dependent -- and the surface-name lookup flaky -- if any `GlobalId`
/// had two names. `D3` enforces this directly rather than only measuring it:
/// the `assert_eq!` below fails the moment `env.globals` ever admits a
/// second name for one id, instead of leaving it a doc-comment claim that
/// could go stale silently.
fn trusted_base_labels(env: &ElabEnv) -> Vec<String> {
    use ken_kernel::Decl;
    let by_global_name: std::collections::HashMap<u32, &String> =
        env.globals.iter().map(|(name, id)| (id.0, name)).collect();
    assert_eq!(
        by_global_name.len(),
        env.globals.len(),
        "env.globals is not injective; labels would be order-dependent"
    );
    let mut labels: Vec<String> = env
        .env
        .trusted_base()
        .iter()
        .map(|id| {
            let decl = env
                .env
                .declarations()
                .iter()
                .find(|d| d.id() == *id)
                .expect("every trusted_base id must resolve to a declaration");
            let surface_name = || {
                by_global_name
                    .get(&id.0)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "<unregistered>".to_string())
            };
            match decl {
                Decl::Opaque { name, .. } => format!("Opaque({name})"),
                Decl::Primitive { .. } => format!("Primitive({})", surface_name()),
                Decl::Transparent { .. } | Decl::Inductive(_) => panic!(
                    "trusted_base() yielded a {decl:?} entry; its own filter admits \
                     only Opaque and non-literal Primitive declarations"
                ),
            }
        })
        .collect();
    labels.sort();
    labels
}

/// D5b/AC-6 -- the full `trusted_base()` enumeration from a bare env, by
/// name, not a count. It is not itself a live differential, but precisely:
/// an *env-level* differential over the whole trusted base is impossible,
/// because `ElabEnv::new()` has no "before" env to diff against. A
/// *block-level* bracket over a sub-range of registration is a different
/// and available instrument -- `LANG-PRELUDE-COMBINATOR-BLOCK-DELTA D2`
/// brackets exactly the four `List` combinator `elaborate_decl` calls in
/// `prelude.rs`, following the established `conversions.rs:303/364` idiom,
/// and asserts their contribution to the trusted base is empty.
/// 107 entries: large enough to be a finding about the shape of the
/// trusted base in its own right (per D5b, that finding is reported rather
/// than a reason to fall back to the per-name check AC-5 already is) --
/// nearly all of it is the numeric-tower floor (`numbers.rs`/
/// `conversions.rs`), not user-authored postulates. `LANG-TRUSTED-BASE-
/// LABEL-KIND-TAG D1`/`D4` retagged every entry with its `Decl` kind without
/// changing membership: the count stays 107. Three entries are
/// `"Primitive(<unregistered>)"`: `conversions.rs`'s
/// `int_to_{usize,isize,cint}_raw`, each deliberately dropped from
/// `env.globals` after its safe wrapper and retract postulate are built
/// (`conversions.rs` ~L359, "unchecked narrowing is not a public API") --
/// present in the trusted base with no public surface name, which is the
/// exact shape this enumeration exists to make visible rather than let a
/// per-name check silently miss. That visibility has a real edge: three
/// collided entries make the *count* of unnamed members visible (three, not
/// two or four), but the tagged label cannot distinguish one unnamed entry
/// being replaced by a different unnamed one -- both render as the same
/// `"Primitive(<unregistered>)"` string, so a substitution among the three
/// is invisible here even though growth or shrinkage of the unnamed
/// population is not. What the kind tag newly makes visible is the
/// orthogonal movement: an entry changing from `Opaque(x)` to
/// `Primitive(x)` (or the reverse) under the same spelling `x`, which the
/// untagged `Vec<String>` this enumeration replaced could not see at all.
///
/// `LANG-PRELUDE-COMBINATOR-BLOCK-DELTA D3` -- this enumeration and the
/// block delta above are not redundant, and neither retires the other: the
/// block delta is `GlobalId`-keyed and covers only the four combinators'
/// contribution, while this enumeration is label-keyed and covers the
/// *whole* 107-entry trusted base, carrying census value the block delta
/// does not (the shape-of-the-trusted-base finding two paragraphs up is a
/// property of this list specifically). Seeing an id-keyed delta land next
/// to this label-keyed census is not a reason to retire the census.
#[test]
fn d5b_trusted_base_full_enumeration_from_bare_env() {
    let env = ElabEnv::new().expect("base env");
    let labels = trusted_base_labels(&env);
    let mut expected: Vec<&str> = vec![
        "Opaque(BytesRoundTripLaw)",
        "Opaque(NoOvfAddInt16)",
        "Opaque(NoOvfAddInt32)",
        "Opaque(NoOvfAddInt64)",
        "Opaque(NoOvfAddInt8)",
        "Opaque(NoOvfAddUInt16)",
        "Opaque(NoOvfAddUInt32)",
        "Opaque(NoOvfAddUInt64)",
        "Opaque(NoOvfAddUInt8)",
        "Opaque(RecordNil)",
        "Opaque(bytes_list_roundtrip)",
        "Opaque(cint_int_retract)",
        "Opaque(decidable equality complete)",
        "Opaque(decidable equality sound)",
        "Opaque(decimalPow10Unbounded)",
        "Opaque(isize_int_retract)",
        "Opaque(list_bytes_roundtrip)",
        "Opaque(record_nil_val)",
        "Opaque(uint8_int_retract)",
        "Opaque(usize_int_retract)",
        "Primitive(<unregistered>)",
        "Primitive(<unregistered>)",
        "Primitive(<unregistered>)",
        "Primitive(Bytes)",
        "Primitive(CInt)",
        "Primitive(Cap)",
        "Primitive(Float)",
        "Primitive(Float32)",
        "Primitive(ISize)",
        "Primitive(Int)",
        "Primitive(Int16)",
        "Primitive(Int32)",
        "Primitive(Int64)",
        "Primitive(Int8)",
        "Primitive(Resource)",
        "Primitive(String)",
        "Primitive(UInt16)",
        "Primitive(UInt32)",
        "Primitive(UInt64)",
        "Primitive(UInt8)",
        "Primitive(USize)",
        "Primitive(add_float)",
        "Primitive(add_float32)",
        "Primitive(add_int)",
        "Primitive(add_int16)",
        "Primitive(add_int32)",
        "Primitive(add_int64)",
        "Primitive(add_int8)",
        "Primitive(add_uint16)",
        "Primitive(add_uint32)",
        "Primitive(add_uint64)",
        "Primitive(add_uint8)",
        "Primitive(and_bool)",
        "Primitive(byte_length)",
        "Primitive(bytes_at)",
        "Primitive(bytes_concat)",
        "Primitive(bytes_decode)",
        "Primitive(bytes_encode)",
        "Primitive(bytes_length)",
        "Primitive(bytes_slice)",
        "Primitive(bytes_to_list)",
        "Primitive(char_length)",
        "Primitive(cint_to_int)",
        "Primitive(div_float)",
        "Primitive(eq_float)",
        "Primitive(eq_float32)",
        "Primitive(eq_int)",
        "Primitive(int16_to_int)",
        "Primitive(int32_to_int)",
        "Primitive(int64_to_int)",
        "Primitive(int8_to_int)",
        "Primitive(int_to_int16_raw)",
        "Primitive(int_to_int32_raw)",
        "Primitive(int_to_int64_raw)",
        "Primitive(int_to_int8_raw)",
        "Primitive(int_to_uint16_raw)",
        "Primitive(int_to_uint32_raw)",
        "Primitive(int_to_uint64_raw)",
        "Primitive(int_to_uint8_raw)",
        "Primitive(isize_to_int)",
        "Primitive(leq_int)",
        "Primitive(list_char_to_string)",
        "Primitive(list_to_bytes)",
        "Primitive(mul_float)",
        "Primitive(mul_int)",
        "Primitive(neg_int16)",
        "Primitive(neg_int32)",
        "Primitive(neg_int64)",
        "Primitive(neg_int8)",
        "Primitive(not_bool)",
        "Primitive(or_bool)",
        "Primitive(string_to_list_char)",
        "Primitive(sub_float)",
        "Primitive(sub_int)",
        "Primitive(uint16_to_int)",
        "Primitive(uint32_to_int)",
        "Primitive(uint64_to_int)",
        "Primitive(uint8_to_int)",
        "Primitive(usize_to_int)",
        "Primitive(wrapping_add_int16)",
        "Primitive(wrapping_add_int32)",
        "Primitive(wrapping_add_int64)",
        "Primitive(wrapping_add_int8)",
        "Primitive(wrapping_add_uint16)",
        "Primitive(wrapping_add_uint32)",
        "Primitive(wrapping_add_uint64)",
        "Primitive(wrapping_add_uint8)",
    ];
    expected.sort();
    assert_eq!(
        labels, expected,
        "bare ElabEnv::new()'s trusted_base() enumeration changed -- if this \
         is an intended new postulate/primitive, name it in this list; if \
         not, something newly failed to discharge as a transparent \
         definition"
    );
}
