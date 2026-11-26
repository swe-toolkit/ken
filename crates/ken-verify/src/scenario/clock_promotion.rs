//! Lane-local validation before projecting nondeterministic clock and entropy fields.

use std::fmt;

use ken_host::{
    CanonicalOutcomeV1 as Outcome, CanonicalReplyV1 as Reply, CanonicalRequestV1 as Request,
    EffectEvent, EffectObservation, HostOpV1 as Op, IoErrorIdentityV1,
    MAX_ENTROPY_REQUEST_BYTES_V1, RuntimeTrapCode, SemanticErrorV1, TerminalErrorV1,
    TerminalExitClass,
};
use num_bigint::BigInt;

use super::{CanonicalDifferentialRun, compare_canonical_exact};
use crate::ObservationMismatch;

#[derive(Clone, Copy, Debug)]
pub(super) struct MonotonicWindow {
    pub(super) start: u128,
    pub(super) end: u128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MonotonicReadingError {
    TraceShape {
        lane: &'static str,
        index: usize,
    },
    OutsidePlausibleWindow {
        lane: &'static str,
        reading: BigInt,
        start: u128,
        end: u128,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MonotonicNowDifferentialError {
    Reading(MonotonicReadingError),
    WentBackwards { lane: &'static str },
    Observation(ObservationMismatch),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SleepUntilDifferentialError {
    Reading(MonotonicReadingError),
    Deadline { lane: &'static str },
    ReturnedBeforeDeadline { lane: &'static str },
    Observation(ObservationMismatch),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SleepUntilDomainDifferentialError {
    Reading(MonotonicReadingError),
    TerminalClass { lane: &'static str },
    ExitStatus { interpreter: i32, native: i32 },
    TraceMismatch,
    StdoutMismatch,
    FilesystemMismatch,
    TerminalIdentity { lane: &'static str },
    StderrMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RandomBytesDifferentialError {
    TraceShape { lane: &'static str, index: usize },
    Reply { lane: &'static str, index: usize },
    RepeatedBytes { lane: &'static str },
    Observation(ObservationMismatch),
}

macro_rules! differential_error {
    ($($ty:ty),+ $(,)?) => {$ (
        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{self:?}")
            }
        }
        impl std::error::Error for $ty {}
    )+};
}

differential_error!(
    MonotonicNowDifferentialError,
    SleepUntilDifferentialError,
    SleepUntilDomainDifferentialError,
    RandomBytesDifferentialError,
);

// Compare the typed planner contract independently of the value emitted by
// Runtime. Derive the textual launcher line from this same expected value.
const SLEEP_DOMAIN_MESSAGE: &str = "Clock.sleep_until deadline is outside 0 <= d < 2^64 (38 §1.15)";
const STARTER_LINE: &[u8] = b"ken native trap: planned runtime trap token\n";

fn project_monotonic_reading(
    lane: &'static str,
    event: &mut EffectEvent,
    index: usize,
    window: MonotonicWindow,
) -> Result<BigInt, MonotonicReadingError> {
    let shape = || MonotonicReadingError::TraceShape { lane, index };
    if !ambient_event(event, index, Op::ClockMonotonicNow)
        || event.request != Request::ClockMonotonicNow
    {
        return Err(shape());
    }
    let Outcome::Success(Reply::MonotonicInstant(bytes)) = &mut event.outcome else {
        return Err(shape());
    };
    if bytes.is_empty() {
        return Err(shape());
    }
    let reading = BigInt::from_signed_bytes_be(bytes);
    if reading < BigInt::from(window.start) || reading > BigInt::from(window.end) {
        return Err(MonotonicReadingError::OutsidePlausibleWindow {
            lane,
            reading,
            start: window.start,
            end: window.end,
        });
    }
    bytes.clear();
    Ok(reading)
}

fn ambient_event(event: &EffectEvent, index: usize, operation: Op) -> bool {
    event.sequence == index as u64
        && event.operation == operation
        && event.capability.is_none()
        && event.resource_bindings.is_empty()
}

impl CanonicalDifferentialRun {
    pub fn compare_monotonic_now(&self) -> Result<(), MonotonicNowDifferentialError> {
        use MonotonicNowDifferentialError as Error;
        let project = |lane, observation: &EffectObservation, window| {
            let mut observation = observation.clone();
            let [first, second] = observation.effect_trace.as_mut_slice() else {
                return Err(Error::Reading(MonotonicReadingError::TraceShape {
                    lane,
                    index: 0,
                }));
            };
            let first =
                project_monotonic_reading(lane, first, 0, window).map_err(Error::Reading)?;
            let second =
                project_monotonic_reading(lane, second, 1, window).map_err(Error::Reading)?;
            if second < first {
                return Err(Error::WentBackwards { lane });
            }
            Ok(observation)
        };
        compare_canonical_exact(
            &project(
                "interpreter",
                &self.interpreter,
                self.interpreter_monotonic_window,
            )?,
            &project("native", &self.native, self.native_monotonic_window)?,
        )
        .map_err(Error::Observation)
    }

    pub fn compare_sleep_until(&self, duration: u64) -> Result<(), SleepUntilDifferentialError> {
        use SleepUntilDifferentialError as Error;
        let project = |lane, observation: &EffectObservation, window| {
            let mut observation = observation.clone();
            let [before, sleep, after] = observation.effect_trace.as_mut_slice() else {
                return Err(Error::Reading(MonotonicReadingError::TraceShape {
                    lane,
                    index: 0,
                }));
            };
            let before =
                project_monotonic_reading(lane, before, 0, window).map_err(Error::Reading)?;
            let after =
                project_monotonic_reading(lane, after, 2, window).map_err(Error::Reading)?;
            let Request::ClockSleepUntil { deadline } = sleep.request else {
                return Err(Error::Deadline { lane });
            };
            if !ambient_event(sleep, 1, Op::ClockSleepUntil)
                || sleep.outcome != Outcome::Success(Reply::Unit)
                || BigInt::from(deadline) != before + duration
            {
                return Err(Error::Deadline { lane });
            }
            if after < BigInt::from(deadline) {
                return Err(Error::ReturnedBeforeDeadline { lane });
            }
            sleep.request = Request::ClockSleepUntil { deadline: 0 };
            Ok(observation)
        };
        compare_canonical_exact(
            &project(
                "interpreter",
                &self.interpreter,
                self.interpreter_monotonic_window,
            )?,
            &project("native", &self.native, self.native_monotonic_window)?,
        )
        .map_err(Error::Observation)
    }

    /// MEASURED: trap class, status and exact per-lane terminal identity and
    /// stderr transport, plus the complete prefix except its reading bytes.
    /// CLAIMED: neither lane dispatched the out-of-domain deadline.
    /// THE GAP: the reading's width (i128 native, minimal interpreter) is
    /// erased with its value; being in its own window is the only value check.
    pub fn compare_sleep_until_out_of_domain(
        &self,
    ) -> Result<(), SleepUntilDomainDifferentialError> {
        compare_sleep_domain(
            &self.interpreter,
            self.interpreter_monotonic_window,
            &self.native,
            self.native_monotonic_window,
        )
    }

    pub fn compare_random_bytes(&self) -> Result<(), RandomBytesDifferentialError> {
        use RandomBytesDifferentialError as Error;
        let project = |lane, observation: &EffectObservation| {
            let mut observation = observation.clone();
            let [first, second, empty, excess] = observation.effect_trace.as_mut_slice() else {
                return Err(Error::TraceShape { lane, index: 0 });
            };
            let mut first_bytes = None;
            for (index, (event, count)) in [
                (first, 32),
                (second, 32),
                (empty, 0),
                (excess, MAX_ENTROPY_REQUEST_BYTES_V1 + 1),
            ]
            .into_iter()
            .enumerate()
            {
                if !ambient_event(event, index, Op::EntropyRandomBytes)
                    || event.request != (Request::EntropyRandomBytes { count })
                {
                    return Err(Error::TraceShape { lane, index });
                }
                if index == 3 {
                    if event.outcome
                        != Outcome::Error(SemanticErrorV1::Io(IoErrorIdentityV1::InvalidInput))
                    {
                        return Err(Error::Reply { lane, index });
                    }
                    continue;
                }
                let Outcome::Success(Reply::Bytes(bytes)) = &mut event.outcome else {
                    return Err(Error::Reply { lane, index });
                };
                if bytes.len() as u64 != count {
                    return Err(Error::Reply { lane, index });
                }
                if index == 0 {
                    first_bytes = Some(bytes.clone());
                } else if index == 1 && first_bytes.as_ref() == Some(bytes) {
                    return Err(Error::RepeatedBytes { lane });
                }
                bytes.fill(0);
            }
            Ok(observation)
        };
        compare_canonical_exact(
            &project("interpreter", &self.interpreter)?,
            &project("native", &self.native)?,
        )
        .map_err(Error::Observation)
    }
}

fn compare_sleep_domain(
    interpreter: &EffectObservation,
    interpreter_window: MonotonicWindow,
    native: &EffectObservation,
    native_window: MonotonicWindow,
) -> Result<(), SleepUntilDomainDifferentialError> {
    use SleepUntilDomainDifferentialError as Error;
    for (lane, observation) in [("interpreter", interpreter), ("native", native)] {
        if observation.terminal_exit != TerminalExitClass::ControlledTrap {
            return Err(Error::TerminalClass { lane });
        }
    }
    if interpreter.exit_status != native.exit_status {
        return Err(Error::ExitStatus {
            interpreter: interpreter.exit_status,
            native: native.exit_status,
        });
    }
    let prefix = |lane, observation: &EffectObservation, window| {
        let mut trace = observation.effect_trace.clone();
        let [before, marker] = trace.as_mut_slice() else {
            return Err(Error::Reading(MonotonicReadingError::TraceShape {
                lane,
                index: 0,
            }));
        };
        project_monotonic_reading(lane, before, 0, window).map_err(Error::Reading)?;
        if marker.sequence != 1 || marker.operation != Op::ConsoleWrite {
            return Err(Error::Reading(MonotonicReadingError::TraceShape {
                lane,
                index: 1,
            }));
        }
        Ok(trace)
    };
    if prefix("interpreter", interpreter, interpreter_window)?
        != prefix("native", native, native_window)?
    {
        return Err(Error::TraceMismatch);
    }
    if interpreter.stdout != native.stdout {
        return Err(Error::StdoutMismatch);
    }
    if interpreter.filesystem_delta != native.filesystem_delta {
        return Err(Error::FilesystemMismatch);
    }
    if interpreter.terminal_error != Some(TerminalErrorV1::DriverFailure) {
        return Err(Error::TerminalIdentity {
            lane: "interpreter",
        });
    }
    let trap = ken_host::RuntimeTrap {
        code: RuntimeTrapCode::ExplicitTrap,
        message: SLEEP_DOMAIN_MESSAGE.to_owned(),
    };
    if !matches!(&native.terminal_error, Some(TerminalErrorV1::RuntimeTrap(provenance)) if provenance.trap == trap)
    {
        return Err(Error::TerminalIdentity { lane: "native" });
    }
    let typed_line = format!("ken native trap: {:?}: {}\n", trap.code, trap.message);
    if native.stderr
        != [
            interpreter.stderr.as_slice(),
            STARTER_LINE,
            typed_line.as_bytes(),
        ]
        .concat()
    {
        return Err(Error::StderrMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
