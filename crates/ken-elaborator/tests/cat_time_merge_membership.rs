//! Premise-free merge membership: spec/50-stdlib/69-time-and-events.md §4.
//! Source: conformance/stdlib/time-and-events/seed-merge-membership.md.
//! Promise class: durable invariant. Checked generic laws and the authored
//! unordered seed are separate; a refusal must be about the same merge.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::Decl;

const TRACE: &str = "Data.Time.EventTrace";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/time-and-events/seed-merge-membership.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace §4 roots-loads from checked providers");
    env
}

// These restatements are independent of the package's stated types. They
// require an arbitrary payload, event and pair of traces without ordering or
// payload-comparison premises, and consume each proof at its full conclusion.
const GENERIC_CLIENT: &str = r#"
import Core.Logic.Or (Or)
import Data.Collections.Derived (list_elem)
import Data.Time.EventTrace
  (Event, trace_merge, trace_merge_member_sound,
   trace_merge_member_left, trace_merge_member_right)

theorem generic_sound
    (p : Type) (e : Event p) (xs : List (Event p)) :
  (ys : List (Event p)) →
  list_elem (Event p) e (trace_merge p xs ys) →
  ‖ Or (list_elem (Event p) e xs) (list_elem (Event p) e ys) ‖ =
  trace_merge_member_sound p e xs

theorem generic_left
    (p : Type) (e : Event p) (xs : List (Event p)) :
  (ys : List (Event p)) →
  list_elem (Event p) e xs →
  list_elem (Event p) e (trace_merge p xs ys) =
  trace_merge_member_left p e xs

theorem generic_right
    (p : Type) (e : Event p) (xs : List (Event p)) :
  (ys : List (Event p)) →
  list_elem (Event p) e ys →
  list_elem (Event p) e (trace_merge p xs ys) =
  trace_merge_member_right p e xs
"#;

/// SOURCE: spec/50-stdlib/69-time-and-events.md §4.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the checked collection and logic providers roots-load, and the
/// exact trusted GlobalId identity set is unchanged when EventTrace loads.
/// All three public laws are owner-local transparent declarations, consumed
/// at their full generic result types; seven helper names cannot be imported.
/// CLAIMED: the universal laws are public checked proofs at zero new trust,
/// while their proof scaffolding is private.
/// THE GAP: inherited trust remains, and a checked type can be weakened;
/// the arbitrary-payload client consumes full §4 propositions, and the seed
/// below independently witnesses both unordered head decisions.
#[test]
fn generic_merge_membership_laws_are_checked_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], "Data.Collections.Derived")
        .expect("Derived roots-loads with checked membership helpers");
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace §4 law bodies check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§4 trusted_base() count: before={}, after={}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "EventTrace adds no trusted GlobalId");
    for law in [
        "trace_merge_member_sound",
        "trace_merge_member_left",
        "trace_merge_member_right",
    ] {
        let id = env.globals[&format!("{TRACE}.{law}")];
        assert!(owned.contains(&id), "EventTrace owns {law}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        env.elaborate_file(&format!("import {TRACE} ({law} as public_control_{law})"))
            .unwrap_or_else(|error| panic!("{law} must import from its owner: {error:?}"));
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent arbitrary-payload §4 types check");
    for helper in [
        "trace_merge_into_member_sound_cons",
        "trace_merge_into_member_sound",
        "trace_merge_into_member_left_cons",
        "trace_merge_into_member_left",
        "trace_merge_into_member_right_cons",
        "trace_merge_into_member_right",
        "trace_window_merge_member_sound",
    ] {
        let id = env.globals[&format!("{TRACE}.{helper}")];
        assert!(owned.contains(&id), "EventTrace owns {helper}");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        match env.elaborate_file(&format!("import {TRACE} ({helper})")) {
            Err(ElabError::UnboundName { name: missing, .. }) => {
                assert_eq!(missing, format!("{TRACE}.{helper}"));
            }
            other => panic!("private {helper} must refuse selective import: {other:?}"),
        }
    }
}

fn seed_fences() -> Vec<&'static str> {
    extract_ken_md(SEED)
        .expect("merge-membership seed fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
        .map(|fence| &SEED[fence.body_range])
        .collect()
}

/// SOURCE: conformance/stdlib/time-and-events/seed-merge-membership.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the unchanged positive Ken fence elaborates against the
/// roots-loaded package, including two unsorted input controls, a cross-input
/// tie, the independently expected merged list, two completeness instances
/// and a soundness instance.
/// CLAIMED: all three public laws can be consumed on unordered inputs.
/// THE GAP: the Bool-payload fixture cannot prove the generic quantifiers;
/// the checked universal proof bodies and restated signatures are above.
#[test]
fn shipped_unordered_tie_client_checks() {
    let fences = seed_fences();
    let [positive, _] = fences.as_slice() else {
        panic!("seed needs one positive fence and one isolated refusal");
    };
    trace_env()
        .elaborate_file(positive)
        .expect("the full shipped unordered positive client checks");
}

// AST shapes ignore source spans and formatting, and compare only the Ken
// proposition or witness that the checker is actually asked to accept.
fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unsupported closed seed proposition: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) => format!("con:{name}"),
        SourceExpr::EVar(name, _) => format!("var:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unsupported closed seed witness: {other:?}"),
    }
}

fn type_applied<'a>(ty: &'a SourceType, expected: &str) -> Vec<&'a SourceType> {
    let mut args = Vec::new();
    let mut head = ty;
    while let SourceType::TApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    assert!(
        matches!(head, SourceType::TCon(name, _) | SourceType::TVar(name, _) if name == expected),
        "expected {expected} proposition, found {head:?}"
    );
    args
}

fn expr_applied<'a>(expr: &'a SourceExpr, expected: &str) -> Vec<&'a SourceExpr> {
    let mut args = Vec::new();
    let mut head = expr;
    while let SourceExpr::EApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    assert!(
        matches!(head, SourceExpr::ECon(name, _) | SourceExpr::EVar(name, _) if name == expected),
        "expected {expected} witness, found {head:?}"
    );
    args
}

fn theorem<'a>(decls: &'a [SourceDecl], expected: &str) -> &'a SourceType {
    let matching = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl { name, theorem, .. } if name == expected => Some(theorem),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [found] = matching.as_slice() else {
        panic!("one checked {expected} control must exist");
    };
    found
}

fn fixture<'a>(decls: &'a [SourceDecl], expected: &str) -> &'a SourceExpr {
    let matching = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::ViewDecl {
                name, params, body, ..
            } if name == expected && params.is_empty() => Some(body),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [found] = matching.as_slice() else {
        panic!("one checked {expected} fixture must exist");
    };
    found
}

fn assert_refusal_is_same_input_false_head(positive: &str, negative: &str) -> ken_elaborator::Span {
    let checked = parse_decls(positive).expect("positive seed parses");
    let negative_decls = parse_decls(negative).expect("isolated refusal parses");
    let [SourceDecl::TheoremDecl {
        params,
        theorem: refused,
        body,
        span,
        ..
    }] = negative_decls.as_slice()
    else {
        panic!("refusal must be exactly one checked theorem");
    };
    assert!(params.is_empty(), "refused event must be closed");

    // The actual merge is pinned by the positive, checked exact-result
    // control, not inferred from a hardcoded text position in the Markdown.
    let [_, checked_merge, checked_result]: [&SourceType; 3] =
        type_applied(theorem(&checked, "merge_unordered_pair_exact"), "Equal")
            .try_into()
            .expect("positive must check one exact merged-list result");
    let [element, query, refused_merge]: [&SourceType; 3] = type_applied(refused, "list_elem")
        .try_into()
        .expect("refusal queries one element of the merge");
    assert_eq!(type_shape(element), "(name:Event name:Bool)");
    assert_eq!(type_shape(query), "name:neither");
    assert_eq!(type_shape(refused_merge), type_shape(checked_merge));
    assert_eq!(type_shape(checked_result), "name:merged");

    // The accepted closed merge's first head is the only branch this seed
    // attempts. Ensure its queried event differs from every input entry by
    // checked timestamp facts on those same fixture events (below).
    let merged = fixture(&checked, "merged");
    let [_, head, tail]: [&SourceExpr; 3] = expr_applied(merged, "Cons")
        .try_into()
        .expect("closed checked merge must expose a head and tail");
    let excerpt = |expr: &SourceExpr| &positive[expr.span().start..expr.span().end];
    let expected = format!(
        "theorem expected_refusal : list_elem (Event Bool) neither (trace_merge Bool xs ys) = trunc_intro (Inl (Equal (Event Bool) neither {}) (list_elem (Event Bool) neither {}) Refl)",
        excerpt(head),
        excerpt(tail),
    );
    let expected_decls = parse_decls(&expected).expect("expected false head witness parses");
    let [SourceDecl::TheoremDecl {
        theorem: expected_type,
        body: expected_body,
        ..
    }] = expected_decls.as_slice()
    else {
        panic!("one expected head witness");
    };
    assert_eq!(type_shape(refused), type_shape(expected_type));
    assert_eq!(expr_shape(body), expr_shape(expected_body));
    span.clone()
}

/// SOURCE: conformance/stdlib/time-and-events/seed-merge-membership.md refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the full positive fence checks in a fresh roots-loaded client;
/// the isolated refusal's parsed proposition is about the same merge as the
/// accepted exact-result control and its false `Refl` identifies a fresh
/// query with that result's first head. Independently checked time facts
/// distinguish the query from all four input events. The refusal is appended
/// to a fresh accepted client and fails at its own `Refl` with a typed
/// elaborator TypeMismatch, not an import, setup or earlier-positive error.
/// CLAIMED: an event absent from either input cannot be made a member of
/// their merge by a fabricated head-equality proof.
/// THE GAP: one false head witness is not a proof against all possible false
/// witnesses; the generic checked soundness theorem covers every member.
#[test]
fn shipped_neither_input_refusal_is_pinned_to_checked_merge() {
    let fences = seed_fences();
    let [positive, negative] = fences.as_slice() else {
        panic!("seed needs one positive fence and one isolated refusal");
    };
    let negative_span = assert_refusal_is_same_input_false_head(positive, negative);
    let checked = parse_decls(positive).expect("positive seed parses");
    let neither = fixture(&checked, "neither");
    assert!(
        matches!(neither, SourceExpr::EApp(..)),
        "absent event is a constructed event"
    );
    // These closed facts are derived from the actual positive-fixture names.
    // Their successful elaboration establishes non-vacuity even if a fixture
    // gets edited so the query shares a payload or timestamp with an input.
    let distinct_times = r#"
theorem neither_after_left_tie :
  Equal Bool (event_before Bool neither left_tie) False = Proved
theorem neither_after_left_early :
  Equal Bool (event_before Bool neither left_early) False = Proved
theorem neither_after_right_tie :
  Equal Bool (event_before Bool neither right_tie) False = Proved
theorem neither_after_right_early :
  Equal Bool (event_before Bool neither right_early) False = Proved
"#;
    let mut control = trace_env();
    control
        .elaborate_file(positive)
        .expect("same-input positive source controls check before refusal");
    control
        .elaborate_file(distinct_times)
        .expect("queried event has a later timestamp than every input entry");
    let mut env = trace_env();
    env.elaborate_file(positive)
        .expect("entire positive seed fence must check before refusal");
    match env.elaborate_file(negative) {
        Err(ElabError::TypeMismatch { span, reason })
            if reason == "Refl expects an `Eq`-shaped goal" =>
        {
            assert!(
                span.start >= negative_span.start && span.end <= negative_span.end,
                "false witness fails inside its own theorem"
            );
            assert_eq!(
                negative.get(span.start..span.end),
                Some("Refl"),
                "the rejected child is precisely the false head equality"
            );
        }
        other => panic!("false head witness must reject at Refl, got {other:?}"),
    }
}
