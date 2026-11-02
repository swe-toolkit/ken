//! Window commutation: spec/50-stdlib/69-time-and-events.md §11.
//! Source: conformance/stdlib/time-and-events/seed-window-commutation.md.
//! Promise class: durable invariant. A public generic proof and a complete
//! closed seed with a same-form `Refl` control pin different obligations.

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
    "/../../conformance/stdlib/time-and-events/seed-window-commutation.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn trace_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], TRACE)
        .expect("§11 EventTrace proof body roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Data.Time.EventTrace (Event, trace_window, trace_window_commute)

theorem generic_window_commutation
      (p : Type) (lo1 : Nat) (hi1 : Nat) (lo2 : Nat) (hi2 : Nat)
      (tr : List (Event p))
    : Equal (List (Event p))
        (trace_window p lo2 hi2 (trace_window p lo1 hi1 tr))
        (trace_window p lo1 hi1 (trace_window p lo2 hi2 tr)) =
  trace_window_commute p lo1 hi1 lo2 hi2 tr
"#;

/// SOURCE: spec/50-stdlib/69-time-and-events.md §11.
/// PROMISE CLASS: durable invariant.
/// MEASURED: EventTrace's one new theorem is owner-local and Transparent;
/// an external selective-import client consumes its exact structural type
/// with arbitrary payload, bounds, and trace, without a premise. Exact
/// trusted GlobalId identity sets agree before and after loading EventTrace.
/// CLAIMED: the public law is generically reusable, checked and adds no trust.
/// THE GAP: the preloaded providers retain their inherited trust, and a
/// closed seed alone cannot demonstrate arbitrary quantifiers or ownership.
#[test]
fn generic_commutation_is_public_checked_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Core.Logic.Or",
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
        .expect("§11 checked proof and existing laws kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§11 EventTrace trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§11 adds no trusted GlobalId identity");
    let id = env.globals[&format!("{TRACE}.trace_window_commute")];
    assert!(owned.contains(&id), "EventTrace owns the §11 law");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "the public theorem has a kernel-checked body, not an assumption"
    );
    env.elaborate_file(GENERIC_CLIENT)
        .expect("open external client consumes the full premise-free law");
}

// Choose Ken source programs by parsed theorem identity, not by Markdown
// fence position or a source-line/heading snapshot.
fn seed_case(name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§11 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§11 seed source parses")
            .into_iter()
            .any(|decl| matches!(decl, SourceDecl::TheoremDecl { name: actual, .. } if actual == name));
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{name} must identify one source program"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("§11 seed case {name} is absent"))
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected closed §11 equality operand: {other:?}"),
    }
}

fn equality(ty: &SourceType) -> [&SourceType; 3] {
    let mut args = Vec::new();
    let mut head = ty;
    while let SourceType::TApp(function, argument, _) = head {
        args.push(argument.as_ref());
        head = function;
    }
    args.reverse();
    assert!(
        matches!(head, SourceType::TCon(name, _) | SourceType::TVar(name, _) if name == "Equal"),
        "the seed proposition is an equality"
    );
    args.try_into()
        .expect("Equal has a carrier and two endpoints")
}

fn one_theorem<'a>(decls: &'a [SourceDecl], name: &str) -> &'a SourceType {
    let found = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name: actual,
                theorem,
                ..
            } if actual == name => Some(theorem),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = found.as_slice() else {
        panic!("exactly one checked §11 seed theorem {name} required");
    };
    one
}

/// SOURCE: conformance/stdlib/time-and-events/seed-window-commutation.md.
/// PROMISE CLASS: durable invariant.
/// MEASURED: the entire accepted source checks first: law instance, each
/// window order's exact one-event value, outer-only two-event value, and
/// same-form `Refl` control. On the identical fixture, the isolated
/// outer-only claim differs only in its right endpoint and rejects at its
/// own `Refl` span after a fresh accepted prefix; the two endpoint values
/// are demonstrably unequal. CLAIMED: the inner window cannot be dropped
/// when swapping window order. THE GAP: finite Bool examples do not prove
/// the generic result; the public checked proof and client above do that.
#[test]
fn complete_seed_checks_before_dropped_inner_window_refuses_at_its_claim() {
    let accepted = seed_case("window_commutation_law_instance");
    let refused = seed_case("refuse_dropping_inner_window");
    let negative = parse_decls(refused).expect("isolated §11 refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body,
        span: claim_span,
        ..
    }] = negative.as_slice()
    else {
        panic!("isolated refusal is exactly one closed theorem");
    };
    assert_eq!(name, "refuse_dropping_inner_window");
    assert!(
        params.is_empty(),
        "the false claim is closed over the fixture"
    );
    assert!(
        matches!(body, SourceExpr::ECon(name, _) if name == "Refl"),
        "the rejected claim has the accepted control's proof form"
    );

    let mut proof = trace_env();
    proof
        .elaborate_file(accepted)
        .expect("complete seed accepted block, values, and same-form control check");
    for name in [
        "window_commutation_law_instance",
        "first_then_second_exact_value",
        "second_then_first_exact_value",
        "second_window_alone_exact_value",
        "same_form_commutation_control",
    ] {
        let id = proof.globals[name];
        assert!(matches!(
            proof.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }
    match proof.elaborate_file(
        "theorem unequal_window_values : Equal (List (Event Bool)) only4 only4and7 = Refl",
    ) {
        Err(ElabError::TypeMismatch { reason, .. })
            if reason == "Refl expects an `Eq`-shaped goal" => {}
        other => panic!("one-event and two-event controls must differ: {other:?}"),
    }

    let mut refusing = trace_env();
    refusing
        .elaborate_file(accepted)
        .expect("fresh complete seed accepted block checks before refusal");
    match refusing.elaborate_file(refused) {
        Err(ElabError::TypeMismatch { reason, span })
            if reason == "Refl expects an `Eq`-shaped goal"
                && span.start >= claim_span.start
                && span.end <= claim_span.end
                && refused.get(span.start..span.end) == Some("Refl") => {}
        other => panic!("outer-only claim must reject at its own Refl: {other:?}"),
    }

    // Only after the actual refusal, compare parsed Ken goals rather than
    // Markdown wording or source layout. This cannot mask a true-neighbour
    // mutation: it must first reach the rejection assertion above.
    let positive = parse_decls(accepted).expect("accepted §11 client parses");
    let [control_carrier, control_left, control_right] =
        equality(one_theorem(&positive, "same_form_commutation_control"));
    let [wrong_carrier, wrong_left, wrong_right] = equality(theorem);
    let [outer_carrier, outer_window, outer_value] =
        equality(one_theorem(&positive, "second_window_alone_exact_value"));
    let [swapped_carrier, swapped_window, one_event] =
        equality(one_theorem(&positive, "second_then_first_exact_value"));
    let [first_carrier, first_window, first_value] =
        equality(one_theorem(&positive, "first_then_second_exact_value"));
    for carrier in [wrong_carrier, outer_carrier, swapped_carrier, first_carrier] {
        assert_eq!(type_shape(carrier), type_shape(control_carrier));
    }
    assert_eq!(type_shape(wrong_left), type_shape(control_left));
    assert_eq!(type_shape(control_left), type_shape(first_window));
    assert_eq!(type_shape(control_right), type_shape(swapped_window));
    assert_eq!(type_shape(wrong_right), type_shape(outer_window));
    assert_eq!(type_shape(first_value), type_shape(one_event));
    assert_ne!(type_shape(outer_value), type_shape(one_event));
}
