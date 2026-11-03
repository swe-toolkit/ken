//! Window length bound: spec/50-stdlib/69-time-and-events.md §12.
//! Source: conformance/stdlib/time-and-events/seed-window-length-bound.md.
//! Promise class: durable invariant; closed values supplement the generic law.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, ImportKind, KenMdFenceRole,
};
use ken_kernel::{Decl, KernelError};

const DERIVED: &str = "Data.Collections.Derived";
const TRACE: &str = "Data.Time.EventTrace";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/time-and-events/seed-window-length-bound.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

// IsTrue already belongs to the isolated compiler base. Remove only a
// redundant, unaliased IsTrue selective import from parsed Ken declarations;
// no seed line spelling or incidental whitespace is an oracle.
fn omit_redundant_is_true(source: &str) -> String {
    let mut edits = Vec::new();
    for declaration in parse_decls(source).expect("seed Ken parses") {
        if let SourceDecl::ImportDecl {
            module,
            kind: ImportKind::Selective(items),
            span,
        } = declaration
        {
            if module != "Core.Classes.LawfulClasses" {
                continue;
            }
            let mut keep = Vec::new();
            let mut removed = false;
            for item in items {
                if item.name == "IsTrue" && item.rename.is_none() {
                    removed = true;
                } else {
                    keep.push(match item.rename {
                        Some(alias) => format!("{} as {alias}", item.name),
                        None => item.name,
                    });
                }
            }
            if removed {
                let replacement = if keep.is_empty() {
                    String::new()
                } else {
                    format!("import {module} ({})", keep.join(", "))
                };
                edits.push((span.start..span.end, replacement));
            }
        }
    }
    let mut client = source.to_owned();
    for (range, replacement) in edits.into_iter().rev() {
        client.replace_range(range, &replacement);
    }
    client
}

fn seed_programs() -> Vec<String> {
    extract_ken_md(SEED)
        .expect("conformance Ken fences are well-formed")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| omit_redundant_is_true(&SEED[fence.body_range]))
        .collect()
}

fn loaded_trace() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace roots-loads from the checked Derived provider");
    env
}

// The public §12 type and Derived's filter::length, restated on arbitrary
// client inputs through both proof spellings. A weakened or swapped
// contract cannot pass by keeping the same name.
const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Data.Collections.Derived (filter, length)
import Data.Time.EventTrace (Event, trace_window, trace_window_length_bound)

theorem g_filter_attached (a : Type) (q : a → Bool) (xs : List a) :
  IsTrue (leq_nat (length a (filter a q xs)) (length a xs)) =
  (proof length for filter) a q xs
theorem g_filter_path (a : Type) (q : a → Bool) (xs : List a) :
  IsTrue (leq_nat (length a (filter a q xs)) (length a xs)) =
  filter::length a q xs
theorem g_window (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p)) :
  IsTrue (leq_nat (length (Event p) (trace_window p lo hi tr)) (length (Event p) tr)) =
  trace_window_length_bound p lo hi tr
"#;

/// SOURCE: spec/50-stdlib/69-time-and-events.md §12.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after explicit providers, loading Derived and then EventTrace
/// preserves the exact trusted GlobalId identity set. filter::length, its
/// private step helper and trace_window_length_bound belong to their owners
/// with transparent bodies. An independent client checks the generic types
/// through both filter::length spellings, and the helper stays private.
/// CLAIMED: the §12 bound is a checked, public, premise-free law that adds
/// no trust.
/// THE GAP: imported assumptions remain inherited; closed window lengths
/// are checked by the seed and the controls below.
#[test]
fn generic_window_length_bound_checks_without_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Function.Combinators",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Core.Logic.Compare",
        "Core.Classes.LawfulClasses",
        "Core.Logic.And",
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Logic.OrdResult",
        "Core.Logic.Transport",
        "Data.Collections.List",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-load: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let derived_owned = env
        .elaborate_module_from_roots(&[root()], DERIVED)
        .expect("Derived's filter::length roots-loads");
    let after_derived = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_derived, before, "Derived adds no trusted GlobalId");
    let trace_owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace's §12 law kernel-checks");
    let after_trace = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, Derived={}, EventTrace={}",
        before.len(),
        after_derived.len(),
        after_trace.len()
    );
    assert_eq!(
        after_trace, after_derived,
        "EventTrace adds no trusted GlobalId"
    );
    for (module, owned, name) in [
        (DERIVED, &derived_owned, "filter::length"),
        (DERIVED, &derived_owned, "filter_keep_step_length"),
        (TRACE, &trace_owned, "trace_window_length_bound"),
    ] {
        let id = env.globals[&format!("{module}.{name}")];
        assert!(owned.contains(&id), "{name} must belong to {module}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must carry a checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("the §12 type and both filter::length spellings check independently");
    let mut derived_only = ElabEnv::new().expect("compiler base");
    derived_only
        .elaborate_module_from_roots(&[root()], DERIVED)
        .expect("Derived roots-loads alone");
    match derived_only.elaborate_file(&format!("import {DERIVED} (filter_keep_step_length)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert!(name.ends_with("filter_keep_step_length"), "{name}");
        }
        other => panic!("filter_keep_step_length must remain private: {other:?}"),
    }
}

/// SOURCE: conformance/stdlib/time-and-events/seed-window-length-bound.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the seed has exactly three Ken source fences. Its positive
/// client checks the law instance on the e1/e4/e7 trace under [3,9), the
/// two-event lower bound and the [0,9) reverse-bound control. Independent
/// closed controls check the exact window length 2.
/// CLAIMED: §12's law applies to outside clients and the closed window
/// lengths match independently stated values.
/// THE GAP: closed instances cannot establish the arbitrary-input bound;
/// the generic checked body and client types are tested above.
#[test]
fn closed_seed_checks_window_length_cases() {
    let programs = seed_programs();
    assert_eq!(programs.len(), 3, "one positive client and two refusals");
    loaded_trace()
        .elaborate_file(&programs[0])
        .expect("§12 source-shaped positive client checks");
    loaded_trace()
        .elaborate_file(CLOSED_CONTROLS)
        .expect("independent closed window lengths check");
}

fn expect_false_theorem_rejected(env: &mut ElabEnv, negative: &str, label: &str) {
    match env.elaborate_file(negative) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            let in_theorem = parse_decls(negative)
                .expect("negative is valid Ken")
                .into_iter()
                .any(|decl| {
                    matches!(decl, SourceDecl::TheoremDecl { span: theorem, .. }
                        if span.start >= theorem.start && span.end <= theorem.end && span.start < span.end)
                });
            assert!(
                in_theorem,
                "{label}: K0004 must target its false theorem: {span:?}"
            );
        }
        other => panic!("{label}: expected kernel TypeMismatch, got {other:?}"),
    }
}

// Independent closed controls prevent a future seed edit from silently
// dropping a refusal. The exact length 2 is stated as an equality here.
const CLOSED_CONTROLS: &str = r#"
import Core.Classes.LawfulClasses (leq_nat)
import Data.Collections.List (length)
import Data.Time.EventTrace (Event, MkEvent, trace_window, trace_window_length_bound)

const t0 : Nat = Zero
const t1 : Nat = Suc t0
const t2 : Nat = Suc t1
const t3 : Nat = Suc t2
const t4 : Nat = Suc t3
const t5 : Nat = Suc t4
const t6 : Nat = Suc t5
const t7 : Nat = Suc t6
const t8 : Nat = Suc t7
const t9 : Nat = Suc t8
const e1 : Event Bool = MkEvent Bool t1 True
const e4 : Event Bool = MkEvent Bool t4 False
const e7 : Event Bool = MkEvent Bool t7 True
const tr : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e4 (Cons (Event Bool) e7 (Nil (Event Bool))))

theorem window_bound :
  IsTrue (leq_nat (length (Event Bool) (trace_window Bool t3 t9 tr)) (length (Event Bool) tr)) =
  trace_window_length_bound Bool t3 t9 tr
theorem window_keeps_two :
  Equal Nat (length (Event Bool) (trace_window Bool t3 t9 tr)) t2 = Proved
theorem control_reverse_proved :
  IsTrue (leq_nat (length (Event Bool) tr) (length (Event Bool) (trace_window Bool t0 t9 tr))) = Proved
"#;

const FALSE_KEEPS_THREE: &str = r#"
theorem window_keeps_three_wrong :
  Equal Nat (length (Event Bool) (trace_window Bool t3 t9 tr)) t3 = Proved
"#;
const FALSE_REVERSE: &str = r#"
theorem refuse_reverse_proved :
  IsTrue (leq_nat (length (Event Bool) tr) (length (Event Bool) (trace_window Bool t3 t9 tr))) = Proved
"#;

/// SOURCE: §12 seed's two negative Ken fences.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the full positive seed compiles before each of its separate
/// false theorem submissions. Two independent closed refusals also run
/// after the checked controls. Every failure is KernelRejected(TypeMismatch)
/// within its own theorem.
/// CLAIMED: the [3,9) window does not hold three events, and the trace's
/// length is not at most the window's.
/// THE GAP: these fixed false claims do not characterize arbitrary traces;
/// the generic law is separately checked.
#[test]
fn false_window_length_claims_reject_at_their_own_theorems() {
    let programs = seed_programs();
    let (positive, negatives) = programs.split_first().expect("seed has a positive client");
    assert_eq!(negatives.len(), 2, "the seed has two refusals");
    for negative in negatives {
        let mut env = loaded_trace();
        env.elaborate_file(positive)
            .expect("positive seed client checks before its negative");
        expect_false_theorem_rejected(&mut env, negative, "seed refusal");
    }
    for (negative, label) in [
        (FALSE_KEEPS_THREE, "three-event window"),
        (FALSE_REVERSE, "reverse bound"),
    ] {
        let mut env = loaded_trace();
        env.elaborate_file(CLOSED_CONTROLS)
            .expect("independent closed controls check before refusal");
        expect_false_theorem_rejected(&mut env, negative, label);
    }
}
