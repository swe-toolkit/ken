//! Spec 69 §17: ordered k-way trace merging commutes with a half-open window.
//! Promise classes: durable checked open law and trust invariant; fixed-fixture
//! closed value vectors; false premise-free claims with same-form value controls.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, Term};

const TRACE: &str = "Data.Time.EventTrace";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}
fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().unwrap();
    env.elaborate_module_from_roots(&[root()], TRACE).unwrap();
    env
}

fn contains_const(term: &Term, wanted: GlobalId) -> bool {
    matches!(term, Term::Const { id, .. } if *id == wanted)
        || term
            .children()
            .iter()
            .any(|child| contains_const(child, wanted))
}

const W_GEN: &str = r#"
import Data.Collections.Derived (list_elem, map)
import Data.Time.EventTrace (Event, time_ordered, trace_window, trace_merge_all, trace_window_merge_all)

theorem window_merge_all_open
    (p : Type) (lo : Nat) (hi : Nat) (traces : List (List (Event p)))
    : ((t : List (Event p)) → list_elem (List (Event p)) t traces → time_ordered p t)
      → Equal
        (List (Event p))
        (trace_window p lo hi (trace_merge_all p traces))
        (trace_merge_all p (map (List (Event p)) (List (Event p)) (trace_window p lo hi) traces)) =
  trace_window_merge_all p lo hi traces
"#;
const ACCEPTED: &str = r#"
import Data.Time.EventTrace
  (Event, MkEvent, event_before, time_ordered, trace_window, trace_merge_all,
    trace_window_merge_all)
import Data.Collections.Derived (list_elem, map)
import Core.Logic.Transport (sym, trans)
import Core.Logic.Conjunction (and_intro)
import Core.Logic.Or (Or, Inl, Inr)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four
const six : Nat = Suc five
const seven : Nat = Suc six

const first_trace : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool two True)
    (Cons (Event Bool) (MkEvent Bool four True) (Nil (Event Bool)))
const second_trace : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool one False)
    (Cons (Event Bool) (MkEvent Bool four False) (Nil (Event Bool)))
const third_trace : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool three True)
    (Cons (Event Bool) (MkEvent Bool seven False) (Nil (Event Bool)))
const traces : List (List (Event Bool)) =
  Cons (List (Event Bool)) first_trace
    (Cons (List (Event Bool)) second_trace
      (Cons (List (Event Bool)) third_trace (Nil (List (Event Bool)))))
const windowed_merge_expected : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool two True)
    (Cons (Event Bool) (MkEvent Bool three True)
      (Cons (Event Bool) (MkEvent Bool four True)
        (Cons (Event Bool) (MkEvent Bool four False) (Nil (Event Bool)))))

theorem w_closed_distributes
    : Equal
      (List (Event Bool))
      (trace_window Bool two six (trace_merge_all Bool traces))
      (trace_merge_all Bool (map (List (Event Bool)) (List (Event Bool)) (trace_window Bool two six) traces)) =
  Refl

theorem w_closed_value
    : Equal (List (Event Bool)) (trace_window Bool two six (trace_merge_all Bool traces)) windowed_merge_expected =
  Refl

theorem ordered_first : time_ordered Bool first_trace =
  and_intro
    (Equal Bool (event_before Bool (MkEvent Bool two True) (MkEvent Bool four True)) True)
    (time_ordered Bool (Cons (Event Bool) (MkEvent Bool four True) (Nil (Event Bool))))
    Proved Proved

theorem ordered_second : time_ordered Bool second_trace =
  and_intro
    (Equal Bool (event_before Bool (MkEvent Bool one False) (MkEvent Bool four False)) True)
    (time_ordered Bool (Cons (Event Bool) (MkEvent Bool four False) (Nil (Event Bool))))
    Proved Proved

theorem ordered_third : time_ordered Bool third_trace =
  and_intro
    (Equal Bool (event_before Bool (MkEvent Bool three True) (MkEvent Bool seven False)) True)
    (time_ordered Bool (Cons (Event Bool) (MkEvent Bool seven False) (Nil (Event Bool))))
    Proved Proved

theorem every_trace_ordered (t : List (Event Bool))
    : list_elem (List (Event Bool)) t traces → time_ordered Bool t =
  λmember.
    elim_trunc
      (time_ordered Bool t)
      (λcase_first.
        match case_first {
          Inl same_first ↦
            J (λu _. time_ordered Bool u) ordered_first
              (sym (List (Event Bool)) t first_trace same_first);
          Inr later_first ↦
            elim_trunc
              (time_ordered Bool t)
              (λcase_second.
                match case_second {
                  Inl same_second ↦
                    J (λu _. time_ordered Bool u) ordered_second
                      (sym (List (Event Bool)) t second_trace same_second);
                  Inr later_second ↦
                    elim_trunc
                      (time_ordered Bool t)
                      (λcase_third.
                        match case_third {
                          Inl same_third ↦
                            J (λu _. time_ordered Bool u) ordered_third
                              (sym (List (Event Bool)) t third_trace same_third);
                          Inr later_third ↦ absurd later_third
                        })
                      later_second
                })
              later_first
        })
      member

theorem w_law_windowed_merge
    : Equal (List (Event Bool)) (trace_window Bool two six (trace_merge_all Bool traces)) windowed_merge_expected =
  trans
    (List (Event Bool))
    (trace_window Bool two six (trace_merge_all Bool traces))
    (trace_merge_all Bool (map (List (Event Bool)) (List (Event Bool)) (trace_window Bool two six) traces))
    windowed_merge_expected
    (trace_window_merge_all Bool two six traces every_trace_ordered)
    Refl
"#;
const REFUSED_PREFIX: &str = r#"
import Data.Time.EventTrace (Event, MkEvent, trace_window, trace_merge_all)
import Data.Collections.Derived (map)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const four : Nat = Suc three
const five : Nat = Suc four

const unordered_trace : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool five True)
    (Cons (Event Bool) (MkEvent Bool one True) (Nil (Event Bool)))
const ordered_trace : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool three False) (Nil (Event Bool))
const unordered_first : List (List (Event Bool)) =
  Cons (List (Event Bool)) unordered_trace
    (Cons (List (Event Bool)) ordered_trace (Nil (List (Event Bool))))
const unordered_last : List (List (Event Bool)) =
  Cons (List (Event Bool)) ordered_trace
    (Cons (List (Event Bool)) unordered_trace (Nil (List (Event Bool))))
const window_keeps_merge_order : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool three False)
    (Cons (Event Bool) (MkEvent Bool one True) (Nil (Event Bool)))
const merge_of_windows : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool one True)
    (Cons (Event Bool) (MkEvent Bool three False) (Nil (Event Bool)))

"#;
const X_FIRST_CONTROLS: &str = r#"
theorem x_first_window_of_merge
    : Equal (List (Event Bool)) (trace_window Bool Zero four (trace_merge_all Bool unordered_first)) window_keeps_merge_order =
  Refl
theorem x_first_merge_of_windows
    : Equal
      (List (Event Bool))
      (trace_merge_all Bool (map (List (Event Bool)) (List (Event Bool)) (trace_window Bool Zero four) unordered_first))
      merge_of_windows =
  Refl
"#;
const X_LAST_CONTROLS: &str = r#"
theorem x_last_window_of_merge
    : Equal (List (Event Bool)) (trace_window Bool Zero four (trace_merge_all Bool unordered_last)) window_keeps_merge_order =
  Refl
theorem x_last_merge_of_windows
    : Equal
      (List (Event Bool))
      (trace_merge_all Bool (map (List (Event Bool)) (List (Event Bool)) (trace_window Bool Zero four) unordered_last))
      merge_of_windows =
  Refl
"#;
const X_FIRST: &str = r#"
theorem x_first_distributes
    : Equal
      (List (Event Bool))
      (trace_window Bool Zero four (trace_merge_all Bool unordered_first))
      (trace_merge_all Bool (map (List (Event Bool)) (List (Event Bool)) (trace_window Bool Zero four) unordered_first)) =
  Refl
"#;
const X_LAST: &str = r#"
theorem x_last_distributes
    : Equal
      (List (Event Bool))
      (trace_window Bool Zero four (trace_merge_all Bool unordered_last))
      (trace_merge_all Bool (map (List (Event Bool)) (List (Event Bool)) (trace_window Bool Zero four) unordered_last)) =
  Refl
"#;

/// SOURCE: spec 69 §17; checked EventTrace owner and exact trust identity.
/// PROMISE CLASS: durable invariant. MEASURED: provider-first loading and
/// EventTrace loading have the same trusted GlobalId set, and the public law
/// belongs to EventTrace as a Transparent checked declaration. CLAIMED: the
/// new law adds no local trust and is exported from its canonical owner.
/// THE GAP: inherited provider trust persists; the open and closed client
/// tests below exercise the law's proposition and computed behavior.
#[test]
fn window_merge_all_is_public_owned_checked_and_zero_trust_delta() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    for name in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Data.Collections.Derived",
    ] {
        env.elaborate_module_from_roots(&[root()], name)
            .unwrap_or_else(|e| panic!("{name} roots-loads: {e:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("the checked EventTrace owner roots-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "EventTrace trust GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "the new theorem adds no trusted identity");
    let id = *env
        .globals
        .get(&format!("{TRACE}.trace_window_merge_all"))
        .expect("public windowed merge-all declaration");
    assert!(owned.contains(&id), "EventTrace owns the exported law");
    assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    env.elaborate_file("import Data.Time.EventTrace (trace_window_merge_all)")
        .expect("external selective import resolves the checked public law");
}

/// SOURCE: spec 69 §17, Architect's ruled public type and D0 W-GEN.
/// PROMISE CLASS: durable invariant. MEASURED: the generic external client
/// checks all open arguments and its body contains the exact owner Const in
/// the same environment. CLAIMED: a caller can consume the fully stated law,
/// rather than an unrelated same-spelled name or a weaker closed example.
/// THE GAP: the checked theorem supplies generic validity; the fixed-fixture
/// tests below distinguish this operation's particular reduction and premise.
#[test]
fn generic_windowed_merge_all_law_imports_and_applies_checked_owner() {
    let mut env = ElabEnv::new().expect("prelude bootstrap");
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace roots-loads");
    env.elaborate_file(W_GEN)
        .expect("W-GEN checks the public law's open type");
    let id = env.globals[&format!("{TRACE}.trace_window_merge_all")];
    assert!(
        owned.contains(&id),
        "same-env owner authenticates imported Const"
    );
    let client = env.globals["window_merge_all_open"];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(client) else {
        panic!("generic client must be checked");
    };
    assert!(
        contains_const(body, id),
        "client body must use the owner's ID"
    );
}

/// SOURCE: spec 69 §17 and D0's [2,6) three-ordered-trace witness.
/// PROMISE CLASSES: normative compatibility vector for this fixed fixture;
/// durable invariant for the checked application of the open law. MEASURED: two closed Refl claims independently
/// compute the same [2T,3T,4T,4F] list, and a third uses the checked public
/// theorem plus the head/middle/tail ordering premise. CLAIMED: the k-way
/// windowed merge has the specified values and the law applies to this case.
/// THE GAP: these finite values alone do not prove the open theorem; the
/// checked owner and generic client above supply that separate obligation.
#[test]
fn ordered_three_trace_window_values_and_public_law_check() {
    let mut env = trace_env();
    env.elaborate_file(ACCEPTED)
        .expect("W-CLOSED both values and W-LAW check on three ordered traces");
    let provider = env.globals[&format!("{TRACE}.trace_window_merge_all")];
    let client = env.globals["w_law_windowed_merge"];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(client) else {
        panic!("closed law application must have a checked body");
    };
    assert!(
        contains_const(body, provider),
        "the closed law client must actually consume the public proof"
    );
}

fn refused_after_own_value_controls(controls: &str, false_claim: &str, claim_name: &str) {
    let accepted = format!("{REFUSED_PREFIX}\n{controls}");
    trace_env()
        .elaborate_file(&accepted)
        .expect("two endpoint value controls accept on the same prefix");
    eprintln!("{claim_name}: both endpoint value controls accepted");
    let source = format!("{accepted}\n{false_claim}");
    let claims = parse_decls(&source)
        .expect("false claim parses")
        .into_iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name, span, params, ..
            } if name == claim_name && params.is_empty() => Some(span),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [claim] = claims.as_slice() else {
        panic!("one closed {claim_name} false claim must parse");
    };
    match trace_env().elaborate_file(&source) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal"
                && claim.start <= span.start
                && span.start < span.end
                && span.end <= claim.end
                && source.get(span.start..span.end) == Some("Refl") =>
        {
            eprintln!("{claim_name}: {reason}, own Refl {span:?} inside {claim:?}");
        }
        other => panic!("{claim_name} must refuse at its own Refl: {other:?}"),
    }
}

/// SOURCE: spec 69 §17's X-FIRST ordered-premise counterexample.
/// PROMISE CLASS: normative compatibility vector for this fixed false claim.
/// MEASURED: [5T,1T] before [3F] under [0,4) yields [3F,1T] versus
/// [1T,3F], certified by two accepted endpoint controls in a fresh
/// environment; their same-form false
/// equality then refuses at its own Refl. CLAIMED: dropping ordering at the
/// head of a two-trace list cannot support the general window law. THE GAP:
/// this row does not exercise a tail omission; X-LAST does so independently.
#[test]
fn unordered_first_trace_refuses_same_form_distribution() {
    refused_after_own_value_controls(X_FIRST_CONTROLS, X_FIRST, "x_first_distributes");
}

/// SOURCE: spec 69 §17's X-LAST ordered-premise counterexample.
/// PROMISE CLASS: normative compatibility vector for this fixed false claim.
/// MEASURED: [3F] before [5T,1T] gives the same unequal endpoints,
/// certified by its two accepted value controls in a separate environment;
/// the false claim refuses at its
/// own Refl. CLAIMED: the ordering premise is also needed for a later trace.
/// THE GAP: this one two-trace witness does not classify singleton traces.
#[test]
fn unordered_last_trace_refuses_same_form_distribution() {
    refused_after_own_value_controls(X_LAST_CONTROLS, X_LAST, "x_last_distributes");
}
