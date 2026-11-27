//! Spec 65 §25: the kernel of transpose(a) × a is contained in the kernel
//! of a. Promise class: durable invariant. The CV seed is input to these
//! package tests, not an official conformance-runner execution.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const MATRIX: &str = "Algorithm.LinearAlgebra.NatMatrix";
const LAW: &str = "Algorithm.LinearAlgebra.NatMatrix.matrix_vector_zero_of_gram_vector_zero";
const OPEN_ROW: &str = "gram_kernel_open_in_v";
const CLOSED_ROW: &str = "second_unit_in_gram_kernel";
const TWIN: &str = "refused_gram_kernel_is_zero";

const GENERIC: &str = r#"import Data.Vector.Vector (Vec)
import Algorithm.LinearAlgebra.NatMatrix
  (matrix_vector, matrix_product, transpose, vector_zero, matrix_vector_zero_of_gram_vector_zero)

theorem generic_gram_kernel
      (m : Nat) (n : Nat) (a : Vec (Vec Nat n) m) (v : Vec Nat n)
    : Equal
        (Vec Nat n)
        (matrix_vector n n (matrix_product n m n (transpose m n a) a) v)
        (vector_zero n)
      → Equal (Vec Nat m) (matrix_vector m n a v) (vector_zero m) =
  matrix_vector_zero_of_gram_vector_zero m n a v
"#;

// Use the checked controls at the claimed types with a literal [0, 1].
// This prevents weaker closed claims (for example on the zero vector) from
// serving as evidence for the false twin.
const COUNTEREXAMPLE: &str = r#"const witness : Vec Nat two = VCons Nat one Zero (VCons Nat Zero (Suc Zero) (VNil Nat))

theorem witness_in_gram_kernel
    : Equal
        (Vec Nat two)
        (matrix_vector two two (matrix_product two one two (transpose one two pick_first) pick_first) witness)
        (vector_zero two) =
  second_unit_in_gram_kernel

theorem witness_self_dot : Equal Nat (dot two witness witness) (Suc Zero) =
  second_unit_self_dot

theorem witness_equality_implies_one_equals_zero
    : Equal (Vec Nat two) witness (vector_zero two) → Equal Nat (Suc Zero) Zero =
  second_unit_not_zero
"#;

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn trusted(env: &ElabEnv) -> BTreeSet<GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

fn load_matrix(env: &mut ElabEnv) -> Vec<GlobalId> {
    env.elaborate_module_from_roots(&[repo().join("catalog/packages")], MATRIX)
        .expect("NatMatrix root-loads with its checked Gram-kernel law")
}

fn matrix_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_matrix(&mut env);
    env
}

fn cites_law(env: &ElabEnv, theorem: &str) -> bool {
    let law = env.globals[LAW];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(env.globals[theorem]) else {
        panic!("{theorem} must have a checked transparent body");
    };
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == law) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

fn read_seed() -> String {
    let path = repo().join("conformance/stdlib/linear-algebra/seed-gram-kernel.md");
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

// The computation-only fences can also name OPEN_ROW. Select the full
// accepted fence by the closed Gram-kernel witness it defines.
fn seed_case<'a>(seed: &'a str, theorem: &str) -> &'a str {
    let mut selected = None;
    for fence in extract_ken_md(seed)
        .expect("§25 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &seed[fence.body_range];
        for decl in parse_decls(source).expect("§25 seed source parses") {
            if matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem) {
                assert!(selected.replace(source).is_none(), "duplicate {theorem}");
            }
        }
    }
    selected.unwrap_or_else(|| panic!("missing seed theorem {theorem}"))
}

/// MEASURED: exact trusted identities after the three providers stay equal
/// across NatMatrix loading, and a generic selective import cites its owned
/// Transparent declaration. CLAIMED: the law is public without new trust.
/// THE GAP: existing assumptions of the providers are inherited, not erased.
#[test]
fn generic_gram_kernel_is_owned_public_and_adds_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    let roots = [repo().join("catalog/packages")];
    for provider in [
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Vector.Vector",
    ] {
        env.elaborate_module_from_roots(&roots, provider)
            .unwrap_or_else(|error| panic!("provider {provider} root-loads: {error:?}"));
    }
    let before = trusted(&env);
    let owned = load_matrix(&mut env);
    let after = trusted(&env);
    eprintln!(
        "NatMatrix trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "NatMatrix adds no trusted GlobalId identity");
    env.elaborate_file(GENERIC)
        .expect("generic open selective-import client checks");
    let law = env.globals[LAW];
    assert!(owned.contains(&law), "{LAW} is owned by NatMatrix");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    assert!(cites_law(&env, "generic_gram_kernel"));
}

/// MEASURED: the complete accepted block checks with an open citation, and
/// the closed witness inhabits the premise but cannot equal zero unless one
/// equals zero. The stronger twin fails to kernel-check after the accepted
/// prefix. CLAIMED: the implication does not force the input vector to zero.
/// THE GAP: the diagnostic identifies the twin declaration, not a subterm.
#[test]
fn authored_seed_checks_and_the_stronger_twin_refuses() {
    let seed = read_seed();
    let accepted = seed_case(&seed, CLOSED_ROW);
    let mut env = matrix_env();
    let before = trusted(&env);
    env.elaborate_file(accepted)
        .expect("complete authored accepted block checks");
    assert_eq!(trusted(&env), before, "the seed adds no trusted identity");
    assert!(cites_law(&env, OPEN_ROW), "{OPEN_ROW} must cite {LAW}");
    for control in [
        "pick_first_gram_value",
        CLOSED_ROW,
        "second_unit_self_dot",
        "second_unit_not_zero",
    ] {
        assert!(!cites_law(&env, control), "{control} must not cite {LAW}");
    }
    env.elaborate_file(COUNTEREXAMPLE)
        .expect("closed controls identify [0, 1] and force one = zero under the twin's conclusion");

    let refused = seed_case(&seed, TWIN);
    let decls = parse_decls(refused).expect("refused seed parses");
    let claim = decls
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl { name, span, .. } if name == TWIN => Some(span),
            _ => None,
        })
        .expect("refused twin span");
    // Do not let the additional witness clients participate in this trial.
    let mut fresh = matrix_env();
    fresh
        .elaborate_file(accepted)
        .expect("fresh accepted prefix");
    let source = &refused[claim.start..claim.end];
    match fresh.elaborate_file(source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start < span.end && span.end <= source.len() => {
            eprintln!("{TWIN}: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("{TWIN} must kernel-refuse in its own theorem: {other:?}"),
    }
}

/// MEASURED: each compute-only closing fails at the authored open row in a
/// fresh environment. CLAIMED: with v open, a checked application supplies
/// evidence computation does not. THE GAP: this is not a claim of uniqueness
/// among non-computational proofs.
#[test]
fn authored_open_gram_kernel_does_not_close_by_computation() {
    let seed = read_seed();
    let accepted = seed_case(&seed, CLOSED_ROW);
    matrix_env()
        .elaborate_file(accepted)
        .expect("unmodified open row and its prefix check");
    let decls = parse_decls(accepted).expect("accepted seed parses");
    let (claim, body) = decls
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name, span, body, ..
            } if name == OPEN_ROW => Some((span, body.span())),
            _ => None,
        })
        .expect("open theorem declaration and body spans");
    for (closing, kernel) in [("Proved", true), ("Refl", false)] {
        let prefix = &accepted[..body.start];
        let binder = "λgram_zero. ";
        let source = format!(
            "{prefix}{binder}{closing}{}",
            &accepted[body.end..claim.end]
        );
        let token = prefix.len() + binder.len();
        match matrix_env().elaborate_file(&source) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if kernel
                && claim.start <= span.start
                && span.start < span.end
                && span.end <= source.len() =>
            {
                eprintln!("{OPEN_ROW} by {closing}: KernelRejected(TypeMismatch) at {span:?}");
            }
            Err(ElabError::TypeMismatch { span, .. })
                if !kernel
                    && claim.start <= span.start
                    && span.start <= token
                    && token + closing.len() <= span.end
                    && span.end <= source.len() =>
            {
                eprintln!("{OPEN_ROW} by {closing}: TypeMismatch at {span:?}");
            }
            other => panic!("{OPEN_ROW} must refuse {closing} in its own theorem: {other:?}"),
        }
    }
}
