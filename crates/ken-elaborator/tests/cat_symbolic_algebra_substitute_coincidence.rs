//! Substitution depends only on occurring variables (Spec 66 §22).

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

#[path = "support/checked_is_true.rs"]
mod checked_is_true;

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const LAW: &str = "substitute_coincidence";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
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

const OPEN_CLIENT: &str = r#"
import Core.Logic.Transport (cong, sym, trans)
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EVar, occurs, substitute, substitute_var, substitute_coincidence)

theorem open_subst_coincidence
      (e : Expr) (s : Nat → Expr) (t : Nat → Expr)
      (agreement : (w : Nat) → Equal Bool (occurs w e) True → Equal Expr (s w) (t w))
    : Equal Expr (substitute s e) (substitute t e) =
  substitute_coincidence e s t agreement

theorem absent_as_subst_coincidence
      (v : Nat) (s : Nat → Expr) (e : Expr)
      (absent : Equal Bool (occurs v e) False)
      (fixed : (w : Nat) → (Equal Nat v w → Bottom) → Equal Expr (s w) (EVar w))
    : Equal Expr (substitute s e) e =
  trans
    Expr
    (substitute s e)
    (substitute EVar e)
    e
    (substitute_coincidence
      e
      s
      EVar
      (λw. λh.
        fixed w
          (λsame.
            trans Bool False (occurs v e) True
              (sym Bool (occurs v e) False absent)
              (trans Bool (occurs v e) (occurs w e) True
                (cong Nat Bool v w (λi. occurs i e) same) h))))
    (substitute_var e)
"#;

const FIXTURES: &str = r#"
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, occurs, substitute, substitute_coincidence)

const one : Nat = Suc Zero
const two : Nat = Suc one
const seven : Nat = Suc (Suc (Suc (Suc (Suc two))))
const nine : Nat = Suc (Suc seven)

const fixture : Expr = EAdd (EVar Zero) (EConst two)
const fixture_one : Expr = EAdd (EVar one) (EConst two)
const fixture_two : Expr = EAdd (EVar two) (EConst two)

fn moved (w : Nat) : Expr =
  match w {
    Zero ↦ EVar Zero;
    Suc a ↦
      match a {
        Zero ↦ EConst seven;
        Suc b ↦
          match b {
            Zero ↦ EConst nine;
            Suc c ↦ EVar (Suc (Suc (Suc c)))
          }
      }
  }
"#;

const AGREEMENT_BODY: &str = r#"  match w {
    Zero ↦ λh. Proved;
    Suc a ↦
      match a {
        Zero ↦ λh. absurd h;
        Suc b ↦
          match b {
            Zero ↦ λh. absurd h;
            Suc c ↦ λh. Refl
          }
      }
  }
"#;

fn agreement_claim(name: &str, fixture: &str) -> String {
    format!(
        "theorem {name} (w : Nat)
    : Equal Bool (occurs w {fixture}) True → Equal Expr (moved w) (EVar w) =
{AGREEMENT_BODY}"
    )
}

const VALUE_AND_LAW_CONTROLS: &str = r#"
theorem moved_value : Equal Expr (substitute moved fixture) fixture = Refl
theorem identity_value : Equal Expr (substitute EVar fixture) fixture = Refl

theorem moved_one_value
    : Equal Expr (substitute moved fixture_one) (EAdd (EConst seven) (EConst two)) = Refl
theorem identity_one_value : Equal Expr (substitute EVar fixture_one) fixture_one = Refl
theorem moved_two_value
    : Equal Expr (substitute moved fixture_two) (EAdd (EConst nine) (EConst two)) = Refl

theorem closed_subst_coincidence
    : Equal Expr (substitute moved fixture) (substitute EVar fixture) =
  substitute_coincidence fixture moved EVar agree_on_fixture
"#;

fn accepted_prefix() -> String {
    let agreement = agreement_claim("agree_on_fixture", "fixture");
    selected_client(&format!("{FIXTURES}\n{agreement}{VALUE_AND_LAW_CONTROLS}"))
}

/// MEASURED: exact provider-first trust identities, a package-owned
/// Transparent law, and checked open users of the law's actual Const.
/// THE GAP: this compares the package's delta, not providers' own trust.
#[test]
fn exact_trust_owner_open_law_and_absent_consequence() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("substitute_coincidence proof kernel-checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    eprintln!(
        "NatPolynomial trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "NatPolynomial adds no trusted GlobalId");

    env.elaborate_file(&selected_client(OPEN_CLIENT))
        .expect("open law and substitute_absent consequence check");
    let owner = env.globals[&format!("{PACKAGE}.{LAW}")];
    let absent = env.globals[&format!("{PACKAGE}.substitute_absent")];
    let identity = env.globals[&format!("{PACKAGE}.substitute_var")];
    assert!(owned.contains(&owner), "law belongs to NatPolynomial");
    assert!(matches!(
        env.env.lookup(owner),
        Some(Decl::Transparent { .. })
    ));
    for name in ["open_subst_coincidence", "absent_as_subst_coincidence"] {
        let (_, body) = env
            .env
            .transparent_body(env.globals[name])
            .expect("checked client body");
        assert!(contains_const(&body, owner), "{name} uses the law Const");
        assert!(
            !contains_const(&body, absent),
            "{name} must not use the retained substitute_absent proof"
        );
        if name == "absent_as_subst_coincidence" {
            assert!(
                contains_const(&body, identity),
                "closes with substitute_var"
            );
        }
    }
}

/// MEASURED: both substitutions return the original fixture; when index one
/// occurs, the independent value controls compute distinct expression trees.
#[test]
fn two_absent_changes_preserve_the_expression() {
    loaded_package()
        .elaborate_file(&accepted_prefix())
        .expect("closed law application, agreement and independent values check");
}

/// MEASURED: the identical agreement body refuses at the first `absurd h`
/// when index one occurs; the found type carries the package's `occurs` Const.
#[test]
fn occurring_index_refuses_at_its_agreement_premise() {
    let mut env = loaded_package();
    env.elaborate_file(&accepted_prefix())
        .expect("complete accepted prefix in a fresh environment");
    let occurs = env.globals[&format!("{PACKAGE}.occurs")];
    let refused = agreement_claim("agree_on_fixture_one", "fixture_one");
    let parsed = parse_decls(&refused).expect("refused theorem parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("one isolated refused agreement theorem");
    };
    assert_eq!(name, "agree_on_fixture_one");
    let token = refused.find("absurd h").expect("index-one arm");
    match env.elaborate_file(&refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { found, .. },
            span,
        }) if claim.start <= span.start
            && span.start <= token
            && token + "absurd h".len() <= span.end
            && span.end <= claim.end
            && contains_const(&found, occurs) =>
        {
            eprintln!("index-one agreement: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("must reject at index-one occurs agreement: {other:?}"),
    }
}
