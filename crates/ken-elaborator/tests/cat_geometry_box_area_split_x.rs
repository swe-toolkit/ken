//! Spec 67 §24: splitting a closed box along x preserves total area.
//! Promise class: durable invariant. This test checks the authored CV seed
//! against the catalog; it does not run the official conformance runner.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const PACKAGE: &str = "Algorithm.Geometry.NatBox";
const LAW: &str = "Algorithm.Geometry.NatBox.area_split_x";
const ACCEPTED: &str = "split_open_in_hx";
// The computation-only blocks also name ACCEPTED. Select the complete
// accepted block by a closed witness it alone defines.
const ACCEPTED_PREFIX: &str = "left_column_area";
const REFUSED: &str = "refused_split_without_low_premise";

const GENERIC: &str = r#"import Core.Classes.LawfulClasses (IsTrue as CatalogIsTrue, leq_nat)
import Data.Numeric.Nat.Arithmetic (add)
import Algorithm.Geometry.NatBox (MkPoint, MkBox, area, area_split_x)

theorem generic_area_split_x (lx : Nat) (ly : Nat) (m : Nat) (hx : Nat) (hy : Nat)
    : CatalogIsTrue (leq_nat lx (Suc m))
      → CatalogIsTrue (leq_nat m hx)
      → Equal Nat
          (add (area (MkBox (MkPoint lx ly) (MkPoint m hy)))
               (area (MkBox (MkPoint (Suc m) ly) (MkPoint hx hy))))
          (area (MkBox (MkPoint lx ly) (MkPoint hx hy))) =
  area_split_x lx ly m hx hy
"#;

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_package(env: &mut ElabEnv) -> Vec<GlobalId> {
    env.elaborate_module_from_roots(&[repo().join("catalog/packages")], PACKAGE)
        .expect("NatBox root-loads with its checked area split")
}

fn package_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_package(&mut env);
    env
}

fn trusted(env: &ElabEnv) -> BTreeSet<GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

fn cites(body: &Term, owner: GlobalId) -> bool {
    let mut pending = vec![body];
    while let Some(term) = pending.pop() {
        if matches!(term, Term::Const { id, .. } if *id == owner) {
            return true;
        }
        pending.extend(term.children());
    }
    false
}

fn assert_law_citation(env: &ElabEnv, theorem: &str) {
    let Some(Decl::Transparent { body, .. }) = env.env.lookup(env.globals[theorem]) else {
        panic!("{theorem} must have a checked transparent body");
    };
    assert!(cites(body, env.globals[LAW]), "{theorem} must cite {LAW}");
}

fn seed() -> String {
    let path = repo().join("conformance/stdlib/geometry/seed-area-split-x.md");
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn seed_case<'a>(seed: &'a str, theorem: &str) -> &'a str {
    let mut selected = None;
    for fence in extract_ken_md(seed)
        .expect("§24 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &seed[fence.body_range];
        for decl in parse_decls(source).expect("§24 seed source parses") {
            if matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem) {
                assert!(selected.replace(source).is_none(), "duplicate {theorem}");
            }
        }
    }
    selected.unwrap_or_else(|| panic!("missing seed theorem {theorem}"))
}

fn accepted_env(accepted: &str) -> ElabEnv {
    let mut env = package_env();
    let before = trusted(&env);
    env.elaborate_file(accepted)
        .expect("complete authored accepted seed checks");
    assert_eq!(trusted(&env), before, "accepted seed adds no trusted IDs");
    assert_law_citation(&env, ACCEPTED);
    env
}

/// MEASURED: provider-first trust identities are unchanged across NatBox
/// loading; the generic selective import applies the owned Transparent law;
/// the two internal names cannot be imported. CLAIMED: §24 is public at zero
/// added trust, not a library-wide export of subtraction chaining. THE GAP:
/// the trusted baseline still includes inherited provider assumptions.
#[test]
fn generic_area_split_is_owned_public_and_adds_no_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    let roots = [repo().join("catalog/packages")];
    for provider in [
        "Core.Logic.Conjunction",
        "Core.Classes.LawfulClasses",
        "Core.Logic.Transport",
        "Data.Numeric.Nat.Arithmetic",
        "Data.Numeric.Nat.Order",
    ] {
        env.elaborate_module_from_roots(&roots, provider)
            .unwrap_or_else(|error| panic!("provider {provider} root-loads: {error:?}"));
    }
    let before = trusted(&env);
    let owned = load_package(&mut env);
    let after = trusted(&env);
    eprintln!(
        "NatBox trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(before, after, "NatBox adds no trusted GlobalId identity");

    env.elaborate_file(GENERIC)
        .expect("generic selective-import split with both order premises checks");
    let law = env.globals[LAW];
    assert!(owned.contains(&law), "{LAW} is owned by NatBox");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    assert_law_citation(&env, "generic_area_split_x");

    for private in ["sub_chain_add", "axis_span"] {
        let name = format!("{PACKAGE}.{private}");
        let id = env.globals[&name];
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        let source = format!("import {PACKAGE} ({private})");
        match env.elaborate_file(&source) {
            Err(ElabError::UnboundName {
                name: missing,
                span,
            }) if missing == name && span.start < span.end && span.end <= source.len() => {}
            other => panic!("{name} must refuse selective import as UnboundName: {other:?}"),
        }
    }
}

/// MEASURED: the authored prefix checks the open citation and closed area
/// controls, then the isolated no-low-premise theorem fails to kernel-check.
/// CLAIMED: the open application is valid and the weakened claim is false at
/// hx = 3. THE GAP: the kernel's span localizes the refusal to a declaration,
/// not specifically to its proof argument.
#[test]
fn authored_area_split_seed_checks_then_refuses_without_low_premise() {
    let seed = seed();
    let mut env = accepted_env(seed_case(&seed, ACCEPTED_PREFIX));
    for value in [
        "left_column_area",
        "right_column_area",
        "whole_area",
        "empty_left_column",
        "dropped_low_premise_false_at_three",
        "dropped_low_whole_is_zero",
        "dropped_low_premise_is_false",
    ] {
        assert!(
            matches!(
                env.env.lookup(env.globals[value]),
                Some(Decl::Transparent { .. })
            ),
            "{value} must be checked in the authored prefix"
        );
    }
    let refused = seed_case(&seed, REFUSED);
    let decls = parse_decls(refused).expect("refused seed parses");
    let claim = decls
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl { name, span, .. } if name == REFUSED => Some(span),
            _ => None,
        })
        .expect("refused theorem span");
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

/// MEASURED: replacing only the authored open row's proof body by either
/// computation-only closing fails at the row itself. CLAIMED: with hx open,
/// a checked law application supplies evidence computation does not.
/// THE GAP: this does not rule out other non-computational proofs.
#[test]
fn authored_open_area_split_does_not_close_by_computation() {
    let seed = seed();
    let accepted = seed_case(&seed, ACCEPTED_PREFIX);
    accepted_env(accepted);
    let decls = parse_decls(accepted).expect("accepted seed parses");
    let (claim, body) = decls
        .iter()
        .find_map(|decl| match decl {
            SourceDecl::TheoremDecl {
                name, span, body, ..
            } if name == ACCEPTED => Some((span, body.span())),
            _ => None,
        })
        .expect("open theorem declaration and body spans");
    for (closing, kernel) in [("Proved", true), ("Refl", false)] {
        let prefix = &accepted[..body.start];
        let binders = "λhl. λhh. ";
        let source = format!(
            "{prefix}{binders}{closing}{}",
            &accepted[body.end..claim.end]
        );
        let token = prefix.len() + binders.len();
        match package_env().elaborate_file(&source) {
            Err(ElabError::KernelRejected {
                error: KernelError::TypeMismatch { .. },
                span,
            }) if kernel
                && claim.start <= span.start
                && span.start < span.end
                && span.end <= source.len() =>
            {
                eprintln!("{ACCEPTED} by {closing}: KernelRejected(TypeMismatch) at {span:?}");
            }
            Err(ElabError::TypeMismatch { span, .. })
                if !kernel
                    && claim.start <= span.start
                    && span.start <= token
                    && token + closing.len() <= span.end
                    && span.end <= source.len() =>
            {
                eprintln!("{ACCEPTED} by {closing}: TypeMismatch at {span:?}");
            }
            other => panic!("{ACCEPTED} must refuse {closing} in its own theorem: {other:?}"),
        }
    }
}
