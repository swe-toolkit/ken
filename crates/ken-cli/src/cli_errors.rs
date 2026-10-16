//! Failures owned by the command-line front end, rather than a wrapped carrier.

use std::ffi::OsString;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum CliError {
    RunMissingFile,
    RunUnexpectedBeforeSeparator(OsString),
    UnknownSubcommand(String),
    NativeBuildMissingFile,
    NativeBuildMissingOutputDir,
    NativeBuildMissingResourceProfile,
    NativeBuildUnexpectedArgument,
    NativeBuildResourceProfileRead(io::Error),
    NativeBuildInvalidResourceProfile(String),
    NativeBuildSourceRead(PathBuf, io::Error),
    FmtPathNotUtf8(OsString),
    FmtUnknownOption(String),
    FmtMissingPaths,
    FmtPathRead(PathBuf, io::Error),
    FmtUnsupportedPath(PathBuf),
    FmtNonCanonical(PathBuf),
    FmtPathWrite(PathBuf, io::Error),
    CheckMissingFile,
    CheckSourceRead(PathBuf, io::Error),
    RunSourceRead(PathBuf, io::Error),
    RunWorkingDirectoryRead(io::Error),
}

ken_diagnostic::define_error_codes! {
    impl CliError;
    table CLI_ERROR_TABLE;
    Self::RunMissingFile [] => ("C0001", "cli.run_missing_file"),
    Self::RunUnexpectedBeforeSeparator(..) => ("C0002", "cli.run_unexpected_before_separator"),
    Self::UnknownSubcommand(..) => ("C0003", "cli.unknown_subcommand"),
    Self::NativeBuildMissingFile [] => ("C0004", "cli.native_build_missing_file"),
    Self::NativeBuildMissingOutputDir [] => ("C0005", "cli.native_build_missing_output_dir"),
    Self::NativeBuildMissingResourceProfile [] => ("C0006", "cli.native_build_missing_resource_profile"),
    Self::NativeBuildUnexpectedArgument [] => ("C0007", "cli.native_build_unexpected_argument"),
    Self::NativeBuildResourceProfileRead(..) => ("C0008", "cli.native_build_resource_profile_read"),
    Self::NativeBuildInvalidResourceProfile(..) => ("C0009", "cli.native_build_invalid_resource_profile"),
    Self::NativeBuildSourceRead(..) => ("C0010", "cli.native_build_source_read"),
    Self::FmtPathNotUtf8(..) => ("C0011", "cli.fmt_path_not_utf8"),
    Self::FmtUnknownOption(..) => ("C0012", "cli.fmt_unknown_option"),
    Self::FmtMissingPaths [] => ("C0013", "cli.fmt_missing_paths"),
    Self::FmtPathRead(..) => ("C0014", "cli.fmt_path_read"),
    Self::FmtUnsupportedPath(..) => ("C0015", "cli.fmt_unsupported_path"),
    Self::FmtNonCanonical(..) => ("C0016", "cli.fmt_non_canonical"),
    Self::FmtPathWrite(..) => ("C0017", "cli.fmt_path_write"),
    Self::CheckMissingFile [] => ("C0018", "cli.check_missing_file"),
    Self::CheckSourceRead(..) => ("C0019", "cli.check_source_read"),
    Self::RunSourceRead(..) => ("C0020", "cli.run_source_read"),
    Self::RunWorkingDirectoryRead(..) => ("C0021", "cli.run_working_directory_read"),
    ;
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RunMissingFile => write!(f, "ken run: missing <file> argument"),
            Self::RunUnexpectedBeforeSeparator(argument) => {
                write!(f, "ken run: unexpected argument before '--': {argument:?}")
            }
            Self::UnknownSubcommand(command) => {
                write!(f, "ken: unknown subcommand '{command}' — try 'ken help'")
            }
            Self::NativeBuildMissingFile => write!(f, "ken native-build: missing <file> argument"),
            Self::NativeBuildMissingOutputDir => {
                write!(f, "ken native-build: missing <output-dir> argument")
            }
            Self::NativeBuildMissingResourceProfile => {
                write!(
                    f,
                    "ken native-build: missing <resource-profile.json> argument"
                )
            }
            Self::NativeBuildUnexpectedArgument => {
                write!(
                    f,
                    "ken native-build: unexpected argument after resource profile"
                )
            }
            Self::NativeBuildResourceProfileRead(error) => {
                write!(f, "ken native-build: cannot read resource profile: {error}")
            }
            Self::NativeBuildInvalidResourceProfile(error) => {
                write!(f, "ken native-build: invalid resource profile: {error}")
            }
            Self::NativeBuildSourceRead(path, error) => {
                write!(
                    f,
                    "ken native-build: cannot read '{}': {error}",
                    path.display()
                )
            }
            Self::FmtPathNotUtf8(path) => write!(f, "ken fmt: path is not valid UTF-8: {path:?}"),
            Self::FmtUnknownOption(option) => write!(f, "ken fmt: unknown option '{option}'"),
            Self::FmtMissingPaths => write!(f, "ken fmt: missing <paths...> argument"),
            Self::FmtPathRead(path, error) => {
                write!(f, "ken fmt: cannot read '{}': {error}", path.display())
            }
            Self::FmtUnsupportedPath(path) => write!(
                f,
                "ken fmt: unsupported path '{}' (expected .ken or .ken.md)",
                path.display()
            ),
            Self::FmtNonCanonical(path) => {
                write!(f, "ken fmt --check: non-canonical: {}", path.display())
            }
            Self::FmtPathWrite(path, error) => {
                write!(f, "ken fmt: cannot write '{}': {error}", path.display())
            }
            Self::CheckMissingFile => write!(f, "ken check: missing <file> argument"),
            Self::CheckSourceRead(path, error) => {
                write!(f, "ken check: cannot read '{}': {error}", path.display())
            }
            Self::RunSourceRead(path, error) => {
                write!(f, "ken run: cannot read '{}': {error}", path.display())
            }
            Self::RunWorkingDirectoryRead(error) => {
                write!(f, "ken run: cannot read working directory: {error}")
            }
        }
    }
}

pub(crate) fn emit_coded_error(code: ken_diagnostic::Code, message: impl fmt::Display) {
    eprintln!("{}", ken_diagnostic::render_line(code, message));
}

pub(crate) fn emit_cli_error(error: CliError) {
    emit_coded_error(error.code(), error);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_causes_match_the_single_ledger() {
        ken_diagnostic::assert_table_matches_ledger(&["cli."], CLI_ERROR_TABLE);
        assert_eq!(CliError::FmtMissingPaths.code().as_str(), "C0013");
        assert_eq!(CliError::RunMissingFile.code().as_str(), "C0001");
        assert_eq!(CliError::CheckMissingFile.code().as_str(), "C0018");
        assert_eq!(
            CliError::FmtPathWrite(
                PathBuf::from("output.ken"),
                io::Error::from(io::ErrorKind::PermissionDenied),
            )
            .code()
            .as_str(),
            "C0017"
        );
        assert_eq!(
            CliError::RunWorkingDirectoryRead(io::Error::from(io::ErrorKind::NotFound))
                .code()
                .as_str(),
            "C0021"
        );
    }
}
