//! Select checked catalog providers for test-owned in-memory Ken clients.

use ken_elaborator::ElabEnv;
use std::path::PathBuf;

pub fn roots() -> Vec<PathBuf> {
    vec![PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")]
}

pub fn load(env: &mut ElabEnv, module: &str) {
    env.elaborate_module_from_roots(&roots(), module)
        .unwrap_or_else(|error| panic!("{module} checked provider must roots-load: {error:?}"));
}
