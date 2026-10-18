//! Type-position sort links reduce transparent aliases and recurse through
//! higher-order domains. Spec 12 §3, 39 §2; Architect evt_438a6j0bwer46.

use ken_elaborator::{ElabEnv, ElabError, ObligationKind};
use ken_kernel::KernelError;

const USE5: &str = "fn use5 (x : {v : Int | Equal Int v 5}) : Int = x";
const SMALL: &str = "const Small : Type 1 = Type 0";
const BIG: &str = "const Big : Type 2 = Type 1";

fn environment(alias: Option<&str>) -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl(USE5).expect("refined call fixture");
    if let Some(alias) = alias {
        env.elaborate_decl(alias).expect("alias fixture");
    }
    env
}

/// Promise: durable invariant. MEASURED: each checked declaration reports
/// exactly one still-open refinement introduction. CLAIMED: type-position
/// sort equations hold through transparent aliases and Pi domains. THE GAP:
/// refusal of a sort mismatch must still be live; the Big rows check that.
#[test]
fn aliases_and_higher_order_domains_link_type_position_sorts() {
    for (name, alias, source) in [
        (
            "a1",
            Some(SMALL),
            "fn k (a : Small) (h : Eq Type a a) (n : Int) : Int = use5 n",
        ),
        (
            "a3",
            Some(SMALL),
            "fn k (a : Small) (b : Type) (h : Eq Type a b) (n : Int) : Int = use5 n",
        ),
        (
            "a5",
            Some(SMALL),
            "fn k (F : Type -> Type) (a : Small) (o : F a) (n : Int) : Int = use5 n",
        ),
        (
            "g3",
            None,
            "fn k (g : (Type -> Type) -> Type) (f : Type -> Type) (x : g f) (n : Int) : Int = use5 n",
        ),
    ] {
        let mut env = environment(alias);
        let before = env.env.trusted_base();
        let result = env
            .elaborate_decl_v1(source)
            .unwrap_or_else(|error| panic!("{name}: {error:?}"));
        assert_eq!(result.obligations.len(), 1, "{name}: one obligation");
        let obligation = &result.obligations[0];
        assert!(
            matches!(obligation.kind, ObligationKind::RefinementIntroduction)
                && env.is_open_hole(obligation.hole_id),
            "{name}: the obligation must remain reported and open"
        );
        let added: Vec<_> = env
            .env
            .trusted_base()
            .into_iter()
            .filter(|id| !before.contains(id))
            .collect();
        assert_eq!(added, vec![obligation.hole_id], "{name}: only the reported hole");
    }
}

/// Promise: durable invariant. A non-cumulative Type 1 cannot be treated as
/// Type 0, even through an alias or a higher-order former.
#[test]
fn higher_sort_aliases_and_explicit_universes_still_refuse() {
    for (name, alias, source) in [
        (
            "big-eq-refl",
            Some(BIG),
            "fn k (a : Big) (h : Eq (Type 0) a a) (n : Int) : Int = use5 n",
        ),
        (
            "big-eq-int",
            Some(BIG),
            "fn k (a : Big) (h : Eq Type a Int) (n : Int) : Int = use5 n",
        ),
        (
            "big-former",
            Some(BIG),
            "fn k (F : Type -> Type) (a : Big) (o : F a) (q : F Int) (n : Int) : Int = use5 n",
        ),
        (
            "explicit-type-1",
            None,
            "fn k (a : Type 1) (h : Eq (Type 0) a a) (n : Int) : Int = use5 n",
        ),
    ] {
        let mut env = environment(alias);
        let error = env
            .elaborate_decl_v1(source)
            .expect_err("non-cumulative sort mismatch must refuse");
        assert!(
            matches!(
                error,
                ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
            ),
            "{name}: {error:?}"
        );
    }
}

/// Promise: durable invariant. Census of the existing expression-position Eq
/// route; the repair is confined to written type positions, not infer_eq.
#[test]
fn expression_position_eq_checks_alias_endpoint_without_new_sort_link() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("const Alias : Type 0 = Int")
        .expect("transparent alias");
    let before = env.env.trusted_base();
    let result = env
        .elaborate_decl_v1("fn k (a : Alias) : Omega = Eq Int a a")
        .expect("infer_eq checks a term through the alias");
    assert!(result.obligations.is_empty());
    assert_eq!(env.env.trusted_base(), before);
}
