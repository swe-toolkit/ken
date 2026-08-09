//! `RT-MATCH-RECURSOR-CONSUMERS` `AC-1`, frame section 4a.1 -- the CHILD-PROCESS
//! census transport.
//!
//! Section 4a's recorder is a thread-local scope, so it reaches only
//! compilations that happen in the test's own process. The suites that invoke
//! the `ken` binary compile in a child, and a child `ken native-build` reaches
//! `ken_cli::build_native_program` and the common Runtime compilation entry --
//! so those entries are in `AC-1` and were unobserved.
//!
//! This suite observes them through the SAME recorder, carried across the
//! process boundary by one versioned envelope per child. There is no second
//! census: no second enumerator, no second row schema, no sampling rule.
//!
//! **The population boundary is the real compilation gate, not process shape.**
//! `ken native-build` is in; `ken check` is out because it never reaches that
//! entry, not because of anything about how it was launched.
//!
//! **Observation only.** The transport can select which session is recorded and
//! where its envelope lands. It cannot reach a residual, an exclusion, an
//! authority, a lane, a source, or any planner/ABI value -- and the parity
//! control below is what holds that to account rather than asserting it.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Verbatim from `mrc_4a_cross_crate_census.rs`, which took it verbatim from
/// `px7m_hostresult_computational_match.rs`. This suite therefore drives a
/// program already known to elaborate and reach linked native lowering: a
/// census whose fixture is novel measures the fixture first.
const CENSUS_PROGRAM: &str = r#"program capabilities FS APartial
proc two_step (label : String) : HostIO APartial Unit visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    Unit Unit
    (host_console APartial Unit (print_line label))
    (\_. bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result IOError Unit) Unit
      (host_console APartial (Result IOError Unit) (flush Stdout))
      (\_. Ret (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        Unit MkUnit))

proc after_write (written : Result IOError Unit)
  : HostIO APartial Unit visits [Console] =
  match written {
    Err _ |-> two_step "unexpected-error" ;
    Ok unit |-> match unit { MkUnit |-> two_step "ok-payload" }
  }

proc inner : HostIO APartial Unit visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result IOError Unit) Unit
    (host_console APartial (Result IOError Unit)
      (write Stdout (bytes_encode "probe:")))
    after_write

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    Unit ExitCode inner (\_. host_exit APartial Success)
"#;

// ---- the envelope, and the instrument that reads it ----------------------

#[derive(Debug, Clone, PartialEq, Eq)]
struct Row {
    ordinal: u64,
    run: u32,
    thread: String,
    validator_admitted: bool,
    reached_selector: bool,
    authority: Option<String>,
    residuals: Vec<String>,
}

#[derive(Debug, Clone)]
struct Envelope {
    session: String,
    parent: String,
    pid: u32,
    rows: Vec<Row>,
}

/// Parse one envelope, failing closed.
///
/// Frame 4a.1 puts observation failure in the parent test: missing, duplicate,
/// malformed, wrong-session, or incomplete output must red a control here. So
/// every deviation returns `Err` and no shape is tolerated -- in particular a
/// file that stops early fails on its missing `end` terminator rather than
/// reading as a legitimately smaller census.
fn parse_envelope(text: &str) -> Result<Envelope, String> {
    let mut lines = text.lines();
    let header = lines.next().ok_or("empty envelope")?;
    if header != "mrc-census-envelope\tv1" {
        return Err(format!("unknown envelope header: {header:?}"));
    }

    let field = |line: Option<&str>, key: &str| -> Result<String, String> {
        let line = line.ok_or_else(|| format!("envelope ends before `{key}`"))?;
        let (found, value) = line
            .split_once('\t')
            .ok_or_else(|| format!("malformed `{key}` line: {line:?}"))?;
        if found != key {
            return Err(format!("expected `{key}`, found {found:?}"));
        }
        Ok(value.to_string())
    };

    let session = field(lines.next(), "session")?;
    let parent = field(lines.next(), "parent")?;
    let pid: u32 = field(lines.next(), "pid")?
        .parse()
        .map_err(|_| "unparseable pid".to_string())?;
    let declared: usize = field(lines.next(), "rows")?
        .parse()
        .map_err(|_| "unparseable row count".to_string())?;

    let mut rows = Vec::new();
    for _ in 0..declared {
        let line = lines
            .next()
            .ok_or_else(|| format!("envelope declares {declared} rows and is short"))?;
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 8 || parts[0] != "row" {
            return Err(format!("malformed row line: {line:?}"));
        }
        let boolean = |s: &str| -> Result<bool, String> {
            s.parse().map_err(|_| format!("unparseable bool {s:?}"))
        };
        rows.push(Row {
            ordinal: parts[1]
                .parse()
                .map_err(|_| "unparseable ordinal".to_string())?,
            run: parts[2]
                .parse()
                .map_err(|_| "unparseable run".to_string())?,
            thread: parts[3].to_string(),
            validator_admitted: boolean(parts[4])?,
            reached_selector: boolean(parts[5])?,
            authority: match parts[6] {
                "-" => None,
                other => Some(other.to_string()),
            },
            residuals: if parts[7].is_empty() {
                Vec::new()
            } else {
                parts[7].split(',').map(str::to_string).collect()
            },
        });
    }

    // The terminator is the whole reason a truncated write is detectable.
    let terminator = field(lines.next(), "end")?;
    if terminator.parse::<usize>().ok() != Some(declared) {
        return Err(format!(
            "terminator {terminator:?} disagrees with declared row count {declared}"
        ));
    }
    if lines.next().is_some() {
        return Err("trailing content after the envelope terminator".to_string());
    }

    Ok(Envelope {
        session,
        parent,
        pid,
        rows,
    })
}

/// Read the one envelope an observation directory must contain, checking BOTH
/// identity axes the parent controls.
///
/// "Exactly one" is checked against the DIRECTORY, not against the path we
/// chose: a transport that appended, or that shared a sink between children,
/// shows up here as a wrong file count rather than as a plausible census.
///
/// The parent check is not redundant with the session check. The merged identity
/// is `(session, parent test/thread, ordinal)`, and a control that validates only
/// the session is insensitive to a parent value that is missing, substituted, or
/// shared between children -- distinct sessions alone would keep every downstream
/// uniqueness assertion green. Validating it on EVERY observed path is what makes
/// the second axis load-bearing rather than merely present.
fn read_sole_envelope(dir: &Path, expected_session: &str, expected_parent: &str) -> Envelope {
    let mut found: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("observation dir {} unreadable: {error}", dir.display()))
        .map(|entry| entry.expect("dir entry").path())
        .collect();
    found.sort();
    assert_eq!(
        found.len(),
        1,
        "expected exactly one envelope in {}, found {found:?}",
        dir.display()
    );

    let text = std::fs::read_to_string(&found[0]).expect("envelope readable");
    let envelope = parse_envelope(&text)
        .unwrap_or_else(|error| panic!("BROKEN INSTRUMENT -- envelope unparseable: {error}"));
    assert_emitted_identity(&envelope, expected_session, expected_parent);
    envelope
}

/// Both identity axes, checked against what the envelope ACTUALLY CARRIES.
///
/// Every path that parses an envelope goes through this, so no path can validate
/// one axis and quietly skip the other. That is not stylistic: the concurrent
/// path previously did its own inline check of the session alone, and asserting
/// the parent the test PASSED IN is a different claim from asserting the parent
/// the child EMITTED. The first tests the environment hand-off; only the second
/// can catch a transport that substitutes or shares the field on its way out.
fn assert_emitted_identity(envelope: &Envelope, expected_session: &str, expected_parent: &str) {
    assert_eq!(
        envelope.session, expected_session,
        "wrong-session envelope: the child wrote an observation this parent did not open"
    );
    assert_eq!(
        envelope.parent, expected_parent,
        "wrong-parent envelope: the emitted parent identity is not the invoking test/thread \
         identity, so the second axis of the merged identity is not established"
    );
}

/// The three properties every well-formed census must have, wherever it was
/// recorded. Identical in content to the in-process suite's controls, because
/// this is the same recorder reached through a different transport.
fn assert_census_shape(envelope: &Envelope) {
    let rows = &envelope.rows;

    // Captured exactly once. The merged identity is
    // (session/run, parent test/thread, child-local ordinal); a colliding key
    // would DEDUPLICATE, making the population read smaller and cleaner.
    let mut keys: Vec<(&str, &str, u64)> = rows
        .iter()
        .map(|row| {
            (
                envelope.session.as_str(),
                envelope.parent.as_str(),
                row.ordinal,
            )
        })
        .collect();
    let before = keys.len();
    keys.sort_unstable();
    keys.dedup();
    assert_eq!(
        keys.len(),
        before,
        "every row must carry a distinct (session, parent, ordinal) key"
    );

    let ordinals: Vec<u64> = rows.iter().map(|row| row.ordinal).collect();
    assert_eq!(
        ordinals,
        (0..rows.len() as u64).collect::<Vec<_>>(),
        "ordinals must be dense from zero and in order, or a compilation went unrecorded"
    );

    // `entry = selector-arrival + pre-selector-return`, in both directions.
    let arrived = rows.iter().filter(|row| row.reached_selector).count();
    let returned_early = rows.iter().filter(|row| !row.reached_selector).count();
    assert_eq!(
        arrived + returned_early,
        rows.len(),
        "entry must equal selector-arrival plus pre-selector-return"
    );
    for row in rows.iter().filter(|row| row.reached_selector) {
        assert!(
            row.validator_admitted,
            "a row that reached the selector must have been admitted by the transport validator"
        );
        assert!(
            row.authority.is_some(),
            "a row that reached the selector must carry production's own authority"
        );
    }
    for row in rows.iter().filter(|row| !row.reached_selector) {
        assert!(
            row.authority.is_none(),
            "a row that never reached the selector cannot carry an authority"
        );
    }

    // A known NON-MEMBER fixture. A firing row here is a real AC-1 population
    // member: it preserves its exact input and returns through the D1/hard-stop
    // route, and must never be asserted away.
    let firing: Vec<&Row> = rows
        .iter()
        .filter(|row| {
            row.residuals
                .iter()
                .any(|residual| residual == "MatchScrutineeRecursor")
        })
        .collect();
    assert!(
        firing.is_empty(),
        "this fixture is a known NON-MEMBER; a MatchScrutineeRecursor row is a real AC-1 \
         population member and must be returned through the D1/hard-stop path: {firing:?}"
    );
}

// ---- driving a child -----------------------------------------------------

fn ken_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_ken"))
}

fn parent_identity() -> String {
    std::thread::current()
        .name()
        .unwrap_or("<unnamed>")
        .to_string()
}

/// A per-case scratch root, cleared so a previous run's artifacts cannot be
/// mistaken for this one's.
fn case_root(case: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(case);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

/// The source file every child compiles or checks. Held FIXED across commands
/// so the entry/non-entry comparison varies only the command -- the gate --
/// and not the input.
fn census_source(root: &Path) -> PathBuf {
    let path = root.join("census.ken");
    std::fs::write(&path, CENSUS_PROGRAM).expect("source written");
    path
}

struct ChildResult {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

/// One child invocation. `observation` is `Some((session, sink))` to open a
/// session, `None` to leave the child unobserved.
///
/// The parent identity is passed IN rather than read here, so that every child
/// in this test -- serial and spawned alike -- carries one value captured once on
/// the test thread. Reading it per call site invites a caller on some other
/// thread to hand a child a different identity than the one asserted against.
fn run_child(args: &[&str], observation: Option<(&str, &Path)>, parent: &str) -> ChildResult {
    let mut command = Command::new(ken_binary());
    command.args(args);
    if let Some((session, sink)) = observation {
        command
            .env("KEN_MRC_CENSUS_SESSION", session)
            .env("KEN_MRC_CENSUS_PARENT", parent)
            .env("KEN_MRC_CENSUS_SINK", sink);
    }
    let output = command.output().expect("child `ken` runs");
    ChildResult {
        code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

/// Replace the one test-chosen path in the child's output. Frame 4a.1 permits
/// normalizing the artifact path and nothing else, so this substitutes exactly
/// the scratch root and leaves every other byte alone.
fn normalize(text: &str, root: &Path) -> String {
    text.replace(&root.display().to_string(), "<ROOT>")
}

// ---- the controls --------------------------------------------------------

/// All five section-4a.1 transport controls, in one test so that the ordering
/// claim is true by construction.
///
/// **The positive control runs first and everything else depends on it.** A
/// zero-row census is ambiguous between "correctly a non-entry" and "the
/// instrument never existed in the child": `ken-cli` receives
/// `px8-ds-test-support` only through a `[dev-dependencies]` edge, so whether
/// the launched binary carries the feature is a property of Cargo's unit graph
/// that no frame asserts. Until a known child compile has produced a real row,
/// no zero anywhere below means anything.
#[test]
fn mrc_4a1_child_transport_and_its_controls() {
    // The parent test/thread identity, captured ONCE on this thread and threaded
    // through every child below -- serial and spawned alike. Reading it again at
    // each call site would let a spawned worker hand its child a different
    // identity than the one asserted against, and both sides would still agree
    // because each would have re-derived the same wrong value.
    let parent = parent_identity();

    // ---- CONTROL 1: the positive child entry. Load-bearing; runs first. ----
    let root = case_root("mrc_4a1_positive");
    let source = census_source(&root);
    let observations = root.join("obs");
    std::fs::create_dir_all(&observations).expect("obs dir");
    let session = "mrc-4a1-positive";
    let sink = observations.join("positive.envelope");
    let out = root.join("out");

    let observed = run_child(
        &[
            "native-build",
            source.to_str().unwrap(),
            out.to_str().unwrap(),
        ],
        Some((session, &sink)),
        &parent,
    );
    // The child must have terminated normally rather than by signal. Its
    // COMPILE OUTCOME is deliberately not pinned here: this fixture reaches the
    // common compilation entry and is then refused later, at object emission.
    // That is the case `AC-1` most cares about -- entry rows are recorded before
    // the transport validator, so a refusal still has a census -- and it is
    // exactly why the envelope is written before the CLI converts its result to
    // an exit. Pinning success here would measure the fixture, not the gate.
    assert!(
        observed.code.is_some(),
        "the child must terminate normally; stderr: {}",
        observed.stderr
    );
    println!("MRC-4A1-POSITIVE-EXIT\t{:?}", observed.code);

    let envelope = read_sole_envelope(&observations, session, &parent);
    assert!(
        !envelope.rows.is_empty(),
        "BROKEN INSTRUMENT -- a known child native compile produced zero rows. The child \
         installed no scope (feature absent from the launched binary) or never reached the \
         common compilation entry. Until this row exists, every zero below is uninterpretable"
    );
    assert_census_shape(&envelope);

    println!("MRC-4A1-POSITIVE-ROWS\t{}", envelope.rows.len());
    println!("MRC-4A1-POSITIVE-PID\t{}", envelope.pid);
    for row in &envelope.rows {
        println!(
            "MRC-4A1-ROW\tordinal={}\trun={}\tthread={}\tresiduals={:?}\tadmitted={}\tselector={}\tauthority={:?}",
            row.ordinal,
            row.run,
            row.thread,
            row.residuals,
            row.validator_admitted,
            row.reached_selector,
            row.authority
        );
    }

    // ---- The instrument's own fail-closed control. -----------------------
    // A parser that accepted a short file would report a truncated envelope as
    // a smaller census, which is the one failure that looks like a result.
    let full = std::fs::read_to_string(&sink).expect("envelope readable");
    let truncated: String = full.lines().take(4).collect::<Vec<_>>().join("\n");
    assert!(
        parse_envelope(&truncated).is_err(),
        "the envelope reader must reject an incomplete envelope rather than read it short"
    );
    assert!(
        parse_envelope(&full).is_ok(),
        "positive control on the reader: the complete envelope must still parse"
    );

    // ---- CONTROL 2: observation absent vs present is byte-identical. -----
    // If these disagree the transport is activation, not observation, and
    // everything else here is void.
    let absent_root = case_root("mrc_4a1_parity_absent");
    let absent_source = census_source(&absent_root);
    let absent = run_child(
        &[
            "native-build",
            absent_source.to_str().unwrap(),
            absent_root.join("out").to_str().unwrap(),
        ],
        None,
        &parent,
    );

    let present_root = case_root("mrc_4a1_parity_present");
    let present_source = census_source(&present_root);
    let present_obs = present_root.join("obs");
    std::fs::create_dir_all(&present_obs).expect("obs dir");
    let present = run_child(
        &[
            "native-build",
            present_source.to_str().unwrap(),
            present_root.join("out").to_str().unwrap(),
        ],
        Some(("mrc-4a1-parity", &present_obs.join("parity.envelope"))),
        &parent,
    );

    assert_eq!(
        absent.code, present.code,
        "observation must not change the child's exit status"
    );
    assert_eq!(
        normalize(&absent.stdout, &absent_root),
        normalize(&present.stdout, &present_root),
        "observation must not change the child's stdout"
    );
    assert_eq!(
        normalize(&absent.stderr, &absent_root),
        normalize(&present.stderr, &present_root),
        "observation must not change the child's stderr"
    );
    // The unobserved child must leave no artifact behind at all.
    assert!(
        !absent_root.join("obs").exists(),
        "an unobserved child created an observation artifact"
    );
    // And the observed one is still a real census, not an empty stand-in.
    let parity_envelope = read_sole_envelope(&present_obs, "mrc-4a1-parity", &parent);
    assert_eq!(
        parity_envelope.rows.len(),
        envelope.rows.len(),
        "the same command observed twice must record the same number of entries"
    );

    // ---- CONTROL 3: two concurrent children do not collide. --------------
    let concurrent_root = case_root("mrc_4a1_concurrent");
    let concurrent_obs = concurrent_root.join("obs");
    std::fs::create_dir_all(&concurrent_obs).expect("obs dir");

    let mut handles = Vec::new();
    for index in 0..2u32 {
        let child_root = concurrent_root.join(format!("child{index}"));
        std::fs::create_dir_all(&child_root).expect("child dir");
        let child_source = census_source(&child_root);
        let session = format!("mrc-4a1-concurrent-{index}");
        let sink = concurrent_obs.join(format!("{session}.envelope"));
        let binary = ken_binary();
        let parent = parent.clone();
        handles.push(std::thread::spawn(move || {
            let output = Command::new(binary)
                .args([
                    "native-build",
                    child_source.to_str().unwrap(),
                    child_root.join("out").to_str().unwrap(),
                ])
                .env("KEN_MRC_CENSUS_SESSION", &session)
                .env("KEN_MRC_CENSUS_PARENT", &parent)
                .env("KEN_MRC_CENSUS_SINK", &sink)
                .output()
                .expect("concurrent child runs");
            (session, sink, parent, output.status.code())
        }));
    }

    let mut union: Vec<(String, String, u64)> = Vec::new();
    let mut expected_total = 0usize;
    for handle in handles {
        let (session, sink, worker_parent, code) = handle.join().expect("concurrent child joins");
        assert_eq!(
            worker_parent, parent,
            "each spawned worker must hand its child the identity captured on the test thread"
        );
        assert_eq!(
            code, observed.code,
            "concurrent child {session} must reach the same outcome as the serial one"
        );
        let text = std::fs::read_to_string(&sink)
            .unwrap_or_else(|error| panic!("envelope for {session} missing: {error}"));
        let child = parse_envelope(&text)
            .unwrap_or_else(|error| panic!("envelope for {session} unparseable: {error}"));
        // Both axes, from the EMITTED envelope. `worker_parent == parent` above
        // checks only what this test handed the child; this checks what the
        // child wrote back, which is the axis a transport defect would move.
        assert_emitted_identity(&child, &session, &worker_parent);
        assert!(
            !child.rows.is_empty(),
            "concurrent child {session} recorded nothing -- loss, not a small census"
        );
        assert_census_shape(&child);
        expected_total += child.rows.len();
        union.extend(
            child
                .rows
                .iter()
                .map(|row| (child.session.clone(), child.parent.clone(), row.ordinal)),
        );
    }

    let entries = std::fs::read_dir(&concurrent_obs)
        .expect("obs dir readable")
        .count();
    assert_eq!(
        entries, 2,
        "two concurrent children must produce exactly two envelopes, never a shared sink"
    );
    let before_union = union.len();
    union.sort();
    union.dedup();
    assert_eq!(
        union.len(),
        before_union,
        "the two sessions must union without collision"
    );
    assert_eq!(
        union.len(),
        expected_total,
        "the union must lose no row from either child"
    );

    // ---- CONTROL 4: feature-on with no session is inert. -----------------
    // This binary is feature-enabled -- CONTROL 1 proved it by recording a row
    // -- so an unobserved run here isolates the session, not the feature.
    let inert_root = case_root("mrc_4a1_inert");
    let inert_source = census_source(&inert_root);
    let never_passed = inert_root.join("never-passed.envelope");
    let inert = run_child(
        &[
            "native-build",
            inert_source.to_str().unwrap(),
            inert_root.join("out").to_str().unwrap(),
        ],
        None,
        &parent,
    );
    assert_eq!(
        inert.code, observed.code,
        "the inert child must reach the same outcome as the observed one"
    );
    assert!(
        !never_passed.exists(),
        "feature-on with no session must create no artifact"
    );
    let stray: Vec<PathBuf> = std::fs::read_dir(&inert_root)
        .expect("inert root readable")
        .map(|entry| entry.expect("entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "envelope"))
        .collect();
    assert!(
        stray.is_empty(),
        "feature-on with no session wrote an envelope anyway: {stray:?}"
    );

    // ---- CONTROL 5: a non-entry command, classified rather than omitted. --
    // `ken check` elaborates the SAME source and never reaches the common
    // native compilation entry. Holding the input fixed and varying only the
    // command is what makes this a statement about the gate rather than about
    // the fixture.
    let non_entry_root = case_root("mrc_4a1_non_entry");
    let non_entry_source = census_source(&non_entry_root);
    let non_entry_obs = non_entry_root.join("obs");
    std::fs::create_dir_all(&non_entry_obs).expect("obs dir");
    let non_entry_session = "mrc-4a1-non-entry";
    let non_entry = run_child(
        &["check", non_entry_source.to_str().unwrap()],
        Some((non_entry_session, &non_entry_obs.join("non-entry.envelope"))),
        &parent,
    );
    assert_eq!(
        non_entry.code,
        Some(0),
        "`ken check` on the census fixture must succeed; stderr: {}",
        non_entry.stderr
    );

    // The envelope must EXIST. That is the whole content of "classified as a
    // non-entry, not silently omitted": a present envelope with zero rows says
    // the instrument ran and saw nothing, which no absent file can say.
    let non_entry_envelope = read_sole_envelope(&non_entry_obs, non_entry_session, &parent);
    assert!(
        non_entry_envelope.rows.is_empty(),
        "`ken check` reached the common native compilation entry, which contradicts the ruled \
         population boundary: {:?}",
        non_entry_envelope.rows
    );
    println!(
        "MRC-4A1-NON-ENTRY\tsession={}\trows={}",
        non_entry_envelope.session,
        non_entry_envelope.rows.len()
    );
}
