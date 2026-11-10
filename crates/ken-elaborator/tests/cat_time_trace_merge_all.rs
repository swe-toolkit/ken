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

// Independent open restatements exercise the exact four public types; a
// weakened premise or conclusion cannot be hidden by a closed example.
const GENERIC_CLIENT: &str = r#"
import Data.Collections.Derived (list_elem, list_concat)
import Data.Time.EventTrace
  (Event, trace_merge_all, time_ordered, trace_merge_all_ordered,
   trace_merge_all_member_complete, trace_merge_all_member_sound)

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
"#;

/// MEASURED: after loading checked providers, the exact trusted GlobalId
/// identity set is unchanged when EventTrace loads; all four new declarations
/// are owner-local Transparent bodies, and independent selective imports
/// consume their full open signatures. CLAIMED: the §15 right fold and three
/// public laws add no trust and cannot be weakened by a closed fixture alone.
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
        .expect("§15 right fold and all three checked proof bodies load");
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
    ] {
        let id = env.globals[&format!("{TRACE}.{name}")];
        assert!(owned.contains(&id), "EventTrace must own {name}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open ordered, complete and sound signatures check independently");
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
