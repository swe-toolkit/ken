//! `LANG-CHECKED-IH-BODY-VIEW-CAUSE` — the D5 two-recursive-position inorder probe.
//!
//! MEASUREMENT PROBE, not a fix. It reconstructs the traversal whose native
//! lowering the runtime ring measured as a side finding of
//! `RT-DESCENT-LANE-COMPLETENESS` `D5`: an ordinary inorder fold over the
//! two-recursive-position `D5Tree` (`D5Node` carries two `D5Tree` fields),
//! recursing DIRECTLY in both positions (`inorder l` / `inorder r`), package
//! `d5-two-recursive-position`, symbol `inorder` — the exact symbol path
//! measured as failing at `331db0a73` with the opaque `MissingClosureMetadata`.
//!
//! OUTCOME (measured at `d70db3299`): it COMPILES to a native artifact and
//! RUNS. Per the D0 procedure (Architect, via language-leader) a `compiles`
//! outcome dissolves D0, so this node closes with no census-site change. This
//! test is the durable regression guard.
//!
//! EXECUTION ORACLE (QA-required, discriminating): the program observes the
//! inorder sum AS the process exit code, through an injective `classify` that
//! maps each small `Nat` to a distinct `Failure <n>` (`Failure` takes a
//! `UInt8`, and there is no `Nat -> UInt8` arithmetic in the prelude, so the
//! observation is a structural classifier, not a computed code). Over the tree
//! `{1,2,3}` the complete fold observes 6, and each way of dropping a recursive
//! contribution lands a DISTINCT exit: right-call deletion (`add (inorder l) v`,
//! left-only) observes 3, left-call deletion (`add v (inorder r)`, right-only)
//! observes 5, both dropped (`v`, root-only) observes 2. A Zero/nonzero
//! classifier would collapse all four to one code and fail to red under the
//! mutation; keying on the value does not.
//!
//! Scope is test-only: it does not fix any traversal, widen SCT, name a class,
//! or touch the surviving lesser `compiler_driver.rs:2052` census catch-all.

#![cfg(target_os = "linux")]

// The inorder body arm is substituted for `__ARM__`; `classify` maps the small
// Nat sums this test produces (2..6) to `Failure <n>`, so the exit code equals
// the observed sum and discriminates every recursive position.
const D5_TEMPLATE: &str = r#"program capabilities FS APartial

data D5Tree : Type where { D5Leaf : D5Tree ; D5Node : D5Tree -> Nat -> D5Tree -> D5Tree }

fn add (x : Nat) (y : Nat) : Nat = match x { Zero |-> y ; Suc x2 |-> Suc (add x2 y) }

fn classify (n : Nat) : ExitCode = match n {
  Zero |-> Failure 100;
  Suc a |-> match a {
    Zero |-> Failure 1;
    Suc b |-> match b {
      Zero |-> Failure 2;
      Suc c |-> match c {
        Zero |-> Failure 3;
        Suc d |-> match d {
          Zero |-> Failure 4;
          Suc e |-> match e {
            Zero |-> Failure 5;
            Suc f |-> match f {
              Zero |-> Failure 6;
              Suc _ |-> Failure 200
            }
          }
        }
      }
    }
  }
}

fn inorder (t : D5Tree) : Nat = match t {
  D5Leaf |-> Zero;
  D5Node l v r |-> __ARM__
}

const d5_tree : D5Tree =
  D5Node (D5Node D5Leaf (Suc Zero) D5Leaf) (Suc (Suc Zero))
    (D5Node D5Leaf (Suc (Suc (Suc Zero))) D5Leaf)

fn main (_input : ProcessInput) (_caps : ProgramCaps APartial)
  : HostIO APartial ExitCode =
  host_exit APartial (classify (inorder d5_tree))
"#;

fn d5_source(inorder_arm: &str) -> String {
    D5_TEMPLATE.replace("__ARM__", inorder_arm)
}

fn native_exit(source: &str, package: &str) -> i32 {
    let root = tempfile::tempdir().expect("temporary native-build root");
    let output = ken_cli::build_native_program(source, ken_cli::SourceFormat::Ken, package, root.path())
        .unwrap_or_else(|e| {
            panic!("{package}: two-recursive-position traversal must reach a native artifact: {e:#?}")
        });
    let native = ken_runtime::run_bound_process_effect_observation(
        &output.artifact,
        &ken_runtime::NativeEffectRunOptionsV1 {
            arguments: Vec::new(),
            environment: Vec::new(),
            cwd: root.path().to_owned(),
            plan_hash: output.plan_transport_hash,
        },
    )
    .unwrap_or_else(|e| panic!("{package}: linked native artifact must run: {e:?}"));
    assert_eq!(
        native.terminal_error, None,
        "{package}: native run must complete without an abnormal terminal error"
    );
    native.exit_status
}

/// The headline plus the QA-required discriminating mutation proof, in one pass
/// so the four artifacts are built once. The two-recursive-position `inorder`
/// traversal compiles AND runs, and its observed exit VALUE (not a Zero/nonzero
/// class) keys on the complete sum: dropping either recursive call, or both,
/// changes the exit, so the `== 6` oracle reds under the right-call deletion QA
/// flagged. Promise class: durable invariant.
#[test]
fn d5_inorder_compiles_runs_and_its_exit_value_discriminates_every_recursive_position() {
    // Complete inorder: recurse in BOTH positions. Sum over {1,2,3} = 6.
    let complete = native_exit(
        &d5_source("add (inorder l) (add v (inorder r))"),
        "d5-two-recursive-position",
    );
    // Right-call deletion (QA's mutation): left-only fold = 3.
    let drop_right = native_exit(
        &d5_source("add (inorder l) v"),
        "d5-two-recursive-position-drop-right",
    );
    // Left-call deletion: right-only fold = 5.
    let drop_left = native_exit(
        &d5_source("add v (inorder r)"),
        "d5-two-recursive-position-drop-left",
    );
    // Both dropped: root-only = 2.
    let root_only = native_exit(&d5_source("v"), "d5-two-recursive-position-root-only");

    eprintln!(
        "D5_INORDER_EXITS: complete={complete} drop_right={drop_right} drop_left={drop_left} root_only={root_only}"
    );

    // The guard: the complete two-position inorder fold over {1,2,3} observes 6.
    assert_eq!(complete, 6, "the complete two-position inorder fold must observe the full sum 6");

    // The mutation QA flagged: deleting the RIGHT recursive call observes 3, so
    // an oracle keyed on the value reds (3 != 6). A Zero/nonzero classifier
    // would have passed here — that was the blocked candidate's defect.
    assert_ne!(
        drop_right, complete,
        "deleting the right recursive call must change the observed exit (mutation must red the ==6 oracle)"
    );

    // Full discrimination across every way of dropping a recursive contribution.
    assert_eq!(
        (complete, drop_right, drop_left, root_only),
        (6, 3, 5, 2),
        "the exit-value oracle must map complete/left-only/right-only/root-only to distinct 6/3/5/2"
    );
}
