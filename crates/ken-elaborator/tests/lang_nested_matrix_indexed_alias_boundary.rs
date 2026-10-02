//! Increment-0 transition pins: an enclosing alias cannot cross an indexed
//! inner match until the derived telescope replaces coordinate arithmetic.
//! Spec: spec/30-surface/34-data-match.md §3.1–3.2, §4.4.

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{normalize, Context};

const VEC: &str = r#"
data Vec (a : Type) : Nat → Type where {
  VNil : Vec a Zero;
  VCons : (n : Nat) → a → Vec a n → Vec a (Suc n)
}
"#;

fn nat(n: usize) -> String {
    (0..n).fold("Zero".to_owned(), |value, _| format!("Suc ({value})"))
}

#[test]
fn enclosing_alias_through_indexed_inner_match_is_refused() {
    // MEASURED: eleven indexed inner matches refuse at their own match span.
    // CLAIMED: enclosing aliases cannot silently choose a wrong Nat binder.
    // THE GAP: the matching compiler may shift a same-typed alias after this
    // check; disabling the flag restores R1=1/R2=2 in the mutation probe.
    // Transition sentinel: increment 2 flips every row to the value 3.
    // R1 and R2 separate saved=3, outer n=0 or 1, and j=2.
    let cases = [
        ("r2_check_n0", false, false, 0, false),
        ("r1_let_n1", true, true, 1, false),
        ("r1_check_n1", true, false, 1, false),
        ("r1_let_n0", true, true, 0, false),
        ("r1_check_n0", true, false, 0, false),
        ("single_let_n0", false, true, 0, true),
        ("r2_let_n0_wrapped", false, true, 0, false),
        ("r2_let_n0", false, true, 0, false),
        ("single_let_n1", false, true, 1, true),
        ("single_check_n1", false, false, 1, true),
        ("single_check_n0", false, false, 0, true),
    ];
    for (name, r1, via_let, n, single) in cases {
        let index = if r1 || single {
            format!("(Suc ({}))", nat(n))
        } else {
            nat(n)
        };
        let vector = if n == 0 && !r1 && !single {
            "VNil Nat".to_owned()
        } else if n == 0 {
            format!("VCons Nat Zero ({}) (VNil Nat)", nat(5))
        } else {
            format!(
                "VCons Nat (Suc Zero) ({}) (VCons Nat Zero ({}) (VNil Nat))",
                nat(5),
                nat(7)
            )
        };
        let inner = if single {
            "match xs { VCons m e tl ↦ saved }"
        } else if r1 {
            "match xs { VCons m Zero tl ↦ saved; VCons m (Suc k) tl ↦ saved }"
        } else {
            "match xs { VNil ↦ saved; VCons m e tl ↦ saved }"
        };
        let body = if name == "r2_let_n0_wrapped" {
            format!("let q = Zero in let r = {inner} in r")
        } else if via_let {
            format!("let r = {inner} in r")
        } else {
            inner.to_owned()
        };
        let source = format!(
            "{VEC}\nfn f (n : Nat) (xs : Vec Nat {index}) (x : Nat) : Nat = \
             match x {{ Zero ↦ Zero; (Suc j) as saved ↦ {body} }}\n\
             const observed : Nat = f ({}) ({vector}) (Suc (Suc (Suc Zero)))",
            nat(n)
        );
        let mut env = ElabEnv::new().expect("prelude");
        let trusted = env.env.trusted_base();
        let error = env.elaborate_file(&source).expect_err(name);
        assert_eq!(env.env.trusted_base(), trusted);
        assert!(
            matches!(error, ElabError::PatternVariableAcrossDependentSplit { ref span }
                if span.start == source.find("match xs").expect("indexed match")),
            "{name}: {error:?}"
        );
    }
}

fn assert_value(source: &str, observed: &str, expected: &str) {
    let mut env = ElabEnv::new().expect("prelude");
    let trusted = env.env.trusted_base();
    env.elaborate_file(source)
        .unwrap_or_else(|error| panic!("value control checks: {error:?}; source: {source}"));
    let normal = |name: &str| {
        let id = *env.globals.get(name).expect("named definition");
        let body = env.env.transparent_body(id).expect("checked body").1;
        normalize(&env.env, &Context::new(), &body)
    };
    assert_eq!(normal(observed), normal(expected));
    assert_eq!(env.env.trusted_base(), trusted);
}

#[test]
fn nonindexed_splits_and_plain_indexed_variable_preserve_values() {
    // MEASURED: nonindexed inner splits and an indexed plain-variable read
    // keep the value 3 with different competing inner Nat binders.
    // CLAIMED: the new indexed/sentinel conjunction does not refuse these.
    // THE GAP: a value pin does not certify an unmeasured same-typed guard
    // alias below a reverting split; that remains an increment-2 residual.
    let tuple = "fn f (x : Nat) (p : (a : Nat) × Nat) : Nat = let o = match x { \
         Zero ↦ Zero; (Suc j) as saved ↦ let r = match p { (a, b) ↦ saved } in r \
         } in o\nconst observed : Nat = f (Suc (Suc (Suc Zero))) (Suc Zero, Suc (Suc Zero))\n\
         const expected : Nat = Suc (Suc (Suc Zero))";
    assert_value(tuple, "observed", "expected");

    let list_head = "fn f (x : Nat) (ys : List Nat) : Nat = match x { \
         Zero ↦ Zero; (Suc j) as saved ↦ let r = match ys { \
           Nil ↦ saved; Cons Zero _ ↦ saved; \
           Cons (Suc k) _ ↦ saved \
         } in r }\n\
         const observed : Nat = f (Suc (Suc (Suc Zero))) \
           (Cons Nat (Suc (Suc (Suc (Suc Zero)))) (Nil Nat))\n\
         const expected : Nat = Suc (Suc (Suc Zero))";
    assert_value(list_head, "observed", "expected");

    let list_flat = "fn f (x : Nat) (ys : List Nat) : Nat = match x { \
         Zero ↦ Zero; (Suc j) as saved ↦ let r = match ys { \
           Nil ↦ saved; Cons e tl ↦ saved \
         } in r }\n\
         const observed : Nat = f (Suc (Suc (Suc Zero))) \
           (Cons Nat (Suc (Suc (Suc (Suc Zero)))) (Nil Nat))\n\
         const expected : Nat = Suc (Suc (Suc Zero))";
    assert_value(list_flat, "observed", "expected");

    let nested_variable = format!(
        "{VEC}\ndata NatBox : Type where {{ BoxNat : Nat → NatBox }}\n\
         fn f (h : NatBox) (xs : Vec Nat Zero) : Nat = match h {{ \
           BoxNat Zero ↦ Zero; \
           BoxNat (Suc saved) ↦ let r = match xs {{ \
             VNil ↦ saved; VCons m e tl ↦ saved \
           }} in r \
         }}\n\
         const observed : Nat = f (BoxNat (Suc (Suc (Suc (Suc Zero))))) (VNil Nat)\n\
         const expected : Nat = Suc (Suc (Suc Zero))"
    );
    assert_value(&nested_variable, "observed", "expected");

    let plain_variable = format!(
        "{VEC}\nfn f (saved : Nat) (xs : Vec Nat (Suc Zero)) : Nat = \
         match xs {{ VCons m e tl ↦ saved }}\n\
         const observed : Nat = f (Suc (Suc (Suc Zero))) \
           (VCons Nat Zero (Suc (Suc Zero)) (VNil Nat))\n\
         const expected : Nat = Suc (Suc (Suc Zero))"
    );
    assert_value(&plain_variable, "observed", "expected");
}
