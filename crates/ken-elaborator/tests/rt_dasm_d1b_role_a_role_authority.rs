//! `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-a` — the role-authority discriminator.
//!
//! **What this pins, in two separate properties.** ⚠ Neither is the statement
//! "no role in either producer is selected by source spelling" — that one is
//! not a test's to make, and the section below says who makes it instead.
//!
//! 1. **Substitution resistance.** A package that declares its own constructors
//!    under the prelude's role spellings must not redirect any stored Runtime
//!    role onto them. The record must still carry the roles the *canonical
//!    prelude* `GlobalId`s denote.
//! 2. **Roster inventory.** Every role the canonical roster carries is either
//!    covered by property 1's fixture or is a constructor of an exact closed-
//!    floor parent and therefore unshadowable before allocation. This keeps the
//!    substitution control exhaustive over the shadowable roster as it changes.
//!
//! **The producer property is closed by the SIGNATURE, not by this file.** Both
//! `checked_host_spine_v1` and `checked_runtime_symbols_v1` take `&PreludeEnv`
//! and the stable-symbol map — never `&ElabEnv`. The mutable package namespace
//! is therefore *unnameable* inside either producer: `env.globals` is not in
//! scope, so source-name authority cannot be reintroduced by an ordinary edit,
//! only by widening a signature, which a reviewer sees and the compiler forces
//! through every call site.
//!
//! **An earlier revision of this file got that division wrong**, and the
//! Architect blocked it. It asserted equality between the roster and a
//! hand-written fixture and called the result "inventory completeness" — but
//! both producers still took `&ElabEnv`, so a new direct
//! `env.globals.get("SomethingNew")` would have bypassed both enumerations and
//! left every test green. A test that enumerates today's roles establishes that
//! the roles *reviewed* are sound; it cannot establish that no other role can be
//! added. The repair was to narrow the boundary so the bypass does not exist,
//! and to stop this file claiming a property it was never able to hold.
//!
//! **Why bare-name containment is not the assertion.** The rejected candidate
//! `aade3c2f` searched the serialized record for the bare strings `"Nil"`,
//! `"Cons"`, … . Those strings are present whether the stored role is the
//! prelude's constructor or a package constructor of the same name, so the
//! control stayed green under exactly the substitution it was meant to exclude.
//! Every assertion below names a **fully qualified** symbol — the parent chain
//! is the identity, and the parent is what a substitution changes.
//!
//! **MEASURED (reframed by `LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD`).** A
//! package can no longer DECLARE a constructor under a prelude/runtime role's
//! spelling: the flat constructor namespace rejects the collision at declaration
//! time (`DuplicateConstructorSpelling`, naming both sites). The former
//! measurement — declare every shadow, then check that none redirected a stored
//! role — is retired because its fixture is now unrepresentable; the first test
//! below asserts the rejection directly (on a Runtime-role constructor spelling).
//! The roster-inventory relation in the second test still holds and is measured.
//!
//! **CLAIMED.** No package declaration can capture a Runtime role. For
//! constructor roles this is now enforced by declaration-time rejection —
//! strictly stronger than the old "declared but not redirected". For op-defs and
//! family formers, which the guard does not reject (a constructor shadowing a
//! def, or a `data` former shadowing a former, are a different collision class),
//! non-capture stays closed by the producers' `&PreludeEnv` signature, which puts
//! the package namespace out of scope — never measured from a test, by design.
//!
//! **THE GAP.** Unchanged in kind: the inventory relation compares the fixture
//! against the roster, not against the producers, so a role that leaves the
//! roster into an already-canonical `PreludeEnv` field is sound (captured at
//! registration) but invisible here. Roles whose ids were already canonical
//! (`Zero`/`Suc`, the private operations, the resource ids) never passed through
//! name lookup. Closed-floor bindings (`Nil`/`Cons`, `Some`, `Ok`/`Err`,
//! `True`/`False`, `Buffer`) are outside the fixture: module prebinding rejects
//! those spellings before package allocation.

use std::collections::BTreeSet;

use ken_elaborator::checked_core::{StableSymbol, SymbolNamespace};
use ken_elaborator::compiler_driver::{
    compile_ken_package_sources, CompilerDriverError, CompilerManifest, CompilerSource,
    CompilerTargetKind, TargetSelector,
};
use ken_elaborator::modules::PRELUDE_FLOOR_NAMES;
use ken_elaborator::prelude::CanonicalRuntimeRoles;
use ken_elaborator::{ElabEnv, ElabError};

const PACKAGE: &str = "d1b_role_a_shadow_pkg";

/// A package that declares colliding names across every class of role the two
/// producers resolve: the six former entry-plan roles, the ITree/coproduct
/// spine, `Unit`, `Instant`, `ReadResult`, IO errors, non-floor resource
/// errors, progress constructors, file-error and file-operation discriminants,
/// and the public
/// host operations.
///
/// Every one of these spellings was, before this repair, a live
/// `env.globals.get(name)` in `checked_host_spine_v1` or
/// `checked_runtime_symbols_v1`.
const SHADOWING_SOURCE: &str = r#"data FSOp = ShadowFsOp
data ConsoleOp = ShadowConsoleOp
data ClockOp = ShadowClockOp
data EntropyOp = ShadowEntropyOp
data Cap = ShadowCap
data ShadowPlan = MkProd | MkProcessInput | Success | Failure
data ShadowSpine = Ret | Vis | InL | InR
data ShadowUnit = MkUnit
data ShadowInstant = MkInstant
data ShadowReadResult = Chunk | Eof
data ShadowIoErrors = NotFound | PermissionDenied | CapabilityDenied | BrokenPipe | Interrupted | AlreadyExists | InvalidInput | IsDirectory | NotDirectory | NotEmpty | Unsupported | Revoked | Other
data ShadowResource = ResourceKindMismatch | BufferLimit | AllocationFailed | InvalidOffset | InvalidBounds | NoProgress | MappingLimit
data ShadowProgress = ReadSome | ReadEof | Wrote
data ShadowFileOps =
  MkFileError | OpReadFile | OpWriteFile | OpChangeMode | OpAppendFile | OpMetadata | OpRename |
  OpReadDirectory | OpCreateDirectory | OpRemoveFile | OpRemoveDirectory | OpSeek | OpSetLength | OpSync |
  OpGetInheritance | OpSetInheritance | OpDuplicate
data ShadowMetadata = MkFileMetadata | MkDirEntry | KFile | KDirectory | KSymlink | KOther
data ShadowOps = Read | Write | Flush | IsTerminal | WallNow | MonotonicNow | SleepUntil | RandomBytes | ReadFile | WriteFile | AppendFile | Metadata | ReadDirectory | CreateDirectory | RemoveFile | RemoveDirectory | Rename | ChangeMode

const two : Nat = Suc (Suc Zero)
"#;

/// Every `(shadow family, constructor)` pair the source above declares.
///
/// The table is the inventory: one row per role spelling both producers used to
/// resolve by name. It is driven exhaustively, so adding a name-resolved role to
/// a producer without adding it here cannot quietly escape the control — the
/// role's spelling is shadowed and its redirection would be observed.
const SHADOWED_ROLES: &[(&str, &str)] = &[
    ("ShadowPlan", "MkProd"),
    ("ShadowPlan", "MkProcessInput"),
    ("ShadowPlan", "Success"),
    ("ShadowPlan", "Failure"),
    ("ShadowSpine", "Ret"),
    ("ShadowSpine", "Vis"),
    ("ShadowSpine", "InL"),
    ("ShadowSpine", "InR"),
    ("ShadowUnit", "MkUnit"),
    ("ShadowInstant", "MkInstant"),
    ("ShadowReadResult", "Chunk"),
    ("ShadowReadResult", "Eof"),
    ("ShadowIoErrors", "NotFound"),
    ("ShadowIoErrors", "PermissionDenied"),
    ("ShadowIoErrors", "CapabilityDenied"),
    ("ShadowIoErrors", "BrokenPipe"),
    ("ShadowIoErrors", "Interrupted"),
    ("ShadowIoErrors", "AlreadyExists"),
    ("ShadowIoErrors", "InvalidInput"),
    ("ShadowIoErrors", "IsDirectory"),
    ("ShadowIoErrors", "NotDirectory"),
    ("ShadowIoErrors", "NotEmpty"),
    ("ShadowIoErrors", "Unsupported"),
    ("ShadowIoErrors", "Revoked"),
    ("ShadowIoErrors", "Other"),
    ("ShadowResource", "ResourceKindMismatch"),
    ("ShadowResource", "BufferLimit"),
    ("ShadowResource", "AllocationFailed"),
    ("ShadowResource", "InvalidOffset"),
    ("ShadowResource", "InvalidBounds"),
    ("ShadowResource", "NoProgress"),
    ("ShadowResource", "MappingLimit"),
    ("ShadowProgress", "ReadSome"),
    ("ShadowProgress", "ReadEof"),
    ("ShadowProgress", "Wrote"),
    ("ShadowFileOps", "MkFileError"),
    ("ShadowFileOps", "OpReadFile"),
    ("ShadowFileOps", "OpWriteFile"),
    ("ShadowFileOps", "OpChangeMode"),
    ("ShadowFileOps", "OpAppendFile"),
    ("ShadowFileOps", "OpMetadata"),
    ("ShadowFileOps", "OpRename"),
    ("ShadowFileOps", "OpReadDirectory"),
    ("ShadowFileOps", "OpCreateDirectory"),
    ("ShadowFileOps", "OpRemoveFile"),
    ("ShadowFileOps", "OpRemoveDirectory"),
    ("ShadowFileOps", "OpSeek"),
    ("ShadowFileOps", "OpSetLength"),
    ("ShadowFileOps", "OpSync"),
    ("ShadowFileOps", "OpGetInheritance"),
    ("ShadowFileOps", "OpSetInheritance"),
    ("ShadowFileOps", "OpDuplicate"),
    ("ShadowMetadata", "MkFileMetadata"),
    ("ShadowMetadata", "MkDirEntry"),
    ("ShadowMetadata", "KFile"),
    ("ShadowMetadata", "KDirectory"),
    ("ShadowMetadata", "KSymlink"),
    ("ShadowMetadata", "KOther"),
    ("ShadowOps", "Read"),
    ("ShadowOps", "Write"),
    ("ShadowOps", "Flush"),
    ("ShadowOps", "IsTerminal"),
    ("ShadowOps", "WallNow"),
    ("ShadowOps", "MonotonicNow"),
    ("ShadowOps", "SleepUntil"),
    ("ShadowOps", "RandomBytes"),
    ("ShadowOps", "ReadFile"),
    ("ShadowOps", "WriteFile"),
    ("ShadowOps", "AppendFile"),
    ("ShadowOps", "Metadata"),
    ("ShadowOps", "ReadDirectory"),
    ("ShadowOps", "CreateDirectory"),
    ("ShadowOps", "RemoveFile"),
    ("ShadowOps", "RemoveDirectory"),
    ("ShadowOps", "Rename"),
    ("ShadowOps", "ChangeMode"),
];

fn compile_shadowing_package() -> Result<(), CompilerDriverError> {
    compile_ken_package_sources(
        &CompilerManifest::new(PACKAGE, Vec::new()),
        vec![CompilerSource::new("src/main.ken", SHADOWING_SOURCE)],
        TargetSelector::StableSymbol {
            package_identity: StableSymbol::new(
                SymbolNamespace::Module,
                vec![PACKAGE.to_string()],
            ),
            symbol: StableSymbol::new(
                SymbolNamespace::Declaration,
                vec![PACKAGE.to_string(), "two".to_string()],
            ),
            kind: CompilerTargetKind::Executable,
        },
    )
    .map(|_| ())
}

/// The five **family** roles, which are declarations rather than constructors.
///
/// They are a separate path class: `checked_host_spine_v1` resolves them into
/// `fs_family`/`console_family`/`clock_family`/`entropy_family`/`capability`,
/// and a shadow is a top-level `data` of the same name rather than a colliding
/// constructor. Kept in its own table because the symbol namespace differs.
const SHADOWED_FAMILIES: &[&str] = &["FSOp", "ConsoleOp", "ClockOp", "EntropyOp", "Cap"];

#[test]
fn d1b_role_a_package_cannot_declare_a_runtime_role_constructor_spelling() {
    // REFRAMED to the new invariant (LANG-CONSTRUCTOR-NAMESPACE-SHADOWING-GUARD).
    //
    // This test formerly declared the shadowing package (`emit_shadowing_package`
    // expecting it to compile — "colliding declarations are lawful Ken") and
    // MEASURED that no shadow constructor redirected a stored Runtime role. That
    // measurement was belt-and-suspenders: this file's own thesis is that the
    // producer property is "closed by the SIGNATURE, not by this file" (both
    // producers take `&PreludeEnv`, so the package namespace is unnameable in
    // them). The guard now adds an even earlier gate: a package can no longer
    // even DECLARE a constructor under a prelude/runtime role's spelling — the
    // flat constructor namespace rejects it at declaration time. Capture-by-
    // constructor-spelling is therefore unrepresentable, strictly stronger than
    // "declared but not redirected". So the fixture no longer compiles, and the
    // faithful assertion is the rejection itself.
    //
    // (Op-role and family-name shadows are NOT rejected by the guard — a
    // constructor shadowing a def, or a `data` former shadowing a former, are a
    // different collision class — but their non-capture remains closed by the
    // `&PreludeEnv` signature exactly as the module doc describes; nothing here
    // relied on this test to establish it.)
    match compile_shadowing_package() {
        Err(CompilerDriverError::Elaboration(ElabError::DuplicateConstructorSpelling {
            name,
            first_span,
            second_span,
        })) => {
            // The rejected spelling is one of the Runtime-role constructor
            // spellings the fixture shadows — proving the rejection is a role
            // capture the guard prevents, not an incidental collision — and both
            // declaration sites are named with real spans.
            assert!(
                SHADOWED_ROLES
                    .iter()
                    .any(|(_, constructor)| *constructor == name),
                "the rejected constructor spelling {name:?} must be one of the shadowed \
                 Runtime-role spellings"
            );
            assert!(
                first_span.end > first_span.start && second_span.end > second_span.start,
                "both declaration sites must be named with real spans: \
                 first={first_span:?} second={second_span:?}"
            );
        }
        other => panic!(
            "expected the shadowing package to be REJECTED with a duplicate-constructor-spelling \
             diagnostic (a runtime-role constructor spelling can no longer be declared by a \
             package), got {other:?}"
        ),
    }
}

/// Roster inventory, stated as a **relation between two artifacts**.
///
/// The test above proves a package cannot declare a Runtime-role constructor
/// spelling (the guard rejects it). That inventory stays meaningful over the
/// roster only if the fixture's table tracks every role the roster carries —
/// and the table is hand-written, so on its own it is a snapshot that a newly
/// added role would silently escape.
///
/// This keeps the two in step: the roster's spelling table is compared against
/// the union of the fixture's shadow inventory and constructors whose recorded
/// parent is one of the exact nine floor parents. Add a shadowable role to
/// `canonical_runtime_roles!` without shadowing it and this reds, naming it,
/// before the substitution control can go quietly partial.
///
/// ⚠ **What this is NOT.** It is not a proof that the producers resolve nothing
/// by spelling. It compares the fixture against the roster, and neither side is
/// the producers. That property is closed by the producers' `&PreludeEnv`
/// signature, which puts the package namespace out of scope — see this file's
/// module documentation. An earlier revision of this test claimed the producer
/// property and was blocked for it.
///
/// ⚠ **Promise class: durable invariant.** It asserts set equality between two
/// enumerations, not a count. Adding a role keeps it green once the role is
/// shadowed, and removing one keeps it green once the row goes; only a
/// divergence between the two reds it. There is no frozen number to maintain.
#[test]
fn d1b_role_a_every_canonical_role_is_covered_by_shadowing_or_floor_immutability() {
    let mut shadowed: Vec<&str> = SHADOWED_ROLES
        .iter()
        .map(|(_, constructor)| *constructor)
        .chain(SHADOWED_FAMILIES.iter().copied())
        .collect();
    shadowed.sort_unstable();

    // Neither side may be empty, or the two filters below would both find
    // nothing and this would pass while comparing nothing to nothing.
    assert!(!shadowed.is_empty(), "the fixture inventory is empty");
    assert!(
        !CanonicalRuntimeRoles::spellings().is_empty(),
        "the canonical roster is empty"
    );

    let roster: Vec<&str> = CanonicalRuntimeRoles::spellings()
        .iter()
        .map(|(_, spelling)| *spelling)
        .collect();
    let env = ElabEnv::new().expect("base environment");
    let floor_parents = PRELUDE_FLOOR_NAMES
        .iter()
        .map(|name| env.globals[*name])
        .collect::<BTreeSet<_>>();
    let floor_protected = roster
        .iter()
        .copied()
        .filter(|spelling| {
            env.globals.get(*spelling).is_some_and(|id| {
                env.env
                    .constructor(*id)
                    .is_some_and(|(parent, _)| floor_parents.contains(&parent.id))
            })
        })
        .collect::<Vec<_>>();
    assert!(
        !floor_protected.is_empty(),
        "the roster must exercise the exact-parent floor-protection arm"
    );

    let uncovered: Vec<&(&str, &str)> = CanonicalRuntimeRoles::spellings()
        .iter()
        .filter(|(_, spelling)| !shadowed.contains(spelling) && !floor_protected.contains(spelling))
        .collect();
    assert!(
        uncovered.is_empty(),
        "these canonical roles are neither shadowed by the fixture nor protected by exact floor \
         constructor parentage: {uncovered:?}\n\
         Add a shadowable spelling to SHADOWED_ROLES/SHADOWED_FAMILIES, or make the intended \
         structural protection explicit."
    );

    let stale: Vec<&&str> = shadowed
        .iter()
        .filter(|spelling| !roster.contains(spelling))
        .collect();
    assert!(
        stale.is_empty(),
        "the fixture shadows spellings that are no longer canonical roles: {stale:?}\n\
         A stale row is not harmless -- it makes the inventory look larger than the authority it \
         is meant to cover."
    );
}
