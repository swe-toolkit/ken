//! LANG-ELAB-NESTED-FORMER-RECURSION — AC-EXECUTES-AT-RUNTIME (native half).
//!
//! The interpreter half (eval-to-2 + dropped-IH-to-0) is pinned in
//! `ken-elaborator/tests/lang_nested_former_recursion_acceptance.rs`. This file
//! adds the NATIVE/codegen execution witness the frame requires: the same
//! `List (Pair String Self)` total fold — the shape this WP newly admits — is
//! compiled to a linked native artifact, RUN, and its observed process result
//! is compared to the interpreter's. A source mutation that drops the nested-IH
//! consumption changes the observed native result, proving the generated nested
//! induction hypothesis is actually consumed at runtime (not merely admitted).

#![cfg(target_os = "linux")]

// nsize counts the NLeaf leaves reachable through the `List (Pair String Self)`
// object nesting; `two_leaves` has exactly two, so `exactNatTwo` exits Success
// (0) only when the fold consumes the Pair-nested recursive result.
const NESTED_TWO_LEAVES: &str = r#"program capabilities FS APartial

data NRose : Type where { NLeaf : NRose ; NNode : List (Pair String NRose) -> NRose }

fn add (x : Nat) (y : Nat) : Nat = match x { Zero |-> y ; Suc x2 |-> Suc (add x2 y) }

fn nsize (r : NRose) : Nat = match r {
  NLeaf |-> Suc Zero;
  NNode members |-> match members {
    Nil |-> Zero;
    Cons member rest |-> add (recursive result for member) (recursive result for rest)
  }
}

const two_leaves : NRose =
  NNode (Cons (Pair String NRose) (mk_pair String NRose "x" NLeaf)
    (Cons (Pair String NRose) (mk_pair String NRose "y" NLeaf) (Nil (Pair String NRose))))

fn exactNatTwo (value : Nat) : ExitCode = match value {
  Zero |-> Failure 20;
  Suc one |-> match one {
    Zero |-> Failure 21;
    Suc two |-> match two {
      Zero |-> Success;
      Suc _ |-> Failure 24
    }
  }
}

fn main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode =
  host_exit APartial (exactNatTwo (nsize two_leaves))
"#;

// The reaching source mutation: the SAME program whose fold DROPS the nested-IH
// consumption (`recursive result for member`). It can only ever return the list
// tail's accumulated result, so it cannot count the Pair-nested leaves — nsize
// becomes 0 and the process exits Failure 20 instead of Success.
const NESTED_DROPPED_IH: &str = r#"program capabilities FS APartial

data NRose : Type where { NLeaf : NRose ; NNode : List (Pair String NRose) -> NRose }

fn add (x : Nat) (y : Nat) : Nat = match x { Zero |-> y ; Suc x2 |-> Suc (add x2 y) }

fn nsize (r : NRose) : Nat = match r {
  NLeaf |-> Suc Zero;
  NNode members |-> match members {
    Nil |-> Zero;
    Cons member rest |-> recursive result for rest
  }
}

const two_leaves : NRose =
  NNode (Cons (Pair String NRose) (mk_pair String NRose "x" NLeaf)
    (Cons (Pair String NRose) (mk_pair String NRose "y" NLeaf) (Nil (Pair String NRose))))

fn exactNatTwo (value : Nat) : ExitCode = match value {
  Zero |-> Failure 20;
  Suc one |-> match one {
    Zero |-> Failure 21;
    Suc two |-> match two {
      Zero |-> Success;
      Suc _ |-> Failure 24
    }
  }
}

fn main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode =
  host_exit APartial (exactNatTwo (nsize two_leaves))
"#;

fn output_dir() -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix("ken-lang-nested-former-native-")
        .tempdir()
        .unwrap()
}

fn native_exit(source: &str, name: &str) -> i32 {
    let root = output_dir();
    let output =
        ken_cli::build_native_program(source, ken_cli::SourceFormat::Ken, name, root.path())
            .unwrap_or_else(|e| panic!("{name}: nested-former fold must reach a native artifact: {e}"));
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .unwrap_or_else(|e| panic!("{name}: linked native artifact must run: {e:?}"));
    // A `Failure N` host_exit is a NORMAL completion returning a non-zero code
    // (terminal_exit = ReturnedError, not an abnormal trap); only terminal_error
    // signals an abnormal native run. Compare the returned exit code, not the
    // exit CLASS, so both the Success (0) and Failure (20) programs are accepted.
    assert_eq!(
        native.terminal_error, None,
        "{name}: native run must complete without an abnormal terminal error"
    );
    native.exit_status
}

fn interp_exit(source: &str) -> i32 {
    let mut host = ken_interp::CaptureHost::new(Vec::new());
    ken_cli::run_program_effect_observation(
        source,
        ken_cli::SourceFormat::Ken,
        &[b"ken".to_vec()],
        &[],
        b"/",
        &mut host,
    )
    .expect("nested-former fold runs through the interpreter")
    .exit_status
}

/// Promise class: durable invariant (AC-EXECUTES-AT-RUNTIME, native half).
///
/// MEASURED: the `List (Pair String Self)` fold this WP newly admits compiles to
/// a linked native artifact, runs, and its process exit (0 = Nat 2 via the
/// injective classifier) equals the interpreter's; a source mutation dropping the
/// nested-IH consumption changes the observed native exit to 20 (Nat 0). CLAIMED:
/// the generated nested induction hypothesis is consumed at RUNTIME on the native
/// path, in agreement with the interpreter. THE GAP: the opaque Nat is observed
/// through an injective exit classifier rather than exported directly.
#[test]
fn nested_former_fold_executes_natively_in_agreement_with_the_interpreter() {
    let native = native_exit(NESTED_TWO_LEAVES, "nested-former-native-two-leaves");
    let interpreted = interp_exit(NESTED_TWO_LEAVES);
    assert_eq!(interpreted, 0, "interpreter did not observe Nat 2");
    assert_eq!(native, 0, "native did not observe Nat 2");
    assert_eq!(native, interpreted, "native and interpreter results diverged");

    // Reaching mutation: dropping `recursive result for member` loses the
    // Pair-nested leaf count, so the same native pipeline observes Nat 0 -> exit
    // 20. Native and interpreter still agree on the mutated value.
    let native_dropped = native_exit(NESTED_DROPPED_IH, "nested-former-native-dropped-ih");
    let interpreted_dropped = interp_exit(NESTED_DROPPED_IH);
    assert_eq!(
        native_dropped, 20,
        "dropping the nested-IH consumption must change the observed native result"
    );
    assert_eq!(
        native_dropped, interpreted_dropped,
        "native and interpreter must agree on the mutated value too"
    );
    assert_ne!(
        native, native_dropped,
        "the nested-IH consumption must be what carries the native runtime result"
    );
}
