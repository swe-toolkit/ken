use std::collections::BTreeSet;

use ken_kernel::{declare_postulate, Decl, GlobalId, KernelError, Level, Term};
use num_bigint::BigInt;

use crate::classes::{InstanceHeadKey, InstanceResolution};
use crate::effects::{EffectRow, RowType};
use crate::error::{ElabError, Span};
use crate::foreign::{FfiRuntimeCheck, ForeignBinding};
use crate::numbers::{AddEntry, NumericLitVal};
use crate::standard_operators::StandardOperatorRole;
use crate::{ast, ElabEnv};

fn trusted_ids(env: &ElabEnv) -> BTreeSet<GlobalId> {
    env.env.trusted_base().into_iter().collect()
}

/// MEASURED: a deliberately failed operation adds an opaque ID, populates the
/// ElabEnv identity maps, and leaves the exact trusted-base delta empty after
/// rollback; the same ID is then reused for a transparent declaration.
/// CLAIMED: every inventoried ElabEnv ID table is scrubbed before reuse.
/// THE GAP: ModuleState's private nested indexes are exercised by its own
/// module test; this test covers the top-level ElabEnv inventory.
#[test]
fn failed_operation_scrubs_elab_identity_tables_before_id_reuse() {
    let mut env = ElabEnv::new().expect("base environment");
    let before = trusted_ids(&env);
    let stable_root = env.globals["Bool"];
    let stable_alias = env.globals["True"];
    let alias_to_removed_root = env.globals["False"];
    env.refinement_facts
        .refinement_predicates
        .insert(stable_root, Term::constructor(env.env.tt_id(), Vec::new()));
    env.refinement_facts
        .refinement_aliases
        .insert(stable_alias, stable_root);
    let mut allocated = None;

    let result: Result<(), ElabError> = env.with_env_mark_rollback(|env| {
        let id = declare_postulate(
            &mut env.env,
            "ac0_scrub_probe".to_string(),
            Vec::new(),
            Term::Type(Level::zero()),
        )
        .expect("probe postulate");
        allocated = Some(id);
        env.globals.insert("ac0_scrub_probe".into(), id);
        env.preconditions.insert(id, (0, 1));
        env.num_values
            .insert(id, NumericLitVal::Int(BigInt::from(17)));
        env.fixities.insert(id, ast::Fixity::DEFAULT);
        env.fixity_spans.insert(id, Span::zero());
        env.ctor_decl_spans
            .insert("ac0_scrub_probe".into(), Span::zero());
        env.numeric_env.set_add_entry(
            id,
            AddEntry {
                op_id: id,
                wrapping_id: Some(id),
                no_ovf_id: Some(id),
                result_id: id,
            },
        );
        env.standard_operators.insert(StandardOperatorRole::And, id);
        env.foreign_env.register(
            "ac0_scrub_probe".into(),
            ForeignBinding {
                postulate_id: id,
                symbol: "ac0_scrub_probe".into(),
                library: "c".into(),
                is_pure: false,
                effect_row: EffectRow::singleton("FS"),
                marshal_sig: None,
                runtime_checks: vec![FfiRuntimeCheck {
                    hole_id: id,
                    clause_kind: "ensures",
                    clause_str: "probe".into(),
                }],
            },
        );
        env.effect_rows
            .insert("ac0_scrub_probe".into(), RowType::empty());
        env.effect_rows_by_id.insert(id, RowType::empty());
        env.space_metadata
            .initial_states
            .insert("ac0_scrub_probe".into(), id);
        env.prelude_env.native_trusted_base.insert(id);
        env.class_env
            .register_record("ac0_scrub_probe".into(), id, Vec::new(), Vec::new());
        env.resolution_provenance.push(InstanceResolution {
            instance_id: id,
            class_name: "probe".into(),
            head_type: "probe".into(),
            defining_package: "probe".into(),
        });
        let retained_term = Term::const_(stable_root, Vec::new());
        env.refinement_facts
            .refinement_predicates
            .insert(id, retained_term.clone());
        env.refinement_facts
            .refinement_aliases
            .insert(id, stable_root);
        env.refinement_facts
            .refinement_aliases
            .insert(alias_to_removed_root, id);
        env.refinement_facts
            .refined_params
            .insert(id, vec![Some(retained_term.clone())]);
        env.refinement_facts
            .constructor_field_predicates
            .insert(id, vec![Some(retained_term.clone())]);
        env.refinement_facts
            .record_field_predicates
            .insert(id, vec![Some(retained_term)]);
        Err(ElabError::Internal("rollback probe".into()))
    });

    let after = trusted_ids(&env);
    assert_eq!(
        after.difference(&before).copied().collect::<BTreeSet<_>>(),
        BTreeSet::new()
    );
    assert!(matches!(result, Err(ElabError::Internal(message)) if message == "rollback probe"));
    let id = allocated.expect("probe allocated an identity");
    assert_eq!(env.env.next_global_id(), id);
    assert!(!env.globals.contains_key("ac0_scrub_probe"));
    assert!(!env.preconditions.contains_key(&id));
    assert!(!env.num_values.contains_key(&id));
    assert!(!env.fixities.contains_key(&id));
    assert!(!env.fixity_spans.contains_key(&id));
    assert!(!env.ctor_decl_spans.contains_key("ac0_scrub_probe"));
    assert!(env
        .numeric_env
        .classify_add(&Term::const_(id, Vec::new()))
        .is_none());
    assert!(!env.standard_operators.values().any(|stored| *stored == id));
    assert!(!env.foreign_env.bindings.contains_key("ac0_scrub_probe"));
    assert!(!env.effect_rows.contains_key("ac0_scrub_probe"));
    assert!(!env.effect_rows_by_id.contains_key(&id));
    assert_eq!(env.space_initial_state("ac0_scrub_probe"), None);
    assert!(!env.prelude_env.native_trusted_base.contains(&id));
    assert!(env.class_env.projection_by_type_id(id).is_none());
    assert!(env.resolution_provenance.is_empty());
    assert!(!env.refinement_facts.refinement_predicates.contains_key(&id));
    assert!(!env.refinement_facts.refinement_aliases.contains_key(&id));
    assert!(!env
        .refinement_facts
        .refinement_aliases
        .contains_key(&alias_to_removed_root));
    assert_eq!(
        env.refinement_facts.refinement_aliases.get(&stable_alias),
        Some(&stable_root)
    );
    assert!(!env.refinement_facts.refined_params.contains_key(&id));
    assert!(!env
        .refinement_facts
        .constructor_field_predicates
        .contains_key(&id));
    assert!(!env
        .refinement_facts
        .record_field_predicates
        .contains_key(&id));
    assert!(env
        .refinement_facts
        .refinement_predicate(stable_alias)
        .is_some());

    let replacement = env
        .elaborate_decl_v1("const ac0_after : Bool = True")
        .expect("the rolled-back identity remains reusable");
    assert_eq!(replacement.def_id, id);
    assert!(env
        .numeric_env
        .classify_add(&Term::const_(replacement.def_id, Vec::new()))
        .is_none());
    assert_eq!(
        env.refinement_facts.refinement_root(replacement.def_id),
        None
    );
    assert_eq!(
        env.refinement_facts.refinement_root(stable_alias),
        Some(stable_root)
    );
}

/// Promise class: durable ClassEnv identity-scrubbing invariant (AC-2).
/// MEASURED: a real `instance` declaration containing `Axiom` populates the
/// production class/head index inside an outer failed EnvMark transaction;
/// its head ID is then reused by a replacement declaration.
/// CLAIMED: rollback removes the failed instance from every ClassEnv index
/// while preserving a pre-mark instance and its explicit Axiom trust entry.
/// THE GAP: the fixture exercises parsed class and instance declarations;
/// the assertions inspect identity-keyed maps rather than source spellings.
#[test]
fn failed_elabenv_rollback_scrubs_class_instances_before_id_reuse() {
    let mut env = ElabEnv::new().expect("base environment");
    let initial = trusted_ids(&env);
    env.elaborate_file_v1(
        r#"class QaProvider A { evidence : Top }
instance QaProvider Int { evidence = Axiom }"#,
    )
    .expect("pre-mark class and instance");
    let stable_trust = trusted_ids(&env);
    let stable_axiom_id = stable_trust
        .difference(&initial)
        .copied()
        .find(|id| {
            matches!(
                env.env.lookup(*id),
                Some(Decl::Opaque { name, .. }) if name == "QaProvider.Int.evidence"
            )
        })
        .expect("the stable Axiom has its canonical instance-field owner");

    let class_id = env.globals["QaProvider"];
    let stable_head_id = env.globals["Int"];
    let stable_key = (class_id, InstanceHeadKey::Global(stable_head_id));
    let stable_instance_id = env.class_env.instances_by_id[&stable_key].instance_id;
    assert_eq!(
        env.class_env.instance_search("QaProvider", "Int"),
        Some(stable_instance_id)
    );
    let before = trusted_ids(&env);
    let mut removed_head = None;
    let mut removed_instance = None;

    let result: Result<(), ElabError> = env.with_env_mark_rollback(|env| {
        env.elaborate_file_v1(
            "data QaProviderType = MkQaProviderType\n\
             instance QaProvider QaProviderType { evidence = Axiom }",
        )
        .expect("transaction creates a real provider instance");
        let provider_axiom_id = trusted_ids(env)
            .difference(&before)
            .copied()
            .find(|id| {
                matches!(
                    env.env.lookup(*id),
                    Some(Decl::Opaque { name, .. }) if name == "QaProvider.QaProviderType.evidence"
                )
            })
            .expect("provider instance creates its named Axiom");
        let during = trusted_ids(env);
        assert!(!before.contains(&provider_axiom_id));
        assert!(during.contains(&provider_axiom_id));

        let head_id = env.globals["QaProviderType"];
        let key = (class_id, InstanceHeadKey::Global(head_id));
        let instance_id = env.class_env.instances_by_id[&key].instance_id;
        removed_head = Some(head_id);
        removed_instance = Some(instance_id);
        let failure = env
            .elaborate_decl_v1("const ac0_bad_after_instance : Bool = MkQaProviderType")
            .expect_err("the constructor cannot check as Bool");
        assert!(
            env.class_env.instances_by_id.contains_key(&key),
            "the inner failed declaration must retain the earlier instance"
        );
        Err(failure)
    });

    let after = trusted_ids(&env);
    assert_eq!(
        after.difference(&before).copied().collect::<BTreeSet<_>>(),
        BTreeSet::new(),
        "the failed outer operation rolls back the instance's Axiom trust"
    );
    assert!(matches!(
        result,
        Err(ElabError::KernelRejected {
            error: KernelError::TypeMismatch { .. },
            ..
        })
    ));
    let head_id = removed_head.expect("provider head was registered");
    let instance_id = removed_instance.expect("provider instance was registered");
    let removed_key = (class_id, InstanceHeadKey::Global(head_id));

    assert_eq!(env.env.next_global_id(), head_id);
    assert!(!env.globals.contains_key("QaProviderType"));
    assert!(env.env.lookup(instance_id).is_none());
    assert!(!env.class_env.instances_by_id.contains_key(&removed_key));
    assert!(!env
        .class_env
        .instances_by_id
        .values()
        .any(|info| info.instance_id == instance_id));
    assert!(!env
        .class_env
        .instances
        .values()
        .any(|info| info.instance_id == instance_id));
    assert_eq!(
        env.class_env
            .instance_search("QaProvider", "QaProviderType"),
        None
    );
    assert_eq!(
        env.class_env.instances_by_id[&stable_key].instance_id,
        stable_instance_id
    );
    assert_eq!(
        env.class_env.instance_search("QaProvider", "Int"),
        Some(stable_instance_id)
    );
    assert!(after.contains(&stable_axiom_id));
    assert!(matches!(
        env.env.lookup(stable_axiom_id),
        Some(Decl::Opaque { name, .. }) if name == "QaProvider.Int.evidence"
    ));

    let replacement = env
        .elaborate_decl_v1("const QaProviderType : Type = Int")
        .expect("rolled-back head ID is reusable");
    assert_eq!(replacement.def_id, head_id);
    assert!(!env.class_env.instances_by_id.contains_key(&removed_key));
    assert_eq!(
        env.class_env
            .instance_search("QaProvider", "QaProviderType"),
        None
    );
    assert_eq!(
        env.class_env.instances_by_id[&stable_key].instance_id,
        stable_instance_id
    );
}
