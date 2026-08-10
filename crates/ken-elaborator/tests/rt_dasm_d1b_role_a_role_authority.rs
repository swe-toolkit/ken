//! `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-a` — the role-authority discriminator.
//!
//! **What this pins, in two separate properties.**
//!
//! 1. **Substitution resistance.** A package that declares its own constructors
//!    under the prelude's role spellings must not redirect any stored Runtime
//!    role onto them. The record must still carry the roles the *canonical
//!    prelude* `GlobalId`s denote.
//! 2. **Inventory completeness.** No role in either producer may still be
//!    selected by source spelling. This is proved *through* property 1 rather
//!    than by reading the producers' source: the fixture shadows a spelling from
//!    every namespace and path class both producers resolve, so any role still
//!    going through `env.globals.get(name)` would be redirected and observed.
//!
//! **Why bare-name containment is not the assertion.** The rejected candidate
//! `aade3c2f` searched the serialized record for the bare strings `"Nil"`,
//! `"Cons"`, … . Those strings are present whether the stored role is the
//! prelude's constructor or a package constructor of the same name, so the
//! control stayed green under exactly the substitution it was meant to exclude.
//! Every assertion below names a **fully qualified** symbol — the parent chain
//! is the identity, and the parent is what a substitution changes.

use ken_elaborator::checked_core::{CheckedCorePackage, StableSymbol, SymbolNamespace};
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
