fn output_dir() -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix("ken-rt-nested-ih-native-")
        .tempdir()
        .unwrap()
}

const NESTED_NAT_THREE: &str = r#"program capabilities FS APartial

data Bag (a : Type) : Type where {
  Empty : Bag a;
  One : a -> Bag a;
  Join : Bag a -> Bag a -> Bag a
}

data LiftRose = LiftLeaf | LiftNode (Bag LiftRose)

fn liftAdd (x : Nat) (y : Nat) : Nat = match x {
  Zero |-> y;
  Suc x2 |-> Suc (liftAdd x2 y)
}

fn liftSize (r : LiftRose) : Nat = match r {
  LiftLeaf |-> Suc Zero;
  LiftNode b |-> Suc (match b {
    Empty |-> Zero;
    One x |-> liftSize x;
    Join xs ys |-> liftAdd (recursive result for xs)
                          (recursive result for ys)
  })
}

fn exactNatThree (value : Nat) : ExitCode = match value {
  Zero |-> Failure 30;
  Suc one |-> match one {
    Zero |-> Failure 31;
    Suc two |-> match two {
      Zero |-> Failure 32;
      Suc three |-> match three {
        Zero |-> Success;
        Suc _ |-> Failure 34
      }
    }
  }
}

fn main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode =
  host_exit APartial (exactNatThree (liftSize
    (LiftNode (Join LiftRose
      (One LiftRose LiftLeaf)
      (One LiftRose (LiftNode (Empty LiftRose)))))))
"#;

#[test]
fn nested_checked_ih_native_result_is_exactly_interpreter_nat_three() {
    // Promise class: durable invariant. The same checked source traverses both
    // real executors, and the four-way result classifier accepts exactly Nat 3.
    //
    // MEASURED: the linked native artifact and the interpreter produce the same
    // complete process observation, whose exit is Success only after three Suc
    // constructors and then Zero.
    // CLAIMED: nested checked computational IH realizes natively and returns
    // exactly the interpreter's Nat 3.
    // THE GAP: the process boundary observes the result through an injective
    // classifier around 3 rather than exporting the opaque Nat itself.
    let root = output_dir();
    let output = ken_cli::build_native_program(
        NESTED_NAT_THREE,
        ken_cli::SourceFormat::Ken,
        "rt-nested-ih-native-realization",
        root.path(),
    )
    .expect("nested checked IH reaches a linked native artifact");
    let mut host = ken_interp::CaptureHost::new(Vec::new());
    let interpreted = ken_cli::run_program_effect_observation(
        NESTED_NAT_THREE,
        ken_cli::SourceFormat::Ken,
        &[b"ken".to_vec()],
        &[],
        b"/",
        &mut host,
    )
    .expect("the identical checked source runs through the interpreter");
    assert_eq!(interpreted.exit_status, 0, "the interpreter result was not Nat 3");

    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .expect("nested checked IH linked artifact runs");

    assert_eq!(native, interpreted);
    assert_eq!(native.exit_status, 0, "the result was not exactly Nat 3");
    assert_eq!(native.terminal_error, None);
    assert_eq!(
        native.terminal_exit,
        ken_runtime::TerminalExitClass::NormalReturn
    );
}
