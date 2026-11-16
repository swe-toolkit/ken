//! Roots-load an owner's checked catalog unit and execute its checked fences.

use ken_elaborator::ElabEnv;
use std::path::PathBuf;

fn roots() -> Vec<PathBuf> {
    vec![PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")]
}

pub fn load_owner(env: &mut ElabEnv, module: &str) {
    env.elaborate_module_from_roots(&roots(), module)
        .unwrap_or_else(|error| panic!("{module} checked owner must roots-load: {error:?}"));
    env.execute_loaded_entry_checked_fences(module)
        .unwrap_or_else(|error| panic!("{module} checked fences must execute: {error:?}"));
}
