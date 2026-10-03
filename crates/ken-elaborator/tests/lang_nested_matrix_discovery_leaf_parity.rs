//! Discovery of the first inferred leaf under an indexed IH must use the
//! same context coordinates as its checked replay. The non-indexed controls
//! keep ordinary nested matching independent of the indexed route.
//! Spec: spec/30-surface/34-data-match.md §3.1–3.2, §4.4.
//! Promise class: durable checked-value invariants.

use ken_elaborator::ElabEnv;
use ken_kernel::{normalize, Context};

const FAMILIES: &str = r#"
data CVec (a : Type) : Nat → Type where {
  CCons : (n : Nat) → a → CVec a n → CVec a (Suc n);
  CNil : CVec a Zero
}
data Vec (a : Type) : Nat → Type where {
  VCons : (n : Nat) → a → Vec a n → Vec a (Suc n);
  VNil : Vec a Zero
}
data DBox : Type where {
  MkD : (k : Nat) → CVec Nat k → Vec Nat k → DBox
}
data Tree (a : Type) : Type where {
  Leaf : Tree a;
  Node : a → Tree a → Tree a
}
"#;

fn nat(value: usize) -> String {
    (0..value).fold("Zero".to_owned(), |acc, _| format!("Suc ({acc})"))
}

fn observe(label: &str, body: &str, expected: usize) {
    let source = format!(
        "{FAMILIES}\n{body}\nconst expected : Nat = {}",
        nat(expected)
    );
    let mut env = ElabEnv::new().expect("prelude");
    let trusted = env.env.trusted_base();
    env.elaborate_file(&source)
        .unwrap_or_else(|error| panic!("{label}: {error:?}; source: {source}"));
    let normal = |name: &str| {
        let id = *env.globals.get(name).expect("named checked value");
        let body = env.env.transparent_body(id).expect("transparent value").1;
        normalize(&env.env, &Context::new(), &body)
    };
    assert_eq!(
        normal("observed"),
        normal("expected"),
        "{label}: wrong value"
    );
    assert_eq!(env.env.trusted_base(), trusted, "{label}: new trust");
}

fn infer_or_check(matched: &str, checked: bool) -> String {
    if checked {
        matched.to_owned()
    } else {
        format!("let r = {matched} in r")
    }
}

fn e1(checked: bool) {
    // MEASURED: e=2 is distinct from m=0, b and the fallback; the first
    // CCons leaf precedes its indexed IH and the Bool column. CLAIMED:
    // inference and checking agree after first-leaf reuse. THE GAP: the
    // checked peer alone cannot reach discovery, so run the inferred form.
    let matched = "match (xs, b) { (CCons m e tl, True) ↦ e; \
        (CCons m e tl, False) ↦ m; (CNil, _) ↦ Zero }";
    let body = format!(
        "fn f (n : Nat) (xs : CVec Nat n) (b : Bool) : Nat = {}\n\
         const observed : Nat = f (Suc Zero) \
           (CCons Nat Zero (Suc (Suc Zero)) (CNil Nat)) True",
        infer_or_check(matched, checked)
    );
    observe(if checked { "E1-check" } else { "E1-infer" }, &body, 2);
}

#[test]
fn e1_inferred_first_leaf_after_indexed_ih() {
    e1(false);
}

#[test]
fn e1_checked_control() {
    e1(true);
}

fn tuple_columns(row: &str, checked: bool) {
    // MEASURED: every row observes a closed Nat value after splitting CVec
    // before the independent indexed Vec column. CLAIMED: discovery and
    // checked replay agree for both IH-bearing columns. THE GAP: rows only
    // claim their explicit shapes, not every possible indexed family.
    let (matched, args, expected) = match row {
        "D1" => (
            "match (xs, ys) { (CCons m e tl, VNil) ↦ e; \
             (CCons m e tl, VCons j y rest) ↦ m; (CNil, _) ↦ Zero }",
            "(Suc Zero) (CCons Nat Zero (Suc (Suc Zero)) (CNil Nat)) \
             Zero (VNil Nat)",
            2,
        ),
        "D2" => (
            "match (xs, ys) { (CCons m e tl, VCons j y rest) ↦ y; \
             (CCons m e tl, VNil) ↦ e; (CNil, _) ↦ Zero }",
            "(Suc Zero) (CCons Nat Zero (Suc (Suc Zero)) (CNil Nat)) \
             (Suc Zero) (VCons Nat Zero (Suc (Suc (Suc Zero))) (VNil Nat))",
            3,
        ),
        "D5" => (
            "match (xs, ys) { (CCons m e tl, VCons j Zero rest) ↦ e; \
             (CCons m e tl, VCons j (Suc y) rest) ↦ y; \
             (CCons m e tl, VNil) ↦ m; (CNil, _) ↦ Zero }",
            "(Suc Zero) (CCons Nat Zero (Suc (Suc Zero)) (CNil Nat)) \
             (Suc Zero) (VCons Nat Zero (Suc (Suc (Suc (Suc Zero)))) (VNil Nat))",
            3,
        ),
        "D8" => (
            "match (xs, ys) { (CCons m e tl, VNil) ↦ saved; \
             (CCons m e tl, VCons j y rest) ↦ e; (CNil, _) ↦ Zero }",
            "(Suc (Suc (Suc Zero))) (Suc Zero) \
             (CCons Nat Zero (Suc (Suc (Suc (Suc (Suc Zero))))) (CNil Nat)) \
             Zero (VNil Nat)",
            3,
        ),
        _ => panic!("unknown tuple row: {row}"),
    };
    let params = if row == "D8" {
        "(saved : Nat) (n : Nat)"
    } else {
        "(n : Nat)"
    };
    let body = format!(
        "fn f {params} (xs : CVec Nat n) (k : Nat) (ys : Vec Nat k) : Nat = {}\n\
         const observed : Nat = f {args}",
        infer_or_check(matched, checked)
    );
    observe(
        &format!("{row}-{}", if checked { "check" } else { "infer" }),
        &body,
        expected,
    );
}

#[test]
fn d1_infer() {
    tuple_columns("D1", false);
}
#[test]
fn d1_check() {
    tuple_columns("D1", true);
}
#[test]
fn d2_infer() {
    tuple_columns("D2", false);
}
#[test]
fn d2_check() {
    tuple_columns("D2", true);
}
#[test]
fn d5_infer() {
    tuple_columns("D5", false);
}
#[test]
fn d5_check() {
    tuple_columns("D5", true);
}
#[test]
fn d8_infer() {
    tuple_columns("D8", false);
}

fn sibling_fields(row: &str, checked: bool) {
    // MEASURED: the sibling Vec field is reached after the nested CVec IH.
    // CLAIMED: both sibling field and CVec tail use the rerun's coordinates.
    // THE GAP: a value pin cannot locate which internal coordinate failed;
    // the direct check peer separates discovery from the checked route.
    let leaf = if row == "D3" { "e" } else { "y" };
    let matched = format!(
        "match b {{ MkD k (CCons m e tl) (VCons j y rest) ↦ {leaf}; \
         MkD k _ _ ↦ Zero }}"
    );
    let body = format!(
        "fn f (b : DBox) : Nat = {}\n\
         const observed : Nat = f (MkD (Suc Zero) \
           (CCons Nat Zero (Suc (Suc Zero)) (CNil Nat)) \
           (VCons Nat Zero (Suc (Suc (Suc Zero))) (VNil Nat)))",
        infer_or_check(&matched, checked)
    );
    observe(
        &format!("{row}-{}", if checked { "check" } else { "infer" }),
        &body,
        if row == "D3" { 2 } else { 3 },
    );
}

#[test]
fn d3_infer() {
    sibling_fields("D3", false);
}
#[test]
fn d3_check() {
    sibling_fields("D3", true);
}
#[test]
fn d4_infer() {
    sibling_fields("D4", false);
}
#[test]
fn d4_check() {
    sibling_fields("D4", true);
}

fn d7(checked: bool) {
    // MEASURED: an inferred Vec/Zero tuple sits under an outer CVec IH
    // and returns e=2, not y=3, whether the outer match infers or checks.
    // CLAIMED: both outer routes preserve discovery of the inner first leaf.
    // THE GAP: the checked outer peer still infers its inner result and so
    // cannot establish an explicitly checked inner result.
    let matched = "match (ys, Zero) { (VCons j y rest, Zero) ↦ e; \
        (VCons j y rest, Suc z) ↦ Zero; (VNil, _) ↦ Zero }";
    let inner = format!("let q = {matched} in q");
    let outer = format!("match xs {{ CCons m e tl ↦ {inner}; CNil ↦ Zero }}");
    let body = format!(
        "fn f (n : Nat) (xs : CVec Nat n) (k : Nat) (ys : Vec Nat k) : Nat = {}\n\
         const observed : Nat = f (Suc Zero) \
           (CCons Nat Zero (Suc (Suc Zero)) (CNil Nat)) (Suc Zero) \
           (VCons Nat Zero (Suc (Suc (Suc Zero))) (VNil Nat))",
        infer_or_check(&outer, checked)
    );
    observe(if checked { "D7-check" } else { "D7-infer" }, &body, 2);
}

#[test]
fn d7_infer() {
    d7(false);
}
#[test]
fn d7_check() {
    d7(true);
}

#[test]
fn e2_nonindexed_tree_bool_control() {
    // MEASURED: Tree/Bool returns its distinct element 2 under Tree's IH.
    // CLAIMED: the non-indexed route remains available. THE GAP: this
    // control says nothing about indexed field-domain coordinates.
    let body = "fn f (t : Tree Nat) (b : Bool) : Nat = \
       let r = match (t, b) { (Node e tl, True) ↦ e; \
         (Node e tl, False) ↦ Zero; (Leaf, _) ↦ Zero } in r
\
       const observed : Nat = f (Node Nat (Suc (Suc Zero)) (Leaf Nat)) True";
    observe("E2", body, 2);
}

#[test]
fn e3_nested_nonindexed_tree_bool_control() {
    // MEASURED: the inner (Tree, Bool) split under Tree's IH returns 2.
    // CLAIMED: nested non-indexed matching remains available. THE GAP:
    // Tree's IH has no index substitution, unlike the failing CVec rows.
    let body = "fn f (t : Tree Nat) (b : Bool) : Nat = match t { \
      Node e tl ↦ let q = match (tl, b) { \
        (Node x rest, True) ↦ x; (Node x rest, False) ↦ Zero; \
        (Leaf, _) ↦ Zero } in q; Leaf ↦ Zero }
\
      const observed : Nat = f \
        (Node Nat Zero (Node Nat (Suc (Suc Zero)) (Leaf Nat))) True";
    observe("E3", body, 2);
}

#[test]
fn d6_nonindexed_nested_recursive_self_call_control() {
    // MEASURED: matching a nested Tree and recursively calling on rest
    // reduces to 1. CLAIMED: ordinary nested recursion remains available.
    // THE GAP: this path has no indexed field-domain transformation.
    let body = "fn f (t : Tree Nat) : Nat = match t { \
      Leaf ↦ Zero; Node x (Node y rest) ↦ Suc (f rest); \
      Node x Leaf ↦ Suc Zero }
\
      const observed : Nat = f \
        (Node Nat Zero (Node Nat (Suc Zero) (Leaf Nat)))";
    observe("D6", body, 1);
}
