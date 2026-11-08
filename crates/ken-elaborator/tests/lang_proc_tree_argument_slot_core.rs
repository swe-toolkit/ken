//! Spec 36 §2.4, SLOTS Increment 2. Promise class: durable invariant.
//! MEASURED: checked bodies of two direct argument slots and their explicit
//! left-to-right let twins are exactly equal in one elaboration environment.
//! CLAIMED: a performed argument is sequenced through the same checked bind
//! and latent bookkeeping as an explicit let, without moving pure or held-tree
//! arguments. THE GAP: two-action, mixed pure/action, and held-tree-first
//! applications exercise this identity; literal occurrences have opaque checked
//! global identities, so both twins reference the same declared messages.

use ken_elaborator::{render_open_obligations, ElabEnv, Span};
use ken_kernel::{Decl, Term};

const PRE: &str = r#"program capabilities FS APartial
const first : String = "1"
const second : String = "2"
const held : String = "held"
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

#[test]
fn held_tree_first_and_performed_value_second_have_exact_let_core() {
    compare_bodies(
        "held tree, performed value",
        r#"fn take (tree : HostIO APartial (Result IOError Unit)) (unit : Unit) : Unit = unit
proc direct (u : Unit) : Unit visits [Console] =
  take (host_console APartial (Result IOError Unit)
    (write ConsoleStream.Stdout (bytes_encode held))) (emit first)
proc explicit (u : Unit) : Unit visits [Console] =
  let x = emit first in take (host_console APartial (Result IOError Unit)
    (write ConsoleStream.Stdout (bytes_encode held))) x
"#,
    );
}

/// Promise class: durable invariant.
/// MEASURED: a pure first argument opens one Requires hole in both the direct
/// performed-second call and its explicit-let twin; their checked cores agree.
/// CLAIMED: the argument-domain type probe does not publish obligations.
/// THE GAP: the fixture covers an earlier pure argument with a Requires hole,
/// not every kind of instance or meta resolved while probing a prefix.
#[test]
fn prefix_probe_discards_earlier_argument_obligation() {
    fn elaborate(body: &str) -> (Term, Vec<String>) {
        let mut env = ElabEnv::new().expect("checked prelude");
        let results = env
            .elaborate_file_v1(&format!(
                "{PRE}const zero : Int = 0\n\
                 fn need (n : Int) : Unit requires Equal Int n 5 = MkUnit\n\
                 proc probe_user (u : Unit) : Unit visits [Console] =\n  {body}\n"
            ))
            .expect("open precondition in a pure earlier argument");
        let result = results
            .iter()
            .find(|result| result.name == "probe_user")
            .expect("checked probe result");
        // Each source puts the same call at a different byte offset. Compare
        // rendered obligations without that incidental source coordinate.
        let mut canonical = result.clone();
        for obligation in &mut canonical.obligations {
            obligation.span = Span::zero();
        }
        let reports = render_open_obligations(&[canonical]);
        let core = match env.env.lookup(result.def_id) {
            Some(Decl::Transparent { body, .. }) => body.clone(),
            other => panic!("probe_user not a checked definition: {other:?}"),
        };
        (core, reports)
    }

    let (direct_core, direct_open) = elaborate("two (need zero) (emit first)");
    let (explicit_core, explicit_open) =
        elaborate("let action = emit first in two (need zero) action");
    assert_eq!(direct_open, explicit_open, "probe must not add open holes");
    assert_eq!(direct_open.len(), 1, "the earlier argument opens one hole");
    assert!(
        direct_open[0].starts_with("unknown probe_user.requires."),
        "{direct_open:?}"
    );
    assert_eq!(direct_core, explicit_core, "direct and let checked bodies");
}
