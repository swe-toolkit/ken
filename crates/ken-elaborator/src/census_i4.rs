//! Temporary I4 CI census sink. Never merge this measurement-only module.
//!
//! Nextest runs one test per process. The test executable's --exact argument
//! identifies that process; a spawned Ken child inherits its parent test's
//! identity by walking /proc, not by guessing from a worker-thread name.
//! The staged shard artifact is uploaded even when the test step goes red.

use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

thread_local! {
    static THREAD_RECORDS: RefCell<Vec<Value>> = const { RefCell::new(Vec::new()) };
}

#[derive(Clone)]
struct Identity {
    binary: String,
    test: String,
    test_pid: u32,
}

fn proc_test(pid: u32) -> Option<(PathBuf, String)> {
    let cmdline = fs::read(format!("/proc/{pid}/cmdline")).ok()?;
    let args: Vec<_> = cmdline
        .split(|byte| *byte == 0)
        .filter(|part| !part.is_empty())
        .map(|part| String::from_utf8_lossy(part).into_owned())
        .collect();
    // A local libtest invocation can run several filters in one process.
    // It has no process-level test identity; never mislabel all its threads
    // with the first filter. Nextest's one-test-per-process call has one.
    if args
        .iter()
        .skip(1)
        .filter(|arg| !arg.starts_with('-'))
        .count()
        != 1
    {
        return None;
    }
    let test = args
        .windows(2)
        .find_map(|pair| (pair[0] == "--exact").then(|| pair[1].clone()))?;
    let exe = fs::read_link(format!("/proc/{pid}/exe")).ok()?;
    Some((exe, test))
}

fn parent(pid: u32) -> Option<u32> {
    let status = fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    status.lines().find_map(|line| {
        line.strip_prefix("PPid:")
            .and_then(|value| value.trim().parse::<u32>().ok())
    })
}

fn inventory() -> &'static HashMap<PathBuf, String> {
    static INVENTORY: OnceLock<HashMap<PathBuf, String>> = OnceLock::new();
    INVENTORY.get_or_init(|| {
        let path = std::env::var_os("KEN_I4_CENSUS_INVENTORY")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("GITHUB_WORKSPACE")
                    .map(|root| PathBuf::from(root).join("inventory.json"))
            });
        let Some(path) = path else {
            return HashMap::new();
        };
        let Ok(bytes) = fs::read(&path) else {
            return HashMap::new();
        };
        let Ok(parsed) = serde_json::from_slice::<Value>(&bytes) else {
            return HashMap::new();
        };
        parsed["rust-suites"]
            .as_object()
            .into_iter()
            .flat_map(|suites| suites.values())
            .filter_map(|suite| {
                let executable = fs::canonicalize(suite["binary-path"].as_str()?).ok()?;
                let id = suite["binary-id"].as_str()?.to_owned();
                Some((executable, id))
            })
            .collect()
    })
}

fn local_binary(exe: &Path) -> Option<String> {
    let filename = exe.file_name()?.to_str()?;
    let (stem, _) = filename.rsplit_once('-')?;
    let crates = Path::new(env!("CARGO_MANIFEST_DIR")).parent()?;
    for crate_dir in fs::read_dir(crates).ok()?.flatten() {
        let path = crate_dir.path();
        let package = path.file_name()?.to_str()?.to_owned();
        if path.join("tests").join(format!("{stem}.rs")).is_file() {
            return Some(format!("{package}::{stem}"));
        }
        if stem == package.replace('-', "_") {
            return Some(package);
        }
    }
    None
}

fn identity() -> Option<Identity> {
    static IDENTITY: OnceLock<Option<Identity>> = OnceLock::new();
    IDENTITY
        .get_or_init(|| {
            let mut pid = std::process::id();
            for _ in 0..24 {
                if let Some((exe, test)) = proc_test(pid) {
                    let exe = fs::canonicalize(&exe).unwrap_or(exe);
                    let binary = inventory()
                        .get(&exe)
                        .cloned()
                        .or_else(|| local_binary(&exe))?;
                    return Some(Identity {
                        binary,
                        test,
                        test_pid: pid,
                    });
                }
                pid = parent(pid)?;
                if pid <= 1 {
                    break;
                }
            }
            None
        })
        .clone()
}

fn artifact_dir() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("KEN_I4_CENSUS_DIR") {
        return Some(PathBuf::from(path));
    }
    let workspace = PathBuf::from(std::env::var_os("GITHUB_WORKSPACE")?);
    let staged: Vec<_> = fs::read_dir(workspace)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| {
                        name.strip_prefix("realized-shard-").is_some_and(|suffix| {
                            !suffix.is_empty() && suffix.bytes().all(|c| c.is_ascii_digit())
                        })
                    })
        })
        .collect();
    (staged.len() == 1).then(|| staged[0].join("i4-census"))
}

/// One JSON event per call. A sink failure is visible but never changes a
/// resolver result: the only unconditional census panic is the separate CI
/// sentinel (plus the existing missing-finish instrumentation guard).
pub(crate) fn emit(kind: &str, payload: Value) {
    let id = identity();
    let value = json!({
        "kind": kind,
        "binary": id.as_ref().map(|identity| identity.binary.as_str()),
        "test": id.as_ref().map(|identity| identity.test.as_str()),
        "test_pid": id.as_ref().map(|identity| identity.test_pid),
        "pid": std::process::id(),
        "payload": payload,
    });
    THREAD_RECORDS.with(|records| records.borrow_mut().push(value.clone()));
    let line = value.to_string();
    eprintln!("LEGACY_CENSUS_JSON {line}");
    let Some(directory) = artifact_dir() else {
        if std::env::var_os("GITHUB_WORKSPACE").is_some() {
            eprintln!("LEGACY_CENSUS_SINK_ERROR no unique staged shard directory: {line}");
        }
        return;
    };
    let result = fs::create_dir_all(&directory).and_then(|_| {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(directory.join("records.jsonl"))?;
        file.write_all(format!("{line}\n").as_bytes())
    });
    if let Err(error) = result {
        eprintln!("LEGACY_CENSUS_SINK_ERROR {error}: {line}");
    }
}

/// Scratch controls inspect their own production-driven measurements, never
/// manufacture an entry. Records in child processes use the artifact instead.
pub fn current_thread_records() -> Vec<Value> {
    THREAD_RECORDS.with(|records| records.borrow().clone())
}
