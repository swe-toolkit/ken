//! Differentiation preserves absence (Symbolic Algebra §24, AC-0 and AC-2).

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const LAW: &str = "deriv_fresh";

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

fn assert_cites(env: &ElabEnv, client: &str, laws: &[&str]) {
    let (_, body) = env
        .env
        .transparent_body(env.globals[client])
        .expect("checked client body");
    for law in laws {
        let owner = env.globals[&format!("{PACKAGE}.{law}")];
        assert!(
            contains_const(&body, owner),
            "{client} cites the {law} Const"
        );
    }
}

const OPEN_CLIENT: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial (Expr, deriv, occurs, eval, deriv_fresh, deriv_absent)

theorem second_deriv_absent
      (u : Nat) (v : Nat) (e : Expr) (env : Nat → Nat)
      (absent : Equal Bool (occurs u e) False)
    : Equal Nat (eval env (deriv u (deriv v e))) Zero =
  deriv_absent u (deriv v e) env (deriv_fresh u v e absent)
"#;

const FIXTURE: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial (Expr, EVar, EMul, occurs, deriv, deriv_fresh)

const one : Nat = Suc Zero
const two : Nat = Suc one
const seed : Expr = EMul (EVar Zero) (EVar one)
"#;

fn freshness_claims(var: &str) -> String {
    format!(
        "theorem premise_{var} : Equal Bool (occurs {var} seed) False = Proved

theorem fresh_{var} : Equal Bool (occurs {var} (deriv Zero seed)) False =
  deriv_fresh {var} Zero seed premise_{var}
"
    )
}

fn accepted_prefix() -> ElabEnv {
    let mut env = package_env();
    env.elaborate_file(&format!("{FIXTURE}\n{}", freshness_claims("two")))
        .expect("the complete variable-two prefix checks in a fresh environment");
    env
}

fn assert_refuses_occurrence_claim(source: &str, declaration: &str) {
    let mut env = accepted_prefix();
    let occurs = env.globals[&format!("{PACKAGE}.occurs")];
    let parsed = parse_decls(source).expect("the negative client parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }, ..] = parsed.as_slice()
    else {
        panic!("expected an occurrence theorem first");
    };
    assert_eq!(name, declaration);
    let proof = source.find("Proved").expect("closed occurrence proof");
    match env.elaborate_file(source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, .. },
            span,
        }) if claim.start <= span.start
            && span.start <= proof
            && proof + "Proved".len() <= span.end
            && span.end <= claim.end
            && contains_const(&expected, occurs) =>
        {
            eprintln!("{declaration}: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("must reject at {declaration}: {other:?}"),
    }
}

/// Source: Symbolic Algebra §24, AC-0 and AC-T. Durable invariant.
/// MEASURED: exact provider-first trust identities and an owned, Transparent
/// law cited alongside `deriv_absent` by an open client. CLAIMED: the public
/// law adds no trusted identities and composes under only the absence premise.
/// GAP: this does not audit providers' trust. Proof rewrites and new proved
/// exports are compatible; removal or an added assumption is not.
#[test]
fn exact_trust_owner_and_open_second_derivative() {
    let mut env = provider_env();
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&roots(), PACKAGE)
        .expect("NatPolynomial roots-loads");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "NatPolynomial trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "NatPolynomial adds no trusted GlobalId");

    env.elaborate_file(OPEN_CLIENT)
        .expect("the open second-derivative composition checks");
    let owner = env.globals[&format!("{PACKAGE}.{LAW}")];
    assert!(owned.contains(&owner), "law belongs to NatPolynomial");
    assert!(matches!(
        env.env.lookup(owner),
        Some(Decl::Transparent { .. })
    ));
    assert_cites(&env, "second_deriv_absent", &[LAW, "deriv_absent"]);
}

/// Source: Symbolic Algebra §24, AC-2. Durable invariant.
/// MEASURED: the product fixture checks a closed premise and client with
/// variable two, and the client cites the owned law. CLAIMED: differentiation
/// preserves this absence. GAP: this row does not pin the generic signature
/// or execute the conformance seed. Proof rewrites are compatible; refusing
/// this absent variable or bypassing the law would defeat the exercised use.
#[test]
fn absent_variable_two_stays_absent_by_law() {
    let env = accepted_prefix();
    assert_cites(&env, "fresh_two", &[LAW]);
}

/// Source: Symbolic Algebra §24, AC-2 as ruled in AC-0. Durable invariant.
/// MEASURED: querying variable one refuses at the premise and, independently,
/// at the conclusion, in each case covering the `Proved` in the named theorem.
/// CLAIMED: this derivative keeps an occurrence of one. GAP: the premise
/// failure does not reach the law application. Spans within each declaration
/// covering the proof are compatible; accepting either false claim is not.
#[test]
fn retained_variable_one_refuses_premise_and_direct_conclusion() {
    assert_refuses_occurrence_claim(&freshness_claims("one"), "premise_one");
    assert_refuses_occurrence_claim(
        "theorem result_one : Equal Bool (occurs one (deriv Zero seed)) False = Proved\n",
        "result_one",
    );
}

/// Source: AC-0, the two proof helpers are private. Durable invariant.
/// MEASURED: neither name is selectable across the package boundary.
/// CLAIMED: the helper implementations are not public API. GAP: this is not
/// a full export census. Private refactoring is compatible; publishing either
/// helper would break the ruled boundary.
#[test]
fn differentiation_freshness_helpers_stay_private() {
    for helper in ["var_deriv_fresh", "bool_or_false_both"] {
        let qualified = format!("{PACKAGE}.{helper}");
        match package_env().elaborate_file(&format!("import {PACKAGE} ({helper})")) {
            Err(ElabError::UnboundName { name, .. }) if name == qualified => {}
            other => panic!("{qualified} must refuse selective import: {other:?}"),
        }
    }
}
