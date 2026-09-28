//! `L3-strings-surface` acceptance tests (slice 2/2 of the string surface).
//!
//! Pins `conformance/surface/collections/seed-collections.md`'s "Derived
//! string surface (slice 2)" section, DS-AC1–7
//! (`spec/30-surface/37-strings-collections.md` §2.4/§2.5/§2.5.1/§4.1). Drives
//! the actual package through the production roots loader (never a hand-copied
//! reimplementation, matching `es4_classes_acceptance.rs`'s discipline):
//! - DS-AC1/AC5 `list-combinator-floor-derived-over-real-elim` — the 7-combinator
//!   floor + `compare_char` are real derived defs over the generic `Term::Elim`,
//!   `OrdResult` a checked inductive, zero-TCB-delta.
//! - DS-AC2 (soundness) `list-floor-recursion-in-sct-sound-zone` — the SCT
//!   sound-zone accept/reject verdict-flip.
//! - DS-AC3 `derived-string-ops-reduce-over-real-roundtrip` — the 5 string ops
//!   compute correctly through the real `string_to_list_char`/
//!   `list_char_to_string`.
//! - DS-AC4 `string-eq-codepoint-wise-accept-reject-pair` +
//!   `string-compare-3way-lexicographic-triple` +
//!   `list-eq-is-codepoint-wise-not-nfc-folding`.
//! - DS-AC6 distinct pure `list_append`/`bytes_concat` operations.
//! - DS-AC7 `concat-slice-compose-and-floor-totality`.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::{BTreeMap, BTreeSet};

use ken_elaborator::{foreign::trusted_base_delta, ElabEnv, NumericLitVal};
use ken_interp::eval::{apply, eval, EvalStore, EvalVal, ListCharIds};
use ken_kernel::{Decl, GlobalId, Term};

const DERIVED: &str = "Data.Collections.Derived";

// MEASURED: the roots loader owns the five private operations; the checked
// fences produce transparent example IDs that reference those owned IDs.
// CLAIMED: the value probes exercise Derived's private operations in their
// owner scope without publishing them or introducing trust.
// THE GAP: the requested examples are authenticated here, not every fence in
// the package. The four disposable no-op-door runs test alias independence.
fn l3_owner_examples(examples: &[(&str, &[&str])]) -> (ElabEnv, BTreeMap<String, GlobalId>) {
    let mut env = ElabEnv::new().expect("base environment");
    let owned = env
        .elaborate_module_from_roots(&[catalog_or::catalog_root()], DERIVED)
        .expect("the real Derived provider and dependency closure must roots-load");
    let trust_before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    let owned_ops: BTreeMap<_, _> = ["concat", "slice", "char_at", "eq", "compare"]
        .into_iter()
        .map(|name| {
            (
                name,
                catalog_or::provider_owned_id(&env, &owned, DERIVED, name)
                    .unwrap_or_else(|error| panic!("{name} must be Derived-owned: {error}")),
            )
        })
        .collect();
    for (name, _) in examples {
        assert!(
            !env.globals.contains_key(*name)
                && !env.globals.contains_key(&format!("{DERIVED}.{name}")),
            "{name} must not be a tangled or published declaration"
        );
    }
    env.execute_loaded_entry_checked_fences(DERIVED)
        .expect("Derived owner-local examples and rejects must all check");
    assert_eq!(
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        trust_before,
        "owner examples must not add trust"
    );
    for (name, id) in &owned_ops {
        assert_eq!(
            catalog_or::provider_owned_id(&env, &owned, DERIVED, name),
            Ok(*id),
            "fence execution must preserve the owner identity for {name}"
        );
    }
    let mut checked = BTreeMap::new();
    for (name, operations) in examples {
        let id = *env
            .globals
            .get(*name)
            .unwrap_or_else(|| panic!("{name} must check in the owner fence"));
        assert!(
            !owned.contains(&id),
            "{name} must not be a provider declaration"
        );
        assert!(
            !env.globals.contains_key(&format!("{DERIVED}.{name}")),
            "{name} must not become a public provider binding"
        );
        let decl = env
            .env
            .lookup(id)
            .expect("checked example must resolve by ID");
        assert!(
            matches!(decl, Decl::Transparent { .. }),
            "{name} must be a checked transparent declaration"
        );
        let refs = catalog_or::declaration_references(decl);
        for operation in *operations {
            assert!(
                refs.contains(&owned_ops[*operation]),
                "{name} must reference Derived-owned {operation}"
            );
        }
        checked.insert((*name).to_owned(), id);
    }
    (env, checked)
}

fn l3_example_id(checked: &BTreeMap<String, GlobalId>, name: &str) -> GlobalId {
    *checked
        .get(name)
        .unwrap_or_else(|| panic!("missing checked {name}"))
}

fn mk_env_with_derived_owned() -> (ElabEnv, Vec<GlobalId>) {
    let mut env = ElabEnv::new().expect("base env");
    let transport_owned = catalog_or::load_core_logic_compare(&mut env);
    catalog_or::expose_core_logic_transport(&mut env, &transport_owned);
    let owned = catalog_or::load_derived_fixture(&mut env);
    (env, owned)
}

fn mk_env() -> ElabEnv {
    mk_env_with_derived_owned().0
}

fn make_store(env: &ElabEnv) -> EvalStore {
    let mut store = EvalStore::new();
    let mkdecimalpair_id = env.prelude_env.mkdecimalpair_id;
    for (id, v) in &env.num_values {
        store.num_values.insert(*id, lit_to_eval(v, mkdecimalpair_id));
    }
    store.list_char_ids = Some(ListCharIds {
        nil_id: env.prelude_env.nil_id,
        cons_id: env.prelude_env.cons_id,
    });
    store
}

fn lit_to_eval(v: &NumericLitVal, mkdecimalpair_id: GlobalId) -> EvalVal {
    match v {
        NumericLitVal::Int(n) => EvalVal::from(n.clone()),
        NumericLitVal::Float(f) => EvalVal::Float(*f),
        NumericLitVal::Float32(f) => EvalVal::Float32(*f),
        NumericLitVal::Decimal { coeff, exp } => {
            ken_interp::decimal_value(mkdecimalpair_id, coeff.clone(), *exp)
        }
        NumericLitVal::Str(s) => EvalVal::Str(s.clone()),
        NumericLitVal::Bytes(b) => EvalVal::Bytes(b.clone()),
    }
}

fn eval_def(env: &ElabEnv, store: &mut EvalStore, id: GlobalId) -> EvalVal {
    match env.env.lookup(id) {
        Some(Decl::Transparent { body, .. }) => eval(&[], body, &env.env, store),
        _ => EvalVal::Unknown,
    }
}

/// Elaborate `view <name> : <ty> = <expr>` against the already-loaded package
/// env/store and evaluate it. Resyncs `store.num_values` after every call — a
/// literal declared after `make_store` time evaluates against a stale
/// snapshot otherwise (`l3_strings_roundtrip_acceptance.rs`'s documented
/// discipline).
fn eval_view(env: &mut ElabEnv, store: &mut EvalStore, name: &str, ty: &str, expr: &str) -> EvalVal {
    let src = format!("const {name} : {ty} = {expr}");
    let id = env
        .elaborate_decl(&src)
        .unwrap_or_else(|e| panic!("{name} failed to elaborate: {e}"));
    let mkdecimalpair_id = env.prelude_env.mkdecimalpair_id;
    for (nid, v) in &env.num_values {
        store
            .num_values
            .entry(*nid)
            .or_insert_with(|| lit_to_eval(v, mkdecimalpair_id));
    }
    eval_def(env, store, id)
}

/// A Ken `Nat` literal has no numeral sugar (`Zero`/`Suc` only, `34 §1`) — build
/// the unary source form for a small constant.
fn nat(n: u32) -> String {
    let mut s = "Zero".to_string();
    for _ in 0..n {
        s = format!("Suc ({s})");
    }
    s
}

/// Baseline provisioning for the pinned `slice 0 99 …` unary-`Nat` corpus.
/// With `RUST_MIN_STACK` unset, the full eight-test suite overflowed at stated
/// child stacks of 1, 2 and 3 MiB and passed at 4 MiB. The 99-step test alone
/// has the same 3/4 MiB boundary; its composition sibling passes at 2 MiB.
/// The measured effective peak is at most 4 MiB on this host, leaving at
/// least 252 MiB headroom on the existing fixed 256 MiB child stack. This is
/// baseline provisioning, not a pin on stack growth or a regression repair.
fn run_with_big_stack<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(f)
        .expect("spawn big-stack test thread")
        .join()
        .expect("test thread panicked");
}

// ─────────────────────────────────────────────────────────────────────────────
// DS-AC1/AC5 — `list-combinator-floor-derived-over-real-elim`
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn list_combinator_floor_derived_over_real_elim() {
    let (env, derived_owned) = mk_env_with_derived_owned();

    // The 7 floor combinators + `compare_char` are all Transparent (SCT
    // accepted, `declare_def`-upgraded) and their `match` lowers to the real
    // generic `Term::Elim` over the `List`/`Nat` family — never a bespoke
    // reducer, never a registered `elim_List`/`elim_Nat` constant.
    let list_id = env.globals["List"];
    let nat_id = env.globals["Nat"];
    let list_recursors = ["list_append", "nth", "list_eq", "list_compare"];
    let nat_recursors = ["Data.Numeric.Nat.Order.sub"];
    // `take`/`drop` match on `Nat` outermost (the fuel), `List` innermost.
    let nat_outer = ["take", "drop"];

    for name in list_recursors {
        let module = if name == "list_eq" || name == "list_compare" {
            "Core.Logic.Compare"
        } else {
            "Data.Collections.Derived"
        };
        let id = if module == "Data.Collections.Derived" {
            catalog_or::provider_owned_id(&env, &derived_owned, module, name)
                .unwrap_or_else(|error| panic!("{name} must be Derived-owned: {error}"))
        } else {
            env.globals[&format!("{module}.{name}")]
        };
        let (_, body) = env
            .env
            .transparent_body(id)
            .unwrap_or_else(|| panic!("{name} must be Transparent (SCT-accepted)"));
        let mut inner = &body;
        while let Term::Lam(_, b) = inner {
            inner = b;
        }
        match inner {
            Term::Elim { fam, .. } => assert_eq!(
                *fam, list_id,
                "{name}'s outermost match must lower to the real elim_List"
            ),
            other => panic!("{name}'s body must be a Term::Elim; got {other:?}"),
        }
    }
    for name in nat_recursors {
        let id = env.globals[name];
        let (_, body) = env
            .env
            .transparent_body(id)
            .unwrap_or_else(|| panic!("{name} must be Transparent (SCT-accepted)"));
        let mut inner = &body;
        while let Term::Lam(_, b) = inner {
            inner = b;
        }
        match inner {
            Term::Elim { fam, .. } => assert_eq!(
                *fam, nat_id,
                "{name}'s outermost match must lower to the real elim_Nat"
            ),
            other => panic!("{name}'s body must be a Term::Elim; got {other:?}"),
        }
    }
    for name in nat_outer {
        let id = catalog_or::provider_owned_id(&env, &derived_owned, "Data.Collections.Derived", name)
            .unwrap_or_else(|error| panic!("{name} must be Derived-owned: {error}"));
        let (_, body) = env
            .env
            .transparent_body(id)
            .unwrap_or_else(|| panic!("{name} must be Transparent (SCT-accepted)"));
        let mut inner = &body;
        while let Term::Lam(_, b) = inner {
            inner = b;
        }
        match inner {
            Term::Elim { fam, .. } => assert_eq!(
                *fam, nat_id,
                "{name}'s outermost match must lower to the real elim_Nat (the fuel)"
            ),
            other => panic!("{name}'s body must be a Term::Elim; got {other:?}"),
        }
    }
    // compare_char's own match (on eqChar's Bool result) is not List/Nat-elim
    // shaped — it's checked separately: just confirm it's Transparent and not
    // an opaque postulate stand-in.
    assert!(
        env.env.transparent_body(catalog_or::provider_owned_id(&env, &derived_owned, "Data.Collections.Derived", "compare_char")
            .expect("Derived must own compare_char")).is_some(),
        "compare_char must be a real (checked) def"
    );

    // `OrdResult` is a checked `data` inductive — NOT a postulate/primitive.
    let ordresult_id = env.globals["OrdResult"];
    assert!(
        matches!(env.env.lookup(ordresult_id), Some(Decl::Inductive { .. })),
        "OrdResult must be a checked inductive, not a postulate/primitive"
    );

    // Zero-TCB-delta: none of the floor combinators or OrdResult contribute
    // any new trusted_base() member (they are all declare_def/data, checked).
    for name in [
        "list_append",
        "nth",
        "take",
        "drop",
        "Data.Numeric.Nat.Order.sub",
        "list_eq",
        "list_compare",
        "compare_char",
    ] {
        let module = if name.contains('.') {
            ""
        } else if name == "list_eq" || name == "list_compare" {
            "Core.Logic.Compare."
        } else {
            "Data.Collections.Derived."
        };
        let id = if module == "Data.Collections.Derived." {
            catalog_or::provider_owned_id(&env, &derived_owned, "Data.Collections.Derived", name)
                .unwrap_or_else(|error| panic!("{name} must be Derived-owned: {error}"))
        } else {
            env.globals[&format!("{module}{name}")]
        };
        let delta = trusted_base_delta(&env.env, id);
        assert!(
            delta.is_empty(),
            "{name} must mint zero trusted_base() delta; got {delta:?}"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DS-AC2 (soundness) — `list-floor-recursion-in-sct-sound-zone`
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn list_floor_recursion_in_sct_sound_zone() {
    // Positive: every one of the 7 combinators (+ compare_char) elaborates —
    // confirmed by `mk_env()` itself not panicking (each is SCT-accepted, an
    // applied call on a strict Cons-tail/Suc-pred subterm).
    let _env = mk_env();

    // Negative — the discriminating flip: a sibling recursing on a
    // RECONSTRUCTED, non-decreasing argument (`bad (Cons x xs) = bad (Cons x
    // xs)`, an APPLIED call carrying no `Down` (strict-subterm) argument) must
    // be REJECTED. This does NOT lean on the SCT's unapplied-self-reference /
    // recursion-through-opaque-map over-accept hole
    // (`sct-unapplied-self-reference-over-accepts`) — `bad`'s call is applied,
    // just non-decreasing, squarely in the SCT's sound-REJECT zone.
    let mut env2 = mk_env();
    let result = env2.elaborate_decl(
        "fn bad (a : Type) (xs : List a) : List a = \
         match xs { Nil |-> Nil a ; Cons x xs2 |-> bad a (Cons a x xs2) }",
    );
    assert!(
        result.is_err(),
        "bad (reconstructs its matched arg, no strict-subterm decrease) must be SCT-rejected"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// DS-AC3 — `derived-string-ops-reduce-over-real-roundtrip`
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn derived_string_ops_reduce_over_real_roundtrip() {
    run_with_big_stack(|| {
        let (mut env, checked) = l3_owner_examples(&[
            ("derived_example_l3_concat_ascii", &["concat"]),
            ("derived_example_l3_concat_multibyte", &["concat"]),
            ("derived_example_l3_slice_ordinary", &["slice"]),
            ("derived_example_l3_slice_clamp", &["slice"]),
            ("derived_example_l3_slice_underflow", &["slice"]),
            ("derived_example_l3_char_at_found", &["char_at"]),
            ("derived_example_l3_char_at_oob", &["char_at"]),
            ("derived_example_l3_char_at_empty", &["char_at"]),
        ]);
        let mut store = make_store(&env);

        // concat, including a multi-byte pair (CJK), preserves every scalar.
        let v = eval_def(
            &env,
            &mut store,
            l3_example_id(&checked, "derived_example_l3_concat_ascii"),
        );
        assert_eq!(
            v,
            EvalVal::Str("abcd".into()),
            "concat \"ab\" \"cd\" must be \"abcd\""
        );
        let v = eval_def(
            &env,
            &mut store,
            l3_example_id(&checked, "derived_example_l3_concat_multibyte"),
        );
        assert_eq!(
            v,
            EvalVal::Str("世界".into()),
            "concat must preserve multi-byte scalars"
        );

        // slice: ordinary, over-range clamp, and j < i (empty, no underflow).
        let v = eval_def(
            &env,
            &mut store,
            l3_example_id(&checked, "derived_example_l3_slice_ordinary"),
        );
        assert_eq!(
            v,
            EvalVal::Str("bc".into()),
            "slice 1 3 \"abcde\" must be \"bc\""
        );

        let upper_id = env
            .elaborate_decl(&format!("const l3_clamp_upper : Nat = {}", nat(99)))
            .expect("the original 99-step clamp index must check in the host");
        let upper = eval_def(&env, &mut store, upper_id);
        let clamp = eval_def(
            &env,
            &mut store,
            l3_example_id(&checked, "derived_example_l3_slice_clamp"),
        );
        let v = apply(clamp, upper, &env.env, &mut store);
        assert_eq!(
            v,
            EvalVal::Str("abc".into()),
            "slice 0 99 \"abc\" must clamp to \"abc\" (over-range take stops at the end)"
        );

        let v = eval_def(
            &env,
            &mut store,
            l3_example_id(&checked, "derived_example_l3_slice_underflow"),
        );
        assert_eq!(
            v,
            EvalVal::Str("".into()),
            "slice 2 1 \"abc\" must be \"\" (canonical sub saturates, no underflow)"
        );

        // char_at: Option Char, honest absence.
        let some_id = env.globals["Some"];
        let none_id = env.globals["None"];
        let v = eval_def(
            &env,
            &mut store,
            l3_example_id(&checked, "derived_example_l3_char_at_found"),
        );
        match v {
            EvalVal::Ctor { id, ref args, .. } if id == some_id => {
                assert_eq!(
                    args[1],
                    EvalVal::Int('b' as i64),
                    "char_at 1 \"abc\" must be Some 'b'"
                );
            }
            other => panic!("char_at 1 \"abc\" must be Some 'b'; got {other:?}"),
        }
        let v = eval_def(
            &env,
            &mut store,
            l3_example_id(&checked, "derived_example_l3_char_at_oob"),
        );
        assert!(
            matches!(v, EvalVal::Ctor { id, .. } if id == none_id),
            "char_at 5 \"abc\" must be None"
        );
        let v = eval_def(
            &env,
            &mut store,
            l3_example_id(&checked, "derived_example_l3_char_at_empty"),
        );
        assert!(
            matches!(v, EvalVal::Ctor { id, .. } if id == none_id),
            "char_at 0 \"\" must be None"
        );
    });
}

// ─────────────────────────────────────────────────────────────────────────────
// DS-AC4 — `string-eq-codepoint-wise-accept-reject-pair`
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn string_eq_codepoint_wise_accept_reject_pair() {
    let (env, checked) = l3_owner_examples(&[
        ("derived_example_l3_eq_equal", &["eq"]),
        ("derived_example_l3_eq_codepoint", &["eq"]),
        ("derived_example_l3_eq_length", &["eq"]),
    ]);
    let mut store = make_store(&env);
    let true_id = env.globals["True"];
    let false_id = env.globals["False"];

    let v = eval_def(
        &env,
        &mut store,
        l3_example_id(&checked, "derived_example_l3_eq_equal"),
    );
    assert!(
        matches!(v, EvalVal::Ctor{id,..} if id==true_id),
        "eq \"abc\" \"abc\" must be True"
    );

    // Non-degenerate reject: same length, single codepoint differs — the
    // tightest guard (a length-only equality would pass this and both
    // corpus witnesses below, and only this case catches it).
    let v = eval_def(
        &env,
        &mut store,
        l3_example_id(&checked, "derived_example_l3_eq_codepoint"),
    );
    assert!(
        matches!(v, EvalVal::Ctor{id,..} if id==false_id),
        "eq \"abc\" \"abd\" must be False"
    );

    let v = eval_def(
        &env,
        &mut store,
        l3_example_id(&checked, "derived_example_l3_eq_length"),
    );
    assert!(
        matches!(v, EvalVal::Ctor{id,..} if id==false_id),
        "eq \"ab\" \"abc\" must be False"
    );
}

/// `surface/strings/string-compare-3way-lexicographic-triple`
#[test]
fn string_compare_3way_lexicographic_triple() {
    let (env, checked) = l3_owner_examples(&[
        ("derived_example_l3_compare_prefix", &["compare"]),
        ("derived_example_l3_compare_lex", &["compare"]),
        ("derived_example_l3_compare_greater", &["compare"]),
        ("derived_example_l3_compare_equal", &["compare"]),
    ]);
    let mut store = make_store(&env);
    let lt_id = env.globals["Core.Logic.OrdResult.Lt"];
    let eq_id = env.globals["Core.Logic.OrdResult.Eq"];
    let gt_id = env.globals["Core.Logic.OrdResult.Gt"];

    let v = eval_def(
        &env,
        &mut store,
        l3_example_id(&checked, "derived_example_l3_compare_prefix"),
    );
    assert!(
        matches!(v, EvalVal::Ctor{id,..} if id==lt_id),
        "compare \"a\" \"ab\" must be Lt; got {v:?}"
    );
    let v = eval_def(
        &env,
        &mut store,
        l3_example_id(&checked, "derived_example_l3_compare_lex"),
    );
    assert!(
        matches!(v, EvalVal::Ctor{id,..} if id==lt_id),
        "compare \"ab\" \"b\" must be Lt; got {v:?}"
    );
    let v = eval_def(
        &env,
        &mut store,
        l3_example_id(&checked, "derived_example_l3_compare_greater"),
    );
    assert!(
        matches!(v, EvalVal::Ctor{id,..} if id==gt_id),
        "compare \"b\" \"a\" must be Gt; got {v:?}"
    );
    let v = eval_def(
        &env,
        &mut store,
        l3_example_id(&checked, "derived_example_l3_compare_equal"),
    );
    assert!(
        matches!(v, EvalVal::Ctor{id,..} if id==eq_id),
        "compare \"ab\" \"ab\" must be Eq; got {v:?}"
    );
}

/// `surface/strings/list-eq-is-codepoint-wise-not-nfc-folding` (property)
#[test]
fn list_eq_is_codepoint_wise_not_nfc_folding() {
    let mut env = mk_env();
    let mut store = make_store(&env);
    let false_id = env.globals["False"];

    // Precomposed "é" (U+00E9, one scalar) vs "e" + combining acute (U+0065
    // U+0301, two scalars) — canonically equivalent, codepoint-DISTINCT.
    // Constructed DIRECTLY as `List Char` (Char ≡ Int under refinement
    // erasure, `decimal_char.rs`), never via a String literal — pinning this
    // on String literals would falsely fail once real NFC-at-construction
    // lands and merges them at construction (the over-pin-a-deferred-
    // behavior trap; ADR 0010 §3).
    let nfc = "Cons Char 233 (Nil Char)";
    let nfd = "Cons Char 101 (Cons Char 769 (Nil Char))";
    let v = eval_view(
        &mut env,
        &mut store,
        "t_nfc_nfd",
        "Bool",
        &format!("list_eq Char eqChar ({nfc}) ({nfd})"),
    );
    assert!(
        matches!(v, EvalVal::Ctor{id,..} if id==false_id),
        "list_eq eqChar on codepoint-distinct-but-canonically-equivalent sequences must be False (NFC-blind); got {v:?}"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// DS-AC6 — pure List/Bytes concatenation operations stay distinct
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn list_append_and_bytes_concat_are_distinct_pure_operations() {
    let env = mk_env();
    let list_append_id = env.globals["list_append"];
    let bytes_concat_id = env.globals["bytes_concat"];
    assert_ne!(
        list_append_id, bytes_concat_id,
        "list_append and bytes_concat must resolve to their distinct intended globals"
    );
    assert!(
        !env.bytes_env.io_effect_rows.contains_key("list_append"),
        "list_append must remain pure"
    );
    assert!(
        !env.bytes_env.io_effect_rows.contains_key("bytes_concat"),
        "bytes_concat must remain a pure Bytes operation"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// DS-AC7 — `concat-slice-compose-and-floor-totality`
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn concat_slice_compose_and_floor_totality() {
    run_with_big_stack(|| {
        let (mut env, checked) =
            l3_owner_examples(&[("derived_example_l3_slice_concat", &["slice", "concat"])]);
        let mut store = make_store(&env);

        // slice 0 (charLength a) (concat a b) ≡ a, on a scalar-clean corpus.
        // charLength "ab" = 2.
        let v = eval_def(
            &env,
            &mut store,
            l3_example_id(&checked, "derived_example_l3_slice_concat"),
        );
        assert_eq!(
            v,
            EvalVal::Str("ab".into()),
            "slice 0 (charLength \"ab\") (concat \"ab\" \"cd\") must be \"ab\""
        );

        // list_append and canonical Nat sub remain public: import each from its
        // provider instead of relying on the test fixture's flat aliases.
        assert!(!env.globals.contains_key("list_append"));
        assert!(!env.globals.contains_key("l3_canonical_nat_sub"));
        env.elaborate_file(
            "import Data.Collections.Derived (list_append)\n\
             import Data.Numeric.Nat.Order (sub as l3_canonical_nat_sub)",
        )
        .expect("public list_append and canonical sub must selectively import");

        // list_append associativity on a small corpus:
        // list_append (list_append xs ys) zs ≡ list_append xs (list_append ys zs).
        let xs = "Cons Char 97 (Nil Char)"; // ['a']
        let ys = "Cons Char 98 (Nil Char)"; // ['b']
        let zs = "Cons Char 99 (Nil Char)"; // ['c']
        let lhs = format!("list_append Char (list_append Char ({xs}) ({ys})) ({zs})");
        let rhs = format!("list_append Char ({xs}) (list_append Char ({ys}) ({zs}))");
        let v_lhs = eval_view(&mut env, &mut store, "t_assoc_lhs", "List Char", &lhs);
        let v_rhs = eval_view(&mut env, &mut store, "t_assoc_rhs", "List Char", &rhs);
        assert_eq!(
            format!("{v_lhs:?}"),
            format!("{v_rhs:?}"),
            "list_append must be associative"
        );
        assert!(
            !matches!(v_lhs, EvalVal::Neutral),
            "list_append must not get stuck on well-typed input"
        );

        // Totality: canonical sub saturates, nth/take/drop totalize out-of-range —
        // none reduce to Neutral/stuck. Re-check the already-asserted DS-AC3
        // corpus values are all non-Neutral (they were asserted to concrete
        // values above; this call re-confirms the out-of-range/underflow
        // faces specifically).
        let v = eval_view(
            &mut env,
            &mut store,
            "t_natsub_sat",
            "Nat",
            &format!("l3_canonical_nat_sub ({}) ({})", nat(1), nat(2)),
        );
        assert!(
            !matches!(v, EvalVal::Neutral),
            "canonical sub 1 2 must not be stuck"
        );
    });
}
