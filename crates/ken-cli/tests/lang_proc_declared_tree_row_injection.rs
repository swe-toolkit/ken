//! Spec 36 §2.5: every label in a declared tree result's concrete row
//! injects into the checked tree signature. Promise class: durable invariant.
//! MEASURED: each mismatched label is refused by name and a matched sibling
//! is admitted, across ambient, host, alias, arrow-final and local signatures.
//! CLAIMED: row labels cannot promise effects absent from the declared tree.
//! THE GAP: these cases do not enumerate every host op; the source classifier
//! matches HostOpV1 exhaustively, and the separate prelude pin covers the
//! compiler-owned declarations before signature ids are captured.

use ken_elaborator::{ElabEnv, ElabError};

fn accepts(case: &str, source: &str) {
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_file(source)
        .unwrap_or_else(|error| panic!("{case}: expected accepted declaration: {error:?}"));
}

fn refuses_label(case: &str, source: &str, label: &str) {
    let mut env = ElabEnv::new().expect("checked prelude");
    let error = match env.elaborate_file(source) {
        Ok(_) => panic!("{case}: label `{label}` was accepted"),
        Err(error) => error,
    };
    assert!(
        matches!(&error, ElabError::TypeMismatch { reason, .. }
            if reason.contains(&format!("row label `{label}` has no summand"))
                && reason.contains("36 §2.5")),
        "{case}: expected declared-tree injection refusal for `{label}`, got {error:?}"
    );
}

/// 36 §2.5: distinct written Cap indices matter only if this declaration
/// chooses an FS signature for a non-tree final result. These four controls
/// are independently named so a premature comparison cannot hide behind a
/// prior failure. MEASURED: each checks against the same checked prelude.
/// CLAIMED: unused Caps cannot restrict ambient or declared-tree results.
/// THE GAP: P5 in lang_proc_fs_row_and_arrow guards the non-tree FS refusal.
#[test]
fn auth_index_a_two_caps_ambient_console_is_accepted() {
    accepts(
        "A ambient Console",
        "proc p (src : Cap APartial) (dst : Cap AFull) : Nat visits [Console] = Suc Zero",
    );
}

#[test]
fn auth_index_b_two_caps_declared_host_tree_is_accepted() {
    accepts(
        "B declared HostIO",
        "proc copy (src : Cap AFull) (dst : Cap APartial) : HostIO AFull Unit \
         visits [FS, Console] = host_pure AFull Unit MkUnit",
    );
}

#[test]
fn auth_index_c_two_caps_declared_fs_tree_is_accepted() {
    accepts(
        "C declared FS",
        "proc copy (src : Cap AFull) (dst : Cap APartial) : FS AFull Unit \
         visits [FS] = ITree.Ret (FSOp AFull) (fs_resp AFull) Unit MkUnit",
    );
}

#[test]
fn auth_index_d_two_caps_ambient_clock_is_accepted() {
    accepts(
        "D ambient Clock",
        "proc p (src : Cap APartial) (dst : Cap AFull) : Nat visits [Clock] = Suc Zero",
    );
}

#[test]
fn r1_clock_cannot_inject_into_console_tree() {
    accepts(
        "R1 Console",
        "proc p : IO Unit visits [Console] = print_line \"x\"",
    );
    refuses_label(
        "R1 Clock",
        "proc q : IO Unit visits [Console, Clock] = print_line \"x\"",
        "Clock",
    );
}

const HOST_RET: &str = "ITree.Ret (Coproduct (FSOp APartial) AmbientOp) \
    (resp_coproduct (FSOp APartial) AmbientOp (fs_resp APartial) ambient_resp) Unit MkUnit";

#[test]
fn r2_host_operation_labels_inject_only_into_their_family() {
    for (case, row) in [("R2 host family", "FS, FsOpen"), ("R2 FS", "FS")] {
        accepts(
            case,
            &format!("proc h : HostIO APartial Unit visits [{row}] = {HOST_RET}"),
        );
    }
    refuses_label(
        "R2 FsOpen in IO",
        "proc i : IO Unit visits [Console, FsOpen] = print_line \"x\"",
        "FsOpen",
    );
}

#[test]
fn r3_transparent_tree_head_is_classified_by_checked_identity() {
    accepts(
        "R3 alias Console",
        "def ConsoleTree = IO Unit\nproc p : ConsoleTree visits [Console] = print_line \"x\"",
    );
    refuses_label(
        "R3 alias Clock",
        "def ConsoleTree = IO Unit\nproc p : ConsoleTree visits [Console, Clock] = print_line \"x\"",
        "Clock",
    );
}

#[test]
fn r4_fs_tree_does_not_admit_an_ambient_label() {
    accepts(
        "R4 FS and FsOpen",
        "proc r (c : Cap AFull) : FS AFull (Result FileError Bytes) \
         visits [FS, FsOpen] = read_bytes AFull c (bytes_encode \"x\")",
    );
    refuses_label(
        "R4 Console in FS",
        "proc r (c : Cap AFull) : FS AFull (Result FileError Bytes) \
         visits [FS, Console] = read_bytes AFull c (bytes_encode \"x\")",
        "Console",
    );
}

#[test]
fn r5_tree_final_arrow_checks_the_signature_at_the_leaf() {
    accepts(
        "R5 arrow Console",
        "proc tf : Unit -> IO Unit visits [Console] = \\_. print_line \"x\"",
    );
    refuses_label(
        "R5 arrow Clock",
        "proc tf : Unit -> IO Unit visits [Console, Clock] = \\_. print_line \"x\"",
        "Clock",
    );
}

const LOCAL_PING: &str = r#"
data LocalOp = Ping | Pong

fn local_resp (_op : LocalOp) : Type = Unit

proc target (_value : Unit)
  : ITree LocalOp local_resp Unit visits [Local] =
  ITree.Vis LocalOp local_resp Unit Ping
    (\_response. ITree.Ret LocalOp local_resp Unit MkUnit)
"#;

#[test]
fn r6a_family_label_injects_into_its_own_tree() {
    // Identical to the b1 Ping source: the row is required by L5 export.
    accepts("R6 own family", LOCAL_PING);
}

#[test]
fn r6b_unrelated_label_cannot_inject_into_user_tree() {
    let wrong_label = LOCAL_PING.replacen("visits [Local]", "visits [Other]", 1);
    assert_ne!(wrong_label, LOCAL_PING, "R6 must move the row label");
    refuses_label("R6 other family", &wrong_label, "Other");
}

#[test]
fn r6c_local_label_cannot_inject_into_host_tree() {
    refuses_label(
        "R6 user label in HostIO",
        "proc target (_value : Unit) : HostIO AFull (Result IOError Unit) \
         visits [Console, Local] = host_console AFull (Result IOError Unit) \
         (flush ConsoleStream.Stdout)",
        "Local",
    );
}
