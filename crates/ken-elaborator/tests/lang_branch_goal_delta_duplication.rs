//! A named duplicating helper must not make a dependent branch goal exponential.
//! Promise class: durable debug-profile cost invariant. Spec 34 §3.5 and 39
//! §2.1; Architect BGD D0 evt_2yxvr07dmkd0b.

use std::time::{Duration, Instant};

use ken_elaborator::ElabEnv;
use ken_kernel::{Decl, Term};

const LARGE_DEPTH: usize = 16;
const ELABORATION_BUDGET: Duration = Duration::from_secs(1);

fn stored_nodes(term: &Term) -> usize {
    1 + term.children().into_iter().map(stored_nodes).sum::<usize>()
}

fn branch_goal_source(helper: &str, depth: usize) -> String {
    let mut nested = String::from("x");
    for _ in 0..depth {
        nested = format!("{helper} f ({nested})");
    }
    let definition = match helper {
        "dupf" => "fn dupf (f : Nat → Nat → Nat) (x : Nat) : Nat = f x x",
        "dup2" => "fn dup2 (f : Nat → Nat → Nat) (x : Nat) : Nat = f (Suc x) (Suc (Suc x))",
        "linf" => "fn linf (f : Nat → Nat → Nat) (x : Nat) : Nat = f x Zero",
        _ => panic!("unrecognized fixture helper"),
    };
    format!(
        "{definition}\ntheorem t1 (f : Nat → Nat → Nat) (x : Nat) (n : Nat) : Equal Nat ({nested}) ({nested}) = match n {{ Zero ↦ Refl; Suc m ↦ Refl }}\n"
    )
}

fn check_cost(helper: &str, depth: usize, must_accept: bool) -> Option<usize> {
    let mut env = ElabEnv::new().expect("prelude");
    let trust = env.env.trusted_base();
    let start = Instant::now();
    let result = env.elaborate_file(&branch_goal_source(helper, depth));
    let elapsed = start.elapsed(); // Exclude prelude and stored-term inspection.
    assert_eq!(env.env.trusted_base(), trust, "{helper} must add no trust");
    if must_accept {
        assert!(result.is_ok(), "{helper} n={depth} must check: {result:?}");
    }
    let nodes = if result.is_ok() {
        let id = *env.globals.get("t1").expect("checked t1 declaration");
        let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(id) else {
            panic!("t1 must be kernel-recorded as a transparent declaration");
        };
        Some(stored_nodes(ty) + stored_nodes(body))
    } else {
        None
    };
    let max_nodes = 64 * depth * depth;
    eprintln!("{helper} n={depth}: elapsed={elapsed:?}, stored_nodes={nodes:?}");
    assert!(
        elapsed < ELABORATION_BUDGET && nodes.map_or(true, |count| count <= max_nodes),
        "{helper} n={depth}: elapsed={elapsed:?} (budget {ELABORATION_BUDGET:?}), stored_nodes={nodes:?} (bound {max_nodes})"
    );
    nodes
}

/// MEASURED: named δ-duplication at depth 16 returns in one second, with at
/// most 64·n² stored checked-core nodes if admitted. CLAIMED: this duplicator
/// cannot exhaust the branch-goal simplifier. THE GAP: this measures only the
/// fixture; the cost of a single kernel `whnf` is outside this WP.
#[test]
fn named_duplicate_at_sixteen_stays_bounded() {
    check_cost("dupf", LARGE_DEPTH, false);
}

/// MEASURED: δ duplicates unequal argument trees at depth 16 within the same
/// bounds. CLAIMED: cost does not depend on syntactic equality of the copies.
/// THE GAP: other transparent helper shapes remain unenumerated.
#[test]
fn distinct_duplicate_at_sixteen_stays_bounded() {
    check_cost("dup2", LARGE_DEPTH, false);
}

/// MEASURED: the nonduplicating helper still checks within the shared cost
/// bound. CLAIMED: the budget does not merely refuse every source with `match`.
/// THE GAP: a green linear proof says nothing about duplicating helpers.
#[test]
fn linear_helper_at_sixteen_still_checks() {
    check_cost("linf", LARGE_DEPTH, true);
}

/// MEASURED: below the allowance, duplicating δ stores more than the linear
/// helper on the same source depth. CLAIMED: the budget does not skip all
/// simplification. THE GAP: a relational node count does not establish
/// byte-exact identity for every below-allowance branch goal.
#[test]
fn below_allowance_duplicate_expands_more_than_linear() {
    let duplicate = check_cost("dupf", 8, true).expect("checked duplicate");
    let linear = check_cost("linf", 8, true).expect("checked linear control");
    assert!(
        duplicate > linear,
        "below allowance: {duplicate} <= {linear}"
    );
}
