//! Downward persistence of `None`: spec/50-stdlib/72-optimization.md §2.9.
//! Promise class: durable invariant. The authored CV seed is a package-test
//! input, not an official runner result.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const PACKAGE: &str = "Algorithm.Optimization.BoundedSearch";
const LAW: &str = "Algorithm.Optimization.BoundedSearch.find_below_none_shrink";
const ACCEPTED: &str = "none_shrink_five_to_three";
const REFUSED: &str = "none_grow_three_to_five";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/optimization/seed-bounded-search-none-shrink.md"
));

const GENERIC: &str = r#"
import Algorithm.Optimization.BoundedSearch (find_below, find_below_none_shrink)
import Core.Classes.LawfulClasses (leq_nat)

theorem client_none_shrink (p : Nat → Bool) (bound : Nat) (larger : Nat) :
  Equal Bool (leq_nat bound larger) True →
  Equal (Option Nat) (find_below p larger) (None Nat) →
  Equal (Option Nat) (find_below p bound) (None Nat) =
  find_below_none_shrink p bound larger
"#;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")
}

fn trusted(env: &ElabEnv) -> BTreeSet<GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

fn load_package(env: &mut ElabEnv) -> Vec<GlobalId> {
    env.elaborate_module_from_roots(&[root()], PACKAGE)
        .expect("BoundedSearch roots-loads and checks")
}

fn package_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_package(&mut env);
    env
}

fn cites(term: &Term, owner: GlobalId) -> bool {
    matches!(term, Term::Const { id, .. } if *id == owner)
        || term.children().into_iter().any(|child| cites(child, owner))
}

fn assert_checked_citation(env: &ElabEnv, theorem: &str, law: GlobalId) {
    let id = env.globals[theorem];
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(id) else {
        panic!("{theorem} has no checked body");
    };
    assert!(cites(body, law), "{theorem} must cite the owned law");
}

fn seed_case(theorem: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§2.9 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        for decl in parse_decls(source).expect("seed source parses") {
            if matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem) {
                assert!(
                    selected.replace(source).is_none(),
                    "{theorem} must identify one seed theorem"
                );
            }
        }
    }
    selected.unwrap_or_else(|| panic!("missing seed theorem {theorem}"))
}

fn accepted_seed() -> ElabEnv {
    let mut env = package_env();
    let law = env.globals[LAW];
    let before = trusted(&env);
    env.elaborate_file(seed_case(ACCEPTED))
        .expect("complete authored accepted seed checks");
    assert_eq!(
        trusted(&env),
        before,
        "accepted seed adds no trusted GlobalId"
    );
    assert_checked_citation(&env, ACCEPTED, law);
    env
}

/// MEASURED: exact loaded-provider trust identities are unchanged across
/// BoundedSearch loading, and an arbitrary-argument checked client cites
/// the package-owned Transparent law. CLAIMED: §2.9 is public at zero new
/// trust. THE GAP: pre-existing provider assumptions remain in the baseline.
#[test]
fn generic_none_shrink_is_checked_owned_and_adds_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Conjunction",
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
        "Data.Numeric.Nat.Order",
        "Data.Numeric.Nat.Arithmetic",
    ] {
        env.elaborate_module_from_roots(&[root()], provider)
            .unwrap_or_else(|error| panic!("{provider} roots-loads: {error:?}"));
    }
    let before = trusted(&env);
    let owned = load_package(&mut env);
    let after = trusted(&env);
    eprintln!("§2.9 trusted GlobalIds: {} → {}", before.len(), after.len());
    assert_eq!(after, before, "BoundedSearch adds no trusted GlobalId");

    env.elaborate_file(GENERIC)
        .expect("generic client checks with order and larger None premises only");
    let law = env.globals[LAW];
    assert!(owned.contains(&law), "law belongs to BoundedSearch");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    assert_checked_citation(&env, "client_none_shrink", law);
}

/// MEASURED: the authored open client checks and cites the owned law before
/// the converse in the same proof form fails within its own declaration.
/// CLAIMED: the shrink application is valid, not its reversed-bound twin.
/// THE GAP: the §2.4 closed counterexample, not this refusal, witnesses a
/// predicate for which the converse's conclusion is false.
#[test]
fn authored_seed_accepts_shrink_and_refuses_converse() {
    let mut env = accepted_seed();
    let refused = seed_case(REFUSED);
    let decls = parse_decls(refused).expect("refused seed parses");
    let claim = decls
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl { name, span, .. } if name == REFUSED => Some(span),
            _ => None,
        })
        .expect("refused theorem span");
    // Submit only the refused declaration after the actual accepted prefix.
    let source = &refused[claim.start..claim.end];
    match env.elaborate_file(source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start < span.end && span.end <= source.len() => {
            eprintln!("{REFUSED}: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("{REFUSED} must kernel-refuse in its own theorem: {other:?}"),
    }
}

/// MEASURED: `λmissing. Proved`, `λmissing. Refl` and `λmissing. missing`
/// cannot close the actual accepted seed type. CLAIMED: the open client is
/// not a computation-only observation or a restatement of the premise.
/// THE GAP: other non-computational proofs of this type may be possible.
#[test]
fn open_seed_does_not_close_by_computation_or_premise() {
    accepted_seed();
    let accepted = seed_case(ACCEPTED);
    let decls = parse_decls(accepted).expect("accepted seed parses");
    let (claim, body) = decls
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name, span, body, ..
            } if name == ACCEPTED => Some((span, body.span())),
            _ => None,
        })
        .expect("accepted theorem spans");
    for (replacement, kernel) in [
        ("λmissing. Proved", true),
        ("λmissing. Refl", false),
        ("λmissing. missing", true),
    ] {
        let mut env = package_env();
        let source = format!(
            "{}{replacement}{}",
            &accepted[..body.start],
            &accepted[body.end..claim.end],
        );
        match env.elaborate_file(&source) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if kernel
                && claim.start <= span.start
                && span.start < span.end
                && span.end <= source.len() =>
            {
                eprintln!("{ACCEPTED} by {replacement}: KernelRejected(TypeMismatch) at {span:?}");
            }
            Err(ElabError::TypeMismatch { span, .. })
                if !kernel
                    && claim.start <= span.start
                    && span.start < span.end
                    && span.end <= source.len() =>
            {
                eprintln!("{ACCEPTED} by {replacement}: TypeMismatch at {span:?}");
            }
            other => panic!("{ACCEPTED} must refuse {replacement} in its own theorem: {other:?}"),
        }
    }
}
