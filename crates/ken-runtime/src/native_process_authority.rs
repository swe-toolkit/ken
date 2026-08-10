//! `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-c1` — the named authority-validation
//! lane for package-backed native compilation.
//!
//! **What this lane is for.** A package-backed compile must lower against the
//! constructor identities its own checked package recorded, not against the
//! legacy prelude spellings. Before `D1b-role-c1` every package-backed compile
//! passed `None` for its authority and the lowerer silently substituted
//! `NativeProcessSymbols::legacy_prelude()` — so a package-qualified `Nat` never
//! matched, and nothing said so.
//!
//! ⛔ **This lane fails closed.** Missing, malformed, duplicated or
//! metadata-inconsistent authority is a rejection, never a fallback. It runs
//! **before** `plan_static_transition_graph_with_symbols`, so a rejected package
//! never reaches planning at all.
//!
//! ## Where this sits relative to erasure's validation
//!
//! Erasure (`D1b-role-b`) already decodes the record and validates every role
//! against the package's own `semantic.symbols` and `data_metadata`. **That
//! layer is upstream of this one and subsumes much of it**: a corrupt header or
//! a role swapped to a foreign symbol is rejected there and never produces a
//! `RuntimeProgram` for this lane to see.
//!
//! ⇒ **The rejection this lane uniquely owns is ABSENCE** — a well-formed
//! program that simply carries no record, which erasure leaves as a lawful
//! `None` because the seed-only lane legitimately has none. The other cases are
//! defence in depth, reachable here only if the upstream layer is bypassed.
//! Saying that plainly is the point; presenting them as independent coverage
//! would overstate what this lane adds.

use crate::ir::{
    RuntimeAssumptionTrustKind, RuntimeCheckedRoleSymbolsV1, RuntimeLowerabilityStatus,
    RuntimeProgram, RuntimeSymbol,
};
use crate::NativeProcessSymbols;

/// Why a package-backed compile was refused its authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativeAuthorityError {
    /// The package carries no decoded role record.
    ///
    /// This is the case this lane uniquely catches: erasure leaves the field
    /// `None` for a package that never had a record, which is lawful there and
    /// fatal here.
    MissingRoleRecord,
    /// A role's symbol is empty, so it cannot denote any constructor.
    EmptyRole { role: &'static str },
    /// Two distinct roles resolve to the same symbol.
    ///
    /// Runtime assigns each role a different meaning; one symbol standing in
    /// two role positions makes the lowering's dispatch ambiguous.
    DuplicateRole {
        first: &'static str,
        second: &'static str,
        symbol: String,
    },
    /// A role is absent from the package's own recorded data metadata.
    MetadataInconsistent { role: &'static str, symbol: String },
    /// The record carries the wrong number of IO errors for the lowering.
    IoErrorArity { found: usize, expected: usize },
    /// The role authority resolved, but the package's trust did not close
    /// against its own pre-source roster (`D1b-role-c1`).
    UntrustedProgram(NativeTrustError),
}

impl std::fmt::Display for NativeAuthorityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NativeAuthorityError::MissingRoleRecord => write!(
                f,
                "package-backed native compilation requires the checked role authority, and this \
                 program carries none; refusing rather than falling back to the legacy prelude"
            ),
            NativeAuthorityError::EmptyRole { role } => {
                write!(f, "checked role {role} is empty")
            }
            NativeAuthorityError::DuplicateRole {
                first,
                second,
                symbol,
            } => write!(
                f,
                "checked roles {first} and {second} both resolve to {symbol}, so the lowering \
                 cannot tell them apart"
            ),
            NativeAuthorityError::MetadataInconsistent { role, symbol } => write!(
                f,
                "checked role {role} names {symbol}, which the package's own data metadata does \
                 not record; the authority and the package disagree"
            ),
            NativeAuthorityError::IoErrorArity { found, expected } => write!(
                f,
                "checked authority carries {found} IO error role(s) but the lowering requires \
                 {expected}"
            ),
            NativeAuthorityError::UntrustedProgram(err) => write!(f, "{err}"),
        }
    }
}

/// The number of IO error constructors the lowering indexes positionally.
const IO_ERROR_ARITY: usize = 12;

/// Why a package-backed program was refused native admission on its **trust**.
///
/// `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-c1`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativeTrustError {
    /// The package carries no pre-source trusted-base roster.
    ///
    /// ⛔ There is no fallback. Without the roster the question "was this
    /// trusted before the package's own source was elaborated?" is
    /// unanswerable, and every remaining check would be comparing a package
    /// against itself.
    MissingRoster,
    /// The assumption keys and the trust-metadata keys are not the same set.
    TupleKeysUnpaired { detail: String },
    /// A trust item is not a `Postulate`, or does not affect runtime meaning.
    UnsupportedTrustKind { assumption: String, detail: String },
    /// Two trust items name the same target.
    DuplicateTarget { target: String },
    /// The tuple targets and the `trusted_base_delta` keys disagree.
    DeltaTargetsUnpaired { detail: String },
    /// A trust target was NOT in the pre-source roster, or the roster holds a
    /// target the package no longer claims.
    ///
    /// This is the case the whole mechanism exists for: a postulate introduced
    /// by the package's own source lands here, whatever identity it was given.
    RosterMismatch {
        introduced_by_source: Vec<String>,
        missing_from_package: Vec<String>,
    },
    /// An assumption or target is absent from the package's own symbol set.
    SymbolNotInPackage { symbol: String, role: &'static str },
    /// The assumption's identity is not its target's canonical projection.
    NoncanonicalAssumptionIdentity { assumption: String, expected: String },
    /// The package carries effect, foreign, or trusted-partial metadata.
    UntrustedMetadataPresent { lane: &'static str, detail: String },
    /// Something the package itself marks as not lowerable.
    NotSupported { symbol: String, detail: String },
}

impl std::fmt::Display for NativeTrustError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NativeTrustError::MissingRoster => write!(
                f,
                "package-backed native admission requires the pre-source trusted-base roster, and \
                 this program carries none; refusing rather than trusting the package's own \
                 account of what it assumed"
            ),
            NativeTrustError::TupleKeysUnpaired { detail } => write!(
                f,
                "the assumption set and the trust-metadata set are not the same set: {detail}"
            ),
            NativeTrustError::UnsupportedTrustKind { assumption, detail } => write!(
                f,
                "trust item {assumption} is not an admissible postulate: {detail}"
            ),
            NativeTrustError::DuplicateTarget { target } => write!(
                f,
                "two trust items name the target {target}; a trusted base cannot be entered twice"
            ),
            NativeTrustError::DeltaTargetsUnpaired { detail } => write!(
                f,
                "the trust tuple targets and the trusted-base delta disagree: {detail}"
            ),
            NativeTrustError::RosterMismatch {
                introduced_by_source,
                missing_from_package,
            } => write!(
                f,
                "the package's trust does not match the pre-source trusted-base roster; \
                 introduced after the prelude (i.e. by this package's own source): {:?}; \
                 in the roster but not claimed by the package: {:?}",
                introduced_by_source, missing_from_package
            ),
            NativeTrustError::SymbolNotInPackage { symbol, role } => write!(
                f,
                "the {role} {symbol} is not in the package's own symbol set, so the package does \
                 not actually carry what its trust metadata claims"
            ),
            NativeTrustError::NoncanonicalAssumptionIdentity {
                assumption,
                expected,
            } => write!(
                f,
                "assumption {assumption} is not its target's canonical projection; expected \
                 {expected}"
            ),
            NativeTrustError::UntrustedMetadataPresent { lane, detail } => write!(
                f,
                "the package carries {lane} metadata outside the admissible native subset: \
                 {detail}"
            ),
            NativeTrustError::NotSupported { symbol, detail } => {
                write!(f, "{symbol} is not lowerable: {detail}")
            }
        }
    }
}

/// A package-backed program admitted to native compilation.
///
/// ⛔ Constructible **only** by `native_program_admission`, which fails closed.
/// Holding one is the evidence that the role authority resolved *and* the
/// package's trust closed against its own pre-source roster.
#[derive(Clone, Debug)]
pub struct NativeProgramAdmission {
    authority: NativeProcessSymbols,
    admitted_trust: std::collections::BTreeSet<RuntimeSymbol>,
}

impl NativeProgramAdmission {
    /// The validated constructor-identity authority.
    pub(crate) fn authority(&self) -> &NativeProcessSymbols {
        &self.authority
    }

    /// The assumptions this program was admitted **with**, for the trust report.
    ///
    /// These are not incidental: a native artifact rests on them, so they
    /// propagate into the run and object reports rather than being discarded at
    /// the admission boundary.
    pub fn admitted_trust(&self) -> &std::collections::BTreeSet<RuntimeSymbol> {
        &self.admitted_trust
    }

    /// The pair the lowering entrypoints actually consume.
    pub(crate) fn compilation(&self) -> AdmittedNativeCompilation<'_> {
        AdmittedNativeCompilation {
            authority: &self.authority,
            admitted_trust: &self.admitted_trust,
        }
    }
}

/// The empty admitted-trust set a synthetic program rests on.
///
/// ⛔ A synthetic program was never admitted, so it has **no** admitted trust —
/// that is a statement about it, not a missing value. Giving the synthetic lane
/// its own named constant is what keeps `Option` out of the seam.
#[cfg(test)]
static NO_ADMITTED_TRUST: std::collections::BTreeSet<RuntimeSymbol> =
    std::collections::BTreeSet::new();

/// The authority and the admitted trust, travelling as **one** value.
///
/// ⛔ This exists so a lowering entrypoint cannot be handed an authority while
/// the admitted trust is silently dropped: the shared bodies take this pair and
/// there is no way to name the authority alone at those seams. A required
/// second parameter would have proven only that a value was *supplied*; making
/// the pair indivisible is what removes the "forgot to thread it" failure mode
/// from the type, and the report control below is what proves it is *consumed*.
///
/// Exactly two producers: `NativeProgramAdmission::compilation` in production,
/// which is reachable only through the fail-closed `native_program_admission`;
/// and `synthetic_admitted_compilation`, which is `#[cfg(test)]` and therefore
/// absent from a production build.
#[derive(Clone, Copy, Debug)]
pub(crate) struct AdmittedNativeCompilation<'a> {
    authority: &'a NativeProcessSymbols,
    admitted_trust: &'a std::collections::BTreeSet<RuntimeSymbol>,
}

impl<'a> AdmittedNativeCompilation<'a> {
    pub(crate) fn authority(&self) -> &'a NativeProcessSymbols {
        self.authority
    }

    /// The assumption identities that must reach the emitted trust report.
    pub(crate) fn admitted_trust(&self) -> &'a std::collections::BTreeSet<RuntimeSymbol> {
        self.admitted_trust
    }
}

/// The synthetic lane's compilation pair. ⛔ `#[cfg(test)]`; see
/// `AdmittedNativeCompilation`.
#[cfg(test)]
pub(crate) fn synthetic_admitted_compilation(
    authority: &NativeProcessSymbols,
) -> AdmittedNativeCompilation<'_> {
    AdmittedNativeCompilation {
        authority,
        admitted_trust: &NO_ADMITTED_TRUST,
    }
}


/// The canonical assumption identity for a trust target.
///
/// `decl:pkg::X` projects to `assume:pkg::X::trusted_base`. ⛔ This is a
/// **relation between two fields**, not a name whitelist: it accepts any target
/// whatsoever, and refuses only an assumption whose identity does not derive
/// from the target it claims. A retargeted assumption fails it.
fn canonical_assumption_identity(target: &str) -> Option<String> {
    let path = target.strip_prefix("decl:")?;
    Some(format!("assume:{path}::trusted_base"))
}

/// Admit a package-backed program to native compilation, or refuse it.
///
/// ⛔ Fail-closed throughout. Order matters: the trust closure runs **before**
/// the general native blockers, so a roster mismatch is diagnosed as a roster
/// mismatch rather than being masked by a coarser refusal that happens to fire
/// on the same package for an unrelated reason.
pub fn native_program_admission(
    program: &RuntimeProgram,
) -> Result<NativeProgramAdmission, NativeAuthorityError> {
    let authority = native_authority_for_program(program)?;
    let admitted_trust =
        validate_native_trust(program).map_err(NativeAuthorityError::UntrustedProgram)?;
    Ok(NativeProgramAdmission {
        authority,
        admitted_trust,
    })
}

/// The trust closure. Returns the admitted assumption set.
fn validate_native_trust(
    program: &RuntimeProgram,
) -> Result<std::collections::BTreeSet<RuntimeSymbol>, NativeTrustError> {
    let metadata = &program.erased_core.metadata;
    let checked = &metadata.checked_core;

    // (1) The roster is REQUIRED here. Absence is the seed lane's lawful state,
    // never this one's.
    let roster = checked
        .native_trusted_base
        .as_ref()
        .ok_or(NativeTrustError::MissingRoster)?;

    // (2) No effect, foreign, or trusted-partial metadata anywhere.
    if !metadata.effects.is_empty() {
        return Err(NativeTrustError::UntrustedMetadataPresent {
            lane: "effect",
            detail: format!("{:?}", metadata.effects),
        });
    }
    if !checked.effects_foreign_metadata.is_empty() {
        return Err(NativeTrustError::UntrustedMetadataPresent {
            lane: "foreign",
            detail: format!(
                "{:?}",
                checked.effects_foreign_metadata.keys().collect::<Vec<_>>()
            ),
        });
    }
    if !metadata.capabilities.is_empty() || !metadata.runtime_checks.is_empty() {
        return Err(NativeTrustError::UntrustedMetadataPresent {
            lane: "trusted-partial",
            detail: format!(
                "capabilities={:?} runtime_checks={:?}",
                metadata.capabilities, metadata.runtime_checks
            ),
        });
    }
    if !metadata.unsupported.is_empty() {
        return Err(NativeTrustError::UntrustedMetadataPresent {
            lane: "unsupported",
            detail: format!("{:?}", metadata.unsupported.keys().collect::<Vec<_>>()),
        });
    }

    // (3) Assumption keys and trust-metadata keys are the SAME set. A tuple
    // with a severed partner on either side dies here.
    let assumption_keys: std::collections::BTreeSet<&RuntimeSymbol> =
        metadata.assumptions.keys().collect();
    let trust_keys: std::collections::BTreeSet<&RuntimeSymbol> =
        metadata.assumption_trust_metadata.keys().collect();
    if assumption_keys != trust_keys {
        let only_assumptions: Vec<_> = assumption_keys.difference(&trust_keys).collect();
        let only_trust: Vec<_> = trust_keys.difference(&assumption_keys).collect();
        return Err(NativeTrustError::TupleKeysUnpaired {
            detail: format!(
                "assumptions without trust metadata: {only_assumptions:?}; trust metadata \
                 without assumptions: {only_trust:?}"
            ),
        });
    }

    // (4) Per-item shape, canonical identity, package membership, uniqueness.
    let mut targets: std::collections::BTreeSet<RuntimeSymbol> =
        std::collections::BTreeSet::new();
    for (assumption, trust) in &metadata.assumption_trust_metadata {
        if trust.kind != RuntimeAssumptionTrustKind::Postulate {
            return Err(NativeTrustError::UnsupportedTrustKind {
                assumption: assumption.clone(),
                detail: format!("kind is {:?}, only Postulate is admissible", trust.kind),
            });
        }
        if !trust.affects_runtime_meaning {
            return Err(NativeTrustError::UnsupportedTrustKind {
                assumption: assumption.clone(),
                detail: "affects_runtime_meaning is false, so this is not a trusted-base entry"
                    .to_string(),
            });
        }
        let expected = canonical_assumption_identity(&trust.target).ok_or_else(|| {
            NativeTrustError::NoncanonicalAssumptionIdentity {
                assumption: assumption.clone(),
                expected: format!("a declaration target, got {}", trust.target),
            }
        })?;
        if assumption != &expected {
            return Err(NativeTrustError::NoncanonicalAssumptionIdentity {
                assumption: assumption.clone(),
                expected,
            });
        }
        if !program.erased_core.symbols.contains(assumption) {
            return Err(NativeTrustError::SymbolNotInPackage {
                symbol: assumption.clone(),
                role: "assumption",
            });
        }
        if !program.erased_core.symbols.contains(&trust.target) {
            return Err(NativeTrustError::SymbolNotInPackage {
                symbol: trust.target.clone(),
                role: "trust target",
            });
        }
        if !targets.insert(trust.target.clone()) {
            return Err(NativeTrustError::DuplicateTarget {
                target: trust.target.clone(),
            });
        }
    }

    // (5) Targets equal the trusted-base delta keys.
    let delta_keys: std::collections::BTreeSet<RuntimeSymbol> =
        metadata.trusted_base_delta.keys().cloned().collect();
    if targets != delta_keys {
        return Err(NativeTrustError::DeltaTargetsUnpaired {
            detail: format!(
                "targets without a delta entry: {:?}; delta entries without a target: {:?}",
                targets.difference(&delta_keys).collect::<Vec<_>>(),
                delta_keys.difference(&targets).collect::<Vec<_>>()
            ),
        });
    }

    // (6) THE PROVENANCE CHECK. Targets must equal the pre-source roster
    // exactly. A postulate the package's own source introduced is in `targets`
    // and not in `roster`, because the roster was closed before that source
    // was elaborated -- and no identity the source can choose changes that.
    if targets != roster.targets {
        return Err(NativeTrustError::RosterMismatch {
            introduced_by_source: targets
                .difference(&roster.targets)
                .cloned()
                .collect(),
            missing_from_package: roster
                .targets
                .difference(&targets)
                .cloned()
                .collect(),
        });
    }

    // (7) Where lowerability IS recorded it must be Supported; absence is
    // valid, because these targets are prelude declarations that carry no
    // primitive audit metadata at all.
    for (symbol, primitive) in &checked.primitive_metadata {
        if primitive.lowerability != RuntimeLowerabilityStatus::Supported {
            return Err(NativeTrustError::NotSupported {
                symbol: symbol.clone(),
                detail: format!("primitive metadata says {:?}", primitive.lowerability),
            });
        }
    }
    for (symbol, status) in &metadata.lowerability {
        if *status != RuntimeLowerabilityStatus::Supported {
            return Err(NativeTrustError::NotSupported {
                symbol: symbol.clone(),
                detail: format!("declared lowerability is {status:?}"),
            });
        }
    }
    // Executable declarations must still carry Supported lowerability. Absence
    // is permitted for trust targets, never for the code being compiled.
    for declaration in &program.declarations {
        match &declaration.metadata.lowerability {
            Some(RuntimeLowerabilityStatus::Supported) => {}
            other => {
                return Err(NativeTrustError::NotSupported {
                    symbol: declaration.symbol.clone(),
                    detail: format!("executable declaration lowerability is {other:?}"),
                })
            }
        }
    }

    Ok(metadata.assumptions.keys().cloned().collect())
}

/// Derive the validated native authority for a package-backed compile.
///
/// ⛔ Fail-closed: every failure is a refusal. There is no path from this
/// function to `legacy_prelude()`.
pub fn native_authority_for_program(
    program: &RuntimeProgram,
) -> Result<NativeProcessSymbols, NativeAuthorityError> {
    let record = program
        .erased_core
        .metadata
        .checked_core
        .runtime_symbols
        .as_ref()
        .ok_or(NativeAuthorityError::MissingRoleRecord)?;
    validate_role_record(record, program)?;
    Ok(native_process_symbols_from_record(record))
}

/// Check the decoded record against itself and against the package.
fn validate_role_record(
    record: &RuntimeCheckedRoleSymbolsV1,
    program: &RuntimeProgram,
) -> Result<(), NativeAuthorityError> {
    let roles = record.roles();

    for (role, symbol) in &roles {
        if symbol.trim().is_empty() {
            return Err(NativeAuthorityError::EmptyRole { role });
        }
    }

    // Distinctness. Two roles sharing a symbol is not a stale-metadata problem
    // -- it is an ambiguity the lowering cannot resolve at all.
    for (index, (role, symbol)) in roles.iter().enumerate() {
        for (other_role, other_symbol) in roles.iter().skip(index + 1) {
            if symbol == other_symbol {
                return Err(NativeAuthorityError::DuplicateRole {
                    first: role,
                    second: other_role,
                    symbol: (*symbol).clone(),
                });
            }
        }
    }

    if record.spine.io_errors.len() != IO_ERROR_ARITY {
        return Err(NativeAuthorityError::IoErrorArity {
            found: record.spine.io_errors.len(),
            expected: IO_ERROR_ARITY,
        });
    }

    // Agreement with the package's own recorded constructors. Erasure already
    // checked this against `semantic.symbols`; here it is checked against the
    // metadata the RuntimeProgram itself carries, which is what a consumer of
    // this program can see.
    let data = &program.erased_core.metadata.checked_core.data_metadata;
    for (role, symbol) in &roles {
        if !symbol.starts_with("ctor:") {
            continue;
        }
        let recorded = data
            .values()
            .any(|family| family.constructors.iter().any(|c| &c.symbol == *symbol));
        if !recorded {
            return Err(NativeAuthorityError::MetadataInconsistent {
                role,
                symbol: (*symbol).clone(),
            });
        }
    }
    Ok(())
}

/// Project the decoded record onto the lowering's authority type.
///
/// The record is the wider of the two: it also carries effect families, the
/// capability, the ITree spine and the operation table, none of which the
/// constructor-identity lowering consults. Only the roles the lowering actually
/// dispatches on are projected.
fn native_process_symbols_from_record(
    record: &RuntimeCheckedRoleSymbolsV1,
) -> NativeProcessSymbols {
    let spine = &record.spine;
    NativeProcessSymbols {
        process_input: record.process_input.clone(),
        list_nil: record.list_nil.clone(),
        list_cons: record.list_cons.clone(),
        prod: record.prod.clone(),
        exit_success: record.exit_success.clone(),
        exit_failure: record.exit_failure.clone(),
        result_err: spine.result_err.clone(),
        result_ok: spine.result_ok.clone(),
        option_some: spine.option_some.clone(),
        file_error: spine.file_error.clone(),
        file_operation_read: spine.file_operation_read.clone(),
        file_operation_write: spine.file_operation_write.clone(),
        file_operation_change_mode: spine.file_operation_change_mode.clone(),
        io_errors: spine.io_errors.clone(),
        resource_host_io: spine.resource_host_io.clone(),
        resource_closed: spine.resource_closed.clone(),
        resource_malformed: spine.resource_malformed.clone(),
        resource_right_not_held: spine.resource_right_not_held.clone(),
        resource_release_failed: spine.resource_release_failed.clone(),
        resource_kind_mismatch: spine.resource_kind_mismatch.clone(),
        resource_buffer_limit: spine.resource_buffer_limit.clone(),
        resource_allocation_failed: spine.resource_allocation_failed.clone(),
        resource_invalid_offset: spine.resource_invalid_offset.clone(),
        resource_invalid_bounds: spine.resource_invalid_bounds.clone(),
        resource_no_progress: spine.resource_no_progress.clone(),
        resource_kind_fs_handle: spine.resource_kind_fs_handle.clone(),
        resource_kind_buffer: spine.resource_kind_buffer.clone(),
        resource_trace_identity: spine.resource_trace_identity.clone(),
        nat_zero: spine.nat_zero.clone(),
        nat_suc: spine.nat_suc.clone(),
        private_buffer_span: spine.private_buffer_span.clone(),
        private_transfer_count: spine.private_transfer_count.clone(),
        read_some: spine.read_some.clone(),
        read_eof: spine.read_eof.clone(),
        wrote: spine.wrote.clone(),
        unit: spine.unit.clone(),
        bool_false: spine.bool_false.clone(),
        bool_true: spine.bool_true.clone(),
    }
}

/// The legacy prelude authority, for **synthetic** hand-built test programs.
///
/// `RT-DYNAMIC-ARM-SCALAR-MERGE` `D1b-role-c1`. The tests that reach native
/// lowering build a `RuntimeProgram` struct literal by hand, with a fabricated
/// package identity and hash; none comes from a `CheckedCorePackage` or from
/// erasure. They are entitled to legacy authority for the same reason the seed
/// lane is — their IR is minted in the `prelude::` namespace — but they must
/// **say so**, which is why every synthetic compilation call passes this
/// explicitly rather than inheriting a default.
///
/// ⛔ **`#[cfg(test)]` is the boundary.** In a production build this item does
/// not exist, so no production path can name it. That is proved by a
/// compilation-boundary positive control — referencing it from a production
/// path fails to compile — not by a grep or a zero-result check.
#[cfg(test)]
pub(crate) fn synthetic_test_legacy_authority() -> NativeProcessSymbols {
    NativeProcessSymbols::legacy_prelude()
}
