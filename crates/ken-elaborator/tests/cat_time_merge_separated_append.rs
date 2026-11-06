//! Separated merge is append: spec/50-stdlib/69-time-and-events.md §13.
//! Source: conformance/stdlib/time-and-events/seed-merge-separated-append.md.
//! Promise class: durable invariant. An open generic client and an ordered
//! window-join client accompany same-form separated/interleaved Refl controls.

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
    "/../../conformance/stdlib/time-and-events/seed-merge-separated-append.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("EventTrace and its checked providers roots-load");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Collections.Derived (list_append, list_elem)
import Data.Time.EventTrace (Event, event_before, trace_merge, trace_merge_separated)

theorem open_separated_merge
      (p : Type) (xs : List (Event p)) (ys : List (Event p))
    : ((x : Event p)
        → list_elem (Event p) x xs
        → (y : Event p)
        → list_elem (Event p) y ys
        → Equal Bool (event_before p x y) True)
      → Equal (List (Event p))
          (trace_merge p xs ys)
          (list_append (Event p) xs ys) =
  trace_merge_separated p xs ys
"#;

/// MEASURED: complete trusted GlobalId identity sets agree before and after
/// roots-loading EventTrace over the checked collection provider. The law and
/// private helper are owner-local Transparent declarations; an arbitrary-payload
/// open client consumes the full quantified proposition and list equality,
/// while selective import of the helper is refused. CLAIMED: separation gives
/// a reusable public checked equality without new trust or exposed plumbing.
/// THE GAP: imported providers retain inherited trust; closed seed examples
/// cannot establish the generic quantifiers without the kernel-checked body.
#[test]
fn generic_separated_merge_is_owned_checked_public_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], "Data.Collections.Derived")
        .expect("checked list membership and append provider roots-loads");
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], TRACE)
        .expect("§13 public law and private helper check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!("§13 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "EventTrace introduces no trusted GlobalId");
    for name in ["trace_merge_separated", "trace_separated_starts_after"] {
        let id = env.globals[&format!("{TRACE}.{name}")];
        assert!(owned.contains(&id), "EventTrace owns {name}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{name} has a kernel-checked body"
        );
    }
    env.elaborate_file(GENERIC_CLIENT)
        .expect("an open client consumes the full membership premise");
    match trace_env().elaborate_file(&format!("import {TRACE} (trace_separated_starts_after)")) {
        Err(ElabError::UnboundName { name, .. }) => {
            assert_eq!(name, format!("{TRACE}.trace_separated_starts_after"));
        }
        other => panic!("separation helper must remain private: {other:?}"),
    }
}

fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§13 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§13 seed source fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must identify exactly one Ken source program"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §13 theorem {theorem_name}"))
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed §13 proposition: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) | SourceExpr::EVar(name, _) => format!("name:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unexpected §13 proof form: {other:?}"),
    }
}

fn closed_theorem_shape(source: &str, theorem_name: &str) -> (String, String) {
    let found = parse_decls(source)
        .expect("§13 theorem parses")
        .into_iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name,
                params,
                theorem,
                body,
                ..
            } if name == theorem_name => {
                assert!(params.is_empty(), "{theorem_name} is a closed claim");
                Some((type_shape(&theorem), expr_shape(&body)))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = found.as_slice() else {
        panic!("exactly one theorem named {theorem_name}");
    };
    one.clone()
}

const EXPECTED_SEPARATED: &str = r#"
theorem expected_control
    : Equal (List (Event Bool))
        (trace_merge Bool sep_left sep_right)
        (list_append (Event Bool) sep_left sep_right) = Refl
"#;
const EXPECTED_INTERLEAVED: &str = r#"
theorem expected_refusal
    : Equal (List (Event Bool))
        (trace_merge Bool interleaved_left interleaved_right)
        (list_append (Event Bool) interleaved_left interleaved_right) = Refl
"#;

const SEPARATED_VALUES: &str = r#"
theorem left_is_early : Equal (List (Event Bool)) sep_left
  (Cons (Event Bool) e1 (Cons (Event Bool) e3 (Nil (Event Bool)))) = Refl
theorem right_is_late : Equal (List (Event Bool)) sep_right
  (Cons (Event Bool) e7 (Nil (Event Bool))) = Refl
theorem joined_whole : Equal (List (Event Bool))
  (trace_merge Bool (trace_window Bool t0 t4 tr) (trace_window Bool t4 t9 tr)) tr = Refl
theorem whole_window : Equal (List (Event Bool)) (trace_window Bool t0 t9 tr) tr = Refl
theorem early_before_late : Equal Bool (event_before Bool e3 e7) True = Proved
"#;

const UNORDERED_AND_TIE_VALUES: &str = r#"
const unordered_left : List (Event Bool) =
  Cons (Event Bool) e3 (Cons (Event Bool) e1 (Nil (Event Bool)))
theorem unordered_head_reverses : Equal Bool (event_before Bool e3 e1) False = Proved
theorem unordered_merge_is_append : Equal (List (Event Bool))
  (trace_merge Bool unordered_left sep_right)
  (list_append (Event Bool) unordered_left sep_right) = Refl
const tied_right : Event Bool = MkEvent Bool t3 True
theorem equal_times_compare_true : Equal Bool (event_before Bool e3 tied_right) True = Proved
theorem tie_keeps_left_first : Equal (List (Event Bool))
  (trace_merge Bool
    (Cons (Event Bool) e3 (Nil (Event Bool)))
    (Cons (Event Bool) tied_right (Nil (Event Bool))))
  (Cons (Event Bool) e3 (Cons (Event Bool) tied_right (Nil (Event Bool)))) = Refl
"#;

const INTERLEAVED_VALUES: &str = r#"
theorem mixed_left_value : Equal (List (Event Bool)) interleaved_left
  (Cons (Event Bool) e1 (Cons (Event Bool) e7 (Nil (Event Bool)))) = Refl
theorem mixed_right_value : Equal (List (Event Bool)) interleaved_right
  (Cons (Event Bool) e3 (Nil (Event Bool))) = Refl
theorem merge_reorders : Equal (List (Event Bool))
  (trace_merge Bool interleaved_left interleaved_right)
  (Cons (Event Bool) e1 (Cons (Event Bool) e3 (Cons (Event Bool) e7 (Nil (Event Bool))))) = Refl
theorem append_keeps_left : Equal (List (Event Bool))
  (list_append (Event Bool) interleaved_left interleaved_right)
  (Cons (Event Bool) e1 (Cons (Event Bool) e7 (Cons (Event Bool) e3 (Nil (Event Bool))))) = Refl
theorem late_not_before_mid : Equal Bool (event_before Bool e7 e3) False = Proved
"#;

/// MEASURED: the exact accepted seed program, including an open windows_join
/// and closed join, checks with independent exact merge/window observations.
/// The parsed same-form separated Refl theorem checks; an unordered
/// separated pair also appends, and a tied pair keeps the left event first.
/// After a fresh complete accepted prefix, the interleaved inputs produce
/// independently observed, distinct merge and append lists; their false
/// equality rejects at its own Refl with a typed mismatch. CLAIMED: a separated merge
/// equals append and an interleaved merge need not. THE GAP: the finite trace
/// is not an official conformance run or a proof of the universal theorem;
/// the generic client and kernel-checked body above supply the latter.
#[test]
fn complete_window_join_seed_accepts_and_interleaved_append_refuses() {
    let accepted = seed_case("closed_join");
    let refused = seed_case("refuse_interleaved_merge_is_append");
    let mut accepting = trace_env();
    let trusted = accepting
        .env
        .trusted_base()
        .into_iter()
        .collect::<BTreeSet<_>>();
    accepting
        .elaborate_file(accepted)
        .expect("complete §13 accepted client checks with generic and closed join");
    assert_eq!(
        accepting
            .env
            .trusted_base()
            .into_iter()
            .collect::<BTreeSet<_>>(),
        trusted,
        "accepted seed adds no trusted GlobalId"
    );
    accepting
        .elaborate_file(SEPARATED_VALUES)
        .expect("the complete ordered join has independently observed exact values");
    accepting
        .elaborate_file(UNORDERED_AND_TIE_VALUES)
        .expect("unordered separated inputs and equal-time ties keep append order");
    for name in ["windows_join", "closed_join", "separated_control"] {
        let id = accepting.globals[name];
        assert!(matches!(
            accepting.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }

    let parsed = parse_decls(refused).expect("isolated refusal parses");
    let claims = parsed
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name, params, span, ..
            } => {
                assert_eq!(name, "refuse_interleaved_merge_is_append");
                assert!(params.is_empty(), "refusal is closed");
                Some(span.clone())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let [span] = claims.as_slice() else {
        panic!("exactly one isolated false theorem required");
    };
    let fixture = refused
        .get(..span.start)
        .expect("fixture precedes the refusal theorem");
    let claim = refused
        .get(span.start..span.end)
        .expect("refusal span is in its source fence");
    assert!(matches!(
        parse_decls(claim)
            .expect("isolated claim parses")
            .as_slice(),
        [SourceDecl::TheoremDecl { .. }]
    ));
    let mut refusing = trace_env();
    refusing
        .elaborate_file(accepted)
        .expect("fresh complete accepted seed checks first");
    refusing
        .elaborate_file(fixture)
        .expect("interleaved input fixtures elaborate before the claim");
    refusing
        .elaborate_file(INTERLEAVED_VALUES)
        .expect("the exact same inputs yield distinct merge and append lists");
    match refusing.elaborate_file(claim) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal"
                && claim.get(span.start..span.end) == Some("Refl") => {}
        other => panic!("only interleaved append Refl must refuse: {other:?}"),
    }
    // Assert the seed's propositions after both claims have reached their
    // intended semantic checks, so a false fixture cannot mask the refusal.
    assert_eq!(
        closed_theorem_shape(accepted, "separated_control"),
        closed_theorem_shape(EXPECTED_SEPARATED, "expected_control"),
        "same-form separated control retains its append equality and Refl"
    );
    assert_eq!(
        closed_theorem_shape(refused, "refuse_interleaved_merge_is_append"),
        closed_theorem_shape(EXPECTED_INTERLEAVED, "expected_refusal"),
        "interleaved claim retains the same equality and Refl form"
    );
}
