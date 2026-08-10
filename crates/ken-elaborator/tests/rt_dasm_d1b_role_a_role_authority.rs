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
//! 2. **Roster inventory.** Every role the canonical roster carries is covered
//!    by property 1's fixture, in both directions. This keeps the substitution
//!    control exhaustive over the roster as the roster changes; it is a claim
//!    about the fixture's coverage, not about the producers.
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
//! **MEASURED.** With all sixty shadowable role spellings and all five family
//! spellings declared by a package, no shadow's framed symbol occurs anywhere in
//! the emitted record, the canonical prelude parents are still carried, and the
//! roster's spelling table and the fixture's inventory agree in both directions.
//! Seven mutations discriminate it: one per producer path class (record
//! constructor, spine constructor, family, operation), each restored to
//! `env.globals.get(name)` and each reddening while naming exactly the redirected
//! role; and three on the inventory relation (fixture shrinks, roster grows,
//! fixture goes stale).
//!
//! **CLAIMED.** No package declaration can capture a Runtime role: the roles
//! the roster carries resist substitution (measured here), and no role outside
//! the roster can be selected by spelling at all (closed by the signature, and
//! not measurable from a test).
//!
//! **THE GAP.** This file cannot see a role that stops going through the roster,
//! because the relation compares the fixture against the roster rather than
//! against the producers. That is now a *bounded* gap rather than a bypass: with
//! only `&PreludeEnv` and the symbol map in scope, the roster and the
//! already-canonical `PreludeEnv` ids are the sole ids a producer can reach, so
//! leaving the roster means widening a signature. What this file still cannot
//! discriminate is a role that moves from the roster to one of those
//! already-canonical fields — sound, since both are captured at registration,
//! but invisible here.
//! Roles whose ids were already canonical before this repair (`Zero`/`Suc`, the
//! private operations, the resource ids) are outside the fixture by
//! construction — they never passed through name lookup, so there is nothing to
//! substitute.

use ken_elaborator::checked_core::{CheckedCorePackage, StableSymbol, SymbolNamespace};
use ken_elaborator::prelude::CanonicalRuntimeRoles;
use ken_elaborator::compiler_driver::{
    checked_runtime_symbols_v1_key, compile_ken_package_sources, CompilerManifest, CompilerSource,
    CompilerTargetKind, TargetSelector,
};

const PACKAGE: &str = "d1b_role_a_shadow_pkg";

/// A package that declares colliding names across every class of role the two
/// producers resolve: the six former entry-plan roles, the ITree/coproduct
/// spine, `Result`/`Option`, `Bool`/`Unit`, IO errors, resource errors and
/// kinds, progress constructors, file-error and file-operation discriminants,
/// and the public host operations.
///
/// Every one of these spellings was, before this repair, a live
/// `env.globals.get(name)` in `checked_host_spine_v1` or
/// `checked_runtime_symbols_v1`.
const SHADOWING_SOURCE: &str = r#"data FSOp = ShadowFsOp
data ConsoleOp = ShadowConsoleOp
data ClockOp = ShadowClockOp
data EntropyOp = ShadowEntropyOp
data Cap = ShadowCap
data ShadowPlan = Nil | Cons | MkProd | MkProcessInput | Success | Failure
data ShadowSpine = Ret | Vis | InL | InR
data ShadowSums = Some | Ok | Err
data ShadowUnitBool = MkUnit | True | False
data ShadowIoErrors = NotFound | PermissionDenied | CapabilityDenied | BrokenPipe | Interrupted | AlreadyExists | InvalidInput | IsDirectory | NotDirectory | NotEmpty | Unsupported | Other
data ShadowResource = ResourceKindMismatch | BufferLimit | AllocationFailed | InvalidOffset | InvalidBounds | NoProgress | Buffer
data ShadowProgress = ReadSome | ReadEof | Wrote
data ShadowFileOps = MkFileError | OpReadFile | OpWriteFile | OpChangeMode
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
    ("ShadowPlan", "Nil"),
    ("ShadowPlan", "Cons"),
    ("ShadowPlan", "MkProd"),
    ("ShadowPlan", "MkProcessInput"),
    ("ShadowPlan", "Success"),
    ("ShadowPlan", "Failure"),
    ("ShadowSpine", "Ret"),
    ("ShadowSpine", "Vis"),
    ("ShadowSpine", "InL"),
    ("ShadowSpine", "InR"),
    ("ShadowSums", "Some"),
    ("ShadowSums", "Ok"),
    ("ShadowSums", "Err"),
    ("ShadowUnitBool", "MkUnit"),
    ("ShadowUnitBool", "True"),
    ("ShadowUnitBool", "False"),
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
    ("ShadowIoErrors", "Other"),
    ("ShadowResource", "ResourceKindMismatch"),
    ("ShadowResource", "BufferLimit"),
    ("ShadowResource", "AllocationFailed"),
    ("ShadowResource", "InvalidOffset"),
    ("ShadowResource", "InvalidBounds"),
    ("ShadowResource", "NoProgress"),
    ("ShadowResource", "Buffer"),
    ("ShadowProgress", "ReadSome"),
    ("ShadowProgress", "ReadEof"),
    ("ShadowProgress", "Wrote"),
    ("ShadowFileOps", "MkFileError"),
    ("ShadowFileOps", "OpReadFile"),
    ("ShadowFileOps", "OpWriteFile"),
    ("ShadowFileOps", "OpChangeMode"),
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

fn emit_shadowing_package() -> CheckedCorePackage {
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
    .expect("the shadowing package compiles -- colliding declarations are lawful Ken")
    .package
}

/// The five **family** roles, which are declarations rather than constructors.
///
/// They are a separate path class: `checked_host_spine_v1` resolves them into
/// `fs_family`/`console_family`/`clock_family`/`entropy_family`/`capability`,
/// and a shadow is a top-level `data` of the same name rather than a colliding
/// constructor. Kept in its own table because the symbol namespace differs.
const SHADOWED_FAMILIES: &[&str] = &["FSOp", "ConsoleOp", "ClockOp", "EntropyOp", "Cap"];

/// The fully qualified symbol a shadow constructor receives.
fn shadow_symbol(family: &str, constructor: &str) -> String {
    format!("ctor:{PACKAGE}::{family}::{constructor}")
}

/// The fully qualified symbol a shadow family declaration receives.
fn shadow_family_symbol(family: &str) -> String {
    format!("decl:{PACKAGE}::{family}")
}

/// Does the record carry this symbol as one of its encoded entries?
///
/// ⚠ **This deliberately is not a substring search.** Every symbol in the record
/// is framed as a little-endian `u64` length followed by its bytes, and matching
/// the frame is what makes an occurrence exact. A plain `contains` reports role
/// `X` whenever role `XY` is present, because `X`'s spelling is a prefix of
/// `XY`'s: the operation-role mutation below redirected only `ReadFile`, and a
/// substring reader also accused `Read`. That direction is safe — it over-reports
/// rather than under-reports — but it misattributes the defect, and a control
/// that names the wrong role sends its reader to the wrong line.
fn record_carries_symbol(record: &[u8], symbol: &str) -> bool {
    let mut needle = (symbol.len() as u64).to_le_bytes().to_vec();
    needle.extend_from_slice(symbol.as_bytes());
    record.windows(needle.len()).any(|window| window == needle)
}

#[test]
fn d1b_role_a_package_shadowing_cannot_redirect_any_stored_runtime_role() {
    let package = emit_shadowing_package();
    let metadata = &package.artifact.semantic.metadata;

    let record = metadata
        .get(&checked_runtime_symbols_v1_key())
        .unwrap_or_else(|| {
            panic!(
                "no CheckedRuntimeSymbolsV1 in the shadowing package's metadata; keys: {:?}",
                metadata.keys().collect::<Vec<_>>()
            )
        });
    let text = String::from_utf8_lossy(record);

    // POSITIVE CONTROL ON THE READER. Without it, every absence asserted below
    // is equally consistent with a reader that can see nothing at all.
    assert!(
        text.contains("CheckedRuntimeSymbolsV1"),
        "the stored bytes carry no version header, so nothing below is reading the record"
    );

    // POSITIVE CONTROL ON THE FIXTURE. The shadow declarations must really have
    // elaborated and really have produced these symbols. Without this, "the
    // shadow symbols are absent from the record" would also hold if the source
    // had silently failed to declare them, and the control would prove nothing.
    let declared: Vec<String> = package
        .artifact
        .semantic
        .symbols
        .iter()
        .map(|symbol| symbol.to_string())
        .collect();
    for (family, constructor) in SHADOWED_ROLES {
        let symbol = shadow_symbol(family, constructor);
        assert!(
            declared.contains(&symbol),
            "the fixture did not actually declare {symbol}; the shadowing control would then be \
             vacuous -- it would observe the absence of something that was never created"
        );
    }
    for family in SHADOWED_FAMILIES {
        let symbol = shadow_family_symbol(family);
        assert!(
            declared.contains(&symbol),
            "the fixture did not actually declare the shadow family {symbol}; see above"
        );
    }

    // SUBSTITUTION RESISTANCE. Not one shadow constructor may appear in the
    // record. Each assertion names the FULLY QUALIFIED symbol: the parent chain
    // is the role's identity, and a redirected role is spelled under the
    // package's shadow family rather than under the prelude's.
    let mut redirected = Vec::new();
    for (family, constructor) in SHADOWED_ROLES {
        let symbol = shadow_symbol(family, constructor);
        if record_carries_symbol(record, &symbol) {
            redirected.push(symbol);
        }
    }
    for family in SHADOWED_FAMILIES {
        let symbol = shadow_family_symbol(family);
        if record_carries_symbol(record, &symbol) {
            redirected.push(symbol);
        }
    }
    assert!(
        redirected.is_empty(),
        "the record carries package constructors in Runtime role positions: {redirected:?}\n\
         A role selected by source spelling after package elaboration resolves to the package's \
         declaration, which would hand user constructors Runtime's special meaning."
    );

    // The canonical prelude parents are still present, so the roles were carried
    // rather than merely dropped. An empty or truncated record would satisfy the
    // absence check above for the wrong reason.
    //
    // These are deliberately PREFIX probes on the parent chain, not framed exact
    // matches: the role's own final component is not what is being asked about,
    // and under shadowing it degrades (the prelude's `Nil` loses its name to the
    // package's, so its symbol becomes `...::List::ctor_<id>`). The parent is
    // what survives shadowing, and the parent is the identity.
    for parent in [
        "ctor:d1b_role_a_shadow_pkg::List::",
        "ctor:d1b_role_a_shadow_pkg::Prod::",
        "ctor:d1b_role_a_shadow_pkg::ITree::",
        "ctor:d1b_role_a_shadow_pkg::Coproduct::",
        "ctor:d1b_role_a_shadow_pkg::Result::",
        "ctor:d1b_role_a_shadow_pkg::Option::",
        "ctor:d1b_role_a_shadow_pkg::Bool::",
        "ctor:d1b_role_a_shadow_pkg::Unit::",
    ] {
        assert!(
            text.contains(parent),
            "the record carries no role under the canonical prelude parent {parent}; the roles \
             were not merely un-redirected, they are missing"
        );
    }
}

/// Roster inventory, stated as a **relation between two artifacts**.
///
/// The test above proves that nothing the fixture shadows can be redirected.
/// That stays exhaustive over the roster only if the fixture shadows every role
/// the roster carries — and the fixture's table is hand-written, so on its own
/// it is a snapshot that a newly added role would silently escape.
///
/// This keeps the two in step: the roster's spelling table is compared against
/// the fixture's shadow inventory in both directions. Add a role to
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
fn d1b_role_a_every_canonical_role_is_covered_by_the_shadowing_fixture() {
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

    let unshadowed: Vec<&(&str, &str)> = CanonicalRuntimeRoles::spellings()
        .iter()
        .filter(|(_, spelling)| !shadowed.contains(spelling))
        .collect();
    assert!(
        unshadowed.is_empty(),
        "these canonical roles are NOT shadowed by the fixture, so the substitution control says \
         nothing about them: {unshadowed:?}\n\
         Add each spelling to the fixture's declarations and to SHADOWED_ROLES (or \
         SHADOWED_FAMILIES for a type), or the completeness claim is partial."
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
