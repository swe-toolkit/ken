//! A duplicating branch-goal argument must not make checking exponential.
//! Promise class: durable cost invariant at the debug test profile.
//! Spec 34 §3.5, 39 §2, and Architect D0 evt_1q0gv51yjdkq4.

use std::time::{Duration, Instant};

use ken_elaborator::ElabEnv;
use ken_kernel::{Decl, Term};

const DEPTH: usize = 16;
const MAX_STORED_NODES: usize = 64 * DEPTH * DEPTH;
const ELABORATION_BUDGET: Duration = Duration::from_secs(1);

fn stored_nodes(term: &Term) -> usize {
    1 + term.children().into_iter().map(stored_nodes).sum::<usize>()
}

fn branch_goal_source(argument: &str) -> String {
    let mut nested = String::from("x");
    for _ in 0..DEPTH {
        nested = format!("g ({nested})");
    }
    format!(
        "fn lmap (g : Nat → Nat) (xs : List Nat) : List Nat =
  match xs {{ Nil ↦ Nil Nat; Cons y yt ↦ Cons Nat (g y) (lmap g yt) }}
fn mapdup (g : Nat → Nat) (xs : List Nat) : List Nat = lmap (λx. {nested}) xs
fn mkl (f : Nat → Nat → Nat) (xs : List Nat) : List Nat = mapdup ({argument}) xs
theorem blow (f : Nat → Nat → Nat) (xs : List Nat) (n : Nat) : Equal (List Nat) (mkl f xs) (mkl f xs) =
  match n {{ Zero ↦ Refl; Suc m ↦ Refl }}
"
    )
}

fn check_cost(argument: &str, must_accept: bool) {
    let mut env = ElabEnv::new().expect("prelude");
    let trust = env.env.trusted_base();
    let start = Instant::now();
    let result = env.elaborate_file(&branch_goal_source(argument));
    let elapsed = start.elapsed(); // Exclude prelude and stored-term inspection.
    assert_eq!(env.env.trusted_base(), trust, "no new trust for {argument}");

    if must_accept {
        assert!(result.is_ok(), "linear control must check: {result:?}");
    }
    let nodes = if result.is_ok() {
        let id = *env.globals.get("blow").expect("checked blow declaration");
        let Some(Decl::Transparent { ty, body, .. }) = env.env.lookup(id) else {
            panic!("blow must be kernel-recorded as a transparent declaration");
        };
        Some(stored_nodes(ty) + stored_nodes(body))
    } else {
        None
    };
    eprintln!("{argument}: elapsed={elapsed:?}, stored_nodes={nodes:?}");
    assert!(
        elapsed < ELABORATION_BUDGET && nodes.map_or(true, |count| count <= MAX_STORED_NODES),
        "{argument}: elaboration took {elapsed:?} (budget {ELABORATION_BUDGET:?}), stored nodes {nodes:?} (bound {MAX_STORED_NODES})"
    );
}

/// MEASURED: the n=16 duplicating repro completes within the debug-profile
/// budget and, if accepted, stores no more than 64·n² checked-core nodes;
/// the otherwise-identical linear argument checks within both bounds.
/// CLAIMED: branch-goal contraction does not make this small source blow up.
/// THE GAP: the budget measures this family, not arbitrary source programs;
/// the positive control excludes a change that merely refuses both forms.
#[test]
fn duplicating_branch_goal_stays_bounded_and_linear_control_checks() {
    check_cost("λx. f x x", false);
    check_cost("λx. f x Zero", true);
}
