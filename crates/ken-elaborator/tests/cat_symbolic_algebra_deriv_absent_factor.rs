//! A factor absent from the derivative variable: Symbolic Algebra §25.
//! Promise class: durable invariant. The authored CV seed is a package-test
//! input, not an official runner result.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const PACKAGE: &str = "Algorithm.SymbolicAlgebra.NatPolynomial";
const LAW: &str = "deriv_absent_factor";
const OPEN: &str = "factor_absent_open";
const CLOSED: &str = "factor_present_decides_false";
const REFUSED: &str = "factor_present_refused";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/symbolic-algebra/seed-derivative-absent-factor.md"
));

const GENERIC_CLIENT: &str = r#"
import Core.Classes.LawfulClasses (IsTrue)
import Algorithm.SymbolicAlgebra.NatPolynomial
  (Expr, EMul, deriv, occurs, poly_eq, deriv_absent_factor)

theorem client_absent_factor
      (v : Nat) (c : Expr) (e : Expr) (absent : Equal Bool (occurs v c) False)
    : IsTrue (poly_eq (deriv v (EMul c e)) (EMul c (deriv v e))) =
  deriv_absent_factor v c e absent
"#;

fn roots() -> [PathBuf; 1] {
    [PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

fn trusted(env: &ElabEnv) -> BTreeSet<GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

fn load_package(env: &mut ElabEnv) -> GlobalId {
    let owned = env
        .elaborate_module_from_roots(&roots(), PACKAGE)
        .expect("NatPolynomial roots-loads");
    let law = env.globals[&format!("{PACKAGE}.{LAW}")];
    assert!(owned.contains(&law), "NatPolynomial owns the §25 law");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    law
}

fn cites(term: &Term, owner: GlobalId) -> bool {
    let mut pending = vec![term];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

fn checked_citation(env: &ElabEnv, name: &str, law: GlobalId) -> bool {
    let (_, body) = env
        .env
        .transparent_body(env.globals[name])
        .unwrap_or_else(|| panic!("{name} has a checked Transparent body"));
    cites(&body, law)
}

/// MEASURED: roots-loading preserves the exact provider-first trust set;
/// a generic selective-import client and the existing literal theorem both
/// have checked bodies that cite the owned law. CLAIMED: the new law is
/// reusable with only syntactic absence, and §12 reuses rather than repeats
/// its proof. THE GAP: equality of sets does not audit inherited trust.
#[test]
fn generic_absent_factor_is_owned_public_and_zero_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Collections.Derived",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&roots(), provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = trusted(&env);
    let law = load_package(&mut env);
    let after = trusted(&env);
    eprintln!(
        "§25 NatPolynomial trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "NatPolynomial adds no trusted GlobalId");

    assert!(
        checked_citation(&env, &format!("{PACKAGE}.deriv_const_factor"), law),
        "the literal factor theorem cites the owned §25 law"
    );
    env.elaborate_file(GENERIC_CLIENT)
        .expect("generic client applies the selective import under only absence");
    assert!(checked_citation(&env, "client_absent_factor", law));
    assert_eq!(after, trusted(&env), "generic client adds no trust");
}

fn seed_case(name: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§25 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        if parse_decls(source)
            .expect("§25 seed client parses")
            .iter()
            .any(|decl| matches!(decl, SourceDecl::TheoremDecl { name: actual, .. } if actual == name))
        {
            assert!(selected.replace(source).is_none(), "unique fence for {name}");
        }
    }
    selected.unwrap_or_else(|| panic!("§25 seed is missing {name}"))
}

fn accepted_prefix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    let law = load_package(&mut env);
    let before = trusted(&env);
    env.elaborate_file(seed_case(OPEN))
        .expect("complete actual §25 seed accepted prefix checks");
    assert!(
        checked_citation(&env, OPEN, law),
        "open seed cites the owned law"
    );
    assert!(
        !checked_citation(&env, CLOSED, law),
        "closed seed theorem checks without the factoring law"
    );
    assert_eq!(before, trusted(&env), "accepted §25 seed adds no trust");
    env
}

/// MEASURED: the complete actual accepted fence checks, then the isolated
/// twin fails within its own theorem at a type involving the owned `occurs`.
/// CLAIMED: the present-factor application does not prove syntactic absence.
/// THE GAP: the refusal alone does not establish falsity of the factored
/// equation; the accepted closed row demonstrates a counterexample.
#[test]
fn actual_seed_accepts_then_refuses_at_the_occurrence_premise() {
    let mut env = accepted_prefix();
    let occurs = env.globals[&format!("{PACKAGE}.occurs")];
    let source = seed_case(REFUSED);
    let decls = parse_decls(source).expect("§25 isolated twin parses");
    let [SourceDecl::TheoremDecl {
        name, span: claim, ..
    }] = decls.as_slice()
    else {
        panic!("the §25 twin must isolate one theorem after its accepted prefix");
    };
    assert_eq!(name, REFUSED);
    match env.elaborate_file(source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { expected, .. },
            span,
        }) if claim.start <= span.start
            && span.start < span.end
            && span.end <= claim.end
            && cites(&expected, occurs) =>
        {
            eprintln!("{REFUSED}: occurrence premise KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("{REFUSED} must kernel-refuse at its own occurrence premise: {other:?}"),
    }
}

/// MEASURED: the actual open seed theorem does not close with `Proved` or
/// evaluation reflexivity. CLAIMED: it needs a proof beyond computation.
/// THE GAP: equivalent non-computational proofs could also close the goal.
#[test]
fn open_seed_does_not_close_by_computation() {
    use ken_elaborator::Type::{TApp, TCon, TVar};

    accepted_prefix();
    let accepted = seed_case(OPEN);
    let decls = parse_decls(accepted).expect("accepted source parses");
    let (claim, goal, body) = decls
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name,
                span,
                theorem,
                body,
                ..
            } if name == OPEN => Some((span, theorem, body.span())),
            _ => None,
        })
        .expect("actual open theorem");
    let TApp(is_true, decision, _) = goal else {
        panic!("open seed proves an IsTrue decision");
    };
    assert!(matches!(is_true.as_ref(), TVar(name, _) | TCon(name, _) if name == "IsTrue"));
    let TApp(decides_left, right, _) = decision.as_ref() else {
        panic!("open seed decides canonical equality");
    };
    let TApp(poly_eq, left, _) = decides_left.as_ref() else {
        panic!("open seed supplies both poly_eq operands");
    };
    assert!(matches!(poly_eq.as_ref(), TVar(name, _) | TCon(name, _) if name == "poly_eq"));
    let operand = |ty: &ken_elaborator::Type| &accepted[ty.span().start..ty.span().end];
    let evaluation_refl = format!(
        "poly_eq_complete ({}) ({}) (λenv. Refl)",
        operand(left),
        operand(right)
    );

    for (extra_import, replacement, is_refl) in [
        ("", "Proved", false),
        (
            "import Algorithm.SymbolicAlgebra.NatPolynomial (poly_eq_complete)\n",
            evaluation_refl.as_str(),
            true,
        ),
    ] {
        let mut env = ElabEnv::new().expect("compiler base");
        load_package(&mut env);
        // The seed is unchanged. Only this test-supplied control imports the
        // completeness lemma and replaces the parsed open theorem body.
        let source = format!(
            "{extra_import}{}{replacement}{}",
            &accepted[..body.start],
            &accepted[body.end..claim.end],
        );
        let start = extra_import.len() + claim.start;
        let end = source.len();
        let refl_start = replacement
            .strip_suffix("Refl)")
            .map(|prefix| extra_import.len() + body.start + prefix.len());
        match env.elaborate_file(&source) {
            Err(ElabError::TypeMismatch { span, .. })
                if is_refl
                    && start <= span.start
                    && span.end <= end
                    && refl_start
                        .is_some_and(|refl| span.start <= refl && refl + 4 <= span.end) =>
            {
                eprintln!("{OPEN} by evaluation Refl: TypeMismatch at {span:?}");
            }
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if !is_refl && start <= span.start && span.start < span.end && span.end <= end => {
                eprintln!("{OPEN} by Proved: KernelRejected(TypeMismatch) at {span:?}");
            }
            other => panic!("{OPEN} must refuse {replacement} in its own theorem: {other:?}"),
        }
    }
}
