//! Behavioral pins for `LANG-MOD-NAT-FLOOR-REALIZATION`.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::modules::PRELUDE_FLOOR_NAMES;
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{declare_primitive, Decl, GlobalId, PrimReduction, Term};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

const LANDED_FLOOR_NAMES: [&str; 10] = [
    "Auth",
    "Bool",
    "Char",
    "List",
    "Nat",
    "Option",
    "Pair",
    "ResourceKind",
    "Result",
    "Utf8Error",
];

struct FloorCase {
    name: &'static str,
    constructors: &'static [&'static str],
    source: &'static str,
}

const FLOOR_CASES: [FloorCase; 10] = [
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
        name: "Pair",
        constructors: &[],
        source: "fn witness (x : Pair Bool Bool) : Pair Bool Bool = x",
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

/// Promise class: normative compatibility vector. The configured type floor
/// must be exactly the executable primitive-signature inventory plus the two
/// internal-provision identities Nat and Pair.
///
/// **MEASURED:** walking every kernel `Decl::Primitive` type reaches exactly
/// the eight checked surface-type identities and a test-only primitive extends
/// that result with its checked `Extra` parameter. **CLAIMED:** the signature
/// arm is producer-closed rather than a selected helper census, and adding the
/// two independently specified internal identities yields exactly the configured
/// floor. **THE GAP:** the primitive traversal does not derive either internal
/// provision witness.
#[test]
fn primitive_signature_inventory_is_executable_and_closed() {
    let mut env = ElabEnv::new().expect("base environment");
    let expected_signature = LANDED_FLOOR_NAMES
        .into_iter()
        .filter(|name| !matches!(*name, "Nat" | "Pair"))
        .map(|name| env.globals[name])
        .collect::<BTreeSet<_>>();
    let observed = primitive_signature_type_ids(&env);
    assert_eq!(observed, expected_signature);

    let configured = PRELUDE_FLOOR_NAMES
        .into_iter()
        .map(|name| env.globals[name])
        .collect::<BTreeSet<_>>();
    let mut observed_plus_internal_provision = observed.clone();
    assert!(observed_plus_internal_provision.insert(env.globals["Nat"]));
    assert!(observed_plus_internal_provision.insert(env.globals["Pair"]));
    assert_eq!(configured, observed_plus_internal_provision);
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
/// ten-name type floor reuses canonical identities and is constructor-parent
/// closed with zero trust growth. **THE GAP:** this checks the current closed
/// inventory; the producer-derived signature equality above guards why eight
/// members belong.
#[test]
fn strict_roots_accept_all_ten_canonical_families_and_constructors() {
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
    root.write("export Auth, Bool, Char, List, Nat, Option, Pair, ResourceKind, Result, Utf8Error");
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
    let canonical_id = env.globals[name];
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
    assert_eq!(env.globals[name], canonical_id);
    assert_eq!(env.env.declarations().len(), declarations_before);
    assert_eq!(env.env.next_global_id(), next_id_before);
    assert_eq!(env.env.trusted_base(), trusted_before);
}

fn assert_public_floor_collision_rejects_before_allocation(label: &str, source: &str, name: &str) {
    let mut env = ElabEnv::new().expect("base environment");
    let canonical_bindings = FLOOR_CASES
        .iter()
        .flat_map(|case| {
            std::iter::once(case.name)
                .chain(case.constructors.iter().copied())
                .map(|name| (name, env.globals[name]))
        })
        .collect::<Vec<_>>();
    let declarations_before = env.env.declarations().len();
    let next_id_before = env.env.next_global_id();
    let trusted_before = env.env.trusted_base();

    let error = env
        .elaborate_file(source)
        .expect_err("a public same-spelling floor binding must reject before allocation");
    assert!(
        matches!(error, ElabError::AmbiguousReference { name: ref rejected, .. } if rejected == name),
        "{label} must reject at {name}, got {error:?}"
    );
    for (floor_name, canonical_id) in canonical_bindings {
        assert_eq!(
            env.globals[floor_name], canonical_id,
            "{label} changed canonical floor binding {floor_name}"
        );
    }
    assert_eq!(env.env.declarations().len(), declarations_before);
    assert_eq!(env.env.next_global_id(), next_id_before);
    assert_eq!(env.env.trusted_base(), trusted_before);
}

fn assert_root_floor_collision_rejects_before_allocation(source: &str, name: &str) {
    let mut env = ElabEnv::new().expect("base environment");
    let canonical_id = env.globals[name];
    let declarations_before = env.env.declarations().len();
    let next_id_before = env.env.next_global_id();
    let trusted_before = env.env.trusted_base();
    let error = env
        .elaborate_file(source)
        .expect_err("a root-only same-spelling floor binding must reject before allocation");
    assert!(
        matches!(error, ElabError::AmbiguousReference { name: ref rejected, .. } if rejected == name),
        "root-only route must reject at {name}, got {error:?}"
    );
    assert_eq!(env.globals[name], canonical_id);
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

/// Promise class: durable invariant. The public wrapper preserves declaration
/// namespace effects in the production parser/elaborator path.
///
/// **MEASURED:** separate public parent and constructor declarations reject at
/// their retained floor spelling with every canonical floor binding,
/// declaration count, allocator position, and trust unchanged. An all-renamed
/// public data declaration allocates exactly one local family and constructor
/// with kernel-recorded local parentage. **CLAIMED:** the classifier-boundary
/// wrapper law is realized before allocation for both safety-relevant binding
/// effect shapes. **THE GAP:** the boundary law's one-level constructible-leaf
/// population is a manually maintained unit-test table and must grow with a new
/// non-`Pub` `Decl` leaf.
#[test]
fn public_wrapper_rejects_floor_parent_before_allocation() {
    assert_public_floor_collision_rejects_before_allocation(
        "public-parent",
        "pub def Nat = Bool",
        "Nat",
    );
}

/// A public data declaration reaches constructor-name collision enumeration
/// independently of the family-name route above.
#[test]
fn public_wrapper_rejects_floor_constructor_before_allocation() {
    assert_public_floor_collision_rejects_before_allocation(
        "public-constructor",
        "pub data LocalNat = Zero",
        "Zero",
    );
}

/// An all-renamed public data declaration is the reaching positive: the same
/// production form remains lawful and records exact local parentage.
#[test]
fn public_wrapper_all_renamed_data_preserves_local_parentage() {
    let mut env = ElabEnv::new().expect("base environment");
    let canonical_ids = floor_ids(&env);
    let declarations_before = env.env.declarations().len();
    let next_id_before = env.env.next_global_id();
    let trusted_before = env.env.trusted_base();
    env.elaborate_file("pub data LocalNat = LocalZero")
        .expect("an all-renamed public family must elaborate");

    let parent = env.globals["LocalNat"];
    let constructor = env
        .env
        .inductive(parent)
        .expect("public renamed family must be kernel-recorded")
        .constructors[0]
        .id;
    assert!(!canonical_ids.contains(&parent));
    assert!(!canonical_ids.contains(&constructor));
    let (recorded_parent, _) = env
        .env
        .constructor(constructor)
        .expect("public renamed constructor must be kernel-recorded");
    assert_eq!(recorded_parent.id, parent);
    assert_eq!(env.env.declarations().len(), declarations_before + 1);
    assert_eq!(env.env.next_global_id().0, next_id_before.0 + 2);
    assert_eq!(env.env.trusted_base(), trusted_before);
}

struct BindingRouteCase {
    label: &'static str,
    collision_template: &'static str,
    renamed_source: &'static str,
    renamed_identity: &'static str,
}

const BINDING_ROUTE_CASES: &[BindingRouteCase] = &[
    BindingRouteCase {
        label: "view",
        collision_template: "const {name} : Bool = True",
        renamed_source: "const LocalView : Bool = True",
        renamed_identity: "Entry.LocalView",
    },
    BindingRouteCase {
        label: "let",
        collision_template: "let {name} : Bool = True",
        renamed_source: "let LocalLet : Bool = True",
        renamed_identity: "Entry.LocalLet",
    },
    BindingRouteCase {
        label: "prove",
        collision_template: "prove {name} : Bool",
        renamed_source: "prove LocalProve : Bool",
        renamed_identity: "LocalProve",
    },
    BindingRouteCase {
        label: "prop",
        collision_template: "prop {name} : Omega where { intro : {name} }",
        renamed_source: "prop LocalProp : Omega where { intro : LocalProp }",
        renamed_identity: "Entry.LocalProp",
    },
    BindingRouteCase {
        label: "theorem",
        collision_template: "prop RouteGoal : Omega where { route_intro : RouteGoal }\ntheorem {name} : RouteGoal = RouteGoal.route_intro",
        renamed_source: "prop RouteGoal : Omega where { route_intro : RouteGoal }\ntheorem LocalTheorem : RouteGoal = RouteGoal.route_intro",
        renamed_identity: "Entry.LocalTheorem",
    },
    BindingRouteCase {
        label: "axiom",
        collision_template: "prop AxiomGoal : Omega where { axiom_intro : AxiomGoal }\naxiom {name} : AxiomGoal",
        renamed_source: "prop AxiomGoal : Omega where { axiom_intro : AxiomGoal }\naxiom LocalAxiom : AxiomGoal",
        renamed_identity: "Entry.LocalAxiom",
    },
    BindingRouteCase {
        label: "law",
        collision_template: "law {name} (x) { field : Bool }",
        renamed_source: "law LocalLaw (x) { field : Bool }",
        renamed_identity: "LocalLaw",
    },
    BindingRouteCase {
        label: "type-alias",
        collision_template: "def {name} = Bool",
        renamed_source: "def LocalAlias = Bool",
        renamed_identity: "Entry.LocalAlias",
    },
    BindingRouteCase {
        label: "record",
        collision_template: "record {name} { field : Bool }",
        renamed_source: "record LocalRecord { field : Bool }",
        renamed_identity: "LocalRecord",
    },
    BindingRouteCase {
        label: "class",
        collision_template: "class {name} { field : Bool }",
        renamed_source: "class LocalClass { field : Bool }",
        renamed_identity: "LocalClass",
    },
    BindingRouteCase {
        label: "foreign",
        collision_template: "foreign {name} : Int = \"floor_probe\" \"libc.so\" pure",
        renamed_source: "foreign LocalForeign : Int = \"floor_probe\" \"libc.so\" pure",
        renamed_identity: "LocalForeign",
    },
    BindingRouteCase {
        label: "temporal",
        collision_template: "temporal {name} { always True }",
        renamed_source: "temporal LocalTemporal { always True }",
        renamed_identity: "LocalTemporal",
    },
];

/// Promise class: durable invariant. The exhaustive declaration namespace-
/// effect classifier guards every executable top-level binding producer,
/// independently of the separate module-qualification taxonomy.
///
/// **MEASURED:** every route rejects both a floor parent and a floor constructor
/// at the exact spelling without changing canonical identity, declarations,
/// allocator, or trust, while the same declaration form under a renamed binding
/// accepts and produces a distinct identity. **CLAIMED:** collision population
/// is closed over declaration binding producers, not a hand-picked syntax list.
/// **THE GAP:** data and explicit-data constructor sub-bindings are exercised by
/// the complete per-binding matrix above; the root-only space route is exercised
/// separately because strict roots intentionally reject its nested placement.
#[test]
fn every_executable_top_level_binding_route_is_floor_immutable() {
    for route in BINDING_ROUTE_CASES {
        for name in ["Nat", "Zero"] {
            assert_floor_collision_rejects_before_allocation(
                &format!("{}-{name}", route.label),
                &route.collision_template.replace("{name}", name),
                name,
            );
        }

        let root = FixtureRoot::new(&format!("{}-renamed", route.label));
        root.write(route.renamed_source);
        let mut env = ElabEnv::new().expect("base environment");
        let canonical_ids = floor_ids(&env);
        env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry")
            .unwrap_or_else(|error| {
                panic!(
                    "renamed {} binding route must elaborate: {error}",
                    route.label
                )
            });
        let local = env.globals[route.renamed_identity];
        assert!(
            !canonical_ids.contains(&local),
            "renamed {} route reused a floor identity",
            route.label
        );
    }

    for name in ["Nat", "Zero"] {
        assert_root_floor_collision_rejects_before_allocation(
            &format!("space {name} {{ mut cell : Int = 0 }}"),
            name,
        );
    }
    let mut space_env = ElabEnv::new().expect("base environment");
    let canonical_ids = floor_ids(&space_env);
    space_env
        .elaborate_file("space LocalSpace { mut cell : Int = 0 }")
        .expect("renamed root-only space must elaborate");
    let local_space = space_env.globals["LocalSpace"];
    assert!(!canonical_ids.contains(&local_space));
}

/// Promise class: durable invariant. Selective imports consult the complete
/// floor binding set, while per-item renaming and qualification remain lawful.
///
/// **MEASURED:** importing an all-renamed local constructor under `Zero`
/// rejects without allocation after its provider is loaded; re-importing the
/// canonical `Nil` under the same spelling is idempotent and preserves its exact
/// identity; per-item renaming and qualified provider access both elaborate.
/// **CLAIMED:** import collision checking shares the exact-derived floor
/// binding authority with local prebinding and distinguishes identity from
/// spelling. **THE GAP:** representative constructors reach both identity
/// orientations; the matrix above proves every specified parent and constructor
/// belongs to the shared set.
#[test]
fn selective_import_floor_collision_rejects_but_renamed_and_qualified_access_accept() {
    let same = FixtureRoot::new("same-identity-import");
    same.write_named("Provider", "export Nil");
    same.write("import Provider (Nil)\nconst witness : List Bool = Nil Bool");
    let mut same_env = ElabEnv::new().expect("base environment");
    let canonical_nil = same_env.globals["Nil"];
    same_env
        .elaborate_module_from_roots_strict(&[same.0.clone()], "Provider")
        .expect("provider must re-export the canonical Nil identity");
    let declarations_before_same_import = same_env.env.declarations().len();
    let next_id_before_same_import = same_env.env.next_global_id();
    let trusted_before_same_import = same_env.env.trusted_base();
    let same_ids = same_env
        .elaborate_module_from_roots_strict(&[same.0.clone()], "Entry")
        .expect("a second path to the same canonical Nil identity must be idempotent");
    assert_eq!(same_env.globals["Nil"], canonical_nil);
    let same_witness = *same_ids.last().expect("same-identity witness");
    let (_, same_body) = same_env
        .env
        .transparent_body(same_witness)
        .expect("same-identity witness body");
    assert!(mentions_global(&same_body, canonical_nil));
    assert_eq!(
        same_env.env.declarations().len(),
        declarations_before_same_import + 1
    );
    assert_eq!(
        same_env.env.next_global_id().0,
        next_id_before_same_import.0 + 1
    );
    assert_eq!(same_env.env.trusted_base(), trusted_before_same_import);

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
