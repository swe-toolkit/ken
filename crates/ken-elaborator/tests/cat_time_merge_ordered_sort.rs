//! Checked clients for `spec/50-stdlib/69-time-and-events.md` §22.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, KernelError, Term};

const TRACE: &str = "Data.Time.EventTrace";
const LAW: &str = "Data.Time.EventTrace.trace_merge_ordered_sort";
const SEED: &str =
    include_str!("../../../conformance/stdlib/time-and-events/seed-trace-merge-ordered-sort.md");
const OPEN: &str = r#"
import Data.Collections.Derived (sort, list_append)
import Data.Time.EventTrace
  (Event, event_before, time_ordered, trace_merge, trace_merge_ordered_sort)

theorem open_ordered_merge_sort
      (p : Type) (xs : List (Event p)) (ys : List (Event p))
    : time_ordered p xs
      → time_ordered p ys
      → Equal
        (List (Event p))
        (trace_merge p xs ys)
        (sort (Event p) (event_before p) (list_append (Event p) xs ys)) =
  λox. λoy. trace_merge_ordered_sort p xs ys ox oy
"#;

const TIE_AND_ORDER_CONTROLS: &str = r#"
const right_tie : Event Bool = MkEvent Bool (Suc Zero) False
const left : List (Event Bool) = Cons (Event Bool) e1 (Nil (Event Bool))
const right : List (Event Bool) = Cons (Event Bool) right_tie (Nil (Event Bool))
const left_first : List (Event Bool) = Cons (Event Bool) e1 right
const right_first : List (Event Bool) = Cons (Event Bool) right_tie left

const e5 : Event Bool = MkEvent Bool (Suc (Suc (Suc (Suc (Suc Zero))))) True
const unordered : List (Event Bool) = Cons (Event Bool) e5 left
const ordered : List (Event Bool) = Cons (Event Bool) e1 (Cons (Event Bool) e5 (Nil (Event Bool)))
const empty : List (Event Bool) = Nil (Event Bool)

theorem empty_ordered : time_ordered Bool empty = Proved

theorem tie_merge : Equal (List (Event Bool)) (trace_merge Bool left right) left_first = Refl

theorem tie_sort
    : Equal
        (List (Event Bool))
        (sort (Event Bool) (event_before Bool) (list_append (Event Bool) left right))
        left_first =
  Refl
"#;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn providers() -> ElabEnv {
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
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
    env
}

fn loaded() -> ElabEnv {
    let mut env = providers();
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace roots-loads");
    env
}

fn seed_sources() -> (&'static str, &'static str, &'static str) {
    let mut sources = SEED.split("\n```ken\n").skip(1).map(|part| {
        part.split_once("\n```")
            .expect("Ken fence has closing delimiter")
            .0
    });
    let prefix = sources.next().expect("shared seed prefix");
    let accepted = sources.next().expect("accepted seed row");
    let refused = sources.next().expect("refused seed row");
    (prefix, accepted, refused)
}

fn seed_prefix() -> ElabEnv {
    let mut env = loaded();
    env.elaborate_file(seed_sources().0)
        .expect("shared imports and ordering witnesses check");
    env
}

fn cites_law(env: &ElabEnv, name: &str) {
    let (_, body) = env
        .env
        .transparent_body(env.globals[name])
        .expect("checked client body");
    let owner = env.globals[LAW];
    let mut pending = vec![&body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return;
        }
        pending.extend(term.children());
    }
    panic!("{name} must cite the exact owned law Const");
}

fn refuses_closing_refl(env: &mut ElabEnv, source: &str, label: &str) {
    let refl = source.rfind("Refl").expect("closing proof token");
    match env.elaborate_file(source) {
        Err(ElabError::TypeMismatch { span, reason })
            if span.start == refl
                && span.end == refl + "Refl".len()
                && reason == "Refl expects an `Eq`-shaped goal" =>
        {
            eprintln!("{label}: {reason} at {span:?}");
        }
        other => panic!("{label} must refuse at its closing Refl: {other:?}"),
    }
}

/// Source: §22's generic law and AC-T. Promise class: durable invariant.
/// MEASURED: provider-first exact trust identity equality and a Transparent,
/// EventTrace-owned law referenced by the checked open client's body.
/// CLAIMED: the package provides the generic equation without new trust.
/// THE GAP: the test neither fixes inherited provider trust nor an export
/// count. Checked proof refactors and extra checked laws are compatible;
/// additional trusted IDs, loss of transparency or type drift are breaking.
#[test]
fn exact_trust_and_owned_open_ordered_merge_sort() {
    let mut env = providers();
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert!(!before.is_empty(), "providers inherit a nonempty trust set");
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace roots-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "EventTrace trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "EventTrace adds no trusted GlobalIds");
    let law = env.globals[LAW];
    assert!(owned.contains(&law), "EventTrace owns the theorem");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    env.elaborate_file(OPEN).expect("open ordered merge checks");
    cites_law(&env, "open_ordered_merge_sort");
}

/// Source: §22's private ordered-sort support, ruled by Architect AC-0.
/// Promise class: durable invariant for the public/private boundary.
/// MEASURED: the rooted module loads, but a selective import refuses at
/// the helper's qualified name. CLAIMED: clients cannot use that helper.
/// THE GAP: the check does not require any specific private proof staging.
/// Private refactors are compatible; publishing the helper is breaking.
#[test]
fn ordered_sort_helper_is_not_importable() {
    let mut env = loaded();
    const HELPER: &str = "trace_ordered_sort_fixed";
    match env.elaborate_file(&format!("import {TRACE} ({HELPER})")) {
        Err(ElabError::UnboundName { name, .. }) if name == format!("{TRACE}.{HELPER}") => {}
        other => panic!("private helper must refuse selective import: {other:?}"),
    }
}

/// Source: §22's interleaved input and plain-append refusal, seed AC-2.
/// Promise class: durable invariant. MEASURED: acceptance through the owned
/// law and elaborator refusal at the changed row's own closing Refl.
/// CLAIMED: on these inputs the merge is the sorted append, not plain append.
/// THE GAP: closed conversion does not establish the generic law. Equivalent
/// event spellings are compatible; accepting the unsorted result is breaking.
#[test]
fn seed_checks_sorted_append_and_refuses_plain_append() {
    let (_, accepted, refused) = seed_sources();
    let mut env = seed_prefix();
    env.elaborate_file(accepted)
        .expect("sorted append checks through the law");
    cites_law(&env, "merge_result");
    refuses_closing_refl(&mut seed_prefix(), refused, "plain append");
}

fn tie_claim(name: &str, expected: &str) -> String {
    format!(
        r#"theorem {name} : Equal (List (Event Bool)) (trace_merge Bool left right) {expected} =
  trans
    (List (Event Bool))
    (trace_merge Bool left right)
    (sort (Event Bool) (event_before Bool) (list_append (Event Bool) left right))
    {expected}
    (trace_merge_ordered_sort Bool left right Proved Proved)
    Refl"#
    )
}

/// Source: §22's left bias for equal times. Promise class: durable invariant.
/// MEASURED: separate merge and sort values and a law client put the True
/// payload first; swapping only the claimed order refuses at Refl.
/// CLAIMED: this tied merge preserves left-first order, not just the elements.
/// THE GAP: one tie cannot prove general stability. Payload substitutions
/// that keep distinct events are compatible; right-first bias is breaking.
#[test]
fn distinct_tied_payloads_keep_left_first_order() {
    let mut env = seed_prefix();
    env.elaborate_file(TIE_AND_ORDER_CONTROLS)
        .expect("closed tie values check independently");
    env.elaborate_file(&tie_claim("left_tie_by_law", "left_first"))
        .expect("tie equation reaches left-first order");
    cites_law(&env, "left_tie_by_law");
    refuses_closing_refl(
        &mut env,
        &tie_claim("right_tie_by_law", "right_first"),
        "swapped tie",
    );
}

/// Source: §22's two order premises. Promise class: durable invariant.
/// MEASURED: with one side empty and the other descending, independent
/// closed proofs show merge returns the descending trace and sort reverses
/// it; applying the law refuses for either position of the descending trace.
/// CLAIMED: neither ordering premise can be dropped at this fixture.
/// THE GAP: the kernel may span the whole isolated theorem, not a token.
/// Equivalent fixtures are compatible; a successful false application breaks.
#[test]
fn both_order_premises_are_necessary() {
    for unordered_on_left in [true, false] {
        let sides = |nonempty| {
            if unordered_on_left {
                (nonempty, "empty")
            } else {
                ("empty", nonempty)
            }
        };
        let application = |first, second, nonempty, name| {
            let witness = format!(
                "(and_intro (Equal Bool (event_before Bool {first} {second}) True) (time_ordered Bool (Cons (Event Bool) {second} empty)) Proved Proved)"
            );
            let (left, right) = sides(nonempty);
            let (ox, oy) = if unordered_on_left {
                (witness.as_str(), "empty_ordered")
            } else {
                ("empty_ordered", witness.as_str())
            };
            format!(
                r#"theorem {name}
    : Equal
        (List (Event Bool))
        (trace_merge Bool {left} {right})
        (sort (Event Bool) (event_before Bool) (list_append (Event Bool) {left} {right})) =
  trace_merge_ordered_sort Bool {left} {right} {ox} {oy}"#
            )
        };
        let (left, right) = sides("unordered");
        let mut env = seed_prefix();
        env.elaborate_file(TIE_AND_ORDER_CONTROLS)
            .expect("descending and ordered traces, with checked empty order");
        let controls = format!(
            r#"theorem descending_merge
    : Equal (List (Event Bool)) (trace_merge Bool {left} {right}) unordered = Refl

theorem ascending_sort
    : Equal
        (List (Event Bool))
        (sort (Event Bool) (event_before Bool) (list_append (Event Bool) {left} {right}))
        ordered = Refl"#
        );
        env.elaborate_file(&controls)
            .expect("merge and sort differ by value");
        env.elaborate_file(&application("e1", "e5", "ordered", "true_order_premise"))
            .expect("the same proof form checks for an ascending trace on this side");
        let false_application = application("e5", "e1", "unordered", "false_order_premise");
        let proof = false_application
            .find("Proved")
            .expect("only false witness");
        match env.elaborate_file(&false_application) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if span.start <= proof
                && span.end >= proof + "Proved".len()
                && span.end <= false_application.len() =>
            {
                eprintln!("descending {left}/{right}: kernel TypeMismatch at {span:?}");
            }
            other => panic!("{left}/{right} must refuse at its false order application: {other:?}"),
        }
    }
}
