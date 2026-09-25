//! CAT-CAPABILITY-LAWS-PRELUDE-MOVE: local checked owners, not prelude aliases.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ken_elaborator::ElabEnv;
use ken_kernel::Decl;

const MOVED_AND_RETIRED: [&str; 7] = [
    "transfer_count_request_budget",
    "write_all_complete",
    "write_all_call_bound",
    "write_all_first_error",
    "write_all_all_success",
    "buffer_nat_add",
    "buffer_suc_cong",
];

fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

fn without_prelude_copies() -> ElabEnv {
    let mut env = ElabEnv::new().expect("fresh compiler base");
    for name in MOVED_AND_RETIRED {
        assert!(
            env.globals.remove(name).is_some(),
            "the prelude {name} must exist until its L2 flip"
        );
    }
    env
}

/// Promise class: transition sentinel. Retire or rebaseline when L2 removes
/// the seven prelude registrations. MEASURED: separately roots-loaded Buffer
/// and IO check after all seven unqualified copies are withheld. Every moved
/// function and its attached proof has a transparent module-qualified identity
/// in its consumer, and the cold trusted set equals an independent compiler
/// baseline. CLAIMED: the consumer laws use checked local owners without
/// prelude fall-through or new trust. THE GAP: this pins name resolution and
/// kernel checking, not the runtime buffer and IO boundary properties.
#[test]
fn capability_laws_have_checked_local_owners_without_prelude_fallthrough() {
    let compiler = ElabEnv::new().expect("independent compiler base");
    let baseline: BTreeSet<_> = compiler.env.trusted_base().into_iter().collect();

    for (module, attached) in [
        (
            "Capability.System.Buffer",
            &[("transfer_count_request_budget", "bounded")][..],
        ),
        (
            "Capability.System.IO",
            &[
                ("write_all_call_bound", "termination"),
                ("write_all_complete", "success_complete"),
                ("write_all_first_error", "first_error"),
                ("write_all_all_success", "all_success"),
            ][..],
        ),
    ] {
        let mut env = without_prelude_copies();
        env.elaborate_module_from_roots(&[catalog_root()], module)
            .unwrap_or_else(|error| {
                panic!("{module} must check without prelude copies: {error:?}")
            });
        for (function, proof) in attached {
            for name in [(*function).to_owned(), format!("{function}::{proof}")] {
                let qualified = format!("{module}.{name}");
                let id = *env
                    .globals
                    .get(&qualified)
                    .unwrap_or_else(|| panic!("{qualified} must resolve locally"));
                assert!(
                    matches!(env.env.lookup(id), Some(Decl::Transparent { .. })),
                    "{qualified} must have a checked transparent body"
                );
            }
        }
        let loaded: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
        assert_eq!(
            loaded, baseline,
            "{module} must add no trusted declarations"
        );
    }
}
