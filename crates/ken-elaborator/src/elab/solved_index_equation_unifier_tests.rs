//! Checked solved-index and no-trial body-entry controls (34 §3.5, 39 §2).

use super::SOLVED_INDEX_BODY_VISITS;
use crate::{ElabEnv, ElabError};
use ken_kernel::{normalize, Context, KernelError, Term};

// The adjacent indexed-convoy fixture measured a 1,982,464-byte prelude
// peak. A fixed 32 MiB stack adds over 30 MiB headroom for seven nested
// methods; Builder::stack_size, not ambient RUST_MIN_STACK, provisions it.
const ELABORATION_STACK_BYTES: usize = 32 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

const CHAIN: &str = "data Chain (a : Type) : Nat -> Type where { ChainNil : Chain a Zero; ChainCons : (n : Nat) -> a -> Chain a n -> Chain a (Suc n) }";
const PARAM_NEST: &str = "data ParamNest (p : Nat) : Nat -> Type where { ParamStop : ParamNest p p; ParamStep : ParamNest p p -> ParamNest p p }";
const VEC: &str = "data Vec (a : Type) : Nat -> Type where { VNil : Vec a Zero; VCons : (n : Nat) -> a -> Vec a n -> Vec a (Suc n) }";

fn suc_count(depth: usize) -> String {
    (0..depth).fold("n".to_string(), |value, _| format!("(Suc {value})"))
}

fn chain_body(depth: usize, leaf: &str) -> String {
    let mut body = leaf.to_string();
    for index in (1..=depth).rev() {
        let scrutinee = if index == 1 {
            "xs".to_string()
        } else {
            format!("rest{}", index - 1)
        };
        body =
            format!("match {scrutinee} {{ ChainCons k{index} value{index} rest{index} ↦ {body} }}");
    }
    format!(
        "fn nested (a : Type) (n : Nat) (xs : Chain a {}) : Nat = {body}",
        suc_count(depth)
    )
}

fn param_body(depth: usize, leaf: &str) -> String {
    let mut body = leaf.to_string();
    for index in (1..=depth).rev() {
        let scrutinee = if index == 1 {
            "xs".to_string()
        } else {
            format!("rest{}", index - 1)
        };
        body = format!("match {scrutinee} {{ ParamStop ↦ Zero; ParamStep rest{index} ↦ {body} }}");
    }
    format!("fn nested_param (p : Nat) (xs : ParamNest p p) : Nat = {body}")
}

fn vec_body(depth: usize, leaf: &str) -> String {
    let mut body = leaf.to_string();
    for index in (1..=depth).rev() {
        let scrutinee = if index == 1 {
            "xs".to_string()
        } else {
            format!("t{}", index - 1)
        };
        body = format!(
            "match {scrutinee} {{ VNil ↦ VNil a; VCons m{index} x{index} t{index} ↦ VCons a m{index} x{index} ({body}) }}"
        );
    }
    format!("fn deep_vec (a : Type) (n : Nat) (xs : Vec a n) : Vec a n = {body}")
}

fn closed_nat(depth: usize) -> String {
    (0..depth).fold("Zero".to_string(), |value, _| format!("(Suc {value})"))
}

fn closed_vec(depth: usize) -> String {
    (0..depth).fold("VNil Nat".to_string(), |tail, index| {
        format!(
            "VCons Nat {} {} ({tail})",
            closed_nat(index),
            closed_nat(index + 1)
        )
    })
}

fn check_count(source: &str, fixture: &str) -> (Result<(), ElabError>, usize) {
    let mut env = ElabEnv::new().expect("prelude");
    env.elaborate_decl(fixture).expect("indexed family");
    let trust = env.env.trusted_base();
    SOLVED_INDEX_BODY_VISITS.with(|count| count.set(Some(0)));
    let result = env.elaborate_decl(source).map(|_| ());
    let count = SOLVED_INDEX_BODY_VISITS.with(|count| count.replace(None).expect("active count"));
    assert_eq!(env.env.trusted_base(), trust, "no trusted-base delta");
    (result, count)
}

/// Promise class: durable invariant. MEASURED: every checked Chain nesting
/// visits exactly one method body per level for both rejected and accepted
/// leaves. CLAIMED: removed trial cannot multiply work at depth. THE GAP:
/// the separate parameter-side family checks a regime not solved by a field.
#[test]
fn solved_field_nesting_uses_one_body_entry_per_level() {
    on_elaboration_stack(|| {
        for depth in (1..=7).rev() {
            let (error, entries) = check_count(&chain_body(depth, "Refl"), CHAIN);
            assert!(
                matches!(&error, Err(ElabError::TypeMismatch { .. })),
                "depth {depth}: false Nat leaf should refuse: {error:?}"
            );
            assert_eq!(entries, depth, "depth {depth}: no retry multiplication");
        }
        for depth in [3, 5, 7] {
            let (result, entries) = check_count(&chain_body(depth, "Zero"), CHAIN);
            result.unwrap_or_else(|error| panic!("accepted twin depth {depth}: {error:?}"));
            assert_eq!(
                entries, depth,
                "accepted depth {depth}: one visit per level"
            );
        }
    });
}

/// Promise class: durable invariant. MEASURED: for an all-parameter-side
/// indexed family, both accepted and typed-refused nests visit exactly two
/// constructor bodies per level and add no trust. CLAIMED: refined F checks
/// each arm once without trying U first. THE GAP: this family isolates the
/// parameter-side route; the independent Vec control exercises accepted
/// nested goal substitution even when there are no restorations.
#[test]
fn parameter_side_nesting_checks_each_arm_once() {
    on_elaboration_stack(|| {
        for depth in [1, 3, 5, 7] {
            let (result, accepted_entries) = check_count(&param_body(depth, "Zero"), PARAM_NEST);
            result.unwrap_or_else(|error| panic!("accepted parameter nest {depth}: {error:?}"));
            assert_eq!(
                accepted_entries,
                2 * depth,
                "accepted parameter nest {depth}"
            );
            let (refusal, refused_entries) = check_count(&param_body(depth, "Refl"), PARAM_NEST);
            assert!(
                matches!(&refusal, Err(ElabError::TypeMismatch { .. })),
                "false Nat leaf depth {depth}: {refusal:?}"
            );
            assert_eq!(refused_entries, 2 * depth, "refused parameter nest {depth}");
        }
    });
}

/// Promise class: durable invariant. MEASURED: nested Vec reconstruction
/// admits a correct leaf and rejects a wrong leaf, with two constructor-body
/// entries per level in both cases. CLAIMED: F can succeed without retry or
/// selecting only nonempty restorations. THE GAP: an independent checked
/// closed-call result is observed by the separate output test below.
#[test]
fn returned_vector_nesting_checks_each_arm_once() {
    on_elaboration_stack(|| {
        for depth in 2..=7 {
            let (accepted, accepted_entries) =
                check_count(&vec_body(depth, &format!("t{depth}")), VEC);
            accepted.unwrap_or_else(|error| panic!("accepted Vec depth {depth}: {error:?}"));
            assert_eq!(accepted_entries, 2 * depth, "accepted Vec depth {depth}");
            let (refusal, refused_entries) =
                check_count(&vec_body(depth, &format!("x{depth}")), VEC);
            assert!(
                matches!(
                    &refusal,
                    Err(ElabError::TypeMismatch { .. })
                        | Err(ElabError::KernelRejected {
                            error: KernelError::TypeMismatch { .. },
                            ..
                        })
                ),
                "wrong Vec leaf depth {depth}: {refusal:?}"
            );
            assert_eq!(refused_entries, 2 * depth, "refused Vec depth {depth}");
        }
    });
}

/// Promise class: durable invariant. MEASURED: independently constructed
/// closed vectors emerge unchanged from admitted deep Vec reconstruction
/// (including distinct head/tail elements). CLAIMED: selecting F directly
/// preserves emitted values, not merely admission. THE GAP: the open generic
/// method is tested separately; these checked calls observe closed instances.
#[test]
fn returned_vector_nesting_emits_the_independent_vector() {
    on_elaboration_stack(|| {
        for depth in [2, 5, 7] {
            let mut env = ElabEnv::new().expect("prelude");
            env.elaborate_decl(VEC).expect("indexed vector fixture");
            let trust = env.env.trusted_base();
            env.elaborate_decl(&vec_body(depth, &format!("t{depth}")))
                .unwrap_or_else(|error| panic!("deep Vec {depth}: {error:?}"));
            let length = closed_nat(depth);
            let independent = closed_vec(depth);
            let observed = env
                .elaborate_decl(&format!(
                    "const output : Vec Nat {length} = deep_vec Nat {length} ({independent})"
                ))
                .unwrap_or_else(|error| panic!("checked Vec call depth {depth}: {error:?}"));
            let expected = env
                .elaborate_decl(&format!("const oracle : Vec Nat {length} = {independent}"))
                .unwrap_or_else(|error| panic!("checked Vec expectation depth {depth}: {error:?}"));
            let normal = |id| {
                let body = env
                    .env
                    .transparent_body(id)
                    .expect("checked transparent const")
                    .1;
                normalize(&env.env, &Context::new(), &body)
            };
            assert_eq!(
                normal(observed),
                normal(expected),
                "emitted Vec depth {depth}"
            );
            assert_eq!(env.env.trusted_base(), trust, "Vec output adds no trust");
        }
    });
}

// Inspect the checked core term, not a rendering of source text. A retained
// mixed or multi-leaf alias installs a Cast in each of the two hand methods.
fn contains_cast(term: &Term) -> bool {
    match term {
        Term::Cast(..) => true,
        Term::Elim {
            params,
            motive,
            methods,
            indices,
            scrut,
            ..
        } => {
            params.iter().any(contains_cast)
                || contains_cast(motive)
                || methods.iter().any(contains_cast)
                || indices.iter().any(contains_cast)
                || contains_cast(scrut)
        }
        Term::Let { ty, val, body } => {
            contains_cast(ty) || contains_cast(val) || contains_cast(body)
        }
        Term::QuotElim {
            motive,
            method,
            respect,
            scrut,
        } => {
            contains_cast(motive)
                || contains_cast(method)
                || contains_cast(respect)
                || contains_cast(scrut)
        }
        Term::Pi(a, b)
        | Term::Lam(a, b)
        | Term::App(a, b)
        | Term::Sigma(a, b)
        | Term::Pair(a, b)
        | Term::Ascript(a, b)
        | Term::Absurd(a, b) => contains_cast(a) || contains_cast(b),
        Term::Eq(a, b, c) | Term::J(a, b, c) | Term::Quot(a, b, c) => {
            contains_cast(a) || contains_cast(b) || contains_cast(c)
        }
        Term::Proj1(a)
        | Term::Proj2(a)
        | Term::Refl(a)
        | Term::QuotClass(a)
        | Term::Trunc(a)
        | Term::TruncProj(a) => contains_cast(a),
        Term::Type(_)
        | Term::Omega(_)
        | Term::Var(_)
        | Term::Const { .. }
        | Term::IntLit(_)
        | Term::IndFormer { .. }
        | Term::Constructor { .. } => false,
    }
}

fn checked_hand_rows(mixed: bool) {
    let mut env = ElabEnv::new().expect("prelude");
    for fixture in [
        "data GridPoint : Type where { Pt : Nat -> Nat -> GridPoint }",
        "data BiIndex : GridPoint -> Type where { At : (u : Nat) -> (v : Nat) -> BiIndex (Pt u v) }",
        "data OneIndex : GridPoint -> Type where { On : (u : Nat) -> OneIndex (Pt u Zero) }",
    ] {
        env.elaborate_decl(fixture).expect("hand-row fixture");
    }
    let trust = env.env.trusted_base();
    let bi = "(x : Nat) (y : Nat) (P : Nat -> Nat -> Type) (h : (u : Nat) -> (v : Nat) -> BiIndex (Pt u v) -> P u v)";
    let one =
        "(x : Nat) (y : Nat) (P : Nat -> Type) (h : (u : Nat) -> OneIndex (Pt u Zero) -> P u)";
    let rows = if mixed {
        [
            (
                "one_true",
                one,
                "OneIndex (Pt x y)",
                "P x",
                "On u ↦ h u (On u)",
            ),
            (
                "one_false",
                one,
                "OneIndex (Pt x y)",
                "P y",
                "On u ↦ h u (On u)",
            ),
        ]
    } else {
        [
            (
                "bi_true",
                bi,
                "BiIndex (Pt x y)",
                "P x y",
                "At u v ↦ h u v (At u v)",
            ),
            (
                "bi_false",
                bi,
                "BiIndex (Pt x y)",
                "P y x",
                "At u v ↦ h u v (At u v)",
            ),
        ]
    };
    for (name, head, scrutinee, result_ty, arm) in rows {
        let source =
            format!("fn {name} {head} (z : {scrutinee}) : {result_ty} = match z {{ {arm} }}");
        let result = env.elaborate_decl(&source);
        if name.ends_with("_true") {
            let id = result.unwrap_or_else(|error| panic!("{name}: {error:?}"));
            let (_, body) = env.env.transparent_body(id).expect("checked hand-row body");
            assert!(
                !contains_cast(&body),
                "{name}: variable-side field must be eliminated"
            );
        } else {
            assert!(
                matches!(
                    &result,
                    Err(ElabError::KernelRejected {
                        error: KernelError::TypeMismatch { .. },
                        ..
                    })
                ),
                "{name}: expected kernel-refused wrong result: {result:?}"
            );
        }
        assert_eq!(env.env.trusted_base(), trust, "{name}: no added trust");
    }
}

/// Promise class: durable admission/refusal invariant for a decomposed
/// multi-leaf equation. MEASURED: the accepted emitted core has no alias Cast
/// and the reversed result refuses at the kernel with unchanged trust.
/// CLAIMED: gate removal reaches this hand-row method, not only acceptance.
/// THE GAP: this core-shape pin is a transition sentinel: revisit it if the
/// Architect later approves SolvedJ-as-Cast; other index families remain in
/// the independent consumer suite.
#[test]
fn multi_leaf_equation_eliminates_its_field() {
    on_elaboration_stack(|| checked_hand_rows(false));
}

/// Promise class: durable admission/refusal invariant for a mixed-variable
/// and constructor index. MEASURED: the accepted emitted core has no alias
/// Cast and the reversed result refuses at the kernel with unchanged trust.
/// CLAIMED: the method uses elimination, not the retained alias path. THE GAP:
/// the Cast exclusion is a transition sentinel until a future approved
/// SolvedJ-as-Cast route replaces this implementation.
#[test]
fn mixed_equation_eliminates_its_field() {
    on_elaboration_stack(|| checked_hand_rows(true));
}

const NONLINEAR_INDEX_FIXTURES: [&str; 6] = [
    "data GridPoint : Type where { Pt : Nat -> Nat -> GridPoint }",
    "data Two : Nat -> Nat -> Type where { MkTwo : (u : Nat) -> Two u u }",
    "data TwoS : Nat -> Nat -> Type where { MkTwoS : (u : Nat) -> TwoS u (Suc u) }",
    "data Diag : GridPoint -> Type where { MkDiag : (u : Nat) -> Diag (Pt u u) }",
    "data SucD : GridPoint -> Type where { MkSucD : (u : Nat) -> SucD (Pt u (Suc u)) }",
    "data Tri : GridPoint -> Nat -> Type where { MkTri : (u : Nat) -> Tri (Pt u Zero) u }",
];

fn nonlinear_index_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    for fixture in NONLINEAR_INDEX_FIXTURES {
        env.elaborate_decl(fixture).expect("indexed family fixture");
    }
    env
}

fn check_nonlinear_accepted(row: &str, source: &str) {
    let mut env = nonlinear_index_env();
    let trust = env.env.trusted_base();
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("{row}: {error:?}"));
    assert_eq!(env.env.trusted_base(), trust, "{row}: no added trust");
}

/// Promise class: durable admission invariant (34 §3.5, 39 §2).
/// MEASURED: five indexed constructor families with a repeated field each
/// admit a plain value, a dependent continuation and a Refl theorem, without
/// adding trusted declarations. CLAIMED: the method's non-linear residual
/// receives the same sound alias solution as before the solved-index unifier.
/// THE GAP: acceptance does not assert the emitted core's Cast shape; the
/// separate mixed and multi-leaf tests above pin the elimination boundary.
#[test]
fn nonlinear_index_fields_preserve_parent_accepted_methods() {
    on_elaboration_stack(|| {
        for (tag, scrutinee, constructor, equality) in [
            ("two", "Two x y", "MkTwo u", "Equal Nat x y"),
            ("twos", "TwoS x y", "MkTwoS u", "Equal Nat (Suc x) y"),
            ("diag", "Diag (Pt x y)", "MkDiag u", "Equal Nat x y"),
            ("sucd", "SucD (Pt x y)", "MkSucD u", "Equal Nat (Suc x) y"),
            ("tri", "Tri (Pt x Zero) y", "MkTri u", "Equal Nat x y"),
        ] {
            for (kind, source) in [
                (
                    "value",
                    format!("fn {tag}_value (x : Nat) (y : Nat) (z : {scrutinee}) : Nat = match z {{ {constructor} ↦ u }}"),
                ),
                (
                    "continuation",
                    format!("fn {tag}_continuation (x : Nat) (y : Nat) (P : Nat -> Type) (h : (u : Nat) -> P u) (z : {scrutinee}) : P x = match z {{ {constructor} ↦ h u }}"),
                ),
                (
                    "theorem",
                    format!("theorem {tag}_theorem (x : Nat) (y : Nat) (z : {scrutinee}) : {equality} = match z {{ {constructor} ↦ Refl }}"),
                ),
            ] {
                check_nonlinear_accepted(&format!("{tag}/{kind}"), &source);
            }
        }
    });
}

/// Promise class: durable refusal and acceptance boundaries.
/// MEASURED: a false TwoS equality refuses specifically as TypeMismatch,
/// not an internal solved-premise error; both reflexive Two forms and two
/// continuations to P y remain accepted without a trusted-base delta.
/// CLAIMED: routing a non-linear method to the alias rule does not make a
/// false theorem provable or reject a method already valid on the parent.
/// THE GAP: deeper nested-constructor costs stay with the depth-seven test.
#[test]
fn nonlinear_index_false_goal_refuses_and_reflexive_controls_check() {
    on_elaboration_stack(|| {
        let mut env = nonlinear_index_env();
        let trust = env.env.trusted_base();
        let false_theorem = "theorem twos_false (x : Nat) (y : Nat) (z : TwoS x y) : Equal Nat x y = match z { MkTwoS u ↦ Refl }";
        let result = env.elaborate_decl(false_theorem);
        assert!(
            matches!(result, Err(ElabError::TypeMismatch { .. })),
            "false TwoS equality must refuse as TypeMismatch: {result:?}"
        );
        assert_eq!(env.env.trusted_base(), trust, "false theorem adds no trust");
        for (row, source) in [
            (
                "two_reflexive_value",
                "fn two_reflexive_value (x : Nat) (z : Two x x) : Nat = match z { MkTwo u ↦ u }",
            ),
            (
                "two_reflexive_theorem",
                "theorem two_reflexive_theorem (x : Nat) (z : Two x x) : Equal Nat x x = match z { MkTwo u ↦ Refl }",
            ),
            (
                "two_continuation_y",
                "fn two_continuation_y (x : Nat) (y : Nat) (P : Nat -> Type) (h : (u : Nat) -> P u) (z : Two x y) : P y = match z { MkTwo u ↦ h u }",
            ),
            (
                "twos_continuation_y",
                "fn twos_continuation_y (x : Nat) (y : Nat) (P : Nat -> Type) (h : (u : Nat) -> P (Suc u)) (z : TwoS x y) : P y = match z { MkTwoS u ↦ h u }",
            ),
        ] {
            check_nonlinear_accepted(row, source);
        }
    });
}
