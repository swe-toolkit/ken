use std::collections::{HashMap, HashSet};

use ken_kernel::GlobalId;

use super::{ExportProvenance, ModuleState, Scope};

/// MEASURED: the same fake identity occurs in every private ModuleState table
/// that can select, expose, or cache a checked global; all occurrences vanish.
/// CLAIMED: a reused ID cannot inherit a stale module binding after rollback.
/// THE GAP: this isolates ModuleState's scrubber; production reachability is
/// established by the enclosing ElabEnv rollback test.
#[test]
fn scrub_removes_reused_ids_from_scopes_and_module_exports() {
    let id = GlobalId(9000);
    let mut state = ModuleState::default();
    let mut scope = Scope::default();
    scope.qualified_ids.insert("M.value".into(), id);
    scope.binding_ids.insert("value".into(), id);
    scope.exported_ids.insert("value".into(), id);
    scope.checked_local_ids.insert("value".into(), id);
    scope.floor_type_ids.insert("Floor".into(), id);
    scope
        .constructor_members
        .insert(id, HashMap::from([("Ctor".to_string(), id)]));
    scope.private_ids.insert(id);
    state.root_scope = scope.clone();
    state.loaded_unit_scopes.insert("M".into(), scope);
    state.loaded_units.insert("M".into(), vec![id]);
    state.prelude_floor_ids.insert("Floor".into(), id);
    state
        .constructor_members
        .insert(id, HashMap::from([("Ctor".to_string(), id)]));
    state.private_ids.insert(id);
    state.scoped_constructor_types.insert(id);
    let member_ids = HashMap::from([("M".to_string(), HashMap::from([("value".to_string(), id)]))]);
    state.file_export_ids.insert("M".into(), member_ids.clone());
    state.export_provenance.insert(
        "M".into(),
        ExportProvenance {
            inline_paths: HashSet::new(),
            file_root: Some("M".into()),
            member_ids,
        },
    );

    state.scrub_global_ids(&HashSet::from([id]));

    assert!(state.root_scope.qualified_ids.is_empty());
    assert!(state.root_scope.binding_ids.is_empty());
    assert!(state.root_scope.exported_ids.is_empty());
    assert!(state.root_scope.checked_local_ids.is_empty());
    assert!(state.root_scope.floor_type_ids.is_empty());
    assert!(state.root_scope.constructor_members.is_empty());
    assert!(state.root_scope.private_ids.is_empty());
    let loaded_scope = &state.loaded_unit_scopes["M"];
    assert!(loaded_scope.binding_ids.is_empty());
    assert!(state.loaded_units["M"].is_empty());
    assert!(state.prelude_floor_ids.is_empty());
    assert!(state.constructor_members.is_empty());
    assert!(state.private_ids.is_empty());
    assert!(state.scoped_constructor_types.is_empty());
    assert!(state.file_export_ids.is_empty());
    assert!(state.export_provenance["M"].member_ids.is_empty());
}
