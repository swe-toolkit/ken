//! Finite timestamped trace clients: spec/50-stdlib/69-time-and-events.md §1.
//! Source: conformance/stdlib/time-and-events/seed-event-trace.md.
//! Promise class: durable invariant. Separately spelled expected event lists
//! check half-open windows, left-biased ties and unsorted-input semantics;
//! generic restatements require checked universal law types.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const DERIVED: &str = "Data.Collections.Derived";
const TRACE: &str = "Data.Time.EventTrace";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace roots-loads from the checked Derived provider");
    env
}

/// SOURCE: spec/50-stdlib/69-time-and-events.md §§1.3–1.4.
/// PROMISE CLASS: durable invariant.
/// MEASURED: exact trusted-GlobalId inventories before and after loading
/// Derived and then EventTrace are equal; all added generic Derived proofs
/// and three EventTrace laws are transparent and package-owned. Generic
/// clients independently restate the filtering/count and all trace law types.
/// CLAIMED: no new trust and the generic order and count propositions are
/// checked for arbitrary payloads, bounds, input traces and classifier.
/// THE GAP: equality retains inherited assumptions; closed operational
/// outcomes are checked separately by the source-shaped seed client below.
#[test]
fn generic_trace_and_derived_laws_check_without_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Function.Combinators",
        "Core.Logic.Transport",
        "Core.Logic.Compare",
        "Core.Logic.And",
        "Core.Logic.Or",
        "Core.Logic.OrdResult",
        "Core.Classes.LawfulClasses",
        "Data.Collections.List",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|err| panic!("{provider} roots-load: {err:?}"));
    }
    let baseline = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let derived_owned = env
        .elaborate_module_from_roots(&[root()], DERIVED)
        .expect("Derived's generic checked list proofs roots-load");
    let after_derived = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_derived, baseline, "Derived adds no trusted GlobalId");
    for name in [
        "starts_after",
        "sorted_cons",
        "sorted_tail",
        "sorted_head",
        "filter_sorted",
        "count_cons",
        "count_left_first",
        "count_right_first",
    ] {
        let id = env.globals[&format!("{DERIVED}.{name}")];
        assert!(derived_owned.contains(&id), "{name} must belong to Derived");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must carry a checked body"
        );
    }
    let trace_owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace's three laws kernel-check");
    let after_trace = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "trusted_base() count: before={}, Derived={}, EventTrace={}",
        baseline.len(),
        after_derived.len(),
        after_trace.len()
    );
    assert_eq!(
        after_trace, after_derived,
        "EventTrace adds no trusted GlobalId"
    );
    for name in [
        "trace_window_ordered",
        "trace_merge_ordered",
        "trace_merge_count",
    ] {
        let id = env.globals[&format!("{TRACE}.{name}")];
        assert!(
            trace_owned.contains(&id),
            "{name} must belong to EventTrace"
        );
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must carry a checked proof body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("all three §1.3 law types and two generic list proofs check independently");
}

// Restate generic types independently of package law declarations, including
// a classifier with no payload equality or sorted-input premise.
const GENERIC_CLIENT: &str = r##"
import Data.Collections.Derived (count, filter, filter_sorted, count_cons)
import Data.Time.EventTrace
  (Event, event_before, event_in_window, time_ordered, trace_window,
   trace_merge, trace_window_ordered, trace_merge_ordered, trace_merge_count)
import Data.Numeric.Nat.Arithmetic (add)

theorem client_filter_sorted
  (a : Type) (le : a → a → Bool)
  (le_trans : (x : a) → (y : a) → (z : a) →
    Equal Bool (le x y) True → Equal Bool (le y z) True →
    Equal Bool (le x z) True)
  (keep : a → Bool) (xs : List a) :
  is_sorted a le xs → is_sorted a le (filter a keep xs) =
  filter_sorted a le le_trans keep xs

theorem client_count_cons
  (a : Type) (eqf : a → a → Bool) (q : a) (x : a) (xs : List a) :
  Equal Nat (count a eqf q (Cons a x xs))
    (add (count a eqf q xs) (count a eqf q (Cons a x (Nil a)))) =
  count_cons a eqf q x xs

theorem client_trace_window_ordered
  (p : Type) (lo : Nat) (hi : Nat) (tr : List (Event p)) :
  time_ordered p tr → time_ordered p (trace_window p lo hi tr) =
  trace_window_ordered p lo hi tr

theorem client_trace_merge_ordered
  (p : Type) (xs : List (Event p)) :
  (ys : List (Event p)) → time_ordered p xs → time_ordered p ys →
  time_ordered p (trace_merge p xs ys) =
  trace_merge_ordered p xs

theorem client_trace_merge_count
  (p : Type) (eqf : Event p → Event p → Bool)
  (q : Event p) (xs : List (Event p)) :
  (ys : List (Event p)) →
  Equal Nat (count (Event p) eqf q (trace_merge p xs ys))
    (add (count (Event p) eqf q xs) (count (Event p) eqf q ys)) =
  trace_merge_count p eqf q xs
"##;

// The conformance seed's positive code block and 22 assertions are exact,
// including the independent expected lists and sorted-input witnesses.
const SEED_POSITIVE: &str = r##"
import Data.Time.EventTrace
  (Event, MkEvent, event_before, event_in_window, event_payload,
   time_ordered, trace_window, trace_merge, trace_window_ordered,
   trace_merge_ordered, trace_merge_count)
import Data.Collections.Derived (count)
import Data.Numeric.Nat.Arithmetic (add)

const t1 : Nat = Suc Zero
const t2 : Nat = Suc t1
const t3 : Nat = Suc t2
const t4 : Nat = Suc t3

const e1 : Event Bool = MkEvent Bool t1 True
const e2 : Event Bool = MkEvent Bool t2 False
const e3 : Event Bool = MkEvent Bool t3 False
const e3b : Event Bool = MkEvent Bool t3 True

const left : List (Event Bool) =
  Cons (Event Bool) e1 (Cons (Event Bool) e3 (Nil (Event Bool)))
const right : List (Event Bool) =
  Cons (Event Bool) e2 (Cons (Event Bool) e3b (Nil (Event Bool)))
const ordered_input : List (Event Bool) =
  Cons (Event Bool) e1
    (Cons (Event Bool) e2
      (Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool)))))
const merged : List (Event Bool) =
  Cons (Event Bool) e1
    (Cons (Event Bool) e2
      (Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool)))))
const window_half_open : List (Event Bool) =
  Cons (Event Bool) e2 (Nil (Event Bool))
const window_keeps_tie : List (Event Bool) =
  Cons (Event Bool) e2
    (Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool))))
const window_keeps_unordered : List (Event Bool) =
  Cons (Event Bool) e3 (Cons (Event Bool) e1 (Nil (Event Bool)))
const scrambled_left : List (Event Bool) =
  Cons (Event Bool) e3 (Cons (Event Bool) e1 (Nil (Event Bool)))
const scrambled_right : List (Event Bool) =
  Cons (Event Bool) e3b (Cons (Event Bool) e2 (Nil (Event Bool)))
const merged_unordered : List (Event Bool) =
  Cons (Event Bool) e3
    (Cons (Event Bool) e1
      (Cons (Event Bool) e3b (Cons (Event Bool) e2 (Nil (Event Bool)))))
const tie_lr : List (Event Bool) =
  Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool)))
const tie_rl : List (Event Bool) =
  Cons (Event Bool) e3b (Cons (Event Bool) e3 (Nil (Event Bool)))

fn payload_flag (q : Event Bool) (e : Event Bool) : Bool =
  event_payload Bool e

-- Both endpoints are tested with the same half-open window.
theorem window_lower_included :
  Equal Bool (event_in_window Bool t2 t3 e2) True = Proved
theorem window_upper_excluded :
  Equal Bool (event_in_window Bool t2 t3 e3) False = Proved
theorem window_result_half_open :
  Equal (List (Event Bool))
    (trace_window Bool t2 t3 ordered_input) window_half_open = Refl
theorem window_preserves_tie_order :
  Equal (List (Event Bool))
    (trace_window Bool t2 t4 ordered_input) window_keeps_tie = Refl
theorem window_equal_bounds_is_empty :
  Equal (List (Event Bool))
    (trace_window Bool t3 t3 ordered_input) (Nil (Event Bool)) = Proved
theorem window_keeps_unordered_relative_order :
  Equal (List (Event Bool))
    (trace_window Bool t1 t4 scrambled_left) window_keeps_unordered = Refl

-- The exact interleaving and each tie direction are observable values.
theorem merge_interleaves_and_takes_left_tie :
  Equal (List (Event Bool)) (trace_merge Bool left right) merged = Refl
theorem merge_left_tie :
  Equal (List (Event Bool))
    (trace_merge Bool
      (Cons (Event Bool) e3 (Nil (Event Bool)))
      (Cons (Event Bool) e3b (Nil (Event Bool)))) tie_lr = Refl
theorem merge_reversed_inputs_reverse_tie :
  Equal (List (Event Bool))
    (trace_merge Bool
      (Cons (Event Bool) e3b (Nil (Event Bool)))
      (Cons (Event Bool) e3 (Nil (Event Bool)))) tie_rl = Refl
theorem merge_empty_left :
  Equal (List (Event Bool))
    (trace_merge Bool (Nil (Event Bool)) right) right = Refl
theorem merge_empty_right :
  Equal (List (Event Bool))
    (trace_merge Bool left (Nil (Event Bool))) left = Refl
theorem merge_unordered_inputs_keep_head_order :
  Equal (List (Event Bool))
    (trace_merge Bool scrambled_left scrambled_right) merged_unordered = Refl

-- These are explicit witnesses for the ordering premises, not law-derived
-- input facts. Equal timestamps satisfy the non-decreasing relation.
theorem seed_left_ordered : time_ordered Bool left =
  and_intro
    (Equal Bool (event_before Bool e1 e3) True)
    (time_ordered Bool (Cons (Event Bool) e3 (Nil (Event Bool))))
    Proved Proved
theorem seed_right_ordered : time_ordered Bool right =
  and_intro
    (Equal Bool (event_before Bool e2 e3b) True)
    (time_ordered Bool (Cons (Event Bool) e3b (Nil (Event Bool))))
    Proved Proved
theorem seed_ordered_tail :
  time_ordered Bool
    (Cons (Event Bool) e2
      (Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool))))) =
  and_intro
    (Equal Bool (event_before Bool e2 e3) True)
    (time_ordered Bool
      (Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool)))))
    Proved
    (and_intro
      (Equal Bool (event_before Bool e3 e3b) True)
      (time_ordered Bool (Cons (Event Bool) e3b (Nil (Event Bool))))
      Proved Proved)
theorem seed_ordered_input : time_ordered Bool ordered_input =
  and_intro
    (Equal Bool (event_before Bool e1 e2) True)
    (time_ordered Bool
      (Cons (Event Bool) e2
        (Cons (Event Bool) e3 (Cons (Event Bool) e3b (Nil (Event Bool))))))
    Proved seed_ordered_tail

theorem seed_window_ordered :
  time_ordered Bool (trace_window Bool t2 t4 ordered_input) =
  trace_window_ordered Bool t2 t4 ordered_input seed_ordered_input

theorem seed_merge_ordered :
  time_ordered Bool (trace_merge Bool left right) =
  trace_merge_ordered Bool left right seed_left_ordered seed_right_ordered

-- The classifier counts True payloads and ignores its query argument.
-- The inputs need not be ordered for this law.
theorem seed_merge_count_without_order_premises :
  Equal Nat
    (count (Event Bool) payload_flag e3
      (trace_merge Bool scrambled_left scrambled_right))
    (add
      (count (Event Bool) payload_flag e3 scrambled_left)
      (count (Event Bool) payload_flag e3 scrambled_right)) =
  trace_merge_count Bool payload_flag e3 scrambled_left scrambled_right
theorem count_true_payload_left :
  Equal Nat (count (Event Bool) payload_flag e3 scrambled_left) (Suc Zero) = Proved
theorem count_true_payload_right :
  Equal Nat (count (Event Bool) payload_flag e3 scrambled_right) (Suc Zero) = Proved
theorem count_true_payload_after_merge :
  Equal Nat
    (count (Event Bool) payload_flag e3
      (trace_merge Bool scrambled_left scrambled_right))
    (Suc (Suc Zero)) = Proved
"##;

/// SOURCE: conformance/stdlib/time-and-events/seed-event-trace.md positives.
/// PROMISE CLASS: durable invariant.
/// MEASURED: all 22 independently spelled values and closed law instances
/// elaborate as a client of exported constructors and operations, including
/// two tie directions, both half-open endpoints and unordered-input controls.
/// CLAIMED: these closed values follow the specified head comparison and
/// window filter, while all three generic theorem names are importable.
/// THE GAP: these finite cases do not prove the general laws; those checked
/// bodies and independently restated types are verified above.
#[test]
fn twenty_two_seed_assertions_check_as_a_client() {
    let mut env = trace_env();
    env.elaborate_file(SEED_POSITIVE)
        .expect("all 22 source-shaped trace assertions check from imports");
    for name in [
        "window_lower_included",
        "window_upper_excluded",
        "window_result_half_open",
        "window_preserves_tie_order",
        "window_equal_bounds_is_empty",
        "window_keeps_unordered_relative_order",
        "merge_interleaves_and_takes_left_tie",
        "merge_left_tie",
        "merge_reversed_inputs_reverse_tie",
        "merge_empty_left",
        "merge_empty_right",
        "merge_unordered_inputs_keep_head_order",
        "seed_left_ordered",
        "seed_right_ordered",
        "seed_ordered_tail",
        "seed_ordered_input",
        "seed_window_ordered",
        "seed_merge_ordered",
        "seed_merge_count_without_order_premises",
        "count_true_payload_left",
        "count_true_payload_right",
        "count_true_payload_after_merge",
    ] {
        let id = env.globals[name];
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must be a checked seed declaration"
        );
    }
}

// Submit the exact false seed theorem only after the same positive client
// checks; an import or fixture failure must never masquerade as refusal.
const NEGATIVE: &str = r##"
-- Refused: this expected merge value places time 3 before time 2.
theorem refuse_out_of_time_order_merge_result :
  Equal (List (Event Bool))
    (trace_merge Bool left right)
    (Cons (Event Bool) e1
      (Cons (Event Bool) e3
        (Cons (Event Bool) e2 (Cons (Event Bool) e3b (Nil (Event Bool))))))
  = Proved
"##;

/// SOURCE: conformance/stdlib/time-and-events/seed-event-trace.md refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: with all 22 positives already checked, the independently
/// constructed out-of-time-order merged list is refused with a typed kernel
/// TypeMismatch at its own submitted theorem, not at setup or client import.
/// CLAIMED: the merge of the two ordered seed inputs cannot have time 3
/// before time 2; the true exact-output theorem passes on the same inputs.
/// THE GAP: this is one false result, not a generic sortedness proof.
#[test]
fn out_of_time_order_merge_result_is_refused_at_its_theorem() {
    let mut env = trace_env();
    env.elaborate_file(SEED_POSITIVE)
        .expect("positive source-shaped trace client must check first");
    match env.elaborate_file(NEGATIVE) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) => {
            let start = NEGATIVE
                .find("theorem refuse_out_of_time_order_merge_result")
                .expect("negative theorem anchor");
            assert!(
                span.start >= start && span.end <= NEGATIVE.len() && span.start < span.end,
                "refusal must target the false theorem, got {span:?}"
            );
        }
        other => panic!("expected typed kernel refusal at the false theorem, got {other:?}"),
    }
}
