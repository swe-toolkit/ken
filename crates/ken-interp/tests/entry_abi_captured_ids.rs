//! Console, Clock and FS host ABI constructors come from the pre-source
//! checked prelude, not the mutable source-spelling table. Contract: spec 42
//! §6 and 62 §3.2; VERIFY-ENTRY-ABI-CHECKED-IDENTITY D0.

use ken_elaborator::ElabEnv;
use ken_interp::{ClockIds, ConsoleIds, FSIds};

/// Promise class: durable invariant. MEASURED: each of three independent
/// harvester routes still reports the original prelude GlobalId when a lawful
/// top-level source const redirects its spelling to a distinct ID. CLAIMED:
/// interpreter effect dispatch cannot be redirected by a source declaration.
/// THE GAP: this probes one role per table; each harvester's `&PreludeEnv`
/// input makes a further mutable-source lookup unnameable at that seam.
#[test]
fn effect_harvesters_keep_checked_roles_after_three_source_shadows() {
    let mut env = ElabEnv::new().expect("checked prelude");
    let before_console = ConsoleIds::from_elab(&env.prelude_env).expect("Console ABI");
    let before_clock = ClockIds::from_elab(&env.prelude_env).expect("Clock ABI");
    let before_fs = FSIds::from_elab(&env.prelude_env).expect("FS ABI");
    env.elaborate_file_v1(
        "const Write : Nat = Zero\nconst WallNow : Nat = Zero\nconst ReadFile : Nat = Zero",
    )
    .expect("user const declarations may shadow prelude operation spellings");
    assert_ne!(env.globals["Write"], before_console.write_id);
    assert_ne!(env.globals["WallNow"], before_clock.wall_now_id);
    assert_ne!(env.globals["ReadFile"], before_fs.readfile_id);
    let console = ConsoleIds::from_elab(&env.prelude_env).expect("Console ABI after source");
    let clock = ClockIds::from_elab(&env.prelude_env).expect("Clock ABI after source");
    let fs = FSIds::from_elab(&env.prelude_env).expect("FS ABI after source");
    assert_eq!(console.write_id, before_console.write_id);
    assert_eq!(clock.wall_now_id, before_clock.wall_now_id);
    assert_eq!(fs.readfile_id, before_fs.readfile_id);
}
