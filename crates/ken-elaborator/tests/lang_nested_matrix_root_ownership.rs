//! An inner match cannot determine the motive owned by an enclosing indexed
//! match. Spec: spec/30-surface/34-data-match.md §3.1–3.2, §4.4.
//! Promise class: durable value invariant. Each fixture enters through
//! `elaborate_file`; the kernel checks its generated eliminators.

use ken_elaborator::ElabEnv;
use ken_kernel::{normalize, Context};

const VEC: &str = r#"
data Vec (a : Type) : Nat → Type where {
  VNil : Vec a Zero;
  VCons : (n : Nat) → a → Vec a n → Vec a (Suc n)
}
"#;

fn assert_root_value(body: &str, expected_value: &str) {
    let source = format!(
        "{VEC}\nfn b2n (b : Bool) : Nat = match b {{ True ↦ Suc Zero; False ↦ Zero }}\n\
         fn f (n : Nat) (v : Vec Nat n) : Nat = {body}\n\
         const observed : Nat = f Zero (VNil Nat)\n\
         const expected : Nat = {expected_value}"
    );
    let mut env = ElabEnv::new().expect("prelude");
    let trusted = env.env.trusted_base();
    env.elaborate_file(&source)
        .expect("inner match must not take the outer indexed root motive");
    let normal = |name: &str| {
        let id = *env.globals.get(name).expect("named checked constant");
        let body = env.env.transparent_body(id).expect("transparent value").1;
        normalize(&env.env, &Context::new(), &body)
    };
    assert_eq!(normal("observed"), normal("expected"));
    assert_eq!(env.env.trusted_base(), trusted);
}

#[test]
fn inner_inferred_match_does_not_claim_enclosing_indexed_root() {
    // MEASURED: an inference-mode Bool match inside the outer first leaf
    // leaves f Zero (VNil Nat) equal to Zero (b2n False). CLAIMED: only
    // f's own match can determine its indexed motive. THE GAP: removing
    // the gate at the leaf producer must redden this pin by refusing the
    // outer root; the checked-inner pin below guards the other producer.
    assert_root_value(
        "let r = match v { \
           VNil ↦ (let q = match True { True ↦ False; False ↦ True } in b2n q); \
           VCons m e tl ↦ m } in r",
        "Zero",
    );
}

#[test]
fn inner_checked_nonflat_match_does_not_claim_enclosing_indexed_root() {
    // MEASURED: a check-mode, non-flat Nat match inside the first leaf
    // preserves the outer indexed motive and normalizes to Suc Zero.
    // CLAIMED: entry seeding cannot write a frame owned by the outer match.
    // THE GAP: this exercises entry seeding, while the preceding fixture
    // independently exercises the first-leaf writer.
    assert_root_value(
        "let r = match v { \
           VNil ↦ b2n (match Suc Zero { \
             Zero ↦ False; Suc Zero ↦ True; Suc (Suc k) ↦ False }); \
           VCons m e tl ↦ m } in r",
        "Suc Zero",
    );
}

#[test]
fn outer_indexed_root_without_inner_match_still_computes() {
    // MEASURED: the indexed root without any inner match returns Suc Zero.
    // CLAIMED: the ownership gate does not block the root's own first leaf.
    // THE GAP: this is a positive control for root ownership, not a witness
    // for either kind of inner match; the two fixtures above provide those.
    assert_root_value(
        "let r = match v { VNil ↦ Suc Zero; VCons m e tl ↦ m } in r",
        "Suc Zero",
    );
}
