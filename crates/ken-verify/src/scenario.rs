//! Checked-source, twin-real-root canonical differential runner.

use std::ffi::OsString;
use std::fmt;

use num_bigint::BigInt;

#[cfg(unix)]
use std::os::unix::ffi::{OsStrExt, OsStringExt};

use ken_elaborator::capabilities::{
    Authority, RightSet, SymlinkPolicy, AUTH_FULL, AUTH_PARTIAL,
};
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

fn execute_scenario(
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
