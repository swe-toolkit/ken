//! Freshness after substitution (Spec 66 §23, including AC-2's variable pair).

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const LAW: &str = "substitute_fresh";

fn roots() -> [PathBuf; 1] {
    [PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

fn provider_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
        "Data.Collections.Derived",
    ] {
        env.elaborate_module_from_roots(&roots(), provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    env
}

fn package_env() -> ElabEnv {
    let mut env = provider_env();
    env.elaborate_module_from_roots(&roots(), PACKAGE)
        .expect("NatPolynomial roots-loads");
    env
}

fn selected_client(client: &str) -> String {
    checked_is_true::selected_source_with_import(&format!(
        "{client}\ntheorem selected_true : IsTrue True = Proved\n"
    ))
}

fn contains_const(term: &Term, owner: GlobalId) -> bool {
    let mut pending = vec![term];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

fn assert_cites(env: &ElabEnv, client: &str, providers: &[&str]) {
    let (_, body) = env
        .env
        .transparent_body(env.globals[client])
        .expect("checked client body");
    for provider in providers {
        let owner = env.globals[&format!("{PACKAGE}.{provider}")];
        assert!(
            contains_const(&body, owner),
            "{client} cites the {provider} Const"
        );
    }
}

const OPEN_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, occurs, substitute, substitute_fresh, eval, eval_absent, deriv, deriv_absent)

theorem open_subst_fresh
      (v : Nat) (s : Nat → Expr) (e : Expr)
      (fresh : (w : Nat) → Equal Bool (occurs w e) True → Equal Bool (occurs v (s w)) False)
    : Equal Bool (occurs v (substitute s e)) False =
  substitute_fresh v s e fresh

theorem eval_after_fresh_subst
      (v : Nat) (s : Nat → Expr) (e : Expr) (env : Nat → Nat) (env2 : Nat → Nat)
      (fresh : (w : Nat) → Equal Bool (occurs w e) True → Equal Bool (occurs v (s w)) False)
      (agree : (w : Nat) → (Equal Nat v w → Bottom) → Equal Nat (env2 w) (env w))
    : Equal Nat (eval env2 (substitute s e)) (eval env (substitute s e)) =
  eval_absent v (substitute s e) env (substitute_fresh v s e fresh) env2 agree

theorem deriv_after_fresh_subst
      (v : Nat) (s : Nat → Expr) (e : Expr) (env : Nat → Nat)
      (fresh : (w : Nat) → Equal Bool (occurs w e) True → Equal Bool (occurs v (s w)) False)
    : Equal Nat (eval env (deriv v (substitute s e))) Zero =
  deriv_absent v (substitute s e) env (substitute_fresh v s e fresh)
"#;

const FIXTURES: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, occurs, substitute, substitute_fresh)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two

const fixture : Expr = EAdd (EVar Zero) (EVar one)

fn moved (w : Nat) : Expr =
  match w {
    Zero ↦ EVar two;
    Suc a ↦
      match a {
        Zero ↦ EConst three;
        Suc b ↦ EVar (Suc (Suc b))
      }
  }

theorem one_in_input : Equal Bool (occurs one fixture) True = Proved
theorem two_not_in_input : Equal Bool (occurs two fixture) False = Proved
theorem one_not_in_result : Equal Bool (occurs one (substitute moved fixture)) False = Proved
theorem two_in_result : Equal Bool (occurs two (substitute moved fixture)) True = Proved
"#;

fn freshness_claims(var: &str) -> String {
    format!(
        "theorem fresh_premise_{var} (w : Nat)
    : Equal Bool (occurs w fixture) True → Equal Bool (occurs {var} (moved w)) False =
  match w {{
    Zero ↦ λh. Proved;
    Suc a ↦
      match a {{
        Zero ↦ λh. Proved;
        Suc b ↦ λh. absurd h
      }}
  }}

theorem fresh_after_{var} : Equal Bool (occurs {var} (substitute moved fixture)) False =
  substitute_fresh {var} moved fixture fresh_premise_{var}
"
    )
}

fn accepted_prefix() -> String {
    selected_client(&format!("{FIXTURES}\n{}", freshness_claims("one")))
}

/// Source: Spec 66 §23 and the work item's AC-T. Durable invariant.
/// MEASURED: exact provider-first trust identities, a package-owned Transparent
/// law, and open clients citing its Const alone and with each absence law.
/// CLAIMED: the package adds no trusted identity and exports the generic law
/// for both consumers. THE GAP: this does not audit providers' own trust.
/// Additional proved exports are compatible; a new trusted identity is not.
#[test]
fn exact_trust_owner_and_open_freshness_consumers() {
    let mut env = provider_env();
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&roots(), PACKAGE)
        .expect("substitute_fresh proof kernel-checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "NatPolynomial trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "NatPolynomial adds no trusted GlobalId");

    let owner = env.globals[&format!("{PACKAGE}.{LAW}")];
    assert!(owned.contains(&owner), "law belongs to NatPolynomial");
    assert!(matches!(
        env.env.lookup(owner),
        Some(Decl::Transparent { .. })
    ));
    env.elaborate_file(&selected_client(OPEN_CLIENT))
        .expect("the open law and both absence compositions check");
    assert_cites(&env, "open_subst_fresh", &[LAW]);
    assert_cites(&env, "eval_after_fresh_subst", &[LAW, "eval_absent"]);
    assert_cites(&env, "deriv_after_fresh_subst", &[LAW, "deriv_absent"]);
}

/// Source: Spec 66 §23, work item AC-2. Durable invariant.
/// MEASURED: independent input/result occurrence decisions and a closed
/// variable-one client citing the law's Const. CLAIMED: the checked law
/// supplies absence after substitution, not absence in the original tree.
/// THE GAP: these closed decisions only cover this fixture; the generic
/// client checks the signature separately. Proof rewrites are compatible;
/// replacing the law use with a direct decision would no longer measure it.
#[test]
fn used_replacements_leave_variable_one_fresh() {
    let mut env = package_env();
    env.elaborate_file(&accepted_prefix())
        .expect("independent value decisions and closed variable-one client check");
    assert_cites(&env, "fresh_after_one", &[LAW]);
}

/// Source: Spec 66 §23, work item AC-2. Durable invariant.
/// MEASURED: changing the queried variable to two rejects the same proof
/// body in its freshness-premise declaration, whose expected type cites
/// the package's `occurs`. CLAIMED: variable two, introduced by replacement
/// zero, cannot satisfy the premise at that occurring index. THE GAP: the
/// error covers the declaration, not necessarily just the zero arm; the
/// application is not reached. Different spans within the declaration are
/// compatible if they still cover the false proof; accepting it breaks.
#[test]
fn introduced_variable_two_refuses_at_its_freshness_premise() {
    let mut env = package_env();
    env.elaborate_file(&accepted_prefix())
        .expect("complete accepted prefix in a fresh environment");
    let occurs = env.globals[&format!("{PACKAGE}.occurs")];
    let refused = freshness_claims("two");
    let parsed = parse_decls(&refused).expect("both refused-client declarations parse");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }, SourceDecl::TheoremDecl { .. }] = parsed.as_slice()
    else {
        panic!("one premise followed by one law application");
    };
    assert_eq!(name, "fresh_premise_two");
    let proof = refused.find("Proved").expect("zero-arm proof");
    match env.elaborate_file(&refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, .. },
            span,
        }) if claim.start <= span.start
            && span.start <= proof
            && proof + "Proved".len() <= span.end
            && span.end <= claim.end
            && contains_const(&expected, occurs) =>
        {
            eprintln!("variable-two premise: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("must reject at the variable-two freshness premise: {other:?}"),
    }
}
