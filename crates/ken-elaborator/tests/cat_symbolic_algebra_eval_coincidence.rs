//! Evaluation depends only on occurring variables: Architect AC-0 for spec 66 §21.
//! Promise class: durable invariant. Open checked clients and exact trust
//! identities pin the general law; distinct closed values discriminate refusal.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const LAW: &str = "eval_coincidence";
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

const OPEN_CLIENT: &str = r#"
import Core.Logic.Transport (cong, sym, trans)
import Algorithm.SymbolicAlgebra.NatPolynomial (Expr, occurs, eval, eval_coincidence)

theorem open_coincidence
      (e : Expr) (env1 : Nat → Nat) (env2 : Nat → Nat)
      (agreement : (w : Nat) → Equal Bool (occurs w e) True → Equal Nat (env1 w) (env2 w))
    : Equal Nat (eval env1 e) (eval env2 e) =
  eval_coincidence e env1 env2 agreement

theorem absent_as_coincidence
      (v : Nat) (e : Expr) (env : Nat → Nat)
      (absent : Equal Bool (occurs v e) False)
      (env2 : Nat → Nat)
      (agree : (w : Nat) → (Equal Nat v w → Bottom) → Equal Nat (env2 w) (env w))
    : Equal Nat (eval env2 e) (eval env e) =
  eval_coincidence
    e
    env2
    env
    (λw. λh.
      agree w
        (λsame.
          trans Bool False (occurs v e) True
            (sym Bool (occurs v e) False absent)
            (trans Bool (occurs v e) (occurs w e) True
              (cong Nat Bool v w (λi. occurs i e) same) h)))
"#;

const ACCEPTED: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, occurs, eval, eval_coincidence)

const one : Nat = Suc Zero
const two : Nat = Suc one
const three : Nat = Suc two
const five : Nat = Suc (Suc three)
const seven : Nat = Suc (Suc five)
const nine : Nat = Suc (Suc seven)

const fixture : Expr = EAdd (EVar Zero) (EMul (EConst two) (EVar one))
const fixture_two : Expr = EAdd (EVar two) (EMul (EConst two) (EVar one))

fn ones (w : Nat) : Nat = one

fn changed_at_two_three (w : Nat) : Nat =
  match w {
    Zero ↦ one;
    Suc a ↦
      match a {
        Zero ↦ one;
        Suc b ↦
          match b {
            Zero ↦ seven;
            Suc c ↦
              match c {
                Zero ↦ five;
                Suc d ↦ one
              }
          }
      }
  }

theorem agree_on_fixture (w : Nat)
    : Equal Bool (occurs w fixture) True
      → Equal Nat (changed_at_two_three w) (ones w) =
  match w {
    Zero ↦ λh. Proved;
    Suc a ↦
      match a {
        Zero ↦ λh. Proved;
        Suc b ↦
          match b {
            Zero ↦ λh. absurd h;
            Suc c ↦
              match c {
                Zero ↦ λh. absurd h;
                Suc d ↦ λh. Proved
              }
          }
      }
  }

theorem value_at_ones : Equal Nat (eval ones fixture) three = Proved
theorem value_at_changes : Equal Nat (eval changed_at_two_three fixture) three = Proved
theorem value_two_at_ones : Equal Nat (eval ones fixture_two) three = Proved
theorem value_two_at_changes : Equal Nat (eval changed_at_two_three fixture_two) nine = Proved

theorem closed_coincidence
    : Equal Nat (eval changed_at_two_three fixture) (eval ones fixture) =
  eval_coincidence fixture changed_at_two_three ones agree_on_fixture
"#;

const FALSE_AGREEMENT: &str = r#"theorem agree_on_fixture_two (w : Nat)
    : Equal Bool (occurs w fixture_two) True
      → Equal Nat (changed_at_two_three w) (ones w) =
  match w {
    Zero ↦ λh. Proved;
    Suc a ↦
      match a {
        Zero ↦ λh. Proved;
        Suc b ↦
          match b {
            Zero ↦ λh. absurd h;
            Suc c ↦
              match c {
                Zero ↦ λh. absurd h;
                Suc d ↦ λh. Proved
              }
          }
      }
  }
"#;

/// SOURCE: Architect AC-0 `evt_7k0fe3tsc76s6`; spec 66 §§19–21.
/// PROMISE CLASS: durable invariant. MEASURED: provider-first exact trusted
/// GlobalId sets, owner-local Transparent declaration, and two checked open
/// client bodies with the owner Const and no eval_absent Const. CLAIMED: the
/// generic law and its eval_absent-typed consequence add no new trust. THE GAP:
/// root-only loading is not measured; inherited provider trust persists.
#[test]
fn exact_trust_owner_open_law_and_absent_consequence() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("generic coincidence proof kernel-checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        before, after,
        "NatPolynomial adds no trusted GlobalId identity"
    );
    let owner = env.globals[&format!("{PACKAGE}.{LAW}")];
    let absent = env.globals[&format!("{PACKAGE}.eval_absent")];
    assert!(owned.contains(&owner), "law belongs to NatPolynomial");
    assert!(
        matches!(env.env.lookup(owner), Some(Decl::Transparent { .. })),
        "owner law is Transparent and checked"
    );
    env.elaborate_file(&selected_client(OPEN_CLIENT))
        .expect("open law and independent eval_absent-typed consequence check");
    for name in ["open_coincidence", "absent_as_coincidence"] {
        let (_, body) = env
            .env
            .transparent_body(env.globals[name])
            .expect("checked client body");
        assert!(
            term_contains_const(&body, owner),
            "{name} uses exact law Const"
        );
        assert!(
            !term_contains_const(&body, absent),
            "{name} may not invoke the retained eval_absent proof"
        );
    }
}

/// SOURCE: Architect AC-0 accepted fixture, spec 66 §21.
/// PROMISE CLASS: durable invariant. MEASURED: the whole accepted program
/// checks both independent evaluation values at 3, and the changed fixture
/// values at 3 and 9; agreement at every occurring index discharges the law.
/// CLAIMED: differences at two absent indices cannot change evaluation.
/// THE GAP: closed observations cannot establish the generic theorem.
#[test]
fn two_absent_changes_preserve_evaluation_but_occurring_change_does_not() {
    loaded_package()
        .elaborate_file(&selected_client(ACCEPTED))
        .expect("agreement, independent evaluations and law application check");
}

/// SOURCE: Architect AC-0 rejected fixture, spec 66 §21.
/// PROMISE CLASS: durable invariant. MEASURED: after a full accepted prefix,
/// changing only the fixture's EVar Zero to EVar two makes the same proof fail
/// at its own index-two `absurd h` within the agreement declaration; found
/// carries the resolved owner `occurs` Const. CLAIMED: pointwise agreement
/// cannot be asserted where an occurring variable's values differ.
/// THE GAP: this is one distinguishing instance, not the generic proof.
#[test]
fn occurring_index_refuses_at_its_agreement_premise() {
    let mut env = loaded_package();
    env.elaborate_file(&selected_client(ACCEPTED))
        .expect("complete accepted prefix in a fresh environment");
    let occurs = env.globals[&format!("{PACKAGE}.occurs")];
    let parsed = parse_decls(FALSE_AGREEMENT).expect("refused theorem parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("one isolated refused theorem");
    };
    assert_eq!(name, "agree_on_fixture_two");
    let token = FALSE_AGREEMENT
        .find("absurd h")
        .expect("index-two false agreement token");
    match env
        .elaborate_file(FALSE_AGREEMENT)
        .expect_err("index two occurs")
    {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { found, .. },
            span,
        } if claim.start <= span.start
            && span.start <= token
            && token < span.end
            && span.end <= claim.end
            && term_contains_const(&found, occurs) => {}
        other => panic!("must reject at index-two occurs agreement: {other:?}"),
    }
}
