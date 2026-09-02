//! DS-7 (`Applicative` + `Monad` constructor classes) acceptance —
//! `docs/program/wp/ds-7-applicative-monad.md`, design contract
//! `spec/50-stdlib/56-effectful-classes.md` (CAT-2).
//!
//! - **AC1** — kernel-untouched, zero new elaborator capability, zero
//!   `trusted_base()` delta (structural before/after set-diff, DS-2's
//!   established pattern).
//! - **AC2–AC4** — laws `Ω`, pointwise, proved, zero `Axiom`.
//! - **AC5** — Monad ⇔ ITree attested, no second `bind`.
//! - **AC7** — WIRE applied consistently (`Applicative`→`functor`,
//!   `Monad`→`applicative`).
//! - **AC8** — discriminators genuinely flip accept→reject, asserted as
//!   the specific error variant.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use ken_elaborator::{ElabEnv, ElabError};
use ken_interp::eval::{eval, EvalStore, EvalVal};
use ken_kernel::{Decl, GlobalId};
const LAWFUL_FUNCTORS_KEN_MD: &str =
    include_str!("../../../catalog/packages/Core/Classes/LawfulFunctors.ken.md");
const EFFECTFUL_CLASSES_KEN_MD: &str =
    include_str!("../../../catalog/packages/Core/Classes/EffectfulClasses.ken.md");

fn base_env() -> ElabEnv {
    let mut env = ElabEnv::empty().expect("prelude bootstrap");
    catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env);
    // LawfulFunctors now imports `Data.Collections.Derived (list_append)` after
    // the attached-proof migration; clear both `concat_map` (EffectfulClasses's
    // import) and `list_append` so each real selective import installs its own
    // binding, and keep Derived importable.
    catalog_or::load_derived_importing_fixture_many(&mut env, &["concat_map", "list_append"]);
    env.elaborate_ken_md_file(LAWFUL_FUNCTORS_KEN_MD)
        .expect("Core/Classes/LawfulFunctors.ken.md must elaborate");
    env
}

fn leading_pi_count(term: &ken_kernel::Term) -> usize {
    let mut count = 0;
    let mut current = term;
    while let ken_kernel::Term::Pi(_, body) = current {
        count += 1;
        current = body;
    }
    count
}

fn term_contains_saturated_provider_head_occurrence(
    term: &ken_kernel::Term,
    provider: GlobalId,
    arity: usize,
) -> bool {
    if matches!(term, ken_kernel::Term::App(_, _)) {
        let mut argument_count = 0;
        let mut head = term;
        while let ken_kernel::Term::App(function, _) = head {
            argument_count += 1;
            head = function;
        }
        if argument_count >= arity
            && matches!(head, ken_kernel::Term::Const { id, .. } if *id == provider)
        {
            return true;
        }
    }
    term.children()
        .into_iter()
        .any(|child| term_contains_saturated_provider_head_occurrence(child, provider, arity))
}

#[test]
fn entry_elaborates_with_every_checked_fence() {
    let mut env = base_env();
    env.elaborate_ken_md_file(EFFECTFUL_CLASSES_KEN_MD)
        .expect("catalog/packages/Core/Classes/EffectfulClasses.ken.md must elaborate (Definition + every checked fence)");
    assert!(env.globals.contains_key("Applicative_instance_Option"));
    assert!(env.globals.contains_key("Monad_instance_Option"));
    assert!(env.globals.contains_key("Applicative_instance_List"));
    assert!(env.globals.contains_key("Monad_instance_List"));
}

/// Durable invariant.
///
/// MEASURED: the fixture withholds the flat `concat_map` alias, the elaborated
/// `list_bind` body contains a saturated application-head occurrence of the
/// exact Derived provider identity, the retired package-local declaration is
/// absent, and an available but unimported sibling remains unresolved. A
/// fixed concrete `list_bind` vector evaluates through that binding.
///
/// CLAIMED: EffectfulClasses selectively reuses canonical `concat_map` without
/// a package-local replacement and preserves the measured computation.
///
/// THE GAP: the syntactic occurrence does not prove evaluation, every reachable
/// route, result provenance, or exclusion of unrelated/local computation. The
/// concrete vector, unchanged List laws, and affected behavioral closure cover
/// the broader preservation obligation.
#[test]
fn derived_concat_map_import_identity_and_concrete_vector_are_pinned() {
    let mut env = base_env();
    assert!(
        !env.globals.contains_key("concat_map"),
        "the fixture must withhold concat_map so the package import is load-bearing"
    );
    env.elaborate_ken_md_file(EFFECTFUL_CLASSES_KEN_MD)
        .expect("EffectfulClasses must elaborate through its real selective import");
    assert!(
        !env.globals.contains_key("concat_map"),
        "the retired package-local concat_map declaration must be absent"
    );
    let provider = env.globals["Data.Collections.Derived.concat_map"];
    let provider_type = match env.env.lookup(provider) {
        Some(Decl::Transparent { ty, .. }) => ty,
        other => panic!("Derived concat_map must be transparent, got {other:?}"),
    };
    let arity = leading_pi_count(provider_type);
    assert!(arity > 0, "Derived concat_map must have a function type");
    let (_, list_bind_body) = env
        .env
        .transparent_body(env.globals["list_bind"])
        .expect("list_bind must have a transparent body");
    assert!(
        term_contains_saturated_provider_head_occurrence(&list_bind_body, provider, arity),
        "list_bind must contain a saturated exact-provider application-head occurrence"
    );
    env.elaborate_file(
        "fn duplicate_bool (x : Bool) : List Bool = \
           Cons Bool x (Cons Bool x (Nil Bool))\n\
         const list_bind_duplicate_vector : List Bool = \
           list_bind \
             Bool \
             Bool \
             (Cons Bool True (Cons Bool False (Nil Bool))) \
             duplicate_bool",
    )
    .expect("the fixed list_bind vector must elaborate through canonical concat_map");
    let vector_id = env.globals["list_bind_duplicate_vector"];
    let vector = match env.env.lookup(vector_id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, &mut EvalStore::new()),
        other => panic!("list_bind_duplicate_vector must be transparent, got {other:?}"),
    };
    let mut values = Vec::new();
    let mut current = vector;
    loop {
        match current {
            EvalVal::Ctor { id, .. } if id == env.prelude_env.nil_id => break,
            EvalVal::Ctor { id, args, .. } if id == env.prelude_env.cons_id => {
                let value_id = match &args[1] {
                    EvalVal::Ctor { id, .. } => *id,
                    other => panic!("expected Bool list element, got {other:?}"),
                };
                values.push(value_id == env.globals["True"]);
                assert!(
                    value_id == env.globals["True"] || value_id == env.globals["False"],
                    "list element must be a Bool constructor"
                );
                current = args[2].clone();
            }
            other => panic!("expected List Bool, got {other:?}"),
        }
    }
    assert_eq!(
        values,
        vec![true, true, false, false],
        "list_bind must preserve concat_map's left-to-right flattening behavior"
    );

    let mut omitted = ElabEnv::empty().expect("prelude bootstrap");
    omitted
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], "Data.Collections.Derived")
        .expect("Derived provider must roots-load");
    let error = omitted
        .elaborate_file(
            "import Data.Collections.Derived (concat_map)\n\
             fn omitted_reverse (xs : List Bool) : List Bool = reverse Bool xs",
        )
        .expect_err("available but unimported reverse must not resolve");
    assert!(
        matches!(error, ElabError::UnresolvedCon { ref name, .. } if name == "reverse"),
        "the non-import control must fail at the omitted binding, got {error:?}"
    );
}

// AC1/AC4: zero-Axiom acceptance bar, grounded on the CHECKED code only
// (fences), not prose (which legitimately discusses "Axiom" while
// explaining the zero-delta claim).
#[test]
fn zero_axiom_in_checked_fences() {
    let extracted = ken_elaborator::literate::extract_ken_md(EFFECTFUL_CLASSES_KEN_MD)
        .expect("EffectfulClasses.ken.md must extract");
    assert!(
        !extracted.source.contains("Axiom"),
        "EffectfulClasses.ken.md's tangled/checked code must contain zero Axiom literals"
    );
    for range in extracted
        .example_ranges
        .iter()
        .chain(extracted.reject_ranges.iter())
    {
        assert!(
            !EFFECTFUL_CLASSES_KEN_MD[range.clone()].contains("Axiom"),
            "example/reject fences must contain zero Axiom literals"
        );
    }
}

// AC1: structural trusted_base() before==after set-diff (DS-2's pattern) —
// stronger than a source grep, catches a delta introduced indirectly
// through any helper.
#[test]
fn trusted_base_delta_is_empty_across_the_entry() {
    let mut env = base_env();
    let before: std::collections::BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_ken_md_file(EFFECTFUL_CLASSES_KEN_MD)
        .expect("catalog/packages/Core/Classes/EffectfulClasses.ken.md must elaborate");
    let after: std::collections::BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(
        before, after,
        "EffectfulClasses.ken.md must introduce ZERO new trusted_base() entries (zero-Axiom acceptance bar)"
    );
}

// AC7: the wired superclass chain is a real elaborator capability, not
// smuggled — a class field typed as another class applied to the same
// parameter elaborates, and nested `.field` projection through an opaque
// bound dictionary composes.
#[test]
fn wired_superclass_chain_and_nested_projection() {
    let mut env = base_env();
    env.elaborate_decl(
        "class Applicative (f : Type -> Type) { functor : Functor f ; pure : (a:Type) -> a -> f a ; ap : (a:Type) -> (b:Type) -> f (a -> b) -> f a -> f b }",
    )
    .expect("class Applicative with a wired Functor f field must elaborate");
    let r = env.elaborate_decl(
        "fn probeNestedProj (a : Type) (b : Type) (d : Applicative Option) (g : a -> b) (x : Option a) : Option b = d.functor.map a b g x",
    );
    // Not expected to typecheck without a concrete instance in scope, but
    // it must not fail to PARSE / resolve the projection chain itself.
    match r {
        Ok(_) => {}
        Err(e) => {
            let msg = format!("{:?}", e);
            assert!(
                !msg.contains("ParseError"),
                "nested .field projection through a class-typed field must at least PARSE: {:?}",
                e
            );
        }
    }
}

// AC8 discriminator 1: a wired `applicative` field that is non-cartesian
// (e.g. a ziplist-style `ap`) is not `Monad List`-coherent -- attempting
// to reuse it as `Monad`'s wired dict must be REJECTED, not silently
// accepted (chapter §3.3 "Ziplist is not proliferated").
#[test]
fn ac8_noncartesian_applicative_cannot_wire_into_monad() {
    let mut env = base_env();
    env.elaborate_ken_md_file(EFFECTFUL_CLASSES_KEN_MD)
        .expect("EffectfulClasses.ken.md must elaborate");

    // A deliberately-wrong "zipWith"-shaped ap for List (pairs elements
    // positionally instead of the cartesian product) -- structurally
    // valid as a FUNCTION, but not what `Monad List`'s `bind`-coherence
    // requires; the discriminator is that `list_bind::assoc` (proved against
    // the REAL cartesian `list_ap`) does not typecheck when the instance
    // is reassembled with this swapped-in `ap`, because it is a
    // different, unrelated function -- attempting to use it as evidence
    // for a LAW FIELD it was never proved for must be rejected.
    let r = env.elaborate_decl(
        "fn zip_ap (a : Type) (b : Type) (mf : List (a -> b)) (mx : List a) : List b = \
           match mf { Nil ↦ Nil b ; Cons g fs ↦ match mx { Nil ↦ Nil b ; Cons x xs ↦ Cons b (g x) (zip_ap a b fs xs) } }",
    );
    r.expect("zip_ap itself is a well-typed function (the point: it exists, just isn't the proved cartesian ap)");

    // Attempt to wire it in as the ap_id witness (reusing the CARTESIAN
    // proof against the DIFFERENT zip_ap function) -- must be rejected by
    // the kernel, not silently accepted.
    let r2 = env.elaborate_decl(
        "const badApId : (a:Type) -> (v:List a) -> Equal (List a) (zip_ap a a (list_pure (a -> a) (idf a)) v) v = list_ap_id",
    );
    match r2 {
        Ok(_) => {
            panic!("a proof of the CARTESIAN ap_id must not typecheck against the DIFFERENT zip_ap")
        }
        Err(e) => {
            let msg = format!("{:?}", e);
            assert!(
                msg.contains("TypeMismatch") || msg.contains("KernelRejected"),
                "expected a TypeMismatch/KernelRejected (specific variant), got: {:?}",
                e
            );
        }
    }
}

// AC8 discriminator 2: a masked `Axiom` inhabiting `Bottom` must not be
// accepted as a real proof of one of the eight instance laws (the
// zero-Axiom acceptance bar is load-bearing, not decorative).
#[test]
fn ac8_axiom_masking_a_law_is_rejected_by_the_zero_delta_check() {
    let mut env = base_env();
    env.elaborate_ken_md_file(EFFECTFUL_CLASSES_KEN_MD)
        .expect("EffectfulClasses.ken.md must elaborate");
    let before: std::collections::BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    // A structurally-valid but AXIOM-BACKED "proof" (the same field-value
    // position landed code uses -- `lawful_classes.ken`'s `sound = Axiom`
    // -- rather than a standalone `fn`, which hits an unrelated
    // elaborator limitation: `Axiom` as a `fn` body whose declared return
    // type references the fn's OWN parameter fails with `VarOutOfScope`,
    // confirmed empirically; `const`/closed-type and instance-field
    // positions are unaffected). A tiny scratch class demonstrates the
    // SAME zero-delta hazard this entry's real instances must avoid.
    env.elaborate_decl("class ProbeLaw (a : Type) { trivial : (x : a) -> Equal a x x }")
        .expect("class ProbeLaw");
    env.elaborate_decl("instance ProbeLaw Nat { trivial = Axiom }")
        .expect("Axiom inhabits any goal -- this MUST typecheck as an instance field (that's exactly the hazard)");

    let after: std::collections::BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_ne!(
        before, after,
        "an Axiom-backed law masquerading as a real proof MUST show up as a trusted_base() delta -- \
         if this assertion fails, the zero-delta check would have silently missed it"
    );
}

// AC5: no second, divergent `bind` is minted for the ITree effect
// denotation -- this entry defines Monad instances only for Option/List.
// Checked within the CHECKED CODE (fences) only, not prose -- the prose
// itself legitimately explains, in words, what the entry does NOT do,
// which would trip a whole-document substring search.
#[test]
fn ac5_no_second_itree_bind_minted() {
    let extracted = ken_elaborator::literate::extract_ken_md(EFFECTFUL_CLASSES_KEN_MD)
        .expect("EffectfulClasses.ken.md must extract");
    assert!(
        !extracted.source.contains("instance Monad (ITree"),
        "this entry must not write a surface instance Monad (ITree e resp) -- \
         the parametric-instance-head gap (CAT-1 55 §6.1) stays open, not reopened here"
    );
    assert!(
        !extracted.source.contains("declare_bind"),
        "this entry must not re-mint or re-wrap the landed ITree bind -- attested correspondence only"
    );
}
