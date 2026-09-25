//! Behavioral pins for `LANG-MOD-CANONICAL-PAIR-PACKAGE`.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::modules::{PRELUDE_COMPANION_BINDING_NAMES, PRELUDE_FLOOR_NAMES};
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{convert, Context, Decl, GlobalId, Term};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

const TYPE_FLOOR: [&str; 15] = [
    "Auth",
    "Bool",
    "Bottom",
    "Char",
    "Equal",
    "List",
    "Nat",
    "Option",
    "Pair",
    "Prop",
    "Proved",
    "ResourceKind",
    "Result",
    "Top",
    "Utf8Error",
];

const COMPANIONS: [&str; 3] = ["mk_pair", "pair_fst", "pair_snd"];
const PAIR_BINDINGS: [&str; 4] = ["Pair", "mk_pair", "pair_fst", "pair_snd"];

struct FixtureRoot(PathBuf);

impl FixtureRoot {
    fn new(label: &str) -> Self {
        let serial = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ken-pair-floor-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create Pair-floor fixture root");
        Self(path)
    }

    fn write(&self, module: &str, source: &str) {
        fs::write(self.0.join(format!("{module}.ken")), source).expect("write Pair-floor fixture");
    }
}

impl Drop for FixtureRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn mentions_global(term: &Term, target: GlobalId) -> bool {
    match term {
        Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. }
            if *id == target =>
        {
            true
        }
        Term::Elim { fam, .. } if *fam == target => true,
        _ => term
            .children()
            .into_iter()
            .any(|child| mentions_global(child, target)),
    }
}

fn primitive_signature_type_ids(env: &ElabEnv) -> BTreeSet<GlobalId> {
    fn collect(term: &Term, found: &mut BTreeSet<GlobalId>) {
        match term {
            Term::Const { id, .. } | Term::IndFormer { id, .. } | Term::Constructor { id, .. } => {
                found.insert(*id);
            }
            Term::Elim { fam, .. } => {
                found.insert(*fam);
            }
            _ => {}
        }
        for child in term.children() {
            collect(child, found);
        }
    }

    let mut referenced = BTreeSet::new();
    for declaration in env.env.declarations() {
        if let Decl::Primitive { ty, .. } = declaration {
            collect(ty, &mut referenced);
        }
    }
    referenced
        .into_iter()
        .filter_map(|id| match env.env.lookup(id) {
            Some(Decl::Inductive(_)) | Some(Decl::Transparent { .. }) => Some(id),
            _ => env.env.constructor(id).map(|(parent, _)| parent.id),
        })
        .collect()
}

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn strict_one(source: &str) -> Result<ElabEnv, ElabError> {
    let root = FixtureRoot::new("one");
    root.write("Entry", source);
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")?;
    Ok(env)
}

fn strict_one_with_proof_terminal(source: &str) -> Result<ElabEnv, ElabError> {
    let root = FixtureRoot::new("strict-proof-terminal");
    root.write("Entry", &format!("import ProofTerms (Proved)\n{source}"));
    root.write("ProofTerms", "export Proved");
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")?;
    Ok(env)
}

/// Promise class: durable invariant (spec 16 §1.4; 33 §§3.2–3.3).
///
/// MEASURED: the strict Entry's selective import from a real ProofTerms.ken
/// yields a checked theorem body pointing to the exact fixed tt_id. CLAIMED:
/// file-backed facade re-export preserves the Top-introduction identity and
/// permits conversion without adding an assumption. THE GAP: the missing-file
/// negative below proves an in-memory export alone cannot supply the import.
#[test]
fn strict_file_backed_proof_terminal_facade_keeps_kernel_intro_identity() {
    let env = strict_one_with_proof_terminal("theorem selected_intro : Eq Bool True True = Proved")
        .expect("file-backed proof facade must elaborate by conversion");
    let tt = env.env.tt_id();
    assert_eq!(env.globals["Proved"], tt);
    assert!(!env.env.trusted_base().contains(&tt));
    let (_, body) = env
        .env
        .transparent_body(env.globals["Entry.selected_intro"])
        .expect("selected intro has a checked transparent body");
    match body {
        Term::Const { id, .. } => assert_eq!(id, tt),
        other => panic!("selectively imported Proved must be fixed tt: {other:?}"),
    }
}

/// Promise class: durable invariant (spec 33 §§3.2–3.3).
///
/// MEASURED: a strict file import of nonexistent ProofTerms.ken rejects at
/// that import, both cold and after a distinct in-memory module with the
/// same public spelling has been checked. CLAIMED: a globally preloaded
/// export is not file-loader authority. THE GAP: the checked positive above
/// shows that a real file-backed provider is importable instead.
#[test]
fn missing_proof_terms_file_rejects_despite_in_memory_preload() {
    let root = FixtureRoot::new("missing-proof-terms");
    root.write(
        "Entry",
        "import ProofTerms (Proved)\n\
         theorem selected_intro : Eq Bool True True = Proved",
    );
    for preload in [false, true] {
        let mut env = ElabEnv::new().expect("base environment");
        if preload {
            env.elaborate_file("module ProofTerms { export Proved }")
                .expect("separate legacy in-memory module");
        }
        match env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry") {
            Err(ElabError::UnboundName { name, span }) => {
                assert_eq!(name, "ProofTerms");
                assert_eq!(
                    (span.start, span.end),
                    (0, "import ProofTerms (Proved)".len())
                );
            }
            Err(other) => panic!("missing file must reject at import, got {other:?}"),
            Ok(_) => panic!("missing ProofTerms file was admitted, preload={preload}"),
        }
        assert!(!env.globals.contains_key("Entry.selected_intro"));
    }
}

/// Promise class: normative compatibility vector.
///
/// MEASURED: the primitive producer traversal yields exactly eight signature
/// identities, and Nat, Pair and five kernel-keyed proposition/equality members
/// close the type inventory to fifteen. The separately configured companion
/// inventory has three checked-transparent constants keyed to the exact Pair
/// identity. CLAIMED: type and binding membership are independent closed
/// inventories rather than one compiler-global allow-list. THE GAP: source
/// resolution through these inventories is pinned independently below.
#[test]
fn prelude_signature_inventory_is_executable_and_closed() {
    assert_eq!(PRELUDE_FLOOR_NAMES.as_slice(), TYPE_FLOOR.as_slice());
    assert_eq!(
        PRELUDE_COMPANION_BINDING_NAMES.as_slice(),
        COMPANIONS.as_slice()
    );

    let env = ElabEnv::new().expect("base environment");
    let signature_expected = TYPE_FLOOR
        .into_iter()
        .filter(|name| {
            !matches!(
                *name,
                "Nat" | "Pair" | "Bottom" | "Equal" | "Prop" | "Proved" | "Top"
            )
        })
        .map(|name| env.globals[name])
        .collect::<BTreeSet<_>>();
    assert_eq!(primitive_signature_type_ids(&env), signature_expected);

    let internal = ["Nat", "Pair", "Bottom", "Equal", "Prop", "Proved", "Top"]
        .map(|name| env.globals[name])
        .into_iter()
        .collect::<BTreeSet<_>>();
    let configured = PRELUDE_FLOOR_NAMES
        .map(|name| env.globals[name])
        .into_iter()
        .collect::<BTreeSet<_>>();
    let mut derived = signature_expected;
    derived.extend(internal);
    assert_eq!(configured, derived);
    assert!(!configured.contains(&env.globals["Prod"]));

    let pair = env.globals["Pair"];
    assert!(matches!(
        env.env.lookup(pair),
        Some(Decl::Transparent { .. })
    ));
    assert!(env.env.inductive(pair).is_none());
    assert!(!env.env.trusted_base().contains(&pair));
    for name in PRELUDE_COMPANION_BINDING_NAMES {
        let id = env.globals[name];
        assert!(!configured.contains(&id), "{name} is not a type member");
        assert!(matches!(env.env.lookup(id), Some(Decl::Transparent { .. })));
        let (_, ty) = env.env.const_type(id).expect("companion checked type");
        assert!(mentions_global(&ty, pair), "{name} type must name Pair");
        assert!(!env.env.trusted_base().contains(&id));
    }
}

/// Promise class: durable invariant.
///
/// MEASURED: four independent Strict roots resolve one bare Pair-family name
/// to its recorded pre-source id, allocate only their witness, and preserve
/// trust. CLAIMED: acceptance comes from the closed floor inventories and
/// reuses the compiler-bootstrap declarations. THE GAP: the nonmember controls
/// below exclude ambient-global fallback.
#[test]
fn strict_bare_pair_floor_name_matrix_accepts_exact_ids() {
    let cases = [
        (
            "Pair",
            "fn witness (x : Pair Bool Bool) : Pair Bool Bool = x",
        ),
        (
            "mk_pair",
            "const witness : Pair Bool Bool = mk_pair Bool Bool True False",
        ),
        (
            "pair_fst",
            "fn witness (p : Pair Bool Bool) : Bool = pair_fst Bool Bool p",
        ),
        (
            "pair_snd",
            "fn witness (p : Pair Bool Bool) : Bool = pair_snd Bool Bool p",
        ),
    ];

    for (name, source) in cases {
        let root = FixtureRoot::new(name);
        root.write("Entry", source);
        let mut env = ElabEnv::new().expect("base environment");
        let canonical = env.globals[name];
        let declarations_before = env.env.declarations().len();
        let next_before = env.env.next_global_id();
        let trusted_before = env.env.trusted_base();
        env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
            .unwrap_or_else(|error| panic!("bare {name} must resolve in Strict: {error}"));
        assert_eq!(env.globals[name], canonical);
        let witness = env.globals["Entry.witness"];
        let (_, ty) = env.env.const_type(witness).expect("witness checked type");
        let (_, body) = env
            .env
            .transparent_body(witness)
            .expect("witness checked body");
        assert!(
            mentions_global(&ty, canonical) || mentions_global(&body, canonical),
            "witness must contain the recorded {name} identity"
        );
        assert_eq!(env.env.declarations().len(), declarations_before + 1);
        assert_eq!(env.env.next_global_id().0, next_before.0 + 1);
        assert_eq!(env.env.trusted_base(), trusted_before);
    }
}

/// Pair floor availability is independent of loaded-unit cache contents.
#[test]
fn pair_floor_remains_available_after_unrelated_loads() {
    let root = FixtureRoot::new("unrelated-loads");
    root.write("Provider", "pub const provided : Bool = True");
    root.write("Facade", "export Provider (provided)");
    root.write(
        "Entry",
        "const p : Pair Bool Bool = mk_pair Bool Bool True False\n\
         const x : Bool = pair_fst Bool Bool p\n\
         const y : Bool = pair_snd Bool Bool p",
    );
    root.write("Leaky", "const leak : Bool = provided");
    root.write(
        "Explicit",
        "import Provider (provided)\nconst accepted : Bool = provided",
    );

    let mut env = ElabEnv::new().expect("base environment");
    let ids = PAIR_BINDINGS.map(|name| env.globals[name]);
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Provider")
        .expect("unrelated provider");
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Facade")
        .expect("unrelated facade");
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect("Pair floor after unrelated loads");
    assert_eq!(PAIR_BINDINGS.map(|name| env.globals[name]), ids);
    let error = env
        .elaborate_module_from_roots_strict(&[root.0.clone()], "Leaky")
        .expect_err("loaded provider declaration must not leak");
    assert!(matches!(error, ElabError::UnboundName { ref name, .. } if name == "provided"));
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Explicit")
        .expect("explicit import remains lawful");
}

fn assert_collision(source: &str, name: &str) {
    let root = FixtureRoot::new(name);
    root.write("Entry", source);
    let mut env = ElabEnv::new().expect("base environment");
    let ids = PAIR_BINDINGS.map(|binding| env.globals[binding]);
    let declarations_before = env.env.declarations().len();
    let next_before = env.env.next_global_id();
    let trusted_before = env.env.trusted_base();
    let error = env
        .elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect_err("Pair-floor binding collision must reject");
    assert!(
        matches!(error, ElabError::AmbiguousReference { name: ref rejected, .. } if rejected == name),
        "collision must reject at {name}, got {error:?}"
    );
    assert_eq!(PAIR_BINDINGS.map(|binding| env.globals[binding]), ids);
    assert_eq!(env.env.declarations().len(), declarations_before);
    assert_eq!(env.env.next_global_id(), next_before);
    assert_eq!(env.env.trusted_base(), trusted_before);
}

/// Every Pair binding is immutable at top level, while renamed declarations and
/// narrower lexical shadowing remain ordinary source behavior.
#[test]
fn pair_floor_binding_collisions_reject_before_allocation() {
    assert_collision("data Pair a b = LocalPairCtor a b", "Pair");
    assert_collision("const mk_pair : Bool = True", "mk_pair");
    assert_collision("const pair_fst : Bool = True", "pair_fst");
    assert_collision("const pair_snd : Bool = True", "pair_snd");

    strict_one(
        "data LocalPair a b = LocalPairCtor a b\n\
         const local_mk_pair : Bool = True\n\
         const local_pair_fst : Bool = True\n\
         const local_pair_snd : Bool = True",
    )
    .expect("all-renamed Pair lookalikes must remain lawful");
    strict_one("fn lexical (pair_fst : Bool) : Bool = pair_fst")
        .expect("narrow lexical binder must retain ordinary shadowing");
}

/// Promise class: durable invariant.
///
/// MEASURED: an ordinary fresh transparent alias has a distinct id while its
/// body converts with the canonical Pair body; an earlier checked reference
/// remains keyed to the floor id. CLAIMED: conversion never manufactures floor
/// provenance. THE GAP: no claim is made about source ownership for the fresh
/// declaration.
#[test]
fn definitionally_equal_pair_is_a_distinct_identity() {
    let mut env = ElabEnv::new().expect("base environment");
    let pair = env.globals["Pair"];
    env.elaborate_file(
        "const canonical_pair_witness : Pair Bool Bool = mk_pair Bool Bool True False\n\
         const LocalPair : Type -> Type -> Type = Pair",
    )
    .expect("fresh Pair-shaped checked definition");
    let local = env.globals["LocalPair"];
    assert_ne!(local, pair);

    let (_, pair_ty) = env.env.const_type(pair).expect("Pair type");
    let (_, pair_body) = env.env.transparent_body(pair).expect("Pair body");
    let (_, local_body) = env.env.transparent_body(local).expect("LocalPair body");
    assert!(convert(
        &env.env,
        &Context::new(),
        &pair_ty,
        &pair_body,
        &local_body
    ));

    let witness = env.globals["canonical_pair_witness"];
    let (_, witness_ty) = env.env.const_type(witness).expect("canonical witness type");
    let (_, witness_body) = env
        .env
        .transparent_body(witness)
        .expect("canonical witness body");
    assert!(mentions_global(&witness_ty, pair));
    assert!(!mentions_global(&witness_ty, local));
    assert!(!mentions_global(&witness_body, local));
}

/// Nonmembers remain unavailable even when the implementation global map owns
/// a checked declaration with the requested spelling.
#[test]
fn closed_floor_accepts_pair_but_arbitrary_globals_and_prod_do_not() {
    let root = FixtureRoot::new("closed");
    root.write(
        "PairEntry",
        "fn X (a : Type) (b : Type) (p : Pair a b) : Pair a b = p",
    );
    root.write("AmbientEntry", "def X = Ambient");
    root.write("ProdEntry", "def X = Prod Bool Bool");
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_file("def Ambient = Bool")
        .expect("register unrelated checked implementation global");
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "PairEntry")
        .expect("Pair type floor member");
    let ambient = env
        .elaborate_module_from_roots_strict(&[root.0.clone()], "AmbientEntry")
        .expect_err("arbitrary implementation global must remain unavailable");
    assert!(matches!(ambient, ElabError::UnboundName { ref name, .. } if name == "Ambient"));
    let prod = env
        .elaborate_module_from_roots_strict(&[root.0.clone()], "ProdEntry")
        .expect_err("Prod must remain unavailable");
    assert!(matches!(prod, ElabError::UnboundName { ref name, .. } if name == "Prod"));
}

/// Re-export republishes the existing four identities and allocates no provider
/// replacement.
#[test]
fn pair_reexport_is_identity_preserving_republication() {
    let root = FixtureRoot::new("reexport");
    root.write("Facade", "export Pair, mk_pair, pair_fst, pair_snd");
    root.write(
        "Entry",
        "import Facade (Pair, mk_pair, pair_fst, pair_snd)\n\
         const p : Pair Bool Bool = mk_pair Bool Bool True False\n\
         const x : Bool = pair_fst Bool Bool p\n\
         const y : Bool = pair_snd Bool Bool p",
    );
    let mut env = ElabEnv::new().expect("base environment");
    let ids = PAIR_BINDINGS.map(|name| env.globals[name]);
    let declarations_before = env.env.declarations().len();
    let next_before = env.env.next_global_id();
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Facade")
        .expect("floor facade");
    assert_eq!(env.env.declarations().len(), declarations_before);
    assert_eq!(env.env.next_global_id(), next_before);
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect("floor facade consumer");
    assert_eq!(PAIR_BINDINGS.map(|name| env.globals[name]), ids);
}

/// The reused checked-transparent family retains its Sigma beta/eta conversion
/// behavior and the two non-interchangeable proof terminals.
#[test]
fn pair_floor_beta_eta_are_definitional() {
    strict_one_with_proof_terminal(
        "theorem fst_beta\n\
           : Eq Bool (pair_fst Bool Bool (mk_pair Bool Bool True False)) True = Proved\n\
         theorem snd_beta\n\
           : Eq Bool (pair_snd Bool Bool (mk_pair Bool Bool True False)) False = Proved\n\
         theorem eta (p : Pair Bool Bool)\n\
           : Eq (Pair Bool Bool)\n\
               (mk_pair Bool Bool (pair_fst Bool Bool p) (pair_snd Bool Bool p)) p = Refl",
    )
    .expect("Pair beta and eta equations must hold by conversion");

    for (index, source) in [
        "theorem bad : Eq Bool (pair_fst Bool Bool (mk_pair Bool Bool True False)) True = Refl",
        "theorem bad : Eq Bool (pair_snd Bool Bool (mk_pair Bool Bool True False)) False = Refl",
        "theorem bad (p : Pair Bool Bool) : Eq (Pair Bool Bool) (mk_pair Bool Bool (pair_fst Bool Bool p) (pair_snd Bool Bool p)) p = Proved",
        "theorem bad : Eq Bool (pair_fst Bool Bool (mk_pair Bool Bool True False)) False = Proved",
        "theorem bad : Eq Bool (pair_snd Bool Bool (mk_pair Bool Bool True False)) True = Proved",
    ]
    .into_iter()
    .enumerate()
    {
        let error = match strict_one_with_proof_terminal(source) {
            Ok(_) => panic!("wrong Pair equation/terminal must reject: {source}"),
            Err(error) => error,
        };
        match (index, error) {
            (0 | 1, ElabError::TypeMismatch { reason, span }) => {
                assert_eq!(reason, "Refl expects an `Eq`-shaped goal");
                assert!(span.start > "import ProofTerms (Proved)".len());
            }
            (
                2..=4,
                ElabError::KernelRejected {
                    error: ken_kernel::KernelError::TypeMismatch { .. },
                    span,
                },
            ) => assert!(span.start >= "import ProofTerms (Proved)".len()),
            (_, other) => panic!("wrong proof must reject at its obligation: {other:?}"),
        }
    }
}

/// Fresh Strict execution re-derives the frontier after the five additional
/// proposition/equality floor members, without projecting Legacy evidence or
/// declaring every catalog subject closed.
#[test]
fn pair_floor_closure_is_rederived_after_realization() {
    let root = catalog_root();
    let expected_failures = BTreeSet::from([
        "Algorithm.Numeric.Gcd",
        "Core.Classes.LawfulClasses",
        "Core.Logic.Compare",
        "Data.Collections.Derived",
        "Data.Numeric.Nat.Order",
    ]);
    let mut observed_failures = BTreeSet::new();
    let mut observed_successes = BTreeSet::new();

    for module in &expected_failures {
        let mut env = ElabEnv::new().expect("base environment");
        let pair_bindings = PAIR_BINDINGS.map(|name| env.globals[name]);
        let result = env.elaborate_module_from_roots_strict(&[root.clone()], module);
        assert_eq!(
            PAIR_BINDINGS.map(|name| env.globals[name]),
            pair_bindings,
            "{module} must retain every pre-source Pair identity"
        );
        for id in pair_bindings {
            assert!(
                !env.env.trusted_base().contains(&id),
                "{module} must not trust a Pair-floor declaration"
            );
        }
        match result {
            Err(ElabError::UnboundName { name, .. }) if name == "And" => {
                observed_failures.insert(*module);
            }
            Ok(_) => {
                observed_successes.insert(*module);
            }
            other => panic!(
                "{module} must stop at the exact post-fifteen UnboundName(And) frontier, got {other:?}"
            ),
        }
    }
    assert_eq!(observed_failures, expected_failures);
    assert!(
        observed_successes.is_empty(),
        "the five governed rows have no earned Strict success yet"
    );

    let prod = match strict_one("def X = Prod Bool Bool") {
        Ok(_) => panic!("Prod must not enter the closure authority"),
        Err(error) => error,
    };
    assert!(matches!(prod, ElabError::UnboundName { ref name, .. } if name == "Prod"));

    let ambient_root = FixtureRoot::new("closure-ambient");
    ambient_root.write("Entry", "def X = Ambient");
    let mut ambient_env = ElabEnv::new().expect("base environment");
    ambient_env
        .elaborate_file("def Ambient = Bool")
        .expect("register checked non-floor global");
    let ambient = ambient_env
        .elaborate_module_from_roots_strict(&[ambient_root.0.clone()], "Entry")
        .expect_err("arbitrary checked global must not enter closure authority");
    assert!(matches!(ambient, ElabError::UnboundName { ref name, .. } if name == "Ambient"));
}

/// Named Pair follows transparent Sigma positivity through both components and
/// rejects an inner-arrow negative in either component.
#[test]
fn floor_pair_positive_path_unfolds_to_sigma() {
    for source in [
        "data Good1 = MkGood1 (Pair Good1 Bool)",
        "data Good2 = MkGood2 (Pair Bool Good2)",
    ] {
        strict_one(source).expect("positive Pair recursion must elaborate");
    }
    for source in [
        "data Bad1 = MkBad1 (Pair (Bad1 -> Bool) Bool)",
        "data Bad2 = MkBad2 (Pair Bool (Bad2 -> Bool))",
    ] {
        assert!(
            strict_one(source).is_err(),
            "inner-arrow negative Pair recursion must reject"
        );
    }
}
