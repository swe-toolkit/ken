//! Same-head type-position sorts link argument levels before expensive delta.
//! Promise class: durable sort-linking and cost boundary (spec 12 §3, 39 §2;
//! Architect D0 evt_h497bj8yv4jk). The kernel rechecks every accepted type.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{convert_type, Context, KernelError, Term};
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

/// Promise: non-regression sentinel, not evidence for the gate. MEASURED:
/// K=12 fallback accepts without obligations, and full cost is at most 8×
/// direct kernel conversion of the same T2 towers. CLAIMED: the fallback
/// does not add unbounded elaborator work. THE GAP: kernel conversion also
/// reduces `blow NN`; it is already expensive when the gate is absent.
#[test]
fn nonconverting_spine_fallback_suc_tracks_kernel_cost() {
    let mut env = environment(12);
    let ll = env
        .elaborate_decl_v1("const LL : Type 1 = T2 NN Type")
        .expect("left T2 tower");
    let rr = env
        .elaborate_decl_v1("const RR : Type 1 = T2 (Suc NN) Type")
        .expect("right T2 tower");
    let started_kernel = Instant::now();
    let converted = convert_type(
        &env.env,
        &Context::new(),
        &Term::const_(ll.def_id, vec![]),
        &Term::const_(rr.def_id, vec![]),
    );
    let kernel_elapsed = started_kernel.elapsed();
    assert!(converted, "T2 towers must convert after delta");

    let source =
        "fn k (F : T2 NN Type -> Type) (x : T2 (Suc NN) Type) (o : F x) (n : Int) : Int = n";
    let started_full = Instant::now();
    let result = env
        .elaborate_decl_v1(source)
        .unwrap_or_else(|error| panic!("fallback_suc: {error:?}"));
    let full_elapsed = started_full.elapsed();
    assert!(
        result.obligations.is_empty(),
        "fallback_suc adds no obligations"
    );
    let ratio = full_elapsed.as_secs_f64() / kernel_elapsed.as_secs_f64();
    eprintln!("sort-spine-gate row=fallback_suc depth=12 full={full_elapsed:?} kernel={kernel_elapsed:?} ratio={ratio:.3}");
    assert!(ratio <= 8.0, "fallback_suc: ratio {ratio:.3} > 8");
}

/// MEASURED: a same-constant, duplicating alias nested twenty levels around
/// a bare universe elaborates below 2s even when every Nat argument differs
/// syntactically by a transparent name. CLAIMED: linking same-head spines
/// prevents repeated unfolding throughout the nested domain. THE GAP:
/// acceptance alone is insufficient; removing only the production fast return
/// must make this unchanged row exceed the same threshold.
#[test]
fn duplicating_alias_with_different_nat_spelling_avoids_exponential_whnf() {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("fn D (u : Nat) (a : Type 1) : Type 1 = a -> a")
        .expect("duplicating alias");
    env.elaborate_decl("const z : Nat = Zero")
        .expect("closed alias for Zero");
    const DEPTH: usize = 20;
    let nested =
        |nat: &str| (0..DEPTH).fold(String::from("Type"), |term, _| format!("D {nat} ({term})"));
    let source = format!(
        "fn k (F : {} -> Type) (x : {}) (o : F x) (n : Int) : Int = n",
        nested("Zero"),
        nested("z")
    );
    let started = Instant::now();
    let result = env
        .elaborate_decl_v1(&source)
        .unwrap_or_else(|error| panic!("bare_z: {error:?}"));
    let elapsed = started.elapsed();
    eprintln!("sort-congruence row=bare_z depth={DEPTH} elapsed={elapsed:?}");
    assert!(result.obligations.is_empty(), "bare_z adds no obligations");
    assert!(elapsed < LIMIT, "bare_z: {elapsed:?} >= {LIMIT:?}");
}

const GATE_DEPTH: usize = 20;

fn spine_gate_env(swapped: bool) -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    let declaration = if swapped {
        "fn H (b : Type 1) (a : Bool) : Type 1 = b -> Int"
    } else {
        "fn H (a : Bool) (b : Type 1) : Type 1 = b -> Int"
    };
    env.elaborate_decl(declaration)
        .expect("discarded Bool alias");
    env
}

fn h_tower(depth: usize, swapped: bool, flag: &str, leaf: &str) -> String {
    (0..depth).fold(leaf.to_string(), |term, _| {
        if swapped {
            format!("H ({term}) {flag}")
        } else {
            format!("H {flag} ({term})")
        }
    })
}

fn h_row_at(
    depth: usize,
    swapped: bool,
    lhs: &str,
    rhs: &str,
    left_leaf: &str,
    right_leaf: &str,
) -> String {
    format!(
        "fn k (F : {} -> Type) (x : {}) (o : F x) (n : Int) : Int = n",
        h_tower(depth, swapped, lhs, left_leaf),
        h_tower(depth, swapped, rhs, right_leaf)
    )
}

fn h_row(swapped: bool, lhs: &str, rhs: &str, left_leaf: &str, right_leaf: &str) -> String {
    h_row_at(GATE_DEPTH, swapped, lhs, rhs, left_leaf, right_leaf)
}

/// MEASURED: twenty nested aliases discard a Bool argument that differs on
/// each side; this accepted declaration has no obligations and checks in
/// under 2s. CLAIMED: a failing first spine pair cannot cause a speculative
/// walk of the next, nested pair. THE GAP: neither acceptance nor elapsed time
/// alone establishes the gate; deleting it must cross the 2s boundary.
#[test]
fn discarded_bool_first_pair_avoids_exponential_walk() {
    let mut env = spine_gate_env(false);
    let source = h_row(false, "True", "False", "Type", "Type");
    let started = Instant::now();
    let result = env
        .elaborate_decl_v1(&source)
        .unwrap_or_else(|error| panic!("R1: {error:?}"));
    let elapsed = started.elapsed();
    eprintln!("sort-spine-gate row=R1 depth={GATE_DEPTH} elapsed={elapsed:?}");
    assert!(result.obligations.is_empty(), "R1 adds no obligations");
    assert!(elapsed < LIMIT, "R1: {elapsed:?} >= {LIMIT:?}");
}

/// MEASURED: full elaboration of the failing-last-pair sibling is at most
/// 20× direct kernel conversion of the same towers at K=14 and K=16.
/// CLAIMED: the elaborator's amplification is bounded.
/// THE GAP: an absolute K=20 bound cannot isolate the elaborator, since
/// kernel conversion is already exponential in this swapped shape.
#[test]
fn discarded_bool_last_pair_cost_is_bounded_by_kernel_conversion() {
    let mut ratios = Vec::new();
    for depth in [14, 16] {
        let mut env = spine_gate_env(true);
        let left = h_tower(depth, true, "True", "Type");
        let right = h_tower(depth, true, "False", "Type");
        let ll = env
            .elaborate_decl_v1(&format!("const LL : Type 1 = {left}"))
            .expect("left checked tower");
        let rr = env
            .elaborate_decl_v1(&format!("const RR : Type 1 = {right}"))
            .expect("right checked tower");
        let started_kernel = Instant::now();
        let converted = convert_type(
            &env.env,
            &Context::new(),
            &Term::const_(ll.def_id, vec![]),
            &Term::const_(rr.def_id, vec![]),
        );
        let kernel_elapsed = started_kernel.elapsed();
        assert!(converted, "LL and RR must convert at K={depth}");

        let source = h_row_at(depth, true, "True", "False", "Type", "Type");
        let started_full = Instant::now();
        let result = env
            .elaborate_decl_v1(&source)
            .unwrap_or_else(|error| panic!("R1-swap K={depth}: {error:?}"));
        let full_elapsed = started_full.elapsed();
        assert!(result.obligations.is_empty(), "R1-swap adds no obligations");
        let ratio = full_elapsed.as_secs_f64() / kernel_elapsed.as_secs_f64();
        eprintln!("sort-spine-gate row=R1-swap depth={depth} full={full_elapsed:?} kernel={kernel_elapsed:?} ratio={ratio:.3}");
        ratios.push(ratio);
    }
    for (index, depth) in [14, 16].into_iter().enumerate() {
        let ratio = ratios[index];
        assert!(
            ratio <= 20.0,
            "R1-swap K={depth}: ratio {ratio:.3} > 20; K14={:.3}, K16={:.3}",
            ratios[0],
            ratios[1]
        );
    }
}

/// MEASURED: nonconvertible leaf types still refuse K0004, and the refusal
/// arrives under 2s at K=20. CLAIMED: the gate does not silently admit the
/// mismatched leaf. THE GAP: the same kernel diagnostic is owed on the parent,
/// even though its sort walk reaches it more slowly.
#[test]
fn mismatched_nested_leaf_refuses_without_exponential_walk() {
    let mut env = spine_gate_env(false);
    let source = h_row(false, "True", "True", "Type", "(Type -> Type)");
    let started = Instant::now();
    let result = env.elaborate_decl_v1(&source);
    let elapsed = started.elapsed();
    eprintln!("sort-spine-gate row=R2 depth={GATE_DEPTH} elapsed={elapsed:?}");
    assert!(
        matches!(
            result,
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            })
        ),
        "R2 must refuse K0004: {result:?}"
    );
    assert!(elapsed < LIMIT, "R2: {elapsed:?} >= {LIMIT:?}");
}
