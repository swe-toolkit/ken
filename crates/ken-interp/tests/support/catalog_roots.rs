//! Roots-load an owner's checked catalog unit and execute its checked fences.

use ken_elaborator::ElabEnv;
use ken_kernel::GlobalId;
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

/// Pre-source identities, captured before any package source loads. A
/// later package declaration cannot overwrite an entry in this copy.
pub fn pre_load_ids(env: &ElabEnv) -> std::collections::HashMap<String, GlobalId> {
    env.globals.clone()
}

pub fn selected_owner_id(
    env: &ElabEnv,
    pre_load: &std::collections::HashMap<String, GlobalId>,
    module: &str,
    name: &str,
) -> GlobalId {
    let qualified = format!("{module}.{name}");
    env.module_state
        .root_results_for_report(module)
        .and_then(|results| results.iter().find(|result| result.name == qualified))
        .map(|result| result.def_id)
        .or_else(|| pre_load.get(name).copied())
        .unwrap_or_else(|| panic!("missing checked {module} owner or pre-source name {name}"))
}
