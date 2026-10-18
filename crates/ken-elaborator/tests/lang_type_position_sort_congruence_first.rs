//! Same-head type-position sorts link argument levels before expensive delta.
//! Promise class: durable sort-linking and cost boundary (spec 12 §3, 39 §2;
//! Architect D0 evt_h497bj8yv4jk). The kernel rechecks every accepted type.

use ken_elaborator::ElabEnv;
use std::time::{Duration, Instant};

const DEPTH: usize = 16;
const LIMIT: Duration = Duration::from_secs(2);

fn environment(depth: usize) -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    for declaration in [
        "fn plus (a : Nat) (b : Nat) : Nat = match a { Zero |-> b; Suc p |-> Suc (plus p b) }",
        "fn blow (n : Nat) : Nat = match n { Zero |-> Zero; Suc m |-> plus (blow m) (blow m) }",
        "fn idn (n : Nat) : Nat = n",
        "fn T2 (n : Nat) (a : Type 1) : Type 1 = match blow n { Zero |-> a; Suc m |-> a }",
        "fn D (n : Nat) (a : Type 1) : Type = match blow n { Zero |-> Int; Suc m |-> Int }",
        "const Small : Type 1 = Type 0",
    ] {
        env.elaborate_decl(declaration)
            .unwrap_or_else(|error| panic!("{declaration}: {error:?}"));
    }
    let nat = (0..depth).fold(String::from("Zero"), |term, _| format!("Suc ({term})"));
    env.elaborate_decl(&format!("const NN : Nat = {nat}"))
        .expect("closed Nat depth fixture");
    env
}

fn fast_row(name: &str, source: &str) {
    let mut env = environment(DEPTH);
    let started = Instant::now();
    let result = env
        .elaborate_decl_v1(source)
        .unwrap_or_else(|error| panic!("{name}: {error:?}"));
    let elapsed = started.elapsed();
    eprintln!("sort-congruence row={name} depth={DEPTH} elapsed={elapsed:?}");
    assert!(
        result.obligations.is_empty(),
        "{name}: no obligations expected"
    );
    assert!(elapsed < LIMIT, "{name}: an otherwise convertible same-head sort reduced the exponential alias: {elapsed:?} >= {LIMIT:?}");
}

/// MEASURED: five named same-constant type-position rows accept without an
/// obligation and elaborate below 2s at depth 16; removing only congruence's
/// fast return makes each exceed 2s. CLAIMED: same-head type-position sorts
/// need not reduce the alias to link levels. THE GAP: acceptance and timing do
/// not independently expose links at discarded positions; the checked-core
/// catalog comparison and the design review bound that residual.
#[test]
fn meta_bearing_former_argument_avoids_delta() {
    fast_row(
        "fa_meta",
        "fn k (F : T2 NN Type -> Type) (x : T2 NN Type) (o : F x) (n : Int) : Int = n",
    );
}

#[test]
fn convertible_nonidentical_meta_argument_avoids_delta() {
    fast_row(
        "fa_meta_ne",
        "fn k (F : T2 NN Type -> Type) (x : T2 (idn NN) Type) (o : F x) (n : Int) : Int = n",
    );
}

#[test]
fn discarded_meta_argument_links_without_delta_or_obligation() {
    fast_row(
        "discard",
        "fn k (F : D NN Type -> Type) (x : D NN Type) (o : F x) (n : Int) : Int = n",
    );
}

#[test]
fn small_alias_and_meta_argument_avoid_delta() {
    fast_row(
        "small_arg",
        "fn k (F : T2 NN Small -> Type) (x : T2 NN Type) (o : F x) (n : Int) : Int = n",
    );
}

#[test]
fn eq_endpoints_with_meta_bearing_alias_avoid_delta() {
    fast_row(
        "eq_meta",
        "fn k (x y : T2 NN Type) (p : Eq (T2 NN Type) x y) (n : Int) : Int = n",
    );
}

/// A non-convertible argument pair cannot use same-head conversion. At K=4
/// the declaration must still accept with no obligation, like whnf-first;
/// acceptance alone does not establish that the tentative links were restored.
#[test]
fn nonconverting_spine_still_accepts_at_small_depth() {
    let mut env = environment(4);
    let source =
        "fn k (F : T2 NN Type -> Type) (x : T2 (Suc NN) Type) (o : F x) (n : Int) : Int = n";
    let result = env
        .elaborate_decl_v1(source)
        .unwrap_or_else(|error| panic!("restore path: {error:?}"));
    assert!(
        result.obligations.is_empty(),
        "fallback adds no obligations"
    );
}
