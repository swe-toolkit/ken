//! Spec 51 §9, AC-2 seed, and the three AC-3 migrations.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::parser::parse_decls;
use ken_elaborator::{extract_ken_md, Decl as SourceDecl, ElabEnv, ElabError, KenMdFenceRole};
use ken_kernel::{postulates_reachable, Decl, GlobalId, KernelError, Term};

const LAWFUL: &str = "Core.Classes.LawfulClasses";
const LAW: &str = "bool_eq_from_iff";
const IMPORTERS: [&str; 3] = [
    "Data.Numeric.Rational",
    "Algorithm.Graphs.Dependency",
    "Algorithm.Numeric.RationalInterval",
];
const OPEN: &str = "bool_eq_open";
const REFUSED: &str = "bool_eq_forward_only";
const SEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/stdlib/classes/seed-bool-eq-from-iff.md"
));

fn roots() -> [PathBuf; 1] {
    [PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog/packages")]
}

fn trusted(env: &ElabEnv) -> BTreeSet<GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

fn load_core(env: &mut ElabEnv) -> GlobalId {
    let owned = env
        .elaborate_module_from_roots(&roots(), LAWFUL)
        .expect("LawfulClasses checks");
    let law = env.globals[&format!("{LAWFUL}.{LAW}")];
    assert!(owned.contains(&law), "LawfulClasses owns {LAW}");
    assert!(matches!(
        env.env.lookup(law),
        Some(Decl::Transparent { .. })
    ));
    law
}

fn cites(body: &Term, law: GlobalId) -> bool {
    matches!(body, Term::Const { id, .. } if *id == law)
        || body.children().into_iter().any(|child| cites(child, law))
}

fn checked_citation(env: &ElabEnv, declaration: GlobalId, law: GlobalId) -> bool {
    matches!(env.env.lookup(declaration), Some(Decl::Transparent { body, .. }) if cites(body, law))
}

/// PROMISE CLASS: durable migration invariant; the existing trusted Ord Int
/// identities are a review boundary, not a count pin.
/// MEASURED: the load adds only inherited, named trust; a selective import
/// succeeds and each importer has an owned checked citation and no local copy.
/// CLAIMED: one transparent Core proof is reused at zero new trust.
/// THE GAP: this pins reuse in each importer, not the number of call sites.
#[test]
fn core_owns_the_law_and_all_three_packages_use_it_at_zero_new_trust() {
    let mut env = ElabEnv::new().expect("compiler base");
    for provider in [
        "Core.Logic.Conjunction",
        "Core.Logic.Or",
        "Core.Logic.OrdResult",
        "Core.Logic.Transport",
        "Core.Logic.Compare",
        "Data.Text.StringBijection",
    ] {
        env.elaborate_module_from_roots(&roots(), provider)
            .unwrap_or_else(|error| panic!("{provider} checks: {error:?}"));
    }
    let mut expected = trusted(&env);
    let law = load_core(&mut env);
    // The four existing Ord Int axioms each wrap a kernel postulate.
    // Resolve the actual postulate identities, not the wrapper identities.
    for old_axiom in [
        "ord_int_refl",
        "ord_int_antisym",
        "ord_int_trans",
        "ord_int_total",
    ] {
        let wrapper = Term::const_(env.globals[&format!("{LAWFUL}.{old_axiom}")], vec![]);
        let mut postulates = postulates_reachable(&env.env, &wrapper).into_iter();
        let postulate = postulates.next().expect("existing Ord Int postulate");
        assert!(postulates.next().is_none(), "one postulate for {old_axiom}");
        assert!(
            expected.insert(postulate),
            "distinct postulate for {old_axiom}"
        );
    }
    let after_core = trusted(&env);
    eprintln!(
        "{LAWFUL} trusted GlobalIds (inherited + existing Ord Int → measured): {} → {}",
        expected.len(),
        after_core.len()
    );
    assert_eq!(expected, after_core, "LawfulClasses adds no new trust");

    for module in IMPORTERS {
        let owned = env
            .elaborate_module_from_roots(&roots(), module)
            .unwrap_or_else(|error| panic!("{module} checks: {error:?}"));
        let after = trusted(&env);
        eprintln!(
            "{module} trusted GlobalIds: {} → {}",
            after_core.len(),
            after.len()
        );
        assert_eq!(after_core, after, "{module} adds no trusted identities");
        assert!(
            !env.globals.contains_key(&format!("{module}.{LAW}")),
            "{module} must not declare a local {LAW}"
        );
        assert!(
            owned.iter().any(|id| checked_citation(&env, *id, law)),
            "{module} must own a checked body citing {LAWFUL}.{LAW}"
        );
    }
    assert!(env
        .elaborate_file(&format!("import {LAWFUL} ({LAW})"))
        .expect("selective import of the Core law resolves")
        .is_empty());
}

fn seed_case(theorem: &str) -> &'static str {
    let mut selected = None;
    for fence in extract_ken_md(SEED)
        .expect("§9 seed extracts")
        .fences
        .into_iter()
        .filter(|fence| fence.role == KenMdFenceRole::Source)
    {
        let source = &SEED[fence.body_range];
        if parse_decls(source)
            .expect("§9 seed source parses")
            .into_iter()
            .any(|decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == theorem))
        {
            assert!(selected.replace(source).is_none(), "duplicate {theorem}");
        }
    }
    selected.unwrap_or_else(|| panic!("missing §9 seed case {theorem}"))
}

fn accepted_prefix() -> ElabEnv {
    let mut env = ElabEnv::new().expect("compiler base");
    let law = load_core(&mut env);
    let before = trusted(&env);
    // No test-supplied imports in the client's namespace.
    let declared = env
        .elaborate_file(seed_case(OPEN))
        .expect("complete accepted §9 seed prefix checks");
    let client = env.globals[OPEN];
    assert!(declared.contains(&client), "seed declares {OPEN}");
    assert!(checked_citation(&env, client, law), "{OPEN} cites the law");
    assert_eq!(before, trusted(&env), "accepted seed adds no trust");
    env
}

fn open_claim(source: &str) -> SourceDecl {
    parse_decls(source)
        .expect("open statement parses")
        .into_iter()
        .find(|decl| matches!(decl, SourceDecl::TheoremDecl { name, .. } if name == OPEN))
        .expect("open §9 theorem is present")
}

/// PROMISE CLASS: durable invariant.
/// MEASURED: the exact open seed cites the law. Replacing only its closing
/// expression with Refl or Proved produces a TypeMismatch in that theorem.
/// CLAIMED: this is not a computation-only witness. THE GAP: these are two
/// specific bypass terms, not a claim that the law has no alternate proof.
#[test]
fn open_seed_uses_the_law_and_refuses_refl_and_proved() {
    accepted_prefix();
    let accepted = seed_case(OPEN);
    let SourceDecl::TheoremDecl { params, body, .. } = open_claim(accepted) else {
        unreachable!("theorem selector")
    };
    assert!(!params.is_empty(), "accepted witness is open");
    for closing_term in ["Refl", "Proved"] {
        let mut control = accepted.to_owned();
        control.replace_range(body.span().start..body.span().end, closing_term);
        let SourceDecl::TheoremDecl { span, body, .. } = open_claim(&control) else {
            unreachable!("theorem selector")
        };
        let mut env = ElabEnv::new().expect("compiler base");
        load_core(&mut env);
        match (closing_term, env.elaborate_file(&control)) {
            ("Refl", Err(ElabError::TypeMismatch { span: at, .. })) if &at == body.span() => {
                eprintln!("{OPEN} by Refl: TypeMismatch at {at:?}");
            }
            (
                "Proved",
                Err(ElabError::KernelRejected {
                    error: KernelError::TypeMismatch { .. },
                    span: at,
                }),
            ) if span.start <= at.start && at.start < at.end && at.end <= span.end => {
                eprintln!("{OPEN} by Proved: KernelRejected(TypeMismatch) at {at:?}");
            }
            (_, other) => panic!("{OPEN} by {closing_term} must refuse in its theorem: {other:?}"),
        }
    }
}

/// PROMISE CLASS: durable invariant.
/// MEASURED: the isolated seed twin kernel-refuses within its own theorem,
/// after only its own fresh import prefix. CLAIMED: the forward implication
/// alone does not complete this application. THE GAP: this does not pin
/// diagnostic offsets or wording, and is not official conformance execution.
#[test]
fn forward_only_refuses_after_its_fresh_import_prefix() {
    accepted_prefix();
    let mut env = ElabEnv::new().expect("compiler base");
    load_core(&mut env);
    let refused = seed_case(REFUSED);
    let decls = parse_decls(refused).expect("isolated refusal parses");
    let [SourceDecl::ImportDecl { .. }, SourceDecl::TheoremDecl {
        name, params, span, ..
    }] = decls.as_slice()
    else {
        panic!("expected an import and one isolated refused theorem");
    };
    assert_eq!(name, REFUSED);
    assert!(!params.is_empty(), "refused twin is open");
    match env.elaborate_file(refused) {
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            span: at,
        }) if span.start <= at.start && at.start < at.end && at.end <= span.end => {
            eprintln!("{REFUSED}: KernelRejected(TypeMismatch) at {at:?}");
        }
        other => panic!("{REFUSED} must kernel-refuse in its own theorem: {other:?}"),
    }
}
