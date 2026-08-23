//! `V3-FO-SOUNDNESS-SCT-EXPRESSIBILITY`: the real FoKripke soundness proof
//! passes full admission under the unchanged structural size relation.

#[path = "support/catalog_or.rs"]
mod catalog_or;

use std::collections::BTreeSet;

use ken_elaborator::ElabEnv;

const FOK_SOURCE: &str =
    include_str!("../../../catalog/packages/Tooling/Verification/FoKripke.ken");

#[test]
fn real_fok_checker_soundness_passes_full_admission_without_new_trust() {
    let mut env = ElabEnv::new().expect("base environment");
    catalog_or::load_core_logic_or(&mut env);
    let before: BTreeSet<_> = env.env.trusted_base().into_iter().collect();

    env.elaborate_file(FOK_SOURCE).expect(
        "the real FoKripke checker_soundness proof must elaborate, kernel-check, and pass SCT",
    );

    let after: BTreeSet<_> = env.env.trusted_base().into_iter().collect();
    assert_eq!(before, after, "the structural proof must add no trust");
}
