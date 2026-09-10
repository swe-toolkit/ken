//! `LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD` — the declaration-time
//! duplicate-constructor-spelling diagnostic.
//!
//! The elaborator writes every constructor into one flat `globals` map. The
//! resolver's `DuplicateDefinition` check catches a repeated constructor
//! spelling WITHIN a single resolution pass, but two families declared in
//! SEPARATE passes (separate files / `elaborate_*` calls on one persistent
//! `ElabEnv`) are resolved independently, so the second family's constructor
//! silently replaced the first's binding — surfacing downstream as an unrelated
//! `TypeMismatch`. The guard closes that gap at the persistent insert site: it
//! detects an insert whose spelling is already bound to a constructor of a
//! DIFFERENT family and rejects with `DuplicateConstructorSpelling`, naming both
//! declaration sites.
//!
//! Scope is the diagnostic only: qualified / type-directed / overloaded
//! coexistence is deliberately NOT offered (flat-namespace spec §1, Architect
//! ruling), and the flat namespace semantics are unchanged for every accepting
//! program.

use ken_elaborator::error::ElabError;
use ken_elaborator::ElabEnv;

/// Promise class: durable invariant. Two sum families declared in SEPARATE
/// resolution passes and sharing a constructor spelling reject with the exact
/// diagnostic, naming both sites with real (non-degenerate) spans — the earlier
/// span proves the persistent `ctor_decl_spans` registry was consulted, not the
/// `Span::zero()` fallback.
#[test]
fn same_constructor_spelling_across_two_sum_families_is_rejected_with_both_sites() {
    let mut env = ElabEnv::empty().expect("prelude elaborates");
    // Two SEPARATE elaboration passes on one persistent env: the resolver's
    // within-pass DuplicateDefinition cannot see across them, so this is the
    // silent-shadow gap the guard closes.
    env.elaborate_file("data ShadowColor = ShadowRed | ShadowGreen\n")
        .expect("first family with a fresh constructor spelling elaborates");
    match env.elaborate_file("data ShadowSignal = ShadowRed | ShadowAmber\n") {
        Err(ElabError::DuplicateConstructorSpelling {
            name,
            first_span,
            second_span,
        }) => {
            assert_eq!(name, "ShadowRed", "the diagnostic must name the colliding spelling");
            assert!(
                first_span.end > first_span.start,
                "the EARLIER declaration site must be a real span (registry consulted, not the \
                 zero fallback): {first_span:?}"
            );
            assert!(
                second_span.end > second_span.start,
                "the LATER declaration site must be a real span: {second_span:?}"
            );
        }
        other => panic!("expected DuplicateConstructorSpelling for `ShadowRed`, got {other:?}"),
    }
}

/// Control: distinct constructor spellings across separate families compile —
/// the guard fires ONLY on a genuine cross-family collision, not on every
/// second `data` declaration.
#[test]
fn distinct_constructor_spellings_across_families_compile() {
    let mut env = ElabEnv::empty().expect("prelude elaborates");
    env.elaborate_file("data DistinctA = OnlyAlpha | OnlyBeta\n")
        .expect("first family");
    env.elaborate_file("data DistinctB = OnlyGamma | OnlyDelta\n")
        .expect("distinct spellings in a separate family must compile");
}

/// The guard is scoped to CONSTRUCTORS of a different family: a family may reuse
/// its OWN constructor spellings across a re-elaboration boundary is not the
/// case here, but a single family declaring distinct constructors is accepted,
/// and reusing a spelling only within the same declaration is a separate
/// (pre-existing) concern. This pins that an ordinary single family compiles.
#[test]
fn a_single_family_with_distinct_constructors_compiles() {
    let mut env = ElabEnv::empty().expect("prelude elaborates");
    env.elaborate_file("data Lonely = SoloOne | SoloTwo | SoloThree\n")
        .expect("a single sum family with distinct constructors compiles");
}
