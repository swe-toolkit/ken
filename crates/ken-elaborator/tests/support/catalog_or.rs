use std::path::PathBuf;

use ken_elaborator::ElabEnv;

pub fn catalog_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("catalog/packages")
}

pub fn load_core_logic_or(env: &mut ElabEnv) {
    env.elaborate_module_from_roots_strict(&[catalog_root()], "Core.Logic.Or")
        .expect("Core.Logic.Or must load through strict catalog resolution");
}
