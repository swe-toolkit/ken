//! Type-position level links reach transparent aliases nested beneath Pi.
//! Promise class: durable non-cumulative sort behavior (spec 12 §3, 39 §2;
//! Architect n1 ruling evt_1tp17m3dsgqcj).

use ken_elaborator::{ElabEnv, ElabError, ObligationKind};
use ken_kernel::{Decl, KernelError, Level, Term};

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

/// Promise: durable invariant. A fixed Type 0 cannot stand in for Big's
/// Type 1 domain; the kernel rejects this independently of level inference.
#[test]
fn fixed_zero_domain_against_nested_big_stays_refused() {
    let mut env = environment(Some(BIG));
    let error = env
        .elaborate_decl_v1(
            "fn k (g : (Big -> Type) -> Type) (f : Type 0 -> Type) (x : g f) (n : Int) : Int = use5 n",
        )
        .expect_err("a non-cumulative fixed domain must refuse");
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

/// Promise: durable invariant. The bare domain is an inferred universe, not
/// fixed Type 0. The admitted kernel type records its lawful Type 1 solution;
/// acceptance alone could otherwise conceal a dropped constraint.
#[test]
fn bare_nested_big_links_former_domain_to_type_one() {
    let mut env = environment(Some(BIG));
    let source =
        "fn k (g : (Big -> Type) -> Type) (f : Type -> Type) (x : g f) (n : Int) : Int = use5 n";
    let result = env
        .elaborate_decl_v1(source)
        .unwrap_or_else(|error| panic!("{source}: {error:?}"));
    assert_eq!(result.obligations.len(), 1);
    assert!(env.is_open_hole(result.obligations[0].hole_id));
    let Some(Decl::Transparent { ty, .. }) = env.env.lookup(result.def_id) else {
        panic!("k must be a checked transparent definition");
    };
    let Term::Pi(_, after_g) = ty else {
        panic!("k has a g binder: {ty:?}");
    };
    let Term::Pi(f_ty, _) = after_g.as_ref() else {
        panic!("k has an f binder: {after_g:?}");
    };
    let Term::Pi(f_domain, _) = f_ty.as_ref() else {
        panic!("f is a former: {f_ty:?}");
    };
    assert_eq!(
        f_domain.as_ref(),
        &Term::ty(Level::Suc(Box::new(Level::Zero))),
        "f's checked domain must be Type 1, not a defaulted Type 0"
    );
}

// The source is deliberately expensive only if the type-position sort walk
// reduces T NN. The kernel can check these identical or convertible former
// domains without reducing T NN; fa_ne differs only by an identity wrapper.
const TIMING_LIMIT: std::time::Duration = std::time::Duration::from_secs(2);
const TIMING_DEPTH: usize = 16;

fn timed_type_position_row(name: &str, source: &str) {
    let mut env = ElabEnv::new().expect("prelude");
    for declaration in [
        "fn plus (a : Nat) (b : Nat) : Nat = match a { Zero |-> b; Suc p |-> Suc (plus p b) }",
        "fn blow (n : Nat) : Nat = match n { Zero |-> Zero; Suc m |-> plus (blow m) (blow m) }",
        "fn T (n : Nat) : Type = match blow n { Zero |-> Int; Suc m |-> Int }",
        "fn idn (n : Nat) : Nat = n",
    ] {
        env.elaborate_decl(declaration)
            .unwrap_or_else(|error| panic!("{declaration}: {error:?}"));
    }
    let nat = (0..TIMING_DEPTH).fold(String::from("Zero"), |term, _| format!("Suc ({term})"));
    env.elaborate_decl(&format!("const NN : Nat = {nat}"))
        .expect("closed Nat depth fixture");
    let started = std::time::Instant::now();
    let result = env
        .elaborate_decl_v1(source)
        .unwrap_or_else(|error| panic!("{name}: {error:?}"));
    let elapsed = started.elapsed();
    eprintln!("sort-walk timing row={name} depth={TIMING_DEPTH} elapsed={elapsed:?}");
    assert!(
        result.obligations.is_empty(),
        "{name}: no obligations expected"
    );
    assert!(
        elapsed < TIMING_LIMIT,
        "{name}: type-position bookkeeping reduced an inert exponential alias: {elapsed:?} >= {TIMING_LIMIT:?}"
    );
}

/// Promise: durable performance boundary. MEASURED: a closed, repeated alias
/// in the former-argument route checks in under two seconds after the fixture
/// is loaded. CLAIMED: sort-linking does not reduce an inert alias. THE GAP:
/// identity alone is weaker; fa_ne below checks a nonidentical convertible pair.
#[test]
fn inert_former_argument_avoids_exponential_whnf() {
    timed_type_position_row(
        "fa",
        "fn k (F : T NN -> Type) (x : T NN) (o : F x) (n : Int) : Int = n",
    );
}

#[test]
fn inert_equality_endpoints_avoid_exponential_whnf() {
    timed_type_position_row(
        "eq",
        "fn k (x y : T NN) (p : Eq (T NN) x y) (n : Int) : Int = n",
    );
}

#[test]
fn inert_nested_former_avoids_exponential_whnf() {
    timed_type_position_row(
        "nested",
        "fn k (G : (T NN -> Type) -> Type) (F : T NN -> Type) (o : G F) (n : Int) : Int = n",
    );
}

#[test]
fn inert_convertible_nonidentical_former_avoids_exponential_whnf() {
    timed_type_position_row(
        "fa_ne",
        "fn k (F : T NN -> Type) (x : T (idn NN)) (o : F x) (n : Int) : Int = n",
    );
}
