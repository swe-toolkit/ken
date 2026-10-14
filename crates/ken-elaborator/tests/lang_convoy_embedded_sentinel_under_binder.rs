//! Embedded eliminators under binders keep their sentinel premises in scope.
//! Promise class: durable invariant. Spec 34 §3.2, §3.5; Architect D0.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::KernelError;

// The measured 1,982,464-byte prelude peak has more than fourfold headroom.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

const DUAL_FIN: &str = "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }";
const DUAL_ENV: &str = "data DualEnv (a : Type) : Nat -> Type where { DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }";
const FIN_TO_NAT: &str = "fn dual_fin_to_nat (n : Nat) (i : DualFin n) : Nat = match i { DualFZ m ↦ Zero; DualFS m j ↦ Suc (dual_fin_to_nat m j) }";
const LOOKUP: &str = "fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup a m rest j } }";
const L: &str = "dual_lookup a (Suc m) xs (DualFZ m)";

fn env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    for fixture in [DUAL_FIN, DUAL_ENV, FIN_TO_NAT, LOOKUP] {
        env.elaborate_decl(fixture)
            .unwrap_or_else(|error| panic!("fixture {fixture}: {error:?}"));
    }
    env
}

fn row(name: &str, binders: &str, goal: &str, leaf: &str) -> String {
    let binders = binders.replace("{L}", L);
    let goal = goal.replace("{L}", L);
    format!("theorem {name} (a : Type) (m : Nat) (xs : DualEnv a (Suc m)) (y : a) {binders} : {goal} = match xs {{ DualCons k x rest ↦ {leaf} }}")
}

fn checks(source: String) {
    on_elaboration_stack(move || {
        let mut env = env();
        let trust = env.env.trusted_base();
        env.elaborate_decl(&source)
            .unwrap_or_else(|error| panic!("must check {source}: {error:?}"));
        assert_eq!(env.env.trusted_base(), trust, "no added trusted identity");
    });
}

#[derive(Clone, Copy)]
enum Refusal {
    KernelTypeMismatch,
    SurfaceTypeMismatch,
}

fn refuses(source: String, expected: Refusal) {
    on_elaboration_stack(move || {
        let mut env = env();
        let trust = env.env.trusted_base();
        let error = env
            .elaborate_decl(&source)
            .expect_err("false goal must refuse");
        match expected {
            Refusal::KernelTypeMismatch => assert!(
                matches!(
                    error,
                    ElabError::KernelRejected {
                        error: KernelError::TypeMismatch { .. },
                        ..
                    }
                ),
                "must refuse with a kernel TypeMismatch, not Internal or VarOutOfScope: {source}: {error:?}"
            ),
            Refusal::SurfaceTypeMismatch => assert!(
                matches!(error, ElabError::TypeMismatch { .. }),
                "must refuse with a surface TypeMismatch, not Internal or VarOutOfScope: {source}: {error:?}"
            ),
        }
        assert_eq!(env.env.trusted_base(), trust, "no added trusted identity");
    });
}

fn direct(name: &str, binders: &str, goal: &str, body: &str) -> String {
    let binders = binders.replace("{L}", L);
    let goal = goal.replace("{L}", L);
    format!("theorem {name} (a : Type) (m : Nat) (xs : DualEnv a (Suc m)) (y : a) {binders} : {goal} = {body}")
}

const INDEX_LOOKUP: &str = "dual_lookup a n xs i";

fn index_row(name: &str, binders: &str, goal: &str, leaf: &str) -> String {
    let binders = binders.replace("{I}", INDEX_LOOKUP);
    let goal = goal.replace("{I}", INDEX_LOOKUP);
    format!("theorem {name} (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) (y : a) {binders} : {goal} = match i {{ DualFZ m ↦ match xs {{ DualCons k x rest ↦ {leaf} }}; DualFS m j ↦ match xs {{ DualCons k x rest ↦ {leaf} }} }}")
}

fn index_direct(name: &str, binders: &str, goal: &str, body: &str) -> String {
    let binders = binders.replace("{I}", INDEX_LOOKUP);
    let goal = goal.replace("{I}", INDEX_LOOKUP);
    format!("theorem {name} (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) (y : a) {binders} : {goal} = {body}")
}

// MEASURED: the inner method installs a sentinel below one binder and the
// proof checks. CLAIMED: the planner may keep binder-independent eliminators
// under binders. THE GAP: index companions have a separate installer and test.
#[test]
fn k9_goal_under_binder_checks() {
    checks(row("k9", "", "Nat → Equal a ({L}) ({L})", "λn. Refl"));
}

// MEASURED: k6/k7/k8/k10 and the dependent-premise u4/u5 shapes check
// through the method installer, including one and two goal binders. CLAIMED:
// outer planned methods retain their binding when moved beneath a telescope.
// THE GAP: the index-equation installer is independently checked below.
#[test]
fn method_premise_and_goal_binders_check() {
    let g = "(g : Nat → Equal a ({L}) y)";
    let h = "(h : Equal a ({L}) y)";
    let phi = "(phi : Equal a ({L}) y → Prop)";
    for source in [
        row("k6r", g, "Nat → Equal a ({L}) y", "g"),
        row("k7", g, "Equal a ({L}) y", "g Zero"),
        row("k8", h, "Nat → Equal a ({L}) y", "λn. h"),
        row(
            "k10r",
            "",
            "Nat → Nat → Equal a ({L}) ({L})",
            "λn. λp. Refl",
        ),
        row(
            "u4r",
            &format!("{h} {phi} (g : phi h → Equal a ({{L}}) y)"),
            "Equal a ({L}) y",
            "h",
        ),
        row(
            "u5r",
            &format!("{h} {phi} (r : phi h) (g : phi h → Equal a ({{L}}) y)"),
            "Equal a ({L}) y",
            "g r",
        ),
    ] {
        checks(source);
    }
}

// MEASURED: a mismatched endpoint is rejected with the named typed error,
// and Refl cannot close the false endpoint even under a goal binder. CLAIMED:
// weakening cannot convert a false proposition into a checked proof. THE GAP:
// the examples cover one distinct endpoint each; kernel checking is general.
#[test]
fn method_false_endpoints_refuse_typed() {
    refuses(
        row(
            "w2f",
            "(z : a) (g : Nat → Equal a ({L}) y)",
            "Nat → Equal a ({L}) z",
            "g",
        ),
        Refusal::KernelTypeMismatch,
    );
    refuses(
        row("w3f", "(z : a)", "Nat → Equal a ({L}) z", "λn. Refl"),
        Refusal::SurfaceTypeMismatch,
    );
}

// MEASURED: a nested index/matched-value eliminator checks with one and two
// return binders and with a proof premise. CLAIMED: the index companion uses
// the same ambient frame and local binder depth as its occurrence. THE GAP:
// QA independently removes this installer's local correction in AC-3b.
#[test]
fn index_companions_under_binders_check() {
    let i = "Equal a ({I}) ({I})";
    checks(index_row("i7b", "", &format!("Nat → {i}"), "λp. Refl"));
    checks(index_row(
        "i7bb",
        "",
        &format!("Nat → Nat → {i}"),
        "λp. λq. Refl",
    ));
    checks(index_row(
        "i6b",
        "(h : Equal a ({I}) y)",
        "Nat → Equal a ({I}) y",
        "λp. h",
    ));
}

// MEASURED: a false endpoint and two higher-order index-premise witnesses
// refuse with KernelRejected TypeMismatch, never Internal. CLAIMED: binding
// repair is fail-closed where the coordinate follow-up R1 is incomplete. THE
// GAP: R1's direct-body control checks, not the match; unification is separate.
#[test]
fn index_premises_and_false_endpoints_refuse_typed() {
    let g = "(g : Nat → Equal a ({I}) y)";
    refuses(
        index_row("i6g", g, "Equal a ({I}) y", "g Zero"),
        Refusal::KernelTypeMismatch,
    );
    refuses(
        index_row("i6gb", g, "Nat → Equal a ({I}) y", "g"),
        Refusal::KernelTypeMismatch,
    );
    refuses(
        index_row(
            "i6bf",
            "(z : a) (h : Equal a ({I}) y)",
            "Nat → Equal a ({I}) z",
            "λp. h",
        ),
        Refusal::KernelTypeMismatch,
    );
}

// MEASURED: direct bodies without the match, plus the binder-free d3/i7
// controls, still check. CLAIMED: the repair targets embedded match operands
// only. THE GAP: the existing 15-suite AC-2 run tests adjacent convoy forms.
#[test]
fn direct_bodies_and_binder_free_controls_still_check() {
    let g = "(g : Nat → Equal a ({L}) y)";
    let h = "(h : Equal a ({L}) y)";
    let phi = "(phi : Equal a ({L}) y → Prop)";
    for source in [
        direct("k6c", g, "Nat → Equal a ({L}) y", "g"),
        direct("k7c", g, "Equal a ({L}) y", "g Zero"),
        direct("k8c", h, "Nat → Equal a ({L}) y", "λn. h"),
        direct("k9c", "", "Nat → Equal a ({L}) ({L})", "λn. Refl"),
        direct(
            "k10c",
            "",
            "Nat → Nat → Equal a ({L}) ({L})",
            "λn. λp. Refl",
        ),
        direct(
            "u4c",
            &format!("{h} {phi} (g : phi h → Equal a ({{L}}) y)"),
            "Equal a ({L}) y",
            "h",
        ),
        direct(
            "u5c",
            &format!("{h} {phi} (r : phi h) (g : phi h → Equal a ({{L}}) y)"),
            "Equal a ({L}) y",
            "g r",
        ),
        row("d3", "", "Equal a ({L}) ({L})", "Refl"),
        index_row("i7", "", "Equal a ({I}) ({I})", "Refl"),
        index_direct("i7bc", "", "Nat → Equal a ({I}) ({I})", "λp. Refl"),
        index_direct(
            "i7bbc",
            "",
            "Nat → Nat → Equal a ({I}) ({I})",
            "λp. λq. Refl",
        ),
        index_direct(
            "i6bc",
            "(h : Equal a ({I}) y)",
            "Nat → Equal a ({I}) y",
            "λp. h",
        ),
        index_direct(
            "i6gc",
            "(g : Nat → Equal a ({I}) y)",
            "Equal a ({I}) y",
            "g Zero",
        ),
        index_direct(
            "i6gbc",
            "(g : Nat → Equal a ({I}) y)",
            "Nat → Equal a ({I}) y",
            "g",
        ),
    ] {
        checks(source);
    }
}
