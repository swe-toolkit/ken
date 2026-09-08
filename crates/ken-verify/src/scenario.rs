//! Checked-source, twin-real-root canonical differential runner.

use std::ffi::OsString;
use std::fmt;

use num_bigint::BigInt;

#[cfg(unix)]
use std::os::unix::ffi::{OsStrExt, OsStringExt};

use ken_elaborator::capabilities::{Authority, RightSet, SymlinkPolicy, AUTH_FULL};
use ken_host::EffectObservation;
use ken_runtime::{
    BoundProcessExecutableArtifact, NativeEffectRunErrorV1, NativeEffectRunOptionsV1,
};

use crate::{
    canonical_filesystem_delta, compare_canonical_exact, AmbientScript, ExpectedFsEffect,
    LaneActionEvidence, ObservationMismatch, ScriptedPosixHost, SeedNode, TwinRealRoots,
    TwinRootError,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RawProcessInput {
    /// Arguments after argv[0]. The runner supplies the exact produced artifact
    /// path as argv[0] to both lanes.
    pub arguments: Vec<Vec<u8>>,
    pub environment: Vec<(Vec<u8>, Vec<u8>)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramCapsShape {
    pub fs_authority: Authority,
    pub relative_root: Vec<u8>,
    pub rights: RightSet,
    pub symlink: SymlinkPolicy,
}

impl Default for ProgramCapsShape {
    fn default() -> Self {
        Self {
            fs_authority: AUTH_FULL,
            relative_root: Vec::new(),
            rights: RightSet::ALL,
            symlink: SymlinkPolicy::NoFollow,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckedProgramEntry {
    pub identity: String,
    pub package_name: String,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scenario {
    pub process_input: RawProcessInput,
    pub ambient: AmbientScript,
    pub program_caps: ProgramCapsShape,
    pub entry: CheckedProgramEntry,
    pub initial_filesystem: Vec<SeedNode>,
    /// Independent execution assertions for the interpreter descriptor calls.
    /// These values never author or alter an `EffectObservation` field.
    pub expected_fs: Vec<ExpectedFsEffect>,
}

#[derive(Debug)]
pub enum HarnessError {
    TwinRoots(TwinRootError),
    UnsupportedAmbient(&'static str),
    InvalidRawProcessInput(&'static str),
    Interpreter(String),
    NativeBuild(String),
    NativeRun(NativeEffectRunErrorV1),
    Observation(ObservationMismatch),
}

impl fmt::Display for HarnessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TwinRoots(error) => error.fmt(formatter),
            Self::UnsupportedAmbient(field) => {
                write!(formatter, "native production lane cannot script {field}")
            }
            Self::InvalidRawProcessInput(field) => {
                write!(
                    formatter,
                    "raw ProcessInput field is not host-executable: {field}"
                )
            }
            Self::Interpreter(error) => write!(formatter, "interpreter lane failed: {error}"),
            Self::NativeBuild(error) => write!(formatter, "native artifact build failed: {error}"),
            Self::NativeRun(error) => error.fmt(formatter),
            Self::Observation(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for HarnessError {}

impl From<TwinRootError> for HarnessError {
    fn from(error: TwinRootError) -> Self {
        Self::TwinRoots(error)
    }
}

impl From<ObservationMismatch> for HarnessError {
    fn from(error: ObservationMismatch) -> Self {
        Self::Observation(error)
    }
}

/// A passing result owns the real roots and exact linked artifact so mutation
/// gates can alter real launcher inputs after the baseline run.
pub struct CanonicalDifferentialRun {
    pub scenario_identity: String,
    pub interpreter: EffectObservation,
    pub native: EffectObservation,
    pub interpreter_actions: LaneActionEvidence,
    pub native_actions: LaneActionEvidence,
    pub exact_artifact_executed: bool,
    pub process_input_arguments: Vec<Vec<u8>>,
    pub process_input_environment: Vec<(Vec<u8>, Vec<u8>)>,
    pub process_input_cwd: Vec<u8>,
    artifact: BoundProcessExecutableArtifact,
    plan_hash: u64,
    roots: TwinRealRoots,
    interpreter_wall_window: WallClockWindow,
    native_wall_window: WallClockWindow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct WallClockWindow {
    start_nanoseconds: i128,
    end_nanoseconds: i128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClockWallNowDifferentialError {
    TraceShape {
        lane: &'static str,
        reason: String,
    },
    WentBackwards {
        lane: &'static str,
        first: BigInt,
        second: BigInt,
    },
    OutsidePlausibleWindow {
        lane: &'static str,
        reading: BigInt,
        start_nanoseconds: i128,
        end_nanoseconds: i128,
    },
    NormalizedMismatch(ObservationMismatch),
}

impl fmt::Display for ClockWallNowDifferentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TraceShape { lane, reason } => {
                write!(formatter, "{lane} ClockWallNow trace shape: {reason}")
            }
            Self::WentBackwards {
                lane,
                first,
                second,
            } => write!(
                formatter,
                "{lane} ClockWallNow went backwards: {first} then {second}"
            ),
            Self::OutsidePlausibleWindow {
                lane,
                reading,
                start_nanoseconds,
                end_nanoseconds,
            } => write!(
                formatter,
                "{lane} ClockWallNow reading {reading} lies outside \
                 controlled window [{start_nanoseconds}, {end_nanoseconds}]"
            ),
            Self::NormalizedMismatch(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ClockWallNowDifferentialError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConsoleReadDifferentialError {
    TraceShape {
        lane: &'static str,
        reason: String,
    },
    FixtureMismatch {
        lane: &'static str,
        event: usize,
        reason: String,
    },
    NormalizedMismatch(ObservationMismatch),
}

impl fmt::Display for ConsoleReadDifferentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TraceShape { lane, reason } => {
                write!(formatter, "{lane} ConsoleRead trace shape: {reason}")
            }
            Self::FixtureMismatch {
                lane,
                event,
                reason,
            } => write!(
                formatter,
                "{lane} ConsoleRead event {event} disagrees with stdin fixture: \
                 {reason}"
            ),
            Self::NormalizedMismatch(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ConsoleReadDifferentialError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FsAppendFileDifferentialError {
    Shape {
        lane: &'static str,
        reason: String,
    },
    ContentCount {
        lane: &'static str,
        reason: String,
    },
    Observation(ObservationMismatch),
}

impl fmt::Display for FsAppendFileDifferentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shape { lane, reason } => {
                write!(formatter, "{lane} FsAppendFile shape: {reason}")
            }
            Self::ContentCount { lane, reason } => {
                write!(formatter, "{lane} FsAppendFile content/count: {reason}")
            }
            Self::Observation(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for FsAppendFileDifferentialError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsMetadataField {
    Size,
    Kind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FsMetadataDifferentialError {
    Shape {
        lane: &'static str,
        event: usize,
        reason: String,
    },
    Field {
        lane: &'static str,
        event: usize,
        field: FsMetadataField,
        reason: String,
    },
    Observation(ObservationMismatch),
}

impl fmt::Display for FsMetadataDifferentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shape {
                lane,
                event,
                reason,
            } => write!(
                formatter,
                "{lane} FsMetadata event {event} shape: {reason}"
            ),
            Self::Field {
                lane,
                event,
                field,
                reason,
            } => write!(
                formatter,
                "{lane} FsMetadata event {event} {field:?}: {reason}"
            ),
            Self::Observation(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for FsMetadataDifferentialError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FsRenameDifferentialError {
    Shape {
        lane: &'static str,
        reason: String,
    },
    Transition {
        lane: &'static str,
        reason: String,
    },
    Observation(ObservationMismatch),
}

impl fmt::Display for FsRenameDifferentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shape { lane, reason } => {
                write!(formatter, "{lane} FsRename shape: {reason}")
            }
            Self::Transition { lane, reason } => {
                write!(formatter, "{lane} FsRename transition: {reason}")
            }
            Self::Observation(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for FsRenameDifferentialError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FsDirectoryDifferentialError {
    Shape {
        operation: ken_host::HostOpV1,
        lane: &'static str,
        reason: String,
    },
    Listing {
        lane: &'static str,
        reason: String,
    },
    Transition {
        operation: ken_host::HostOpV1,
        lane: &'static str,
        reason: String,
    },
    Observation(ObservationMismatch),
}

impl fmt::Display for FsDirectoryDifferentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shape {
                operation,
                lane,
                reason,
            } => write!(formatter, "{lane} {operation:?} shape: {reason}"),
            Self::Listing { lane, reason } => {
                write!(formatter, "{lane} FsReadDirectory listing: {reason}")
            }
            Self::Transition {
                operation,
                lane,
                reason,
            } => write!(formatter, "{lane} {operation:?} transition: {reason}"),
            Self::Observation(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for FsDirectoryDifferentialError {}

#[derive(Clone, Copy)]
enum MutationStateContract {
    CreateOne,
    RemoveSubtree,
    Unchanged,
    MidTraversalUnconstrained,
}

impl CanonicalDifferentialRun {
    pub fn compare_exact(&self) -> Result<(), ObservationMismatch> {
        compare_canonical_exact(&self.interpreter, &self.native)
    }

    /// Apply the same ClockWallNow projection to both real lanes before
    /// comparing. The projection erases only each instant's bytes; it retains
    /// the complete response variant/field shape and all other observation
    /// fields, while requiring two same-side non-decreasing readings inside
    /// each lane's independently measured wall-clock window.
    pub fn compare_clock_wall_now(
        &self,
    ) -> Result<(), ClockWallNowDifferentialError> {
        compare_clock_wall_now_observations(
            &self.interpreter,
            self.interpreter_wall_window,
            &self.native,
            self.native_wall_window,
        )
    }

    /// Apply the same finite-stdin projection to both lanes, then compare the
    /// complete canonical observations without erasing the reply bytes.
    pub fn compare_console_read(
        &self,
        stdin: &[u8],
        limits: &[u64],
    ) -> Result<(), ConsoleReadDifferentialError> {
        compare_console_read_observations(
            &self.interpreter,
            &self.native,
            stdin,
            limits,
        )
    }

    /// Require exact FsAppendFile request/reply and filesystem content/count
    /// behavior independently on both lanes before comparing all observations.
    /// The existing result is Unit, so count is the exact after-minus-before
    /// byte length; no reply field or wire identity is invented.
    pub fn compare_fs_append_file(
        &self,
        request_path: &[u8],
        filesystem_path: &[u8],
        before: &[u8],
        appended: &[u8],
    ) -> Result<(), FsAppendFileDifferentialError> {
        compare_fs_append_file_observations(
            &self.interpreter,
            &self.native,
            request_path,
            filesystem_path,
            before,
            appended,
        )
    }

    /// Bind every FsMetadata response independently to the file or directory
    /// that exists in that lane's real root, then compare the full canonical
    /// observations. No response field is erased or normalized.
    pub fn compare_fs_metadata(
        &self,
        paths: &[(&[u8], &[u8])],
    ) -> Result<(), FsMetadataDifferentialError> {
        let interpreter_expected = expected_fs_metadata(
            "interpreter",
            self.roots.interpreter(),
            paths,
        )?;
        let native_expected =
            expected_fs_metadata("native", self.roots.native(), paths)?;
        compare_fs_metadata_observations(
            &self.interpreter,
            &self.native,
            paths,
            &interpreter_expected,
            &native_expected,
        )
    }

    /// Require each lane to perform the same exact one-node move before
    /// comparing every canonical observation field.
    pub fn compare_fs_rename(
        &self,
        request_source: &[u8],
        request_destination: &[u8],
        filesystem_source: &[u8],
        filesystem_destination: &[u8],
        original: &[u8],
    ) -> Result<(), FsRenameDifferentialError> {
        compare_fs_rename_observations(
            &self.interpreter,
            &self.interpreter_actions,
            &self.native,
            &self.native_actions,
            request_source,
            request_destination,
            filesystem_source,
            filesystem_destination,
            original,
        )
    }

    /// Compare a whole-directory response as a name-sorted `{name, kind}` set.
    /// Raw host iteration order is deliberately not part of this projection.
    pub fn compare_fs_read_directory(
        &self,
        request_path: &[u8],
        expected_entries: &[ken_host::DirEntryV1],
    ) -> Result<(), FsDirectoryDifferentialError> {
        compare_fs_read_directory_observations(
            &self.interpreter,
            &self.native,
            request_path,
            expected_entries,
        )
    }

    pub fn compare_fs_create_directory(
        &self,
        request_path: &[u8],
        filesystem_path: &[u8],
        recursive: bool,
        expected_error: Option<ken_host::IoErrorIdentityV1>,
    ) -> Result<(), FsDirectoryDifferentialError> {
        compare_fs_mutation_observations(
            &self.interpreter,
            &self.interpreter_actions,
            &self.native,
            &self.native_actions,
            ken_host::CanonicalRequestV1::FsCreateDirectory {
                recursive,
                path: request_path.to_vec(),
            },
            filesystem_path,
            expected_error,
            if expected_error.is_some() {
                MutationStateContract::Unchanged
            } else {
                MutationStateContract::CreateOne
            },
        )
    }

    pub fn compare_fs_remove_file(
        &self,
        request_path: &[u8],
        filesystem_path: &[u8],
        expected_error: Option<ken_host::IoErrorIdentityV1>,
    ) -> Result<(), FsDirectoryDifferentialError> {
        compare_fs_mutation_observations(
            &self.interpreter,
            &self.interpreter_actions,
            &self.native,
            &self.native_actions,
            ken_host::CanonicalRequestV1::FsRemoveFile {
                path: request_path.to_vec(),
            },
            filesystem_path,
            expected_error,
            if expected_error.is_some() {
                MutationStateContract::Unchanged
            } else {
                MutationStateContract::RemoveSubtree
            },
        )
    }

    pub fn compare_fs_remove_directory(
        &self,
        request_path: &[u8],
        filesystem_path: &[u8],
        recursive: bool,
        expected_error: Option<ken_host::IoErrorIdentityV1>,
    ) -> Result<(), FsDirectoryDifferentialError> {
        compare_fs_mutation_observations(
            &self.interpreter,
            &self.interpreter_actions,
            &self.native,
            &self.native_actions,
            ken_host::CanonicalRequestV1::FsRemoveDirectory {
                recursive,
                path: request_path.to_vec(),
            },
            filesystem_path,
            expected_error,
            if expected_error.is_some() {
                MutationStateContract::Unchanged
            } else {
                MutationStateContract::RemoveSubtree
            },
        )
    }

    /// The ruled non-transactional carve-out: classification remains exact,
    /// while no relation is asserted between the two residual trees.
    pub fn compare_fs_remove_directory_mid_traversal_error(
        &self,
        request_path: &[u8],
        expected_error: ken_host::IoErrorIdentityV1,
    ) -> Result<(), FsDirectoryDifferentialError> {
        if !matches!(
            expected_error,
            ken_host::IoErrorIdentityV1::PermissionDenied
                | ken_host::IoErrorIdentityV1::Interrupted
                | ken_host::IoErrorIdentityV1::Other(_)
        ) {
            return Err(FsDirectoryDifferentialError::Transition {
                operation: ken_host::HostOpV1::FsRemoveDirectory,
                lane: "contract",
                reason: format!(
                    "{expected_error:?} is not a ruled mid-traversal error class"
                ),
            });
        }
        compare_fs_mutation_observations(
            &self.interpreter,
            &self.interpreter_actions,
            &self.native,
            &self.native_actions,
            ken_host::CanonicalRequestV1::FsRemoveDirectory {
                recursive: true,
                path: request_path.to_vec(),
            },
            request_path,
            Some(expected_error),
            MutationStateContract::MidTraversalUnconstrained,
        )
    }

    /// Mutate the real launch binding; the production decoder must fail closed.
    pub fn rejects_wrong_plan_binding(&self) -> bool {
        let options = NativeEffectRunOptionsV1 {
            arguments: self.process_input_arguments[1..]
                .iter()
                .map(|argument| raw_os_string(argument))
                .collect::<Result<Vec<_>, _>>()
                .expect("a completed run already validated arguments"),
            environment: self
                .process_input_environment
                .iter()
                .map(|(key, value)| Ok((raw_os_string(key)?, raw_os_string(value)?)))
                .collect::<Result<Vec<_>, HarnessError>>()
                .expect("a completed run already validated environment"),
            cwd: self.roots.native().to_path_buf(),
            plan_hash: self.plan_hash ^ 1,
        };
        matches!(
            ken_runtime::run_bound_process_effect_observation(&self.artifact, &options),
            Err(NativeEffectRunErrorV1::BindingMismatch)
        )
    }
}

/// Execute the same checked program through the interpreter and the real
/// linked artifact. Canonical equality is part of the return gate.
pub fn run_scenario(scenario: &Scenario) -> Result<CanonicalDifferentialRun, HarnessError> {
    let run = execute_scenario(scenario)?;
    compare_canonical_exact(&run.interpreter, &run.native)?;
    Ok(run)
}

// `ElabEnv::new` alone overflowed a stated 1,992 KiB stack and passed at
// 2,048 KiB in the ABI-A2 stack probe. The production CLI runs on an 8 MiB
// main-thread stack, so this local equivalent retains at least 6 MiB of
// measured headroom without depending on ambient `RUST_MIN_STACK`.
const SCENARIO_COMPILER_STACK_BYTES: usize = 8 * 1024 * 1024;

fn execute_scenario(
    scenario: &Scenario,
) -> Result<CanonicalDifferentialRun, HarnessError> {
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .name("ken-verify-scenario-compiler".to_string())
            .stack_size(SCENARIO_COMPILER_STACK_BYTES)
            .spawn_scoped(scope, || execute_scenario_on_compiler_stack(scenario))
            .expect("the stated scenario compiler stack must spawn")
            .join()
            .unwrap_or_else(|payload| std::panic::resume_unwind(payload))
    })
}

fn execute_scenario_on_compiler_stack(
    scenario: &Scenario,
) -> Result<CanonicalDifferentialRun, HarnessError> {
    validate_native_ambient(&scenario.ambient)?;
    let roots = TwinRealRoots::create(&scenario.initial_filesystem)?;
    let build = ken_cli::build_native_program(
        &scenario.entry.source,
        ken_cli::SourceFormat::Ken,
        &scenario.entry.package_name,
        roots.artifacts(),
    )
    .map_err(|error| HarnessError::NativeBuild(error.to_string()))?;

    let argv0 = raw_path_bytes(&build.artifact.executable_path)?;
    let mut arguments = vec![argv0];
    arguments.extend(scenario.process_input.arguments.clone());
    let cwd = raw_path_bytes(roots.native())?;

    let interpreter_before = roots.snapshot_interpreter()?;
    let native_before = roots.snapshot_native()?;
    let mut host = ScriptedPosixHost::new_scoped(
        roots.interpreter(),
        scenario.ambient.clone(),
        scenario.program_caps.fs_authority,
        &scenario.program_caps.relative_root,
        scenario.program_caps.rights,
        scenario.program_caps.symlink,
        scenario.expected_fs.clone(),
    )
    .map_err(|error| HarnessError::Interpreter(error.to_string()))?;
    let interpreter_window_start = wall_clock_nanoseconds();
    let mut interpreter = ken_cli::run_program_effect_observation(
        &scenario.entry.source,
        ken_cli::SourceFormat::Ken,
        &arguments,
        &scenario.process_input.environment,
        &cwd,
        &mut host,
    )
    .map_err(|error| HarnessError::Interpreter(format!("{error:?}")))?;
    let interpreter_window_end = wall_clock_nanoseconds();
    let interpreter_after = roots.snapshot_interpreter()?;
    host.finish_assertions()
        .map_err(HarnessError::Interpreter)?;
    if !interpreter.filesystem_delta.is_empty() {
        return Err(HarnessError::Interpreter(
            "PX5B interpreter producer unexpectedly authored filesystem_delta".to_string(),
        ));
    }
    interpreter.filesystem_delta =
        canonical_filesystem_delta(&interpreter_before, &interpreter_after);
    let interpreter_actions = LaneActionEvidence {
        root_before: interpreter_before,
        root_after: interpreter_after,
        fs_actions_after_resolve: Some(host.fs_actions_after_resolve()),
    };

    let options = NativeEffectRunOptionsV1 {
        arguments: scenario
            .process_input
            .arguments
            .iter()
            .map(|argument| raw_os_string(argument))
            .collect::<Result<Vec<_>, _>>()?,
        environment: scenario
            .process_input
            .environment
            .iter()
            .map(|(key, value)| Ok((raw_os_string(key)?, raw_os_string(value)?)))
            .collect::<Result<Vec<_>, HarnessError>>()?,
        cwd: roots.native().to_path_buf(),
        plan_hash: build.plan_transport_hash,
    };
    let native_window_start = wall_clock_nanoseconds();
    let native = ken_runtime::run_bound_process_effect_observation_with_stdin(
        &build.artifact,
        &options,
        &scenario.ambient.stdin,
    )
    .map_err(HarnessError::NativeRun)?;
    let native_window_end = wall_clock_nanoseconds();
    let native_after = roots.snapshot_native()?;
    let native_actions = LaneActionEvidence {
        root_before: native_before,
        root_after: native_after,
        fs_actions_after_resolve: None,
    };

    Ok(CanonicalDifferentialRun {
        scenario_identity: scenario.entry.identity.clone(),
        interpreter,
        native,
        interpreter_actions,
        native_actions,
        exact_artifact_executed: true,
        process_input_arguments: arguments,
        process_input_environment: scenario.process_input.environment.clone(),
        process_input_cwd: cwd,
        artifact: build.artifact,
        plan_hash: build.plan_transport_hash,
        roots,
        interpreter_wall_window: WallClockWindow {
            start_nanoseconds: interpreter_window_start,
            end_nanoseconds: interpreter_window_end,
        },
        native_wall_window: WallClockWindow {
            start_nanoseconds: native_window_start,
            end_nanoseconds: native_window_end,
        },
    })
}

fn wall_clock_nanoseconds() -> i128 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => i128::try_from(duration.as_nanos())
            .expect("a SystemTime duration always fits signed nanoseconds"),
        Err(error) => -i128::try_from(error.duration().as_nanos())
            .expect("a SystemTime duration always fits signed nanoseconds"),
    }
}

fn compare_clock_wall_now_observations(
    interpreter: &EffectObservation,
    interpreter_window: WallClockWindow,
    native: &EffectObservation,
    native_window: WallClockWindow,
) -> Result<(), ClockWallNowDifferentialError> {
    let interpreter = normalize_clock_wall_now_observation(
        "interpreter",
        interpreter,
        interpreter_window,
    )?;
    let native =
        normalize_clock_wall_now_observation("native", native, native_window)?;
    compare_canonical_exact(&interpreter, &native)
        .map_err(ClockWallNowDifferentialError::NormalizedMismatch)
}

fn normalize_clock_wall_now_observation(
    lane: &'static str,
    observation: &EffectObservation,
    window: WallClockWindow,
) -> Result<EffectObservation, ClockWallNowDifferentialError> {
    let mut normalized = observation.clone();
    let [first, second] = normalized.effect_trace.as_mut_slice() else {
        return Err(ClockWallNowDifferentialError::TraceShape {
            lane,
            reason: format!(
                "expected exactly two events, observed {}",
                observation.effect_trace.len()
            ),
        });
    };
    let mut readings = Vec::with_capacity(2);
    for (index, event) in [first, second].into_iter().enumerate() {
        if event.sequence != index as u64
            || event.operation != ken_host::HostOpV1::ClockWallNow
            || event.capability.is_some()
            || !event.resource_bindings.is_empty()
            || event.request != ken_host::CanonicalRequestV1::ClockWallNow
        {
            return Err(ClockWallNowDifferentialError::TraceShape {
                lane,
                reason: format!(
                    "event {index} is not the exact ambient ClockWallNow shape"
                ),
            });
        }
        let ken_host::CanonicalOutcomeV1::Success(
            ken_host::CanonicalReplyV1::Instant(bytes),
        ) = &mut event.outcome
        else {
            return Err(ClockWallNowDifferentialError::TraceShape {
                lane,
                reason: format!("event {index} did not return Instant(bytes)"),
            });
        };
        if bytes.is_empty() {
            return Err(ClockWallNowDifferentialError::TraceShape {
                lane,
                reason: format!("event {index} returned an empty Instant field"),
            });
        }
        readings.push(BigInt::from_signed_bytes_be(bytes));
        // Preserve `Success(Instant(one field))`, erasing only the
        // nondeterministic instant itself on both sides.
        bytes.clear();
    }
    if readings[1] < readings[0] {
        return Err(ClockWallNowDifferentialError::WentBackwards {
            lane,
            first: readings[0].clone(),
            second: readings[1].clone(),
        });
    }
    let start = BigInt::from(window.start_nanoseconds);
    let end = BigInt::from(window.end_nanoseconds);
    for reading in readings {
        if reading < start || reading > end {
            return Err(ClockWallNowDifferentialError::OutsidePlausibleWindow {
                lane,
                reading,
                start_nanoseconds: window.start_nanoseconds,
                end_nanoseconds: window.end_nanoseconds,
            });
        }
    }
    Ok(normalized)
}

fn compare_console_read_observations(
    interpreter: &EffectObservation,
    native: &EffectObservation,
    stdin: &[u8],
    limits: &[u64],
) -> Result<(), ConsoleReadDifferentialError> {
    let interpreter = normalize_console_read_observation(
        "interpreter",
        interpreter,
        stdin,
        limits,
    )?;
    let native =
        normalize_console_read_observation("native", native, stdin, limits)?;
    compare_canonical_exact(&interpreter, &native)
        .map_err(ConsoleReadDifferentialError::NormalizedMismatch)
}

fn normalize_console_read_observation(
    lane: &'static str,
    observation: &EffectObservation,
    stdin: &[u8],
    limits: &[u64],
) -> Result<EffectObservation, ConsoleReadDifferentialError> {
    if observation.effect_trace.len() != limits.len() {
        return Err(ConsoleReadDifferentialError::TraceShape {
            lane,
            reason: format!(
                "expected {} events, observed {}",
                limits.len(),
                observation.effect_trace.len()
            ),
        });
    }
    let mut cursor = 0usize;
    for (index, (event, expected_limit)) in observation
        .effect_trace
        .iter()
        .zip(limits)
        .enumerate()
    {
        if event.sequence != index as u64
            || event.operation != ken_host::HostOpV1::ConsoleRead
            || event.capability.is_some()
            || !event.resource_bindings.is_empty()
            || event.request
                != (ken_host::CanonicalRequestV1::ConsoleRead {
                    stream: ken_host::ConsoleStreamV1::Stdin,
                    limit: *expected_limit,
                })
        {
            return Err(ConsoleReadDifferentialError::TraceShape {
                lane,
                reason: format!(
                    "event {index} is not the exact ConsoleRead request shape"
                ),
            });
        }
        if cursor == stdin.len() {
            if event.outcome
                != ken_host::CanonicalOutcomeV1::Success(
                    ken_host::CanonicalReplyV1::ReadEof,
                )
            {
                return Err(ConsoleReadDifferentialError::FixtureMismatch {
                    lane,
                    event: index,
                    reason: "expected EOF after the fixture's last byte".to_string(),
                });
            }
            continue;
        }
        let requested = usize::try_from(*expected_limit).unwrap_or(usize::MAX);
        let count = requested.min(stdin.len() - cursor);
        let expected = &stdin[cursor..cursor + count];
        let ken_host::CanonicalOutcomeV1::Success(
            ken_host::CanonicalReplyV1::ReadChunk(actual),
        ) = &event.outcome
        else {
            return Err(ConsoleReadDifferentialError::FixtureMismatch {
                lane,
                event: index,
                reason: format!("expected a {count}-byte Chunk"),
            });
        };
        if actual != expected {
            return Err(ConsoleReadDifferentialError::FixtureMismatch {
                lane,
                event: index,
                reason: format!(
                    "expected bytes {:?}, observed {:?}",
                    expected, actual
                ),
            });
        }
        cursor += count;
    }
    if cursor != stdin.len() {
        return Err(ConsoleReadDifferentialError::TraceShape {
            lane,
            reason: format!(
                "limits consumed {cursor} of {} injected byte(s)",
                stdin.len()
            ),
        });
    }
    Ok(observation.clone())
}

fn compare_fs_append_file_observations(
    interpreter: &EffectObservation,
    native: &EffectObservation,
    request_path: &[u8],
    filesystem_path: &[u8],
    before: &[u8],
    appended: &[u8],
) -> Result<(), FsAppendFileDifferentialError> {
    for (lane, observation) in [
        ("interpreter", interpreter),
        ("native", native),
    ] {
        validate_fs_append_file_observation(
            lane,
            observation,
            request_path,
            filesystem_path,
            before,
            appended,
        )?;
    }
    compare_canonical_exact(interpreter, native)
        .map_err(FsAppendFileDifferentialError::Observation)
}

fn validate_fs_append_file_observation(
    lane: &'static str,
    observation: &EffectObservation,
    request_path: &[u8],
    filesystem_path: &[u8],
    before: &[u8],
    appended: &[u8],
) -> Result<(), FsAppendFileDifferentialError> {
    let [event] = observation.effect_trace.as_slice() else {
        return Err(FsAppendFileDifferentialError::Shape {
            lane,
            reason: format!(
                "expected one event, observed {}",
                observation.effect_trace.len()
            ),
        });
    };
    if event.sequence != 0
        || event.operation != ken_host::HostOpV1::FsAppendFile
        || event.capability.is_none()
        || !event.resource_bindings.is_empty()
        || event.request
            != (ken_host::CanonicalRequestV1::FsAppendFile {
                path: request_path.to_vec(),
                bytes: appended.to_vec(),
            })
        || event.outcome
            != ken_host::CanonicalOutcomeV1::Success(
                ken_host::CanonicalReplyV1::Unit,
            )
    {
        return Err(FsAppendFileDifferentialError::Shape {
            lane,
            reason: "event is not the exact capability/request/Unit shape"
                .to_string(),
        });
    }
    let [ken_host::FsDeltaV1::Modified {
        relative_path,
        before: before_node,
        after: after_node,
    }] = observation.filesystem_delta.as_slice()
    else {
        return Err(FsAppendFileDifferentialError::ContentCount {
            lane,
            reason: "expected exactly one modified filesystem node".to_string(),
        });
    };
    let expected_after = [before, appended].concat();
    let before_bytes = before_node.file_bytes.as_deref();
    let after_bytes = after_node.file_bytes.as_deref();
    let count = after_bytes
        .and_then(|after| after.len().checked_sub(before.len()));
    if relative_path != filesystem_path
        || before_node.kind != ken_host::FsNodeKindV1::File
        || after_node.kind != ken_host::FsNodeKindV1::File
        || before_bytes != Some(before)
        || after_bytes != Some(expected_after.as_slice())
        || count != Some(appended.len())
    {
        return Err(FsAppendFileDifferentialError::ContentCount {
            lane,
            reason: format!(
                "expected before={before:?}, appended={appended:?}, \
                 after={expected_after:?}; observed delta={:?}",
                observation.filesystem_delta
            ),
        });
    }
    Ok(())
}

fn compare_fs_metadata_observations(
    interpreter: &EffectObservation,
    native: &EffectObservation,
    paths: &[(&[u8], &[u8])],
    interpreter_expected: &[ken_host::FileMetadataV1],
    native_expected: &[ken_host::FileMetadataV1],
) -> Result<(), FsMetadataDifferentialError> {
    validate_fs_metadata_observation(
        "interpreter",
        interpreter,
        paths,
        interpreter_expected,
    )?;
    validate_fs_metadata_observation("native", native, paths, native_expected)?;
    compare_canonical_exact(interpreter, native)
        .map_err(FsMetadataDifferentialError::Observation)
}

fn validate_fs_metadata_observation(
    lane: &'static str,
    observation: &EffectObservation,
    paths: &[(&[u8], &[u8])],
    expected: &[ken_host::FileMetadataV1],
) -> Result<(), FsMetadataDifferentialError> {
    if observation.effect_trace.len() != paths.len()
        || expected.len() != paths.len()
    {
        return Err(FsMetadataDifferentialError::Shape {
            lane,
            event: 0,
            reason: format!(
                "expected {} events and metadata rows, observed {} events and \
                 {} rows",
                paths.len(),
                observation.effect_trace.len(),
                expected.len()
            ),
        });
    }
    if !observation.filesystem_delta.is_empty() {
        return Err(FsMetadataDifferentialError::Shape {
            lane,
            event: 0,
            reason: "read-only metadata produced a filesystem delta".to_string(),
        });
    }
    for (index, ((request_path, _), expected)) in
        paths.iter().zip(expected).enumerate()
    {
        let event = &observation.effect_trace[index];
        if event.sequence != index as u64
            || event.operation != ken_host::HostOpV1::FsMetadata
            || event.capability.is_none()
            || !event.resource_bindings.is_empty()
            || event.request
                != (ken_host::CanonicalRequestV1::FsMetadata {
                    path: request_path.to_vec(),
                })
        {
            return Err(FsMetadataDifferentialError::Shape {
                lane,
                event: index,
                reason: "event is not the exact capability/request shape"
                    .to_string(),
            });
        }
        let ken_host::CanonicalOutcomeV1::Success(
            ken_host::CanonicalReplyV1::FileMetadata(actual),
        ) = &event.outcome
        else {
            return Err(FsMetadataDifferentialError::Shape {
                lane,
                event: index,
                reason: "event did not return FileMetadata".to_string(),
            });
        };
        if actual.size != expected.size {
            return Err(FsMetadataDifferentialError::Field {
                lane,
                event: index,
                field: FsMetadataField::Size,
                reason: format!(
                    "expected {}, observed {}",
                    expected.size, actual.size
                ),
            });
        }
        if actual.kind != expected.kind {
            return Err(FsMetadataDifferentialError::Field {
                lane,
                event: index,
                field: FsMetadataField::Kind,
                reason: format!(
                    "expected {:?}, observed {:?}",
                    expected.kind, actual.kind
                ),
            });
        }
    }
    Ok(())
}

fn compare_fs_rename_observations(
    interpreter: &EffectObservation,
    interpreter_actions: &LaneActionEvidence,
    native: &EffectObservation,
    native_actions: &LaneActionEvidence,
    request_source: &[u8],
    request_destination: &[u8],
    filesystem_source: &[u8],
    filesystem_destination: &[u8],
    original: &[u8],
) -> Result<(), FsRenameDifferentialError> {
    for (lane, observation, actions) in [
        ("interpreter", interpreter, interpreter_actions),
        ("native", native, native_actions),
    ] {
        validate_fs_rename_observation(
            lane,
            observation,
            actions,
            request_source,
            request_destination,
            filesystem_source,
            filesystem_destination,
            original,
        )?;
    }
    compare_canonical_exact(interpreter, native)
        .map_err(FsRenameDifferentialError::Observation)
}

fn validate_fs_rename_observation(
    lane: &'static str,
    observation: &EffectObservation,
    actions: &LaneActionEvidence,
    request_source: &[u8],
    request_destination: &[u8],
    filesystem_source: &[u8],
    filesystem_destination: &[u8],
    original: &[u8],
) -> Result<(), FsRenameDifferentialError> {
    let [event] = observation.effect_trace.as_slice() else {
        return Err(FsRenameDifferentialError::Shape {
            lane,
            reason: format!(
                "expected one event, observed {}",
                observation.effect_trace.len()
            ),
        });
    };
    if event.sequence != 0
        || event.operation != ken_host::HostOpV1::FsRename
        || event.capability.is_none()
        || !event.resource_bindings.is_empty()
        || event.request
            != (ken_host::CanonicalRequestV1::FsRename {
                source: request_source.to_vec(),
                destination: request_destination.to_vec(),
            })
        || event.outcome
            != ken_host::CanonicalOutcomeV1::Success(
                ken_host::CanonicalReplyV1::Unit,
            )
        || observation.terminal_error.is_some()
        || observation.terminal_exit != ken_host::TerminalExitClass::NormalReturn
        || observation.exit_status != 0
    {
        return Err(FsRenameDifferentialError::Shape {
            lane,
            reason: "event/result is not the exact capability/request/Unit success shape"
                .to_string(),
        });
    }

    let source_before = actions
        .root_before
        .nodes
        .iter()
        .find(|node| node.relative_path == filesystem_source)
        .ok_or_else(|| FsRenameDifferentialError::Transition {
            lane,
            reason: "source was absent before rename".to_string(),
        })?;
    if source_before.kind != crate::SnapshotNodeKind::File
        || source_before.bytes != original
        || actions
            .root_before
            .nodes
            .iter()
            .any(|node| node.relative_path == filesystem_destination)
    {
        return Err(FsRenameDifferentialError::Transition {
            lane,
            reason: "before-state did not contain only the exact source file"
                .to_string(),
        });
    }
    let mut expected_after = actions.root_before.clone();
    let moved = expected_after
        .nodes
        .iter_mut()
        .find(|node| node.relative_path == filesystem_source)
        .expect("the source-before check established this node");
    moved.relative_path = filesystem_destination.to_vec();
    expected_after.nodes.sort();
    if actions.root_after != expected_after {
        return Err(FsRenameDifferentialError::Transition {
            lane,
            reason: format!(
                "expected source removal plus destination creation with the original node; \
                 before={:?}, after={:?}",
                actions.root_before, actions.root_after
            ),
        });
    }

    let removed = observation.filesystem_delta.iter().find_map(|delta| {
        if let ken_host::FsDeltaV1::Removed {
            relative_path,
            node,
        } = delta
        {
            (relative_path == filesystem_source).then_some(node)
        } else {
            None
        }
    });
    let created = observation.filesystem_delta.iter().find_map(|delta| {
        if let ken_host::FsDeltaV1::Created {
            relative_path,
            node,
        } = delta
        {
            (relative_path == filesystem_destination).then_some(node)
        } else {
            None
        }
    });
    if observation.filesystem_delta.len() != 2
        || removed.is_none()
        || created.is_none()
        || removed != created
        || removed.and_then(|node| node.file_bytes.as_deref()) != Some(original)
    {
        return Err(FsRenameDifferentialError::Transition {
            lane,
            reason: format!(
                "expected exactly Removed(source) and Created(destination) for \
                 the same original node; observed {:?}",
                observation.filesystem_delta
            ),
        });
    }
    Ok(())
}

fn compare_fs_read_directory_observations(
    interpreter: &EffectObservation,
    native: &EffectObservation,
    request_path: &[u8],
    expected_entries: &[ken_host::DirEntryV1],
) -> Result<(), FsDirectoryDifferentialError> {
    let mut expected = expected_entries.to_vec();
    sort_directory_entries(&mut expected);
    for (lane, observation) in [
        ("interpreter", interpreter),
        ("native", native),
    ] {
        let [event] = observation.effect_trace.as_slice() else {
            return Err(FsDirectoryDifferentialError::Shape {
                operation: ken_host::HostOpV1::FsReadDirectory,
                lane,
                reason: format!(
                    "expected one event, observed {}",
                    observation.effect_trace.len()
                ),
            });
        };
        if event.sequence != 0
            || event.operation != ken_host::HostOpV1::FsReadDirectory
            || event.capability.is_none()
            || !event.resource_bindings.is_empty()
            || event.request
                != (ken_host::CanonicalRequestV1::FsReadDirectory {
                    path: request_path.to_vec(),
                })
            || observation.terminal_error.is_some()
            || observation.terminal_exit != ken_host::TerminalExitClass::NormalReturn
            || observation.exit_status != 0
            || !observation.filesystem_delta.is_empty()
        {
            return Err(FsDirectoryDifferentialError::Shape {
                operation: ken_host::HostOpV1::FsReadDirectory,
                lane,
                reason: "event/result is not the exact read-only success shape"
                    .to_string(),
            });
        }
        let ken_host::CanonicalOutcomeV1::Success(
            ken_host::CanonicalReplyV1::DirectoryEntries(entries),
        ) = &event.outcome
        else {
            return Err(FsDirectoryDifferentialError::Listing {
                lane,
                reason: "response is not DirectoryEntries".to_string(),
            });
        };
        let mut actual = entries.clone();
        sort_directory_entries(&mut actual);
        if actual != expected {
            return Err(FsDirectoryDifferentialError::Listing {
                lane,
                reason: format!(
                    "expected canonicalized {expected:?}, observed {actual:?}"
                ),
            });
        }
    }
    let mut interpreter = interpreter.clone();
    let mut native = native.clone();
    normalize_directory_listing(&mut interpreter);
    normalize_directory_listing(&mut native);
    compare_canonical_exact(&interpreter, &native)
        .map_err(FsDirectoryDifferentialError::Observation)
}

fn sort_directory_entries(entries: &mut [ken_host::DirEntryV1]) {
    let kind = |kind| match kind {
        ken_host::FsNodeKindV1::File => 0,
        ken_host::FsNodeKindV1::Directory => 1,
        ken_host::FsNodeKindV1::Symlink => 2,
        ken_host::FsNodeKindV1::Other => 3,
    };
    entries.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| kind(left.kind).cmp(&kind(right.kind)))
    });
}

fn normalize_directory_listing(observation: &mut EffectObservation) {
    for event in &mut observation.effect_trace {
        if event.operation != ken_host::HostOpV1::FsReadDirectory {
            continue;
        }
        if let ken_host::CanonicalOutcomeV1::Success(
            ken_host::CanonicalReplyV1::DirectoryEntries(entries),
        ) = &mut event.outcome
        {
            sort_directory_entries(entries);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn compare_fs_mutation_observations(
    interpreter: &EffectObservation,
    interpreter_actions: &LaneActionEvidence,
    native: &EffectObservation,
    native_actions: &LaneActionEvidence,
    request: ken_host::CanonicalRequestV1,
    filesystem_path: &[u8],
    expected_error: Option<ken_host::IoErrorIdentityV1>,
    state_contract: MutationStateContract,
) -> Result<(), FsDirectoryDifferentialError> {
    let operation = match &request {
        ken_host::CanonicalRequestV1::FsCreateDirectory { .. } => {
            ken_host::HostOpV1::FsCreateDirectory
        }
        ken_host::CanonicalRequestV1::FsRemoveFile { .. } => {
            ken_host::HostOpV1::FsRemoveFile
        }
        ken_host::CanonicalRequestV1::FsRemoveDirectory { .. } => {
            ken_host::HostOpV1::FsRemoveDirectory
        }
        _ => unreachable!("directory mutation comparator has a closed request set"),
    };
    for (lane, observation, actions) in [
        ("interpreter", interpreter, interpreter_actions),
        ("native", native, native_actions),
    ] {
        validate_fs_mutation_observation(
            lane,
            observation,
            actions,
            operation,
            &request,
            filesystem_path,
            expected_error,
            state_contract,
        )?;
    }
    let mut interpreter = interpreter.clone();
    let mut native = native.clone();
    if matches!(
        state_contract,
        MutationStateContract::MidTraversalUnconstrained
    ) {
        interpreter.filesystem_delta.clear();
        native.filesystem_delta.clear();
    }
    compare_canonical_exact(&interpreter, &native)
        .map_err(FsDirectoryDifferentialError::Observation)
}

#[allow(clippy::too_many_arguments)]
fn validate_fs_mutation_observation(
    lane: &'static str,
    observation: &EffectObservation,
    actions: &LaneActionEvidence,
    operation: ken_host::HostOpV1,
    request: &ken_host::CanonicalRequestV1,
    filesystem_path: &[u8],
    expected_error: Option<ken_host::IoErrorIdentityV1>,
    state_contract: MutationStateContract,
) -> Result<(), FsDirectoryDifferentialError> {
    let [event] = observation.effect_trace.as_slice() else {
        return Err(FsDirectoryDifferentialError::Shape {
            operation,
            lane,
            reason: format!(
                "expected one event, observed {}",
                observation.effect_trace.len()
            ),
        });
    };
    let request_path = match request {
        ken_host::CanonicalRequestV1::FsCreateDirectory { path, .. }
        | ken_host::CanonicalRequestV1::FsRemoveFile { path }
        | ken_host::CanonicalRequestV1::FsRemoveDirectory { path, .. } => path,
        _ => unreachable!("validated mutation request set"),
    };
    let expected_outcome = match expected_error {
        Some(error) => ken_host::CanonicalOutcomeV1::Error(
            ken_host::SemanticErrorV1::File(ken_host::FileErrorIdentityV1 {
                operation,
                relative_path: request_path.clone(),
                cause: ken_host::FileErrorCauseV1::Io(error),
            }),
        ),
        None => ken_host::CanonicalOutcomeV1::Success(
            ken_host::CanonicalReplyV1::Unit,
        ),
    };
    let expected_terminal_exit = if expected_error.is_some() {
        ken_host::TerminalExitClass::ReturnedError
    } else {
        ken_host::TerminalExitClass::NormalReturn
    };
    let expected_exit_status = match expected_error {
        None => 0,
        Some(ken_host::IoErrorIdentityV1::NotFound) => 45,
        Some(ken_host::IoErrorIdentityV1::PermissionDenied) => 46,
        Some(ken_host::IoErrorIdentityV1::BrokenPipe) => 47,
        Some(ken_host::IoErrorIdentityV1::Interrupted) => 48,
        Some(ken_host::IoErrorIdentityV1::AlreadyExists) => 49,
        Some(ken_host::IoErrorIdentityV1::InvalidInput) => 50,
        Some(ken_host::IoErrorIdentityV1::IsDirectory) => 51,
        Some(ken_host::IoErrorIdentityV1::NotDirectory) => 52,
        Some(ken_host::IoErrorIdentityV1::NotEmpty) => 53,
        Some(ken_host::IoErrorIdentityV1::Unsupported) => 54,
        Some(ken_host::IoErrorIdentityV1::Revoked) => 55,
        Some(ken_host::IoErrorIdentityV1::Other(_)) => 56,
    };
    if event.sequence != 0
        || event.operation != operation
        || event.capability.is_none()
        || !event.resource_bindings.is_empty()
        || &event.request != request
        || event.outcome != expected_outcome
        || observation.terminal_error.is_some()
        || observation.terminal_exit != expected_terminal_exit
        || observation.exit_status != expected_exit_status
    {
        return Err(FsDirectoryDifferentialError::Shape {
            operation,
            lane,
            reason: format!(
                "event/result differs from request={request:?}, outcome={expected_outcome:?}"
            ),
        });
    }

    match state_contract {
        MutationStateContract::CreateOne => {
            if actions
                .root_before
                .nodes
                .iter()
                .any(|node| node.relative_path == filesystem_path)
            {
                return Err(FsDirectoryDifferentialError::Transition {
                    operation,
                    lane,
                    reason: "create target was already present before the operation"
                        .to_string(),
                });
            }
            let created = actions
                .root_after
                .nodes
                .iter()
                .filter(|node| node.relative_path == filesystem_path)
                .collect::<Vec<_>>();
            let mut without_created = actions.root_after.clone();
            without_created
                .nodes
                .retain(|node| node.relative_path != filesystem_path);
            let exact_delta = matches!(
                observation.filesystem_delta.as_slice(),
                [ken_host::FsDeltaV1::Created { relative_path, node }]
                    if relative_path == filesystem_path
                        && node.kind == ken_host::FsNodeKindV1::Directory
                        && node.file_bytes.is_none()
                        && node.symlink_target.is_none()
            );
            if created.len() != 1
                || created[0].kind != crate::SnapshotNodeKind::Directory
                || without_created != actions.root_before
                || !exact_delta
            {
                return Err(FsDirectoryDifferentialError::Transition {
                    operation,
                    lane,
                    reason: format!(
                        "create did not add exactly one directory node; \
                         before={:?}, after={:?}, delta={:?}",
                        actions.root_before,
                        actions.root_after,
                        observation.filesystem_delta
                    ),
                });
            }
        }
        MutationStateContract::RemoveSubtree => {
            let prefix = [filesystem_path, b"/"].concat();
            let removed_before = actions
                .root_before
                .nodes
                .iter()
                .filter(|node| {
                    node.relative_path == filesystem_path
                        || node.relative_path.starts_with(&prefix)
                })
                .count();
            let mut expected_after = actions.root_before.clone();
            expected_after.nodes.retain(|node| {
                node.relative_path != filesystem_path
                    && !node.relative_path.starts_with(&prefix)
            });
            let deltas_are_exact_removals = observation.filesystem_delta.len()
                == removed_before
                && observation.filesystem_delta.iter().all(|delta| {
                    matches!(
                        delta,
                        ken_host::FsDeltaV1::Removed { relative_path, .. }
                            if relative_path == filesystem_path
                                || relative_path.starts_with(&prefix)
                    )
                });
            if removed_before == 0
                || actions.root_after != expected_after
                || !deltas_are_exact_removals
            {
                return Err(FsDirectoryDifferentialError::Transition {
                    operation,
                    lane,
                    reason: format!(
                        "remove did not delete exactly the selected subtree; \
                         before={:?}, after={:?}, delta={:?}",
                        actions.root_before,
                        actions.root_after,
                        observation.filesystem_delta
                    ),
                });
            }
        }
        MutationStateContract::Unchanged => {
            if actions.root_before != actions.root_after
                || !observation.filesystem_delta.is_empty()
            {
                return Err(FsDirectoryDifferentialError::Transition {
                    operation,
                    lane,
                    reason: format!(
                        "refusal changed deterministic state; before={:?}, after={:?}, delta={:?}",
                        actions.root_before,
                        actions.root_after,
                        observation.filesystem_delta
                    ),
                });
            }
        }
        MutationStateContract::MidTraversalUnconstrained => {
            if expected_error.is_none() {
                return Err(FsDirectoryDifferentialError::Transition {
                    operation,
                    lane,
                    reason: "mid-traversal carve-out was applied to success"
                        .to_string(),
                });
            }
        }
    }
    Ok(())
}

fn expected_fs_metadata(
    lane: &'static str,
    root: &std::path::Path,
    paths: &[(&[u8], &[u8])],
) -> Result<Vec<ken_host::FileMetadataV1>, FsMetadataDifferentialError> {
    paths
        .iter()
        .enumerate()
        .map(|(event, (_, filesystem_path))| {
            let path = raw_os_string(filesystem_path).map_err(|error| {
                FsMetadataDifferentialError::Shape {
                    lane,
                    event,
                    reason: error.to_string(),
                }
            })?;
            let metadata = std::fs::symlink_metadata(root.join(path))
                .map_err(|error| FsMetadataDifferentialError::Shape {
                    lane,
                    event,
                    reason: format!("cannot observe fixture metadata: {error}"),
                })?;
            let file_type = metadata.file_type();
            let kind = if file_type.is_file() {
                ken_host::FsNodeKindV1::File
            } else if file_type.is_dir() {
                ken_host::FsNodeKindV1::Directory
            } else if file_type.is_symlink() {
                ken_host::FsNodeKindV1::Symlink
            } else {
                ken_host::FsNodeKindV1::Other
            };
            Ok(ken_host::FileMetadataV1 {
                size: metadata.len(),
                kind,
            })
        })
        .collect()
}

fn validate_native_ambient(ambient: &AmbientScript) -> Result<(), HarnessError> {
    if ambient.stdin_is_terminal || ambient.stdout_is_terminal || ambient.stderr_is_terminal {
        return Err(HarnessError::UnsupportedAmbient(
            "terminal state other than piped/false",
        ));
    }
    if !ambient.wall_clock_nanoseconds.is_empty() {
        return Err(HarnessError::UnsupportedAmbient("clock script"));
    }
    Ok(())
}

#[cfg(unix)]
fn raw_os_string(bytes: &[u8]) -> Result<OsString, HarnessError> {
    if bytes.contains(&0) {
        return Err(HarnessError::InvalidRawProcessInput("embedded NUL"));
    }
    Ok(OsString::from_vec(bytes.to_vec()))
}

#[cfg(not(unix))]
fn raw_os_string(_bytes: &[u8]) -> Result<OsString, HarnessError> {
    Err(HarnessError::InvalidRawProcessInput(
        "raw bytes require Unix",
    ))
}

#[cfg(unix)]
fn raw_path_bytes(path: &std::path::Path) -> Result<Vec<u8>, HarnessError> {
    Ok(path.as_os_str().as_bytes().to_vec())
}

#[cfg(not(unix))]
fn raw_path_bytes(_path: &std::path::Path) -> Result<Vec<u8>, HarnessError> {
    Err(HarnessError::InvalidRawProcessInput(
        "raw paths require Unix",
    ))
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::{
        apply_canonical_mutation, confirm_native_tested_transition, denial_precedes_host_action,
        CanonicalMutation, NativeTestedEvidence, RunnerOnlyProxy, StatusTransitionError,
    };
    use ken_elaborator::capabilities::{AUTH_NONE, AUTH_PARTIAL};
    use ken_host::{
        dispatch_host_op_v1, program_caps_fs_trace_identity_v1, CanonicalOutcomeV1,
        CanonicalReplyV1, CanonicalRequestV1, CapabilityDeniedV1, CapabilityGrantV1,
        CapabilityTableV1, ConsoleStreamV1, CreatePolicyV1, FileErrorCauseV1, HostEffectBackendV1,
        HostOpAvailabilityV1, HostOpV1, IoErrorIdentityV1, SemanticErrorV1,
        PX5_PLANNED_NATIVE_TARGETS,
    };

    const FIVE_OP_SOURCE: &str = r#"program capabilities FS AFull
proc main (input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS, Console] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit AFull (Failure 20) ;
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit AFull (Failure 21) ;
        Cons path more |-> match more {
          Nil |-> host_exit AFull (Failure 22) ;
          Cons contents _ |-> match caps {
            MkProgramCaps cap |->
              bind (Coproduct (FSOp AFull) AmbientOp)
                (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
                (Result FileError Unit) ExitCode
                (inject_l (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp
                  (Result FileError Unit)
                  (writeFile cap path CreateNew contents))
                (\written. match written {
                  Err _ |-> host_exit AFull (Failure 23) ;
                  Ok _ |->
                    bind (Coproduct (FSOp AFull) AmbientOp)
                      (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
                      (Result FileError Bytes) ExitCode
                      (inject_l (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp
                        (Result FileError Bytes) (readFile AFull cap path))
                      (\read. match read {
                        Err _ |-> host_exit AFull (Failure 24) ;
                        Ok bytes |->
                          bind (Coproduct (FSOp AFull) AmbientOp)
                            (resp_coproduct (FSOp AFull) AmbientOp
                              (fs_resp AFull) ambient_resp)
                            (Result IOError Unit) ExitCode
                            (host_console AFull (Result IOError Unit)
                              (write Stdout bytes))
                            (\written_console. match written_console {
                              Err _ |-> host_exit AFull (Failure 25) ;
                              Ok _ |->
                                bind (Coproduct (FSOp AFull) AmbientOp)
                                  (resp_coproduct (FSOp AFull) AmbientOp
                                    (fs_resp AFull) ambient_resp)
                                  (Result IOError Unit) ExitCode
                                  (host_console AFull (Result IOError Unit) (flush Stdout))
                                  (\flushed. match flushed {
                                    Err _ |-> host_exit AFull (Failure 26) ;
                                    Ok _ |->
                                      bind (Coproduct (FSOp AFull) AmbientOp)
                                        (resp_coproduct (FSOp AFull) AmbientOp
                                          (fs_resp AFull) ambient_resp)
                                        Bool ExitCode
                                        (host_console AFull Bool (is_terminal Stdout))
                                        (\terminal. match terminal {
                                          False |-> host_exit AFull Success ;
                                          True |-> host_exit AFull (Failure 27)
                                        })
                                  })
                            })
                      })
                })
          }
        }
      }
    }
  }
"#;

    const CLOCK_WALL_SOURCE: &str = r#"program capabilities FS AFull
proc main (_input : ProcessInput) (_caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [Clock] =
  bind (Coproduct (FSOp AFull) AmbientOp)
       (resp_coproduct (FSOp AFull) AmbientOp
         (fs_resp AFull) ambient_resp)
       Instant ExitCode
    (host_clock AFull Instant wall_now)
    (\first . match first {
      MkInstant first_nanoseconds |->
        bind (Coproduct (FSOp AFull) AmbientOp)
             (resp_coproduct (FSOp AFull) AmbientOp
               (fs_resp AFull) ambient_resp)
             Instant ExitCode
          (host_clock AFull Instant wall_now)
          (\second . match second {
            MkInstant second_nanoseconds |->
              match leq_int 1600000000000000000 first_nanoseconds {
                False |-> host_exit AFull (Failure 31) ;
                True |-> match leq_int first_nanoseconds second_nanoseconds {
                  False |-> host_exit AFull (Failure 32) ;
                  True |-> match leq_int second_nanoseconds 4102444800000000000 {
                    False |-> host_exit AFull (Failure 33) ;
                    True |-> host_exit AFull Success
                  }
                }
              }
          })
    })
"#;

    const CONSOLE_READ_SOURCE: &str = r#"program capabilities FS AFull
proc main (_input : ProcessInput) (_caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [Console] =
  bind (Coproduct (FSOp AFull) AmbientOp)
       (resp_coproduct (FSOp AFull) AmbientOp
         (fs_resp AFull) ambient_resp)
       (Result IOError ReadResult) ExitCode
    (host_console AFull (Result IOError ReadResult) (read Stdin (2 : Int)))
    (\first. match first {
      Err _ |-> host_exit AFull (Failure 41) ;
      Ok first_read |-> match first_read {
        Eof |-> host_exit AFull (Failure 42) ;
        Chunk first_bytes |->
          match eq_int (bytes_length first_bytes) 2 {
            False |-> host_exit AFull (Failure 47) ;
            True |->
              bind (Coproduct (FSOp AFull) AmbientOp)
                   (resp_coproduct (FSOp AFull) AmbientOp
                     (fs_resp AFull) ambient_resp)
                   (Result IOError ReadResult) ExitCode
                (host_console AFull (Result IOError ReadResult)
                  (read Stdin (4 : Int)))
                (\second. match second {
              Err _ |-> host_exit AFull (Failure 43) ;
              Ok second_read |-> match second_read {
                Eof |-> host_exit AFull (Failure 44) ;
                Chunk second_bytes |->
                  match eq_int (bytes_length second_bytes) 1 {
                    False |-> host_exit AFull (Failure 48) ;
                    True |->
                      bind (Coproduct (FSOp AFull) AmbientOp)
                           (resp_coproduct (FSOp AFull) AmbientOp
                             (fs_resp AFull) ambient_resp)
                           (Result IOError ReadResult) ExitCode
                        (host_console AFull (Result IOError ReadResult)
                          (read Stdin (4 : Int)))
                        (\third. match third {
                          Err _ |-> host_exit AFull (Failure 45) ;
                          Ok third_read |-> match third_read {
                            Chunk _ |-> host_exit AFull (Failure 46) ;
                            Eof |-> host_exit AFull Success
                          }
                        })
                  }
              }
            })
          }
      }
    })
"#;

    const FS_APPEND_SOURCE: &str = r#"program capabilities FS AFull "./data"
proc main (input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit AFull (Failure 80) ;
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit AFull (Failure 81) ;
        Cons path more |-> match more {
          Nil |-> host_exit AFull (Failure 82) ;
          Cons contents _ |-> match caps {
            MkProgramCaps cap |->
              bind (Coproduct (FSOp AFull) AmbientOp)
                (resp_coproduct (FSOp AFull) AmbientOp
                  (fs_resp AFull) ambient_resp)
                (Result FileError Unit) ExitCode
                (inject_l (FSOp AFull) AmbientOp
                  (fs_resp AFull) ambient_resp
                  (Result FileError Unit)
                  (append_file AFull cap path contents))
                (\appended. match appended {
                  Ok _ |-> host_exit AFull Success ;
                  Err _ |-> host_exit AFull (Failure 84)
                })
          }
        }
      }
    }
  }
"#;

    const FS_METADATA_SOURCE: &str = r#"program capabilities FS APartial "./data"
proc main (input : ProcessInput) (caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [FS] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit APartial (Failure 90) ;
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit APartial (Failure 91) ;
        Cons file_path more |-> match more {
          Nil |-> host_exit APartial (Failure 92) ;
          Cons directory_path _ |-> match caps {
            MkProgramCaps cap |->
              bind (Coproduct (FSOp APartial) AmbientOp)
                (resp_coproduct (FSOp APartial) AmbientOp
                  (fs_resp APartial) ambient_resp)
                (Result FileError FileMetadata) ExitCode
                (inject_l (FSOp APartial) AmbientOp
                  (fs_resp APartial) ambient_resp
                  (Result FileError FileMetadata)
                  (file_metadata APartial cap file_path))
                (\file_result. match file_result {
                  Err _ |-> host_exit APartial (Failure 94) ;
                  Ok file_metadata_result |-> match file_metadata_result {
                    MkFileMetadata file_size file_kind |->
                      match eq_int file_size 5 {
                        False |-> host_exit APartial (Failure 95) ;
                        True |-> match file_kind {
                          KDirectory |-> host_exit APartial (Failure 96) ;
                          KSymlink |-> host_exit APartial (Failure 97) ;
                          KOther |-> host_exit APartial (Failure 98) ;
                          KFile |->
                            bind (Coproduct (FSOp APartial) AmbientOp)
                              (resp_coproduct (FSOp APartial) AmbientOp
                                (fs_resp APartial) ambient_resp)
                              (Result FileError FileMetadata) ExitCode
                              (inject_l (FSOp APartial) AmbientOp
                                (fs_resp APartial) ambient_resp
                                (Result FileError FileMetadata)
                                (file_metadata APartial cap directory_path))
                              (\directory_result. match directory_result {
                                Err _ |-> host_exit APartial (Failure 99) ;
                                Ok directory_metadata |-> match directory_metadata {
                                  MkFileMetadata _directory_size directory_kind |->
                                    match directory_kind {
                                      KFile |-> host_exit APartial (Failure 100) ;
                                      KSymlink |-> host_exit APartial (Failure 101) ;
                                      KOther |-> host_exit APartial (Failure 102) ;
                                      KDirectory |-> host_exit APartial Success
                                    }
                                }
                              })
                        }
                      }
                  }
                })
          }
        }
      }
    }
  }
"#;

    // Policy refusals occur at the first metadata request. Keep this source to
    // that single request so the policy test does not compile the deeper
    // two-result success program three additional times.
    const FS_METADATA_POLICY_SOURCE: &str =
        r#"program capabilities FS APartial "./data"
proc main (input : ProcessInput) (caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [FS] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit APartial (Failure 90) ;
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit APartial (Failure 91) ;
        Cons path _ |-> match caps {
          MkProgramCaps cap |->
            bind (Coproduct (FSOp APartial) AmbientOp)
              (resp_coproduct (FSOp APartial) AmbientOp
                (fs_resp APartial) ambient_resp)
              (Result FileError FileMetadata) ExitCode
              (inject_l (FSOp APartial) AmbientOp
                (fs_resp APartial) ambient_resp
                (Result FileError FileMetadata)
                (file_metadata APartial cap path))
              (\metadata_result. match metadata_result {
                Err _ |-> host_exit APartial (Failure 94) ;
                Ok _ |-> host_exit APartial Success
              })
        }
      }
    }
  }
"#;

    const FS_RENAME_SOURCE: &str = r#"program capabilities FS AFull "./data"
proc main (input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit AFull (Failure 100) ;
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit AFull (Failure 101) ;
        Cons source more |-> match more {
          Nil |-> host_exit AFull (Failure 102) ;
          Cons destination _ |-> match caps {
            MkProgramCaps cap |->
              bind (Coproduct (FSOp AFull) AmbientOp)
                (resp_coproduct (FSOp AFull) AmbientOp
                  (fs_resp AFull) ambient_resp)
                (Result FileError Unit) ExitCode
                (inject_l (FSOp AFull) AmbientOp
                  (fs_resp AFull) ambient_resp
                  (Result FileError Unit)
                  (rename_file AFull cap source destination))
                (\renamed. match renamed {
                  Ok _ |-> host_exit AFull Success ;
                  Err _ |-> host_exit AFull (Failure 104)
                })
          }
        }
      }
    }
  }
"#;

    // FileOperation matches must enumerate every constructor: a new op needs a poison arm here. Only native builds elaborate this fixture, so CI catches omissions.
    const FS_READ_DIRECTORY_SOURCE: &str = r#"program capabilities FS AFull "./data"
proc main (input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit AFull (Failure 110) ;
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit AFull (Failure 111) ;
        Cons path _ |-> match caps {
          MkProgramCaps cap |->
            bind (Coproduct (FSOp AFull) AmbientOp)
              (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
              (Result FileError (List DirEntry)) ExitCode
              (inject_l (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp
                (Result FileError (List DirEntry))
                (read_directory AFull cap path))
              (\observed. match observed {
                Err error |-> match error {
                  MkFileError file_operation _path cause |-> match file_operation {
                    OpReadDirectory |-> match cause {
                      CapabilityDenied |-> host_exit AFull (Failure 44) ;
                      NotFound |-> host_exit AFull (Failure 45) ;
                      PermissionDenied |-> host_exit AFull (Failure 46) ;
                      BrokenPipe |-> host_exit AFull (Failure 47) ;
                      Interrupted |-> host_exit AFull (Failure 48) ;
                      AlreadyExists |-> host_exit AFull (Failure 49) ;
                      InvalidInput |-> host_exit AFull (Failure 50) ;
                      IsDirectory |-> host_exit AFull (Failure 51) ;
                      NotDirectory |-> host_exit AFull (Failure 52) ;
                      NotEmpty |-> host_exit AFull (Failure 53) ;
                      Unsupported |-> host_exit AFull (Failure 54) ;
                      Revoked |-> host_exit AFull (Failure 55) ;
                      Other _ |-> host_exit AFull (Failure 56)
                    } ;
                    OpReadFile |-> host_exit AFull (Failure 199) ;
                    OpWriteFile |-> host_exit AFull (Failure 199) ;
                    OpChangeMode |-> host_exit AFull (Failure 199) ;
                    OpAppendFile |-> host_exit AFull (Failure 199) ;
                    OpMetadata |-> host_exit AFull (Failure 199) ;
                    OpCreateDirectory |-> host_exit AFull (Failure 199) ;
                    OpRemoveFile |-> host_exit AFull (Failure 199) ;
                    OpRemoveDirectory |-> host_exit AFull (Failure 199) ;
                    OpRename |-> host_exit AFull (Failure 199) ;
                    OpSeek |-> host_exit AFull (Failure 199) ;
                    OpSetLength |-> host_exit AFull (Failure 199)
                  }
                } ;
                Ok entries |-> match entries {
                  Nil |-> host_exit AFull (Failure 112) ;
                  Cons _ rest1 |-> match rest1 {
                    Nil |-> host_exit AFull (Failure 113) ;
                    Cons _ rest2 |-> match rest2 {
                      Nil |-> host_exit AFull (Failure 114) ;
                      Cons _ rest3 |-> match rest3 {
                        Nil |-> host_exit AFull Success ;
                        Cons _ _ |-> host_exit AFull (Failure 115)
                      }
                    }
                  }
                }
              })
        }
      }
    }
  }
"#;

    // FileOperation matches must enumerate every constructor: a new op needs a poison arm here. Only native builds elaborate this fixture, so CI catches omissions.
    const FS_CREATE_DIRECTORY_SOURCE: &str = r#"program capabilities FS AFull "./data"
proc main (input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit AFull (Failure 120) ;
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit AFull (Failure 121) ;
        Cons path _ |-> match caps {
          MkProgramCaps cap |->
            bind (Coproduct (FSOp AFull) AmbientOp)
              (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
              (Result FileError Unit) ExitCode
              (inject_l (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp
                (Result FileError Unit)
                (create_directory AFull cap False path))
              (\observed. match observed {
                Err error |-> match error {
                  MkFileError file_operation _path cause |-> match file_operation {
                    OpCreateDirectory |-> match cause {
                      CapabilityDenied |-> host_exit AFull (Failure 44) ;
                      NotFound |-> host_exit AFull (Failure 45) ;
                      PermissionDenied |-> host_exit AFull (Failure 46) ;
                      BrokenPipe |-> host_exit AFull (Failure 47) ;
                      Interrupted |-> host_exit AFull (Failure 48) ;
                      AlreadyExists |-> host_exit AFull (Failure 49) ;
                      InvalidInput |-> host_exit AFull (Failure 50) ;
                      IsDirectory |-> host_exit AFull (Failure 51) ;
                      NotDirectory |-> host_exit AFull (Failure 52) ;
                      NotEmpty |-> host_exit AFull (Failure 53) ;
                      Unsupported |-> host_exit AFull (Failure 54) ;
                      Revoked |-> host_exit AFull (Failure 55) ;
                      Other _ |-> host_exit AFull (Failure 56)
                    } ;
                    OpReadFile |-> host_exit AFull (Failure 199) ;
                    OpWriteFile |-> host_exit AFull (Failure 199) ;
                    OpChangeMode |-> host_exit AFull (Failure 199) ;
                    OpAppendFile |-> host_exit AFull (Failure 199) ;
                    OpMetadata |-> host_exit AFull (Failure 199) ;
                    OpReadDirectory |-> host_exit AFull (Failure 199) ;
                    OpRemoveFile |-> host_exit AFull (Failure 199) ;
                    OpRemoveDirectory |-> host_exit AFull (Failure 199) ;
                    OpRename |-> host_exit AFull (Failure 199) ;
                    OpSeek |-> host_exit AFull (Failure 199) ;
                    OpSetLength |-> host_exit AFull (Failure 199)
                  }
                } ;
                Ok _ |-> host_exit AFull Success
              })
        }
      }
    }
  }
"#;

    // FileOperation matches must enumerate every constructor: a new op needs a poison arm here. Only native builds elaborate this fixture, so CI catches omissions.
    const FS_REMOVE_FILE_SOURCE: &str = r#"program capabilities FS AFull "./data"
proc main (input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit AFull (Failure 130) ;
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit AFull (Failure 131) ;
        Cons path _ |-> match caps {
          MkProgramCaps cap |->
            bind (Coproduct (FSOp AFull) AmbientOp)
              (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
              (Result FileError Unit) ExitCode
              (inject_l (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp
                (Result FileError Unit) (remove_file AFull cap path))
              (\observed. match observed {
                Err error |-> match error {
                  MkFileError file_operation _path cause |-> match file_operation {
                    OpRemoveFile |-> match cause {
                      CapabilityDenied |-> host_exit AFull (Failure 44) ;
                      NotFound |-> host_exit AFull (Failure 45) ;
                      PermissionDenied |-> host_exit AFull (Failure 46) ;
                      BrokenPipe |-> host_exit AFull (Failure 47) ;
                      Interrupted |-> host_exit AFull (Failure 48) ;
                      AlreadyExists |-> host_exit AFull (Failure 49) ;
                      InvalidInput |-> host_exit AFull (Failure 50) ;
                      IsDirectory |-> host_exit AFull (Failure 51) ;
                      NotDirectory |-> host_exit AFull (Failure 52) ;
                      NotEmpty |-> host_exit AFull (Failure 53) ;
                      Unsupported |-> host_exit AFull (Failure 54) ;
                      Revoked |-> host_exit AFull (Failure 55) ;
                      Other _ |-> host_exit AFull (Failure 56)
                    } ;
                    OpReadFile |-> host_exit AFull (Failure 199) ;
                    OpWriteFile |-> host_exit AFull (Failure 199) ;
                    OpChangeMode |-> host_exit AFull (Failure 199) ;
                    OpAppendFile |-> host_exit AFull (Failure 199) ;
                    OpMetadata |-> host_exit AFull (Failure 199) ;
                    OpReadDirectory |-> host_exit AFull (Failure 199) ;
                    OpCreateDirectory |-> host_exit AFull (Failure 199) ;
                    OpRemoveDirectory |-> host_exit AFull (Failure 199) ;
                    OpRename |-> host_exit AFull (Failure 199) ;
                    OpSeek |-> host_exit AFull (Failure 199) ;
                    OpSetLength |-> host_exit AFull (Failure 199)
                  }
                } ;
                Ok _ |-> host_exit AFull Success
              })
        }
      }
    }
  }
"#;

    // FileOperation matches must enumerate every constructor: a new op needs a poison arm here. Only native builds elaborate this fixture, so CI catches omissions.
    const FS_REMOVE_DIRECTORY_SOURCE: &str = r#"program capabilities FS AFull "./data"
proc main (input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit AFull (Failure 140) ;
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit AFull (Failure 141) ;
        Cons path _ |-> match caps {
          MkProgramCaps cap |->
            bind (Coproduct (FSOp AFull) AmbientOp)
              (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
              (Result FileError Unit) ExitCode
              (inject_l (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp
                (Result FileError Unit)
                (remove_directory AFull cap False path))
              (\observed. match observed {
                Err error |-> match error {
                  MkFileError file_operation _path cause |-> match file_operation {
                    OpRemoveDirectory |-> match cause {
                      CapabilityDenied |-> host_exit AFull (Failure 44) ;
                      NotFound |-> host_exit AFull (Failure 45) ;
                      PermissionDenied |-> host_exit AFull (Failure 46) ;
                      BrokenPipe |-> host_exit AFull (Failure 47) ;
                      Interrupted |-> host_exit AFull (Failure 48) ;
                      AlreadyExists |-> host_exit AFull (Failure 49) ;
                      InvalidInput |-> host_exit AFull (Failure 50) ;
                      IsDirectory |-> host_exit AFull (Failure 51) ;
                      NotDirectory |-> host_exit AFull (Failure 52) ;
                      NotEmpty |-> host_exit AFull (Failure 53) ;
                      Unsupported |-> host_exit AFull (Failure 54) ;
                      Revoked |-> host_exit AFull (Failure 55) ;
                      Other _ |-> host_exit AFull (Failure 56)
                    } ;
                    OpReadFile |-> host_exit AFull (Failure 199) ;
                    OpWriteFile |-> host_exit AFull (Failure 199) ;
                    OpChangeMode |-> host_exit AFull (Failure 199) ;
                    OpAppendFile |-> host_exit AFull (Failure 199) ;
                    OpMetadata |-> host_exit AFull (Failure 199) ;
                    OpReadDirectory |-> host_exit AFull (Failure 199) ;
                    OpCreateDirectory |-> host_exit AFull (Failure 199) ;
                    OpRemoveFile |-> host_exit AFull (Failure 199) ;
                    OpRename |-> host_exit AFull (Failure 199) ;
                    OpSeek |-> host_exit AFull (Failure 199) ;
                    OpSetLength |-> host_exit AFull (Failure 199)
                  }
                } ;
                Ok _ |-> host_exit AFull Success
              })
        }
      }
    }
  }
"#;

    const DENIAL_SOURCE: &str = r#"program capabilities FS AFull
proc main (input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit AFull (Failure 40) ;
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit AFull (Failure 41) ;
        Cons path _ |-> match caps {
          MkProgramCaps cap |->
            bind (Coproduct (FSOp AFull) AmbientOp)
              (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
              (Result FileError Unit) ExitCode
              (inject_l (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp
                (Result FileError Unit) (writeFile cap path CreateNew path))
              (\written. match written {
                Ok _ |-> host_exit AFull (Failure 43) ;
                Err error |-> match error {
                  MkFileError _operation _path cause |-> match cause {
                    CapabilityDenied |-> host_exit AFull (Failure 44) ;
                    NotFound |-> host_exit AFull (Failure 45) ;
                    PermissionDenied |-> host_exit AFull (Failure 46) ;
                    BrokenPipe |-> host_exit AFull (Failure 47) ;
                    Interrupted |-> host_exit AFull (Failure 48) ;
                    AlreadyExists |-> host_exit AFull (Failure 49) ;
                    InvalidInput |-> host_exit AFull (Failure 50) ;
                    IsDirectory |-> host_exit AFull (Failure 51) ;
                    NotDirectory |-> host_exit AFull (Failure 52) ;
                    NotEmpty |-> host_exit AFull (Failure 53) ;
                    Unsupported |-> host_exit AFull (Failure 54) ;
                    Revoked |-> host_exit AFull (Failure 54) ;
                    Other _ |-> host_exit AFull (Failure 55)
                  }
                }
              })
        }
      }
    }
  }
"#;

    const RAW_COLLISION_SOURCE: &str = r#"program capabilities FS AFull
proc main (input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit AFull (Failure 60) ;
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit AFull (Failure 61) ;
        Cons raw_path more |-> match more {
          Nil |-> host_exit AFull (Failure 62) ;
          Cons normalized_path _ |-> match caps {
            MkProgramCaps cap |->
              bind (Coproduct (FSOp AFull) AmbientOp)
                (resp_coproduct (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp)
                (Result FileError Bytes) ExitCode
                (inject_l (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp
                  (Result FileError Bytes) (readFile AFull cap raw_path))
                (\first. match first {
                  Err _ |-> host_exit AFull (Failure 63) ;
                  Ok _ |->
                    bind (Coproduct (FSOp AFull) AmbientOp)
                      (resp_coproduct (FSOp AFull) AmbientOp
                        (fs_resp AFull) ambient_resp)
                      (Result FileError Bytes) ExitCode
                      (inject_l (FSOp AFull) AmbientOp (fs_resp AFull) ambient_resp
                        (Result FileError Bytes) (readFile AFull cap normalized_path))
                      (\second. match second {
                        Err _ |-> host_exit AFull (Failure 64) ;
                        Ok _ |-> host_exit AFull Success
                      })
                })
          }
        }
      }
    }
  }
"#;

    const CHANGE_MODE_SOURCE: &str = r#"program capabilities FS AFull
proc main (input : ProcessInput) (caps : ProgramCaps AFull)
  : HostIO AFull ExitCode visits [FS] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit AFull (Failure 70) ;
      Cons _argv0 rest |-> match rest {
        Nil |-> host_exit AFull (Failure 71) ;
        Cons path _ |-> match caps {
          MkProgramCaps cap |->
            bind (Coproduct (FSOp AFull) AmbientOp)
              (resp_coproduct (FSOp AFull) AmbientOp
                (fs_resp AFull) ambient_resp)
              (Result FileError Unit) ExitCode
              (inject_l (FSOp AFull) AmbientOp
                (fs_resp AFull) ambient_resp
                (Result FileError Unit)
                (change_mode AFull cap path 416))
              (\changed. match changed {
                Err _ |-> host_exit AFull (Failure 72) ;
                Ok _ |-> host_exit AFull Success
              })
        }
      }
    }
  }
"#;

    fn five_op_scenario() -> Scenario {
        let path = b"dir/./px6.bin".to_vec();
        let bytes = vec![b'r', 0xff, b'x'];
        Scenario {
            process_input: RawProcessInput {
                arguments: vec![path.clone(), bytes.clone()],
                environment: vec![(b"PX6_ENV".to_vec(), vec![0xfe, b'v'])],
            },
            ambient: AmbientScript::default(),
            program_caps: ProgramCapsShape::default(),
            entry: CheckedProgramEntry {
                identity: "px6-five-op-real-artifact".to_string(),
                package_name: "px6-five-op-real-artifact".to_string(),
                source: FIVE_OP_SOURCE.to_string(),
            },
            initial_filesystem: vec![SeedNode {
                relative_path: b"dir".to_vec(),
                kind: crate::SeedNodeKind::Directory,
            }],
            expected_fs: vec![
                ExpectedFsEffect::WriteFile {
                    path: path.clone(),
                    create_policy: CreatePolicyV1::CreateNew,
                    bytes,
                },
                ExpectedFsEffect::ReadFile { path },
            ],
        }
    }

    fn clock_wall_scenario() -> Scenario {
        Scenario {
            process_input: RawProcessInput::default(),
            ambient: AmbientScript {
                use_real_wall_clock: true,
                ..AmbientScript::default()
            },
            program_caps: ProgramCapsShape::default(),
            entry: CheckedProgramEntry {
                identity: "abi-a1-clock-wall-real-artifact".to_string(),
                package_name: "abi-a1-clock-wall-real-artifact".to_string(),
                source: CLOCK_WALL_SOURCE.to_string(),
            },
            initial_filesystem: Vec::new(),
            expected_fs: Vec::new(),
        }
    }

    fn console_read_scenario() -> Scenario {
        Scenario {
            process_input: RawProcessInput::default(),
            ambient: AmbientScript {
                stdin: vec![0xff, b'a', 0],
                ..AmbientScript::default()
            },
            program_caps: ProgramCapsShape::default(),
            entry: CheckedProgramEntry {
                identity: "abi-a1-console-read-real-artifact".to_string(),
                package_name: "abi-a1-console-read-real-artifact".to_string(),
                source: CONSOLE_READ_SOURCE.to_string(),
            },
            initial_filesystem: Vec::new(),
            expected_fs: Vec::new(),
        }
    }

    fn fs_append_scenario(
        identity: &str,
        source: String,
        authority: Authority,
        rights: RightSet,
        path: Vec<u8>,
        bytes: Vec<u8>,
        initial_filesystem: Vec<SeedNode>,
    ) -> Scenario {
        Scenario {
            process_input: RawProcessInput {
                arguments: vec![path.clone(), bytes.clone()],
                environment: Vec::new(),
            },
            ambient: AmbientScript::default(),
            program_caps: ProgramCapsShape {
                fs_authority: authority,
                relative_root: b"data".to_vec(),
                rights,
                symlink: SymlinkPolicy::NoFollow,
            },
            entry: CheckedProgramEntry {
                identity: identity.to_string(),
                package_name: identity.to_string(),
                source,
            },
            initial_filesystem,
            expected_fs: vec![ExpectedFsEffect::AppendFile { path, bytes }],
        }
    }

    fn fs_append_success_scenario() -> Scenario {
        fs_append_scenario(
            "abi-a2-fs-append-success",
            FS_APPEND_SOURCE.to_string(),
            AUTH_FULL,
            RightSet::ALL,
            b"file.bin".to_vec(),
            vec![0xff, b'x', 0xfe],
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"data/file.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"before".to_vec()),
                },
            ],
        )
    }

    fn fs_append_escape_scenario() -> Scenario {
        fs_append_scenario(
            "abi-a2-fs-append-scope-escape",
            FS_APPEND_SOURCE.to_string(),
            AUTH_FULL,
            RightSet::ALL,
            b"../outside.bin".to_vec(),
            b"forbidden".to_vec(),
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"outside.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"outside-before".to_vec()),
                },
            ],
        )
    }

    fn fs_append_symlink_scenario() -> Scenario {
        fs_append_scenario(
            "abi-a2-fs-append-symlink-denied",
            FS_APPEND_SOURCE.to_string(),
            AUTH_FULL,
            RightSet::ALL,
            b"link".to_vec(),
            b"forbidden".to_vec(),
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"outside.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"outside-before".to_vec()),
                },
                SeedNode {
                    relative_path: b"data/link".to_vec(),
                    kind: crate::SeedNodeKind::Symlink(b"../outside.bin".to_vec()),
                },
            ],
        )
    }

    fn fs_append_missing_right_scenario() -> Scenario {
        let source = FS_APPEND_SOURCE.replace("AFull", "APartial");
        let rights = RightSet::READ
            .union(RightSet::ENUMERATE)
            .union(RightSet::METADATA);
        fs_append_scenario(
            "abi-a2-fs-append-missing-right",
            source,
            AUTH_PARTIAL,
            rights,
            b"file.bin".to_vec(),
            b"forbidden".to_vec(),
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"data/file.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"before".to_vec()),
                },
            ],
        )
    }

    fn fs_metadata_scenario(
        identity: &str,
        source: &str,
        rights: RightSet,
        arguments: Vec<Vec<u8>>,
        initial_filesystem: Vec<SeedNode>,
        expected_paths: Vec<Vec<u8>>,
    ) -> Scenario {
        Scenario {
            process_input: RawProcessInput {
                arguments,
                environment: Vec::new(),
            },
            ambient: AmbientScript::default(),
            program_caps: ProgramCapsShape {
                fs_authority: AUTH_PARTIAL,
                relative_root: b"data".to_vec(),
                rights,
                symlink: SymlinkPolicy::NoFollow,
            },
            entry: CheckedProgramEntry {
                identity: identity.to_string(),
                package_name: identity.to_string(),
                source: source.to_string(),
            },
            initial_filesystem,
            expected_fs: expected_paths
                .into_iter()
                .map(|path| ExpectedFsEffect::Metadata { path })
                .collect(),
        }
    }

    fn fs_metadata_success_scenario() -> Scenario {
        fs_metadata_scenario(
            "abi-a2-fs-metadata-success",
            FS_METADATA_SOURCE,
            RightSet::METADATA,
            vec![b"file.bin".to_vec(), b"known-dir".to_vec()],
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"data/file.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"five!".to_vec()),
                },
                SeedNode {
                    relative_path: b"data/known-dir".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
            ],
            vec![b"file.bin".to_vec(), b"known-dir".to_vec()],
        )
    }

    fn fs_metadata_escape_scenario() -> Scenario {
        fs_metadata_scenario(
            "abi-a2-fs-metadata-scope-escape",
            FS_METADATA_POLICY_SOURCE,
            RightSet::METADATA,
            vec![b"../outside.bin".to_vec()],
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"outside.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"five!".to_vec()),
                },
            ],
            vec![b"../outside.bin".to_vec()],
        )
    }

    fn fs_metadata_symlink_scenario() -> Scenario {
        fs_metadata_scenario(
            "abi-a2-fs-metadata-symlink-denied",
            FS_METADATA_POLICY_SOURCE,
            RightSet::METADATA,
            vec![b"link".to_vec()],
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"outside.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"five!".to_vec()),
                },
                SeedNode {
                    relative_path: b"data/link".to_vec(),
                    kind: crate::SeedNodeKind::Symlink(b"../outside.bin".to_vec()),
                },
            ],
            vec![b"link".to_vec()],
        )
    }

    fn fs_metadata_missing_right_scenario() -> Scenario {
        let mut scenario = fs_metadata_scenario(
            "abi-a2-fs-metadata-missing-right",
            FS_METADATA_POLICY_SOURCE,
            RightSet::NONE,
            vec![b"file.bin".to_vec()],
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"data/file.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"five!".to_vec()),
                },
            ],
            vec![b"file.bin".to_vec()],
        );
        scenario.entry.source = scenario.entry.source.replace("APartial", "ANone");
        scenario.program_caps.fs_authority = AUTH_NONE;
        scenario
    }

    fn fs_rename_scenario(
        identity: &str,
        authority: Authority,
        rights: RightSet,
        source: Vec<u8>,
        destination: Vec<u8>,
        initial_filesystem: Vec<SeedNode>,
    ) -> Scenario {
        let program_source = if authority == AUTH_FULL {
            FS_RENAME_SOURCE.to_string()
        } else {
            FS_RENAME_SOURCE.replace("AFull", "ANone")
        };
        Scenario {
            process_input: RawProcessInput {
                arguments: vec![source.clone(), destination.clone()],
                environment: Vec::new(),
            },
            ambient: AmbientScript::default(),
            program_caps: ProgramCapsShape {
                fs_authority: authority,
                relative_root: b"data".to_vec(),
                rights,
                symlink: SymlinkPolicy::NoFollow,
            },
            entry: CheckedProgramEntry {
                identity: identity.to_string(),
                package_name: identity.to_string(),
                source: program_source,
            },
            initial_filesystem,
            expected_fs: vec![ExpectedFsEffect::Rename {
                source,
                destination,
            }],
        }
    }

    fn fs_rename_success_scenario() -> Scenario {
        fs_rename_scenario(
            "abi-a2-fs-rename-success",
            AUTH_FULL,
            RightSet::ALL,
            b"a.bin".to_vec(),
            b"b.bin".to_vec(),
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"data/a.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"original".to_vec()),
                },
            ],
        )
    }

    fn fs_rename_source_escape_scenario() -> Scenario {
        fs_rename_scenario(
            "abi-a2-fs-rename-source-escape",
            AUTH_FULL,
            RightSet::ALL,
            b"../outside.bin".to_vec(),
            b"b.bin".to_vec(),
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"outside.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"outside".to_vec()),
                },
            ],
        )
    }

    fn fs_rename_destination_escape_scenario() -> Scenario {
        fs_rename_scenario(
            "abi-a2-fs-rename-destination-escape",
            AUTH_FULL,
            RightSet::ALL,
            b"a.bin".to_vec(),
            b"../outside.bin".to_vec(),
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"data/a.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"original".to_vec()),
                },
                SeedNode {
                    relative_path: b"outside.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"outside".to_vec()),
                },
            ],
        )
    }

    fn fs_rename_source_symlink_scenario() -> Scenario {
        fs_rename_scenario(
            "abi-a2-fs-rename-source-symlink-denied",
            AUTH_FULL,
            RightSet::ALL,
            b"link".to_vec(),
            b"b.bin".to_vec(),
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"outside.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"outside".to_vec()),
                },
                SeedNode {
                    relative_path: b"data/link".to_vec(),
                    kind: crate::SeedNodeKind::Symlink(b"../outside.bin".to_vec()),
                },
            ],
        )
    }

    fn fs_rename_destination_symlink_scenario() -> Scenario {
        fs_rename_scenario(
            "abi-a2-fs-rename-destination-symlink-denied",
            AUTH_FULL,
            RightSet::ALL,
            b"a.bin".to_vec(),
            b"link".to_vec(),
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"data/a.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"original".to_vec()),
                },
                SeedNode {
                    relative_path: b"outside.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"outside".to_vec()),
                },
                SeedNode {
                    relative_path: b"data/link".to_vec(),
                    kind: crate::SeedNodeKind::Symlink(b"../outside.bin".to_vec()),
                },
            ],
        )
    }

    fn fs_rename_missing_right_scenario() -> Scenario {
        fs_rename_scenario(
            "abi-a2-fs-rename-missing-right",
            AUTH_NONE,
            RightSet::NONE,
            b"a.bin".to_vec(),
            b"b.bin".to_vec(),
            vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"data/a.bin".to_vec(),
                    kind: crate::SeedNodeKind::File(b"original".to_vec()),
                },
            ],
        )
    }

    fn abi_a3_scenario(
        identity: &str,
        source: String,
        authority: Authority,
        rights: RightSet,
        path: Vec<u8>,
        initial_filesystem: Vec<SeedNode>,
        expected_fs: ExpectedFsEffect,
    ) -> Scenario {
        Scenario {
            process_input: RawProcessInput {
                arguments: vec![path],
                environment: Vec::new(),
            },
            ambient: AmbientScript::default(),
            program_caps: ProgramCapsShape {
                fs_authority: authority,
                relative_root: b"data".to_vec(),
                rights,
                symlink: SymlinkPolicy::NoFollow,
            },
            entry: CheckedProgramEntry {
                identity: identity.to_string(),
                package_name: identity.to_string(),
                source,
            },
            initial_filesystem,
            expected_fs: vec![expected_fs],
        }
    }

    fn abi_a3_base_nodes() -> Vec<SeedNode> {
        vec![SeedNode {
            relative_path: b"data".to_vec(),
            kind: crate::SeedNodeKind::Directory,
        }]
    }

    fn fs_read_directory_success_scenario() -> Scenario {
        let mut nodes = abi_a3_base_nodes();
        nodes.extend([
            SeedNode {
                relative_path: b"data/listing".to_vec(),
                kind: crate::SeedNodeKind::Directory,
            },
            SeedNode {
                relative_path: b"data/listing/file.bin".to_vec(),
                kind: crate::SeedNodeKind::File(b"payload".to_vec()),
            },
            SeedNode {
                relative_path: b"data/listing/subdir".to_vec(),
                kind: crate::SeedNodeKind::Directory,
            },
            SeedNode {
                relative_path: b"data/listing/link".to_vec(),
                kind: crate::SeedNodeKind::Symlink(b"file.bin".to_vec()),
            },
        ]);
        abi_a3_scenario(
            "abi-a3-fs-read-directory-success",
            FS_READ_DIRECTORY_SOURCE.to_string(),
            AUTH_FULL,
            RightSet::ALL,
            b"listing".to_vec(),
            nodes,
            ExpectedFsEffect::ReadDirectory {
                path: b"listing".to_vec(),
            },
        )
    }

    fn fs_create_directory_scenario(
        identity: &str,
        recursive: bool,
        path: &[u8],
        nodes: Vec<SeedNode>,
    ) -> Scenario {
        let source = if recursive {
            FS_CREATE_DIRECTORY_SOURCE.replace("cap False", "cap True")
        } else {
            FS_CREATE_DIRECTORY_SOURCE.to_string()
        };
        abi_a3_scenario(
            identity,
            source,
            AUTH_FULL,
            RightSet::ALL,
            path.to_vec(),
            nodes,
            ExpectedFsEffect::CreateDirectory {
                path: path.to_vec(),
                recursive,
            },
        )
    }

    fn fs_remove_file_scenario(
        identity: &str,
        path: &[u8],
        nodes: Vec<SeedNode>,
    ) -> Scenario {
        abi_a3_scenario(
            identity,
            FS_REMOVE_FILE_SOURCE.to_string(),
            AUTH_FULL,
            RightSet::ALL,
            path.to_vec(),
            nodes,
            ExpectedFsEffect::RemoveFile {
                path: path.to_vec(),
            },
        )
    }

    fn fs_remove_directory_scenario(
        identity: &str,
        recursive: bool,
        path: &[u8],
        nodes: Vec<SeedNode>,
    ) -> Scenario {
        let source = if recursive {
            FS_REMOVE_DIRECTORY_SOURCE.replace("cap False", "cap True")
        } else {
            FS_REMOVE_DIRECTORY_SOURCE.to_string()
        };
        abi_a3_scenario(
            identity,
            source,
            AUTH_FULL,
            RightSet::ALL,
            path.to_vec(),
            nodes,
            ExpectedFsEffect::RemoveDirectory {
                path: path.to_vec(),
                recursive,
            },
        )
    }

    #[derive(Clone, Copy)]
    enum AbiA3PolicyCase {
        Escape,
        Symlink,
        MissingRight,
    }

    fn abi_a3_policy_scenario(
        operation: HostOpV1,
        policy: AbiA3PolicyCase,
    ) -> (Scenario, CapabilityDeniedV1) {
        let (source, expected_fs, required) = match operation {
            HostOpV1::FsReadDirectory => (
                FS_READ_DIRECTORY_SOURCE.to_string(),
                ExpectedFsEffect::ReadDirectory {
                    path: Vec::new(),
                },
                ken_host::FsCapabilityOperationV1::Enumerate,
            ),
            HostOpV1::FsCreateDirectory => (
                FS_CREATE_DIRECTORY_SOURCE.to_string(),
                ExpectedFsEffect::CreateDirectory {
                    path: Vec::new(),
                    recursive: false,
                },
                ken_host::FsCapabilityOperationV1::CreateDirectory,
            ),
            HostOpV1::FsRemoveFile => (
                FS_REMOVE_FILE_SOURCE.to_string(),
                ExpectedFsEffect::RemoveFile {
                    path: Vec::new(),
                },
                ken_host::FsCapabilityOperationV1::RemoveFile,
            ),
            HostOpV1::FsRemoveDirectory => (
                FS_REMOVE_DIRECTORY_SOURCE.to_string(),
                ExpectedFsEffect::RemoveDirectory {
                    path: Vec::new(),
                    recursive: false,
                },
                ken_host::FsCapabilityOperationV1::RemoveDirectory,
            ),
            _ => unreachable!("ABI-A3 policy has exactly four operations"),
        };
        let mut nodes = abi_a3_base_nodes();
        let (identity, path, authority, rights, source, expected) = match policy {
            AbiA3PolicyCase::Escape => {
                nodes.push(SeedNode {
                    relative_path: b"outside".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                });
                (
                    "escape",
                    b"../outside".to_vec(),
                    AUTH_FULL,
                    RightSet::ALL,
                    source,
                    CapabilityDeniedV1::ScopeEscape,
                )
            }
            AbiA3PolicyCase::Symlink => {
                nodes.extend([
                    SeedNode {
                        relative_path: b"outside".to_vec(),
                        kind: crate::SeedNodeKind::Directory,
                    },
                    SeedNode {
                        relative_path: b"data/link".to_vec(),
                        kind: crate::SeedNodeKind::Symlink(
                            b"../outside".to_vec(),
                        ),
                    },
                ]);
                (
                    "symlink",
                    b"link".to_vec(),
                    AUTH_FULL,
                    RightSet::ALL,
                    source,
                    CapabilityDeniedV1::SymlinkDenied,
                )
            }
            AbiA3PolicyCase::MissingRight => {
                nodes.push(SeedNode {
                    relative_path: b"data/target".to_vec(),
                    kind: if operation == HostOpV1::FsRemoveFile {
                        crate::SeedNodeKind::File(b"payload".to_vec())
                    } else {
                        crate::SeedNodeKind::Directory
                    },
                });
                (
                    "missing-right",
                    b"target".to_vec(),
                    AUTH_NONE,
                    RightSet::NONE,
                    source.replace("AFull", "ANone"),
                    CapabilityDeniedV1::RightNotHeld {
                        operation: required,
                        held_rights: RightSet::NONE.bits(),
                    },
                )
            }
        };
        let expected_fs = match expected_fs {
            ExpectedFsEffect::ReadDirectory { .. } => {
                ExpectedFsEffect::ReadDirectory { path: path.clone() }
            }
            ExpectedFsEffect::CreateDirectory { recursive, .. } => {
                ExpectedFsEffect::CreateDirectory {
                    path: path.clone(),
                    recursive,
                }
            }
            ExpectedFsEffect::RemoveFile { .. } => {
                ExpectedFsEffect::RemoveFile { path: path.clone() }
            }
            ExpectedFsEffect::RemoveDirectory { recursive, .. } => {
                ExpectedFsEffect::RemoveDirectory {
                    path: path.clone(),
                    recursive,
                }
            }
            _ => unreachable!("ABI-A3 policy expected-effect set"),
        };
        let scenario = abi_a3_scenario(
            &format!("abi-a3-{operation:?}-{identity}"),
            source,
            authority,
            rights,
            path.clone(),
            nodes,
            expected_fs,
        );
        (scenario, expected)
    }

    fn denial_scenario() -> Scenario {
        let path = b"../escape".to_vec();
        Scenario {
            process_input: RawProcessInput {
                arguments: vec![path.clone()],
                environment: Vec::new(),
            },
            ambient: AmbientScript::default(),
            program_caps: ProgramCapsShape::default(),
            entry: CheckedProgramEntry {
                identity: "px6-denial-real-artifact".to_string(),
                package_name: "px6-denial-real-artifact".to_string(),
                source: DENIAL_SOURCE.to_string(),
            },
            initial_filesystem: Vec::new(),
            expected_fs: vec![ExpectedFsEffect::WriteFile {
                path: path.clone(),
                create_policy: CreatePolicyV1::CreateNew,
                bytes: path,
            }],
        }
    }

    fn cwd_root_denial_scenario(path: Vec<u8>, symlink: bool) -> Scenario {
        let mut scenario = denial_scenario();
        scenario.entry.identity = if symlink {
            "px15-cwd-root-symlink-denial"
        } else {
            "px15-cwd-root-scope-denial"
        }
        .to_string();
        scenario.entry.package_name = scenario.entry.identity.clone();
        scenario.entry.source = scenario.entry.source.replacen(
            "program capabilities FS AFull",
            r#"program capabilities FS AFull "./data""#,
            1,
        );
        scenario.process_input.arguments = vec![path.clone()];
        scenario.initial_filesystem = vec![SeedNode {
            relative_path: b"data".to_vec(),
            kind: crate::SeedNodeKind::Directory,
        }];
        if symlink {
            scenario.initial_filesystem.push(SeedNode {
                relative_path: b"data/link".to_vec(),
                kind: crate::SeedNodeKind::Symlink(b"../../outside".to_vec()),
            });
        }
        scenario.expected_fs = vec![ExpectedFsEffect::WriteFile {
            path: path.clone(),
            create_policy: CreatePolicyV1::CreateNew,
            bytes: path,
        }];
        scenario
    }

    fn raw_descriptor_collision_scenario() -> Scenario {
        let raw_path = b"dir/./x".to_vec();
        let normalized_path = b"dir/x".to_vec();
        Scenario {
            process_input: RawProcessInput {
                arguments: vec![raw_path.clone(), normalized_path.clone()],
                environment: Vec::new(),
            },
            ambient: AmbientScript::default(),
            program_caps: ProgramCapsShape::default(),
            entry: CheckedProgramEntry {
                identity: "px6-raw-descriptor-collision".to_string(),
                package_name: "px6-raw-descriptor-collision".to_string(),
                source: RAW_COLLISION_SOURCE.to_string(),
            },
            initial_filesystem: vec![SeedNode {
                relative_path: normalized_path.clone(),
                kind: crate::SeedNodeKind::File(b"same-node".to_vec()),
            }],
            expected_fs: vec![
                ExpectedFsEffect::ReadFile { path: raw_path },
                ExpectedFsEffect::ReadFile {
                    path: normalized_path,
                },
            ],
        }
    }

    fn execution_start_cwd_root_scenario() -> Scenario {
        let path = b"x".to_vec();
        Scenario {
            process_input: RawProcessInput {
                arguments: vec![path.clone(), path.clone()],
                environment: Vec::new(),
            },
            ambient: AmbientScript::default(),
            program_caps: ProgramCapsShape::default(),
            entry: CheckedProgramEntry {
                identity: "px15-execution-start-cwd-root".to_string(),
                package_name: "px15-execution-start-cwd-root".to_string(),
                source: RAW_COLLISION_SOURCE.replacen(
                    "program capabilities FS AFull",
                    r#"program capabilities FS AFull "./data""#,
                    1,
                ),
            },
            initial_filesystem: vec![
                SeedNode {
                    relative_path: b"data".to_vec(),
                    kind: crate::SeedNodeKind::Directory,
                },
                SeedNode {
                    relative_path: b"data/x".to_vec(),
                    kind: crate::SeedNodeKind::File(b"cwd-root".to_vec()),
                },
            ],
            expected_fs: vec![
                ExpectedFsEffect::ReadFile { path: path.clone() },
                ExpectedFsEffect::ReadFile { path },
            ],
        }
    }

    fn change_mode_scenario() -> Scenario {
        let path = b"mode.bin".to_vec();
        Scenario {
            process_input: RawProcessInput {
                arguments: vec![path.clone()],
                environment: Vec::new(),
            },
            ambient: AmbientScript::default(),
            program_caps: ProgramCapsShape::default(),
            entry: CheckedProgramEntry {
                identity: "px13-change-mode-real-artifact".to_string(),
                package_name: "px13-change-mode-real-artifact".to_string(),
                source: CHANGE_MODE_SOURCE.to_string(),
            },
            initial_filesystem: vec![SeedNode {
                relative_path: path.clone(),
                kind: crate::SeedNodeKind::File(b"mode-retained".to_vec()),
            }],
            expected_fs: vec![ExpectedFsEffect::ChangeMode { path, mode: 0o640 }],
        }
    }

    fn change_directory_mode_scenario() -> Scenario {
        let path = b"mode-dir".to_vec();
        Scenario {
            process_input: RawProcessInput {
                arguments: vec![path.clone()],
                environment: Vec::new(),
            },
            ambient: AmbientScript::default(),
            program_caps: ProgramCapsShape::default(),
            entry: CheckedProgramEntry {
                identity: "px13-change-directory-mode-real-artifact".to_string(),
                package_name: "px13-change-directory-mode-real-artifact".to_string(),
                source: CHANGE_MODE_SOURCE.to_string(),
            },
            initial_filesystem: vec![SeedNode {
                relative_path: path.clone(),
                kind: crate::SeedNodeKind::Directory,
            }],
            expected_fs: vec![ExpectedFsEffect::ChangeMode { path, mode: 0o640 }],
        }
    }

    fn invalid_change_mode_scenario() -> Scenario {
        let mut scenario = change_mode_scenario();
        scenario.entry.identity = "px13-invalid-change-mode".to_string();
        scenario.entry.package_name = "px13-invalid-change-mode".to_string();
        scenario.entry.source = scenario.entry.source.replace(
            "change_mode AFull cap path 416",
            "change_mode AFull cap path 4096",
        );
        scenario.expected_fs.clear();
        scenario
    }

    #[derive(Default)]
    struct NoLeafBackend {
        calls: u64,
    }

    impl HostEffectBackendV1 for NoLeafBackend {
        fn console_write(
            &mut self,
            _stream: ConsoleStreamV1,
            _bytes: &[u8],
        ) -> Result<(), IoErrorIdentityV1> {
            self.calls += 1;
            Ok(())
        }

        fn console_flush(&mut self, _stream: ConsoleStreamV1) -> Result<(), IoErrorIdentityV1> {
            self.calls += 1;
            Ok(())
        }

        fn console_is_terminal(&mut self, _stream: ConsoleStreamV1) -> bool {
            self.calls += 1;
            false
        }

        fn fs_read_file(
            &mut self,
            _grant: &CapabilityGrantV1,
            _path: &[u8],
        ) -> Result<Vec<u8>, FileErrorCauseV1> {
            self.calls += 1;
            Ok(Vec::new())
        }

        fn fs_write_file(
            &mut self,
            _grant: &CapabilityGrantV1,
            _path: &[u8],
            _create_policy: CreatePolicyV1,
            _bytes: &[u8],
        ) -> Result<(), FileErrorCauseV1> {
            self.calls += 1;
            Ok(())
        }
    }

    /// Promise class: durable invariant. ABI-A1's operation-specific
    /// differential applies one identical projection to both real lanes. It
    /// retains the complete response shape and every non-clock observation
    /// field, requires two non-decreasing
    /// readings inside each lane's controlled wall window, and erases only the
    /// instant bytes before equality.
    ///
    /// The wrong-subject controls independently violate ordering, the measured
    /// window, exact Clock event shape, and a non-clock observation field. Each
    /// must be rejected by the same projection that accepts the real pair.
    #[test]
    fn clock_wall_now_normalized_real_artifact_differential_discriminates() {
        let run = execute_scenario(&clock_wall_scenario())
            .expect("ClockWallNow real-artifact differential executes");
        run.compare_clock_wall_now()
            .expect("symmetric temporal normalization agrees");
        assert_eq!(
            confirm_native_tested_transition(
                HostOpV1::ClockWallNow,
                NativeTestedEvidence::from_clock_wall_now_run(&run),
            ),
            Ok(HostOpAvailabilityV1::NativeTested),
            "the promoted lane must be backed by normalized real-artifact evidence"
        );

        let mut wrong_native = run.native.clone();
        let [first, second] = wrong_native.effect_trace.as_mut_slice() else {
            panic!("clock fixture must produce two native observations")
        };
        let CanonicalOutcomeV1::Success(CanonicalReplyV1::Instant(first_bytes)) =
            &first.outcome
        else {
            panic!("first native response must be Instant")
        };
        let first_reading = BigInt::from_signed_bytes_be(first_bytes);
        let CanonicalOutcomeV1::Success(CanonicalReplyV1::Instant(second_bytes)) =
            &mut second.outcome
        else {
            panic!("second native response must be Instant")
        };
        *second_bytes = (first_reading - BigInt::from(1u8)).to_signed_bytes_be();
        assert!(matches!(
            normalize_clock_wall_now_observation(
                "mutated native",
                &wrong_native,
                run.native_wall_window,
            ),
            Err(ClockWallNowDifferentialError::WentBackwards {
                lane: "mutated native",
                ..
            })
        ));

        let mut outside_window = run.native.clone();
        let outside_reading = BigInt::from(run.native_wall_window.end_nanoseconds)
            + BigInt::from(1u8);
        let outside_reading = outside_reading.to_signed_bytes_be();
        for event in &mut outside_window.effect_trace {
            let CanonicalOutcomeV1::Success(CanonicalReplyV1::Instant(bytes)) =
                &mut event.outcome
            else {
                panic!("native response must be Instant")
            };
            *bytes = outside_reading.clone();
        }
        assert!(matches!(
            normalize_clock_wall_now_observation(
                "outside-window native",
                &outside_window,
                run.native_wall_window,
            ),
            Err(ClockWallNowDifferentialError::OutsidePlausibleWindow {
                lane: "outside-window native",
                ..
            })
        ));

        let mut wrong_shape = run.native.clone();
        wrong_shape.effect_trace[1].operation = HostOpV1::ClockMonotonicNow;
        assert!(matches!(
            normalize_clock_wall_now_observation(
                "wrong-shape native",
                &wrong_shape,
                run.native_wall_window,
            ),
            Err(ClockWallNowDifferentialError::TraceShape {
                lane: "wrong-shape native",
                ..
            })
        ));

        let mut wrong_non_clock_field = run.native.clone();
        wrong_non_clock_field.stdout.push(b'x');
        assert!(matches!(
            compare_clock_wall_now_observations(
                &run.interpreter,
                run.interpreter_wall_window,
                &wrong_non_clock_field,
                run.native_wall_window,
            ),
            Err(ClockWallNowDifferentialError::NormalizedMismatch(_))
        ));
    }

    /// Promise class: durable invariant. Both lanes read the same finite stdin
    /// fixture at limits 2, 4, and 4. The projection requires the exact request
    /// sequence, a two-byte Chunk, a one-byte partial Chunk, then EOF, and keeps
    /// every reply byte for the final canonical comparison.
    #[test]
    fn console_read_normalized_real_artifact_differential_discriminates() {
        const STDIN: &[u8] = &[0xff, b'a', 0];
        const LIMITS: &[u64] = &[2, 4, 4];
        let run = execute_scenario(&console_read_scenario())
            .expect("ConsoleRead real-artifact differential executes");
        run.compare_console_read(STDIN, LIMITS)
            .expect("the finite-stdin projection agrees");
        assert_eq!(
            crate::confirm_native_tested_transition(
                ken_host::HostOpV1::ConsoleRead,
                crate::NativeTestedEvidence::from_console_read_run(
                    &run, STDIN, LIMITS,
                ),
            ),
            Ok(ken_host::HostOpAvailabilityV1::NativeTested)
        );

        let mut wrong_native = run.native.clone();
        let ken_host::CanonicalOutcomeV1::Success(
            ken_host::CanonicalReplyV1::ReadChunk(bytes),
        ) = &mut wrong_native.effect_trace[1].outcome
        else {
            panic!("the second native response must be a partial Chunk")
        };
        bytes.clear();
        assert!(matches!(
            compare_console_read_observations(
                &run.interpreter,
                &wrong_native,
                STDIN,
                LIMITS,
            ),
            Err(ConsoleReadDifferentialError::FixtureMismatch {
                lane: "native",
                event: 1,
                ..
            })
        ));
    }

    /// Promise class: durable invariant over the provisional whole-directory
    /// payload. Intended streaming replacement keeps this set-level test green.
    #[test]
    fn fs_read_directory_real_artifact_is_order_independent_and_discriminating() {
        let expected = vec![
            ken_host::DirEntryV1 {
                name: b"file.bin".to_vec(),
                kind: ken_host::FsNodeKindV1::File,
            },
            ken_host::DirEntryV1 {
                name: b"subdir".to_vec(),
                kind: ken_host::FsNodeKindV1::Directory,
            },
            ken_host::DirEntryV1 {
                name: b"link".to_vec(),
                kind: ken_host::FsNodeKindV1::Symlink,
            },
        ];
        let run = execute_scenario(&fs_read_directory_success_scenario())
            .expect("FsReadDirectory real-artifact differential executes");
        run.compare_fs_read_directory(b"listing", &expected)
            .expect("name/kind set parity");
        assert_eq!(run.interpreter_actions.fs_actions_after_resolve, Some(1));
        assert_eq!(
            confirm_native_tested_transition(
                HostOpV1::FsReadDirectory,
                NativeTestedEvidence::from_fs_read_directory_run(
                    &run,
                    b"listing",
                    &expected,
                ),
            ),
            Ok(HostOpAvailabilityV1::NativeTested)
        );

        let mut reordered = run.native.clone();
        let CanonicalOutcomeV1::Success(CanonicalReplyV1::DirectoryEntries(entries)) =
            &mut reordered.effect_trace[0].outcome
        else {
            panic!("listing fixture must return DirectoryEntries")
        };
        entries.reverse();
        compare_fs_read_directory_observations(
            &run.interpreter,
            &reordered,
            b"listing",
            &expected,
        )
        .expect("raw listing order is not contractual");

        let mut missing = run.native.clone();
        let CanonicalOutcomeV1::Success(CanonicalReplyV1::DirectoryEntries(entries)) =
            &mut missing.effect_trace[0].outcome
        else {
            panic!("listing fixture must return DirectoryEntries")
        };
        entries.pop();
        assert!(matches!(
            compare_fs_read_directory_observations(
                &run.interpreter,
                &missing,
                b"listing",
                &expected,
            ),
            Err(FsDirectoryDifferentialError::Listing {
                lane: "native",
                ..
            })
        ));

        let mut wrong_name = run.native.clone();
        let CanonicalOutcomeV1::Success(CanonicalReplyV1::DirectoryEntries(entries)) =
            &mut wrong_name.effect_trace[0].outcome
        else {
            panic!("listing fixture must return DirectoryEntries")
        };
        entries[0].name.push(0xff);
        assert!(matches!(
            compare_fs_read_directory_observations(
                &run.interpreter,
                &wrong_name,
                b"listing",
                &expected,
            ),
            Err(FsDirectoryDifferentialError::Listing {
                lane: "native",
                ..
            })
        ));

        let mut wrong_kind = run.native.clone();
        let CanonicalOutcomeV1::Success(CanonicalReplyV1::DirectoryEntries(entries)) =
            &mut wrong_kind.effect_trace[0].outcome
        else {
            panic!("listing fixture must return DirectoryEntries")
        };
        entries[0].kind = ken_host::FsNodeKindV1::Other;
        assert!(matches!(
            compare_fs_read_directory_observations(
                &run.interpreter,
                &wrong_kind,
                b"listing",
                &expected,
            ),
            Err(FsDirectoryDifferentialError::Listing {
                lane: "native",
                ..
            })
        ));
    }

    /// Promise class: durable state-transition and failure-classification
    /// invariant. The recursive flag's inertness is explicitly part of the
    /// promoted-as-is contract, not a parent-chain capability claim.
    #[test]
    fn fs_create_directory_real_artifact_matches_the_landed_inert_flag_contract() {
        let success = fs_create_directory_scenario(
            "abi-a3-create-success",
            false,
            b"created",
            abi_a3_base_nodes(),
        );
        let run = run_scenario(&success).expect("create success executes");
        run.compare_fs_create_directory(
            b"created",
            b"data/created",
            false,
            None,
        )
        .expect("exact create transition");
        assert_eq!(
            confirm_native_tested_transition(
                HostOpV1::FsCreateDirectory,
                NativeTestedEvidence::from_fs_create_directory_run(
                    &run,
                    b"created",
                    b"data/created",
                    false,
                ),
            ),
            Ok(HostOpAvailabilityV1::NativeTested)
        );
        let mut wrong_native = run.native.clone();
        wrong_native.filesystem_delta.clear();
        let mut wrong_actions = run.native_actions.clone();
        wrong_actions.root_after = wrong_actions.root_before.clone();
        assert!(matches!(
            compare_fs_mutation_observations(
                &run.interpreter,
                &run.interpreter_actions,
                &wrong_native,
                &wrong_actions,
                CanonicalRequestV1::FsCreateDirectory {
                    recursive: false,
                    path: b"created".to_vec(),
                },
                b"data/created",
                None,
                MutationStateContract::CreateOne,
            ),
            Err(FsDirectoryDifferentialError::Transition {
                operation: HostOpV1::FsCreateDirectory,
                lane: "native",
                ..
            })
        ));

        let mut existing_nodes = abi_a3_base_nodes();
        existing_nodes.push(SeedNode {
            relative_path: b"data/existing".to_vec(),
            kind: crate::SeedNodeKind::Directory,
        });
        let existing = run_scenario(&fs_create_directory_scenario(
            "abi-a3-create-existing",
            false,
            b"existing",
            existing_nodes,
        ))
        .expect("create-existing executes");
        existing
            .compare_fs_create_directory(
                b"existing",
                b"data/existing",
                false,
                Some(IoErrorIdentityV1::AlreadyExists),
            )
            .expect("create-existing classification/state");

        for recursive in [false, true] {
            let missing = run_scenario(&fs_create_directory_scenario(
                if recursive {
                    "abi-a3-create-missing-true"
                } else {
                    "abi-a3-create-missing-false"
                },
                recursive,
                b"missing/child",
                abi_a3_base_nodes(),
            ))
            .expect("missing-parent create executes");
            missing
                .compare_fs_create_directory(
                    b"missing/child",
                    b"data/missing/child",
                    recursive,
                    Some(IoErrorIdentityV1::NotFound),
                )
                .expect("both recursive values preserve the landed NotFound/no-op behavior");
        }
    }

    /// Promise class: durable state-transition and exact failure-classification
    /// invariant for unlinking a non-directory node.
    #[test]
    fn fs_remove_file_real_artifact_distinguishes_success_wrong_kind_and_missing() {
        let mut success_nodes = abi_a3_base_nodes();
        success_nodes.push(SeedNode {
            relative_path: b"data/remove.bin".to_vec(),
            kind: crate::SeedNodeKind::File(b"payload".to_vec()),
        });
        let run = run_scenario(&fs_remove_file_scenario(
            "abi-a3-remove-file-success",
            b"remove.bin",
            success_nodes,
        ))
        .expect("remove-file success executes");
        run.compare_fs_remove_file(
            b"remove.bin",
            b"data/remove.bin",
            None,
        )
        .expect("exact remove-file transition");
        assert_eq!(
            confirm_native_tested_transition(
                HostOpV1::FsRemoveFile,
                NativeTestedEvidence::from_fs_remove_file_run(
                    &run,
                    b"remove.bin",
                    b"data/remove.bin",
                ),
            ),
            Ok(HostOpAvailabilityV1::NativeTested)
        );
        let mut wrong_native = run.native.clone();
        wrong_native.filesystem_delta.clear();
        let mut wrong_actions = run.native_actions.clone();
        wrong_actions.root_after = wrong_actions.root_before.clone();
        assert!(matches!(
            compare_fs_mutation_observations(
                &run.interpreter,
                &run.interpreter_actions,
                &wrong_native,
                &wrong_actions,
                CanonicalRequestV1::FsRemoveFile {
                    path: b"remove.bin".to_vec(),
                },
                b"data/remove.bin",
                None,
                MutationStateContract::RemoveSubtree,
            ),
            Err(FsDirectoryDifferentialError::Transition {
                operation: HostOpV1::FsRemoveFile,
                lane: "native",
                ..
            })
        ));

        let mut directory_nodes = abi_a3_base_nodes();
        directory_nodes.push(SeedNode {
            relative_path: b"data/not-a-file".to_vec(),
            kind: crate::SeedNodeKind::Directory,
        });
        let wrong_kind = run_scenario(&fs_remove_file_scenario(
            "abi-a3-remove-file-wrong-kind",
            b"not-a-file",
            directory_nodes,
        ))
        .expect("remove-file wrong-kind executes");
        wrong_kind
            .compare_fs_remove_file(
                b"not-a-file",
                b"data/not-a-file",
                Some(IoErrorIdentityV1::IsDirectory),
            )
            .expect("wrong-kind classification/state");

        let missing = run_scenario(&fs_remove_file_scenario(
            "abi-a3-remove-file-missing",
            b"missing",
            abi_a3_base_nodes(),
        ))
        .expect("remove-file missing executes");
        missing
            .compare_fs_remove_file(
                b"missing",
                b"data/missing",
                Some(IoErrorIdentityV1::NotFound),
            )
            .expect("missing classification/state");
    }

    /// Promise class: durable error-class-keyed differential. Deterministic
    /// classes assert exact state; the ruled mid-traversal class deliberately
    /// compares classification only.
    #[test]
    fn fs_remove_directory_real_artifact_honors_the_nontransactional_carve_out() {
        let mut tree_nodes = abi_a3_base_nodes();
        tree_nodes.extend([
            SeedNode {
                relative_path: b"data/tree".to_vec(),
                kind: crate::SeedNodeKind::Directory,
            },
            SeedNode {
                relative_path: b"data/tree/child".to_vec(),
                kind: crate::SeedNodeKind::Directory,
            },
            SeedNode {
                relative_path: b"data/tree/file".to_vec(),
                kind: crate::SeedNodeKind::File(b"payload".to_vec()),
            },
        ]);
        let run = run_scenario(&fs_remove_directory_scenario(
            "abi-a3-rmdir-recursive-success",
            true,
            b"tree",
            tree_nodes.clone(),
        ))
        .expect("recursive rmdir success executes");
        run.compare_fs_remove_directory(
            b"tree",
            b"data/tree",
            true,
            None,
        )
        .expect("exact recursive success transition");
        assert_eq!(
            confirm_native_tested_transition(
                HostOpV1::FsRemoveDirectory,
                NativeTestedEvidence::from_fs_remove_directory_run(
                    &run,
                    b"tree",
                    b"data/tree",
                    true,
                ),
            ),
            Ok(HostOpAvailabilityV1::NativeTested)
        );
        let mut wrong_native = run.native.clone();
        wrong_native.filesystem_delta.clear();
        let mut wrong_actions = run.native_actions.clone();
        wrong_actions.root_after = wrong_actions.root_before.clone();
        assert!(matches!(
            compare_fs_mutation_observations(
                &run.interpreter,
                &run.interpreter_actions,
                &wrong_native,
                &wrong_actions,
                CanonicalRequestV1::FsRemoveDirectory {
                    recursive: true,
                    path: b"tree".to_vec(),
                },
                b"data/tree",
                None,
                MutationStateContract::RemoveSubtree,
            ),
            Err(FsDirectoryDifferentialError::Transition {
                operation: HostOpV1::FsRemoveDirectory,
                lane: "native",
                ..
            })
        ));

        let mut empty_nodes = abi_a3_base_nodes();
        empty_nodes.push(SeedNode {
            relative_path: b"data/empty".to_vec(),
            kind: crate::SeedNodeKind::Directory,
        });
        let empty = run_scenario(&fs_remove_directory_scenario(
            "abi-a3-rmdir-empty",
            false,
            b"empty",
            empty_nodes,
        ))
        .expect("non-recursive empty rmdir executes");
        empty
            .compare_fs_remove_directory(
                b"empty",
                b"data/empty",
                false,
                None,
            )
            .expect("empty directory is removed exactly");

        let nonempty = run_scenario(&fs_remove_directory_scenario(
            "abi-a3-rmdir-nonempty",
            false,
            b"tree",
            tree_nodes,
        ))
        .expect("non-recursive non-empty rmdir executes");
        nonempty
            .compare_fs_remove_directory(
                b"tree",
                b"data/tree",
                false,
                Some(IoErrorIdentityV1::NotEmpty),
            )
            .expect("NotEmpty preserves the exact tree");

        let mut partial_nodes = abi_a3_base_nodes();
        partial_nodes.extend([
            SeedNode {
                relative_path: b"data/partial".to_vec(),
                kind: crate::SeedNodeKind::Directory,
            },
            SeedNode {
                relative_path: b"data/partial/locked".to_vec(),
                kind: crate::SeedNodeKind::DirectoryWithMode(0o500),
            },
            SeedNode {
                relative_path: b"data/partial/locked/held".to_vec(),
                kind: crate::SeedNodeKind::File(b"held".to_vec()),
            },
            SeedNode {
                relative_path: b"data/partial/removable".to_vec(),
                kind: crate::SeedNodeKind::File(b"remove".to_vec()),
            },
        ]);
        let partial = execute_scenario(&fs_remove_directory_scenario(
            "abi-a3-rmdir-mid-traversal",
            true,
            b"partial",
            partial_nodes,
        ))
        .expect("mid-traversal rmdir executes both lanes");
        assert_eq!(
            partial.interpreter_actions.fs_actions_after_resolve,
            Some(1),
            "the mid-traversal fixture must reach the backend after resolution"
        );
        partial
            .compare_fs_remove_directory_mid_traversal_error(
                b"partial",
                IoErrorIdentityV1::PermissionDenied,
            )
            .expect("classification agrees while residuals remain unconstrained");
        assert!(matches!(
            partial.compare_fs_remove_directory_mid_traversal_error(
                b"partial",
                IoErrorIdentityV1::NotEmpty,
            ),
            Err(FsDirectoryDifferentialError::Transition {
                lane: "contract",
                ..
            })
        ));

        let mut different_residual = partial.native.clone();
        different_residual.filesystem_delta.clear();
        let mut different_actions = partial.native_actions.clone();
        different_actions.root_after = different_actions.root_before.clone();
        compare_fs_mutation_observations(
            &partial.interpreter,
            &partial.interpreter_actions,
            &different_residual,
            &different_actions,
            CanonicalRequestV1::FsRemoveDirectory {
                recursive: true,
                path: b"partial".to_vec(),
            },
            b"partial",
            Some(IoErrorIdentityV1::PermissionDenied),
            MutationStateContract::MidTraversalUnconstrained,
        )
        .expect("a different residual remains lawful in the mid-traversal class");

        let mut wrong_class = partial.native.clone();
        let CanonicalOutcomeV1::Error(SemanticErrorV1::File(error)) =
            &mut wrong_class.effect_trace[0].outcome
        else {
            panic!("mid-traversal fixture must return FileError")
        };
        error.cause = FileErrorCauseV1::Io(IoErrorIdentityV1::NotEmpty);
        assert!(matches!(
            compare_fs_mutation_observations(
                &partial.interpreter,
                &partial.interpreter_actions,
                &wrong_class,
                &partial.native_actions,
                CanonicalRequestV1::FsRemoveDirectory {
                    recursive: true,
                    path: b"partial".to_vec(),
                },
                b"partial",
                Some(IoErrorIdentityV1::PermissionDenied),
                MutationStateContract::MidTraversalUnconstrained,
            ),
            Err(FsDirectoryDifferentialError::Shape {
                operation: HostOpV1::FsRemoveDirectory,
                lane: "native",
                ..
            })
        ));
    }

    fn assert_abi_a3_path_policy(operation: HostOpV1) {
        for policy in [
            AbiA3PolicyCase::Escape,
            AbiA3PolicyCase::Symlink,
            AbiA3PolicyCase::MissingRight,
        ] {
            let (scenario, expected) =
                abi_a3_policy_scenario(operation, policy);
            let request_path = scenario.process_input.arguments[0].clone();
            let run = run_scenario(&scenario).unwrap_or_else(|error| {
                panic!("{}: {error}", scenario.entry.identity)
            });
            assert_eq!(
                run.interpreter_actions.fs_actions_after_resolve,
                Some(0),
                "{} reached a post-resolution action",
                scenario.entry.identity
            );
            assert_eq!(
                run.interpreter_actions.root_before,
                run.interpreter_actions.root_after,
                "{} changed the interpreter root",
                scenario.entry.identity
            );
            assert_eq!(
                run.native_actions.root_before,
                run.native_actions.root_after,
                "{} changed the native root",
                scenario.entry.identity
            );
            for observation in [&run.interpreter, &run.native] {
                assert_eq!(observation.exit_status, 44);
                assert!(observation.filesystem_delta.is_empty());
                let [event] = observation.effect_trace.as_slice() else {
                    panic!("{} must emit one refusal", scenario.entry.identity)
                };
                assert_eq!(event.operation, operation);
                assert!(matches!(
                    &event.outcome,
                    CanonicalOutcomeV1::Error(SemanticErrorV1::File(error))
                        if error.operation == operation
                            && error.relative_path == request_path
                            && error.cause
                                == FileErrorCauseV1::Capability(expected.clone())
                ));
            }
        }
    }

    /// Promise class: durable invariant over the landed scoped-root, NoFollow,
    /// and Enumerate-right gate.
    #[test]
    fn fs_read_directory_exercises_every_path_policy_refusal() {
        assert_abi_a3_path_policy(HostOpV1::FsReadDirectory);
    }

    /// Promise class: durable invariant over the landed scoped-root, NoFollow,
    /// and Create-right gate.
    #[test]
    fn fs_create_directory_exercises_every_path_policy_refusal() {
        assert_abi_a3_path_policy(HostOpV1::FsCreateDirectory);
    }

    /// Promise class: durable invariant over the landed scoped-root, NoFollow,
    /// and RemoveFile right gate.
    #[test]
    fn fs_remove_file_exercises_every_path_policy_refusal() {
        assert_abi_a3_path_policy(HostOpV1::FsRemoveFile);
    }

    /// Promise class: durable invariant over the landed scoped-root, NoFollow,
    /// and RemoveDirectory right gate.
    #[test]
    fn fs_remove_directory_exercises_every_path_policy_refusal() {
        assert_abi_a3_path_policy(HostOpV1::FsRemoveDirectory);
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: both real lanes append the same non-text bytes to the same
    /// pre-existing file; the comparator requires the exact request and Unit
    /// reply, exact before/after content, and appended count equal to request
    /// length before comparing every canonical observation field.
    /// CLAIMED: the promoted native append agrees exactly with the interpreter
    /// on its deterministic content/count contract.
    /// THE GAP: an exact correct-vs-correct run alone cannot show the comparator
    /// reads the delta, so the wrong-native after-content mutation must redden.
    #[test]
    fn fs_append_file_real_artifact_content_count_differential_discriminates() {
        let run = run_scenario(&fs_append_success_scenario())
            .expect("FsAppendFile real-artifact differential executes");
        run.compare_fs_append_file(
            b"file.bin",
            b"data/file.bin",
            b"before",
            &[0xff, b'x', 0xfe],
        )
        .expect("exact append content/count parity");
        assert_eq!(run.interpreter_actions.fs_actions_after_resolve, Some(1));
        assert_eq!(
            confirm_native_tested_transition(
                HostOpV1::FsAppendFile,
                NativeTestedEvidence::from_fs_append_file_run(
                    &run,
                    b"file.bin",
                    b"data/file.bin",
                    b"before",
                    &[0xff, b'x', 0xfe],
                ),
            ),
            Ok(HostOpAvailabilityV1::NativeTested)
        );

        let mut wrong_native = run.native.clone();
        let [ken_host::FsDeltaV1::Modified { after, .. }] =
            wrong_native.filesystem_delta.as_mut_slice()
        else {
            panic!("append fixture must modify exactly one file")
        };
        after
            .file_bytes
            .as_mut()
            .expect("modified file has bytes")
            .pop();
        assert!(matches!(
            compare_fs_append_file_observations(
                &run.interpreter,
                &wrong_native,
                b"file.bin",
                b"data/file.bin",
                b"before",
                &[0xff, b'x', 0xfe],
            ),
            Err(FsAppendFileDifferentialError::ContentCount {
                lane: "native",
                ..
            })
        ));
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: the same append surface succeeds in-root above, while escape,
    /// symlink-under-no-follow, and missing-write-right inputs return their
    /// exact policy identities on both real lanes without filesystem mutation.
    /// CLAIMED: native FsAppendFile exercises the landed scoped-root, rights,
    /// and no-follow policy rather than bypassing it.
    /// THE GAP: symmetric refusals could survive a shared policy bypass; the
    /// exact identities and before/after roots make such a bypass observable,
    /// and the production-call mutation demonstrates that they redden.
    #[test]
    fn fs_append_file_real_artifact_exercises_all_path_policy_refusals() {
        let partial_rights = RightSet::READ
            .union(RightSet::ENUMERATE)
            .union(RightSet::METADATA);
        for (scenario, expected) in [
            (
                fs_append_escape_scenario(),
                CapabilityDeniedV1::ScopeEscape,
            ),
            (
                fs_append_symlink_scenario(),
                CapabilityDeniedV1::SymlinkDenied,
            ),
            (
                fs_append_missing_right_scenario(),
                CapabilityDeniedV1::RightNotHeld {
                    operation: ken_host::FsCapabilityOperationV1::Append,
                    held_rights: partial_rights.bits(),
                },
            ),
        ] {
            let run = run_scenario(&scenario).unwrap_or_else(|error| {
                panic!("{}: {error}", scenario.entry.identity)
            });
            assert_eq!(run.interpreter.exit_status, 84);
            assert_eq!(run.native.exit_status, 84);
            assert!(run.interpreter.filesystem_delta.is_empty());
            assert!(run.native.filesystem_delta.is_empty());
            assert_eq!(run.interpreter_actions.fs_actions_after_resolve, Some(0));
            assert_eq!(
                run.interpreter_actions.root_before,
                run.interpreter_actions.root_after
            );
            assert_eq!(
                run.native_actions.root_before,
                run.native_actions.root_after
            );
            for observation in [&run.interpreter, &run.native] {
                let [event] = observation.effect_trace.as_slice() else {
                    panic!(
                        "{} must emit exactly one refusal",
                        scenario.entry.identity
                    )
                };
                assert_eq!(event.operation, HostOpV1::FsAppendFile);
                assert!(matches!(
                    &event.outcome,
                    CanonicalOutcomeV1::Error(SemanticErrorV1::File(error))
                        if error.operation == HostOpV1::FsAppendFile
                            && error.cause
                                == FileErrorCauseV1::Capability(expected.clone())
                ));
            }
        }
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: each real lane reports both size and kind for one known
    /// regular file and one known directory, with each response independently
    /// bound to that lane's OS artifact before full canonical comparison.
    /// CLAIMED: promoted native FsMetadata agrees exactly with the interpreter
    /// on the complete landed `{size, kind}` contract for both node classes.
    /// THE GAP: correct-vs-correct equality alone cannot show either field is
    /// read; independent wrong-native size and kind mutations must each redden.
    #[test]
    fn fs_metadata_real_artifact_fields_are_exact_and_discriminating() {
        const PATHS: &[(&[u8], &[u8])] = &[
            (b"file.bin", b"data/file.bin"),
            (b"known-dir", b"data/known-dir"),
        ];
        let run = run_scenario(&fs_metadata_success_scenario())
            .expect("FsMetadata real-artifact differential executes");
        run.compare_fs_metadata(PATHS)
            .expect("exact file and directory metadata parity");
        assert_eq!(run.interpreter.exit_status, 0);
        assert_eq!(run.native.exit_status, 0);
        assert_eq!(run.interpreter_actions.fs_actions_after_resolve, Some(2));
        assert_eq!(
            confirm_native_tested_transition(
                HostOpV1::FsMetadata,
                NativeTestedEvidence::from_fs_metadata_run(&run, PATHS),
            ),
            Ok(HostOpAvailabilityV1::NativeTested)
        );

        let interpreter_expected = expected_fs_metadata(
            "interpreter",
            run.roots.interpreter(),
            PATHS,
        )
        .expect("interpreter fixtures have metadata");
        let native_expected =
            expected_fs_metadata("native", run.roots.native(), PATHS)
                .expect("native fixtures have metadata");

        let mut wrong_size = run.native.clone();
        let CanonicalOutcomeV1::Success(CanonicalReplyV1::FileMetadata(metadata)) =
            &mut wrong_size.effect_trace[0].outcome
        else {
            panic!("the first metadata response must describe the file")
        };
        metadata.size = metadata.size.saturating_add(1);
        assert!(matches!(
            compare_fs_metadata_observations(
                &run.interpreter,
                &wrong_size,
                PATHS,
                &interpreter_expected,
                &native_expected,
            ),
            Err(FsMetadataDifferentialError::Field {
                lane: "native",
                event: 0,
                field: FsMetadataField::Size,
                ..
            })
        ));

        let mut wrong_kind = run.native.clone();
        let CanonicalOutcomeV1::Success(CanonicalReplyV1::FileMetadata(metadata)) =
            &mut wrong_kind.effect_trace[1].outcome
        else {
            panic!("the second metadata response must describe the directory")
        };
        metadata.kind = ken_host::FsNodeKindV1::File;
        assert!(matches!(
            compare_fs_metadata_observations(
                &run.interpreter,
                &wrong_kind,
                PATHS,
                &interpreter_expected,
                &native_expected,
            ),
            Err(FsMetadataDifferentialError::Field {
                lane: "native",
                event: 1,
                field: FsMetadataField::Kind,
                ..
            })
        ));
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: the successful in-root scenario above performs two metadata
    /// leaves, while escape, leaf-symlink-under-NoFollow, and missing-Metadata-
    /// right inputs return their exact policy identities on both real lanes,
    /// with zero post-resolution actions and byte-identical roots.
    /// CLAIMED: native FsMetadata exercises the landed scoped-root, rights, and
    /// no-follow policy rather than bypassing it.
    /// THE GAP: symmetric refusals could survive a shared bypass; exact expected
    /// outcomes plus production-site policy mutations make that bypass visible.
    #[test]
    fn fs_metadata_real_artifact_exercises_the_honest_policy_contract() {
        let held = RightSet::NONE;
        for (scenario, expected) in [
            (
                fs_metadata_escape_scenario(),
                CapabilityDeniedV1::ScopeEscape,
            ),
            (
                fs_metadata_symlink_scenario(),
                CapabilityDeniedV1::SymlinkDenied,
            ),
            (
                fs_metadata_missing_right_scenario(),
                CapabilityDeniedV1::RightNotHeld {
                    operation: ken_host::FsCapabilityOperationV1::Metadata,
                    held_rights: held.bits(),
                },
            ),
        ] {
            let run = run_scenario(&scenario).unwrap_or_else(|error| {
                panic!("{}: {error}", scenario.entry.identity)
            });
            assert_eq!(run.interpreter.exit_status, 94);
            assert_eq!(run.native.exit_status, 94);
            assert!(run.interpreter.filesystem_delta.is_empty());
            assert!(run.native.filesystem_delta.is_empty());
            assert_eq!(run.interpreter_actions.fs_actions_after_resolve, Some(0));
            assert_eq!(
                run.interpreter_actions.root_before,
                run.interpreter_actions.root_after
            );
            assert_eq!(
                run.native_actions.root_before,
                run.native_actions.root_after
            );
            for observation in [&run.interpreter, &run.native] {
                let [event] = observation.effect_trace.as_slice() else {
                    panic!("{} must emit one refusal", scenario.entry.identity)
                };
                assert_eq!(event.operation, HostOpV1::FsMetadata);
                assert!(matches!(
                    &event.outcome,
                    CanonicalOutcomeV1::Error(SemanticErrorV1::File(error))
                        if error.operation == HostOpV1::FsMetadata
                            && error.cause
                                == FileErrorCauseV1::Capability(expected.clone())
                ));
            }
        }
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: both real lanes start with `data/a.bin` only, issue the exact
    /// FsRename request with a Unit success, and finish with `a.bin` absent and
    /// `b.bin` present as the identical original file node.
    /// CLAIMED: native FsRename agrees with the interpreter on the complete
    /// landed state transition and result classification.
    /// THE GAP: correct-vs-correct equality cannot show the transition is read;
    /// the wrong-native control restores the complete before state and must
    /// redden this named transition comparator.
    #[test]
    fn fs_rename_real_artifact_state_transition_discriminates() {
        let run = run_scenario(&fs_rename_success_scenario())
            .expect("FsRename real-artifact differential executes");
        run.compare_fs_rename(
            b"a.bin",
            b"b.bin",
            b"data/a.bin",
            b"data/b.bin",
            b"original",
        )
        .expect("exact rename state transition parity");
        assert_eq!(run.interpreter_actions.fs_actions_after_resolve, Some(2));
        assert_eq!(
            confirm_native_tested_transition(
                HostOpV1::FsRename,
                NativeTestedEvidence::from_fs_rename_run(
                    &run,
                    b"a.bin",
                    b"b.bin",
                    b"data/a.bin",
                    b"data/b.bin",
                    b"original",
                ),
            ),
            Ok(HostOpAvailabilityV1::NativeTested)
        );

        let mut wrong_native = run.native.clone();
        wrong_native.filesystem_delta.clear();
        let mut wrong_native_actions = run.native_actions.clone();
        wrong_native_actions.root_after = wrong_native_actions.root_before.clone();
        assert!(
            wrong_native_actions
                .root_after
                .nodes
                .iter()
                .any(|node| node.relative_path.as_slice() == b"data/a.bin"),
            "the wrong-native control must leave the source present"
        );
        assert!(matches!(
            compare_fs_rename_observations(
                &run.interpreter,
                &run.interpreter_actions,
                &wrong_native,
                &wrong_native_actions,
                b"a.bin",
                b"b.bin",
                b"data/a.bin",
                b"data/b.bin",
                b"original",
            ),
            Err(FsRenameDifferentialError::Transition {
                lane: "native",
                ..
            })
        ));
    }

    /// Promise class: durable invariant.
    ///
    /// MEASURED: source and destination escape and leaf-symlink inputs plus a
    /// missing Rename right yield their exact policy identities on both real
    /// lanes, with the expected interpreter resolution count and no state
    /// change.
    /// CLAIMED: native FsRename resolves both operands through the landed
    /// scoped-root, rights, and NoFollow policy instead of bypassing it.
    /// THE GAP: symmetric refusals could survive a shared policy bypass; exact
    /// outcomes, before/after roots, and production-site policy mutations make
    /// that bypass observable.
    #[test]
    fn fs_rename_real_artifact_exercises_both_path_policy_operands() {
        for (scenario, expected, resolved) in [
            (
                fs_rename_source_escape_scenario(),
                CapabilityDeniedV1::ScopeEscape,
                0,
            ),
            (
                fs_rename_destination_escape_scenario(),
                CapabilityDeniedV1::ScopeEscape,
                1,
            ),
            (
                fs_rename_source_symlink_scenario(),
                CapabilityDeniedV1::SymlinkDenied,
                0,
            ),
            (
                fs_rename_destination_symlink_scenario(),
                CapabilityDeniedV1::SymlinkDenied,
                1,
            ),
            (
                fs_rename_missing_right_scenario(),
                CapabilityDeniedV1::RightNotHeld {
                    operation:
                        ken_host::FsCapabilityOperationV1::RenameSource,
                    held_rights: RightSet::NONE.bits(),
                },
                0,
            ),
        ] {
            let request_source = scenario.process_input.arguments[0].clone();
            let run = run_scenario(&scenario).unwrap_or_else(|error| {
                panic!("{}: {error}", scenario.entry.identity)
            });
            assert_eq!(run.interpreter.exit_status, 104);
            assert_eq!(run.native.exit_status, 104);
            assert!(run.interpreter.filesystem_delta.is_empty());
            assert!(run.native.filesystem_delta.is_empty());
            assert_eq!(
                run.interpreter_actions.fs_actions_after_resolve,
                Some(resolved)
            );
            assert_eq!(
                run.interpreter_actions.root_before,
                run.interpreter_actions.root_after
            );
            assert_eq!(
                run.native_actions.root_before,
                run.native_actions.root_after
            );
            for observation in [&run.interpreter, &run.native] {
                let [event] = observation.effect_trace.as_slice() else {
                    panic!("{} must emit one refusal", scenario.entry.identity)
                };
                assert_eq!(event.operation, HostOpV1::FsRename);
                assert!(matches!(
                    &event.outcome,
                    CanonicalOutcomeV1::Error(SemanticErrorV1::File(error))
                        if error.operation == HostOpV1::FsRename
                            && error.relative_path == request_source
                            && error.cause
                                == FileErrorCauseV1::Capability(expected.clone())
                ));
            }
        }
    }

    /// Promise class: durable-invariant companion control. ABI-A1 D4 is
    /// intentionally ignored: exact instant
    /// equality is the wrong live gate, but running it manually demonstrates
    /// that two correct real lanes differ before normalization.
    #[test]
    #[ignore = "ABI-A1 D4: demonstrates why exact instant equality is wrong"]
    fn clock_wall_now_naive_exact_equality_is_wrong_on_correct_real_clocks() {
        let run = execute_scenario(&clock_wall_scenario())
            .expect("ClockWallNow real-artifact differential executes");
        run.compare_clock_wall_now()
            .expect("both observations are individually lawful");
        assert!(
            run.compare_exact().is_err(),
            "the raw real-clock observations unexpectedly compared exactly"
        );
    }

    // Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
    //
    // Observed signature, exactly:
    //   Effect: seat Argument(0) of FsWriteFile needs BytesPointerLength, which it cannot observe in CarriedWord
    //
    // Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
    // Pre-existing base debt, NOT a bind-order regression: measured failing at
    // the frozen base 21fd46dc by the D14 differential, before any
    // RT-SRCBODY-BIND-ORDER commit.
    // It refuses at object emission, so the program never executes and no
    // binding order is observable in it.
    // Annotation only -- test body and expectations are unchanged.
    #[test]
    // RT-SITEOP-CARRIED-WITNESS D1a/D2: FsWriteFile Argument(0) was site-bound:
    // FileError SiteOperand(0) could not project its carried word. D5 byte-span
    // observation was not the blocker; D2 supplies the exact emitted-helper port.
    fn real_artifact_five_op_observation_matches_interp_on_twin_roots() {
        let run = run_scenario(&five_op_scenario()).expect("real five-op differential");
        run.compare_exact().expect("six-field equality");
        assert!(run.exact_artifact_executed);
        assert_eq!(run.interpreter.stdout, vec![b'r', 0xff, b'x']);
        assert_eq!(run.interpreter.filesystem_delta.len(), 1);
        assert_eq!(
            run.interpreter_actions.root_after,
            run.native_actions.root_after
        );
        assert_eq!(
            run.interpreter
                .effect_trace
                .iter()
                .map(|event| event.operation)
                .collect::<Vec<_>>(),
            vec![
                HostOpV1::FsWriteFile,
                HostOpV1::FsReadFile,
                HostOpV1::ConsoleWrite,
                HostOpV1::ConsoleFlush,
                HostOpV1::ConsoleIsTerminal,
            ]
        );

        for operation in PX5_PLANNED_NATIVE_TARGETS {
            let evidence = NativeTestedEvidence::from_run(operation, &run);
            assert!(evidence.permits_confirmation());
            assert_eq!(
                confirm_native_tested_transition(operation, evidence),
                Ok(HostOpAvailabilityV1::NativeTested)
            );
        }
        assert!(run.rejects_wrong_plan_binding());
    }

    // Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
    //
    // Observed signature, exactly:
    //   Effect: seat Argument(0) of FsChangeMode needs BytesPointerLength, which it cannot observe in CarriedWord
    //
    // Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
    // Pre-existing base debt, NOT a bind-order regression: measured failing at
    // the frozen base 21fd46dc by the D14 differential, before any
    // RT-SRCBODY-BIND-ORDER commit.
    // It refuses at object emission, so the program never executes and no
    // binding order is observable in it.
    // Annotation only -- test body and expectations are unchanged.
    #[test]
    // RT-SITEOP-CARRIED-WITNESS D1a/D2: FsChangeMode Argument(0) was site-bound:
    // FileError SiteOperand(0) could not project its carried word. D5 byte-span
    // observation was not the blocker; D2 supplies the exact emitted-helper port.
    fn change_mode_is_observed_and_matches_across_real_twin_roots() {
        let run = run_scenario(&change_mode_scenario()).expect("real change-mode differential");
        run.compare_exact().expect("mode-aware equality");
        assert!(run.exact_artifact_executed);
        assert_eq!(run.interpreter.exit_status, 0);
        assert_eq!(
            run.interpreter
                .effect_trace
                .iter()
                .map(|event| event.operation)
                .collect::<Vec<_>>(),
            vec![HostOpV1::FsChangeMode]
        );
        assert!(matches!(
            run.interpreter.filesystem_delta.as_slice(),
            [ken_host::FsDeltaV1::Modified {
                relative_path,
                before,
                after,
            }] if relative_path == b"mode.bin"
                && before.file_bytes == after.file_bytes
                && before.mode != after.mode
                && after.mode == Some(0o640)
        ));
        assert_eq!(
            run.interpreter_actions.root_after,
            run.native_actions.root_after
        );
        let mut wrong_mode = run.native.clone();
        let [ken_host::FsDeltaV1::Modified { after, .. }] =
            wrong_mode.filesystem_delta.as_mut_slice()
        else {
            panic!("change-mode scenario must have one modified node")
        };
        after.mode = Some(0o600);
        assert!(compare_canonical_exact(&run.interpreter, &wrong_mode).is_err());
        let evidence = NativeTestedEvidence::from_run(HostOpV1::FsChangeMode, &run);
        assert!(evidence.permits_confirmation());
        assert_eq!(
            confirm_native_tested_transition(HostOpV1::FsChangeMode, evidence),
            Ok(HostOpAvailabilityV1::NativeTested)
        );
    }

    // Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
    //
    // Observed signature, exactly:
    //   Effect: seat Argument(0) of FsChangeMode needs BytesPointerLength, which it cannot observe in CarriedWord
    //
    // Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
    // Pre-existing base debt, NOT a bind-order regression: measured failing at
    // the frozen base 21fd46dc by the D14 differential, before any
    // RT-SRCBODY-BIND-ORDER commit.
    // It refuses at object emission, so the program never executes and no
    // binding order is observable in it.
    // Annotation only -- test body and expectations are unchanged.
    #[test]
    // RT-SITEOP-CARRIED-WITNESS D1a/D2: FsChangeMode Argument(0) was site-bound:
    // FileError SiteOperand(0) could not project its carried word. D5 byte-span
    // observation was not the blocker; D2 supplies the exact emitted-helper port.
    fn directory_change_mode_matches_across_real_twin_roots() {
        let run = run_scenario(&change_directory_mode_scenario())
            .expect("real directory change-mode differential");
        run.compare_exact()
            .expect("directory trace, mode delta, and exit equality");
        assert!(run.exact_artifact_executed);
        assert_eq!(run.interpreter.exit_status, 0);
        assert_eq!(run.native.exit_status, 0);
        assert_eq!(run.interpreter.effect_trace, run.native.effect_trace);
        assert_eq!(
            run.interpreter.filesystem_delta,
            run.native.filesystem_delta
        );
        assert_eq!(
            run.interpreter
                .effect_trace
                .iter()
                .map(|event| event.operation)
                .collect::<Vec<_>>(),
            vec![HostOpV1::FsChangeMode]
        );
        assert!(matches!(
            run.interpreter.filesystem_delta.as_slice(),
            [ken_host::FsDeltaV1::Modified {
                relative_path,
                before,
                after,
            }] if relative_path == b"mode-dir"
                && before.kind == ken_host::FsNodeKindV1::Directory
                && before.file_bytes == after.file_bytes
                && before.mode != after.mode
                && after.mode == Some(0o640)
        ));
    }

    // Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
    //
    // Observed signature, exactly:
    //   Effect: seat Argument(0) of FsChangeMode needs BytesPointerLength, which it cannot observe in CarriedWord
    //
    // Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
    // Pre-existing base debt, NOT a bind-order regression: measured failing at
    // the frozen base 21fd46dc by the D14 differential, before any
    // RT-SRCBODY-BIND-ORDER commit.
    // It refuses at object emission, so the program never executes and no
    // binding order is observable in it.
    // Annotation only -- test body and expectations are unchanged.
    #[test]
    // RT-SITEOP-CARRIED-WITNESS D1a/D2: FsChangeMode Argument(0) was site-bound:
    // FileError SiteOperand(0) could not project its carried word. D5 byte-span
    // observation was not the blocker; D2 supplies the exact emitted-helper port.
    fn invalid_change_mode_is_a_typed_pre_dispatch_result_in_both_lanes() {
        let run =
            run_scenario(&invalid_change_mode_scenario()).expect("real invalid-mode differential");
        run.compare_exact().expect("typed invalid-input equality");
        assert_eq!(run.interpreter.exit_status, 72);
        assert!(run.interpreter.effect_trace.is_empty());
        assert!(run.interpreter.filesystem_delta.is_empty());
        assert_eq!(run.interpreter_actions.fs_actions_after_resolve, Some(0));
        assert_eq!(
            run.interpreter_actions.root_before,
            run.interpreter_actions.root_after
        );
        assert_eq!(
            run.native_actions.root_before,
            run.native_actions.root_after
        );
    }

    // Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
    //
    // Observed signature, exactly:
    //   Effect: seat Argument(0) of FsReadFile needs BytesPointerLength, which it cannot observe in CarriedWord
    //
    // Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
    // Pre-existing base debt, NOT a bind-order regression: measured failing at
    // the frozen base 21fd46dc by the D14 differential, before any
    // RT-SRCBODY-BIND-ORDER commit.
    // It refuses at object emission, so the program never executes and no
    // binding order is observable in it.
    // Annotation only -- test body and expectations are unchanged.
    #[test]
    // RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile Argument(0) was site-bound:
    // FileError SiteOperand(0) could not project its carried word. D5 byte-span
    // observation was not the blocker; D2 supplies the exact emitted-helper port.
    fn real_producers_preserve_raw_requests_across_descriptor_collision() {
        let run = run_scenario(&raw_descriptor_collision_scenario())
            .expect("real raw-descriptor collision differential");
        run.compare_exact().expect("six-field equality");
        assert!(run.exact_artifact_executed);
        assert_eq!(
            run.interpreter_actions.root_before,
            run.interpreter_actions.root_after
        );
        assert_eq!(
            run.native_actions.root_before,
            run.native_actions.root_after
        );

        let expected_identity = Some(program_caps_fs_trace_identity_v1());
        for observation in [&run.interpreter, &run.native] {
            let [raw, normalized] = observation.effect_trace.as_slice() else {
                panic!("collision program must emit exactly two FS events")
            };
            assert_eq!(raw.sequence, 0);
            assert_eq!(normalized.sequence, 1);
            assert_eq!(raw.capability, expected_identity);
            assert_eq!(normalized.capability, expected_identity);
            assert_eq!(
                raw.request,
                CanonicalRequestV1::FsReadFile {
                    path: b"dir/./x".to_vec(),
                }
            );
            assert_eq!(
                normalized.request,
                CanonicalRequestV1::FsReadFile {
                    path: b"dir/x".to_vec(),
                }
            );
            assert_eq!(
                raw.outcome,
                CanonicalOutcomeV1::Success(CanonicalReplyV1::Bytes(b"same-node".to_vec()))
            );
            assert_eq!(raw.outcome, normalized.outcome);
        }
    }

    // Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
    //
    // Observed signature, exactly:
    //   Effect: seat Argument(0) of FsReadFile needs BytesPointerLength, which it cannot observe in CarriedWord
    //
    // Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
    // Pre-existing base debt, NOT a bind-order regression: measured failing at
    // the frozen base 21fd46dc by the D14 differential, before any
    // RT-SRCBODY-BIND-ORDER commit.
    // It refuses at object emission, so the program never executes and no
    // binding order is observable in it.
    // Annotation only -- test body and expectations are unchanged.
    #[test]
    // RT-SITEOP-CARRIED-WITNESS D1a/D2: FsReadFile Argument(0) was site-bound:
    // FileError SiteOperand(0) could not project its carried word. D5 byte-span
    // observation was not the blocker; D2 supplies the exact emitted-helper port.
    fn execution_start_cwd_root_reaches_the_same_real_node_in_both_lanes() {
        let run = run_scenario(&execution_start_cwd_root_scenario())
            .expect("PX15 real cwd-root differential");
        run.compare_exact().expect("cwd-root canonical equality");
        assert!(run.exact_artifact_executed);
        assert_eq!(run.interpreter.exit_status, 0);
        assert!(run.interpreter.filesystem_delta.is_empty());
        assert_eq!(run.interpreter_actions.fs_actions_after_resolve, Some(2));
        for observation in [&run.interpreter, &run.native] {
            assert!(observation.effect_trace.iter().all(|event| matches!(
                &event.request,
                CanonicalRequestV1::FsReadFile { path } if path == b"x"
            )));
            assert!(observation.effect_trace.iter().all(|event| matches!(
                &event.outcome,
                CanonicalOutcomeV1::Success(CanonicalReplyV1::Bytes(bytes))
                    if bytes == b"cwd-root"
            )));
        }
    }

    #[cfg(unix)]
    #[test]
    fn cwd_and_absolute_root_spellings_emit_byte_identical_observations() {
        let root = std::env::temp_dir().join(format!(
            "ken-px15-spelling-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let data = root.join("data");
        std::fs::create_dir_all(&data).unwrap();
        std::fs::write(data.join("x"), b"same").unwrap();
        let base = "program capabilities FS AFull";
        let cwd_source = RAW_COLLISION_SOURCE.replacen(
            base,
            r#"program capabilities FS AFull "./data", RootExecution Allow"#,
            1,
        );
        let absolute_source = RAW_COLLISION_SOURCE.replacen(
            base,
            &format!(
                r#"program capabilities FS AFull "{}", RootExecution Allow"#,
                data.display()
            ),
            1,
        );
        let arguments = vec![b"px15".to_vec(), b"x".to_vec(), b"x".to_vec()];
        let cwd_bytes = root.as_os_str().as_bytes();
        let mut cwd_host = ken_interp::PosixHost::new_at(&root);
        let cwd_observation = ken_cli::run_program_effect_observation(
            &cwd_source,
            ken_cli::SourceFormat::Ken,
            &arguments,
            &[],
            cwd_bytes,
            &mut cwd_host,
        )
        .expect("cwd-root interpreter observation");
        let mut absolute_host = ken_interp::PosixHost::new_at(&root);
        let absolute_observation = ken_cli::run_program_effect_observation(
            &absolute_source,
            ken_cli::SourceFormat::Ken,
            &arguments,
            &[],
            cwd_bytes,
            &mut absolute_host,
        )
        .expect("absolute-root interpreter observation");
        assert_eq!(cwd_observation, absolute_observation);
        assert!(cwd_observation.effect_trace.iter().all(|event| matches!(
            &event.request,
            CanonicalRequestV1::FsReadFile { path } if path == b"x"
        )));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn wrong_token_malformed_identity_and_error_are_reply_owned() {
        let mut source_revocation = ken_host::RevocationDomain::default();
        let mut source_table = CapabilityTableV1::default();
        let wrong_token = source_table.insert(CapabilityGrantV1::mint_root(
            program_caps_fs_trace_identity_v1(),
            ken_elaborator::capabilities::Cap::mint(AUTH_FULL, "FS"),
            &mut source_revocation,
        ));
        let target_revocation = ken_host::RevocationDomain::default();
        let target_table = CapabilityTableV1::default();
        let request = CanonicalRequestV1::FsReadFile {
            path: b"raw/./identity".to_vec(),
        };
        let mut backend = NoLeafBackend::default();
        let mut resources = ken_host::ResourceTableV1::default();

        let reply = dispatch_host_op_v1(
            &mut backend,
            &target_table,
            &target_revocation,
            &mut resources,
            HostOpV1::FsReadFile,
            Some(wrong_token),
            ken_host::ResourceInputsV1::None,
            &request,
        )
        .expect("malformed token is a typed canonical reply");

        assert_eq!(backend.calls, 0, "denial must precede every host leaf");
        assert_eq!(reply.capability_identity, None);
        assert!(matches!(
            reply.outcome,
            CanonicalOutcomeV1::Error(SemanticErrorV1::File(ken_host::FileErrorIdentityV1 {
                operation: HostOpV1::FsReadFile,
                relative_path,
                cause: FileErrorCauseV1::Capability(CapabilityDeniedV1::MalformedCapability),
            })) if relative_path == b"raw/./identity"
        ));
    }

    // Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
    //
    // Observed signature, exactly:
    //   Effect: seat Argument(0) of FsWriteFile needs BytesPointerLength, which it cannot observe in CarriedWord
    //
    // Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
    // Pre-existing base debt, NOT a bind-order regression: measured failing at
    // the frozen base 21fd46dc by the D14 differential, before any
    // RT-SRCBODY-BIND-ORDER commit.
    // It refuses at object emission, so the program never executes and no
    // binding order is observable in it.
    // Annotation only -- test body and expectations are unchanged.
    #[test]
    // RT-SITEOP-CARRIED-WITNESS D1a/D2: FsWriteFile Argument(0) was site-bound:
    // FileError SiteOperand(0) could not project its carried word. D5 byte-span
    // observation was not the blocker; D2 supplies the exact emitted-helper port.
    fn real_captured_evidence_mutations_bite_while_return_proxy_stays_green() {
        let run = run_scenario(&five_op_scenario()).expect("real five-op differential");
        let proxy = RunnerOnlyProxy {
            scenario_identity: run.scenario_identity.clone(),
            returned_value: run.interpreter.exit_status,
        };
        for mutation in [
            CanonicalMutation::SilentSkip,
            CanonicalMutation::DuplicatedResume,
            CanonicalMutation::ReorderedEvents,
            CanonicalMutation::StdoutStderrSwap,
            CanonicalMutation::PathByteNormalization,
            CanonicalMutation::WrongCapabilityToken,
            CanonicalMutation::DeniedBeforeHostAction,
            CanonicalMutation::FilesystemMutationWithoutTrace,
            CanonicalMutation::TraceWithoutFilesystemMutation,
            CanonicalMutation::TargetEffectManifestMismatch,
            CanonicalMutation::OperationStatusTransition,
        ] {
            let mut changed = run.native.clone();
            apply_canonical_mutation(&mut changed, mutation)
                .unwrap_or_else(|error| panic!("real mutation {mutation:?}: {error:?}"));
            assert!(
                compare_canonical_exact(&run.interpreter, &changed).is_err(),
                "real captured mutation {mutation:?} must be rejected"
            );
            let unchanged_proxy = RunnerOnlyProxy {
                scenario_identity: run.scenario_identity.clone(),
                returned_value: run.interpreter.exit_status,
            };
            assert!(proxy.agrees(&unchanged_proxy));
        }
        let mut wrong_exit = run.native.clone();
        wrong_exit.exit_status ^= 1;
        assert!(compare_canonical_exact(&run.interpreter, &wrong_exit).is_err());
        let mut wrong_target = run.native.clone();
        wrong_target.terminal_error = Some(ken_host::TerminalErrorV1::TargetAbiMismatch);
        wrong_target.effect_trace.clear();
        wrong_target.filesystem_delta.clear();
        assert!(compare_canonical_exact(&run.interpreter, &wrong_target).is_err());
        assert!(proxy.agrees(&RunnerOnlyProxy {
            scenario_identity: run.scenario_identity.clone(),
            returned_value: run.interpreter.exit_status,
        }));
    }

    // Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
    //
    // Observed signature, exactly:
    //   Effect: seat Argument(0) of FsWriteFile needs BytesPointerLength, which it cannot observe in CarriedWord
    //
    // Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
    // Pre-existing base debt, NOT a bind-order regression: measured failing at
    // the frozen base 21fd46dc by the D14 differential, before any
    // RT-SRCBODY-BIND-ORDER commit.
    // It refuses at object emission, so the program never executes and no
    // binding order is observable in it.
    // Annotation only -- test body and expectations are unchanged.
    #[test]
    // RT-SITEOP-CARRIED-WITNESS D1a/D2: FsWriteFile Argument(0) was site-bound:
    // FileError SiteOperand(0) could not project its carried word. D5 byte-span
    // observation was not the blocker; D2 supplies the exact emitted-helper port.
    fn real_scope_denial_is_typed_and_precedes_any_host_action() {
        let run = run_scenario(&denial_scenario()).expect("real denial differential");
        run.compare_exact().expect("typed denial equality");
        assert_eq!(run.interpreter.exit_status, 44);
        assert!(run.interpreter.filesystem_delta.is_empty());
        assert!(denial_precedes_host_action(
            &run.interpreter_actions,
            &run.interpreter
        ));
        assert_eq!(
            run.interpreter_actions.root_before,
            run.interpreter_actions.root_after
        );
        assert_eq!(
            run.native_actions.root_before,
            run.native_actions.root_after
        );
        assert!(matches!(
            &run.interpreter.effect_trace[0].outcome,
            ken_host::CanonicalOutcomeV1::Error(SemanticErrorV1::File(error))
                if error.cause == FileErrorCauseV1::Capability(CapabilityDeniedV1::ScopeEscape)
        ));

        let proxy = RunnerOnlyProxy {
            scenario_identity: run.scenario_identity.clone(),
            returned_value: run.interpreter.exit_status,
        };
        let mut weakened = run.native.clone();
        apply_canonical_mutation(&mut weakened, CanonicalMutation::WeakenedErrorIdentity)
            .expect("real denial has an error identity");
        assert!(compare_canonical_exact(&run.interpreter, &weakened).is_err());
        assert!(proxy.agrees(&RunnerOnlyProxy {
            scenario_identity: run.scenario_identity,
            returned_value: run.interpreter.exit_status,
        }));
    }

    // Ignored pending RT-CARRIER-BYTESPAN-OBSERVE.
    //
    // Observed signature, exactly:
    //   Effect: seat Argument(0) of FsWriteFile needs BytesPointerLength, which it cannot observe in CarriedWord
    //
    // Owner node: RT-CARRIER-BYTESPAN-OBSERVE.
    // Pre-existing base debt, NOT a bind-order regression: measured failing at
    // the frozen base 21fd46dc by the D14 differential, before any
    // RT-SRCBODY-BIND-ORDER commit.
    // It refuses at object emission, so the program never executes and no
    // binding order is observable in it.
    // Annotation only -- test body and expectations are unchanged.
    #[test]
    // RT-SITEOP-CARRIED-WITNESS D1a/D2: FsWriteFile Argument(0) was site-bound:
    // FileError SiteOperand(0) could not project its carried word. D5 byte-span
    // observation was not the blocker; D2 supplies the exact emitted-helper port.
    fn cwd_root_preserves_scope_escape_and_symlink_denied_identities() {
        for (scenario, expected) in [
            (
                cwd_root_denial_scenario(b"../escape".to_vec(), false),
                CapabilityDeniedV1::ScopeEscape,
            ),
            (
                cwd_root_denial_scenario(b"link".to_vec(), true),
                CapabilityDeniedV1::SymlinkDenied,
            ),
        ] {
            let run = run_scenario(&scenario).expect("PX15 denial differential");
            run.compare_exact().expect("typed denial equality");
            assert_eq!(run.interpreter.exit_status, 44);
            assert!(run.interpreter.filesystem_delta.is_empty());
            assert!(denial_precedes_host_action(
                &run.interpreter_actions,
                &run.interpreter
            ));
            assert!(matches!(
                &run.interpreter.effect_trace[0].outcome,
                CanonicalOutcomeV1::Error(SemanticErrorV1::File(error))
                    if error.cause == FileErrorCauseV1::Capability(expected)
            ));
        }
    }

    #[test]
    fn capture_host_is_explicitly_insufficient_negative_control() {
        let source = r#"program capabilities FS APartial
proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line "capture-only")
"#;
        let mut host = ken_interp::CaptureHost::new(Vec::new());
        let outcome = ken_cli::run_program(
            source,
            ken_cli::SourceFormat::Ken,
            &[b"capture-control".to_vec()],
            &[],
            b"/",
            &mut host,
        )
        .expect("CaptureHost unit control runs");
        assert_eq!(host.stdout(), b"capture-only\n");

        let observation = EffectObservation {
            stdout: host.stdout().to_vec(),
            stderr: host.stderr().to_vec(),
            filesystem_delta: Vec::new(),
            terminal_error: None,
            effect_trace: vec![ken_host::EffectEvent {
                sequence: 0,
                operation: HostOpV1::ConsoleWrite,
                capability: None,
                resource_bindings: Vec::new(),
                request: ken_host::CanonicalRequestV1::ConsoleWrite {
                    stream: ken_host::ConsoleStreamV1::Stdout,
                    bytes: b"capture-only\n".to_vec(),
                },
                outcome: ken_host::CanonicalOutcomeV1::Success(ken_host::CanonicalReplyV1::Unit),
            }],
            terminal_exit: ken_host::TerminalExitClass::NormalReturn,
            exit_status: outcome.exit_status,
        };
        let evidence = NativeTestedEvidence::unit_or_negative_control(
            HostOpV1::ConsoleWrite,
            &observation,
            &observation,
        );
        assert!(!evidence.permits_confirmation());
        assert_eq!(
            confirm_native_tested_transition(HostOpV1::ConsoleWrite, evidence),
            Err(StatusTransitionError::MissingExactArtifactEvidence(
                HostOpV1::ConsoleWrite
            ))
        );
    }
}
