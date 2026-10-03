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
"#;

const POLYMORPHIC_TREE: &str = r#"
data Tree (a : Type) : Type where {
  Leaf : Tree a;
  Node : a → Tree a → Tree a
}
"#;

const BINARY_TREE: &str = r#"
data Tree : Type where {
  Leaf : Tree;
  Node : Tree → Nat → Tree → Tree
}
"#;

fn nat(value: usize) -> String {
    (0..value).fold("Zero".to_owned(), |acc, _| format!("Suc ({acc})"))
}

fn observe_with_families(label: &str, families: &str, body: &str, expected: usize) {
    let source = format!(
        "{families}\n{body}\nconst expected : Nat = {}",
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

fn observe(label: &str, body: &str, expected: usize) {
    observe_with_families(label, FAMILIES, body, expected);
}

fn observe_tree(label: &str, body: &str, expected: usize) {
    observe_with_families(
        label,
        &format!("{FAMILIES}\n{POLYMORPHIC_TREE}"),
        body,
        expected,
    );
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
        // Exact Architect D1: the VNil bucket returns m=0, not e=2.
        "D1" => (
            "match (xs, ys) { (CCons m e tl, VCons j y yt) ↦ e; \
             (CCons m e tl, VNil) ↦ m; (CNil, _) ↦ Zero }",
            "(Suc Zero) Zero (CCons Nat Zero (Suc (Suc Zero)) (CNil Nat)) (VNil Nat)",
            0,
        ),
        // These three independent reconstructions are not verbatim
        // Adversary D2/D5/D8 sources (those were never published).
        "D2-reconstructed" => (
            "match (xs, ys) { (CCons m e tl, VCons j y rest) ↦ y; \
             (CCons m e tl, VNil) ↦ e; (CNil, _) ↦ Zero }",
            "(Suc Zero) (CCons Nat Zero (Suc (Suc Zero)) (CNil Nat)) \
             (Suc Zero) (VCons Nat Zero (Suc (Suc (Suc Zero))) (VNil Nat))",
            3,
        ),
        "D5-reconstructed" => (
            "match (xs, ys) { (CCons m e tl, VCons j Zero rest) ↦ e; \
             (CCons m e tl, VCons j (Suc y) rest) ↦ y; \
             (CCons m e tl, VNil) ↦ m; (CNil, _) ↦ Zero }",
            "(Suc Zero) (CCons Nat Zero (Suc (Suc Zero)) (CNil Nat)) \
             (Suc Zero) (VCons Nat Zero (Suc (Suc (Suc (Suc Zero)))) (VNil Nat))",
            3,
        ),
        "D8-reconstructed" => (
            "match (xs, ys) { (CCons m e tl, VNil) ↦ saved; \
             (CCons m e tl, VCons j y rest) ↦ e; (CNil, _) ↦ Zero }",
            "(Suc (Suc (Suc Zero))) (Suc Zero) \
             (CCons Nat Zero (Suc (Suc (Suc (Suc (Suc Zero))))) (CNil Nat)) \
             Zero (VNil Nat)",
            3,
        ),
        _ => panic!("unknown tuple row: {row}"),
    };
    let params = if row == "D8-reconstructed" {
        "(saved : Nat) (n : Nat)"
    } else if row == "D1" {
        "(n : Nat) (k : Nat)"
    } else {
        "(n : Nat)"
    };
    let body = format!(
        "fn f {params} (xs : CVec Nat n) {}(ys : Vec Nat k) : Nat = {}\n\
         const observed : Nat = f {args}",
        if row == "D1" { "" } else { "(k : Nat) " },
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
fn d2_reconstructed_infer() {
    tuple_columns("D2-reconstructed", false);
}
#[test]
fn d2_reconstructed_check() {
    tuple_columns("D2-reconstructed", true);
}
#[test]
fn d5_reconstructed_infer() {
    tuple_columns("D5-reconstructed", false);
}
#[test]
fn d5_reconstructed_check() {
    tuple_columns("D5-reconstructed", true);
}
#[test]
fn d8_reconstructed_infer() {
    tuple_columns("D8-reconstructed", false);
}

fn sibling_fields(row: &str, checked: bool) {
    // MEASURED: the sibling Vec field is reached after the nested CVec IH.
    // CLAIMED: both sibling field and CVec tail use the rerun's coordinates.
    // THE GAP: a value pin cannot locate which internal coordinate failed;
    // the direct check peer separates discovery from the checked route.
    let matched = if row == "D3" {
        // Exact Architect D3 source, with a separate VNil sibling.
        "match b { MkD k (CCons m e tl) (VCons j y yt) ↦ e; \
         MkD k (CCons m e tl) VNil ↦ m; MkD k CNil v ↦ Zero }"
            .to_owned()
    } else {
        // The D4 sibling is an independently reconstructed shape control.
        "match b { MkD k (CCons m e tl) (VCons j y rest) ↦ y; \
         MkD k _ _ ↦ Zero }"
            .to_owned()
    };
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
    // MEASURED: the exact Architect D7c inner CVec/Zero match under an
    // outer CVec IH returns outer e=2 on CNil, not the inner y.
    // CLAIMED: both outer routes preserve discovery of the inner first leaf.
    // THE GAP: the checked outer peer still infers its inner result and so
    // cannot establish an explicitly checked inner result.
    let matched = "match (ys, Zero) { (CCons j y yt, _) ↦ y; (CNil, z) ↦ e }";
    let inner = format!("let q = {matched} in q");
    let outer = format!("match xs {{ CCons m e tl ↦ {inner}; CNil ↦ Zero }}");
    let body = format!(
        "fn f (n : Nat) (k : Nat) (xs : CVec Nat n) (ys : CVec Nat k) : Nat = {}\n\
         const observed : Nat = f (Suc Zero) Zero \
           (CCons Nat Zero (Suc (Suc Zero)) (CNil Nat)) (CNil Nat)",
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
    observe_tree("E2", body, 2);
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
    observe_tree("E3", body, 2);
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
    observe_tree("D6", body, 1);
}

fn three_columns(checked: bool) {
    // MEASURED: this exact Architect triple has a Σ tail column after the
    // indexed IH; its distinct y=3 wins over e=2 and m=0. CLAIMED: all
    // later columns share discovery/replay coordinates. THE GAP: a binary
    // tuple cannot reveal a lowering omission hidden inside Σ.
    let matched = "match (xs, b, ys) { \
      (CCons m e tl, True, VCons j y yt) ↦ y; \
      (CCons m e tl, True, VNil) ↦ e; \
      (CCons m e tl, False, _) ↦ m; (CNil, _, _) ↦ Zero }";
    let body = format!(
        "fn f (n : Nat) (k : Nat) (xs : CVec Nat n) (b : Bool) \
           (ys : Vec Nat k) : Nat = {}\n\
         const observed : Nat = f (Suc Zero) (Suc Zero) \
           (CCons Nat Zero (Suc (Suc Zero)) (CNil Nat)) True \
           (VCons Nat Zero (Suc (Suc (Suc Zero))) (VNil Nat))",
        infer_or_check(matched, checked)
    );
    observe(
        if checked {
            "THREE-check"
        } else {
            "THREE-infer"
        },
        &body,
        3,
    );
}

#[test]
fn three_infer_total_lowering_under_sigma_tail() {
    three_columns(false);
}

#[test]
fn three_check_control() {
    three_columns(true);
}

fn sigr_indexed(checked: bool) {
    // MEASURED: the exact Architect SIGR first leaf has a dependent Σ
    // result carrying outer xs. CLAIMED: discovery projects the result
    // out of the constructor telescope using total checked lowering.
    // THE GAP: checking an annotated let bypasses R discovery; test both.
    let matched = "match (xs, b) { \
        (CCons m e tl, True) ↦ (e, xs); \
        (CCons m e tl, False) ↦ (m, xs); \
        (CNil, _) ↦ (Zero, xs) }";
    let binding = if checked {
        format!("let r : (a : Nat) × CVec Nat n = {matched} in match r {{ (a, _) ↦ a }}")
    } else {
        format!("let r = {matched} in match r {{ (a, _) ↦ a }}")
    };
    let body = format!(
        "fn f (n : Nat) (xs : CVec Nat n) (b : Bool) : Nat = {binding}\n\
         const observed : Nat = f (Suc Zero) \
           (CCons Nat Zero (Suc (Suc Zero)) (CNil Nat)) True"
    );
    observe(if checked { "SIGR-check" } else { "SIGR-infer" }, &body, 2);
}

#[test]
fn sigr_indexed_inferred_sigma_result() {
    sigr_indexed(false);
}

#[test]
fn sigr_indexed_checked_sigma_result_control() {
    sigr_indexed(true);
}

fn sigr_tree(checked: bool) {
    // MEASURED: this exact Architect non-indexed binary Tree returns 2
    // through a Σ result with an outer Vec; the old partial traversal
    // kernel-rejected it. CLAIMED: R projection is total under Σ.
    // THE GAP: the check peer alone does not exercise R projection.
    let matched = "match (t, b) { \
      (Node l x rt, True) ↦ (x, xs); \
      (Node l x rt, False) ↦ (Zero, xs); \
      (Leaf, _) ↦ (Suc Zero, xs) }";
    let binding = if checked {
        format!("let r : (a : Nat) × Vec Nat n = {matched} in match r {{ (a, _) ↦ a }}")
    } else {
        format!("let r = {matched} in match r {{ (a, _) ↦ a }}")
    };
    let body = format!(
        "fn f (n : Nat) (xs : Vec Nat n) (t : Tree) (b : Bool) : Nat = {binding}\n\
         const observed : Nat = f Zero (VNil Nat) \
           (Node Leaf (Suc (Suc Zero)) Leaf) True"
    );
    observe_with_families(
        if checked {
            "SIGRtree-check"
        } else {
            "SIGRtree-infer"
        },
        &format!("{FAMILIES}\n{BINARY_TREE}"),
        &body,
        2,
    );
}

#[test]
fn sigr_tree_inferred_sigma_result() {
    sigr_tree(false);
}

#[test]
fn sigr_tree_checked_sigma_result_control() {
    sigr_tree(true);
}

#[test]
fn architect_binary_tree_control() {
    // MEASURED: the Architect's binary Tree source selects x=2 while its
    // fields contribute two IHs. CLAIMED: ordinary non-indexed splitting
    // remains unchanged. THE GAP: its fields have no dependent index.
    let body = "fn f (t : Tree) : Nat = let r = match t { \
      Node l x rt ↦ x; Leaf ↦ Zero } in r\n\
      const observed : Nat = f (Node Leaf (Suc (Suc Zero)) Leaf)";
    observe_with_families("TREE", &format!("{FAMILIES}\n{BINARY_TREE}"), body, 2);
}
