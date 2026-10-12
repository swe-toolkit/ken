//! A goal states equality by checked identity, not the spelling `Equal` or
//! the location of `Refl`. Promise class: durable invariant; spec 17 §1,
//! spec 16 §2.1 and spec 34 §3.5. Every accepted proof is kernel-checked.

use ken_elaborator::{ElabEnv, ElabError};

// Existing Pair-goal and indexed-motive suites measured a 1,982,464-byte
// bare-prelude peak and a 3 MiB nested-match minimum. This new test provisions
// 8 MiB, leaving over 5 MiB above that nested minimum; it pins no stack bound.
const ELABORATION_STACK_BYTES: usize = 8 * 1024 * 1024;

fn on_elaboration_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(ELABORATION_STACK_BYTES)
        .spawn(f)
        .expect("spawn stated-stack elaboration")
        .join()
        .expect("stated-stack elaboration panicked");
}

fn fixture_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("prelude");
    for decl in [
        "const MyEq : Pair Nat Nat -> Pair Nat Nat -> Prop = λp. λq. Eq (Pair Nat Nat) p q",
        "const MyEq2 : Pair Nat Nat -> Pair Nat Nat -> Prop = MyEq",
        "const MyEq3 : Pair Nat Nat -> Pair Nat Nat -> Prop = λp. λq. Equal (Pair Nat Nat) p q",
        "const MyEq4 : Pair Nat Nat -> Pair Nat Nat -> Prop = λp. MyEq p",
        "const Both : Nat -> Nat -> Prop = λa. λb. And (Equal Nat a a) (Equal Nat b b)",
        "data DualFin : Nat -> Type where { DualFZ : (n : Nat) -> DualFin (Suc n); DualFS : (n : Nat) -> DualFin n -> DualFin (Suc n) }",
    ] {
        env.elaborate_decl(decl)
            .unwrap_or_else(|error| panic!("fixture {decl}: {error:?}"));
    }
    env
}

fn pair_goal(relation: &str, left: &str, right: &str) -> String {
    format!("{relation} ({left}) ({right})")
}

fn route(goal: &str, kind: &str, name: &str) -> String {
    let body = match kind {
        "standalone" => "Refl",
        "let" => "let y = z in Refl",
        "list" => "match xs { Nil ↦ Refl; Cons x rest ↦ Refl }",
        "bool" => "match b { True ↦ Refl; False ↦ Refl }",
        "dualfin" => "match i { DualFZ m ↦ Refl; DualFS m j ↦ Refl }",
        _ => panic!("unknown route {kind}"),
    };
    format!(
        "theorem {name} (n : Nat) (i : DualFin n) (xs : List Nat) (b : Bool)
            (z : Nat) (a : Nat) (c : Nat) (p : Pair Nat Nat)
            : {goal} = {body}"
    )
}

#[test]
fn alias_of_eq_checks_on_five_routes_and_alias_chains_on_two() {
    // MEASURED: each route elaborates and kernel-checks with no new trust.
    // CLAIMED: all Refl callers recognize δ/β-chased Eq by checked identity.
    // THE GAP: a one-step arm predicate misses chained aliases, and a
    // spelling-keyed standalone gate misses MyEq even though arms pass.
    on_elaboration_stack(|| {
        let mut env = fixture_env();
        let trust_before = env.env.trusted_base();
        for relation in ["MyEq", "MyEq2", "MyEq3", "MyEq4"] {
            let routes: &[&str] = if relation == "MyEq" {
                &["standalone", "let", "list", "bool", "dualfin"]
            } else {
                &["standalone", "list"]
            };
            for &kind in routes {
                let name = format!("true_{}_{}", relation.to_lowercase(), kind);
                let source = route(&pair_goal(relation, "p", "p"), kind, &name);
                env.elaborate_decl(&source).unwrap_or_else(|error| {
                    panic!("{relation} {kind} must kernel-check: {error:?}")
                });
                assert_eq!(env.env.trusted_base(), trust_before, "{relation} {kind}");
            }
        }
    });
}

#[test]
fn false_alias_reaches_conversion_refusal_on_every_route() {
    // MEASURED: all five failures name the nonconvertible endpoints, not an
    // unrelated shape failure. CLAIMED: chasing the alias grants no false
    // equality. THE GAP: generic rejection also passes on base, where the
    // standalone gate stops prematurely on the wrong spelling.
    on_elaboration_stack(|| {
        let mut env = fixture_env();
        let trust_before = env.env.trusted_base();
        let goal = pair_goal("MyEq", "mk_pair Nat Nat a c", "mk_pair Nat Nat c a");
        for kind in ["standalone", "let", "list", "bool", "dualfin"] {
            let source = route(&goal, kind, &format!("false_{kind}"));
            let error = env
                .elaborate_decl(&source)
                .expect_err("swapped symbolic Pair components cannot check");
            assert!(
                matches!(error, ElabError::TypeMismatch { ref reason, .. }
                    if reason == "Refl: the two sides of the goal are not convertible"),
                "{kind} must reach the conversion refusal: {error:?}"
            );
            assert_eq!(env.env.trusted_base(), trust_before, "{kind}");
        }
    });
}

#[test]
fn and_of_equalities_is_not_a_stated_equality() {
    // MEASURED: an And containing true equalities stops at the Eq-shape gate
    // on both routes. CLAIMED: arbitrary Sigma/And is not equality intro.
    // THE GAP: Both and MyEq whnf to Sigma, so only the stated Eq head
    // distinguishes this refusal from the accepted MyEq controls above.
    on_elaboration_stack(|| {
        let mut env = fixture_env();
        let trust_before = env.env.trusted_base();
        for kind in ["standalone", "list"] {
            let source = route("Both a c", kind, &format!("both_{kind}"));
            let error = env
                .elaborate_decl(&source)
                .expect_err("And of equalities is not an equality goal");
            assert!(
                matches!(error, ElabError::TypeMismatch { ref reason, .. }
                    if reason == "Refl expects an `Eq`-shaped goal"),
                "{kind} must refuse at the shape gate: {error:?}"
            );
            assert_eq!(env.env.trusted_base(), trust_before, "{kind}");
        }
    });
}
