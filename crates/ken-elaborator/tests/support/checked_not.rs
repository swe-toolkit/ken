//! Checked `Not` provider for test clients retaining the pre-source spelling.

use ken_elaborator::ElabEnv;
use std::path::PathBuf;

pub fn roots() -> Vec<PathBuf> {
    vec![PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")]
}

pub fn load(env: &mut ElabEnv) {
    env.elaborate_module_from_roots(&roots(), "Core.Logic.Not")
        .expect("checked Not provider roots-loads");
}
