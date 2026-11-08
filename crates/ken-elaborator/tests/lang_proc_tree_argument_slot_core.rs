//! Spec 36 §2.4, SLOTS Increment 2. Promise class: durable invariant.
//! MEASURED: checked bodies of two direct argument slots and their explicit
//! left-to-right let twins are exactly equal in one elaboration environment.
//! CLAIMED: a performed argument is sequenced through the same checked bind
//! and latent bookkeeping as an explicit let, without moving pure arguments.
//! THE GAP: one concrete two-action and one mixed pure/action application
//! exercise this identity; distinct literal occurrences have opaque checked
//! global identities, so both twins reference the same declared messages.

use ken_elaborator::ElabEnv;
use ken_kernel::Decl;

const PRE: &str = r#"program capabilities FS APartial
const first : String = "1"
const second : String = "2"
proc emit (msg : String) : Unit visits [Console] =
  let r = print_line msg in MkUnit
fn two (x : Unit) (y : Unit) : Unit = MkUnit
"#;

fn compare_bodies(label: &str, definitions: &str) {
    let mut env = ElabEnv::new().expect("checked prelude");
    env.elaborate_file(&format!("{PRE}{definitions}"))
        .unwrap_or_else(|error| panic!("{label}: elaboration failed: {error:?}"));
    let body = |name: &str| match env.env.lookup(env.globals[name]) {
        Some(Decl::Transparent { body, .. }) => body.clone(),
        other => panic!("{label}: {name} not a checked definition: {other:?}"),
    };
    let direct = body("direct");
    let explicit = body("explicit");
    assert_eq!(
        direct, explicit,
        "{label}: direct argument vs explicit let core"
    );
}

#[test]
fn two_performed_argument_slots_have_exact_explicit_let_core() {
    compare_bodies(
        "two action slots",
        r#"proc direct (u : Unit) : Unit visits [Console] =
  two (emit first) (emit second)
proc explicit (u : Unit) : Unit visits [Console] =
  let x1 = emit first in let x2 = emit second in two x1 x2
"#,
    );
}

#[test]
fn pure_argument_stays_at_its_original_slot() {
    compare_bodies(
        "pure first, action second",
        r#"proc direct (u : Unit) : Unit visits [Console] =
  two u (emit second)
proc explicit (u : Unit) : Unit visits [Console] =
  let x2 = emit second in two u x2
"#,
    );
}
