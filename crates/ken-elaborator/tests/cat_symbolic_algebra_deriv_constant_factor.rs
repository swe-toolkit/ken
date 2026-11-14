//! Constant factors in differentiation: spec/50-stdlib/66-symbolic-algebra.md §12.
//! Source: conformance/stdlib/symbolic-algebra/seed-derivative-constant-factor.md.
//! Promise class: durable invariant. A public generic checked proof and an
//! actual closed seed with a same-form `Proved` control pin separate obligations.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, Expr as SourceExpr, KenMdFenceRole,
    Type as SourceType,
};
use ken_kernel::{Decl, KernelError};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/symbolic-algebra/seed-derivative-constant-factor.md"
));

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn package_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("§12 NatPolynomial checked proof roots-loads");
    env
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EMul, deriv, poly_eq, deriv_const_factor)

theorem client_const_factor (v : Nat) (k : Nat) (e : Expr)
    : IsTrue (poly_eq (deriv v (EMul (EConst k) e))
        (EMul (EConst k) (deriv v e))) =
  deriv_const_factor v k e
"#;

/// SOURCE: spec 66 §12. PROMISE CLASS: durable invariant.
/// MEASURED: the five named providers are loaded before NatPolynomial; exact
/// before/after trusted GlobalId identity sets agree; a public owner-local
/// Transparent law checks and a selective-import client consumes its exact
/// unrestricted type at arbitrary variables, constants and expressions.
/// CLAIMED: the constant-factor law is premise-free, reusable and zero-new-trust.
/// THE GAP: inherited provider trust remains, and finite seed instances do not
/// establish the universal statement independently of the checked law body.
#[test]
fn generic_constant_factor_is_public_checked_and_zero_trust() {
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
        .expect("§12 proof and existing laws kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "§12 NatPolynomial trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(after, before, "§12 adds no trusted GlobalId identity");
    let id = env.globals[&format!("{PACKAGE}.deriv_const_factor")];
    assert!(owned.contains(&id), "NatPolynomial owns the §12 law");
    assert!(
        matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
        "public law has a checked body, not an assumption"
    );
    env.elaborate_file(&checked_is_true::selected_source_with_import(GENERIC_CLIENT))
        .expect("external client consumes the complete premise-free §12 law");
}

// Select actual Ken programs by parsed theorem identity, never prose or fence
// position. A complete accepted prefix must check before the isolated refusal.
fn seed_case(name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§12 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        let found = parse_decls(source)
            .expect("§12 seed source parses")
            .into_iter()
            .any(|decl| matches!(decl, SourceDecl::TheoremDecl { name: actual, .. } if actual == name));
        if found {
            assert!(
                selected.replace(source).is_none(),
                "{name} must identify one seed client"
            );
        }
    }
    selected.unwrap_or_else(|| panic!("§12 seed client {name} is absent"))
}

fn shape(ty: &SourceType) -> String {
    match ty {
        SourceType::TCon(name, _) | SourceType::TVar(name, _) => format!("name:{name}"),
        SourceType::TApp(function, argument, _) => {
            format!("({} {})", shape(function), shape(argument))
        }
        other => panic!("unexpected closed §12 proposition form: {other:?}"),
    }
}

fn applied<'a>(ty: &'a SourceType, name: &str) -> Vec<&'a SourceType> {
    let mut arguments = Vec::new();
    let mut head = ty;
    while let SourceType::TApp(function, argument, _) = head {
        arguments.push(argument.as_ref());
        head = function;
    }
    arguments.reverse();
    assert!(
        matches!(head, SourceType::TCon(actual, _) | SourceType::TVar(actual, _) if actual == name),
        "expected {name}, found {head:?}"
    );
    arguments
}

fn one_theorem<'a>(decls: &'a [SourceDecl], name: &str) -> (&'a SourceType, &'a SourceExpr) {
    let found = decls
        .iter()
        .filter_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name: actual,
                theorem,
                body,
                ..
            } if actual == name => Some((theorem, body)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [one] = found.as_slice() else {
        panic!("exactly one seed theorem {name} required");
    };
    *one
}

/// SOURCE: CV §12 seed. PROMISE CLASS: durable invariant.
/// MEASURED: all accepted law instances and the true same-form control check;
/// after a fresh full prefix, the isolated dropped-derivative claim rejects
/// with KernelRejected TypeMismatch at its own declaration. Parsed Ken ASTs then
/// pin the accepted control's `Proved` body, the same `IsTrue (poly_eq ...)`
/// form, and identical left expression,
/// with just the derivative omitted from the refused right expression.
/// CLAIMED: the derivative of 2*x is not canonically equal to 2*x.
/// THE GAP: these closed clients do not replace the generic checked proof.
#[test]
fn actual_seed_accepts_before_dropped_derivative_refuses_at_own_claim() {
    let accepted = seed_case("factor_law_at_variable_zero");
    let refused = seed_case("refuse_dropping_derivative");
    let negatives = parse_decls(refused).expect("§12 isolated refusal parses");
    let [SourceDecl::TheoremDecl {
        name,
        params,
        theorem,
        body,
        ..
    }] = negatives.as_slice()
    else {
        panic!("isolated refusal is exactly one closed theorem");
    };
    assert_eq!(name, "refuse_dropping_derivative");
    assert!(params.is_empty(), "false seed claim is closed");
    assert!(
        matches!(body, SourceExpr::ECon(name, _) if name == "Proved"),
        "refused theorem has the accepted control's proof form"
    );

    let mut positive = package_env();
    positive
        .elaborate_file(&checked_is_true::selected_source_with_import(accepted))
        .expect("complete actual §12 seed and same-form Proved control check");
    for name in [
        "factor_law_at_variable_zero",
        "factor_law_at_variable_product",
        "same_form_proved_control",
    ] {
        let id = positive.globals[name];
        assert!(matches!(
            positive.env.lookup(id),
            Some(Decl::Transparent { .. })
        ));
    }

    let mut negative = package_env();
    negative
        .elaborate_file(&checked_is_true::selected_source_with_import(accepted))
        .expect("fresh full seed accepted prefix checks before refusal");
    let checked_refused = checked_is_true::selected_source(refused);
    let checked_decls = parse_decls(&checked_refused).expect("selected refusal parses");
    let [SourceDecl::TheoremDecl { span: checked_span, .. }] = checked_decls.as_slice() else {
        panic!("selected refusal remains one theorem");
    };
    match negative.elaborate_file(&checked_refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start >= checked_span.start
            && span.start < span.end
            && span.end <= checked_span.end => {}
        other => panic!("dropped derivative must reject at its own claim: {other:?}"),
    }

    // Verify the checked Ken goals only after observing the actual refusal;
    // a true-neighbour mutation must first fail at the refusal assertion.
    let positives = parse_decls(accepted).expect("§12 accepted seed parses");
    let (control, control_body) = one_theorem(&positives, "same_form_proved_control");
    assert!(
        matches!(control_body, SourceExpr::ECon(name, _) if name == "Proved"),
        "accepted control must use the same Proved proof form as the refusal"
    );
    let [control_decision] = applied(control, "IsTrue")
        .try_into()
        .expect("accepted control is an IsTrue decision");
    let [refused_decision] = applied(theorem, "IsTrue")
        .try_into()
        .expect("refusal is an IsTrue decision");
    let [control_left, control_right] = applied(control_decision, "poly_eq")
        .try_into()
        .expect("accepted control decides canonical equality");
    let [refused_left, refused_right] = applied(refused_decision, "poly_eq")
        .try_into()
        .expect("refusal decides canonical equality");
    assert_eq!(
        shape(control_left),
        shape(refused_left),
        "same derivative on left"
    );
    assert_eq!(
        shape(control_right),
        "((name:EMul (name:EConst name:two)) ((name:deriv name:Zero) (name:EVar name:Zero)))"
    );
    assert_eq!(
        shape(refused_right),
        "((name:EMul (name:EConst name:two)) (name:EVar name:Zero))",
        "only the derivative is dropped on the right"
    );
}
