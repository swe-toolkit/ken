//! Let-bound refined recursive calls retain their checked descent and trust boundary.

use ken_elaborator::{ElabEnv, ElabError, ObligationKind};
use ken_kernel::{normalize, Context, Decl, KernelError};
use std::collections::BTreeSet;

fn row(result: &str, step: &str) -> String {
    format!("fn g (n : AnyNat) : {result} = match n {{ Zero |-> Zero ; Suc m |-> {step} }}")
}

const VEC: &str = "data Vec (a : Type) : Nat -> Type where { VNil : Vec a Zero; VCons : (n : Nat) -> a -> Vec a n -> Vec a (Suc n) }";

fn check_row(source: &str, goals: usize, explicit_axioms: usize) -> ElabEnv {
    check_row_with_setup(source, goals, explicit_axioms, false)
}

fn check_row_with_setup(source: &str, goals: usize, explicit_axioms: usize, vec: bool) -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("def AnyNat = { x : Nat | Top }")
        .expect("refined carrier alias");
    if vec {
        env.elaborate_decl(VEC).expect("checked indexed Vec family");
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let result = env
        .elaborate_decl_v1(source)
        .expect("let-bound refined descent must be kernel-admitted");
    assert_eq!(result.auxiliary_ids.len(), 1, "one checked private worker");
    let worker = result.auxiliary_ids[0];
    assert!(matches!(
        env.env.lookup(worker),
        Some(Decl::Transparent { .. })
    ));
    assert!(env.env.is_recursive_transparent(worker));
    assert!(!env.env.is_recursive_transparent(result.def_id));
    assert!(!env.globals.values().any(|id| *id == worker));
    assert_eq!(result.obligations.len(), goals, "source goal count");
    assert!(result.obligations.iter().all(|goal| matches!(
        goal.kind,
        ObligationKind::RefinementIntroduction | ObligationKind::Ensures
    )));
    let open = result
        .obligations
        .iter()
        .filter(|goal| env.is_open_hole(goal.hole_id))
        .map(|goal| goal.hole_id)
        .collect::<BTreeSet<_>>();
    let delta = env
        .env
        .trusted_base()
        .into_iter()
        .filter(|id| !before.contains(id))
        .collect::<BTreeSet<_>>();
    assert!(
        open.is_subset(&delta),
        "open goals must enter the trust ledger"
    );
    let other = delta.difference(&open).copied().collect::<Vec<_>>();
    assert_eq!(
        other.len(),
        explicit_axioms,
        "only explicit source Axiom may add trust beyond open holes"
    );
    assert!(other
        .iter()
        .all(|id| matches!(env.env.lookup(*id), Some(Decl::Opaque { .. }))));
    env
}

// The checked public wrapper calls the emitted private recursive worker.
// Normalize each closed call against an independently written, checked Nat
// constructor. For the match rows, inputs one and two enter different inner
// methods, so type-correct admission alone cannot supply the observation.
fn check_row_with_outputs(source: &str, goals: usize) {
    check_row_with_outputs_and_setup(source, goals, false, ["", "", ""]);
}

fn check_row_with_outputs_and_setup(source: &str, goals: usize, vec: bool, suffixes: [&str; 3]) {
    let mut env = check_row_with_setup(source, goals, 0, vec);
    let trust = env.env.trusted_base();
    for (index, (input, expected)) in [
        ("Zero", "Zero"),
        ("Suc Zero", "Suc Zero"),
        ("Suc (Suc Zero)", "Suc (Suc Zero)"),
    ]
    .into_iter()
    .enumerate()
    {
        let suffix = suffixes[index];
        let observed_id = env
            .elaborate_decl(&format!(
                "const observed_{index} : Nat = g ({input}, Proved){suffix}"
            ))
            .unwrap_or_else(|error| panic!("checked call g ({input}, Proved){suffix}: {error:?}"));
        let expected_id = env
            .elaborate_decl(&format!("const expected_{index} : Nat = {expected}"))
            .unwrap_or_else(|error| panic!("checked Nat constructor {expected}: {error:?}"));
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
            "emitted recursive result for g ({input}, Proved)"
        );
    }
    assert_eq!(env.env.trusted_base(), trust, "closed calls add no trust");
}

fn expect_k0014(source: &str) {
    expect_k0014_with_setup(source, false);
}

fn expect_k0014_with_setup(source: &str, vec: bool) {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl("def AnyNat = { x : Nat | Top }")
        .expect("refined carrier alias");
    if vec {
        env.elaborate_decl(VEC).expect("checked indexed Vec family");
    }
    let error = env
        .elaborate_decl_v1(source)
        .expect_err("nondecreasing or unrelated pair must remain refused");
    assert!(
        matches!(error, ElabError::KernelRejected {
            error: KernelError::NotTerminating(ref msg), ..
        } if msg == "SCT: idempotent self-loop has no strictly-decreasing parameter"),
        "expected K0014, got {error:?}"
    );
}

const RESULT: &str = "{ y : Nat | Equal Nat y n }";
const ENSURES: &str = "Nat ensures Equal Nat result n";

fn indexed_row(parameters: &str, step: &str) -> String {
    format!(
        "fn g (n : AnyNat){parameters} : {RESULT} = match n {{ Zero |-> Zero ; Suc m |-> {step} }}"
    )
}

const VEC_VALUE: &str = "let v : Vec Nat (Suc Zero) = VCons Nat Zero Zero (VNil Nat) in ";
const INDEXED_PARAMS: &str = " (j : Nat) (v : Vec Nat j)";
const INDEXED_PROOF_PARAMS: &str = " (j : Nat) (v : Vec Nat j) (e : Equal Nat j j)";

/// Promise class: durable invariant (spec 17 §4, 39 §2, 64 §1).
/// MEASURED: an indexed match returning a descending call through a let
/// has a kernel-checked private worker, pinned goal count, closed Nat outputs,
/// and no new trust outside its open obligations. CLAIMED: both methods of an
/// applied eliminator grant their common returned-call argument authority.
/// THE GAP: the first row skips a checked absurd branch; mixed, nested, and
/// non-decreasing live branches are separately refused below.
#[test]
fn indexed_a2_impossible_method_does_not_erase_descent() {
    check_row_with_outputs_and_setup(
        &indexed_row(
            "",
            &format!("{VEC_VALUE}let r = (match v {{ VCons i a xs |-> g m }}) in Suc r"),
        ),
        3,
        true,
        ["", "", ""],
    );
}

#[test]
fn indexed_c0_both_methods_return_descending_call() {
    check_row_with_outputs_and_setup(
        &indexed_row(
            INDEXED_PARAMS,
            "let r = (match v { VNil |-> g m j v ; VCons i a xs |-> g m j v }) in Suc r",
        ),
        4,
        true,
        [
            " Zero (VNil Nat)",
            " Zero (VNil Nat)",
            " (Suc Zero) (VCons Nat Zero Zero (VNil Nat))",
        ],
    );
}

#[test]
fn indexed_c2_proof_parameter_keeps_descending_call() {
    check_row_with_outputs_and_setup(
        &indexed_row(
            INDEXED_PROOF_PARAMS,
            "let r = (match v { VNil |-> g m j v e ; VCons i a xs |-> g m j v e }) in Suc r",
        ),
        4,
        true,
        [
            " Zero (VNil Nat) Proved",
            " Zero (VNil Nat) Proved",
            " (Suc Zero) (VCons Nat Zero Zero (VNil Nat)) Proved",
        ],
    );
}

/// Promise class: durable invariant (spec 17 §4, 39 §2, 64 §1).
/// MEASURED: four adjacent previously accepted forms keep exact goal counts,
/// checked workers, and trust deltas. CLAIMED: the applied-eliminator route
/// does not change plain calls or an inline match. THE GAP: a new nested
/// call inside that match is guarded by the typed K0014 refusals below.
#[test]
fn indexed_a1_inline_match_control() {
    check_row_with_setup(
        &indexed_row(
            "",
            &format!("{VEC_VALUE}Suc (match v {{ VCons i a xs |-> g m }})"),
        ),
        3,
        0,
        true,
    );
}

#[test]
fn indexed_c1_inline_match_with_proof_control() {
    check_row_with_setup(
        &indexed_row(
            INDEXED_PROOF_PARAMS,
            "Suc (match v { VNil |-> g m j v e ; VCons i a xs |-> g m j v e })",
        ),
        4,
        0,
        true,
    );
}

#[test]
fn indexed_c3_plain_let_call_control() {
    check_row_with_setup(
        &indexed_row(INDEXED_PROOF_PARAMS, "let r = g m j v e in Suc r"),
        3,
        0,
        true,
    );
}

#[test]
fn indexed_a0_plain_call_after_vec_let_control() {
    check_row_with_setup(
        &indexed_row("", &format!("{VEC_VALUE}let r = g m in Suc r")),
        3,
        0,
        true,
    );
}

/// Promise class: durable invariant (spec 17 §4 and §4.1).
/// MEASURED: the kernel returns exact NotTerminating/K0014 for each
/// nondecreasing, nested, or branch-unequal call after an indexed match.
/// CLAIMED: the returned-call intersection cannot manufacture structural
/// descent or grant authority from an impossible method alone. THE GAP:
/// these four independent counterexamples do not enumerate every family.
#[test]
fn indexed_mixed_nested_method_refused_k0014() {
    expect_k0014_with_setup(
        &indexed_row(
            INDEXED_PARAMS,
            "let r = (match v { VNil |-> g m j v ; VCons i a xs |-> g (g m j v) j v }) in Suc r",
        ),
        true,
    );
}

#[test]
fn indexed_absurd_and_nested_live_method_refused_k0014() {
    expect_k0014_with_setup(
        &indexed_row(
            "",
            &format!("{VEC_VALUE}let r = (match v {{ VCons i a xs |-> g (g m) }}) in Suc r"),
        ),
        true,
    );
}

#[test]
fn indexed_unchanged_argument_in_both_methods_refused_k0014() {
    expect_k0014_with_setup(
        &indexed_row(
            INDEXED_PARAMS,
            "let r = (match v { VNil |-> g n j v ; VCons i a xs |-> g n j v }) in Suc r",
        ),
        true,
    );
}

#[test]
fn indexed_mixed_call_and_noncall_refused_k0014() {
    expect_k0014_with_setup(
        &indexed_row(
            INDEXED_PARAMS,
            "let r = (match v { VNil |-> g m j v ; VCons i a xs |-> Zero }) in Suc r",
        ),
        true,
    );
}

/// Promise class: durable invariant (spec 17 §4; spec 39 §3; spec 64 §1).
/// MEASURED: each let-bound descent produces a kernel-checked private worker,
/// exact goal count and ledger delta. CLAIMED: carrying the call's argument
/// authority through the entire let scope restores descent without trust
/// laundering. THE GAP: these are the five named consumer shapes; the K0014
/// controls below guard unrelated recursive arguments and literal pairs.
#[test]
fn l1_simple_let_bound_recursive_call() {
    check_row(&row(RESULT, "let r = g m in Suc r"), 3, 0);
}

#[test]
fn l2_ensures_let_bound_recursive_call() {
    check_row(&row(ENSURES, "let r = g m in Suc r"), 3, 0);
}

#[test]
fn l3_let_bound_return_of_recursive_call() {
    check_row(&row(RESULT, "let r = g m in r"), 3, 0);
}

#[test]
fn ll_nested_let_inherits_call_argument() {
    check_row(&row(RESULT, "let r = g m in let s = r in Suc s"), 3, 0);
}

#[test]
fn lp_let_of_suc_inherits_call_argument() {
    check_row(&row(RESULT, "let r = g m in let q = Suc r in q"), 3, 0);
}

/// Promise class: durable invariant (spec 17 §4; spec 39 §3; spec 64 §1).
/// MEASURED: each nested-let value yields one kernel-checked recursive worker,
/// three goals, no trust beyond open holes, and the expected Nat results at
/// zero, one and two. CLAIMED: an inner let returning the saturated call
/// preserves the outer result as well as structural descent. THE GAP: match,
/// if and non-tail calls have independent rows below.
#[test]
fn rr2_nested_let_return_result() {
    check_row_with_outputs(&row(RESULT, "let r = (let s = g m in s) in Suc r"), 3);
}

#[test]
fn rr2_nested_let_return_ensures() {
    check_row_with_outputs(&row(ENSURES, "let r = (let s = g m in s) in Suc r"), 3);
}

/// Promise class: durable invariant (spec 17 §4; spec 39 §3; spec 64 §1).
/// MEASURED: checked match/if calls return the expected Nat constructors at
/// zero, one and two, including both nested match methods, with four goals
/// and no trust beyond open holes. CLAIMED: method binders, motive and methods
/// inherit only the arguments common to all arms without changing results.
/// THE GAP: unequal/absent calls refuse earlier with E0083, outside this
/// adapter; nondecreasing calls have independent K0014 rows.
#[test]
fn rr2_match_both_methods_return_call() {
    check_row_with_outputs(
        &row(
            RESULT,
            "let r = (match m { Zero |-> g m ; Suc k |-> g m }) in Suc r",
        ),
        4,
    );
}

#[test]
fn rr2_match_ensures_both_methods_return_call() {
    check_row_with_outputs(
        &row(
            ENSURES,
            "let r = (match m { Zero |-> g m ; Suc k |-> g m }) in Suc r",
        ),
        4,
    );
}

#[test]
fn rr2_if_both_methods_return_call() {
    check_row_with_outputs(
        &row(RESULT, "let r = (if True then g m else g m) in Suc r"),
        4,
    );
}

#[test]
fn rr2_arm_local_let_returns_call() {
    check_row_with_outputs(
        &row(
            RESULT,
            "let r = (match m { Zero |-> let s = g m in s ; Suc k |-> g m }) in Suc r",
        ),
        4,
    );
}

/// Promise class: durable invariant (spec 17 §4; spec 64 §1).
/// MEASURED: these three and the existing `ll_nested_let_inherits_call_argument`
/// keep their checked worker, original goal counts and trust accounting.
/// CLAIMED: only returned saturated calls add authority; a non-tail Suc
/// wrapper is not itself a returned call. THE GAP: these fixed neighbours do
/// not enumerate every possible eliminator family.
#[test]
fn rr2_control_suc_of_match() {
    check_row(
        &row(RESULT, "Suc (match m { Zero |-> g m ; Suc k |-> g m })"),
        4,
        0,
    );
}

#[test]
fn rr2_control_inner_suc_result() {
    check_row(&row(RESULT, "let r = (let s = g m in Suc s) in r"), 3, 0);
}

#[test]
fn rr2_control_suc_of_inner_let() {
    check_row(&row(RESULT, "let r = Suc (let s = g m in s) in r"), 3, 0);
}

/// Promise class: durable invariant (spec 17 §4; spec 64 §1).
/// MEASURED: source pair as the call argument admits, with exactly two goals;
/// trust grows only by source Axiom and open holes. CLAIMED: a call argument's
/// source origin does not disqualify the same in-flight result-ascription rule.
/// THE GAP: unrelated source pairs in the same scope are refused below.
#[test]
fn call_argument_source_pair_admits_without_hidden_trust() {
    check_row(&row(RESULT, "let r = g (m, Axiom) in Suc r"), 2, 1);
}

/// Promise class: durable invariant (spec 17 §4; spec 64 §1).
/// MEASURED: old ascribed-let and plain Nat controls preserve goals and checked
/// worker/trust behavior. CLAIMED: new let authority changes neither old
/// accepted admission route. THE GAP: four additional dependent-result rows
/// remain in lang_refined_parameter_dependent_result.rs and run separately.
#[test]
fn ascribed_let_control_stays_accepted() {
    check_row(&row(RESULT, "let r : Nat = g m in Suc r"), 3, 0);
}

#[test]
fn plain_nat_control_stays_accepted() {
    check_row(&row("Nat", "let r = g m in Suc r"), 1, 0);
}

/// Promise class: durable invariant (spec 17 §4; spec 64 §1).
/// MEASURED: the six tests below each report exact SCT K0014 for a distinct
/// nondecreasing recursive argument. CLAIMED: granting authority to the
/// let-bound result cannot manufacture structural descent. THE GAP: the
/// pair-in-scope negatives are pinned separately below.
#[test]
fn nested_g_of_g_m_remains_k0014() {
    expect_k0014(&row(RESULT, "g (g m)"));
}

#[test]
fn nested_g_of_suc_g_m_remains_k0014() {
    expect_k0014(&row(RESULT, "g (Suc (g m))"));
}

#[test]
fn unchanged_g_n_remains_k0014() {
    expect_k0014(&row(RESULT, "g n"));
}

#[test]
fn let_bound_nested_g_of_g_m_remains_k0014() {
    expect_k0014(&row(RESULT, "let r = g (g m) in Suc r"));
}

#[test]
fn let_bound_nested_g_of_suc_g_m_remains_k0014() {
    expect_k0014(&row(RESULT, "let r = g (Suc (g m)) in Suc r"));
}

#[test]
fn let_bound_unchanged_g_n_remains_k0014() {
    expect_k0014(&row(RESULT, "let r = g n in Suc r"));
}

/// Promise class: durable invariant (spec 17 §4; spec 64 §1).
/// MEASURED: the same nested-let return path with a nondecreasing saturated
/// argument still reaches exact SCT K0014. CLAIMED: returned-call authority
/// cannot license nondecreasing recursion. THE GAP: distinct match-method
/// and unrelated-pair refusals remain independent controls.
#[test]
fn rr2_nested_let_return_of_nested_call_remains_k0014() {
    expect_k0014(&row(RESULT, "let r = (let s = g (g m) in s) in Suc r"));
}

/// Promise class: durable invariant (spec 17 §4; spec 64 §1).
/// MEASURED: when both match methods return the same nondecreasing call,
/// the kernel's SCT refusal remains exact K0014. CLAIMED: intersection
/// cannot manufacture descent from a common but wrong argument. THE GAP:
/// differing or absent calls refuse earlier with E0083 outside this adapter.
#[test]
fn rr2_both_methods_nested_call_remains_k0014() {
    expect_k0014(&row(
        RESULT,
        "let r = (match m { Zero |-> g (g m) ; Suc k |-> g (g m) }) in Suc r",
    ));
}

#[test]
fn rr2_both_methods_unchanged_argument_remains_k0014() {
    expect_k0014(&row(
        RESULT,
        "let r = (match m { Zero |-> g n ; Suc k |-> g n }) in Suc r",
    ));
}

/// Promise class: durable invariant (spec 17 §4; spec 64 §1).
/// MEASURED: both tests below report exact K0014 for literal-pair
/// projections not equal to the saturated call's refined argument, in the
/// same let-of-call scope. CLAIMED: a lexical let alone cannot authorize
/// arbitrary projection redexes. THE GAP: matching the call's real argument
/// is separately accepted above.
#[test]
fn unrelated_m_pair_projection_in_let_scope_remains_k0014() {
    expect_k0014(&row(
        RESULT,
        "let r = g m in let q = ((m, Axiom) : AnyNat).1 in Suc r",
    ));
}

#[test]
fn unrelated_suc_m_pair_projection_in_let_scope_remains_k0014() {
    expect_k0014(&row(
        RESULT,
        "let r = g m in let q = ((Suc m, Axiom) : AnyNat).1 in Suc r",
    ));
}
