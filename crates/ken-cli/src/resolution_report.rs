//! Optional CLI report file: exclusively created, removed unless complete.

use std::ffi::{OsStr, OsString};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

use crate::cli_errors::{emit_cli_error, CliError};

pub(crate) struct ReportFile {
    path: PathBuf,
    file: Option<File>,
    complete: bool,
}

impl ReportFile {
    pub(crate) fn create(path: &OsStr) -> Result<Self, CliError> {
        if path.is_empty() {
            return Err(CliError::ResolutionReportCreate(io::Error::new(
                io::ErrorKind::InvalidInput,
                "empty report path",
            )));
        }
        let path = PathBuf::from(path);
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(CliError::ResolutionReportCreate)?;
        Ok(Self {
            path,
            file: Some(file),
            complete: false,
        })
    }

    pub(crate) fn write(&mut self, report: &str) {
        let result = self
            .file
            .as_mut()
            .expect("report written only once")
            .write_all(report.as_bytes())
            .and_then(|()| self.file.as_mut().expect("open report").flush());
        if let Err(error) = result {
            self.abort();
            emit_cli_error(CliError::ResolutionReportWrite(error));
            std::process::exit(1);
        }
        self.complete = true;
    }

    pub(crate) fn abort(&mut self) {
        if !self.complete {
            self.file.take();
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

impl Drop for ReportFile {
    fn drop(&mut self) {
        self.abort();
    }
}

/// Only the one leading `--resolution-report=<path>` token is recognized.
/// A repeated token remains an ordinary positional argument.
pub(crate) fn strip_first(args: &[OsString]) -> (Option<&OsStr>, &[OsString]) {
    let Some(first) = args.first() else {
        return (None, args);
    };
    let Some(bytes) = first
        .to_str()
        .and_then(|s| s.strip_prefix("--resolution-report="))
    else {
        return (None, args);
    };
    (Some(OsStr::new(bytes)), &args[1..])
}
