//! Derivative evaluation after substitution: spec 66 §8.
//! Source: conformance/stdlib/symbolic-algebra/seed-derivative-substitution.md.
//! Promise class: durable invariant. Two pointwise premises remain visible
//! in a generic client; a closed false conclusion is paired with true values.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/symbolic-algebra/seed-derivative-substitution.md"
));
const PRIVATE_HELPERS: [&str; 2] = ["nat_same_complete", "deriv_substitute_var"];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn package_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial §8 checked package roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, eval, deriv, substitute, compose_env, deriv_substitute)

theorem generic_derivative_substitution
      (v : Nat)
      (s : Nat → Expr)
      (e : Expr)
      (env : Nat → Nat)
      (at_v : Equal Nat (eval env (deriv v (s v))) (Suc Zero))
      (away : (w : Nat) → (Equal Nat v w → Bottom) → Equal Nat (eval env (deriv v (s w))) Zero)
    : Equal Nat
        (eval env (deriv v (substitute s e)))
        (eval (compose_env env s) (deriv v e)) =
  deriv_substitute v s e env at_v away
"#;

/// MEASURED: exact trusted GlobalId sets around provider-loaded→package load,
/// a package-owned Transparent public law, two package-owned Transparent
/// helpers that each refuse selective import, and an independent generic
/// client consuming both pointwise derivative premises and the full equality.
/// CLAIMED: §8 exports a checked conditional law without new trust or a new
/// public helper. THE GAP: inherited providers retain their assumptions;
/// one closed seed cannot stand in for a generic proof of the statement.
#[test]
fn generic_derivative_substitution_is_owned_checked_private_and_zero_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.Derived",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("§8 NatPolynomial law and helpers kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§8 NatPolynomial trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§8 adds no trusted GlobalId identity");
    let law = env.globals[&format!("{PACKAGE}.deriv_substitute")];
    assert!(owned.contains(&law), "NatPolynomial owns §8 law");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    env.elaborate_file(GENERIC_CLIENT)
        .expect("independent generic client checks the exact §8 premise type");
    for helper in PRIVATE_HELPERS {
        let id = env.globals[&format!("{PACKAGE}.{helper}")];
        assert!(owned.contains(&id), "NatPolynomial owns {helper}");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{helper} is checked, not an Axiom"
        );
        match env.elaborate_file(&format!("import {PACKAGE} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) => {
                assert_eq!(name, format!("{PACKAGE}.{helper}"));
            }
            other => panic!("{helper} must stay unimportable: {other:?}"),
        }
    }
}

// The theorem identifies the complete accepted source or isolated refusal;
// adding an unrelated source fence does not change the selected client.
fn seed_case(theorem_name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§8 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§8 seed source fence parses")
            .into_iter()
            .any(
                |decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem_name),
            );
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{theorem_name} must identify one source program"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("missing §8 theorem {theorem_name}"))
}

fn type_shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", type_shape(function), type_shape(argument))
        }
        other => panic!("unexpected §8 seed proposition: {other:?}"),
    }
}

fn expr_shape(expr: &SourceExpr) -> String {
    match expr {
        SourceExpr::ECon(name, _) | SourceExpr::EVar(name, _) => format!("name:{name}"),
        SourceExpr::EApp(function, argument, _) => {
            format!("({} {})", expr_shape(function), expr_shape(argument))
        }
        other => panic!("unexpected §8 seed proof: {other:?}"),
    }
}

fn theorem_shape(source: &str, name: &str) -> (String, String) {
    let found = parse_decls(source)
        .expect("Ken §8 obligation parses")
        .into_iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name: actual,
                theorem,
                body,
                ..
            } if actual == name => Some((type_shape(&theorem), expr_shape(&body))),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = found.as_slice() else {
        panic!("exactly one theorem {name} required");
    };
    one.clone()
}

const EXPECTED_CONTROLS: &str = r#"
theorem law_instance :
  Equal Nat (eval env23 (deriv Zero (substitute shift_off xy)))
    (eval (compose_env env23 shift_off) (deriv Zero xy)) =
  deriv_substitute Zero shift_off xy env23 Proved shift_off_zero_elsewhere

theorem same_shape :
  Equal Nat (eval env23 (deriv Zero (substitute shift_off xy)))
    (eval (compose_env env23 shift_off) (deriv Zero xy)) = Proved

theorem shift_left : Equal Nat (eval env23 (deriv Zero (substitute shift_off xy))) four = Proved

theorem shift_right : Equal Nat (eval (compose_env env23 shift_off) (deriv Zero xy)) four = Proved

theorem merge_left : Equal Nat (eval env23 (deriv Zero (substitute merge_off xy))) four = Proved

theorem merge_right : Equal Nat (eval (compose_env env23 merge_off) (deriv Zero xy)) two = Proved

theorem merge_order :
  Equal Bool
    (leq_nat (eval env23 (deriv Zero (substitute merge_off xy)))
      (eval (compose_env env23 merge_off) (deriv Zero xy))) False = Proved
"#;
const EXPECTED_MERGE_REFUSAL: &str = r#"
theorem false_merge :
  Equal Nat (eval env23 (deriv Zero (substitute merge_off xy)))
    (eval (compose_env env23 merge_off) (deriv Zero xy)) = Proved
"#;
const EXPECTED_DERIVATIVE_ONE_CONTROLS: &str = r#"
theorem const_at_zero :
  Equal Nat (eval env23 (deriv Zero (all_const Zero))) Zero = Proved

theorem var_law_instance :
  Equal Nat
    (eval env23 (deriv Zero (substitute shift_off (EVar Zero))))
    (eval (compose_env env23 shift_off) (deriv Zero (EVar Zero))) =
  deriv_substitute Zero shift_off (EVar Zero) env23
    shift_off_derivative_at_zero shift_off_zero_elsewhere

theorem var_same_shape :
  Equal Nat
    (eval env23 (deriv Zero (substitute shift_off (EVar Zero))))
    (eval (compose_env env23 shift_off) (deriv Zero (EVar Zero))) = Proved

theorem var_shift_left :
  Equal Nat (eval env23 (deriv Zero (substitute shift_off (EVar Zero)))) (Suc Zero) = Proved

theorem var_shift_right :
  Equal Nat (eval (compose_env env23 shift_off) (deriv Zero (EVar Zero))) (Suc Zero) = Proved

theorem var_const_left :
  Equal Nat (eval env23 (deriv Zero (substitute all_const (EVar Zero)))) Zero = Proved

theorem var_const_right :
  Equal Nat (eval (compose_env env23 all_const) (deriv Zero (EVar Zero))) (Suc Zero) = Proved

theorem var_const_order :
  Equal Bool
    (leq_nat
      (eval (compose_env env23 all_const) (deriv Zero (EVar Zero)))
      (eval env23 (deriv Zero (substitute all_const (EVar Zero))))) False = Proved
"#;
const EXPECTED_CONST_REFUSAL: &str = r#"
theorem false_const :
  Equal Nat (eval env23 (deriv Zero (substitute all_const (EVar Zero))))
    (eval (compose_env env23 all_const) (deriv Zero (EVar Zero))) = Proved
"#;

fn assert_isolated_refusal(accepted: &str, refused: &str, name: &str) {
    let decls = parse_decls(refused).expect("isolated §8 refusal parses");
    let [SourceDecl::TheoremDecl {
        name: actual,
        params,
        span: claim_span,
        ..
    }] = decls.as_slice()
    else {
        panic!("isolated refusal is exactly one theorem");
    };
    assert_eq!(actual, name);
    assert!(params.is_empty(), "false conclusion is closed");
    let mut refusing = package_env();
    refusing
        .elaborate_file(accepted)
        .expect("fresh complete accepted prefix checks before false conclusion");
    match refusing.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start >= claim_span.start
            && span.start < span.end
            && span.end <= claim_span.end => {}
        other => panic!("{name} false derivative must reject at its Proved: {other:?}"),
    }
}

/// MEASURED: the complete §8 accepted Ken source checks its zero-elsewhere
/// proof, non-variable successor image, public law instance, same-shape true
/// `Proved` control, and independent 4/4 and 4/2 results and 4≤2=False.
/// The isolated claim is structurally pinned to `merge_off`, not `shift_off`,
/// and is KernelRejected TypeMismatch at its own `Proved` after a fresh
/// accepted prefix. CLAIMED: the zero-elsewhere premise cannot be omitted
/// when a distinct variable maps to the differentiated variable. THE GAP:
/// this hw-false substitution is not every counterexample or a generic proof.
#[test]
fn full_seed_accepts_and_merged_variable_false_conclusion_refuses() {
    let accepted = seed_case("shift_off_law_instance");
    let refused = seed_case("merge_off_commutation_rejected");
    for (actual, expected) in [
        ("shift_off_law_instance", "law_instance"),
        ("shift_off_same_shape_control", "same_shape"),
        ("shift_off_left_value", "shift_left"),
        ("shift_off_right_value", "shift_right"),
        ("merge_off_left_value", "merge_left"),
        ("merge_off_right_value", "merge_right"),
        ("merge_off_sides_order_false", "merge_order"),
    ] {
        assert_eq!(
            theorem_shape(accepted, actual),
            theorem_shape(EXPECTED_CONTROLS, expected),
            "accepted control {actual} must keep its checked §8 obligation"
        );
    }
    assert_eq!(
        theorem_shape(refused, "merge_off_commutation_rejected"),
        theorem_shape(EXPECTED_MERGE_REFUSAL, "false_merge"),
        "refusal must claim the false merge_off derivative equality with Proved"
    );
    let mut env = package_env();
    env.elaborate_file(accepted)
        .expect("complete §8 seed and independent values check");
    for name in [
        "shift_off_law_instance",
        "shift_off_same_shape_control",
        "merge_off_left_value",
        "merge_off_right_value",
        "merge_off_sides_order_false",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    assert_isolated_refusal(accepted, refused, "merge_off_commutation_rejected");
}

/// MEASURED: the complete accepted source checks a universal proof of `hw`
/// for `all_const`, while its derivative at Zero is demonstrably zero, not
/// one. The independent endpoints are 0/1 and 1≤0=False; the same-shaped
/// `shift_off` true control is 1/1. The closed false `all_const` claim is
/// rejected at its own `Proved` on a fresh accepted prefix. CLAIMED: `hv`
/// cannot be omitted while retaining `hw`. THE GAP: this one closed fixture
/// does not prove the generic law or classify every failing derivative value.
#[test]
fn full_seed_accepts_and_constant_replacement_false_hv_refuses() {
    let accepted = seed_case("shift_off_law_instance");
    let refused = seed_case("all_const_commutation_rejected");
    for (actual, expected) in [
        ("all_const_derivative_at_zero_zero", "const_at_zero"),
        ("shift_off_var_law_instance", "var_law_instance"),
        ("shift_off_var_same_shape_control", "var_same_shape"),
        ("shift_off_var_left_value", "var_shift_left"),
        ("shift_off_var_right_value", "var_shift_right"),
        ("all_const_var_left_value", "var_const_left"),
        ("all_const_var_right_value", "var_const_right"),
        ("all_const_var_sides_order_false", "var_const_order"),
    ] {
        assert_eq!(
            theorem_shape(accepted, actual),
            theorem_shape(EXPECTED_DERIVATIVE_ONE_CONTROLS, expected),
            "accepted hv-axis control {actual} must keep its checked obligation"
        );
    }
    assert_eq!(
        theorem_shape(refused, "all_const_commutation_rejected"),
        theorem_shape(EXPECTED_CONST_REFUSAL, "false_const"),
        "refusal must claim the false all_const derivative equality with Proved"
    );
    let mut env = package_env();
    env.elaborate_file(accepted)
        .expect("complete §8 accepted source checks including hv-false controls");
    env.elaborate_file(
        "theorem all_const_hw_client (w : Nat)\n\
         (distinct : Equal Nat Zero w → Bottom)\n\
         : Equal Nat (eval env23 (deriv Zero (all_const w))) Zero =\n\
         all_const_zero_elsewhere w distinct",
    )
    .expect("zero-elsewhere proof applies at arbitrary distinct index");
    for name in [
        "all_const_zero_elsewhere",
        "all_const_derivative_at_zero_zero",
        "shift_off_var_law_instance",
        "shift_off_var_same_shape_control",
        "all_const_var_left_value",
        "all_const_var_right_value",
        "all_const_var_sides_order_false",
    ] {
        let id = env.globals[name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
    }
    assert_isolated_refusal(accepted, refused, "all_const_commutation_rejected");
}
