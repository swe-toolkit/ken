//! Nested indexed arms read enclosing source names in the refined context.
//! Promise class: durable typed-admission/refusal invariant (spec 34 §3.3).
//! Architect AC-0: evt_6j13z4tj96h66.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::KernelError;

// The adjacent indexed-match suite measured a 1,982,464-byte prelude peak.
// Eight MiB states more than fourfold headroom for these nested methods.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

const FIXTURES: &[&str] = &[
    "data Vec (a : Type) : Nat -> Type where { VNil : Vec a Zero; VCons : (m : Nat) -> a -> Vec a m -> Vec a (Suc m) }",
    "fn vlen (n : Nat) (v : Vec Nat n) : Nat = match v { VNil ↦ Zero; VCons m a xs ↦ Suc (vlen m xs) }",
    "fn repl (n : Nat) : Vec Nat n = match n { Zero ↦ VNil Nat; Suc m ↦ VCons Nat m Zero (repl m) }",
];

fn env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    let trust = env.env.trusted_base();
    for fixture in FIXTURES {
        env.elaborate_decl(fixture)
            .unwrap_or_else(|error| panic!("fixture {fixture}: {error:?}"));
    }
    assert_eq!(env.env.trusted_base(), trust, "fixtures add no trust");
    env
}

fn accepts(source: &str) {
    let mut env = env();
    let trust = env.env.trusted_base();
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("expected admission: {source}: {error:?}"));
    assert_eq!(env.env.trusted_base(), trust, "source adds no trust");
}

fn refuses_wrong_index(source: &str) {
    let mut env = env();
    let trust = env.env.trusted_base();
    let error = env
        .elaborate_decl(source)
        .expect_err("wrong index must refuse");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                ..
            }
        ),
        "wrong index must fail kernel type checking: {error:?}"
    );
    assert_eq!(env.env.trusted_base(), trust, "refusal adds no trust");
}

// p3/d3 carry a sibling w; p3b/d3b/p3d do not. Adding an unused w to the
// latter changes the motive and lets their stale-coordinate baseline pass.
fn pair(name: &str, result: &str, nil: &str, first: &str, second: &str, sibling: bool) -> String {
    let w = if sibling { " (w : Vec Nat n)" } else { "" };
    format!(
        "fn {name} (n : Nat) (v : Vec Nat n){w} : {result} = \
         match v {{ VNil ↦ {nil}; VCons m a xs ↦ match xs {{ \
         VNil ↦ {first}; VCons m2 b ys ↦ {second} }} }}"
    )
}

fn triple(
    name: &str,
    result: &str,
    nil: &str,
    first: &str,
    deep_nil: &str,
    deep_cons: &str,
    sibling: bool,
) -> String {
    pair(
        name,
        result,
        nil,
        first,
        &format!("match ys {{ VNil ↦ {deep_nil}; VCons m3 c zs ↦ {deep_cons} }}"),
        sibling,
    )
}

/// MEASURED: p3/p3b check with the nested VNil arm while `n` denotes the
/// outer constructor index rebased through `m = Zero`; trust is unchanged.
/// CLAIMED: an outer arm coordinate remains current in a nested arm's scope.
/// THE GAP: depth-three composition and wrong-index refusal are separate rows.
#[test]
fn p3_outer_index_in_inner_nat_result() {
    on_elaboration_stack(|| {
        accepts(&pair("p3", "Nat", "Zero", "vlen n w", "vlen n w", true));
    });
}

#[test]
fn p3b_outer_vector_in_inner_dependent_result() {
    on_elaboration_stack(|| {
        accepts(&pair("p3b", "Vec Nat n", "VNil Nat", "v", "v", false));
    });
}

/// MEASURED: in d3/d3b, the third nested arm checks using the twice-rebased
/// enclosing coordinate, with no new trust. CLAIMED: installation rebasing
/// composes beyond a single child. THE GAP: untested nesting beyond depth 3.
#[test]
fn d3_composes_three_levels_for_nat_result() {
    on_elaboration_stack(|| {
        accepts(&triple(
            "d3", "Nat", "Zero", "vlen n w", "vlen n w", "vlen n w", true,
        ));
    });
}

#[test]
fn d3b_composes_three_levels_for_dependent_result() {
    on_elaboration_stack(|| {
        accepts(&triple(
            "d3b",
            "Vec Nat n",
            "VNil Nat",
            "v",
            "v",
            "v",
            false,
        ));
    });
}

/// MEASURED: both already-working neighbour routes stay checked and trusted
/// base stays unchanged. CLAIMED: rebasing does not break the existing
/// constructor coordinate or self-scrutinee route. THE GAP: these are only
/// the two named neighbouring consumers; solved-index is a separate binary.
#[test]
fn p3c_explicit_constructor_stays_accepted() {
    on_elaboration_stack(|| {
        accepts(&pair(
            "p3c",
            "Vec Nat n",
            "VNil Nat",
            "VCons Nat m a xs",
            "VCons Nat m a xs",
            false,
        ));
    });
}

#[test]
fn p3e_inner_self_scrutinee_stays_accepted() {
    on_elaboration_stack(|| {
        accepts(&pair("p3e", "Nat", "Zero", "vlen n v", "vlen n v", false));
    });
}

/// MEASURED: repl n at goal Vec Nat n checks only after the nested coordinate
/// agrees with that goal, with trust unchanged. CLAIMED: p3d is an intended
/// newly admitted route (Architect AC-0), not an incorrectly kept refusal.
/// THE GAP: repl tests source-name rebasing, not arbitrary term aliases.
#[test]
fn p3d_constructed_vector_now_checks() {
    on_elaboration_stack(|| {
        accepts(&pair(
            "p3d",
            "Vec Nat n",
            "VNil Nat",
            "repl n",
            "repl n",
            false,
        ));
    });
}

/// MEASURED: the third arm rejects a predecessor-index xs at a successor-
/// successor goal with exact kernel TypeMismatch and unchanged trust.
/// CLAIMED: rebasing does not manufacture equality for genuinely wrong
/// indices. THE GAP: a single named wrong-index shape, not all mismatches.
#[test]
fn d3x_wrong_deep_index_remains_kernel_refused() {
    on_elaboration_stack(|| {
        refuses_wrong_index(&triple(
            "d3x",
            "Vec Nat n",
            "VNil Nat",
            "v",
            "xs",
            "v",
            false,
        ));
    });
}
