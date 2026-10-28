//! Checked entrypoint-ABI floor and scoped-constructor admission (30 §4; 33 §3.3).

use std::collections::BTreeSet;
use std::fs;

use ken_elaborator::{ElabEnv, ElabError};
use ken_kernel::{GlobalId, Term};

const ABI_TYPES: [&str; 5] = ["ExitCode", "HostIO", "ProcessInput", "Prod", "ProgramCaps"];

fn strict(source: &str) -> Result<ElabEnv, ElabError> {
    let root = tempfile::tempdir().expect("entrypoint fixture directory");
    fs::write(root.path().join("Entry.ken"), source).expect("entrypoint fixture");
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")?;
    Ok(env)
}

fn strict_error(source: &str) -> ElabError {
    let root = tempfile::tempdir().expect("entrypoint fixture directory");
    fs::write(root.path().join("Entry.ken"), source).expect("entrypoint fixture");
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_module_from_roots_strict(&[root.path().to_path_buf()], "Entry")
        .expect_err("source must be rejected")
}

fn legacy(source: &str, roots: bool) -> Result<ElabEnv, ElabError> {
    let mut env = ElabEnv::new().expect("checked prelude");
    if roots {
        let root = tempfile::tempdir().expect("legacy fixture directory");
        fs::write(root.path().join("Entry.ken"), source).expect("legacy fixture");
        env.elaborate_module_from_roots_v1(&[root.path().to_path_buf()], "Entry")?;
    } else {
        env.elaborate_file_v1(source)?;
    }
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

/// Promise class: normative compatibility vector. MEASURED: each of the five
/// source-required ABI names resolves via strict roots to its pre-source ID,
/// and any constructor in a witness body has the original checked parent.
/// CLAIMED: floor admission reuses the exact entrypoint identities without
/// allocating a replacement or extending trust. THE GAP: source references
/// alone do not prove the runner uses those IDs; its role-admission tests do.
#[test]
fn entrypoint_types_and_constructors_resolve_to_checked_ids() {
    let base = ElabEnv::new().expect("checked prelude");
    let base_trust: BTreeSet<_> = base.env.trusted_base().into_iter().collect();
    let cases: [(&str, &str, Option<&str>); 5] = [
        (
            "ExitCode",
            "const witness : ExitCode = ExitCode.Success",
            Some("ExitCode.Success"),
        ),
        (
            "HostIO",
            "fn witness (x : HostIO APartial ExitCode) : HostIO APartial ExitCode = x",
            None,
        ),
        (
            "ProcessInput",
            "fn witness (x : ProcessInput) : ProcessInput = match x { MkProcessInput args env cwd ↦ MkProcessInput args env cwd }",
            Some("MkProcessInput"),
        ),
        (
            "Prod",
            "const witness : Prod Bool Bool = MkProd Bool Bool True False",
            Some("MkProd"),
        ),
        (
            "ProgramCaps",
            "fn witness (x : ProgramCaps APartial) : ProgramCaps APartial = match x { ProgramCaps.MkProgramCaps c ↦ ProgramCaps.MkProgramCaps APartial c }",
            Some("ProgramCaps.MkProgramCaps"),
        ),
    ];
    for (name, source, constructor) in cases {
        let family = base.globals[name];
        assert!(!base_trust.contains(&family));
        let env = strict(source).unwrap_or_else(|error| panic!("strict {name}: {error:?}"));
        assert_eq!(env.globals[name], family);
        let witness = env.globals["Entry.witness"];
        let (_, ty) = env.env.const_type(witness).expect("checked witness type");
        let (_, body) = env
            .env
            .transparent_body(witness)
            .expect("checked witness body");
        assert!(contains_id(&ty, family), "{name}: type must use exact ID");
        if let Some(constructor) = constructor {
            let id = env.globals[constructor];
            assert!(contains_id(&body, id), "{name}: body uses {constructor}");
            let (parent, _) = env.env.constructor(id).expect("checked constructor parent");
            assert_eq!(parent.id, family, "{constructor}: exact parent");
            assert!(!base_trust.contains(&id));
        } else {
            assert!(
                env.env.inductive(family).is_none(),
                "HostIO has no constructor"
            );
        }
        assert_eq!(
            env.env.trusted_base(),
            base.env.trusted_base(),
            "{name}: trust"
        );
    }
    assert_ne!(base.globals["Pair"], base.globals["Prod"]);
    assert!(base.env.inductive(base.globals["Pair"]).is_none());
    assert!(base.env.inductive(base.globals["Prod"]).is_some());
}

/// Promise class: durable invariant. MEASURED: the exact checked constructor
/// IDs of scoped families resolve in qualified form, while bare uses reject
/// and legal local same-spelling data constructors can still be introduced.
/// CLAIMED: reservation is per type rather than an accidental bare alias.
/// THE GAP: this does not assert the old legacy API's spelling policy.
#[test]
fn scoped_entrypoint_constructors_have_only_qualified_strict_paths() {
    let base = ElabEnv::new().expect("checked prelude");
    for (name, source) in [
        ("ExitCode.Success", "const witness : ExitCode = ExitCode.Success"),
        ("ExitCode.Failure", "const witness : ExitCode = ExitCode.Failure 1"),
        (
            "ProgramCaps.MkProgramCaps",
            "fn witness (x : ProgramCaps APartial) : ProgramCaps APartial = match x { ProgramCaps.MkProgramCaps c ↦ ProgramCaps.MkProgramCaps APartial c }",
        ),
    ] {
        let env = strict(source).unwrap_or_else(|error| panic!("strict {name}: {error:?}"));
        let (_, body) = env
            .env
            .transparent_body(env.globals["Entry.witness"])
            .expect("checked constructor witness");
        assert!(
            contains_id(&body, base.globals[name]),
            "{name}: strict source uses the exact pre-source constructor ID"
        );
    }
    for (name, source) in [
        ("Success", "const witness : ExitCode = Success"),
        ("Failure", "const witness : ExitCode = Failure 1"),
        (
            "MkProgramCaps",
            "fn witness (x : ProgramCaps APartial) : ProgramCaps APartial = match x { MkProgramCaps cap ↦ x }",
        ),
    ] {
        let error = strict_error(source);
        assert!(
            matches!(error, ElabError::UnboundName { name: ref rejected, .. } if rejected == name),
            "bare {name} refused at its spelling, got {error:?}"
        );
    }
    strict("data Local = Success | Failure | MkProgramCaps")
        .expect("scoped ABI constructor leaves the bare source names available");
}

/// Promise class: durable invariant. MEASURED: both legacy entry paths admit
/// the qualified checked constructor IDs and reject the bare spellings. The
/// source, not the flat globals map, is the subject of both observations.
/// CLAIMED: legacy/shipping routes cannot bypass the per-type scope choice.
/// THE GAP: two API entry routes are covered; the CI consumer sweep covers
/// their existing checked Ken fixtures.
#[test]
fn legacy_entrypoints_preserve_qualified_only_constructor_scope() {
    for roots in [false, true] {
        for (name, source) in [
            ("ExitCode.Success", "const witness : ExitCode = ExitCode.Success"),
            ("ExitCode.Failure", "const witness : ExitCode = ExitCode.Failure 1"),
            (
                "ProgramCaps.MkProgramCaps",
                "fn witness (x : ProgramCaps APartial) : ProgramCaps APartial = match x { ProgramCaps.MkProgramCaps c ↦ ProgramCaps.MkProgramCaps APartial c }",
            ),
        ] {
            let env = legacy(source, roots)
                .unwrap_or_else(|error| panic!("legacy roots={roots} {name}: {error:?}"));
            let expected = env.globals[name];
            let witness = if roots { "Entry.witness" } else { "witness" };
            let (_, body) = env.env.transparent_body(env.globals[witness]).unwrap();
            assert!(contains_id(&body, expected), "{name}: exact checked ID");
        }
        for (name, source) in [
            ("Success", "const witness : ExitCode = Success"),
            ("Failure", "const witness : ExitCode = Failure 1"),
            (
                "MkProgramCaps",
                "fn witness (x : ProgramCaps APartial) : ProgramCaps APartial = match x { MkProgramCaps c ↦ x }",
            ),
        ] {
            let error = legacy(source, roots).err().expect("bare name refused");
            assert!(
                matches!(error, ElabError::UnresolvedCon { name: ref rejected, .. } if rejected == name),
                "legacy roots={roots}, {name}: {error:?}"
            );
        }
    }
}

/// Promise class: durable invariant. MEASURED: source collisions with every
/// newly reserved floor type in type and constructor positions, and both
/// unscoped constructors, refuse before allocation. CLAIMED: the floor is a
/// protected exact-ID namespace. THE GAP: these named heads do not enumerate
/// all future syntax variants.
#[test]
fn entrypoint_floor_names_refuse_same_spelling_source_declarations() {
    for name in ABI_TYPES {
        let source = format!("data {name} = LocalCtor");
        let error = strict_error(&source);
        assert!(
            matches!(error, ElabError::AmbiguousReference { name: ref rejected, .. } if rejected == name),
            "{name}: {error:?}"
        );
    }
    for name in ABI_TYPES {
        let source = format!("data LocalHolder = {name}");
        let error = strict_error(&source);
        assert!(
            matches!(error, ElabError::AmbiguousReference { name: ref rejected, .. } if rejected == name),
            "constructor position {name}: {error:?}"
        );
    }
    for (name, source) in [
        ("MkProcessInput", "data Local = MkProcessInput"),
        ("MkProd", "data Local = MkProd"),
    ] {
        let error = strict_error(source);
        assert!(
            matches!(error, ElabError::AmbiguousReference { name: ref rejected, .. } if rejected == name),
            "{name}: {error:?}"
        );
    }
}
