//! Catalog conjunction and structural sortedness, per
//! spec/30-surface/32-grammar.md §3 and 37-strings-collections.md §6.
//! Promise class: durable checked-identity and closed-behavior invariants.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{conv::convert_type, env::Context, Decl, KernelError, Level, Term};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

/// SOURCE: 32 §3 expression-position Sigma; 33 §3.2 renamed export.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the real package roots-loads four owned checked declarations;
/// `And` selectively imports and resolves to `and`, not a second GlobalId.
/// Both source conversion directions and a mixed prelude projection check;
/// swapped conjuncts refuse, and the exact trusted-identity set is unchanged.
/// CLAIMED: conjunction reuses the Omega Sigma, with no new assumption.
/// THE GAP: this checks the generic types, not every independent consumer;
/// their root-loading and strict-floor census are tested separately.
#[test]
fn conjunction_is_checked_owned_convertible_and_zero_trust() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let prelude_and = env.globals["And"];
    let owned = env
        .elaborate_module_from_roots(&[root()], "Core.Logic.Conjunction")
        .expect("catalog conjunction roots-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(before, after, "conjunction must add no trusted identities");
    assert_eq!(owned.len(), 4, "definition and three checked projections");
    let conjunction = env.globals["Core.Logic.Conjunction.and"];
    assert_ne!(conjunction, prelude_and);
    assert!(!env.globals.contains_key("Core.Logic.Conjunction.And"));
    for name in ["and", "and_intro", "and_fst", "and_snd"] {
        let id = env.globals[&format!("Core.Logic.Conjunction.{name}")];
        assert!(owned.contains(&id), "{name} has one catalog owner");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(
        "import Core.Logic.Conjunction (And as CAnd, and_intro as c_intro, and_fst as c_fst, and_snd as c_snd)\n\
         theorem to_catalog (a : Omega) (b : Omega) (p : And a b) : CAnd a b = p\n\
         theorem to_prelude (a : Omega) (b : Omega) (p : CAnd a b) : And a b = p\n\
         theorem mixed_first (a : Omega) (b : Omega) (x : a) (y : b) : a = and_fst a b (c_intro a b x y)\n\
         theorem selected_first (a : Omega) (b : Omega) (x : a) (y : b) : a = c_fst a b (c_intro a b x y)\n\
         theorem selected_second (a : Omega) (b : Omega) (x : a) (y : b) : b = c_snd a b (c_intro a b x y)",
    )
    .expect("both generic conversion directions and all helpers check");
    let a = Term::var(1);
    let b = Term::var(0);
    let prelude = Term::app(
        Term::app(Term::const_(prelude_and, vec![]), a.clone()),
        b.clone(),
    );
    let catalog = Term::app(Term::app(Term::const_(conjunction, vec![]), a), b);
    let context = Context {
        types: vec![Term::omega(Level::zero()), Term::omega(Level::zero())],
    };
    assert!(convert_type(&env.env, &context, &catalog, &prelude));
    assert!(convert_type(&env.env, &context, &prelude, &catalog));
    match env.elaborate_file(
        "import Core.Logic.Conjunction (And as CAnd)\n\
         theorem wrong_order (a : Omega) (b : Omega) (p : CAnd a b) : And b a = p",
    ) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            ..
        }) => {}
        other => panic!("swapped conjuncts must refuse in the kernel: {other:?}"),
    }
}

/// SOURCE: 37 §6 explicit-comparator structural sortedness.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the real Derived package owns one checked public sortedness
/// definition, distinct from prelude's identity. An alias-importing consumer
/// accepts empty, singleton and ordered two-element cases; an unordered
/// adjacent pair rejects its attempted proof. Open proofs cannot cross either
/// direction between the distinct prelude and catalog identities, and a bare
/// loose import refuses ambiguity rather than choosing one identity. Provider
/// and Derived trusted GlobalId sets agree exactly.
/// CLAIMED: the moved predicate is a checked Omega definition, not a
/// second prelude alias or a new trusted assumption.
/// THE GAP: closed examples do not prove the generic sort laws; those are
/// separately checked in Derived and its consumers.
#[test]
fn sortedness_is_derived_checked_and_rejects_an_unordered_pair() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let prelude_sorted = env.globals["is_sorted"];
    for module in [
        "Core.Function.Combinators",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Core.Logic.Compare",
        "Core.Classes.LawfulClasses",
        "Core.Logic.And",
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Logic.OrdResult",
        "Core.Logic.Transport",
        "Data.Collections.List",
    ] {
        env.elaborate_module_from_roots(&[root()], module)
            .unwrap_or_else(|error| panic!("{module} provider loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], "Data.Collections.Derived")
        .expect("structural sortedness roots-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(before, after, "Derived adds no trusted GlobalId");
    let sorted = env.globals["Data.Collections.Derived.is_sorted"];
    assert_ne!(
        sorted, prelude_sorted,
        "recursive identities remain distinct"
    );
    assert!(
        owned.contains(&sorted),
        "Derived owns its checked definition"
    );
    assert!(matches!(
        env.env.lookup(sorted),
        Some(Decl::Transparent { .. })
    ));
    env.elaborate_file(
        "import Data.Collections.Derived (is_sorted as catalog_is_sorted)\n\
         import Core.Logic.Conjunction (and_intro as catalog_and_intro)\n\
         fn ordered (x : Nat) (y : Nat) : Bool = True\n\
         fn reversed (x : Nat) (y : Nat) : Bool = False\n\
         const empty : List Nat = Nil Nat\n\
         const singleton : List Nat = Cons Nat Zero empty\n\
         const pair : List Nat = Cons Nat Zero singleton\n\
         theorem empty_sorted : catalog_is_sorted Nat ordered empty = Proved\n\
         theorem singleton_sorted : catalog_is_sorted Nat ordered singleton = Proved\n\
         theorem pair_sorted : catalog_is_sorted Nat ordered pair = catalog_and_intro Top Top Proved Proved",
    )
    .expect("closed structural sortedness values check against imported identity");
    match env.elaborate_file(
        "theorem unordered_pair : catalog_is_sorted Nat reversed pair =\n\
         catalog_and_intro Top Top Proved Proved",
    ) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            ..
        }) => {}
        other => panic!("false adjacent comparison must refuse: {other:?}"),
    }
    match env.elaborate_file(
        "import Data.Collections.Derived (is_sorted)\n\
         theorem bare_sorted (xs : List Nat) : is_sorted Nat ordered xs = Proved",
    ) {
        Err(ElabError::AmbiguousReference { .. }) => {}
        other => panic!("loose bare sortedness import must refuse ambiguity: {other:?}"),
    }
    for (direction, source) in [
        (
            "prelude to catalog",
            "theorem from_prelude (xs : List Nat) (le : Nat -> Nat -> Bool) \
             (p : is_sorted Nat le xs) : catalog_is_sorted Nat le xs = p",
        ),
        (
            "catalog to prelude",
            "theorem to_prelude (xs : List Nat) (le : Nat -> Nat -> Bool) \
             (p : catalog_is_sorted Nat le xs) : is_sorted Nat le xs = p",
        ),
    ] {
        match env.elaborate_file(source) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }) => {}
            other => panic!("open {direction} conversion must refuse: {other:?}"),
        }
    }
}
