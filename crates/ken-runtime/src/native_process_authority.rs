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

use crate::ir::{RuntimeCheckedRoleSymbolsV1, RuntimeProgram};
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
        }
    }
}

/// The number of IO error constructors the lowering indexes positionally.
const IO_ERROR_ARITY: usize = 12;

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
