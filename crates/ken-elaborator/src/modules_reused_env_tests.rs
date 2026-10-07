use std::collections::{HashMap, HashSet};

use ken_kernel::GlobalId;

use super::{ExportProvenance, ModuleState, Scope};

/// MEASURED: one synthetic ID is inserted into the named ModuleState identity
/// maps below, and each selected occurrence is removed by the scrubber.
/// CLAIMED: these maps drop rolled-back identities by GlobalId.
/// THE GAP: this isolates scrubber breadth; the companion integration test
/// below reaches it through ElabEnv rollback with a real exported ID.
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

/// Promise class: durable identity-scrubbing invariant (AC-2).
/// MEASURED: a real inline-module export records a new declaration ID; the
/// enclosing failed transaction removes it while preserving an older export,
/// then the declaration ID is reused.
/// CLAIMED: rollback scrubs only removed identities from ModuleState at the
/// production callback.
/// THE GAP: this integration witness covers exported member IDs; the companion
/// unit test checks the additional private ModuleState identity maps.
#[test]
fn failed_elabenv_rollback_scrubs_module_export_before_id_reuse() {
    let mut env = crate::ElabEnv::new().expect("base environment");
    let stable = env
        .elaborate_file_v1("const ac0_stable : Bool = True module Stable { export ac0_stable }")
        .expect("preexisting export");
    assert_eq!(stable.len(), 1);
    let stable_id = stable[0].def_id;
    let next_id = env.env.next_global_id();
    let mut allocated = None;

    let result: Result<(), crate::error::ElabError> = env.with_env_mark_rollback(|env| {
        let results = env.elaborate_file_v1(
            "const ac0_probe : Bool = True module Provider { export ac0_probe }",
        )?;
        assert_eq!(results.len(), 1);
        let id = results[0].def_id;
        assert_eq!(id, next_id);
        assert_eq!(
            env.module_state.export_provenance["Provider"].member_ids["Provider"]["ac0_probe"],
            id
        );
        allocated = Some(id);
        Err(crate::error::ElabError::Internal(
            "module rollback integration probe".into(),
        ))
    });

    assert!(
        matches!(result, Err(crate::error::ElabError::Internal(message))
        if message == "module rollback integration probe")
    );
    let id = allocated.expect("the inline module exported its checked declaration");
    assert_eq!(env.env.next_global_id(), id);
    assert_eq!(
        env.module_state.export_provenance["Stable"].member_ids["Stable"]["ac0_stable"], stable_id,
        "rollback must preserve module identities from before its mark"
    );
    assert!(env.module_state.export_provenance["Provider"]
        .member_ids
        .is_empty());

    let replacement = env
        .elaborate_decl_v1("const ac0_after : Bool = True")
        .expect("the rolled-back GlobalId remains reusable");
    assert_eq!(replacement.def_id, id);
    assert!(env.module_state.export_provenance["Provider"]
        .member_ids
        .is_empty());
    assert_eq!(
        env.module_state.export_provenance["Stable"].member_ids["Stable"]["ac0_stable"],
        stable_id
    );
}
