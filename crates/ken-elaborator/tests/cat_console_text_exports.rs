//! Console.Text's exported helpers and checked laws, through a client module.
//!
//! Durable invariant: selective imports, effectful calls, and independently
//! restated public law types elaborate through the real roots-based loader.

use ken_elaborator::ElabEnv;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);

struct ClientRoot(PathBuf);

impl ClientRoot {
    fn new(client: &str) -> Self {
        let serial = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ken-console-text-exports-{}-{serial}",
            std::process::id()
        ));
        let provider = root.join("Capability/Console");
        fs::create_dir_all(&provider).expect("create client catalog root");
        fs::copy(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../catalog/packages/Capability/Console/Text.ken.md"),
            provider.join("Text.ken.md"),
        )
        .expect("load the current canonical Console.Text package under the client root");
        fs::write(root.join("Entry.ken"), client).expect("write client module");
        Self(root)
    }
}

impl Drop for ClientRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

const COMPLETE_CLIENT: &str = r#"
import Capability.Console.Text
  (print, printLine, eprint, eprintLine,
   print_write_tree, print_line_write_tree,
   eprint_write_tree, eprint_line_write_tree, console_line_payload)

proc client_print (text : String) : IO (Result IOError Unit) visits [Console] =
  print text
proc client_print_line (text : String) : IO (Result IOError Unit) visits [Console] =
  printLine text
proc client_eprint (text : String) : IO (Result IOError Unit) visits [Console] =
  eprint text
proc client_eprint_line (text : String) : IO (Result IOError Unit) visits [Console] =
  eprintLine text

fn client_payload (text : String) : Bytes = console_line_payload text

theorem client_print_write_tree (text : String)
  : Equal (IO (Result IOError Unit)) (print text) (write Stdout (bytes_encode text)) =
  print_write_tree text

theorem client_print_line_write_tree (text : String)
  : Equal (IO (Result IOError Unit))
      (printLine text) (write Stdout (console_line_payload text)) =
  print_line_write_tree text

theorem client_eprint_write_tree (text : String)
  : Equal (IO (Result IOError Unit)) (eprint text) (write Stderr (bytes_encode text)) =
  eprint_write_tree text

theorem client_eprint_line_write_tree (text : String)
  : Equal (IO (Result IOError Unit))
      (eprintLine text) (write Stderr (console_line_payload text)) =
  eprint_line_write_tree text
"#;

const STDOUT_LINE_CLIENT: &str = r#"
import Capability.Console.Text (printLine, print_line_write_tree, console_line_payload)
theorem client_stdout_line (text : String)
  : Equal (IO (Result IOError Unit))
      (printLine text) (write Stdout (console_line_payload text)) =
  print_line_write_tree text
"#;

const STDERR_LINE_CLIENT: &str = r#"
import Capability.Console.Text (eprintLine, eprint_line_write_tree, console_line_payload)
theorem client_stderr_line (text : String)
  : Equal (IO (Result IOError Unit))
      (eprintLine text) (write Stderr (console_line_payload text)) =
  eprint_line_write_tree text
"#;

fn check_client(client: &str) {
    let root = ClientRoot::new(client);
    let mut env = ElabEnv::empty().expect("checked prelude");
    let trusted_before = env.env.trusted_base().into_iter().collect::<BTreeSet<_>>();
    env.elaborate_module_from_roots(&[root.0.clone()], "Entry")
        .expect("client must resolve only public Console.Text exports through module loader");
    assert_eq!(
        trusted_before,
        env.env.trusted_base().into_iter().collect::<BTreeSet<_>>(),
        "neither provider nor client may extend the trusted base"
    );
}

#[test]
fn all_public_console_helpers_and_laws_are_usable_from_client() {
    check_client(COMPLETE_CLIENT);
}

#[test]
fn stdout_line_law_type_is_public_to_selective_client() {
    check_client(STDOUT_LINE_CLIENT);
}

#[test]
fn stderr_line_law_type_is_public_to_selective_client() {
    check_client(STDERR_LINE_CLIENT);
}
