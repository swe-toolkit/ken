//! Error identities for the interpreter application's run boundary.
//! This increment does not change any rendered CLI output.

use crate::RunError;

ken_diagnostic::define_error_codes! {
    impl RunError;
    table RUN_ERROR_TABLE;
    Self::DuplicateEntrypoint [] => ("X0004", "run.duplicate_entrypoint"),
    Self::MissingEntrypoint [] => ("X0005", "run.missing_entrypoint"),
    Self::EntrypointAbiUnavailable [] => ("X0006", "run.entrypoint_abi_unavailable"),
    Self::MissingCapability { .. } => ("X0007", "run.missing_capability"),
    Self::InvalidEntrypoint { .. } => ("X0008", "run.invalid_entrypoint"),
    Self::ConsoleAbiUnavailable [] => ("X0009", "run.console_abi_unavailable"),
    Self::RootExecutionObservationUnavailable [] => ("X0010", "run.root_execution_observation_unavailable"),
    Self::CapabilityRoot(..) => ("X0011", "run.capability_root"),
    ;
    Self::Initialization(error) => error.code(),
    Self::Elaboration(error) => error.code(),
    Self::Io(error) => error.code(),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn runner_codes_match_registry_and_wrappers_delegate() {
        ken_diagnostic::assert_table_matches_ledger(&["run."], RUN_ERROR_TABLE);
        let elab = ken_elaborator::ElabError::UnboundName {
            name: "absent".to_string(),
            span: ken_elaborator::Span::zero(),
        };
        let code = elab.code();
        assert_eq!(RunError::Initialization(elab).code(), code);
        let elab = ken_elaborator::ElabError::UnboundName {
            name: "absent".to_string(),
            span: ken_elaborator::Span::zero(),
        };
        assert_eq!(RunError::Elaboration(elab).code(), code);
        let io = ken_interp::RunIoError::UnknownTree;
        let code = io.code();
        assert_eq!(RunError::Io(io).code(), code);
        assert_eq!(
            RunError::MissingEntrypoint.code(),
            ken_diagnostic::Code::new("X0005")
        );
    }
}
