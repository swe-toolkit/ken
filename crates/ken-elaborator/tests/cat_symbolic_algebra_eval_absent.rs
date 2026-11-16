//! Syntactic absence makes evaluation invariant: spec 66 §19.
//! Promise class: durable invariant. Open clients check the generic type and
//! its derivative consequence; distinct closed values discriminate the claim.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const LAW: &str = "eval_absent";
const ALIAS_WITNESS: &str = "theorem selected_true : IsTrue True = Proved\n";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

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

fn selected_client(client: &str) -> String {
    // The alias witness selects the checked IsTrue identity explicitly in
    // the same env that root-loaded its provider before the client.
    checked_is_true::selected_source_with_import(&format!("{client}\n{ALIAS_WITNESS}"))
}

fn term_contains_const(term: &Term, owner_id: GlobalId) -> bool {
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
  (Expr, occurs, eval, deriv, eval_absent, deriv_independent)

theorem evaluation_ignores_absent
      (v : Nat) (e : Expr) (env : Nat → Nat)
      (absent : Equal Bool (occurs v e) False)
      (env2 : Nat → Nat)
      (agree : (w : Nat) → (Equal Nat v w → Bottom) → Equal Nat (env2 w) (env w))
    : Equal Nat (eval env2 e) (eval env e) =
  eval_absent v e env absent env2 agree

theorem absent_derivative_zero
      (v : Nat) (e : Expr) (env : Nat → Nat)
      (absent : Equal Bool (occurs v e) False)
    : Equal Nat (eval env (deriv v e)) Zero =
  deriv_independent v e env (eval_absent v e env absent)
"#;

const ACCEPTED: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, occurs, eval, eval_absent)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const five : Nat = Suc (Suc three)
const seven : Nat = Suc (Suc five)
const eleven : Nat = Suc (Suc (Suc (Suc seven)))

const fixture : Expr = EAdd (EVar Zero) (EMul (EConst two) (EVar one))

fn ones (w : Nat) : Nat = one

fn changed_at_two (w : Nat) : Nat =
  match w {
    Zero ↦ one;
    Suc a ↦
      match a {
        Zero ↦ one;
        Suc b ↦
          match b {
            Zero ↦ seven;
            Suc c ↦ one
          }
      }
  }

fn changed_at_one (w : Nat) : Nat =
  match w {
    Zero ↦ one;
    Suc a ↦
      match a {
        Zero ↦ five;
        Suc b ↦ one
      }
  }

theorem agree_off_two (w : Nat)
    : (Equal Nat two w → Bottom) → Equal Nat (changed_at_two w) (ones w) =
  match w {
    Zero ↦ λne. Proved;
    Suc a ↦
      match a {
        Zero ↦ λne. Proved;
        Suc b ↦
          match b {
            Zero ↦ λne. absurd (ne Proved);
            Suc c ↦ λne. Proved
          }
      }
  }

theorem agree_off_one (w : Nat)
    : (Equal Nat one w → Bottom) → Equal Nat (changed_at_one w) (ones w) =
  match w {
    Zero ↦ λne. Proved;
    Suc a ↦
      match a {
        Zero ↦ λne. absurd (ne Proved);
        Suc b ↦ λne. Proved
      }
  }

theorem absent_premise : Equal Bool (occurs two fixture) False = Proved

theorem value_at_ones : Equal Nat (eval ones fixture) three = Proved

theorem value_at_absent_change : Equal Nat (eval changed_at_two fixture) three = Proved

theorem value_at_occurring_change : Equal Nat (eval changed_at_one fixture) eleven = Proved

theorem closed_absent_law
    : Equal Nat (eval changed_at_two fixture) (eval ones fixture) =
  eval_absent two fixture ones absent_premise changed_at_two agree_off_two
"#;

const FALSE_ABSENCE: &str = r#"theorem occurring_variable_absent
    : Equal Nat (eval changed_at_one fixture) (eval ones fixture) =
  eval_absent one fixture ones Proved changed_at_one agree_off_one
"#;

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §19, Architect AC-0.
/// PROMISE CLASS: durable invariant. MEASURED: five providers precede the
/// owner module, whose trusted GlobalId set must exactly match the pre-load
/// set. The public owner is Transparent; selective open clients use its
/// checked Const both directly and via deriv_independent's h.
/// CLAIMED: the premise-free conclusion after absence and agreement is
/// generally reusable with zero new trust. THE GAP: inherited trust persists;
/// no closed fixture can replace the open checked proof body.
#[test]
fn exact_trust_owner_and_open_consequence_check() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("ruled generic eval_absent proof kernel-checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        after, before,
        "eval_absent adds no trusted GlobalId identity"
    );

    let qualified = format!("{PACKAGE}.{LAW}");
    let owner_id = *env
        .globals
        .get(&qualified)
        .expect("owner law global exists");
    assert!(owned.contains(&owner_id), "NatPolynomial owns eval_absent");
    assert!(
        matches!(env.env.lookup(owner_id), Some(Decl::Transparent { .. })),
        "eval_absent is an owner-local checked Transparent body"
    );
    env.elaborate_file(&selected_client(GENERIC_CLIENT))
        .expect("selective open law and independent derivative consequence check");
    for name in ["evaluation_ignores_absent", "absent_derivative_zero"] {
        let id = env.globals[name];
        let (_, body) = env
            .env
            .transparent_body(id)
            .unwrap_or_else(|| panic!("{name} carries a checked body"));
        assert!(
            term_contains_const(&body, owner_id),
            "{name} must contain the exact owner Const, not a same-named surrogate"
        );
    }
}

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §19, AC-2 accepted case.
/// PROMISE CLASS: durable invariant with independently written closed values.
/// MEASURED: the accepted whole client has an absence decision for v=2,
/// all-one and changed-at-two values 3, changed-at-one value 11, the
/// two pointwise agreement functions, and the checked law application.
/// CLAIMED: an absent variable does not alter this expression's evaluation.
/// THE GAP: closed examples are not a proof for generic Expr or environment.
#[test]
fn closed_absent_change_and_occurring_change_are_distinct() {
    loaded_package()
        .elaborate_file(&selected_client(ACCEPTED))
        .expect("full accepted premise, two agreement functions, values and law check");
}

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §19, AC-2 refusal.
/// PROMISE CLASS: durable invariant with a same-form false application.
/// MEASURED: after a full accepted prefix in a fresh env, applying the same
/// law with changed-at-one and its valid off-one agreement but a false
/// absence premise refuses KernelRejected(TypeMismatch), in its own theorem
/// with a span enclosing the Proved token. The expected goal carries the
/// resolved occurs owner Const, so the refusal is attributed to absence.
/// CLAIMED: a variable that occurs cannot use a false absence decision.
/// THE GAP: syntactic absence is sufficient but not necessary for semantic
/// independence; this fixture does not claim the converse.
#[test]
fn false_absence_refuses_at_its_own_premise() {
    let mut env = loaded_package();
    env.elaborate_file(&selected_client(ACCEPTED))
        .expect("fresh whole accepted prefix before isolated X");
    let occurs_id = env.globals[&format!("{PACKAGE}.occurs")];
    let parsed = parse_decls(FALSE_ABSENCE).expect("X theorem parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("exactly one isolated refused theorem");
    };
    assert_eq!(name, "occurring_variable_absent");
    let token = FALSE_ABSENCE
        .rfind("Proved")
        .expect("own proof premise token");
    match env
        .elaborate_file(FALSE_ABSENCE)
        .expect_err("false absence must refuse")
    {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, .. },
            span,
        } if claim.start <= span.start
            && span.start <= token
            && token + "Proved".len() <= span.end
            && span.end <= claim.end
            && term_contains_const(&expected, occurs_id) => {}
        other => panic!("X must reject at false occurs premise with exact owner: {other:?}"),
    }
}
