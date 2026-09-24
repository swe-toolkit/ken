//! K3: one checked String literal drives the kernel view, interpreter,
//! and native Char-code observation. The generic proof client is tested in
//! ken-elaborator/tests/k3_literal_char_view.rs.

use std::path::PathBuf;
use std::process::Command;

const PROGRAM: &str = r#"program capabilities FS APartial
fn codes_correct (cs : List Char) : Bool =
  match cs {
    Nil |-> False;
    Cons first rest |->
      match eq_int (charToInt first) 65 {
        False |-> False;
        True |-> match rest {
          Nil |-> False;
          Cons second tail |-> match eq_int (charToInt second) 122 {
            False |-> False;
            True |-> match tail { Nil |-> True; Cons _ _ |-> False }
          }
        }
      }
  }

proc main (_input : ProcessInput) (_caps : ProgramCaps APartial) : HostIO APartial ExitCode visits [Console] =
  host_program APartial (print_line (match codes_correct (string_to_list_char "Az") {
    True |-> match eq_int (charToInt 'A') 65 {
      True |-> "codes:Az";
      False |-> "wrong-codes"
    };
    False |-> "wrong-codes"
  }))
"#;

#[test]
fn checked_literal_view_codes_match_interpreter_and_native() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("k3_literal_char_native");
    std::fs::create_dir_all(&dir).expect("native output directory");
    let path = dir.join("program.ken");
    std::fs::write(&path, PROGRAM).expect("write identical checked source");
    let interpreted = Command::new(env!("CARGO_BIN_EXE_ken"))
        .arg("run")
        .arg(&path)
        .output()
        .expect("reference interpreter runs");
    assert!(
        interpreted.status.success(),
        "interpreter stderr: {}",
        String::from_utf8_lossy(&interpreted.stderr)
    );
    assert_eq!(interpreted.stdout, b"codes:Az\n");

    let built = ken_cli::build_native_program(
        PROGRAM,
        ken_cli::SourceFormat::Ken,
        "k3-literal-char-native",
        &dir.join("native"),
        ken_runtime::boundary_resource_profile::starter_smoke_profile(),
    )
    .expect("the identical checked program native-builds");
    let native = Command::new(&built.artifact.executable_path)
        .output()
        .expect("native executable runs");
    assert!(
        native.status.success(),
        "native stderr: {}",
        String::from_utf8_lossy(&native.stderr)
    );
    assert_eq!(
        native.stdout, interpreted.stdout,
        "native view/code output must match interpreter and kernel's checked 65/122 codes"
    );
}
