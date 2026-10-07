//! The private Derived zip projection laws have exact checked propositions.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, Level, Term};

const MODULE: &str = "Data.Collections.Derived";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_derived() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Function.Combinators",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Core.Logic.Compare",
        "Core.Classes.LawfulClasses",
        "Core.Logic.Or",
        "Core.Logic.OrdResult",
        "Core.Logic.Transport",
        "Data.Collections.List",
    ] {
        env.elaborate_module_from_roots(&[catalog_root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must roots-load: {error:?}"));
    }
    let providers: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned = env
        .elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("Derived must roots-load with its zip projection laws");
    let derived: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(derived, providers, "Derived must add no provider trust");
    (env, owned)
}

fn applied(head: Term, args: impl IntoIterator<Item = Term>) -> Term {
    args.into_iter().fold(head, Term::app)
}

fn assert_projection_proposition(first: bool) {
    let (env, owned) = load_derived();
    let law = if first { "zip_fst" } else { "zip_snd" };
    let id = env.globals[&format!("{MODULE}.{law}")];
    assert!(owned.contains(&id), "{law} must be owned by Derived");
    let Decl::Transparent { ty, .. } = env.env.lookup(id).expect("law must load") else {
        panic!("{law} must be a checked transparent proof, not an assumption");
    };

    let global = |name: &str| Term::const_(env.globals[name], vec![]);
    let list = |element: Term| applied(Term::indformer(env.globals["List"], vec![]), [element]);
    // Under all four binders: a=3, b=2, xs=1, ys=0.
    let a = Term::var(3);
    let b = Term::var(2);
    let xs = Term::var(1);
    let ys = Term::var(0);
    let (element, projection, bound, truncated) = if first {
        (a.clone(), "pair_fst", b.clone(), xs.clone())
    } else {
        (b.clone(), "pair_snd", a.clone(), ys.clone())
    };
    let pairs = applied(global("Pair"), [a.clone(), b.clone()]);
    let zipped = applied(
        global(&format!("{MODULE}.zip")),
        [a.clone(), b.clone(), xs.clone(), ys.clone()],
    );
    let projected = applied(
        global(&format!("{MODULE}.map")),
        [
            pairs,
            element.clone(),
            applied(global(projection), [a, b]),
            zipped,
        ],
    );
    let length_input = if first { ys } else { xs };
    let counted = applied(
        global("Data.Collections.List.length"),
        [bound, length_input],
    );
    let taken = applied(
        global(&format!("{MODULE}.take")),
        [element.clone(), counted, truncated],
    );
    let equality = applied(global("Equal"), [list(element), projected, taken]);
    let expected = Term::pi(
        Term::ty(Level::Zero),
        Term::pi(
            Term::ty(Level::Zero),
            Term::pi(list(Term::var(1)), Term::pi(list(Term::var(1)), equality)),
        ),
    );
    assert_eq!(ty, &expected, "{law}'s raw checked proposition changed");
}

/// Promise class: durable checked-proposition invariant.
/// MEASURED: roots loading checks a Derived-owned transparent proof whose raw
/// four-binder type uses checked map/zip/take and List.length GlobalIds, with
/// de Bruijn-bound first-projection endpoints. CLAIMED: for arbitrary lists,
/// projecting zip's first components equals the first list truncated by the
/// second's length. THE GAP: this pins the statement and kernel-checked body,
/// not runtime execution or the trust of inherited providers.
#[test]
fn zip_fst_has_the_exact_raw_checked_proposition() {
    assert_projection_proposition(true);
}

/// Promise class: durable checked-proposition invariant.
/// MEASURED: the independent raw type assertion checks the second projection
/// and its opposite-list bound by checked identity and de Bruijn variable.
/// CLAIMED: zip's second components equal the second list truncated by the
/// first's length for every input. THE GAP: as above, imported trust is not
/// claimed empty, only unchanged by Derived.
#[test]
fn zip_snd_has_the_exact_raw_checked_proposition() {
    assert_projection_proposition(false);
}

/// Promise class: durable visibility invariant.
/// MEASURED: a public operation selectively imports, both private law imports
/// are refused by their exact unresolved name, and Derived's checked closure
/// has the same trust-identity set as its already-loaded providers. CLAIMED:
/// the laws neither export new names nor introduce new trust. THE GAP: the
/// provider set may itself inherit assumptions; it is not asserted empty.
#[test]
fn zip_projection_laws_stay_private_and_add_no_trust() {
    let (mut env, _) = load_derived();
    env.elaborate_file("import Data.Collections.Derived (map as zip_projection_public_control)")
        .expect("positive control: public map must selectively import");
    for law in ["zip_fst", "zip_snd"] {
        match env.elaborate_file(&format!("import {MODULE} ({law})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{MODULE}.{law}"));
            }
            Err(other) => panic!("private {law} import rejected for the wrong reason: {other:?}"),
            Ok(_) => panic!("private {law} was unexpectedly published"),
        }
    }
}
