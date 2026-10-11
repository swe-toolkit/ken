//! `LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD` — the declaration-time
//! constructor-spelling diagnostic.
//!
//! The elaborator writes every constructor into one flat `globals` map. The
//! resolver's package definition set now rejects repeated constructors across
//! separate root units of one `ElabEnv` as `DuplicateDefinition`. A prelude
//! constructor is not in that package set, so the persistent insert guard
//! still reports `DuplicateConstructorSpelling` across different packages.
//!
//! Scope is the diagnostic only: qualified / type-directed / overloaded
//! coexistence is deliberately NOT offered (flat-namespace spec §1, Architect
//! ruling), and the flat namespace semantics are unchanged for every accepting
//! program.

use ken_elaborator::ElabEnv;
use ken_elaborator::error::ElabError;

/// Promise class: durable invariant. Two sum families declared in SEPARATE
/// root units of one package cannot share a constructor spelling. The later
/// collision reaches the package definition set's typed duplicate refusal;
/// prelude collisions are exercised by the separate role-authority test.
#[test]
fn same_constructor_spelling_across_two_package_units_is_rejected() {
    let mut env = ElabEnv::empty().expect("prelude elaborates");
    // Separate elaboration passes on one package environment share the
    // resolver's package definition set.
    env.elaborate_file("data ShadowColor = ShadowRed | ShadowGreen\n")
        .expect("first family with a fresh constructor spelling elaborates");
    match env.elaborate_file("data ShadowSignal = ShadowRed | ShadowAmber\n") {
        Err(ElabError::DuplicateDefinition { name, span }) => {
            assert_eq!(
                name, "ShadowRed",
                "the diagnostic must name the colliding spelling"
            );
            assert!(
                span.end > span.start,
                "the later declaration must have a real span"
            );
        }
        other => panic!("expected DuplicateDefinition for `ShadowRed`, got {other:?}"),
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
