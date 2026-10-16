//! Error identities for the interpreter application's run boundary.
//! Host failures are projected here without changing the host's own types.

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

/// Only the three host failures produced by this CLI runner are projected here.
/// Other host errors are not silently assigned an unrelated CLI or host code.
fn terminal_error_code(error: &ken_runtime::TerminalErrorV1) -> Option<ken_diagnostic::Code> {
    use ken_runtime::TerminalErrorV1;
    match error {
        TerminalErrorV1::RootExecutionDenied => Some(ken_diagnostic::Code::new("H0001")),
        TerminalErrorV1::HomeRootResolutionFailed(..) => Some(ken_diagnostic::Code::new("H0002")),
        TerminalErrorV1::MalformedHostAbiField => Some(ken_diagnostic::Code::new("H0003")),
        _ => None,
    }
}

pub(super) fn terminal_error_line(error: &ken_runtime::TerminalErrorV1, message: &str) -> String {
    let code = terminal_error_code(error).expect("CLI host failure must have a projected code");
    ken_diagnostic::render_line(code, message)
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

    #[test]
    fn projected_host_carriers_match_ledger_and_render_one_code() {
        use ken_runtime::{HomeRootResolutionFailureV1, TerminalErrorV1};
        let cases = [
            (
                TerminalErrorV1::RootExecutionDenied,
                "host.root_execution_denied",
                "TerminalErrorV1::RootExecutionDenied",
                "H0001",
            ),
            (
                TerminalErrorV1::HomeRootResolutionFailed(
                    HomeRootResolutionFailureV1::NoAccountRecord,
                ),
                "host.home_root_resolution_failed",
                "TerminalErrorV1::HomeRootResolutionFailed",
                "H0002",
            ),
            (
                TerminalErrorV1::MalformedHostAbiField,
                "host.malformed_host_abi_field",
                "TerminalErrorV1::MalformedHostAbiField",
                "H0003",
            ),
        ];
        let table: Vec<_> = cases
            .iter()
            .map(|(error, label, variant, expected)| {
                let code = terminal_error_code(error).expect("host carrier must be projected");
                assert_eq!(code.as_str(), *expected);
                let line = terminal_error_line(error, "context");
                assert_eq!(line, format!("error[{code}]: context"));
                assert_eq!(line.matches("error[").count(), 1);
                (code, *label, *variant)
            })
            .collect();
        ken_diagnostic::assert_table_matches_ledger(&["host."], &table);
        assert_eq!(terminal_error_code(&TerminalErrorV1::UnknownTree), None);
    }
}
