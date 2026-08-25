//! Behavioral pins for `LANG-MOD-NAT-FLOOR-REALIZATION`.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::modules::PRELUDE_FLOOR_NAMES;
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{declare_primitive, Decl, GlobalId, PrimReduction, Term};

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

struct CollisionCase {
    parent: &'static str,
    parent_collision_source: &'static str,
    constructor_collisions: &'static [(&'static str, &'static str)],
    renamed_source: &'static str,
    renamed_parent: &'static str,
    renamed_constructors: &'static [&'static str],
}

const COLLISION_CASES: [CollisionCase; 8] = [
    CollisionCase {
        parent: "Auth",
        parent_collision_source: "data Auth = LocalANone | LocalAPartial | LocalAFull",
        constructor_collisions: &[
            (
                "ANone",
                "data LocalAuth = ANone | LocalAPartial | LocalAFull",
            ),
            (
                "APartial",
                "data LocalAuth = LocalANone | APartial | LocalAFull",
            ),
            (
                "AFull",
                "data LocalAuth = LocalANone | LocalAPartial | AFull",
            ),
        ],
        renamed_source: "data LocalAuth = LocalANone | LocalAPartial | LocalAFull",
        renamed_parent: "LocalAuth",
        renamed_constructors: &["LocalANone", "LocalAPartial", "LocalAFull"],
    },
    CollisionCase {
        parent: "Bool",
        parent_collision_source: "data Bool = LocalTrue | LocalFalse",
        constructor_collisions: &[
            ("True", "data LocalBool = True | LocalFalse"),
            ("False", "data LocalBool = LocalTrue | False"),
        ],
        renamed_source: "data LocalBool = LocalTrue | LocalFalse",
        renamed_parent: "LocalBool",
        renamed_constructors: &["LocalTrue", "LocalFalse"],
    },
    CollisionCase {
        parent: "List",
        parent_collision_source: "data List a = LocalNil | LocalCons a (List a)",
        constructor_collisions: &[
            ("Nil", "data LocalList a = Nil | LocalCons a (LocalList a)"),
            ("Cons", "data LocalList a = LocalNil | Cons a (LocalList a)"),
        ],
        renamed_source: "data LocalList a = LocalNil | LocalCons a (LocalList a)",
        renamed_parent: "LocalList",
        renamed_constructors: &["LocalNil", "LocalCons"],
    },
    CollisionCase {
        parent: "Nat",
        parent_collision_source: "data Nat = LocalZero | LocalSuc Nat",
        constructor_collisions: &[
            ("Zero", "data LocalNat = Zero | LocalSuc LocalNat"),
            ("Suc", "data LocalNat = LocalZero | Suc LocalNat"),
        ],
        renamed_source: "data LocalNat = LocalZero | LocalSuc LocalNat",
        renamed_parent: "LocalNat",
        renamed_constructors: &["LocalZero", "LocalSuc"],
    },
    CollisionCase {
        parent: "Option",
        parent_collision_source: "data Option a = LocalNone | LocalSome a",
        constructor_collisions: &[
            ("None", "data LocalOption a = None | LocalSome a"),
            ("Some", "data LocalOption a = LocalNone | Some a"),
        ],
        renamed_source: "data LocalOption a = LocalNone | LocalSome a",
        renamed_parent: "LocalOption",
        renamed_constructors: &["LocalNone", "LocalSome"],
    },
    CollisionCase {
        parent: "ResourceKind",
        parent_collision_source: "data ResourceKind = LocalFsHandle | LocalBuffer",
        constructor_collisions: &[
            (
                "FsHandle",
                "data LocalResourceKind = FsHandle | LocalBuffer",
            ),
            ("Buffer", "data LocalResourceKind = LocalFsHandle | Buffer"),
        ],
        renamed_source: "data LocalResourceKind = LocalFsHandle | LocalBuffer",
        renamed_parent: "LocalResourceKind",
        renamed_constructors: &["LocalFsHandle", "LocalBuffer"],
    },
    CollisionCase {
        parent: "Result",
        parent_collision_source: "data Result e a = LocalErr e | LocalOk a",
        constructor_collisions: &[
            ("Err", "data LocalResult e a = Err e | LocalOk a"),
            ("Ok", "data LocalResult e a = LocalErr e | Ok a"),
        ],
        renamed_source: "data LocalResult e a = LocalErr e | LocalOk a",
        renamed_parent: "LocalResult",
        renamed_constructors: &["LocalErr", "LocalOk"],
    },
    CollisionCase {
        parent: "Utf8Error",
        parent_collision_source: "data Utf8Error = LocalInvalidUtf8",
        constructor_collisions: &[("InvalidUtf8", "data LocalUtf8Error = InvalidUtf8")],
        renamed_source: "data LocalUtf8Error = LocalInvalidUtf8",
        renamed_parent: "LocalUtf8Error",
        renamed_constructors: &["LocalInvalidUtf8"],
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
        self.write_named("Entry", source);
    }

    fn write_named(&self, module: &str, source: &str) {
        fs::write(self.0.join(format!("{module}.ken")), source).expect("write Nat-floor fixture");
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
    assert_eq!(
        PRELUDE_FLOOR_NAMES.as_slice(),
        LANDED_FLOOR_NAMES.as_slice()
    );

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

fn floor_ids(env: &ElabEnv) -> BTreeSet<GlobalId> {
    FLOOR_CASES
        .iter()
        .flat_map(|case| {
            std::iter::once(env.globals[case.name])
                .chain(case.constructors.iter().map(|name| env.globals[*name]))
        })
        .collect()
}

fn assert_floor_collision_rejects_before_allocation(label: &str, source: &str, name: &str) {
    let root = FixtureRoot::new(label);
    root.write(source);
    let mut env = ElabEnv::new().expect("base environment");
    let declarations_before = env.env.declarations().len();
    let next_id_before = env.env.next_global_id();
    let trusted_before = env.env.trusted_base();

    let error = env
        .elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect_err("a same-spelling floor binding must reject under the non-empty Entry prefix");
    assert!(
        matches!(error, ElabError::AmbiguousReference { name: ref rejected, .. } if rejected == name),
        "{label} must reject at the retained floor spelling {name}, got {error:?}"
    );
    assert_eq!(env.env.declarations().len(), declarations_before);
    assert_eq!(env.env.next_global_id(), next_id_before);
    assert_eq!(env.env.trusted_base(), trusted_before);
}

fn assert_renamed_family_accepts(case: &CollisionCase) {
    let root = FixtureRoot::new(case.renamed_parent);
    root.write(case.renamed_source);
    let mut env = ElabEnv::new().expect("base environment");
    let canonical_ids = floor_ids(&env);
    let trusted_before = env.env.trusted_base();

    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .unwrap_or_else(|error| panic!("all-renamed {} must elaborate: {error}", case.parent));

    let local_parent = env.globals[&format!("Entry.{}", case.renamed_parent)];
    assert!(!canonical_ids.contains(&local_parent));
    let local_inductive = env
        .env
        .inductive(local_parent)
        .expect("renamed floor lookalike must be an ordinary local inductive");
    let local_constructors = case
        .renamed_constructors
        .iter()
        .map(|name| env.globals[&format!("Entry.{name}")])
        .collect::<Vec<_>>();
    assert_eq!(
        local_inductive
            .constructors
            .iter()
            .map(|constructor| constructor.id)
            .collect::<Vec<_>>(),
        local_constructors
    );
    for constructor in local_constructors {
        assert!(!canonical_ids.contains(&constructor));
        let (parent, _) = env
            .env
            .constructor(constructor)
            .expect("renamed constructor must be kernel-recorded");
        assert_eq!(parent.id, local_parent);
    }
    assert_eq!(env.env.trusted_base(), trusted_before);
}

/// Promise class: durable invariant. Every parent and exact-parent constructor
/// in the closed floor is unshadowable before allocation under a non-empty
/// module prefix, while equal-shaped all-renamed declarations remain ordinary
/// checked local identities.
///
/// **MEASURED:** one-axis parent and constructor rows reject at the retained
/// spelling with unchanged declarations, allocator, and trust; one all-renamed
/// positive per family plus an explicit-data spelling control allocates only
/// distinct local families and constructors with kernel-recorded local parentage.
/// **CLAIMED:** prelude immutability covers
/// the complete exact-parent-derived floor binding set, not shape or an
/// arbitrary compiler-name inventory. **THE GAP:** selective-import collisions
/// use the same set through a separate production entry, pinned below.
#[test]
fn floor_parent_and_constructor_clash_matrix_is_fail_closed() {
    for case in &COLLISION_CASES {
        assert_floor_collision_rejects_before_allocation(
            &format!("{}-parent", case.parent),
            case.parent_collision_source,
            case.parent,
        );
        for (constructor, source) in case.constructor_collisions {
            assert_floor_collision_rejects_before_allocation(
                &format!("{}-{constructor}", case.parent),
                source,
                constructor,
            );
        }
        assert_renamed_family_accepts(case);
    }

    assert_floor_collision_rejects_before_allocation(
        "explicit-zero",
        "data ExplicitLocalNat : Type where { Zero : ExplicitLocalNat }",
        "Zero",
    );
    let explicit = FixtureRoot::new("explicit-renamed");
    explicit.write("data ExplicitLocalNat : Type where { ExplicitLocalZero : ExplicitLocalNat }");
    let mut explicit_env = ElabEnv::new().expect("base environment");
    let explicit_floor = floor_ids(&explicit_env);
    let explicit_trusted = explicit_env.env.trusted_base();
    explicit_env
        .elaborate_module_from_roots_strict(&[explicit.0.clone()], "Entry")
        .expect("all-renamed explicit family must elaborate");
    let explicit_parent = explicit_env.globals["Entry.ExplicitLocalNat"];
    let explicit_constructor = explicit_env.globals["Entry.ExplicitLocalZero"];
    assert!(!explicit_floor.contains(&explicit_parent));
    assert!(!explicit_floor.contains(&explicit_constructor));
    let (recorded_parent, _) = explicit_env
        .env
        .constructor(explicit_constructor)
        .expect("renamed explicit constructor must be kernel-recorded");
    assert_eq!(recorded_parent.id, explicit_parent);
    assert_eq!(explicit_env.env.trusted_base(), explicit_trusted);

    assert_floor_collision_rejects_before_allocation("char-parent", "def Char = Int", "Char");
    let root = FixtureRoot::new("local-char");
    root.write("def LocalChar = Int");
    let mut env = ElabEnv::new().expect("base environment");
    let floor = floor_ids(&env);
    let declarations_before = env.env.declarations().len();
    let next_id_before = env.env.next_global_id();
    let trusted_before = env.env.trusted_base();
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect("renamed constructor-free Char lookalike must elaborate");
    let local_char = env.globals["Entry.LocalChar"];
    assert!(!floor.contains(&local_char));
    assert!(matches!(
        env.env.lookup(local_char),
        Some(Decl::Transparent { .. })
    ));
    assert_eq!(env.env.declarations().len(), declarations_before + 1);
    assert_eq!(env.env.next_global_id().0, next_id_before.0 + 1);
    assert_eq!(env.env.trusted_base(), trusted_before);
}

/// Promise class: durable invariant. Selective imports consult the complete
/// floor binding set, while per-item renaming and qualification remain lawful.
///
/// **MEASURED:** importing an all-renamed local constructor under `Zero`
/// rejects without allocation after its provider is loaded, while importing it
/// as `UserZero` and using the qualified provider path both elaborate.
/// **CLAIMED:** import collision checking shares the exact-derived floor
/// binding authority with local prebinding. **THE GAP:** one representative
/// constructor reaches the shared set; the matrix above proves every specified
/// parent and constructor belongs to that set.
#[test]
fn selective_import_floor_collision_rejects_but_renamed_and_qualified_access_accept() {
    let root = FixtureRoot::new("selective-import");
    root.write_named(
        "Provider",
        "data LocalNat = LocalZero | LocalSuc LocalNat\nexport LocalNat, LocalZero, LocalSuc",
    );
    root.write("import Provider (LocalZero as Zero)");
    let mut env = ElabEnv::new().expect("base environment");
    env.elaborate_module_from_roots_strict(&[root.0.clone()], "Provider")
        .expect("all-renamed provider must elaborate");
    let declarations_before = env.env.declarations().len();
    let next_id_before = env.env.next_global_id();
    let trusted_before = env.env.trusted_base();
    let error = env
        .elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
        .expect_err("a selective import may not bind a floor constructor spelling");
    assert!(
        matches!(error, ElabError::AmbiguousReference { ref name, .. } if name == "Zero"),
        "selective collision must name Zero, got {error:?}"
    );
    assert_eq!(env.env.declarations().len(), declarations_before);
    assert_eq!(env.env.next_global_id(), next_id_before);
    assert_eq!(env.env.trusted_base(), trusted_before);

    let renamed = FixtureRoot::new("renamed-import");
    renamed.write_named(
        "Provider",
        "data LocalNat = LocalZero | LocalSuc LocalNat\nexport LocalNat, LocalZero, LocalSuc",
    );
    renamed.write(
        "import Provider (LocalNat as UserNat, LocalZero as UserZero)\nconst witness : UserNat = UserZero",
    );
    ElabEnv::new()
        .expect("base environment")
        .elaborate_module_from_roots_strict(&[renamed.0.clone()], "Entry")
        .expect("per-item renaming must preserve lawful access");

    let qualified = FixtureRoot::new("qualified-import");
    qualified.write_named(
        "Provider",
        "data LocalNat = LocalZero | LocalSuc LocalNat\nexport LocalNat, LocalZero, LocalSuc",
    );
    qualified.write("import Provider\nconst witness : Provider.LocalNat = Provider.LocalZero");
    ElabEnv::new()
        .expect("base environment")
        .elaborate_module_from_roots_strict(&[qualified.0.clone()], "Entry")
        .expect("qualified access must preserve the provider identity without a bare collision");
}
