//! Nested indexed arms read enclosing source names in the refined context.
//! Promise class: durable typed-admission/refusal invariant (spec 34 §3.3).
//! Architect AC-0: evt_6j13z4tj96h66.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{normalize, Context, KernelError};

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
    "const v1 : Vec Nat (Suc Zero) = VCons Nat Zero (Suc (Suc Zero)) (VNil Nat)",
    "const v2 : Vec Nat (Suc (Suc Zero)) = VCons Nat (Suc Zero) (Suc (Suc (Suc Zero))) v1",
    "const v3 : Vec Nat (Suc (Suc (Suc Zero))) = VCons Nat (Suc (Suc Zero)) (Suc Zero) v2",
    "const w1 : Vec Nat (Suc Zero) = VCons Nat Zero (Suc Zero) (VNil Nat)",
    "const w2 : Vec Nat (Suc (Suc Zero)) = VCons Nat (Suc Zero) Zero w1",
    "const w3 : Vec Nat (Suc (Suc (Suc Zero))) = VCons Nat (Suc (Suc Zero)) Zero w2",
];

// Independent closed constructor values, not values calculated by p3/d3.
const V1_RESULT: &str = "VCons Nat Zero (Suc (Suc Zero)) (VNil Nat)";
const V2_RESULT: &str =
    "VCons Nat (Suc Zero) (Suc (Suc (Suc Zero))) (VCons Nat Zero (Suc (Suc Zero)) (VNil Nat))";
const V3_RESULT: &str = "VCons Nat (Suc (Suc Zero)) (Suc Zero) (VCons Nat (Suc Zero) (Suc (Suc (Suc Zero))) (VCons Nat Zero (Suc (Suc Zero)) (VNil Nat)))";
const REPL1_RESULT: &str = "VCons Nat Zero Zero (VNil Nat)";
const REPL2_RESULT: &str = "VCons Nat (Suc Zero) Zero (VCons Nat Zero Zero (VNil Nat))";

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

// Each closed call and independent constructor expectation passes through
// elaboration/kernel checking before their emitted core bodies are normalized.
// In particular, acceptance of a well-typed but wrong constant result cannot
// stand in for preservation of the source branch's value.
fn accepts(name: &str, source: &str, cases: &[(&str, &str, &str)]) {
    let mut env = env();
    let trust = env.env.trusted_base();
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("expected admission: {source}: {error:?}"));
    assert!(
        !cases.is_empty(),
        "a positive row must observe emitted output"
    );
    for (index, &(args, ty, expected)) in cases.iter().enumerate() {
        let observed_id = env
            .elaborate_decl(&format!("const observed_{index} : {ty} = {name} {args}"))
            .unwrap_or_else(|error| panic!("checked call {name} {args}: {error:?}"));
        let expected_id = env
            .elaborate_decl(&format!("const expected_{index} : {ty} = {expected}"))
            .unwrap_or_else(|error| panic!("checked expected {expected}: {error:?}"));
        let normal = |id| {
            let body = env
                .env
                .transparent_body(id)
                .expect("checked transparent const")
                .1;
            normalize(&env.env, &Context::new(), &body)
        };
        assert_eq!(
            normal(observed_id),
            normal(expected_id),
            "emitted value for {name} {args} must match {expected}"
        );
    }
    assert_eq!(
        env.env.trusted_base(),
        trust,
        "source and calls add no trust"
    );
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

/// MEASURED: p3/p3b kernel-check, and their closed nil and nonempty calls
/// normalize to independent Nat counts and nonzero vector constructors.
/// CLAIMED: the enclosing coordinate agrees with the nested scope without
/// changing the source branch's result. THE GAP: depth-three composition and
/// wrong-index refusal are separate rows.
#[test]
fn p3_outer_index_in_inner_nat_result() {
    on_elaboration_stack(|| {
        accepts(
            "p3",
            &pair("p3", "Nat", "Zero", "vlen n w", "vlen n w", true),
            &[
                ("Zero (VNil Nat) (VNil Nat)", "Nat", "Zero"),
                ("(Suc Zero) v1 w1", "Nat", "Suc Zero"),
                ("(Suc (Suc Zero)) v2 w2", "Nat", "Suc (Suc Zero)"),
            ],
        );
    });
}

#[test]
fn p3b_outer_vector_in_inner_dependent_result() {
    on_elaboration_stack(|| {
        accepts(
            "p3b",
            &pair("p3b", "Vec Nat n", "VNil Nat", "v", "v", false),
            &[
                ("Zero (VNil Nat)", "Vec Nat Zero", "VNil Nat"),
                ("(Suc Zero) v1", "Vec Nat (Suc Zero)", V1_RESULT),
                ("(Suc (Suc Zero)) v2", "Vec Nat (Suc (Suc Zero))", V2_RESULT),
            ],
        );
    });
}

/// MEASURED: d3/d3b kernel-check and normalize independently at all four
/// depths, including both branches of the third nested match, without trust.
/// CLAIMED: installation rebasing composes beyond a single child while
/// preserving emitted values. THE GAP: untested nesting beyond depth three.
#[test]
fn d3_composes_three_levels_for_nat_result() {
    on_elaboration_stack(|| {
        accepts(
            "d3",
            &triple(
                "d3", "Nat", "Zero", "vlen n w", "vlen n w", "vlen n w", true,
            ),
            &[
                ("Zero (VNil Nat) (VNil Nat)", "Nat", "Zero"),
                ("(Suc Zero) v1 w1", "Nat", "Suc Zero"),
                ("(Suc (Suc Zero)) v2 w2", "Nat", "Suc (Suc Zero)"),
                (
                    "(Suc (Suc (Suc Zero))) v3 w3",
                    "Nat",
                    "Suc (Suc (Suc Zero))",
                ),
            ],
        );
    });
}

#[test]
fn d3b_composes_three_levels_for_dependent_result() {
    on_elaboration_stack(|| {
        accepts(
            "d3b",
            &triple("d3b", "Vec Nat n", "VNil Nat", "v", "v", "v", false),
            &[
                ("Zero (VNil Nat)", "Vec Nat Zero", "VNil Nat"),
                ("(Suc Zero) v1", "Vec Nat (Suc Zero)", V1_RESULT),
                ("(Suc (Suc Zero)) v2", "Vec Nat (Suc (Suc Zero))", V2_RESULT),
                (
                    "(Suc (Suc (Suc Zero))) v3",
                    "Vec Nat (Suc (Suc (Suc Zero)))",
                    V3_RESULT,
                ),
            ],
        );
    });
}

/// MEASURED: checked closed calls of both neighbouring routes return the
/// expected nonempty constructors or lengths with unchanged trust. CLAIMED:
/// rebasing does not change the existing constructor-coordinate or
/// self-scrutinee semantics. THE GAP: solved-index is a separate binary.
#[test]
fn p3c_explicit_constructor_stays_accepted() {
    on_elaboration_stack(|| {
        accepts(
            "p3c",
            &pair(
                "p3c",
                "Vec Nat n",
                "VNil Nat",
                "VCons Nat m a xs",
                "VCons Nat m a xs",
                false,
            ),
            &[
                ("Zero (VNil Nat)", "Vec Nat Zero", "VNil Nat"),
                ("(Suc Zero) v1", "Vec Nat (Suc Zero)", V1_RESULT),
                ("(Suc (Suc Zero)) v2", "Vec Nat (Suc (Suc Zero))", V2_RESULT),
            ],
        );
    });
}

#[test]
fn p3e_inner_self_scrutinee_stays_accepted() {
    on_elaboration_stack(|| {
        accepts(
            "p3e",
            &pair("p3e", "Nat", "Zero", "vlen n v", "vlen n v", false),
            &[
                ("Zero (VNil Nat)", "Nat", "Zero"),
                ("(Suc Zero) v1", "Nat", "Suc Zero"),
                ("(Suc (Suc Zero)) v2", "Nat", "Suc (Suc Zero)"),
            ],
        );
    });
}

/// MEASURED: repl n at goal Vec Nat n kernel-checks, and closed nonzero
/// inputs return independent zero-filled vectors, not their input values.
/// CLAIMED: p3d is an intended newly admitted route (Architect AC-0) without
/// laundering trust or losing its branch result. THE GAP: arbitrary term
/// aliases of the coordinate remain outside this raw-variable test.
#[test]
fn p3d_constructed_vector_now_checks() {
    on_elaboration_stack(|| {
        accepts(
            "p3d",
            &pair("p3d", "Vec Nat n", "VNil Nat", "repl n", "repl n", false),
            &[
                ("Zero (VNil Nat)", "Vec Nat Zero", "VNil Nat"),
                ("(Suc Zero) v1", "Vec Nat (Suc Zero)", REPL1_RESULT),
                (
                    "(Suc (Suc Zero)) v2",
                    "Vec Nat (Suc (Suc Zero))",
                    REPL2_RESULT,
                ),
            ],
        );
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
