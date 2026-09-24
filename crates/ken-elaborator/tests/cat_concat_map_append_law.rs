//! Checked private distributivity law in the real Derived catalog package.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, Level, Term};

const MODULE: &str = "Data.Collections.Derived";

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn load_derived() -> ElabEnv {
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("Derived must roots-load with the checked distributivity law");
    env
}

fn applied(head: Term, args: impl IntoIterator<Item = Term>) -> Term {
    args.into_iter().fold(head, Term::app)
}

/// Promise class: durable invariant.
///
/// MEASURED: a roots-loaded, kernel-checked private theorem has the exact raw
/// five-binder type, canonical GlobalIds and de Bruijn-bound endpoints.
/// CLAIMED: the theorem proves `concat_map` distributes over `list_append` on
/// arbitrary input, rather than a reflexive or narrower substitute.
/// THE GAP: exact raw type plus a transparent checked proof closes this claim;
/// this does not claim an axiom-free imported provider closure.
#[test]
fn concat_map_append_has_the_exact_raw_checked_proposition() {
    let env = load_derived();
    let theorem = env.globals[&format!("{MODULE}.concat_map_append")];
    let Decl::Transparent { ty, .. } = env.env.lookup(theorem).expect("law must be loaded") else {
        panic!("concat_map_append must be a kernel-checked proof, not an assumption");
    };

    let global = |name: &str| Term::const_(env.globals[name], vec![]);
    let list = |item: Term| applied(Term::indformer(env.globals["List"], vec![]), [item]);
    let append = |item: Term, xs: Term, ys: Term| {
        applied(global(&format!("{MODULE}.list_append")), [item, xs, ys])
    };
    let concat = |a: Term, b: Term, f: Term, xs: Term| {
        applied(global(&format!("{MODULE}.concat_map")), [a, b, f, xs])
    };

    // Under five binders, a=4, b=3, f=2, xs=1, ys=0.
    let lhs = concat(
        Term::var(4),
        Term::var(3),
        Term::var(2),
        append(Term::var(4), Term::var(1), Term::var(0)),
    );
    let rhs = append(
        Term::var(3),
        concat(Term::var(4), Term::var(3), Term::var(2), Term::var(1)),
        concat(Term::var(4), Term::var(3), Term::var(2), Term::var(0)),
    );
    let proposition = applied(global("Equal"), [list(Term::var(3)), lhs, rhs]);
    let expected = Term::pi(
        Term::ty(Level::Zero),
        Term::pi(
            Term::ty(Level::Zero),
            Term::pi(
                Term::pi(Term::var(1), list(Term::var(1))),
                Term::pi(
                    list(Term::var(2)),
                    Term::pi(list(Term::var(3)), proposition),
                ),
            ),
        ),
    );
    assert_eq!(
        ty, &expected,
        "concat_map_append's raw checked proposition changed"
    );
}

/// Promise class: durable invariant for visibility and closure trust.
///
/// MEASURED: a real selective import succeeds for public `list_append` but
/// rejects the private law; trust sets are captured at the prelude, provider and
/// Derived stages in the same roots-loaded environment. CLAIMED: the theorem
/// changes neither the public export surface nor the provider-loaded trust
/// ledger. THE GAP: a relative no-growth result does not mean that imported
/// providers themselves are axiom-free; their inherited entries are reported.
#[test]
fn concat_map_append_stays_private_and_adds_no_provider_closure_trust() {
    let mut env = ElabEnv::new().expect("base environment");
    let prelude: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    for provider in [
        "Data.Numeric.Nat.Order",
        "Core.Logic.Compare",
        "Core.Classes.LawfulClasses",
        "Core.Logic.Or",
        "Core.Logic.OrdResult",
        "Core.Logic.Transport",
    ] {
        env.elaborate_module_from_roots(&[catalog_root()], provider)
            .unwrap_or_else(|error| panic!("{provider} must roots-load: {error:?}"));
    }
    let providers: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    env.elaborate_module_from_roots(&[catalog_root()], MODULE)
        .expect("Derived must roots-load through the provider closure");
    let with_derived: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let inherited_axioms: Vec<_> = providers
        .difference(&prelude)
        .filter_map(|id| match env.env.lookup(*id) {
            Some(Decl::Opaque { name, .. }) => Some(name.clone()),
            _ => None,
        })
        .collect();
    eprintln!(
        "Derived closure trust: prelude={} providers={} with_Derived={} inherited_axioms={inherited_axioms:?}",
        prelude.len(),
        providers.len(),
        with_derived.len()
    );
    assert_eq!(
        providers, with_derived,
        "the checked distributivity law must add no trust beyond providers"
    );

    env.elaborate_file(
        "import Data.Collections.Derived (list_append as concat_law_public_control)",
    )
    .expect("positive control: the real public provider must import");
    match env.elaborate_file("import Data.Collections.Derived (concat_map_append)") {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{MODULE}.concat_map_append"));
        }
        Err(other) => panic!("private law rejected for the wrong reason: {other:?}"),
        Ok(_) => panic!("private concat_map_append was unexpectedly published"),
    }
}
