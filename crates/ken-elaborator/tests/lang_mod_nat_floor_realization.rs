//! Behavioral pins for `LANG-MOD-NAT-FLOOR-REALIZATION`.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::modules::PRELUDE_FLOOR_NAMES;
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{Decl, GlobalId, PrimReduction, Term, declare_primitive};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

const LANDED_FLOOR_NAMES: [&str; 9] = [
    "Auth",
    "Bool",
    "Char",
    "List",
    "Nat",
    "Option",
    "ResourceKind",
    "Result",
    "Utf8Error",
];

struct FloorCase {
    name: &'static str,
    constructors: &'static [&'static str],
    source: &'static str,
}

const FLOOR_CASES: [FloorCase; 9] = [
    FloorCase {
        name: "Auth",
        constructors: &["ANone", "APartial", "AFull"],
        source: "fn witness (x : Auth) : Auth = match x { ANone |-> APartial ; APartial |-> AFull ; AFull |-> ANone }",
    },
    FloorCase {
        name: "Bool",
        constructors: &["True", "False"],
        source: "fn witness (x : Bool) : Bool = match x { True |-> False ; False |-> True }",
    },
    FloorCase {
        name: "Char",
        constructors: &[],
        source: "fn witness (x : Char) : Char = x",
    },
    FloorCase {
        name: "List",
        constructors: &["Nil", "Cons"],
        source: "fn witness (x : Bool) : List Bool = Cons Bool x (Nil Bool)",
    },
    FloorCase {
        name: "Nat",
        constructors: &["Zero", "Suc"],
        source: "const witness : Nat = Suc Zero",
    },
    FloorCase {
        name: "Option",
        constructors: &["None", "Some"],
        source: "fn witness (x : Option Bool) : Option Bool = match x { None |-> Some Bool True ; Some y |-> None Bool }",
    },
    FloorCase {
        name: "ResourceKind",
        constructors: &["FsHandle", "Buffer"],
        source: "fn witness (x : ResourceKind) : ResourceKind = match x { FsHandle |-> Buffer ; Buffer |-> FsHandle }",
    },
    FloorCase {
        name: "Result",
        constructors: &["Err", "Ok"],
        source: "fn witness (x : Result Bool Bool) : Result Bool Bool = match x { Err e |-> Ok Bool Bool e ; Ok a |-> Err Bool Bool a }",
    },
    FloorCase {
        name: "Utf8Error",
        constructors: &["InvalidUtf8"],
        source: "const witness : Utf8Error = InvalidUtf8",
    },
];

struct FixtureRoot(PathBuf);

impl FixtureRoot {
    fn new(label: &str) -> Self {
        let serial = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ken-nat-floor-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("create Nat-floor fixture root");
        Self(path)
    }

    fn write(&self, source: &str) {
        fs::write(self.0.join("Entry.ken"), source).expect("write Nat-floor fixture");
    }
}

impl Drop for FixtureRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn assert_canonical_family(env: &ElabEnv, case: &FloorCase) {
    let family_id = env.globals[case.name];
    let trusted = env.env.trusted_base();
    assert!(
        !trusted.contains(&family_id),
        "floor family {} must remain outside trusted_base()",
        case.name
    );

    if case.constructors.is_empty() {
        assert!(
            env.env.inductive(family_id).is_none(),
            "constructor-free floor member {} must keep its existing non-inductive identity",
            case.name
        );
        return;
    }

    let inductive = env
        .env
        .inductive(family_id)
        .unwrap_or_else(|| panic!("{} must keep its existing inductive identity", case.name));
    let expected_ids = case
        .constructors
        .iter()
        .map(|name| env.globals[*name])
        .collect::<Vec<_>>();
    let actual_ids = inductive
        .constructors
        .iter()
        .map(|constructor| constructor.id)
        .collect::<Vec<_>>();
    assert_eq!(actual_ids, expected_ids, "{} constructor roster", case.name);
    for constructor_id in expected_ids {
        assert!(
            !trusted.contains(&constructor_id),
            "floor constructor {constructor_id:?} must remain outside trusted_base()"
        );
        let (parent, _) = env
            .env
            .constructor(constructor_id)
            .expect("expected floor constructor must be kernel-recorded");
        assert_eq!(
            parent.id, family_id,
            "floor constructor must retain the exact canonical parent identity"
        );
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

fn collect_global_ids(term: &Term, found: &mut BTreeSet<GlobalId>) {
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
        collect_global_ids(child, found);
    }
}

fn primitive_signature_type_ids(env: &ElabEnv) -> BTreeSet<GlobalId> {
    let mut referenced = BTreeSet::new();
    for declaration in env.env.decls() {
        if let Decl::Primitive { ty, .. } = declaration {
            collect_global_ids(ty, &mut referenced);
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

/// Promise class: normative compatibility vector. The configured floor must be
/// exactly the executable primitive-signature inventory plus bootstrap Nat.
///
/// **MEASURED:** walking every kernel `Decl::Primitive` type reaches exactly
/// the eight checked surface-type identities and a test-only primitive extends
/// that result with its checked `Extra` parameter. **CLAIMED:** the signature
/// arm is producer-closed rather than a selected helper census, and adding Nat
/// yields exactly the configured floor. **THE GAP:** bootstrap Nat membership
/// remains a separately specified identity arm; the primitive traversal does
/// not derive it.
#[test]
fn primitive_signature_inventory_is_executable_and_closed() {
    let mut env = ElabEnv::new().expect("base environment");
    let expected_signature = LANDED_FLOOR_NAMES
        .into_iter()
        .filter(|name| *name != "Nat")
        .map(|name| env.globals[name])
        .collect::<BTreeSet<_>>();
    let observed = primitive_signature_type_ids(&env);
    assert_eq!(observed, expected_signature);

    let configured = PRELUDE_FLOOR_NAMES
        .into_iter()
        .map(|name| env.globals[name])
        .collect::<BTreeSet<_>>();
    let mut observed_plus_nat = observed.clone();
    assert!(observed_plus_nat.insert(env.globals["Nat"]));
    assert_eq!(configured, observed_plus_nat);
    assert!(!configured.contains(&env.globals["Prod"]));

    env.elaborate_file("data Extra = MkExtra")
        .expect("positive-control type must be ordinary checked data");
    let extra = env.globals["Extra"];
    let extra_type = Term::indformer(extra, vec![]);
    declare_primitive(
        &mut env.env,
        vec![],
        Term::pi(extra_type.clone(), extra_type),
        PrimReduction::Op {
            symbol: "nat_floor_extra_probe",
        },
    )
    .expect("positive-control primitive must be kernel checked");
    let with_extra = primitive_signature_type_ids(&env);
    assert_eq!(
        with_extra
            .difference(&observed)
            .copied()
            .collect::<BTreeSet<_>>(),
        [extra].into_iter().collect(),
        "producer-side control must make the derived signature inventory grow"
    );
}

/// Promise class: durable invariant. Strict roots admits every landed floor
/// family and all its kernel-recorded constructors by their existing ids.
///
/// **MEASURED:** each per-family fixture elaborates through strict roots, its
/// checked type/body mentions the pre-existing family and constructor ids, and
/// only the fixture's one witness declaration is allocated. **CLAIMED:** the
/// nine-name floor reuses canonical identities and is constructor-parent
/// closed with zero trust growth. **THE GAP:** this checks the current closed
/// inventory; the producer-derived signature equality above guards why eight
/// members belong.
#[test]
fn strict_roots_accept_all_nine_canonical_families_and_constructors() {
    assert_eq!(PRELUDE_FLOOR_NAMES, LANDED_FLOOR_NAMES);

    for case in &FLOOR_CASES {
        let root = FixtureRoot::new(case.name);
        root.write(case.source);
        let mut env = ElabEnv::new().expect("base environment");
        assert_canonical_family(&env, case);
        let family_id = env.globals[case.name];
        let constructor_ids = case
            .constructors
            .iter()
            .map(|name| env.globals[*name])
            .collect::<Vec<_>>();
        let declarations_before = env.env.declarations().len();
        let next_id_before = env.env.next_global_id();
        let trusted_before = env.env.trusted_base();

        env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
            .unwrap_or_else(|error| panic!("strict floor {} must elaborate: {error}", case.name));

        assert_eq!(env.globals[case.name], family_id);
        assert!(!env.globals.contains_key(&format!("Entry.{}", case.name)));
        let witness = env.globals["Entry.witness"];
        let (_, witness_type) = env
            .env
            .const_type(witness)
            .expect("floor witness must have a checked type");
        assert!(
            mentions_global(&witness_type, family_id),
            "{} witness type must use the canonical family id",
            case.name
        );
        let (_, witness_body) = env
            .env
            .transparent_body(witness)
            .expect("floor witness must be transparent");
        for constructor_id in constructor_ids {
            assert!(
                mentions_global(&witness_body, constructor_id),
                "{} witness body must use every canonical constructor",
                case.name
            );
        }
        assert_eq!(env.env.declarations().len(), declarations_before + 1);
        assert_eq!(env.env.next_global_id().0, next_id_before.0 + 1);
        assert_eq!(env.env.trusted_base(), trusted_before);
    }
}

/// Promise class: durable invariant. Installing and exporting the configured
/// floor is namespace bookkeeping only and adds no kernel declaration or trust.
#[test]
fn strict_floor_export_allocates_nothing_and_nonmember_prod_rejects() {
    let root = FixtureRoot::new("zero-allocation");
    root.write("export Auth, Bool, Char, List, Nat, Option, ResourceKind, Result, Utf8Error");
    let mut env = ElabEnv::new().expect("base environment");
    let ids_before = LANDED_FLOOR_NAMES.map(|name| env.globals[name]);
    let declarations_before = env.env.declarations().len();
    let next_id_before = env.env.next_global_id();
    let trusted_before = env.env.trusted_base();
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect("strict floor names must export without declarations");
    assert_eq!(LANDED_FLOOR_NAMES.map(|name| env.globals[name]), ids_before);
    assert_eq!(env.env.declarations().len(), declarations_before);
    assert_eq!(env.env.next_global_id(), next_id_before);
    assert_eq!(env.env.trusted_base(), trusted_before);

    let nonmember = FixtureRoot::new("nonmember");
    nonmember.write("const witness : Prod Bool Bool = MkProd Bool Bool True False");
    let error = ElabEnv::new()
        .expect("base environment")
        .elaborate_module_from_roots_strict(&[nonmember.0.clone()], "Entry")
        .expect_err("pre-installed nonmember Prod must remain unavailable in strict roots");
    assert!(
        matches!(error, ElabError::UnboundName { ref name, .. } if name == "Prod"),
        "closed-floor rejection must name Prod, got {error:?}"
    );
}

/// Promise class: durable invariant. A source family spelled `Nat` is a fresh
/// identity and cannot be imported over the canonical floor binding.
#[test]
fn source_redeclared_nat_is_distinct_and_cannot_shadow_the_floor() {
    let root = FixtureRoot::new("source-nat");
    root.write("data Nat = FreshZero\nexport Nat, FreshZero");
    fs::write(
        root.0.join("Consumer.ken"),
        "import Entry (Nat)\nfn witness (x : Nat) : Nat = x",
    )
    .expect("write source-Nat consumer");

    let mut env = ElabEnv::new().expect("base environment");
    let canonical_nat = env.globals["Nat"];
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect("source-local Nat may exist only under its qualified identity");
    let source_nat = env.globals["Entry.Nat"];
    assert_ne!(source_nat, canonical_nat);
    let (parent, _) = env
        .env
        .constructor(env.globals["Entry.FreshZero"])
        .expect("source constructor must be kernel-recorded");
    assert_eq!(parent.id, source_nat);
    assert_eq!(env.globals["Nat"], canonical_nat);

    let error = env
        .elaborate_module_from_roots_strict(&[root.0.clone()], "Consumer")
        .expect_err("a distinct source Nat must not replace the prelude floor identity");
    assert!(
        matches!(error, ElabError::AmbiguousReference { ref name, .. } if name == "Nat"),
        "source Nat import must fail closed at the floor collision, got {error:?}"
    );
}
