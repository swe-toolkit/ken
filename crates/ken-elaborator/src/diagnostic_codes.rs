//! Stable code projections for the checked front end. Rendering is deferred.

use crate::compiler_driver::{CompilerDriverError, NativeProgramBuildError};
use crate::error::ElabError;
use ken_kernel::KernelError;

// Project kernel rejections without changing or depending from the TCB.
ken_diagnostic::define_error_codes! {
    fn kernel_error_code(error: &KernelError);
    table KERNEL_ERROR_TABLE;
    KernelError::VarOutOfScope { .. } => ("K0001", "kernel.var_out_of_scope"),
    KernelError::K2ReservedFormer [] => ("K0002", "kernel.k2_reserved_former"),
    KernelError::UniverseInconsistency { .. } => ("K0003", "kernel.universe_inconsistency"),
    KernelError::TypeMismatch { .. } => ("K0004", "kernel.type_mismatch"),
    KernelError::NotAFunction { .. } => ("K0005", "kernel.not_a_function"),
    KernelError::NotASigma { .. } => ("K0006", "kernel.not_a_sigma"),
    KernelError::PositivityViolation(..) => ("K0007", "kernel.positivity_violation"),
    KernelError::ConstructorUniverseViolation { .. } => ("K0008", "kernel.constructor_universe_violation"),
    KernelError::IllFormedDecl(..) => ("K0009", "kernel.ill_formed_decl"),
    KernelError::LevelArityMismatch { .. } => ("K0010", "kernel.level_arity_mismatch"),
    KernelError::BadEliminator(..) => ("K0011", "kernel.bad_eliminator"),
    KernelError::Primitive(..) => ("K0012", "kernel.primitive"),
    KernelError::Msg(..) => ("K0013", "kernel.msg"),
    KernelError::NotTerminating(..) => ("K0014", "kernel.not_terminating"),
}

ken_diagnostic::define_error_codes! {
    impl ElabError;
    table ELAB_ERROR_TABLE;
    Self::ParseError { .. } => ("E0001", "elab.parse_error"),
    Self::NonAsciiIdentifierCharacter { .. } => ("E0002", "elab.non_ascii_identifier_character"),
    Self::RawFormatCharacter { .. } => ("E0003", "elab.raw_format_character"),
    Self::InvalidEscape { .. } => ("E0004", "elab.invalid_escape"),
    Self::ForeignNameControlCharacter { .. } => ("E0005", "elab.foreign_name_control_character"),
    Self::NamedBoundaryHeader { .. } => ("E0006", "elab.named_boundary_header"),
    Self::UnknownCapabilityFamily { .. } => ("E0007", "elab.unknown_capability_family"),
    Self::InvalidCapabilityAuthority { .. } => ("E0008", "elab.invalid_capability_authority"),
    Self::DuplicateCapabilityFamily { .. } => ("E0009", "elab.duplicate_capability_family"),
    Self::PackageCapabilitiesNotAllowed { .. } => ("E0010", "elab.package_capabilities_not_allowed"),
    Self::MissingCapability { .. } => ("E0011", "elab.missing_capability"),
    Self::UnboundName { .. } => ("E0012", "elab.unbound_name"),
    Self::IfConditionNotBool { .. } => ("E0013", "elab.if_condition_not_bool"),
    Self::PositionalProjectionNotPair { .. } => ("E0014", "elab.positional_projection_not_pair"),
    Self::FixedWidthLiteralOutOfRange { .. } => ("E0015", "elab.fixed_width_literal_out_of_range"),
    Self::StructuralResultOutOfScope { .. } => ("E0016", "elab.structural_result_out_of_scope"),
    Self::StructuralResultAssociationMissing { .. } => ("E0017", "elab.structural_result_association_missing"),
    Self::StructuralResultAssociationDuplicate { .. } => ("E0018", "elab.structural_result_association_duplicate"),
    Self::StructuralResultAssociationSwapped { .. } => ("E0019", "elab.structural_result_association_swapped"),
    Self::StructuralResultAssociationForeign { .. } => ("E0020", "elab.structural_result_association_foreign"),
    Self::RecursiveResultSortMismatch { .. } => ("E0021", "elab.recursive_result_sort_mismatch"),
    Self::RecursiveResultSortAmbiguous { .. } => ("E0022", "elab.recursive_result_sort_ambiguous"),
    Self::RecursiveResultClassifierNotUniverse { .. } => ("E0023", "elab.recursive_result_classifier_not_universe"),
    Self::OldPreStateUnsupported { .. } => ("E0024", "elab.old_pre_state_unsupported"),
    Self::MutationOutsideSpace { .. } => ("E0025", "elab.mutation_outside_space"),
    Self::UnsupportedSpacePlacement { .. } => ("E0026", "elab.unsupported_space_placement"),
    Self::UnresolvedCon { .. } => ("E0027", "elab.unresolved_con"),
    Self::DeclarationIdentityCollision { .. } => ("E0028", "elab.declaration_identity_collision"),
    Self::DuplicateDefinition { .. } => ("E0029", "elab.duplicate_definition"),
    Self::DuplicateConstructorSpelling { .. } => ("E0030", "elab.duplicate_constructor_spelling"),
    Self::FixityTargetNotLocal { .. } => ("E0031", "elab.fixity_target_not_local"),
    Self::ConflictingFixity { .. } => ("E0032", "elab.conflicting_fixity"),
    Self::NonAssociativeInfix { .. } => ("E0033", "elab.non_associative_infix"),
    Self::InvalidFixityPrecedence { .. } => ("E0034", "elab.invalid_fixity_precedence"),
    Self::ImportCycle { .. } => ("E0035", "elab.import_cycle"),
    Self::PremiseWithoutObligationChannel { .. } => ("E0036", "elab.premise_without_obligation_channel"),
    Self::ObligationWithoutChannel { .. } => ("E0037", "elab.obligation_without_channel"),
    Self::TypeMismatch { .. } => ("E0038", "elab.type_mismatch"),
    Self::LambdaVsNonFunction { .. } => ("E0039", "elab.lambda_vs_non_function"),
    Self::NotAFunction { .. } => ("E0040", "elab.not_a_function"),
    Self::LevelConflict { .. } => ("E0041", "elab.level_conflict"),
    Self::ConstructorUniverseViolation { .. } => ("E0042", "elab.constructor_universe_violation"),
    Self::ExhaustivenessError { .. } => ("E0043", "elab.exhaustiveness_error"),
    Self::InferredMatchResultEscapesPattern { .. } => ("E0044", "elab.inferred_match_result_escapes_pattern"),
    Self::MatrixResultDiscovered { .. } => ("E0045", "elab.matrix_result_discovered"),
    Self::ReachabilityError { .. } => ("E0046", "elab.reachability_error"),
    Self::OrphanInstance { .. } => ("E0047", "elab.orphan_instance"),
    Self::OverlappingInstances { .. } => ("E0048", "elab.overlapping_instances"),
    Self::UnadmittedInstance { .. } => ("E0049", "elab.unadmitted_instance"),
    Self::AmbiguousInstance { .. } => ("E0050", "elab.ambiguous_instance"),
    Self::NoInstance { .. } => ("E0051", "elab.no_instance"),
    Self::StandardOperatorRoleUnfilled { .. } => ("E0052", "elab.standard_operator_role_unfilled"),
    Self::StandardOperatorRoleWrongShape { .. } => ("E0053", "elab.standard_operator_role_wrong_shape"),
    Self::StandardOperatorRolesShareABinding { .. } => ("E0054", "elab.standard_operator_roles_share_a_binding"),
    Self::InstanceHeadSpellingsShareAnIdentity { .. } => ("E0055", "elab.instance_head_spellings_share_an_identity"),
    Self::InstanceCarrierIdentityMismatch { .. } => ("E0056", "elab.instance_carrier_identity_mismatch"),
    Self::NonTerminatingInstances { .. } => ("E0057", "elab.non_terminating_instances"),
    Self::AmbiguousReference { .. } => ("E0058", "elab.ambiguous_reference"),
    Self::ReExportCollision { .. } => ("E0059", "elab.re_export_collision"),
    Self::Internal(..) => ("E0060", "elab.internal"),
    ;
    Self::KernelRejected { error, .. } => kernel_error_code(error),
}

ken_diagnostic::define_error_codes! {
    impl CompilerDriverError;
    table COMPILER_DRIVER_ERROR_TABLE;
    Self::EmptyPackageName [] => ("E0061", "compiler.empty_package_name"),
    Self::EmptySourceSet [] => ("E0062", "compiler.empty_source_set"),
    Self::Io(..) => ("E0063", "compiler.io"),
    Self::Package(..) => ("E0064", "compiler.package"),
    Self::OmegaErasurePlan { .. } => ("E0065", "compiler.omega_erasure_plan"),
    Self::MissingTarget { .. } => ("E0066", "compiler.missing_target"),
    Self::AmbiguousManifestTarget { .. } => ("E0067", "compiler.ambiguous_manifest_target"),
    Self::MismatchedPackageIdentity { .. } => ("E0068", "compiler.mismatched_package_identity"),
    Self::TargetFromDifferentPackage { .. } => ("E0069", "compiler.target_from_different_package"),
    Self::MissingStableSymbol { .. } => ("E0070", "compiler.missing_stable_symbol"),
    Self::DuplicateStableSymbol { .. } => ("E0071", "compiler.duplicate_stable_symbol"),
    Self::DuplicateObligationId { .. } => ("E0072", "compiler.duplicate_obligation_id"),
    Self::PackageReferenceOutsidePackage { .. } => ("E0073", "compiler.package_reference_outside_package"),
    Self::MissingClosureMetadata { .. } => ("E0074", "compiler.missing_closure_metadata"),
    Self::CheckedCoreBodyView { .. } => ("E0075", "compiler.checked_core_body_view"),
    Self::EntrypointClosurePackageMismatch { .. } => ("E0076", "compiler.entrypoint_closure_package_mismatch"),
    Self::DuplicateBoundaryHeader { .. } => ("E0077", "compiler.duplicate_boundary_header"),
    ;
    Self::Elaboration(error) => error.code(),
}

// These compiler-driver envelope errors use E; the N domain begins at the
// back-end cause families in increment 2, which this increment does not code.
ken_diagnostic::define_error_codes! {
    impl NativeProgramBuildError;
    table NATIVE_PROGRAM_BUILD_ERROR_TABLE;
    Self::Admission(..) => ("E0078", "native_build.admission"),
    Self::Erasure(..) => ("E0079", "native_build.erasure"),
    Self::Packaging(..) => ("E0080", "native_build.packaging"),
    Self::Unavailable(..) => ("E0081", "native_build.unavailable"),
    ;
    Self::Driver(error) => error.code(),
}

#[cfg(test)]
mod tests {
    use super::*;
    use ken_diagnostic::assert_table_matches_ledger;

    #[test]
    fn front_end_code_tables_match_registry() {
        assert_table_matches_ledger(&["elab."], ELAB_ERROR_TABLE);
        assert_table_matches_ledger(&["compiler."], COMPILER_DRIVER_ERROR_TABLE);
        assert_table_matches_ledger(&["native_build."], NATIVE_PROGRAM_BUILD_ERROR_TABLE);
        assert_table_matches_ledger(&["kernel."], KERNEL_ERROR_TABLE);
    }

    #[test]
    fn wrapper_codes_project_the_underlying_cause() {
        let kernel = KernelError::K2ReservedFormer;
        let projected = ElabError::KernelRejected {
            error: kernel.clone(),
            span: crate::error::Span::zero(),
        };
        assert_eq!(projected.code(), kernel_error_code(&kernel));
        let elab = ElabError::UnboundName {
            name: "absent".to_string(),
            span: crate::error::Span::zero(),
        };
        let compiler = CompilerDriverError::Elaboration(elab);
        let code = compiler.code();
        if let CompilerDriverError::Elaboration(inner) = &compiler {
            assert_eq!(code, inner.code());
        }
        assert_eq!(NativeProgramBuildError::Driver(compiler).code(), code);
    }
}
