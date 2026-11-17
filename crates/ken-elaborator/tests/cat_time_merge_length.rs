//! Spec 69 §19: two-trace and right-fold merge preserve total list length.
//! Promise class: durable invariant. Open checked laws do not need ordered
//! inputs; closed unordered and duplicate traces distinguish merge from
//! append-only and deduplicating interpretations.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const TRACE: &str = "Data.Time.EventTrace";
const PAIR: &str = "trace_merge_length";
const ALL: &str = "trace_merge_all_length";

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
        "Data.Collections.Derived",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
}

fn loaded_trace() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace with two new length laws root-loads");
    env
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
import Data.Time.EventTrace
  (Event, trace_merge, trace_merge_all, trace_merge_length, trace_merge_all_length)
import Data.Collections.Derived (length, list_concat)
import Data.Numeric.Nat.Arithmetic (add)

theorem merge_length_open (p : Type) (xs : List (Event p))
    : (ys : List (Event p))
      → Equal Nat
          (length (Event p) (trace_merge p xs ys))
          (add (length (Event p) xs) (length (Event p) ys)) =
  trace_merge_length p xs

theorem merge_all_length_open (p : Type) (traces : List (List (Event p)))
    : Equal Nat
        (length (Event p) (trace_merge_all p traces))
        (length (Event p) (list_concat (Event p) traces)) =
  trace_merge_all_length p traces
"#;

// The whole Architect D0 accepted client is kept together: each of its
// three closed shapes applies a public law and separately proves the value.
const ACCEPTED: &str = r#"import Data.Time.EventTrace
  (Event, MkEvent, trace_merge, trace_merge_all, trace_merge_length, trace_merge_all_length)
import Data.Collections.Derived (length, list_concat)
import Data.Numeric.Nat.Arithmetic (add)

const zero : Nat = Zero
const one : Nat = Suc zero
const two : Nat = Suc one
const three : Nat = Suc two
const five : Nat = Suc (Suc three)

const late : Event Bool = MkEvent Bool five True
const early : Event Bool = MkEvent Bool three False
const same : Event Bool = MkEvent Bool three True

const late_first : List (Event Bool) = Cons (Event Bool) late (Nil (Event Bool))
const early_second : List (Event Bool) = Cons (Event Bool) early (Nil (Event Bool))
const same_left : List (Event Bool) = Cons (Event Bool) same (Nil (Event Bool))
const same_right : List (Event Bool) = Cons (Event Bool) same (Nil (Event Bool))

theorem unordered_law
    : Equal Nat
        (length (Event Bool) (trace_merge Bool late_first early_second))
        (add (length (Event Bool) late_first) (length (Event Bool) early_second)) =
  trace_merge_length Bool late_first early_second

theorem unordered_two
    : Equal Nat (length (Event Bool) (trace_merge Bool late_first early_second)) two = Proved

theorem duplicate_law
    : Equal Nat
        (length (Event Bool) (trace_merge Bool same_left same_right))
        (add (length (Event Bool) same_left) (length (Event Bool) same_right)) =
  trace_merge_length Bool same_left same_right

theorem duplicate_two
    : Equal Nat (length (Event Bool) (trace_merge Bool same_left same_right)) two = Proved

const three_traces : List (List (Event Bool)) =
  Cons (List (Event Bool)) late_first
    (Cons (List (Event Bool)) (Nil (Event Bool))
      (Cons (List (Event Bool)) same_right (Nil (List (Event Bool)))))

theorem all_law
    : Equal Nat
        (length (Event Bool) (trace_merge_all Bool three_traces))
        (length (Event Bool) (list_concat (Event Bool) three_traces)) =
  trace_merge_all_length Bool three_traces

theorem all_two
    : Equal Nat (length (Event Bool) (trace_merge_all Bool three_traces)) two = Proved
"#;

const FALSE_DUPLICATE: &str = r#"theorem duplicate_one
    : Equal Nat (length (Event Bool) (trace_merge Bool same_left same_right)) one = Proved
"#;

/// SOURCE: spec/50-stdlib/69-time-and-events.md §19 and Architect D0.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after the explicit providers, EventTrace adds no trusted
/// GlobalId identity; both law declarations belong to that package, are
/// Transparent and publicly importable with exact open signatures. Both
/// open client proof bodies refer to their respective checked owner Const.
/// CLAIMED: these generic length laws are checked with zero additional trust.
/// THE GAP: inherited provider assumptions persist; the finite duplicate
/// and unordered witnesses are separately checked below.
#[test]
fn two_length_laws_are_public_owned_checked_and_zero_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace root-loads its checked laws");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        before, after,
        "length laws add no trusted GlobalId identity"
    );
    for name in [PAIR, ALL] {
        let qualified = format!("{TRACE}.{name}");
        let id = *env.globals.get(&qualified).expect("checked law identity");
        assert!(owned.contains(&id), "{name} must belong to EventTrace");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} must have a checked proof body"
        );
    }
    env.elaborate_file(OPEN_CLIENT)
        .expect("both selectively imported open law signatures check");
    for (client, law) in [("merge_length_open", PAIR), ("merge_all_length_open", ALL)] {
        let (_, body) = env
            .env
            .transparent_body(env.globals[client])
            .expect("open client has checked body");
        assert!(
            body_contains_const(&body, env.globals[&format!("{TRACE}.{law}")]),
            "{client} must use the owner law Const"
        );
    }
}

/// SOURCE: spec/50-stdlib/69-time-and-events.md §19, AC-2 accepted client.
/// PROMISE CLASS: durable invariant with specified closed-value controls.
/// MEASURED: on the whole checked client, unordered [5T]/[3F], duplicate
/// [3T]/[3T], and [late]/[]/[same] right fold each preserve two events,
/// with law applications and independently stated Proved results.
/// CLAIMED: neither ordering nor deduplication is a hidden precondition.
/// THE GAP: finite values do not replace the two generic checked laws.
#[test]
fn complete_unordered_duplicate_and_empty_batch_client_checks() {
    loaded_trace()
        .elaborate_file(ACCEPTED)
        .expect("six accepted law/value controls check as a whole");
}

/// SOURCE: spec/50-stdlib/69-time-and-events.md §19, AC-2 refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: an accepted prefix checks afresh, and only its appended false
/// one-occurrence theorem rejects as KernelRejected(TypeMismatch), with the
/// diagnostic inside that theorem and encompassing its own Proved body.
/// CLAIMED: merging two copies never silently deduplicates this fixture.
/// THE GAP: the closed refusal is not itself a universal proof of length.
#[test]
fn duplicate_one_refuses_only_after_fresh_accepted_prefix() {
    let mut env = loaded_trace();
    env.elaborate_file(ACCEPTED)
        .expect("fresh complete accepted prefix checks");
    let parsed = parse_decls(FALSE_DUPLICATE).expect("false claim parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("one isolated false duplicate theorem");
    };
    assert_eq!(name, "duplicate_one");
    let proof = FALSE_DUPLICATE.rfind("Proved").expect("own proof term");
    match env
        .elaborate_file(FALSE_DUPLICATE)
        .expect_err("false one-occurrence claim must reject")
    {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        } if claim.start <= span.start
            && span.start <= proof
            && proof + "Proved".len() <= span.end
            && span.end <= claim.end => {}
        other => panic!("duplicate_one must reject at its own claim: {other:?}"),
    }
}

/// SOURCE: spec/50-stdlib/69-time-and-events.md §19, private helpers.
/// PROMISE CLASS: durable privacy invariant.
/// MEASURED: both interior lemmas are checked but cannot be imported by an
/// outside client; only UnboundName naming the qualified helper is accepted.
/// CLAIMED: clients rely on public laws rather than package proof scaffolding.
/// THE GAP: this visibility assertion does not prove the helper body; its
/// checked package load and the open laws do that separately.
#[test]
fn merge_length_helpers_remain_private() {
    let mut env = loaded_trace();
    for name in ["trace_merge_into_length_cons", "trace_merge_into_length"] {
        let qualified = format!("{TRACE}.{name}");
        match env.elaborate_file(&format!("import {TRACE} ({name})")) {
            Err(ElabError::UnboundName { name: missing, .. }) if missing == qualified => {}
            other => panic!("{name} must refuse selective import: {other:?}"),
        }
    }
}
