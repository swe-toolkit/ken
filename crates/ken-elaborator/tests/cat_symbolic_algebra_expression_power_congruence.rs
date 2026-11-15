//! Expression powers under canonical equality and substitution: spec 66 §18.
//! Promise class: durable invariant. Open clients pin both public law types;
//! closed, non-degenerate inputs distinguish the two equality notions.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

// These five providers bind IsTrue before the in-process client elaborates.
// Importing IsTrue again would introduce an ambiguous spelling.
fn load_providers(env: &mut ElabEnv) {
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
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial proof bodies check");
    env
}

fn body_contains_const(term: &Term, owner_id: GlobalId) -> bool {
    let mut pending = vec![term];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner_id) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

const GENERIC_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, poly_eq, expr_pow, substitute, poly_eq_expr_pow, substitute_expr_pow)

theorem congruence_generic
      (n : Nat) (e1 : Expr) (e2 : Expr) (h : IsTrue (poly_eq e1 e2))
    : IsTrue (poly_eq (expr_pow e1 n) (expr_pow e2 n)) =
  poly_eq_expr_pow n e1 e2 h

theorem substitution_generic
      (s : Nat → Expr) (e : Expr) (n : Nat)
    : Equal Expr (substitute s (expr_pow e n)) (expr_pow (substitute s e) n) =
  substitute_expr_pow s e n
"#;

const ACCEPTED: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, poly_eq, expr_pow, substitute,
   poly_eq_expr_pow, substitute_expr_pow)

const one : Nat = Suc Zero
const two : Nat = Suc one

const x0 : Expr = EVar Zero
const x1 : Expr = EVar one
const sum01 : Expr = EAdd x0 x1
const sum10 : Expr = EAdd x1 x0

theorem commuted_sums_premise : IsTrue (poly_eq sum01 sum10) = Proved

theorem c_closed_squares : IsTrue (poly_eq (expr_pow sum01 two) (expr_pow sum10 two)) = Proved

theorem c_law_squares : IsTrue (poly_eq (expr_pow sum01 two) (expr_pow sum10 two)) =
  poly_eq_expr_pow two sum01 sum10 commuted_sums_premise

fn replace_zero (v : Nat) : Expr =
  match v {
    Zero ↦ EAdd x1 (EConst two);
    Suc w ↦ EVar (Suc w)
  }

const replacement : Expr = EAdd x1 (EConst two)
const substituted_square : Expr = EMul (EMul (EConst one) replacement) replacement
const unsubstituted_square : Expr = EMul (EMul (EConst one) x0) x0

theorem s_closed_substituted_power
    : Equal Expr (substitute replace_zero (expr_pow x0 two)) substituted_square =
  Refl

theorem s_closed_power_of_substituted
    : Equal Expr (expr_pow (substitute replace_zero x0) two) substituted_square =
  Refl

theorem s_closed_unsubstituted_power : Equal Expr (expr_pow x0 two) unsubstituted_square = Refl

theorem s_law_square
    : Equal Expr (substitute replace_zero (expr_pow x0 two)) (expr_pow (substitute replace_zero x0) two) =
  substitute_expr_pow replace_zero x0 two
"#;

const X_BASE: &str =
    "theorem x_base_squares : IsTrue (poly_eq (expr_pow x0 two) (expr_pow x1 two)) = Proved\n";
const X_DROP_SUB: &str = "theorem x_drop_substitution : Equal Expr (substitute replace_zero (expr_pow x0 two)) (expr_pow x0 two) = Refl\n";
const X_STRUCT: &str =
    "theorem x_structural_squares : Equal Expr (expr_pow sum01 two) (expr_pow sum10 two) = Refl\n";

/// MEASURED: after the same five providers, the exact trusted GlobalId set
/// is unchanged when NatPolynomial loads; both laws belong to it and have
/// Transparent bodies. Selective-import clients check at arbitrary terms,
/// and each checked client body contains its own law's checked owner Const.
/// CLAIMED: both public types are reusable, kernel-checked, and add no local
/// trust. THE GAP: inherited provider trust is not erased; the closed rows
/// below independently exercise distinctions that an open proof cannot show.
#[test]
fn open_laws_are_owned_transparent_selectively_imported_and_trust_free() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("both law bodies kernel-check");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(after, before, "no new trusted GlobalId identities");
    for law in ["poly_eq_expr_pow", "substitute_expr_pow"] {
        let id = env.globals[&format!("{PACKAGE}.{law}")];
        assert!(owned.contains(&id), "{law} belongs to NatPolynomial");
        assert!(
            matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
            "{law} carries a checked proof body"
        );
    }
    env.elaborate_file(&checked_is_true::selected_source_with_import(
        GENERIC_CLIENT,
    ))
    .expect("W-GEN selective import checks both open law types");
    for (client, law) in [
        ("congruence_generic", "poly_eq_expr_pow"),
        ("substitution_generic", "substitute_expr_pow"),
    ] {
        let law_id = env.globals[&format!("{PACKAGE}.{law}")];
        let client_id = env.globals[client];
        let (_, body) = env
            .env
            .transparent_body(client_id)
            .expect("the generic client has a checked body");
        assert!(
            body_contains_const(&body, law_id),
            "{client} must use the owner identity of {law}"
        );
    }
}

/// MEASURED: the whole accepted block checks a nontrivial commuted-sum
/// `poly_eq` premise, its square by closed decision and by the exported law,
/// and both structurally equal substituted powers and their exported law.
/// CLAIMED: the operations and public laws agree on closed, non-identity
/// substitution and on syntactically different but canonically equal bases.
/// THE GAP: these cases do not replace the checked generic proof bodies.
#[test]
fn closed_commuted_squares_and_substituted_square_accept_together() {
    loaded_package()
        .elaborate_file(&checked_is_true::selected_source_with_import(ACCEPTED))
        .expect("every accepted W-CLOSED and W-LAW theorem checks together");
}

fn isolated_refusal(name: &str, negative: &str, terminal: &str) -> ElabError {
    // In a fresh environment, the accepted prefix imports the checked
    // IsTrue identity under a distinct alias. Only this negative follows.
    let mut env = loaded_package();
    env.elaborate_file(&checked_is_true::selected_source_with_import(ACCEPTED))
        .unwrap_or_else(|error| panic!("{name} accepted prefix: {error:?}"));
    let checked_negative = checked_is_true::selected_source(negative);
    let claims = parse_decls(&checked_negative)
        .expect("selected negative fixture parses")
        .into_iter()
        .filter_map(|declaration| match declaration {
            SourceDecl::TheoremDecl {
                name: declared,
                span,
                ..
            } if declared == name => Some(span),
            _ => None,
        })
        .collect::<Vec<_>>();
    let [claim] = claims.as_slice() else {
        panic!("{name} must occur as one negative theorem");
    };
    let error = env
        .elaborate_file(&checked_negative)
        .expect_err("isolated false claim must refuse");
    let span = match &error {
        ElabError::KernelRejected { span, .. } | ElabError::TypeMismatch { span, .. } => span,
        other => panic!("{name} must refuse at its proof, not earlier: {other:?}"),
    };
    assert!(
        claim.start <= span.start && span.start < span.end && span.end <= claim.end,
        "{name} must refuse inside its own theorem: {span:?} vs {claim:?}"
    );
    let terminal_start = checked_negative
        .rfind(terminal)
        .expect("terminal in selected negative");
    assert!(
        terminal_start >= claim.start
            && terminal_start + terminal.len() <= claim.end
            && span.start <= terminal_start
            && terminal_start + terminal.len() <= span.end,
        "{name} refusal must include its own {terminal} proof"
    );
    if terminal == "Refl" {
        assert_eq!(&checked_negative[span.start..span.end], "Refl");
    }
    error
}

/// MEASURED: after a fresh accepted prefix, the distinct-variable square
/// claim refuses KernelRejected(TypeMismatch) in its own theorem, enclosing
/// its `Proved` terminal. CLAIMED: the base `poly_eq` premise is necessary
/// for nonzero exponents. THE GAP: at exponent zero both powers are one, so
/// this is a nonzero, non-degenerate counterexample, not a universal claim.
#[test]
fn distinct_variable_bases_refuse_without_a_poly_eq_premise() {
    match isolated_refusal("x_base_squares", X_BASE, "Proved") {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            ..
        } => {}
        other => panic!("X-BASE must reject by kernel TypeMismatch: {other:?}"),
    }
}

/// MEASURED: the accepted prefix checks both sides' distinct expression
/// trees, then the false claim with substitution dropped refuses exactly at
/// its own `Refl`. CLAIMED: the substitution law cannot erase replacement of
/// variable zero by a non-variable expression. THE GAP: this is a finite
/// discriminator; the open checked law covers arbitrary s and n.
#[test]
fn dropping_substitution_from_the_square_refuses_at_its_refl() {
    match isolated_refusal("x_drop_substitution", X_DROP_SUB, "Refl") {
        ElabError::TypeMismatch { reason, .. }
            if reason.contains("Refl expects an `Eq`-shaped goal") => {}
        other => panic!("X-DROP-SUB must reject by Eq-shaped Refl mismatch: {other:?}"),
    }
}

/// MEASURED: after the accepted canonical square congruence, structural
/// square equality of the commuted sums refuses exactly at its own `Refl`.
/// CLAIMED: `poly_eq` congruence does not imply equal expression trees.
/// THE GAP: this is a finite distinction, with the general public law
/// justified by its checked proof term rather than this rejection.
#[test]
fn commuted_sum_squares_refuse_structural_equality_at_their_refl() {
    match isolated_refusal("x_structural_squares", X_STRUCT, "Refl") {
        ElabError::TypeMismatch { reason, .. }
            if reason.contains("Refl expects an `Eq`-shaped goal") => {}
        other => panic!("X-STRUCT must reject by Eq-shaped Refl mismatch: {other:?}"),
    }
}
