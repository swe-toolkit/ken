//! Artifact identity for the opt-in D5 scalar-merge observation.
//!
//! MEASURED: two independently compiled `ken-cli` test workers emit the same
//! checked source into separate native-output directories. One build uses the
//! package's natural feature-off state; the other explicitly enables
//! `dasm-c2-observation`. Each worker asserts which configuration Cargo built,
//! and the feature-on worker proves that the existing observer receives rows.
//!
//! CLAIMED: enabling the observation leaves the emitted `ken-entrypoint.o`
//! byte-identical. This settles native emission only.
//!
//! THE GAP: byte identity does not measure timing or allocation. The nested
//! builds also require separate Cargo target directories: sharing one would let
//! Cargo feature unification turn the A/B into two reads of one configuration.

use std::path::{Path, PathBuf};
use std::process::Command;

const ARTIFACT_OUTPUT: &str = "KEN_DASM_C2_ARTIFACT_OUTPUT";
const EXPECTED_CONFIGURATION: &str = "KEN_DASM_C2_EXPECTED_CONFIGURATION";

const IDENTITY_SOURCE: &str = r#"program capabilities FS APartial
fn exit_for_input (input : ProcessInput) : ExitCode = match input {
  MkProcessInput arguments _environment _cwd |-> match arguments {
    Nil |-> Success;
    Cons _ _ |-> Failure 7
  }
}
fn main (input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode = host_exit APartial (exit_for_input input)
"#;

fn run_with_big_stack<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(f)
        .expect("spawn big-stack native-compilation worker")
        .join()
        .expect("native-compilation worker panicked");
}

#[test]
fn dasm_c2_artifact_identity_worker() {
    // An ordinary targeted test leaves the artifact output unset and
    // intentionally returns green after measuring nothing; only the outer
    // driver supplies the path and reads the emitted object back.
    let Some(output_dir) = std::env::var_os(ARTIFACT_OUTPUT).map(PathBuf::from) else {
        return;
    };
    run_with_big_stack(move || dasm_c2_artifact_identity_worker_impl(&output_dir));
}

fn dasm_c2_artifact_identity_worker_impl(output_dir: &Path) {
    let expected = std::env::var(EXPECTED_CONFIGURATION)
        .expect("the outer driver states the requested observation configuration");
    let local_observation_compiled = cfg!(feature = "dasm-c2-observation");
    // Read Runtime's own compiled fact rather than treating this crate's cfg as
    // authority: dependency feature unification can enable Runtime without
    // defining the forwarding feature in this crate.
    assert_eq!(
        ken_runtime::DASM_C2_OBSERVATION_COMPILED,
        local_observation_compiled,
        "ken-runtime and ken-cli disagree on the D5 observation configuration"
    );
    let actual = if local_observation_compiled {
        "enabled"
    } else {
        "disabled"
    };
    assert_eq!(
        actual, expected,
        "nested build did not compile the requested D5 observation configuration"
    );

    let compile = || {
        ken_cli::build_native_program(
            IDENTITY_SOURCE,
            ken_cli::SourceFormat::Ken,
            "dasm_c2_artifact_identity_pkg",
            output_dir,
        )
    };

    #[cfg(feature = "dasm-c2-observation")]
    let output = {
        let observation = ken_runtime::dasm_c2_scalar_merge_observation_scope();
        let output = compile();
        let rows = observation.finish();
        assert!(
            !rows.is_empty(),
            "the enabled worker must reach the existing scalar-merge observer"
        );
        output
    };
    #[cfg(not(feature = "dasm-c2-observation"))]
    let output = compile();

    output.expect("the identity-control source emits its native object artifact");
    assert!(
        output_dir.join("ken-entrypoint.o").is_file(),
        "the worker must leave the emitted object at the packaging path"
    );
}

fn compile_artifact(target_dir: &Path, output_dir: &Path, observation_enabled: bool) -> Vec<u8> {
    std::fs::create_dir_all(target_dir).expect("per-configuration Cargo target directory");
    std::fs::create_dir_all(output_dir).expect("per-configuration native output directory");

    // The outer invocation already holds scripts/ken-cargo's machine-wide
    // lock. Calling that wrapper recursively would deadlock, so the worker uses
    // Cargo directly and isolates each build with its own target directory.
    let mut command = Command::new(env!("CARGO"));
    command
        .arg("test")
        .arg("--manifest-path")
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .arg("--test")
        .arg("dasm_c2_observation_artifact_identity")
        .arg("--target-dir")
        .arg(target_dir)
        .env(ARTIFACT_OUTPUT, output_dir)
        .env(
            EXPECTED_CONFIGURATION,
            if observation_enabled {
                "enabled"
            } else {
                "disabled"
            },
        );
    if observation_enabled {
        command.arg("--features").arg("dasm-c2-observation");
    }
    command
        .arg("dasm_c2_artifact_identity_worker")
        .arg("--")
        .arg("--exact")
        .arg("--nocapture");

    let output = command.output().expect("nested Cargo compilation runs");
    assert!(
        output.status.success(),
        "nested {} compilation failed:\nstdout:\n{}\nstderr:\n{}",
        if observation_enabled {
            "feature-on"
        } else {
            "feature-off"
        },
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    std::fs::read(output_dir.join("ken-entrypoint.o"))
        .expect("the nested compilation leaves the actual native object artifact")
}

fn first_difference(left: &[u8], right: &[u8]) -> Option<usize> {
    left.iter()
        .zip(right)
        .position(|(left, right)| left != right)
        .or_else(|| (left.len() != right.len()).then(|| left.len().min(right.len())))
}

#[test]
fn dasm_c2_observation_is_native_artifact_identical() {
    let root = tempfile::Builder::new()
        .prefix("dasm-c2-observation-identity-")
        .tempdir_in(env!("CARGO_TARGET_TMPDIR"))
        .expect("create Cargo-owned feature-identity scratch");
    let root_path = root.path().to_owned();
    let off_target = root.path().join("feature-off-target");
    let on_target = root.path().join("feature-on-target");
    let off_output = root.path().join("feature-off-artifact");
    let on_output = root.path().join("feature-on-artifact");
    let off_artifact = off_output.join("ken-entrypoint.o");
    let on_artifact = on_output.join("ken-entrypoint.o");

    println!("DASM-C2-OFF-TARGET\t{}", off_target.display());
    println!("DASM-C2-ON-TARGET\t{}", on_target.display());
    println!("DASM-C2-OFF-ARTIFACT\t{}", off_artifact.display());
    println!("DASM-C2-ON-ARTIFACT\t{}", on_artifact.display());

    let off_bytes = compile_artifact(&off_target, &off_output, false);
    let on_bytes = compile_artifact(&on_target, &on_output, true);

    let identity = std::panic::catch_unwind(|| {
        assert!(
            !off_bytes.is_empty() && !on_bytes.is_empty(),
            "the identity relation must compare two emitted objects, not empty buffers"
        );
        assert!(
            off_bytes == on_bytes,
            "feature-off artifact {} ({} bytes) and feature-on artifact {} ({} bytes) differ; \
             first differing byte: {:?}",
            off_artifact.display(),
            off_bytes.len(),
            on_artifact.display(),
            on_bytes.len(),
            first_difference(&off_bytes, &on_bytes),
        );
    });
    if let Err(failure) = identity {
        let preserved = root.keep();
        eprintln!(
            "feature-identity artifacts preserved at {}",
            preserved.display()
        );
        std::panic::resume_unwind(failure);
    }

    drop(root);
    assert!(
        !root_path.exists(),
        "successful identity control must clean {}",
        root_path.display()
    );
    println!("DASM-C2-CLEANED\t{}", root_path.display());
}
