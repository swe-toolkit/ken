//! Spec 69 §18: merging two batches separately equals merging them at once.
//! Promise class: durable invariant. The open law has no ordering premise;
//! closed interleaving distinguishes merging partial traces from appending them.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, Term};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const DERIVED: &str = "Data.Collections.Derived";
const TRACE: &str = "Data.Time.EventTrace";
const LAW: &str = "trace_merge_all_append";
const ALIAS_WITNESS: &str = "theorem selected_true : IsTrue True = Proved\n";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn load_providers(env: &mut ElabEnv) {
    for provider in [
        "Core.Logic.Transport",
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
}

fn loaded_trace() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    env.elaborate_module_from_roots(&[root()], DERIVED)
        .expect("Derived root loads before EventTrace");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace proof bodies root-load");
    env
}

fn selected_client(client: &str) -> String {
    // The checked alias witness makes the selection explicit even though
    // the batch equality itself needs no IsTrue premise.
    checked_is_true::selected_source_with_import(&format!("{client}\n{ALIAS_WITNESS}"))
}

fn body_contains_const(body: &Term, owner: GlobalId) -> bool {
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

const OPEN_CLIENT: &str = r#"
import Data.Collections.Derived (list_append)
import Data.Time.EventTrace (Event, trace_merge, trace_merge_all, trace_merge_all_append)

theorem merge_all_append_open
      (p : Type) (xs : List (List (Event p))) (ys : List (List (Event p)))
    : Equal
        (List (Event p))
        (trace_merge_all p (list_append (List (Event p)) xs ys))
        (trace_merge p (trace_merge_all p xs) (trace_merge_all p ys)) =
  trace_merge_all_append p xs ys
"#;

const ACCEPTED: &str = r#"
import Data.Collections.Derived (list_append)
import Data.Time.EventTrace
  (Event, MkEvent, trace_merge, trace_merge_all, trace_merge_all_append)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

const left_trace : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool one True)
    (Cons (Event Bool) (MkEvent Bool three True) (Nil (Event Bool)))
const right_trace : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool two False) (Nil (Event Bool))
const xs : List (List (Event Bool)) =
  Cons (List (Event Bool)) left_trace (Nil (List (Event Bool)))
const ys : List (List (Event Bool)) =
  Cons (List (Event Bool)) right_trace (Nil (List (Event Bool)))

const merged_value : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool one True)
    (Cons (Event Bool) (MkEvent Bool two False)
      (Cons (Event Bool) (MkEvent Bool three True) (Nil (Event Bool))))
const appended_value : List (Event Bool) =
  Cons (Event Bool) (MkEvent Bool one True)
    (Cons (Event Bool) (MkEvent Bool three True)
      (Cons (Event Bool) (MkEvent Bool two False) (Nil (Event Bool))))

theorem partial_left_value
    : Equal (List (Event Bool)) (trace_merge_all Bool xs) left_trace =
  Refl
theorem partial_right_value
    : Equal (List (Event Bool)) (trace_merge_all Bool ys) right_trace =
  Refl
theorem appended_partial_value
    : Equal
        (List (Event Bool))
        (list_append (Event Bool) (trace_merge_all Bool xs) (trace_merge_all Bool ys))
        appended_value =
  Refl

theorem merged_batches_agree
    : Equal
        (List (Event Bool))
        (trace_merge_all Bool (list_append (List (Event Bool)) xs ys))
        (trace_merge Bool (trace_merge_all Bool xs) (trace_merge_all Bool ys)) =
  Refl
theorem merged_batches_value
    : Equal
        (List (Event Bool))
        (trace_merge_all Bool (list_append (List (Event Bool)) xs ys))
        merged_value =
  Refl
theorem merged_batches_by_law
    : Equal
        (List (Event Bool))
        (trace_merge_all Bool (list_append (List (Event Bool)) xs ys))
        (trace_merge Bool (trace_merge_all Bool xs) (trace_merge_all Bool ys)) =
  trace_merge_all_append Bool xs ys
"#;

const FALSE_APPEND: &str = r#"theorem batch_merge_is_partial_append
    : Equal
        (List (Event Bool))
        (trace_merge_all Bool (list_append (List (Event Bool)) xs ys))
        (list_append (Event Bool) (trace_merge_all Bool xs) (trace_merge_all Bool ys)) =
  Refl
"#;

/// SOURCE: spec/50-stdlib/69-time-and-events.md §18, Architect AC-0 law.
/// PROMISE CLASS: durable invariant.
/// MEASURED: six providers load first; the exact trusted GlobalId set stays
/// equal across Derived and EventTrace loads. The public theorem belongs to
/// EventTrace and is Transparent. Its selective-import open client checks
/// all three binders without a premise and uses that exact owner Const.
/// CLAIMED: batch merging reuses a generic checked law with no new local
/// trust. THE GAP: inherited provider trust is not erased; closed values
/// below distinguish merge from ordinary concatenation.
#[test]
fn provider_first_exact_trust_and_open_public_law() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    env.elaborate_module_from_roots(&[root()], DERIVED)
        .expect("Derived proofs root-load");
    let after_derived = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_derived, before, "Derived adds no trusted identity");
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace proof bodies check");
    let after_trace = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after_trace, before, "batch law adds no trusted identity");
    let qualified = format!("{TRACE}.{LAW}");
    let id = *env
        .globals
        .get(&qualified)
        .expect("batch law has checked identity");
    assert!(owned.contains(&id), "EventTrace owns the batch law");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "the public law has a checked proof body"
    );
    env.elaborate_file(&selected_client(OPEN_CLIENT))
        .expect("public selective import and open law with aliased IsTrue check");
    let (_, body) = env
        .env
        .transparent_body(env.globals["merge_all_append_open"])
        .expect("open client has a checked proof body");
    assert!(
        body_contains_const(&body, id),
        "open client uses checked owner Const"
    );
}

/// SOURCE: spec/50-stdlib/69-time-and-events.md §18, AC-2 accepted fixture.
/// PROMISE CLASS: durable law with a normative closed-value discriminator.
/// MEASURED: xs=[[1T,3T]], ys=[[2F]] give batch merge [1T,2F,3T]
/// by independent Refl and by the law; the two partial merges and their
/// append independently give [1T,3T,2F]. CLAIMED: merge-all obeys the
/// interleaving law without treating merge as append.
/// THE GAP: finite values cannot replace the kernel-checked open law.
#[test]
fn whole_interleaved_accepted_client_checks() {
    loaded_trace()
        .elaborate_file(&selected_client(ACCEPTED))
        .expect("whole accepted client and explicit IsTrue alias check");
}

/// SOURCE: spec/50-stdlib/69-time-and-events.md §18, AC-2 false append.
/// PROMISE CLASS: durable invariant with a closed refusal discriminator.
/// MEASURED: after the entire accepted prefix checks afresh, the isolated
/// claim replacing merge of the two partial results with append refuses
/// as an ElabError::TypeMismatch at exactly its own Refl token.
/// CLAIMED: interleaving batches cannot be combined by append.
/// THE GAP: the one fixture does not establish arbitrary-trace equality.
#[test]
fn false_partial_append_refuses_at_own_refl_on_fresh_prefix() {
    let mut env = loaded_trace();
    env.elaborate_file(&selected_client(ACCEPTED))
        .expect("fresh whole accepted prefix checks before false append");
    let negative = checked_is_true::selected_source(FALSE_APPEND);
    let parsed = parse_decls(&negative).expect("false append theorem parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("only one appended false-append theorem");
    };
    assert_eq!(name, "batch_merge_is_partial_append");
    match env
        .elaborate_file(&negative)
        .expect_err("false append must refuse")
    {
        ElabError::TypeMismatch { reason, span }
            if reason == "Refl expects an `Eq`-shaped goal"
                && claim.start <= span.start
                && span.start < span.end
                && span.end <= claim.end
                && negative.get(span.start..span.end) == Some("Refl") =>
        {
            let token = negative.rfind("Refl").expect("terminal of own claim");
            assert_eq!(span.start, token, "refusal starts at its own Refl");
            assert_eq!(
                span.end,
                token + "Refl".len(),
                "refusal ends at its own Refl"
            );
        }
        other => panic!("false append must refuse only at own Refl: {other:?}"),
    }
}
