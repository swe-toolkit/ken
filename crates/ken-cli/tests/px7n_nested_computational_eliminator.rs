fn output_dir(name: &str) -> tempfile::TempDir {
    let prefix = format!("ken-px7n-{name}-");
    tempfile::Builder::new().prefix(&prefix).tempdir().unwrap()
}

const PROGRAM: &str = r#"program capabilities FS APartial
proc wrap_result (as_ok : Bool)
  : HostIO APartial (Result Bytes Bytes) visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result IOError Unit) (Result Bytes Bytes)
    (host_console APartial (Result IOError Unit)
      (write Stdout (bytes_encode "seed:")))
    (\written. match written {
      Err _ |-> Ret (Coproduct (FSOp APartial) AmbientOp)
        (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
        (Result Bytes Bytes) (Err Bytes Bytes (bytes_encode "host-error"));
      Ok _ |-> match as_ok {
        False |-> Ret (Coproduct (FSOp APartial) AmbientOp)
          (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
          (Result Bytes Bytes) (Err Bytes Bytes (bytes_encode "err-payload"));
        True |-> Ret (Coproduct (FSOp APartial) AmbientOp)
          (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
          (Result Bytes Bytes) (Ok Bytes Bytes (bytes_encode "ok-payload"))
      }
    })

proc write_then_exit (bytes : Bytes) (code : ExitCode)
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result IOError Unit) ExitCode
    (host_console APartial (Result IOError Unit) (write Stdout bytes))
    (\_. bind (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result IOError Unit) ExitCode
      (host_console APartial (Result IOError Unit) (flush Stdout))
      (\_. host_exit APartial code))

proc wrap_again (body : Unit -> HostIO APartial (Result Bytes Bytes))
  : HostIO APartial ExitCode visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result Bytes Bytes) ExitCode
    (relay body)
    finish

proc relay (body : Unit -> HostIO APartial (Result Bytes Bytes))
  : HostIO APartial (Result Bytes Bytes) visits [Console] =
  bind (Coproduct (FSOp APartial) AmbientOp)
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
    (Result Bytes Bytes) (Result Bytes Bytes)
    (body MkUnit)
    (\outcome. Ret (Coproduct (FSOp APartial) AmbientOp)
      (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp)
      (Result Bytes Bytes) outcome)

proc finish (outcome : Result Bytes Bytes)
  : HostIO APartial ExitCode visits [Console] =
  match outcome {
    Err bytes |-> write_then_exit bytes (Failure 7);
    Ok bytes |-> write_then_exit bytes Success
  }

proc main (input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode visits [Console] =
  match input {
    MkProcessInput arguments _environment _cwd |-> match arguments {
      Nil |-> host_exit APartial (Failure 99);
      Cons _ tail |-> match tail {
        Nil |-> wrap_again (\_. wrap_result True);
        Cons _ _ |-> wrap_again (\_. wrap_result False)
      }
    }
  }
"#;

fn assert_case(arguments: &[&str], expected_stdout: &[u8], expected_exit: i32) {
    let name = if arguments.is_empty() { "ok" } else { "err" };
    let dir = output_dir(name);
    let output = ken_cli::build_native_program(
        PROGRAM,
        ken_cli::SourceFormat::Ken,
        "px7n-nested-computational-eliminator",
        dir.path(),
    )
    .expect("nested computational eliminators compose in the linked artifact");
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: arguments.iter().map(std::ffi::OsString::from).collect(),
            environment: Vec::new(),
            cwd: dir.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("linked artifact returns its complete observation");

    let mut argv = vec![b"ken".to_vec()];
    argv.extend(
        arguments
            .iter()
            .map(|argument| argument.as_bytes().to_vec()),
    );
    let mut host = ken_interp::CaptureHost::new(Vec::new());
    let interpreted = ken_cli::run_program_effect_observation(
        PROGRAM,
        ken_cli::SourceFormat::Ken,
        &argv,
        &[],
        b"/",
        &mut host,
    )
    .expect("same checked source runs through the interpreter");

    assert_eq!(native, interpreted);
    assert_eq!(native.exit_status, expected_exit);
    assert_eq!(native.stdout, expected_stdout);
    assert_eq!(
        native
            .effect_trace
            .iter()
            .map(|event| event.operation)
            .collect::<Vec<_>>(),
        vec![
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleWrite,
            ken_runtime::HostOpV1::ConsoleFlush,
        ]
    );
}

// Ignored pending RT-FRAME-MARKER-ONCE.
//
// Observed signature, exactly:
//   OrientedSubcontinuationPlanV1: checked Runtime frame marker was
//     consumed more than once
//
// Owner node: RT-FRAME-MARKER-ONCE.
// Pre-existing base debt, NOT a bind-order regression: this row fails at
// base 21fd46dc as well, measured by the D12 two-way differential over the
// complete --no-fail-fast surface of both packages.
// It refuses at object emission, so the program never executes and no
// binding order is observable in it.
// The Ok twin of nested_err_payload_reaches_both_real_executors in this
// same file, and it refuses identically.
// Annotation only -- test body and expectations are unchanged.
#[test]
#[ignore = "RT-DUPLICATED-RESPONSE-BLOCK, readmission half measured at 9dfa6978e; the census and the refusal attribution are carried from RT-HOST-RESPONSE-ROUTE-KEY-COLLISION measured at 89dc3b0e5 and re-measured unchanged here. This row refuses EARLIER than its label says, at the planner invariant two host response cases claim one operation constructor. That collision is a FALSE ALARM. Both rows in this file build the SAME single program (const PROGRAM, reached via assert_case), so one census covers both. Measured on THIS program (px7n-nested-computational-eliminator), census run to completion and counted by the probe itself rather than by grep: 29 colliding constructors, all 29 agreeing on their operation, and the effect-origin deltas a single distinct value of 365. A constant offset across the whole set is a duplicated block, not 29 competing claims. The count is of COLLISIONS OBSERVED IN THIS PLAN; it is not a claim about how many host-operation constructors exist. THIS LABEL IS NOT STALE, IT IS SHADOWED: with the collision deferred to the point of use the run reaches exactly the labelled mechanism, OrientedSubcontinuationPlanV1: checked Runtime frame marker was consumed more than once. WHERE THE DUPLICATION ENTERS, measured: main's inner match tail carries two arms, each calling wrap_again (\\_. wrap_result B), so inlining instantiates the wrap_again -> relay -> wrap_result chain once per arm and presents one program's response cases to host_response_routes twice. A single-arm fixture, run with the production check LIVE, removes the collision entirely and this row advances to the frame-marker mechanism above. The check is therefore FAITHFUL: it reports a real second presentation, and the plan that produces it is what correct inlining of two call sites of one proc looks like. CENSUS RE-MEASURED at 9dfa6978e and UNCHANGED: 29 colliding constructors, all 29 agreeing on their operation, effect-origin deltas a single value of 365. That is PER INVOCATION of host_response_routes, which runs TWICE in this compile, so a line count over the probe's output reads 58 where the census reads 29; the two numbers measure different things and neither is wrong. DELETING THE CHECK IS NOT THE REPAIR, measured: with only the refusal suppressed and nothing else changed, each invocation performs 29 silent overwrites, dropping 29 constructors' wiring coordinates, and NOTHING refuses -- not at construction, and not at the point of use, where selected_host_response_route's own one Vis operation subtree selects more than one host response producer does not fire. The last-written route is simply used. READMITS when the route map is keyed on the OCCURRENCE as well as the constructor, AND the frame-marker single-consumption of RT-FRAME-MARKER-ONCE holds. The key change is owned by a successor node that is framed and NOT on main at the time of writing, so it is deliberately not named here; until it lands this row's owner is RT-DUPLICATED-RESPONSE-BLOCK. Base 21fd46dc in the prior label was stale and is dropped."]
fn nested_ok_payload_reaches_both_real_executors() {
    assert_case(&[], b"seed:ok-payload", 0);
}

// Ignored pending RT-FRAME-MARKER-ONCE.
//
// Observed signature, exactly:
//   OrientedSubcontinuationPlanV1: checked Runtime frame marker was consumed more than once
//
// Owner node: RT-FRAME-MARKER-ONCE.
// Pre-existing base debt, NOT a bind-order regression: measured failing at
// the frozen base 21fd46dc by the D10 differential, before any
// RT-SRCBODY-BIND-ORDER commit.
// It refuses at object emission, so the program never executes and no
// binding order is observable in it.
// px7o_heterogeneous_eliminator_frames.rs defines a test of the SAME NAME.
// It is a different binary with a different cause and a different owner
// (RT-ENTRY-TRAP-PX7O) -- do not read the two annotations as copies.
// Annotation only -- test body and expectations are unchanged.
#[test]
#[ignore = "RT-DUPLICATED-RESPONSE-BLOCK, readmission half measured at 9dfa6978e; the census and the refusal attribution are carried from RT-HOST-RESPONSE-ROUTE-KEY-COLLISION measured at 89dc3b0e5 and re-measured unchanged here. This row refuses EARLIER than its label says, at the planner invariant two host response cases claim one operation constructor. That collision is a FALSE ALARM. Both rows in this file build the SAME single program (const PROGRAM, reached via assert_case), so one census covers both. Measured on THIS program (px7n-nested-computational-eliminator), census run to completion and counted by the probe itself rather than by grep: 29 colliding constructors, all 29 agreeing on their operation, and the effect-origin deltas a single distinct value of 365. A constant offset across the whole set is a duplicated block, not 29 competing claims. The count is of COLLISIONS OBSERVED IN THIS PLAN; it is not a claim about how many host-operation constructors exist. THIS LABEL IS NOT STALE, IT IS SHADOWED: with the collision deferred to the point of use the run reaches exactly the labelled mechanism, OrientedSubcontinuationPlanV1: checked Runtime frame marker was consumed more than once. WHERE THE DUPLICATION ENTERS, measured: main's inner match tail carries two arms, each calling wrap_again (\\_. wrap_result B), so inlining instantiates the wrap_again -> relay -> wrap_result chain once per arm and presents one program's response cases to host_response_routes twice. A single-arm fixture, run with the production check LIVE, removes the collision entirely and this row advances to the frame-marker mechanism above. The check is therefore FAITHFUL: it reports a real second presentation, and the plan that produces it is what correct inlining of two call sites of one proc looks like. CENSUS RE-MEASURED at 9dfa6978e and UNCHANGED: 29 colliding constructors, all 29 agreeing on their operation, effect-origin deltas a single value of 365. That is PER INVOCATION of host_response_routes, which runs TWICE in this compile, so a line count over the probe's output reads 58 where the census reads 29; the two numbers measure different things and neither is wrong. DELETING THE CHECK IS NOT THE REPAIR, measured: with only the refusal suppressed and nothing else changed, each invocation performs 29 silent overwrites, dropping 29 constructors' wiring coordinates, and NOTHING refuses -- not at construction, and not at the point of use, where selected_host_response_route's own one Vis operation subtree selects more than one host response producer does not fire. The last-written route is simply used. READMITS when the route map is keyed on the OCCURRENCE as well as the constructor, AND the frame-marker single-consumption of RT-FRAME-MARKER-ONCE holds. The key change is owned by a successor node that is framed and NOT on main at the time of writing, so it is deliberately not named here; until it lands this row's owner is RT-DUPLICATED-RESPONSE-BLOCK. Base 21fd46dc in the prior label was stale and is dropped."]
fn nested_err_payload_reaches_both_real_executors() {
    assert_case(&["err"], b"seed:err-payload", 7);
}
