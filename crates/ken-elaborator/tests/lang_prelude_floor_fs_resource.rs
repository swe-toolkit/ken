//! Exact FS/resource floor identities (30-taxonomy §4; 38-ffi-io §§1.3, 1.7).

use std::collections::BTreeSet;
use std::fs;

use ken_elaborator::modules::PRELUDE_FLOOR_NAMES;
use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

const ADMITTED: [&str; 6] = [
    "BufferSpan",
    "CreatePolicy",
    "FileError",
    "IOError",
    "ResourceError",
    "TransferCount",
];
const SCOPED: [&str; 3] = ["CreatePolicy", "IOError", "ResourceError"];
const PRIVATE: [&str; 2] = ["BufferSpan", "TransferCount"];
const MODES: [&str; 3] = ["strict-roots", "legacy-roots", "legacy-file"];

fn elaborate_into(env: &mut ElabEnv, source: &str, mode: &str) -> Result<(), ElabError> {
    match mode {
        "strict-roots" | "legacy-roots" => {
            let root = tempfile::tempdir().expect("FS floor fixture directory");
            fs::write(root.path().join("Entry.ken"), source).expect("FS floor fixture");
            if mode == "strict-roots" {
                env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")?;
            } else {
                env.elaborate_module_from_roots_v1(&[root.path().to_path_buf()], "Entry")?;
            }
        }
        "legacy-file" => {
            env.elaborate_file_v1(source)?;
        }
        other => panic!("unknown mode: {other}"),
    }
    Ok(())
}

fn elaborate(source: &str, mode: &str) -> Result<ElabEnv, ElabError> {
    let mut env = ElabEnv::new().expect("checked prelude");
    elaborate_into(&mut env, source, mode)?;
    Ok(env)
}

fn contains_id(term: &Term, target: GlobalId) -> bool {
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
            .any(|child| contains_id(child, target)),
    }
}

/// Promise class: normative compatibility vector. MEASURED: each admitted
/// type and its checked constructors retain their pre-source IDs and parents;
/// every public scoped constructor has exactly its qualified key, and the two
/// private families have neither bare nor qualified source keys. CLAIMED:
/// admission follows the six exact-ID witnesses without widening privacy.
/// THE GAP: an ID in the pre-source table is not itself a proof of source
/// resolution; the three-route checked-source row below exercises that seam.
#[test]
fn admitted_fs_families_retain_checked_parents_and_private_visibility() {
    let base = ElabEnv::new().expect("checked prelude");
    let trust: BTreeSet<_> = base.env.trusted_base().into_iter().collect();
    let admitted: BTreeSet<_> = ADMITTED.iter().copied().collect();
    let floor: BTreeSet<_> = PRELUDE_FLOOR_NAMES.iter().copied().collect();
    assert_eq!(
        floor.len(),
        PRELUDE_FLOOR_NAMES.len(),
        "no duplicate floor names"
    );
    assert!(
        admitted.is_subset(&floor),
        "all six exact families admitted"
    );
    assert!(
        !floor.contains("BufferWindow"),
        "unkeyed sibling is not reserved"
    );
    for family in ADMITTED {
        let parent = base.globals[family];
        assert!(!trust.contains(&parent), "{family}: checked, not trusted");
        let declaration = base.env.inductive(parent).expect("checked family");
        assert!(
            !declaration.constructors.is_empty(),
            "{family}: nonempty family"
        );
        for constructor in &declaration.constructors {
            let id = constructor.id;
            let (actual_parent, _) = base.env.constructor(id).expect("checked constructor");
            assert_eq!(actual_parent.id, parent, "{family}: checked parent");
            assert!(!trust.contains(&id), "{family}: constructor is checked");
            if family == "FileError" {
                assert_eq!(base.globals["MkFileError"], id);
                assert!(!base.globals.contains_key("FileError.MkFileError"));
            }
        }
    }
    for (family, names) in [
        (
            "IOError",
            &[
                "NotFound",
                "PermissionDenied",
                "CapabilityDenied",
                "BrokenPipe",
                "Interrupted",
                "AlreadyExists",
                "InvalidInput",
                "IsDirectory",
                "NotDirectory",
                "NotEmpty",
                "Unsupported",
                "Revoked",
                "Other",
            ] as &[&str],
        ),
        (
            "CreatePolicy",
            &["CreateNew", "CreateOrTruncate", "CreateOrKeep"],
        ),
        (
            "ResourceError",
            &[
                "ResourceHostIO",
                "Closed",
                "MalformedResource",
                "RightNotHeld",
                "ReleaseFailed",
                "ResourceKindMismatch",
                "BufferLimit",
                "AllocationFailed",
                "InvalidOffset",
                "InvalidBounds",
                "NoProgress",
                "MappingLimit",
            ],
        ),
    ] {
        let parent = base.globals[family];
        let constructors = &base.env.inductive(parent).unwrap().constructors;
        assert_eq!(
            constructors.len(),
            names.len(),
            "{family}: checked family inventory"
        );
        for (constructor, name) in constructors.iter().zip(names) {
            let qualified = format!("{family}.{name}");
            assert_eq!(
                base.globals[&qualified], constructor.id,
                "{qualified}: exact ID"
            );
            assert!(!base.globals.contains_key(*name), "{name}: no bare alias");
        }
    }
    for (family, name, captured) in [
        (
            "BufferSpan",
            "PrivateBufferSpan",
            base.prelude_env.private_buffer_span_id,
        ),
        (
            "TransferCount",
            "PrivateTransferCount",
            base.prelude_env.private_transfer_count_id,
        ),
    ] {
        let (actual_parent, _) = base.env.constructor(captured).expect("private constructor");
        assert_eq!(actual_parent.id, base.globals[family]);
        assert!(!base.globals.contains_key(name));
        assert!(!base.globals.contains_key(&format!("{family}.{name}")));
        assert!(!trust.contains(&captured));
    }
    for family in PRIVATE {
        assert!(admitted.contains(family));
    }
    for family in SCOPED {
        assert!(admitted.contains(family));
    }
}

/// Promise class: durable invariant. MEASURED: real strict and both legacy
/// routes resolve each of six type IDs, use checked constructors for three
/// public scoped cases, and preserve the exact trusted-base identity set.
/// CLAIMED: all admitted families are source-usable with unchanged trust.
/// THE GAP: these selected constructors do not alone pin every sibling's
/// qualified route; the family-wide checked-parent row above checks their keys.
#[test]
fn fs_resource_source_resolves_checked_types_and_qualified_cases() {
    let base = ElabEnv::new().expect("checked prelude");
    let trust: BTreeSet<_> = base.env.trusted_base().into_iter().collect();
    let source = "const io : IOError = IOError.NotFound\n\
                  const policy : CreatePolicy = CreatePolicy.CreateNew\n\
                  const error : ResourceError = ResourceError.Closed\n\
                  fn file_id (x : FileError) : FileError = x\n\
                  fn span_id (x : BufferSpan) : BufferSpan = x\n\
                  fn count_id (x : TransferCount) : TransferCount = x";
    for mode in MODES {
        let env = elaborate(source, mode).unwrap_or_else(|e| panic!("{mode}: {e:?}"));
        let prefix = if mode == "legacy-file" { "" } else { "Entry." };
        for (name, parent) in [
            ("io", "IOError"),
            ("policy", "CreatePolicy"),
            ("error", "ResourceError"),
            ("file_id", "FileError"),
            ("span_id", "BufferSpan"),
            ("count_id", "TransferCount"),
        ] {
            let id = env.globals[&format!("{prefix}{name}")];
            let (_, ty) = env.env.const_type(id).expect("checked source type");
            assert!(
                contains_id(&ty, base.globals[parent]),
                "{mode}: {name} parent"
            );
            assert_eq!(env.globals[parent], base.globals[parent]);
        }
        for (name, qualified) in [
            ("io", "IOError.NotFound"),
            ("policy", "CreatePolicy.CreateNew"),
            ("error", "ResourceError.Closed"),
        ] {
            let id = env.globals[&format!("{prefix}{name}")];
            let (_, body) = env.env.transparent_body(id).expect("checked source body");
            assert!(
                contains_id(&body, base.globals[qualified]),
                "{mode}: {qualified}"
            );
            assert_eq!(env.globals[qualified], base.globals[qualified]);
        }
        assert_eq!(
            env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
            trust
        );
    }
}

/// Promise class: durable invariant. MEASURED: bare causes, creation cases,
/// and resource failures fail with the route-specific name-resolution error,
/// while qualified checked-source counterparts pass in the row above.
/// CLAIMED: no bare alias survives scoping on any shipping route. THE GAP:
/// source-local same-spelling constructors are separately legal (last row).
#[test]
fn scoped_fs_constructors_refuse_bare_source_in_every_mode() {
    for mode in MODES {
        for (name, family, source) in [
            ("NotFound", "IOError", "const x : IOError = NotFound"),
            ("Other", "IOError", "const x : IOError = Other 3"),
            ("Unsupported", "IOError", "const x : IOError = Unsupported"),
            (
                "CreateNew",
                "CreatePolicy",
                "const x : CreatePolicy = CreateNew",
            ),
            (
                "Closed",
                "ResourceError",
                "const x : ResourceError = Closed",
            ),
        ] {
            let failure = match elaborate(source, mode) {
                Ok(_) => panic!("{mode} bare {family}.{name}: unexpectedly accepted"),
                Err(e) => e,
            };
            if mode == "strict-roots" {
                assert!(
                    matches!(failure, ElabError::UnboundName { name: ref rejected, .. } if rejected == name),
                    "{mode} bare {family}.{name}: {failure:?}"
                );
            } else {
                assert!(
                    matches!(failure, ElabError::UnresolvedCon { name: ref rejected, .. } if rejected == name),
                    "{mode} bare {family}.{name}: {failure:?}"
                );
            }
        }
    }
}

/// Promise class: durable invariant. MEASURED: same-name source data for each
/// admitted type fails before allocation or trust change. CLAIMED: the six
/// checked type IDs cannot be shadowed by a package declaration. THE GAP:
/// unkeyed BufferWindow is the independent opposite outcome below.
#[test]
fn admitted_fs_type_declarations_refuse_without_allocation() {
    let base = ElabEnv::new().expect("checked prelude");
    for mode in MODES {
        for name in ADMITTED {
            let mut env = ElabEnv::new().expect("checked prelude");
            let before_id = env.env.next_global_id();
            let before_declarations = env.env.declarations().len();
            let before_trust = env.env.trusted_base();
            let source = format!("data {name} = LocalCtor");
            let failure = elaborate_into(&mut env, &source, mode).expect_err("floor collision");
            assert!(
                matches!(failure, ElabError::AmbiguousReference { name: ref rejected, .. } if rejected == name),
                "{mode} {name}: {failure:?}"
            );
            assert_eq!(env.globals[name], base.globals[name]);
            assert_eq!(env.env.next_global_id(), before_id);
            assert_eq!(env.env.declarations().len(), before_declarations);
            assert_eq!(env.env.trusted_base(), before_trust);
        }
    }
}

/// Promise class: durable invariant. MEASURED: strict roots permit a
/// same-name data declaration for the unkeyed BufferWindow while refusing
/// an admitted sibling, BufferSpan, on the identical path. Legacy resolution
/// of BufferWindow still points to its pre-source ID. CLAIMED: buffer family
/// labels cannot reserve a sibling without its exact-ID witness. THE GAP:
/// this checks the two discriminating names, not every unkeyed package row.
#[test]
fn buffer_window_package_row_stays_unreserved_while_buffer_span_is_reserved() {
    let base = ElabEnv::new().expect("checked prelude");
    let accepted = elaborate("data BufferWindow = LocalWindow", "strict-roots")
        .expect("unkeyed package row remains unreserved");
    assert_eq!(
        accepted.globals["BufferWindow"],
        base.globals["BufferWindow"]
    );
    let local_id = accepted.globals["Entry.BufferWindow"];
    assert_ne!(local_id, base.globals["BufferWindow"]);
    assert!(accepted.env.inductive(local_id).is_some());
    assert!(!PRELUDE_FLOOR_NAMES.contains(&"BufferWindow"));
    let failure = match elaborate("data BufferSpan = LocalWindow", "strict-roots") {
        Err(failure) => failure,
        Ok(_) => panic!("admitted sibling must be reserved"),
    };
    assert!(
        matches!(failure, ElabError::AmbiguousReference { ref name, .. } if name == "BufferSpan"),
        "same-name admitted sibling must refuse: {failure:?}"
    );
    for mode in ["legacy-roots", "legacy-file"] {
        let env = elaborate("fn window_id (x : BufferWindow) : BufferWindow = x", mode)
            .unwrap_or_else(|e| panic!("{mode}: legacy window resolution: {e:?}"));
        assert_eq!(env.globals["BufferWindow"], base.globals["BufferWindow"]);
    }
    for mode in MODES {
        elaborate(
            "data Local = NotFound | Other | Unsupported | Closed | CreateNew",
            mode,
        )
        .unwrap_or_else(|e| panic!("{mode}: lawful local spellings: {e:?}"));
    }
}
