//! Solved index variables are eliminated with one reverted-goal transport.
//! Promise class: durable typed-admission/refusal invariant (34 §3.2, §3.5).
//! Architect D0 evt_3nbk4bv8c7kpe; LANG-SOLVED-INDEX-SUBSTITUTION AC-1/2.

use ken_elaborator::{ElabEnv, ElabError};

// The landed dual-branch fixture measured a 1,982,464-byte prelude peak;
// 8 MiB provides more than fourfold headroom for its nested methods.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

const DUAL_FIXTURES: &[&str] = &[
    "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }",
    "data DualEnv (a : Type) : Nat -> Type where { DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }",
    "fn dual_fin_to_nat (n : Nat) (i : DualFin n) : Nat = match i { DualFZ m ↦ Zero; DualFS m j ↦ Suc (dual_fin_to_nat m j) }",
    "fn dual_lookup (a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : a = match i { DualFZ m ↦ match xs { DualCons k x rest ↦ x }; DualFS m j ↦ match xs { DualCons k x rest ↦ dual_lookup a m rest j } }",
    "fn dual_len (a : Type) (n : Nat) (xs : DualEnv a n) : Nat = match xs { DualNil ↦ Zero; DualCons k x rest ↦ Suc (dual_len a k rest) }",
];
const LK: &str = "dual_lookup a n xs i";
const EQ_HEAD: &str = "(a : Type) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : Equal a (LK) (LK)";
const PRED_HEAD: &str = "(a : Type) (pp : a → Prop) (hp : (x : a) → pp x) (n : Nat) (xs : DualEnv a n) (i : DualFin n) : pp (LK)";

fn dual_row(name: &str, head: &str, zero: &str, successor: &str) -> String {
    format!(
        "theorem {name} {} = match i {{ DualFZ m ↦ match xs {{ DualCons k x rest ↦ {zero} }}; DualFS m j ↦ match xs {{ DualCons k x rest ↦ {} }} }}",
        head.replace("LK", LK),
        successor.replace("REC", name)
    )
}

fn env_with(fixtures: &[&str]) -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    let trust = env.env.trusted_base();
    for fixture in fixtures {
        env.elaborate_decl(fixture)
            .unwrap_or_else(|error| panic!("fixture {fixture}: {error:?}"));
    }
    assert_eq!(env.env.trusted_base(), trust, "fixtures add no trust");
    env
}

fn accepts(fixtures: &[&str], source: &str) {
    let mut env = env_with(fixtures);
    let trust = env.env.trusted_base();
    env.elaborate_decl(source)
        .unwrap_or_else(|error| panic!("expected acceptance: {source}: {error:?}"));
    assert_eq!(env.env.trusted_base(), trust, "source adds no trust");
}

fn refuses_typed(fixtures: &[&str], source: &str) {
    let mut env = env_with(fixtures);
    let trust = env.env.trusted_base();
    let error = env.elaborate_decl(source).expect_err("false twin accepted");
    assert!(
        matches!(
            error,
            ElabError::KernelRejected { .. } | ElabError::TypeMismatch { .. }
        ),
        "false twin must refuse by a type error: {error:?}"
    );
    assert_eq!(env.env.trusted_base(), trust, "false twin adds no trust");
}

/// MEASURED: sec/secpp and secrest elaborate as kernel-checked proofs.
/// CLAIMED: a field solved by an index premise is substituted through later
/// binders and the goal before a nested match. THE GAP: this probes one Dual
/// family; the PEnv sibling tests separately vary a dependent later field.
macro_rules! accepts_dual {
    ($test:ident, $head:ident, $zero:literal, $body:literal) => {
        #[test]
        fn $test() {
            on_elaboration_stack(|| {
                accepts(
                    DUAL_FIXTURES,
                    &dual_row(stringify!($test), $head, $zero, $body),
                );
            });
        }
    };
}

accepts_dual!(
    sec,
    EQ_HEAD,
    "Refl",
    "match j { DualFZ p ↦ Refl; DualFS p j2 ↦ Refl }"
);
accepts_dual!(
    secpp,
    PRED_HEAD,
    "hp x",
    "match j { DualFZ p ↦ hp (dual_lookup a m rest j); DualFS p j2 ↦ hp (dual_lookup a m rest j) }"
);
accepts_dual!(
    secrest,
    EQ_HEAD,
    "Refl",
    "match rest { DualNil ↦ Refl; DualCons q y r2 ↦ Refl }"
);

const SIBLING_FIXTURES: &[&str] = &[
    "data DualEnv (a : Type) : Nat -> Type where { DualNil : DualEnv a Zero; DualCons : (n : Nat) -> a -> DualEnv a n -> DualEnv a (Suc n) }",
    "fn dual_len (a : Type) (n : Nat) (xs : DualEnv a n) : Nat = match xs { DualNil ↦ Zero; DualCons k x rest ↦ Suc (dual_len a k rest) }",
    "fn eq_use (x : Nat) (y : Nat) (p : Equal Nat x y) : Nat = Zero",
    "data PEnv (a : Type) : Nat -> Type where { PNil : PEnv a Zero; PCons : (n : Nat) -> (rest : DualEnv a n) -> Equal Nat (dual_len a n rest) n -> PEnv a (Suc n) }",
    "data Box : Nat -> Type where { MkBox : (k : Nat) -> Equal Nat k k -> Box (Suc k) }",
];

fn sibling_row(name: &str, body: &str) -> String {
    format!(
        "fn {name} (a : Type) (n : Nat) (ys : PEnv a (Suc n)) : Nat = match ys {{ PCons k rest h ↦ {body} }}"
    )
}

/// MEASURED: q2/q3's written types agree with a sibling proof field whose
/// type depends on both the solved index and a preceding field. CLAIMED: one
/// solved coordinate reaches both declarations. THE GAP: this does not cover
/// every constructor telescope; separate Box and Dual cases vary the shape.
macro_rules! accepts_sibling {
    ($test:ident, $body:literal) => {
        #[test]
        fn $test() {
            on_elaboration_stack(|| {
                accepts(SIBLING_FIXTURES, &sibling_row(stringify!($test), $body));
            });
        }
    };
}

accepts_sibling!(q2, "let h2 : Equal Nat (dual_len a k rest) k = h in Zero");
accepts_sibling!(
    q3,
    "eq_use (dual_len a k rest) k (h : Equal Nat (dual_len a k rest) k)"
);

/// MEASURED: adjacent healthy paths still check and false proposition twins
/// still reject with a typed error. CLAIMED: substitution preserves the
/// existing scope of checked witnesses; missing/equality transport remains
/// fail-closed. THE GAP: these are witnesses, not all possible wrong goals.
#[test]
fn prior_let_and_sibling_controls_stay_typed() {
    on_elaboration_stack(|| {
        accepts(DUAL_FIXTURES, &dual_row("secrestpp", PRED_HEAD, "hp x", "match rest { DualNil ↦ hp (dual_lookup a m rest j); DualCons q y r2 ↦ hp (dual_lookup a m rest j) }"));
        for (name, head, zero, successor) in [
            ("r1", EQ_HEAD, "Refl", "let ih : Equal a (dual_lookup a m rest j) (dual_lookup a m rest j) = REC a m rest j in ih"),
            ("r1pp", PRED_HEAD, "hp x", "let ih : pp (dual_lookup a m rest j) = REC a pp hp m rest j in ih"),
            ("r2", PRED_HEAD, "hp x", "let ih : pp (dual_lookup a m rest j) = hp (dual_lookup a m rest j) in ih"),
            ("r2len", PRED_HEAD, "hp x", "let u : Equal Nat (dual_len a m rest) (dual_len a m rest) = Refl in hp (dual_lookup a m rest j)"),
        ] {
            accepts(DUAL_FIXTURES, &dual_row(name, head, zero, successor));
        }
        for (name, successor) in [
            ("r1f", "let ih : pp x = REC a pp hp m rest j in ih"),
            ("r2f", "let ih : pp x = hp (dual_lookup a m rest j) in ih"),
        ] {
            refuses_typed(DUAL_FIXTURES, &dual_row(name, PRED_HEAD, "hp x", successor));
        }
        accepts(
            SIBLING_FIXTURES,
            &sibling_row("p8", "eq_use (dual_len a k rest) k h"),
        );
        accepts(SIBLING_FIXTURES, "fn b1 (n : Nat) (b : Box (Suc n)) : Nat = match b { MkBox k e ↦ eq_use k k (e : Equal Nat k k) }");
        accepts(SIBLING_FIXTURES, "fn b2 (n : Nat) (b : Box (Suc n)) : Nat = match b { MkBox k e ↦ let e2 : Equal Nat k k = e in Zero }");
    });
}
