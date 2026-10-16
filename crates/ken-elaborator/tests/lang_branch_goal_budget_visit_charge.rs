//! A large nonduplicating closed list must not exhaust a dependent branch goal.
//! Promise class: durable elaboration invariant. Spec 34 §3.5 and 39 §2.1;
//! Architect's visit-charge ruling evt_6h6zc63svyzha.

use ken_elaborator::ElabEnv;

// K=160 overflows at 8 MiB and passes at 16 MiB on this build. Provision
// 32 MiB, at least 16 MiB above that observed upper bound; explicit Builder
// stack size prevents ambient RUST_MIN_STACK from choosing the worker stack.
const ELABORATION_STACK_BYTES: usize = 32 * 1024 * 1024;

fn check_goal(depth: usize) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(move || {
            let mut env = ElabEnv::new().expect("prelude");
            for declaration in [
                "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }",
                "data DualEnv (a : Type) : Nat -> Type where { DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }",
                "fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup a m rest j } }",
                "data Tri (a : Type) : Type where { TR : List Nat -> a -> a -> Tri a }",
                "fn build (n : Nat) (acc : List Nat) : List Nat = match n { Zero ↦ acc; Suc m ↦ build m (Cons Nat Zero acc) }",
            ] {
                env.elaborate_decl(declaration).expect("convoy and list fixture");
            }
            let trust = env.env.trusted_base();
            let n = (0..depth).fold(String::from("Zero"), |n, _| format!("Suc ({n})"));
            let left = format!(
                "TR a (build ({n}) (Nil Nat)) (dual_lookup a (Suc m) xs (DualFZ m)) (dual_lookup a (Suc m) ys (DualFZ m))"
            );
            let source = format!(
                "theorem r1 (a : Type) (m : Nat) (xs : DualEnv a (Suc m)) (ys : DualEnv a (Suc m)) : Equal (Tri a) ({left}) ({left}) = match xs {{ DualCons k x rest ↦ match ys {{ DualCons k2 y rest2 ↦ Refl }} }}"
            );
            let result = env.elaborate_decl(&source);
            assert_eq!(env.env.trusted_base(), trust, "theorem adds no trust");
            assert!(result.is_ok(), "closed-list depth {depth} must check: {result:?}");
        })
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

/// MEASURED: a small closed list checks in a dependent match without added
/// trust. CLAIMED: the fixture exercises a real dependent convoy. THE GAP:
/// success at this size alone cannot detect quadratic budget charging.
#[test]
fn small_closed_list_control() {
    check_goal(0);
}

/// MEASURED: two larger closed lists check in the same convoy, with no trust
/// delta. CLAIMED: the visit allowance does not refuse a nonduplicating list
/// because it repeatedly bills the same reduct's descendants. THE GAP:
/// this bounds only the fixed fixture; other closed computations may exhaust
/// a finite allowance, even after the repair.
#[test]
fn nonduplicating_closed_list_at_eighty_checks() {
    check_goal(80);
}

#[test]
fn nonduplicating_closed_list_at_one_sixty_checks() {
    check_goal(160);
}
