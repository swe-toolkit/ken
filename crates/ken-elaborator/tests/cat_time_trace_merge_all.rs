//! Spec 69 §15: right-fold merging of event traces.
//! Promise class: durable invariant. Open checked laws, closed interleaving and
//! left-biased ties, and an unordered-premise refusal are independent checks.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError};

const TRACE: &str = "Data.Time.EventTrace";
const ACCEPTED: &str = include_str!("fixtures/time_trace_merge_all/accepted.ken");
const UNORDERED_PREMISE: &str =
    include_str!("fixtures/time_trace_merge_all/ordered_premise_refused.ken");

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("§15 EventTrace operations and proofs root-load");
    env
}

// Independent open restatements exercise the five public types; a weakened
// premise or conclusion cannot be hidden by a closed example.
const GENERIC_CLIENT: &str = r#"
import Data.Collections.Derived (count, list_elem, list_concat)
import Data.Time.EventTrace
  (Event, trace_merge_all, time_ordered, trace_merge_all_ordered,
   trace_merge_all_member_complete, trace_merge_all_member_sound,
   trace_merge_all_count)

theorem ordered
    (p : Type) (traces : List (List (Event p)))
    : ((t : List (Event p)) → list_elem (List (Event p)) t traces → time_ordered p t)
      → time_ordered p (trace_merge_all p traces) =
  trace_merge_all_ordered p traces

theorem complete
    (p : Type) (e : Event p) (t : List (Event p)) (traces : List (List (Event p)))
    : list_elem (List (Event p)) t traces
      → list_elem (Event p) e t
      → list_elem (Event p) e (trace_merge_all p traces) =
  trace_merge_all_member_complete p e t traces

theorem sound
    (p : Type) (e : Event p) (traces : List (List (Event p)))
    : list_elem (Event p) e (trace_merge_all p traces)
      → list_elem (Event p) e (list_concat (Event p) traces) =
  trace_merge_all_member_sound p e traces

theorem all_count
    (p : Type) (eqf : Event p → Event p → Bool) (q : Event p)
    (traces : List (List (Event p)))
    : Equal Nat
        (count (Event p) eqf q (trace_merge_all p traces))
        (count (Event p) eqf q (list_concat (Event p) traces)) =
  trace_merge_all_count p eqf q traces
"#;

// Durable invariant: a duplicate in separate traces remains two occurrences.
// The wrong-one row below uses this identical accepted prefix in a fresh env.
const COUNT_ACCEPTED: &str = r#"
import Data.Time.EventTrace (Event, MkEvent, trace_merge_all, trace_merge_all_count, event_before)
import Data.Collections.Derived (count, list_concat, eq_from_ord)
import Core.Logic.Transport (trans)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

const t1 : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool one True)
    (Cons (Event Bool) (MkEvent Bool three True) (Nil (Event Bool)))
const t2 : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool two False) (Nil (Event Bool))
const t3 : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool three True) (Nil (Event Bool))
const ts : List (List (Event Bool)) =
  Cons (List (Event Bool)) t1
    (Cons (List (Event Bool)) t2 (Cons (List (Event Bool)) t3 (Nil (List (Event Bool)))))
const same : Event Bool → Event Bool → Bool = eq_from_ord (Event Bool) (event_before Bool)
const q3 : Event Bool = MkEvent Bool three True

theorem merged_twice : Equal Nat (count (Event Bool) same q3 (trace_merge_all Bool ts)) two = Proved

theorem merged_twice_by_law : Equal Nat (count (Event Bool) same q3 (trace_merge_all Bool ts)) two = trans Nat (count (Event Bool) same q3 (trace_merge_all Bool ts)) (count (Event Bool) same q3 (list_concat (Event Bool) ts)) two (trace_merge_all_count Bool same q3 ts) Proved
"#;

const COUNT_ONCE_REFUSED: &str = "theorem merged_once : Equal Nat (count (Event Bool) same q3 (trace_merge_all Bool ts)) one = Proved\n";

/// MEASURED: after loading checked providers, the exact trusted GlobalId
/// identity set is unchanged when EventTrace loads; the operation and four
/// public laws are owner-local Transparent bodies, and independent selective
/// imports consume their full open signatures. CLAIMED: the §15 right fold
/// and public laws add no trust and cannot be weakened by a closed fixture alone.
/// THE GAP: inherited provider trust persists; V1 and V3 below distinguish
/// the right-fold value and tie order that the generic laws alone do not.
#[test]
fn open_laws_and_operation_are_public_checked_and_zero_delta() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Data.Collections.Derived",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("§15 right fold and all four checked proof bodies load");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "EventTrace trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "§15 cannot add trusted GlobalId identities");
    for name in [
        "trace_merge_all",
        "trace_merge_all_ordered",
        "trace_merge_all_member_complete",
        "trace_merge_all_member_sound",
        "trace_merge_all_count",
    ] {
        let id = env.globals[&format!("{TRACE}.{name}")];
        assert!(owned.contains(&id), "EventTrace must own {name}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open ordered, complete, sound and count signatures check independently");
}

/// MEASURED: two closed occurrences are accepted directly and via the new
/// public law. CLAIMED: merge-all retains multiplicity, not only membership.
/// THE GAP: this finite client does not prove the generic law; the open proof
/// type and checked EventTrace declaration are independently exercised above.
#[test]
fn repeated_event_count_and_public_count_law_check() {
    trace_env()
        .elaborate_file(COUNT_ACCEPTED)
        .expect("R-CLOSED and R-LAW both preserve two occurrences");
}

/// MEASURED: the same accepted prefix in a fresh environment refuses a false
/// one-occurrence conclusion at the `merged_once` theorem's own span.
/// CLAIMED: the merge-all result must retain both occurrences.
/// THE GAP: rejection witnesses this one duplicate input, while the checked
/// open law above supplies the general proof.
#[test]
fn repeated_event_count_wrong_once_refuses_at_own_claim() {
    let source = format!("{COUNT_ACCEPTED}\n{COUNT_ONCE_REFUSED}");
    let decls = parse_decls(&source).expect("negative Ken client parses");
    let claims = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name, params, span, ..
            } if name == "merged_once" && params.is_empty() => Some(span),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [claim_span] = claims.as_slice() else {
        panic!("one merged_once negative claim must be parsed");
    };
    match trace_env().elaborate_file(&source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("R-ONCE must refuse at its own false count: {other:?}"),
    }
}

/// MEASURED: the selective-import Ken client checks Nil by Proved, the
/// [1,4]+[2,5]+[3,6] interleaving by Refl, an earlier-vs-later trace tie by
/// Refl, a closed three-trace ordered premise, third-trace membership through
/// complete, and V1-result membership through sound. CLAIMED: right-fold
/// merge preserves the specified values and earlier-trace tie priority.
/// THE GAP: finite observations do not prove generic laws; those are checked
/// independently above. V1 and the tie row are the AC-3 discriminators.
#[test]
fn closed_three_trace_values_ties_and_public_law_clients_check() {
    trace_env()
        .elaborate_file(ACCEPTED)
        .expect("all seven D0 accepted client rows must check together");
}

/// MEASURED: replacing only ordered b=[2,5] with b=[5,2] in the same C1
/// premise shape rejects at the closed `ordered_b` proof's false head-order
/// evidence with Kernel TypeMismatch, while the accepted client above checks.
/// CLAIMED: the ordered law cannot be applied without every input ordered.
/// THE GAP: this refusal is at the premise, not a purported output value or
/// a generic proof of failure for every unordered trace.
#[test]
fn unordered_middle_trace_refuses_at_its_order_premise() {
    let decls = parse_decls(UNORDERED_PREMISE).expect("negative Ken client parses");
    let claims = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name, params, span, ..
            } if name == "ordered_b" && params.is_empty() => Some(span),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [claim_span] = claims.as_slice() else {
        panic!("one closed ordered_b premise must be the refused claim");
    };
    match trace_env().elaborate_file(UNORDERED_PREMISE) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if claim_span.start <= span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("unordered b must refuse at its own premise proof: {other:?}"),
    }
}
