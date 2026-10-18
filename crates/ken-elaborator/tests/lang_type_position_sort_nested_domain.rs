//! Type-position level links reach transparent aliases nested beneath Pi.
//! Promise class: durable non-cumulative sort behavior (spec 12 §3, 39 §2;
//! Architect n1 ruling evt_1tp17m3dsgqcj).

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

fn one_open_obligation(alias: Option<&str>, source: &str) {
    let mut env = environment(alias);
    let before = env.env.trusted_base();
    let result = env
        .elaborate_decl_v1(source)
        .unwrap_or_else(|error| panic!("{source}: {error:?}"));
    assert_eq!(
        result.obligations.len(),
        1,
        "{source}: exactly one obligation"
    );
    let obligation = &result.obligations[0];
    assert!(matches!(
        obligation.kind,
        ObligationKind::RefinementIntroduction
    ));
    assert!(env.is_open_hole(obligation.hole_id));
    let added: Vec<_> = env
        .env
        .trusted_base()
        .into_iter()
        .filter(|id| !before.contains(id))
        .collect();
    assert_eq!(added, vec![obligation.hole_id], "only the reported hole");
}

/// MEASURED: both nested alias former arguments check and retain exactly one
/// open obligation. CLAIMED: whnf reaches a transparent alias beneath one or
/// two Pi domains. THE GAP: an unrelated acceptance would not prove levels
/// were linked; the head-only configuration makes both rows refuse K0004.
#[test]
fn nested_alias_in_former_domain_links_sort() {
    one_open_obligation(
        Some(SMALL),
        "fn k (g : (Small -> Type) -> Type) (f : Type -> Type) (x : g f) (n : Int) : Int = use5 n",
    );
}

#[test]
fn alias_two_former_domains_deep_links_sort() {
    one_open_obligation(
        Some(SMALL),
        "fn k (g : ((Small -> Type) -> Type) -> Type) (f : (Type -> Type) -> Type) (x : g f) (n : Int) : Int = use5 n",
    );
}

/// The nested-unaliased and head-alias routes stay checkable under this walk.
#[test]
fn head_alias_and_unaliased_former_controls_stay_checkable() {
    one_open_obligation(
        None,
        "fn k (g : (Type -> Type) -> Type) (f : Type -> Type) (x : g f) (n : Int) : Int = use5 n",
    );
    one_open_obligation(
        Some(SMALL),
        "fn k (F : Small -> Type) (a : Type) (o : F a) (n : Int) : Int = use5 n",
    );
}

/// A Type 1 alias cannot be linked to the Type 0 domain by conversion.
#[test]
fn nested_big_sort_stays_refused() {
    let mut env = environment(Some(BIG));
    let error = env
        .elaborate_decl_v1(
            "fn k (g : (Big -> Type) -> Type) (f : Type -> Type) (x : g f) (n : Int) : Int = use5 n",
        )
        .expect_err("a non-cumulative nested domain must refuse");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "{error:?}"
    );
}
