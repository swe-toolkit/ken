//! Behavioral controls for CHECK-REFL-CLOSED-STEP-FOLD-DIVERGENCE.
//! The positive fixture is the exact 682-byte closed-step repro. These checks
//! exercise the real elaborator/kernel boundary, not repository text.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::KernelError;

const REPRO: &str = include_str!("fixtures/check_refl_closed_step_fold.ken");

fn with_definitions() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude admission");
    let (definitions, _) = REPRO
        .split_once("theorem wrapped_fixed_node")
        .expect("owned fixture has a theorem after the definitions");
    env.elaborate_file(definitions)
        .expect("Tree, fold, and wrapper must be admitted");
    env
}

fn assert_type_mismatch(error: ElabError) {
    assert!(
        matches!(
            &error,
            ElabError::TypeMismatch { .. }
                | ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    ..
                }
        ),
        "expected a type mismatch, got {error:?}"
    );
}

#[test]
fn closed_step_wrapper_node_refl_checks() {
    let mut env = ElabEnv::new().expect("prelude admission");
    env.elaborate_file(REPRO)
        .expect("closed-step Node equation is definitionally equal");
}

/// Increment 2: a three-deep ignored-argument wrapper changes a dependent
/// index. Its finite δ/β common reduct must be accepted during type checking,
/// not merely by a `Refl` goal whose observational ι can bypass the ledger.
#[test]
fn tagged_index_accepts_three_nested_skip_first_calls() {
    let mut env = ElabEnv::new().expect("prelude admission");
    env.elaborate_file(
        r#"
        data N = Z | S N
        data Tagged (n : N) : Type where { MkTagged : Tagged n }
        fn skip_first (x : N) (y : N) : N = S y
        fn retag (x1 : N) (x2 : N) (x3 : N) (y1 : N) (y2 : N) (y3 : N) (z : N)
            (t : Tagged (skip_first x1 (skip_first x2 (skip_first x3 z))))
            : Tagged (skip_first y1 (skip_first y2 (skip_first y3 z))) = t
        "#,
    )
    .expect("same acyclic transparent head must reduce to the equal index");
}

#[test]
fn bare_fold_node_equation_checks() {
    let mut env = with_definitions();
    env.elaborate_decl(
        "theorem fold_node (k : Type) (v : Type) (b : Type) \
         (left : Tree k v) (key : k) (val : v) (right : Tree k v) (acc : b) \
         (f : k → v → b → b) : Equal b \
         (fold k v b f acc (Node k v left key val right)) \
         (fold k v b f (f key val (fold k v b f acc left)) right) = Refl",
    )
    .expect("bare fold constructor equation checks");
}

#[test]
fn parameter_step_wrapper_node_equation_checks() {
    let mut env = with_definitions();
    env.elaborate_decl(
        "fn wrapped_param (k : Type) (v : Type) (b : Type) \
         (f : k → v → b → b) (m : Tree k v) (acc : b) : b = \
         fold k v b f acc m",
    )
    .expect("parameter wrapper admission");
    env.elaborate_decl(
        "theorem wrapped_param_node (k : Type) (v : Type) (b : Type) \
         (left : Tree k v) (key : k) (val : v) (right : Tree k v) (acc : b) \
         (f : k → v → b → b) : Equal b \
         (wrapped_param k v b f (Node k v left key val right) acc) \
         (wrapped_param k v b f right (f key val (wrapped_param k v b f left acc))) = Refl",
    )
    .expect("parameter-step wrapper constructor equation checks");
}

#[test]
fn proved_still_rejects() {
    let mut env = with_definitions();
    let (_, theorem) = REPRO
        .split_once("theorem wrapped_fixed_node")
        .expect("owned fixture contains a theorem");
    let source = format!(
        "theorem wrapped_fixed_node{}",
        theorem.replace("Refl", "Proved")
    );
    let error = env.elaborate_decl(&source).expect_err("Proved is not Refl");
    assert_type_mismatch(error);
}

#[test]
fn neutral_fold_with_distinct_steps_rejects() {
    let mut env = with_definitions();
    let error = env
        .elaborate_decl(
            "theorem unequal_step (k : Type) (v : Type) (b : Type) \
             (f : k → v → b → b) (g : k → v → b → b) \
             (acc : b) (left : Tree k v) : Equal b \
             (fold k v b f acc left) (fold k v b g acc left) = Refl",
        )
        .expect_err("different neutral steps cannot be equated");
    assert_type_mismatch(error);
}

#[test]
fn neutral_fold_with_distinct_accumulators_rejects() {
    let mut env = with_definitions();
    let error = env
        .elaborate_decl(
            "theorem unequal_acc (k : Type) (v : Type) (b : Type) \
             (f : k → v → b → b) (acc1 : b) (acc2 : b) \
             (left : Tree k v) : Equal b \
             (fold k v b f acc1 left) (fold k v b f acc2 left) = Refl",
        )
        .expect_err("different neutral accumulators cannot be equated");
    assert_type_mismatch(error);
}

#[test]
fn swapped_children_with_accumulator_step_rejects() {
    let mut env = with_definitions();
    let error = env
        .elaborate_decl(
            "theorem wrong_order (k : Type) (v : Type) (b : Type) \
             (left : Tree k v) (key : k) (val : v) (right : Tree k v) (acc : b) \
             : Equal b \
             (wrapped_fixed k v b (Node k v left key val right) acc) \
             (wrapped_fixed k v b left (wrapped_fixed k v b right acc)) = Refl",
        )
        .expect_err("left/right swap is not convertible for an accumulator step");
    assert_type_mismatch(error);
}
