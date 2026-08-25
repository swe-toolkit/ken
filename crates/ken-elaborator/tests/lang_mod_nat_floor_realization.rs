//! Behavioral pins for `LANG-MOD-NAT-FLOOR-REALIZATION`.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ken_elaborator::modules::PRELUDE_FLOOR_NAMES;
use ken_elaborator::{ElabEnv, ElabError};

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
        source: "const witness : Auth = ANone",
    },
    FloorCase {
        name: "Bool",
        constructors: &["True", "False"],
        source: "const witness : Bool = True",
    },
    FloorCase {
        name: "Char",
        constructors: &[],
        source: "fn witness (x : Char) : Char = x",
    },
    FloorCase {
        name: "List",
        constructors: &["Nil", "Cons"],
        source: "const witness : List Bool = Nil Bool",
    },
    FloorCase {
        name: "Nat",
        constructors: &["Zero", "Suc"],
        source: "const witness : Nat = Zero",
    },
    FloorCase {
        name: "Option",
        constructors: &["None", "Some"],
        source: "const witness : Option Bool = None Bool",
    },
    FloorCase {
        name: "ResourceKind",
        constructors: &["FsHandle", "Buffer"],
        source: "const witness : ResourceKind = FsHandle",
    },
    FloorCase {
        name: "Result",
        constructors: &["Err", "Ok"],
        source: "const witness : Result Bool Bool = Err Bool Bool True",
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

/// Promise class: transition sentinel. The landed nine-name contract is
/// already backed by canonical, ordinary checked identities, while the current
/// strict resolver admits exactly the implemented three-name subset.
///
/// **MEASURED:** every landed family and constructor already has one canonical
/// `GlobalId` outside `trusted_base()`, strict roots accepts Bool/Char/List, and
/// rejects the other six at `UnboundName`. **CLAIMED:** D1 needs only to widen
/// the closed floor admission set and can reuse all existing identities without
/// minting. **THE GAP:** this D0 census does not itself flip strict admission;
/// the final D1 control replaces the three/six partition with per-name accepts.
#[test]
fn d0_census_finds_three_admitted_and_six_missing_canonical_families() {
    assert_eq!(PRELUDE_FLOOR_NAMES, ["Bool", "Char", "List"]);

    let mut admitted = BTreeSet::new();
    let mut missing = BTreeSet::new();
    for case in &FLOOR_CASES {
        let root = FixtureRoot::new(case.name);
        root.write(case.source);
        let mut env = ElabEnv::new().expect("base environment");
        assert_canonical_family(&env, case);
        let family_id = env.globals[case.name];
        let trusted_before = env.env.trusted_base();

        match env.elaborate_module_from_roots_strict(&[root.0.clone()], "Entry") {
            Ok(_) => {
                admitted.insert(case.name);
            }
            Err(ElabError::UnboundName { name, .. }) if name == case.name => {
                missing.insert(case.name);
            }
            Err(error) => panic!("{} produced an unexpected D0 result: {error}", case.name),
        }

        assert_eq!(
            env.globals[case.name], family_id,
            "strict probing must not replace the existing {} identity",
            case.name
        );
        assert_eq!(
            env.env.trusted_base(),
            trusted_before,
            "strict probing must add no trust for {}",
            case.name
        );
    }

    assert_eq!(admitted, ["Bool", "Char", "List"].into_iter().collect());
    assert_eq!(
        missing,
        [
            "Auth",
            "Nat",
            "Option",
            "ResourceKind",
            "Result",
            "Utf8Error",
        ]
        .into_iter()
        .collect()
    );
    assert_eq!(
        admitted.union(&missing).copied().collect::<BTreeSet<_>>(),
        LANDED_FLOOR_NAMES.into_iter().collect(),
        "the D0 partition must exhaust the landed closed floor"
    );
}
