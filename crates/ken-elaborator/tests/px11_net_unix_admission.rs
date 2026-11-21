//! Spec 38 §1.13: a declared Net Unix grant requires FS AFull at checked
//! program admission, independently of the Net class parser.

use ken_elaborator::program_admission::{admit_checked_main, ProgramAdmissionError};
use ken_elaborator::ElabEnv;

#[path = "support/catalog_roots.rs"]
mod catalog_roots;

fn checked_unix_program(authority: &str) -> ElabEnv {
    let source = format!(
        "program capabilities FS {authority}, Net Unix\n\
         import Capability.Process.Exit (host_pure)\n\
         proc main (_input : ProcessInput) (_caps : ProgramCaps {authority})\n\
           : HostIO {authority} ExitCode visits [Net] =\n\
           host_pure {authority} ExitCode ExitCode.Success\n"
    );
    let mut env = ElabEnv::new().expect("prelude");
    catalog_roots::load(&mut env, "Capability.Process.Exit");
    env.elaborate_file(&source)
        .expect("all three authority variants elaborate a checked main");
    env
}

/// Promise class: durable invariant. MEASURED: a checked program with Net
/// Unix and FS ANone is refused by the dedicated admission arm, not by the
/// parser or the main ABI. CLAIMED: Unix authority requires FS AFull.
/// THE GAP: this checks admission, not a live Unix socket system call.
#[test]
fn px11_net_unix_fs_anone_is_refused_at_program_admission() {
    let env = checked_unix_program("ANone");
    assert_eq!(
        admit_checked_main(&env).unwrap_err(),
        ProgramAdmissionError::NetUnixRequiresFullFs,
    );
}

/// Promise class: durable invariant. MEASURED: a checked program with Net
/// Unix and FS APartial passes parsing and elaboration, then the dedicated
/// admission arm rejects it. CLAIMED: even the nearest valid authority below
/// AFull cannot authorize a Unix socket. THE GAP: the test exercises checked
/// admission, not the host's filesystem or socket syscall.
#[test]
fn px11_net_unix_fs_apartial_is_refused_at_program_admission() {
    let env = checked_unix_program("APartial");
    assert_eq!(
        admit_checked_main(&env).unwrap_err(),
        ProgramAdmissionError::NetUnixRequiresFullFs,
    );
}

/// Promise class: durable invariant. MEASURED: the same checked main with FS
/// AFull passes admission and retains the declared Unix grant. CLAIMED: Unix
/// is admitted when FS has full authority. THE GAP: real socket access still
/// requires a Net Unix grant at host dispatch and an OS-available path.
#[test]
fn px11_net_unix_fs_afull_is_admitted() {
    let env = checked_unix_program("AFull");
    let checked = admit_checked_main(&env).expect("FS AFull allows Net Unix");
    assert_eq!(checked.authority_name, "AFull");
    assert_eq!(checked.net_classes, [ken_host::NetClassV1::Unix]);
}
