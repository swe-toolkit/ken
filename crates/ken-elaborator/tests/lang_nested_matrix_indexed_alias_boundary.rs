//! Derived-telescope value pins: enclosing aliases remain in the same
//! coordinates as the plain binders beneath indexed inner matches.
//! Spec: spec/30-surface/34-data-match.md §3.1–3.2, §4.4.

use ken_elaborator::ElabEnv;
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
fn enclosing_alias_through_indexed_inner_match_returns_saved() {
    // Durable invariant. Eleven indexed matches return the enclosing alias
    // saved=3, not same-typed n=0/1, j=2 or inner e=5/7.
    // R1 and R2 separate saved=3, outer n=0 or 1, and j=2. The
    // annotated R1 RHS actually checks the nested match against Nat;
    // unannotated/direct first leaves discover their result by inference.
    let cases = [
        ("r2_check_n0", false, false, 0, false),
        ("r1_let_n1", true, true, 1, false),
        ("r1_check_n1", true, false, 1, false),
        ("r1_let_n0", true, true, 0, false),
        ("r1_check_n0", true, false, 0, false),
        ("single_let_n0", false, true, 0, true),
        ("r1_annotated_check_n1", true, false, 1, false),
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
        let body = if name == "r1_annotated_check_n1" {
            format!("let r : Nat = {inner} in r")
        } else if via_let {
            format!("let r = {inner} in r")
        } else {
            inner.to_owned()
        };
        let source = format!(
            "{VEC}\nfn f (n : Nat) (xs : Vec Nat {index}) (x : Nat) : Nat = \
             match x {{ Zero ↦ Zero; (Suc j) as saved ↦ {body} }}\n\
             const observed : Nat = f ({}) ({vector}) (Suc (Suc (Suc Zero)))
             const expected : Nat = Suc (Suc (Suc Zero))",
            nat(n)
        );
        assert_value(&source, "observed", "expected");
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

#[test]
fn zero_premise_indexed_inner_alias_returns_saved() {
    // Durable invariant. A zero-premise Eq-index method returns the enclosing
    // saved/x=3 instead of same-typed j=2 or inner m=1; no wrap guard can
    // rescue wrong occurrence coordinates.
    let source = "data Ix : ((k : Nat) → Equal Nat k k) → Type where { \
        Mk : (m : Nat) → (p : (k : Nat) → Equal Nat k k) → Ix p }\n\
        theorem eq_refl : (k : Nat) → Equal Nat k k = \\k. Refl\n\
        fn f (p : (k : Nat) → Equal Nat k k) (v : Ix p) (x : Nat) : Nat = match x { \
          Zero ↦ Zero; (Suc j) as saved ↦ let q = match v { \
            Mk Zero w ↦ saved; Mk (Suc m) w ↦ saved \
          } in q \
        }\n\
        const observed : Nat = f eq_refl (Mk (Suc Zero) eq_refl) (Suc (Suc (Suc Zero)))
        const expected : Nat = Suc (Suc (Suc Zero))";
    assert_value(source, "observed", "expected");
}

#[test]
fn checked_indexed_inner_alias_preserves_saved_across_premises() {
    // Durable invariant. A checked inner method returns saved=3 despite its
    // nonempty premise wrap; merely removing the old guard returned Zero.
    // Index-forced group {n,m}=0 follows Suc m = Suc n at xs. Independently
    // assignable same-typed binders are saved=3, j=2 and e=5; the result
    // must be 3, not the base's wrong Zero from {n,m}.
    let source = format!(
        "{VEC}\nfn f (n : Nat) (xs : Vec Nat (Suc n)) (x : Nat) : Nat = match x {{ \
           Zero ↦ Zero; (Suc j) as saved ↦ let r : Nat = match xs {{ \
             VCons m e tl ↦ saved \
           }} in r \
         }}\n\
         const observed : Nat = f Zero \
           (VCons Nat Zero (Suc (Suc (Suc (Suc (Suc Zero))))) (VNil Nat)) \
           (Suc (Suc (Suc Zero)))
         const expected : Nat = Suc (Suc (Suc Zero))"
    );
    assert_value(&source, "observed", "expected");
}

#[test]
fn checked_indexed_inner_match_preserves_plain_and_woven_binders() {
    // Durable value controls for the checked route. The four distinct inputs
    // isolate the enclosing alias from plain references; a guard-only removal
    // selects the wrong Zero on the sibling's `saved` row instead.
    // MEASURED: checked dependent matches return 3, 2, 5, and 3.
    // CLAIMED: checked matching still transports ordinary context occurrences.
    // THE GAP: these rows do not establish the alias representation on their
    // own; the `saved` sibling above pins the distinct alias value.
    for (arm_body, expected) in [("Suc j", 3), ("j", 2), ("e", 5)] {
        let source = format!(
            "{VEC}\nfn f (n : Nat) (xs : Vec Nat (Suc n)) (x : Nat) : Nat = \
             match x {{ Zero ↦ Zero; (Suc j) as saved ↦ let r : Nat = \
             match xs {{ VCons m e tl ↦ {arm_body} }} in r }}\n\
             const observed : Nat = f Zero \
               (VCons Nat Zero ({}) (VNil Nat)) ({})\n\
             const expected : Nat = {}",
            nat(5),
            nat(3),
            nat(expected)
        );
        assert_value(&source, "observed", "expected");
    }

    let nested_woven = format!(
        "{VEC}\ndata NatBox : Type where {{ BoxNat : Nat → NatBox }}\n\
         fn f (n : Nat) (xs : Vec Nat (Suc n)) (h : NatBox) : Nat = \
         match h {{ BoxNat Zero ↦ Zero; BoxNat (Suc saved) ↦ let r : Nat = \
           match xs {{ VCons m e tl ↦ saved }} in r }}\n\
         const observed : Nat = f Zero \
           (VCons Nat Zero ({}) (VNil Nat)) (BoxNat (Suc ({})))\n\
         const expected : Nat = {}",
        nat(5),
        nat(3),
        nat(3)
    );
    assert_value(&nested_woven, "observed", "expected");
}
