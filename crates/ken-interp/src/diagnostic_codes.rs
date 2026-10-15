//! Code identity for the interpreter's returned host-tree errors.

use crate::eval::RunIoError;

ken_diagnostic::define_error_codes! {
    impl RunIoError;
    table RUN_IO_ERROR_TABLE;
    Self::UnknownEffect(..) => ("X0001", "run_io.unknown_effect"),
    Self::UnknownTree [] => ("X0002", "run_io.unknown_tree"),
    Self::NotAnIOTree(..) => ("X0003", "run_io.not_an_io_tree"),
    ;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_io_codes_match_registry() {
        ken_diagnostic::assert_table_matches_ledger(&["run_io."], RUN_IO_ERROR_TABLE);
        assert_eq!(
            RunIoError::UnknownTree.code(),
            ken_diagnostic::Code::new("X0002")
        );
    }
}
