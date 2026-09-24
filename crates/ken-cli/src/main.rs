//! The `ken` command-line driver.

mod repl;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

fn main() {
    // `RT-MATCH-RECURSOR-CONSUMERS` 4a.1. The census scope wraps the whole
    // dispatch, and `dispatch` RETURNS its exit code instead of taking the exit
    // itself, so the envelope is written after the compilation attempt and
    // before this process converts that result to an exit -- on the refusing
    // path as well as the succeeding one. A refused compile is precisely the
    // case `AC-1` cares about, so losing its rows to an early `exit` would
    // hollow out the census.
    //
    // With no session in the environment, or with the feature off, this is a
    // direct call to `dispatch` and nothing else happens.
    let exit_code = ken_runtime::with_child_match_recursor_census(dispatch);
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}

fn dispatch() -> i32 {
    let args: Vec<OsString> = std::env::args_os().collect();
    match args.get(1).and_then(|s| s.to_str()).unwrap_or("") {
        "repl" => {
            repl::run();
            0
        }
        "run" => match parse_run_invocation(&args[2..]) {
            Ok(invocation) => {
                run_file(invocation.path.as_os_str(), &invocation.arguments);
                0
            }
            Err(RunArgumentError::MissingPath) => {
                eprintln!("ken run: missing <file> argument");
                eprintln!("Usage: ken run <file.ken> [-- <arguments>...]");
                1
            }
            Err(RunArgumentError::UnexpectedBeforeSeparator(argument)) => {
                eprintln!("ken run: unexpected argument before '--': {:?}", argument);
                1
            }
        },
        "check" => {
            check_file(args.get(2).map(OsString::as_os_str));
            0
        }
        "native-build" => native_build_file(
            args.get(2).map(OsString::as_os_str),
            args.get(3).map(OsString::as_os_str),
            args.get(4).map(OsString::as_os_str),
            args.get(5).is_some(),
        ),
        "fmt" => {
            format_files(&args[2..]);
            0
        }
        "version" | "--version" | "-V" => {
            println!(
                "ken {} — verified topos-oriented language",
                env!("CARGO_PKG_VERSION")
            );
            println!("kernel {}", ken_kernel::version());
            println!("{}", ken_interp::describe());
            0
        }
        "" | "--help" | "-h" | "help" => {
            print_help();
            0
        }
        unknown => {
            eprintln!("ken: unknown subcommand '{}' — try 'ken help'", unknown);
            1
        }
    }
}

/// Returns the process exit code rather than taking the exit, so that 4a.1's
/// envelope is written while this result is still a value. See `main`.
fn parse_native_resource_profile(
    bytes: &[u8],
) -> Result<ken_runtime::boundary_resource_profile::BoundaryResourceProfileV3, String> {
    use ken_runtime::boundary_resource_profile::{
        BoundaryRegionLimitsV1, BoundaryResourceProfileV3, InvocationCallLimitsV3,
        RuntimeResourceLimitsV2,
    };
    let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    let object = value.as_object().ok_or("profile must be a JSON object")?;
    let keys = ["runtime", "call_events", "invocation", "persistent"];
    if object.len() != keys.len() || keys.iter().any(|key| !object.contains_key(*key)) {
        return Err("profile must name exactly runtime, call_events, invocation, persistent".into());
    }
    let region = |name: &str| -> Result<BoundaryRegionLimitsV1, String> {
        let fields = object[name]
            .as_object()
            .ok_or(format!("{name} must be an object"))?;
        let names = ["nodes", "words", "data_bytes", "native_int_limbs"];
        if fields.len() != names.len() || names.iter().any(|key| !fields.contains_key(*key)) {
            return Err(format!(
                "{name} must name exactly nodes, words, data_bytes, native_int_limbs"
            ));
        }
        let get = |field: &str| -> Result<usize, String> {
            let n = fields[field]
                .as_u64()
                .ok_or(format!("{name}.{field} must be a nonnegative integer"))?;
            usize::try_from(n).map_err(|_| format!("{name}.{field} exceeds the host address space"))
        };
        Ok(BoundaryRegionLimitsV1 {
            nodes: get("nodes")?,
            words: get("words")?,
            data_bytes: get("data_bytes")?,
            native_int_limbs: get("native_int_limbs")?,
        })
    };
    let runtime = object["runtime"]
        .as_object()
        .ok_or("runtime must be an object")?;
    if runtime.len() != 1 || !runtime.contains_key("invocation_epochs") {
        return Err("runtime must name exactly invocation_epochs".into());
    }
    let invocation_epochs = runtime["invocation_epochs"]
        .as_u64()
        .ok_or("runtime.invocation_epochs must be a nonnegative integer")?;
    let calls = object["call_events"].as_object().ok_or("call_events must be an object")?;
    if calls.len() != 2 || !calls.contains_key("event_generations") || !calls.contains_key("live_pending_slots") {
        return Err("call_events must name exactly event_generations, live_pending_slots".into());
    }
    let event_generations = calls["event_generations"].as_u64()
        .ok_or("call_events.event_generations must be a nonnegative integer")?;
    let live_pending_slots = calls["live_pending_slots"].as_u64()
        .ok_or("call_events.live_pending_slots must be a nonnegative integer")?;
    let live_pending_slots = usize::try_from(live_pending_slots)
        .map_err(|_| "call_events.live_pending_slots exceeds the host address space")?;
    Ok(BoundaryResourceProfileV3 {
        runtime: RuntimeResourceLimitsV2 { invocation_epochs },
        call_events: InvocationCallLimitsV3 { event_generations, live_pending_slots },
        invocation: region("invocation")?,
        persistent: region("persistent")?,
    })
}

#[cfg(test)]
mod capacity_profile_tests {
    use super::*;

    #[test]
    fn native_profile_requires_all_eleven_explicit_numbers_and_admits_zero() {
        let valid = br#"{"runtime":{"invocation_epochs":0},"call_events":{"event_generations":0,"live_pending_slots":0},"invocation":{"nodes":1,"words":2,"data_bytes":3,"native_int_limbs":4},"persistent":{"nodes":5,"words":6,"data_bytes":7,"native_int_limbs":8}}"#;
        let profile = parse_native_resource_profile(valid).expect("zero is explicit, not absent");
        assert_eq!(profile.runtime.invocation_epochs, 0);
        assert_eq!(profile.call_events.event_generations, 0);
        assert_eq!(profile.call_events.live_pending_slots, 0);
        assert_eq!(profile.invocation.nodes, 1);
        assert_eq!(profile.persistent.native_int_limbs, 8);
        let missing = br#"{"runtime":{},"call_events":{"event_generations":0,"live_pending_slots":0},"invocation":{"nodes":1,"words":2,"data_bytes":3,"native_int_limbs":4},"persistent":{"nodes":5,"words":6,"data_bytes":7,"native_int_limbs":8}}"#;
        assert!(parse_native_resource_profile(missing).unwrap_err().contains("invocation_epochs"));
        let extra = br#"{"runtime":{"invocation_epochs":0,"pending_slots":1},"call_events":{"event_generations":0,"live_pending_slots":0},"invocation":{"nodes":1,"words":2,"data_bytes":3,"native_int_limbs":4},"persistent":{"nodes":5,"words":6,"data_bytes":7,"native_int_limbs":8}}"#;
        let missing_calls = br#"{"runtime":{"invocation_epochs":0},"call_events":{"live_pending_slots":0},"invocation":{"nodes":1,"words":2,"data_bytes":3,"native_int_limbs":4},"persistent":{"nodes":5,"words":6,"data_bytes":7,"native_int_limbs":8}}"#;
        assert!(parse_native_resource_profile(missing_calls).unwrap_err().contains("event_generations"));
        assert!(parse_native_resource_profile(extra).unwrap_err().contains("runtime"));
    }
}

fn native_build_file(
    path: Option<&OsStr>,
    output_dir: Option<&OsStr>,
    profile_path: Option<&OsStr>,
    extra_argument: bool,
) -> i32 {
    let Some(path) = path else {
        eprintln!("ken native-build: missing <file> argument");
        eprintln!("Usage: ken native-build <file.ken> <output-dir> <resource-profile.json>");
        std::process::exit(1);
    };
    let Some(output_dir) = output_dir else {
        eprintln!("ken native-build: missing <output-dir> argument");
        eprintln!("Usage: ken native-build <file.ken> <output-dir> <resource-profile.json>");
        return 1;
    };
    let Some(profile_path) = profile_path else {
        eprintln!("ken native-build: missing <resource-profile.json> argument");
        return 1;
    };
    if extra_argument {
        eprintln!("ken native-build: unexpected argument after resource profile");
        return 1;
    }
    let profile_bytes = match std::fs::read(profile_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("ken native-build: cannot read resource profile: {error}");
            return 1;
        }
    };
    let profile = match parse_native_resource_profile(&profile_bytes) {
        Ok(profile) => profile,
        Err(error) => {
            eprintln!("ken native-build: invalid resource profile: {error}");
            return 1;
        }
    };
    let source = std::fs::read_to_string(path).unwrap_or_else(|error| {
        eprintln!(
            "ken native-build: cannot read '{}': {error}",
            path.to_string_lossy()
        );
        std::process::exit(1);
    });
    let format = if path.to_string_lossy().ends_with(".ken.md") {
        ken_cli::SourceFormat::LiterateKen
    } else {
        ken_cli::SourceFormat::Ken
    };
    match ken_cli::build_native_program(
        &source,
        format,
        "native-program",
        PathBuf::from(output_dir),
        profile,
    ) {
        Ok(output) => {
            println!("{}", output.artifact.executable_path.display());
            0
        }
        Err(error) => {
            eprintln!("ken native-build: {error}");
            1
        }
    }
}

/// `ken fmt [--check] <paths...>` — the thin CLI over the landed formatter.
fn format_files(args: &[OsString]) {
    let mut check = false;
    let mut paths = Vec::new();
    for arg in args {
        let Some(arg) = arg.to_str() else {
            eprintln!("ken fmt: path is not valid UTF-8: {:?}", arg);
            std::process::exit(1);
        };
        if arg == "--check" {
            check = true;
        } else if arg.starts_with('-') {
            eprintln!("ken fmt: unknown option '{arg}'");
            std::process::exit(1);
        } else {
            paths.push(arg);
        }
    }
    if paths.is_empty() {
        eprintln!("ken fmt: missing <paths...> argument");
        eprintln!("Usage: ken fmt [--check] <paths...>");
        std::process::exit(1);
    }

    let mut failed = false;
    for path in paths {
        let source = match std::fs::read_to_string(path) {
            Ok(source) => source,
            Err(error) => {
                eprintln!("ken fmt: cannot read '{path}': {error}");
                failed = true;
                continue;
            }
        };
        let formatted = if path.ends_with(".ken.md") {
            ken_elaborator::format_ken_md(&source)
        } else if path.ends_with(".ken") {
            ken_elaborator::layout::format_ken(&source)
        } else {
            eprintln!("ken fmt: unsupported path '{path}' (expected .ken or .ken.md)");
            failed = true;
            continue;
        };
        let formatted = match formatted {
            Ok(formatted) => formatted,
            Err(error) => {
                eprintln!("ken fmt: formatting error in '{path}': {error:?}");
                failed = true;
                continue;
            }
        };

        if check {
            if formatted != source {
                eprintln!("ken fmt --check: non-canonical: {path}");
                failed = true;
            }
        } else if formatted != source {
            if let Err(error) = std::fs::write(path, formatted) {
                eprintln!("ken fmt: cannot write '{path}': {error}");
                failed = true;
            }
        }
    }

    if failed {
        std::process::exit(1);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RunArgumentError {
    MissingPath,
    UnexpectedBeforeSeparator(OsString),
}

struct RunInvocation {
    path: PathBuf,
    arguments: Vec<Vec<u8>>,
}

fn parse_run_invocation(args: &[OsString]) -> Result<RunInvocation, RunArgumentError> {
    let Some(path) = args.first() else {
        return Err(RunArgumentError::MissingPath);
    };
    let rest = &args[1..];
    let program_args = match rest.first() {
        None => &[][..],
        Some(separator) if separator == "--" => &rest[1..],
        Some(unexpected) => {
            return Err(RunArgumentError::UnexpectedBeforeSeparator(
                unexpected.clone(),
            ));
        }
    };
    Ok(RunInvocation {
        path: PathBuf::from(path),
        arguments: program_args.iter().map(|arg| os_bytes(arg)).collect(),
    })
}

#[cfg(unix)]
fn os_bytes(value: &OsStr) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    value.as_bytes().to_vec()
}

#[cfg(not(unix))]
fn os_bytes(value: &OsStr) -> Vec<u8> {
    value.to_string_lossy().into_owned().into_bytes()
}

/// Read `<file>` and elaborate it. A path with a derivable `catalog/packages`
/// module address uses the roots loader and then executes only that entry's
/// checked-fence roles; every other path retains the direct `.ken.md`/`.ken`
/// dispatch. Exits 1 on a missing argument, an unreadable file, elaborator init
/// failure, or an elaboration error, with a message prefixed by `cmd` so a user
/// sees the subcommand they actually typed, not a borrowed one.
fn elaborate_cli_file(
    cmd: &str,
    path: Option<&OsStr>,
) -> (PathBuf, ken_elaborator::ElabEnv, Vec<ken_kernel::GlobalId>) {
    let path = match path {
        Some(p) => p,
        None => {
            eprintln!("ken {cmd}: missing <file> argument");
            eprintln!("Usage: ken {cmd} <file.ken>");
            std::process::exit(1);
        }
    };

    let src = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ken {cmd}: cannot read '{}': {}", path.to_string_lossy(), e);
            std::process::exit(1);
        }
    };

    let mut elab_env = match ken_elaborator::ElabEnv::new() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("ken {cmd}: elaborator init failed: {:?}", e);
            std::process::exit(1);
        }
    };

    let catalog_module = ken_elaborator::modules::catalog_module_from_path(Path::new(path));
    let ids_result = if let Some(catalog_module) = catalog_module {
        let roots = [catalog_module.root];
        elab_env
            .elaborate_module_from_roots(&roots, &catalog_module.entry)
            .and_then(|ids| {
                elab_env.execute_loaded_entry_checked_fences(&catalog_module.entry)?;
                Ok(ids)
            })
    } else if path.to_string_lossy().ends_with(".ken.md") {
        elab_env.elaborate_ken_md_file(&src)
    } else {
        elab_env.elaborate_file(&src)
    };

    let ids = match ids_result {
        Ok(ids) => ids,
        Err(ken_elaborator::ElabError::DuplicateDefinition { name, .. })
            if cmd == "run" && name == "main" =>
        {
            eprintln!("ken run: duplicate entrypoint 'main'");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!(
                "ken {cmd}: elaboration error in '{}': {:?}",
                path.to_string_lossy(),
                e
            );
            std::process::exit(1);
        }
    };

    (PathBuf::from(path), elab_env, ids)
}

/// `ken check <file>` — FR-3 (`docs/program/wp/ds-1-findings-remediation.md`):
/// a library check-mode for pure-library catalog entries, which have no
/// natural IO `main` and so cannot satisfy `ken run`'s literal exit-0
/// contract. Catalog-addressed files load their import closure through the
/// roots API; other paths retain the direct isolated-file dispatch. The entry's
/// existing fence-role verdicts (`ken reject` must fail, `ken example` must
/// elaborate) run after roots loading; dependency document roles do not. Exits
/// 0 iff elaboration + every entry fence behaved and inherits the shared front
/// half's `Err -> exit 1` verbatim. Never drives IO, so a
/// runnable program's `main` is simply never executed here (`ken run` is
/// still how you run it) — `ken run` itself is unchanged, strict, and has no
/// auto-detect fallthrough to this mode.
fn check_file(path: Option<&OsStr>) {
    elaborate_cli_file("check", path);
}

/// `ken run <file>` — elaborate, evaluate, and drive a Console IO program.
///
/// Elaborates every declaration in `<file>`, resolves the ABI-shaped `main` by
/// name, supplies process input and capabilities, and drives its host tree.
///
/// Console IDs are harvested from the elaboration environment (`ElabEnv::globals`).
fn run_file(path: &OsStr, arguments: &[Vec<u8>]) {
    let source = std::fs::read_to_string(path).unwrap_or_else(|error| {
        eprintln!("ken run: cannot read '{}': {error}", path.to_string_lossy());
        std::process::exit(1);
    });
    let format = if path.to_string_lossy().ends_with(".ken.md") {
        ken_cli::SourceFormat::LiterateKen
    } else {
        ken_cli::SourceFormat::Ken
    };
    let environment: Vec<_> = std::env::vars_os()
        .map(|(key, value)| (os_bytes(&key), os_bytes(&value)))
        .collect();
    let cwd = std::env::current_dir().unwrap_or_else(|error| {
        eprintln!("ken run: cannot read working directory: {error}");
        std::process::exit(1);
    });
    let mut host = ken_interp::PosixHost::new();
    match ken_cli::run_program(
        &source,
        format,
        arguments,
        &environment,
        &os_bytes(cwd.as_os_str()),
        &mut host,
    ) {
        Ok(outcome) => std::process::exit(outcome.exit_status),
        Err(ken_cli::RunError::DuplicateEntrypoint) => {
            eprintln!("ken run: duplicate entrypoint 'main'");
            std::process::exit(1);
        }
        Err(ken_cli::RunError::MissingEntrypoint) => {
            eprintln!(
                "ken run: missing entrypoint 'main' in '{}'",
                path.to_string_lossy()
            );
            std::process::exit(1);
        }
        Err(ken_cli::RunError::EntrypointAbiUnavailable) => {
            eprintln!("ken run: entrypoint ABI declarations are unavailable");
            std::process::exit(2);
        }
        Err(ken_cli::RunError::MissingCapability { effect }) => {
            eprintln!("ken run: MissingCapability {{ effect = {effect} }}");
            std::process::exit(1);
        }
        Err(ken_cli::RunError::InvalidEntrypoint { authority }) => {
            eprintln!(
                "ken run: invalid entrypoint 'main': expected ProcessInput -> \
                 ProgramCaps {authority} -> HostIO {authority} ExitCode"
            );
            std::process::exit(1);
        }
        Err(ken_cli::RunError::ConsoleAbiUnavailable) => {
            eprintln!("ken run: Console ABI declarations are unavailable");
            std::process::exit(2);
        }
        Err(ken_cli::RunError::RootExecutionObservationUnavailable) => {
            eprintln!("ken run: effective-UID observation is unavailable");
            std::process::exit(1);
        }
        Err(ken_cli::RunError::CapabilityRoot(error)) => {
            eprintln!("ken run: filesystem capability root unavailable: {error}");
            std::process::exit(1);
        }
        Err(ken_cli::RunError::Initialization(error)) => {
            eprintln!("ken run: elaborator init failed: {error:?}");
            std::process::exit(1);
        }
        Err(ken_cli::RunError::Elaboration(error)) => {
            eprintln!(
                "ken run: elaboration error in '{}': {error:?}",
                path.to_string_lossy()
            );
            std::process::exit(1);
        }
        Err(ken_cli::RunError::Io(ken_interp::RunIoError::UnknownTree)) => {
            eprintln!("ken run: program evaluated to an open hole (Unknown)");
            std::process::exit(1);
        }
        Err(ken_cli::RunError::Io(ken_interp::RunIoError::UnknownEffect(value))) => {
            eprintln!("ken run: unhandled effect: {value:?}");
            std::process::exit(1);
        }
        Err(ken_cli::RunError::Io(ken_interp::RunIoError::NotAnIOTree(value))) => {
            eprintln!("ken run: entrypoint did not return an IO tree: {value:?}");
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!(
        "ken {} — verified topos-oriented language",
        env!("CARGO_PKG_VERSION")
    );
    println!();
    println!("Usage: ken <subcommand>");
    println!();
    println!("Subcommands:");
    println!("  run <file>    Elaborate and run a Ken source file (Console IO)");
    println!("  check <file>  Elaborate a Ken source file and verify its fences,");
    println!("                without driving IO (for pure-library entries)");
    println!("  native-build <file> <output-dir> <resource-profile.json>");
    println!("                Build the checked Program I main as a native artifact");
    println!("  fmt [--check] <paths...>");
    println!("                Canonicalize Ken source, or check without writing");
    println!("  repl          Start the interactive REPL (the Little Prover loop)");
    println!("  version       Print version and kernel information");
    println!("  help          Print this message");
}

#[cfg(test)]
mod run_argument_tests {
    use std::ffi::OsString;

    use super::*;

    #[test]
    fn unknown_argument_before_separator_is_a_specific_error() {
        let args = vec![OsString::from("app.ken"), OsString::from("--bad")];
        assert_eq!(
            parse_run_invocation(&args).map(|_| ()),
            Err(RunArgumentError::UnexpectedBeforeSeparator(OsString::from(
                "--bad"
            )))
        );
    }

    #[cfg(unix)]
    #[test]
    fn program_arguments_after_separator_preserve_non_utf8_bytes() {
        use std::os::unix::ffi::OsStringExt;

        let raw = vec![0xff, 0x00, b'a'];
        let args = vec![
            OsString::from("app.ken"),
            OsString::from("--"),
            OsString::from_vec(raw.clone()),
        ];
        let invocation = parse_run_invocation(&args).expect("valid invocation");
        assert_eq!(invocation.arguments, vec![raw]);
    }
}
