//! Spec 62 §24: pointwise-equal edges give equal reachability.
//! Promise class: durable invariant. The CV seed is exercised as package-test
//! input; this is not official conformance-runner execution.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{
    extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole, Span,
};
use ken_kernel::{Decl, GlobalId, KernelError, Term};

const GRAPH: &str = "Algorithm.Graphs.Dependency";
const LAW: &str = "Algorithm.Graphs.Dependency.graph_reachable_edge_equiv";
const OPEN_ROW: &str = "meet_reach_agree";
const TWIN: &str = "refused_inclusion_reach_equal";

const GENERIC: &str = r#"import Data.Finite.Finite (Finite)
import Algorithm.Graphs.Dependency (Graph, edge, graph_reachable, graph_reachable_edge_equiv)

theorem generic_edge_equiv
      (q : Type) (fq : Finite q) (g : Graph q) (h : Graph q) (s : q) (target : q → Bool)
      (same : (u : q) → (v : q) → Equal Bool (edge q g u v) (edge q h u v))
    : Equal Bool (graph_reachable q fq g s target) (graph_reachable q fq h s target) =
  graph_reachable_edge_equiv q fq g h s target same
"#;

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn trusted(env: &ElabEnv) -> BTreeSet<GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

fn load_graph(env: &mut ElabEnv) -> Vec<GlobalId> {
    env.elaborate_module_from_roots(&[repo().join("catalog/packages")], GRAPH)
        .expect("Dependency root-loads with the checked edge-equivalence law")
}

fn graph_env() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    load_graph(&mut env);
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
    let path = repo().join("conformance/stdlib/graphs/seed-reachability-edge-equiv.md");
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

struct Row<'a> {
    fence: &'a str,
    declaration: Span,
    body: Span,
}

impl Row<'_> {
    fn prefix(&self) -> &str {
        &self.fence[..self.declaration.start]
    }

    fn source(&self) -> &str {
        &self.fence[self.declaration.start..self.declaration.end]
    }

    // Keep the entire proposition from the real seed, not a reconstructed
    // version that could accidentally test a different goal.
    fn by(&self, closing: &str) -> (String, usize) {
        let before = &self.fence[self.declaration.start..self.body.start];
        let after = &self.fence[self.body.end..self.declaration.end];
        (format!("{before}{closing}{after}"), before.len())
    }
}

// Locate a declaration by identity, regardless of fence order. Two matches
// are ambiguous even if they happen to be in the same fence.
fn seed_row<'a>(seed: &'a str, theorem: &str) -> Row<'a> {
    let mut selected = None;
    for fence in extract_ken_md(seed)
        .expect("§24 seed Ken fences extract")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &seed[fence.body_range];
        for decl in parse_decls(source).expect("§24 seed source parses") {
            if let SourceDecl::TheoremDecl {
                name, span, body, ..
            } = decl
            {
                if name == theorem {
                    assert!(selected.is_none(), "duplicate seed theorem {theorem}");
                    selected = Some(Row {
                        fence: source,
                        declaration: span,
                        body: body.span(),
                    });
                }
            }
        }
    }
    selected.unwrap_or_else(|| panic!("missing seed theorem {theorem}"))
}

fn fresh_prefix(accepted: &Row<'_>) -> ElabEnv {
    let mut env = graph_env();
    env.elaborate_file(accepted.prefix())
        .expect("fresh seed prefix checks without the open row");
    env
}

fn assert_kernel_refuses(env: &mut ElabEnv, source: &str, row: &str) {
    match env.elaborate_file(source) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span,
        }) if span.start < span.end && span.end <= source.len() => {
            eprintln!("{row}: KernelRejected(TypeMismatch) at {span:?}");
        }
        other => panic!("{row} must kernel-refuse within its declaration: {other:?}"),
    }
}

/// MEASURED: exact trust-set equality, checked ownership/transparency and a
/// generic selective-import citation. CLAIMED: Dependency exports the law
/// with no new trust. THE GAP: provider trust is inherited, not erased.
#[test]
fn generic_edge_equiv_is_owned_public_and_adds_no_trust() {
    let roots = [repo().join("catalog/packages")];
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Conjunction",
        "Data.Finite.Finite",
        "Core.Logic.Transport",
        "Core.Classes.LawfulClasses",
        "Data.Collections.Derived",
        "Core.Logic.Or",
        "Data.Collections.List",
        "Data.Numeric.Nat.Order",
        "Data.Sums.Combinators",
        "Algorithm.FormalLanguages.Dfa",
        "Algorithm.FormalLanguages.Reachability",
    ] {
        env.elaborate_module_from_roots(&roots, provider)
            .unwrap_or_else(|error| panic!("provider {provider} root-loads: {error:?}"));
    }
    let before = trusted(&env);
    let owned = load_graph(&mut env);
    let after = trusted(&env);
    eprintln!(
        "Dependency trusted GlobalIds: {} → {}",
        before.len(),
        after.len()
    );
    assert_eq!(
        before, after,
        "Dependency adds no trusted GlobalId identity"
    );
    env.elaborate_file(GENERIC)
        .expect("generic open selective-import client checks");
    let law = env.globals[LAW];
    assert!(owned.contains(&law), "Dependency must own {LAW}");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    assert!(cites_law(&env, "generic_edge_equiv"));
}

/// MEASURED: the open seed theorem cites the checked law, its five controls
/// do not, and the seed adds no trust. CLAIMED: the authored client checks
/// with independent edge and reachability evidence. THE GAP: these are
/// fixture-specific checks, not a proof for arbitrary graphs.
#[test]
fn authored_open_row_checks_and_controls_do_not_cite_the_law() {
    let seed = read_seed();
    let accepted = seed_row(&seed, OPEN_ROW);
    let mut env = graph_env();
    let before = trusted(&env);
    env.elaborate_file(accepted.fence)
        .expect("complete authored accepted block checks");
    assert_eq!(trusted(&env), before, "the seed adds no trusted identity");
    assert!(cites_law(&env, OPEN_ROW), "{OPEN_ROW} must cite {LAW}");
    for control in [
        "meet_edges_agree",
        "single_step_reaches_true",
        "edgeless_misses_true",
        "edgeless_included_in_single_step",
        "equal_reach_refuted",
    ] {
        assert!(!cites_law(&env, control), "{control} must not cite {LAW}");
    }
}

/// MEASURED: replacing only the open row's closing proof with either terminal
/// refuses locally. CLAIMED: the authored open goal does not close by those
/// computation-only terminals. THE GAP: this does not rule out other proofs,
/// and the closed conjunction example can compute by Refl.
#[test]
fn authored_open_row_does_not_close_by_computation() {
    let seed = read_seed();
    let accepted = seed_row(&seed, OPEN_ROW);
    let (proved, _) = accepted.by("Proved");
    assert_kernel_refuses(
        &mut fresh_prefix(&accepted),
        &proved,
        "meet_reach_agree by Proved",
    );

    let (refl, token) = accepted.by("Refl");
    match fresh_prefix(&accepted).elaborate_file(&refl) {
        Err(ElabError::TypeMismatch { span, .. })
            if span.start <= token
                && token + "Refl".len() <= span.end
                && span.end <= refl.len() =>
        {
            eprintln!("{OPEN_ROW} by Refl: TypeMismatch at {span:?}");
        }
        other => panic!("{OPEN_ROW} must refuse at its Refl token: {other:?}"),
    }
}

/// MEASURED: the authored one-way inclusion twin and its Proved closing
/// refuse in fresh environments. CLAIMED: one-way inclusion does not imply
/// equal reachability. THE GAP: diagnostics locate the declaration, not the
/// failing subterm; the closed controls establish the counterexample.
#[test]
fn authored_inclusion_twin_and_its_proved_claim_refuse() {
    let seed = read_seed();
    let accepted = seed_row(&seed, OPEN_ROW);
    let twin = seed_row(&seed, TWIN);
    assert_kernel_refuses(&mut fresh_prefix(&accepted), twin.source(), TWIN);

    let (proved, _) = twin.by("Proved");
    assert_kernel_refuses(
        &mut fresh_prefix(&accepted),
        &proved,
        "refused_inclusion_reach_equal by Proved",
    );
}
