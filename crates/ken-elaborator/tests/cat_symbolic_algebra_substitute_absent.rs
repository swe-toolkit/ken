//! Spec 66 §20: structural substitution leaves an absent variable unchanged.
//! Promise class: durable invariant. A generic client uses both public
//! premises, and closed controls distinguish absent from present variables.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{Decl as SourceDecl, ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const LAW: &str = "substitute_absent";

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
            .unwrap_or_else(|error| panic!("provider {provider} roots-loads: {error:?}"));
    }
}

fn loaded_package() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("NatPolynomial root-loads with substitute_absent proof");
    env
}

fn term_contains_const(term: &Term, owner: GlobalId) -> bool {
    let mut pending = vec![term];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

// Whole accepted D0 client: the open law, two absent shapes, and the
// present-variable value control check in a single unbroken prefix.
const ACCEPTED: &str = r#"import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EConst, EVar, EAdd, EMul, occurs, substitute, substitute_absent)

const one : Nat = Suc Zero
const two : Nat = Suc one
const seven : Nat = Suc (Suc (Suc (Suc (Suc two))))

fn s1 (w : Nat) : Expr =
  match w {
    Zero ↦ EVar Zero;
    Suc a ↦
      match a {
        Zero ↦ EConst seven;
        Suc b ↦ EVar (Suc (Suc b))
      }
  }

theorem fixed_off_one (w : Nat)
    : (Equal Nat one w → Bottom) → Equal Expr (s1 w) (EVar w) =
  match w {
    Zero ↦ λne. Proved;
    Suc a ↦
      match a {
        Zero ↦ λne. absurd (ne Proved);
        Suc b ↦ λne. Refl
      }
  }

theorem open_law
      (v : Nat) (s : Nat → Expr) (e : Expr)
      (absent : Equal Bool (occurs v e) False)
      (fixed : (w : Nat) → (Equal Nat v w → Bottom) → Equal Expr (s w) (EVar w))
    : Equal Expr (substitute s e) e =
  substitute_absent v s e absent fixed

const sum_fixture : Expr = EAdd (EVar Zero) (EConst two)
const prod_fixture : Expr = EMul (EVar Zero) (EVar two)
const present_fixture : Expr = EAdd (EVar one) (EConst two)

theorem sum_absent : Equal Bool (occurs one sum_fixture) False = Proved
theorem prod_absent : Equal Bool (occurs one prod_fixture) False = Proved

theorem sum_unchanged : Equal Expr (substitute s1 sum_fixture) sum_fixture =
  substitute_absent one s1 sum_fixture sum_absent fixed_off_one

theorem prod_unchanged : Equal Expr (substitute s1 prod_fixture) prod_fixture =
  substitute_absent one s1 prod_fixture prod_absent fixed_off_one

theorem present_value
    : Equal Expr (substitute s1 present_fixture) (EAdd (EConst seven) (EConst two)) = Refl
"#;

const FALSE_VALUE: &str = r#"theorem present_unchanged : Equal Expr (substitute s1 present_fixture) present_fixture = Refl
"#;

const FALSE_PREMISE: &str = r#"theorem present_unchanged_by_law : Equal Expr (substitute s1 present_fixture) present_fixture =
  substitute_absent one s1 present_fixture Proved fixed_off_one
"#;

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §20, Architect AC-0.
/// PROMISE CLASS: durable invariant.
/// MEASURED: five providers load before NatPolynomial; exact trusted
/// GlobalId identities are equal before and after, and the public law is
/// owned by NatPolynomial with a Transparent checked body. Its selectively
/// imported open client contains that exact owner Const in its proof body.
/// CLAIMED: the generic substitution law adds no new trust and carries the
/// two declared premises without a same-named local surrogate.
/// THE GAP: provider assumptions remain inherited; a closed fixture cannot
/// replace the checked generic proof.
#[test]
fn exact_trust_owner_and_open_public_law_check() {
    let mut env = ElabEnv::new().expect("compiler base");
    load_providers(&mut env);
    let before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    let owned = env
        .elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("generic substitute_absent proof kernel-checks");
    let after = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    assert_eq!(before, after, "substitute_absent adds no trusted identity");

    let qualified = format!("{PACKAGE}.{LAW}");
    let owner = *env.globals.get(&qualified).expect("checked law identity");
    assert!(
        owned.contains(&owner),
        "NatPolynomial owns substitute_absent"
    );
    assert!(
        matches!(env.env.lookup(owner), Some(Decl::Transparent { .. })),
        "the law has an owner-local checked Transparent body"
    );
    env.elaborate_file(ACCEPTED)
        .expect("complete accepted client includes the open public type");
    let (_, body) = env
        .env
        .transparent_body(env.globals["open_law"])
        .expect("open client has a checked body");
    assert!(
        term_contains_const(&body, owner),
        "open_law must use the checked owner Const"
    );
}

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §20, D0 closed client.
/// PROMISE CLASS: durable invariant with fixed closed-value controls.
/// MEASURED: a full client checks sum and product absence, their public-law
/// applications with a pointwise off-one witness, and the present-variable
/// structural result against an independent Refl value.
/// CLAIMED: substitute respects syntactic absence, not all occurrences.
/// THE GAP: closed results supplement rather than prove the open theorem.
#[test]
fn full_absent_and_present_client_checks() {
    loaded_package()
        .elaborate_file(ACCEPTED)
        .expect("whole accepted D0 client checks together");
}

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §20, D0 value refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: after a fresh whole accepted prefix, the isolated false
/// present_unchanged claim refuses with ElabError::TypeMismatch precisely
/// at its own Refl, inside its parsed theorem span. The accepted
/// present_value has the identical input and proof form, changing only the
/// claimed output.
/// CLAIMED: substitution does not silently preserve a present variable.
/// THE GAP: this counterexample does not claim the converse of absence.
#[test]
fn present_value_false_unchanged_refuses_at_own_refl() {
    let mut env = loaded_package();
    env.elaborate_file(ACCEPTED)
        .expect("fresh complete accepted prefix checks first");
    let parsed = parse_decls(FALSE_VALUE).expect("false value claim parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("one isolated false value theorem");
    };
    assert_eq!(name, "present_unchanged");
    let token = FALSE_VALUE.rfind("Refl").expect("own proof terminal");
    match env
        .elaborate_file(FALSE_VALUE)
        .expect_err("false unchanged value must refuse")
    {
        ElabError::TypeMismatch { reason, span }
            if reason == "Refl expects an `Eq`-shaped goal"
                && claim.start <= span.start
                && span.start == token
                && span.end == token + "Refl".len()
                && span.end <= claim.end => {}
        other => panic!("false present value must refuse only at own Refl: {other:?}"),
    }
}

/// SOURCE: spec/50-stdlib/66-symbolic-algebra.md §20, D0 premise refusal.
/// PROMISE CLASS: durable invariant.
/// MEASURED: a separate fresh accepted prefix checks before the false
/// absent-premise application; its own Proved token sits within the rejected
/// theorem span, and the expected type contains the checked occurs owner.
/// CLAIMED: a present index cannot pass a false absence premise.
/// THE GAP: syntactic absence remains sufficient, not necessary.
#[test]
fn false_absence_premise_refuses_at_own_law_application() {
    let mut env = loaded_package();
    env.elaborate_file(ACCEPTED)
        .expect("fresh complete accepted prefix checks first");
    let parsed = parse_decls(FALSE_PREMISE).expect("false law claim parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = parsed.as_slice()
    else {
        panic!("one isolated false law theorem");
    };
    assert_eq!(name, "present_unchanged_by_law");
    let token = FALSE_PREMISE.rfind("Proved").expect("own false premise");
    let occurs = env.globals[&format!("{PACKAGE}.occurs")];
    match env
        .elaborate_file(FALSE_PREMISE)
        .expect_err("false absence premise must refuse")
    {
        ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, .. },
            span,
        } if claim.start <= span.start
            && span.start <= token
            && token + "Proved".len() <= span.end
            && span.end <= claim.end
            && term_contains_const(&expected, occurs) => {}
        other => panic!("false absence must reject at own occurs premise: {other:?}"),
    }
}
