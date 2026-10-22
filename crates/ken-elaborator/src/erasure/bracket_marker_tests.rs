//! Guard the source-node binding itself, separate from the checked-source
//! population covered by the integration fixture tests.

use super::*;

fn owner() -> StableSymbol {
    StableSymbol::declaration("rt-bracket-transport", &[], "main")
}

fn private_constructor() -> StableSymbol {
    StableSymbol::constructor(
        &StableSymbol::declaration("rt-bracket-transport", &[], "FSOp"),
        "PrivateBracketRelease",
    )
}

fn private_acquire() -> StableSymbol {
    StableSymbol::constructor(
        &StableSymbol::declaration("rt-bracket-transport", &[], "FSOp"),
        "PrivateFsOpen",
    )
}

fn region(node_id: u32) -> NormalizedBracketRegion {
    NormalizedBracketRegion {
        owner: owner(),
        acquire_vis_path: vec![1],
        acquire_response_binder_path: vec![1, 2],
        ok_resource_binder_path: vec![1, 2, 3],
        acquire_constructor: private_acquire(),
        acquire_constructor_node_id: node_id + 100,
        finalizer_paths: vec![vec![1, 2, 3, 4]],
        finalizer_constructor_node_ids: vec![node_id],
    }
}

fn constructor(
    symbol: StableSymbol,
    node_id: Option<u32>,
) -> checked_core::CheckedCoreConstructorView {
    checked_core::CheckedCoreConstructorView {
        checked_node_id: node_id,
        family_symbol: StableSymbol::declaration("rt-bracket-transport", &[], "FSOp"),
        symbol,
        level_args: Vec::new(),
        family_parameter_count: 0,
        family_index_count: 0,
        argument_count: 3,
        target_index_count: 0,
        recursive_positions: Vec::new(),
        constructor_lowerability: LowerabilityStatus::Supported,
        family_lowerability: LowerabilityStatus::Supported,
    }
}

fn collector(regions: Vec<NormalizedBracketRegion>) -> NativeLoweringPlanCollector {
    NativeLoweringPlanCollector::new(None, vec![], vec![], vec![])
        .with_bracket_regions(private_constructor(), regions)
        .expect("unambiguous checked regions")
}

fn assert_lane(error: ErasureError, expected: &str) {
    assert!(
        matches!(error, ErasureError::ExpressionLowering { lane, .. } if lane == expected),
        "expected exact bracket refusal lane {expected}, got {error:?}"
    );
}

/// Promise class: durable uniqueness of the source-region inventory. A
/// malformed region or two regions naming one checked node cannot create
/// an ambiguous lowering plan while remaining green.
#[test]
fn duplicate_or_nonunique_source_region_seed_refuses_by_name() {
    let mut two_finalizers = region(4);
    two_finalizers.finalizer_paths.clear();
    two_finalizers.finalizer_constructor_node_ids.clear();
    assert_lane(
        NativeLoweringPlanCollector::new(None, vec![], vec![], vec![])
            .with_bracket_regions(private_constructor(), vec![two_finalizers])
            .err()
            .expect("a region without a source finalizer cannot form transport"),
        "checked_bracket_region_finalizer_count",
    );
    assert_lane(
        NativeLoweringPlanCollector::new(None, vec![], vec![], vec![])
            .with_bracket_regions(private_constructor(), vec![region(4), region(4)])
            .err()
            .expect("duplicate source node cannot form two transport regions"),
        "checked_bracket_region_duplicate_source",
    );
}

/// Promise class: durable refusal partition. Three distinct forbidden states
/// must fail at their own binding seam, not via a later unrelated IR failure.
#[test]
fn missing_extra_and_duplicate_constructor_bindings_refuse_by_name() {
    let pending = collector(vec![region(4)]);
    assert_lane(
        pending.validate_bracket_regions_bound(&owner()).unwrap_err(),
        "checked_bracket_acquire_without_marker",
    );
    let mut missing_finalizer = collector(vec![region(4)]);
    missing_finalizer
        .consume_bracket_constructor(&owner(), &constructor(private_acquire(), Some(104)))
        .unwrap();
    assert_lane(
        missing_finalizer.validate_bracket_regions_bound(&owner()).unwrap_err(),
        "checked_bracket_region_without_marker",
    );

    let mut extra = collector(vec![]);
    assert_lane(
        extra
            .consume_bracket_constructor(&owner(), &constructor(private_constructor(), Some(4)))
            .unwrap_err(),
        "checked_bracket_marker_without_region",
    );

    let mut duplicate = collector(vec![region(4)]);
    let acquire = constructor(private_acquire(), Some(104));
    let actual = constructor(private_constructor(), Some(4));
    let acquired = duplicate
        .consume_bracket_constructor(&owner(), &acquire)
        .unwrap()
        .expect("acquire marker emitted");
    assert_eq!(acquired.role, ken_runtime::CheckedBracketMarkRole::Acquire);
    assert_lane(
        duplicate.consume_bracket_constructor(&owner(), &acquire).unwrap_err(),
        "checked_bracket_acquire_duplicate",
    );
    let marker = duplicate
        .consume_bracket_constructor(&owner(), &actual)
        .expect("unique binding")
        .expect("marker emitted");
    assert_eq!(marker.constructor_node_id, 4);
    assert_eq!(marker.role, ken_runtime::CheckedBracketMarkRole::Finalizer);
    assert_lane(
        duplicate.consume_bracket_constructor(&owner(), &actual).unwrap_err(),
        "checked_bracket_marker_duplicate",
    );
    duplicate.validate_bracket_regions_bound(&owner()).expect("both roles consumed");
}

/// Promise class: durable independent identity check. The right node number
/// with a public constructor cannot claim a private finalizer's region.
#[test]
fn checked_node_id_alone_does_not_authenticate_the_constructor() {
    let mut bound = collector(vec![region(4)]);
    let public = StableSymbol::constructor(
        &StableSymbol::declaration("rt-bracket-transport", &[], "FSOp"),
        "ResourceRelease",
    );
    assert_lane(
        bound
            .consume_bracket_constructor(&owner(), &constructor(public.clone(), Some(4)))
            .unwrap_err(),
        "checked_bracket_marker_wrong_constructor",
    );
    assert_lane(
        bound
            .consume_bracket_constructor(&owner(), &constructor(public.clone(), Some(104)))
            .unwrap_err(),
        "checked_bracket_acquire_wrong_constructor",
    );
    assert_lane(
        bound
            .consume_bracket_constructor(&owner(), &constructor(private_constructor(), None))
            .unwrap_err(),
        "checked_bracket_marker_missing_node_id",
    );
}

fn program(body: RuntimeExpr) -> RuntimeProgram {
    RuntimeProgram {
        package_identity: "rt-bracket-transport".into(),
        core_semantic_hash: 0,
        artifact_hash: 0,
        erased_core: ErasedExecutableCore {
            symbols: BTreeSet::new(),
            metadata: RuntimeMetadata::default(),
        },
        declarations: vec![RuntimeDeclaration {
            symbol: owner().to_string(),
            kind: RuntimeDeclarationKind::Transparent { body },
            metadata: RuntimeSymbolMetadata::empty(),
        }],
        examples: vec![],
    }
}

fn bind_bracket_runtime_marker_locations(
    root: &StableSymbol,
    program: &RuntimeProgram,
    expected: &BTreeMap<(StableSymbol, u32), BoundBracketMark>,
    private_constructor: &StableSymbol,
) -> Result<(), ErasureError> {
    super::bind_bracket_runtime_marker_locations(
        root,
        program,
        expected,
        private_constructor,
        &BTreeMap::from([(private_acquire(), ken_host::HostOpV1::FsOpen)]),
    )
}

fn wrapped(region: ken_runtime::CheckedBracketRegionMarker) -> RuntimeExpr {
    let constructor = match region.role {
        ken_runtime::CheckedBracketMarkRole::Acquire => private_acquire(),
        ken_runtime::CheckedBracketMarkRole::Finalizer => private_constructor(),
    };
    RuntimeExpr::CheckedBracketFinalizer {
        region: Box::new(region),
        body: Box::new(RuntimeExpr::Construct {
            constructor: constructor.to_string(),
            args: vec![],
        }),
    }
}

fn paired(
    acquire: ken_runtime::CheckedBracketRegionMarker,
    finalizer: ken_runtime::CheckedBracketRegionMarker,
) -> RuntimeExpr {
    RuntimeExpr::Let {
        value: Box::new(wrapped(acquire)),
        body: Box::new(wrapped(finalizer)),
    }
}

/// Promise class: durable bijection at the emitted Runtime-IR boundary. The
/// second scan checks persisted location and metadata, not merely collector
/// consumption, and cannot silently accept a copied or repointed wrapper.
#[test]
fn emitted_marker_bijection_rejects_missing_duplicate_and_unbound() {
    let mut source = collector(vec![region(4)]);
    let acquire = source
        .consume_bracket_constructor(&owner(), &constructor(private_acquire(), Some(104)))
        .unwrap()
        .unwrap();
    let marker = source
        .consume_bracket_constructor(&owner(), &constructor(private_constructor(), Some(4)))
        .unwrap()
        .unwrap();
    let expected = source.consumed_brackets;
    bind_bracket_runtime_marker_locations(
        &owner(),
        &program(paired(acquire.clone(), marker.clone())),
        &expected,
        &private_constructor(),
    )
    .expect("both exact emitted anchors");
    assert_lane(
        bind_bracket_runtime_marker_locations(
            &owner(),
            &program(wrapped(marker.clone())),
            &expected,
            &private_constructor(),
        ).unwrap_err(),
        "checked_bracket_acquire_without_marker",
    );
    let mut extra_acquire = acquire.clone();
    extra_acquire.constructor_node_id = 900;
    assert_lane(
        bind_bracket_runtime_marker_locations(
            &owner(),
            &program(RuntimeExpr::Let {
                value: Box::new(paired(acquire.clone(), marker.clone())),
                body: Box::new(wrapped(extra_acquire)),
            }),
            &expected,
            &private_constructor(),
        ).unwrap_err(),
        "checked_bracket_acquire_without_region",
    );
    assert_lane(
        bind_bracket_runtime_marker_locations(
            &owner(),
            &program(RuntimeExpr::Var(0)),
            &expected,
            &private_constructor(),
        )
        .unwrap_err(),
        "checked_bracket_acquire_without_marker",
    );
    assert_lane(
        bind_bracket_runtime_marker_locations(
            &owner(),
            &program(RuntimeExpr::Let {
                value: Box::new(wrapped(acquire.clone())),
                body: Box::new(paired(acquire.clone(), marker.clone())),
            }),
            &expected,
            &private_constructor(),
        )
        .unwrap_err(),
        "checked_bracket_acquire_duplicate",
    );
    let mut wrong_role = marker.clone();
    wrong_role.role = ken_runtime::CheckedBracketMarkRole::Acquire;
    assert_lane(
        bind_bracket_runtime_marker_locations(
            &owner(),
            &program(RuntimeExpr::Let {
                value: Box::new(wrapped(acquire.clone())),
                body: Box::new(RuntimeExpr::CheckedBracketFinalizer {
                    region: Box::new(wrong_role),
                    body: Box::new(RuntimeExpr::Construct {
                        constructor: private_constructor().to_string(),
                        args: vec![],
                    }),
                }),
            }),
            &expected,
            &private_constructor(),
        ).unwrap_err(),
        "checked_bracket_marker_mismatch",
    );
    let other = region(5);
    let mut wrong = marker.clone();
    wrong.constructor_node_id = other.finalizer_constructor_node_ids[0];
    assert_lane(
        bind_bracket_runtime_marker_locations(
            &owner(),
            &program(paired(acquire.clone(), wrong)),
            &expected,
            &private_constructor(),
        )
        .unwrap_err(),
        "checked_bracket_region_without_marker",
    );
    let other_private = RuntimeExpr::Construct {
        constructor: private_constructor().to_string(),
        args: vec![],
    };
    assert_lane(
        bind_bracket_runtime_marker_locations(
            &owner(),
            &program(RuntimeExpr::Let {
                value: Box::new(paired(acquire.clone(), marker.clone())),
                body: Box::new(other_private),
            }),
            &expected,
            &private_constructor(),
        )
        .unwrap_err(),
        "checked_bracket_marker_without_region",
    );
    let mut public_marker = wrapped(marker.clone());
    if let RuntimeExpr::CheckedBracketFinalizer { body, .. } = &mut public_marker {
        *body = Box::new(RuntimeExpr::Construct {
            constructor: "public ResourceRelease".into(),
            args: vec![],
        });
    }
    assert_lane(
        bind_bracket_runtime_marker_locations(
            &owner(),
            &program(RuntimeExpr::Let {
                value: Box::new(wrapped(acquire.clone())),
                body: Box::new(public_marker),
            }),
            &expected,
            &private_constructor(),
        )
        .unwrap_err(),
        "checked_bracket_marker_wrong_constructor",
    );
    assert_lane(
        bind_bracket_runtime_marker_locations(
            &owner(),
            &program(paired(acquire, marker)),
            &BTreeMap::new(),
            &private_constructor(),
        )
        .unwrap_err(),
        "checked_bracket_acquire_without_region",
    );
}

fn static_acquire_effect(mark: ken_runtime::CheckedBracketRegionMarker) -> RuntimeExpr {
    RuntimeExpr::Effect {
        family: StableSymbol::declaration("rt-bracket-transport", &[], "FSOp").to_string(),
        operation: ken_host::HostOpV1::FsOpen,
        capability: None,
        args: vec![],
        bracket_acquire: Some(Box::new(mark)),
    }
}

/// Promise class: durable two-route bijection. The static Effect carries the
/// acquire provenance at the executing node, not a shadow Construct. A missing,
/// duplicated, regionless, or mismapped static mark fails by its own lane.
#[test]
fn static_effect_acquire_bijection_refuses_missing_duplicate_and_wrong_mapping() {
    let mut source = collector(vec![region(4)]);
    let acquire = source.consume_bracket_constructor(
        &owner(), &constructor(private_acquire(), Some(104))
    ).unwrap().unwrap();
    let finalizer = source.consume_bracket_constructor(
        &owner(), &constructor(private_constructor(), Some(4))
    ).unwrap().unwrap();
    let expected = source.consumed_brackets;
    let valid = || RuntimeExpr::Let {
        value: Box::new(static_acquire_effect(acquire.clone())),
        body: Box::new(wrapped(finalizer.clone())),
    };
    bind_bracket_runtime_marker_locations(
        &owner(), &program(valid()), &expected, &private_constructor(),
    ).expect("static Effect is the checked acquire anchor");
    let mut missing = valid();
    if let RuntimeExpr::Let { value, .. } = &mut missing {
        if let RuntimeExpr::Effect { bracket_acquire, .. } = value.as_mut() {
            *bracket_acquire = None;
        }
    }
    assert_lane(bind_bracket_runtime_marker_locations(
        &owner(), &program(missing), &expected, &private_constructor(),
    ).unwrap_err(), "checked_bracket_acquire_without_marker");
    assert_lane(bind_bracket_runtime_marker_locations(
        &owner(),
        &program(RuntimeExpr::Let {
            value: Box::new(static_acquire_effect(acquire.clone())),
            body: Box::new(valid()),
        }),
        &expected, &private_constructor(),
    ).unwrap_err(), "checked_bracket_acquire_duplicate");
    let mut extra = acquire.clone();
    extra.constructor_node_id = 904;
    assert_lane(bind_bracket_runtime_marker_locations(
        &owner(), &program(RuntimeExpr::Let {
            value: Box::new(static_acquire_effect(extra)),
            body: Box::new(valid()),
        }), &expected, &private_constructor(),
    ).unwrap_err(), "checked_bracket_acquire_without_region");
    let mut wrong_operation = static_acquire_effect(acquire);
    if let RuntimeExpr::Effect { operation, .. } = &mut wrong_operation {
        *operation = ken_host::HostOpV1::ResourceRelease;
    }
    assert_lane(bind_bracket_runtime_marker_locations(
        &owner(), &program(RuntimeExpr::Let {
            value: Box::new(wrong_operation),
            body: Box::new(wrapped(finalizer)),
        }), &expected, &private_constructor(),
    ).unwrap_err(), "checked_bracket_acquire_wrong_constructor");
}

/// Promise class: durable production deletion discriminator. Removing only
/// static-arm acquire consumption from the real erasure path must refuse by
/// name; an enclosing Vis or finalizer never supplies a fallback anchor.
#[test]
fn production_static_acquire_deletion_refuses_three_real_brackets() {
    let template = include_str!("../../tests/fixtures/rt_bracket_region_multileaf.ken");
    for (index, body) in [
        "(\\resource. leaf_body terminal resource)",
        "(\\resource. level_one_body cap terminal resource)",
        "(\\resource. level_two_body cap terminal resource)",
    ].into_iter().enumerate() {
        OMIT_STATIC_ACQUIRE_MARK.with(|flag| flag.set(true));
        let result = crate::compiler_driver::prepare_native_program_sources(
            &format!("rt-acquire-deletion-{index}"),
            vec![crate::compiler_driver::CompilerSource::new(
                "region.ken", template.replace("__ROOT_BODY__", body),
            )],
        );
        OMIT_STATIC_ACQUIRE_MARK.with(|flag| flag.set(false));
        let error = result.expect_err("deleted production acquire mark must fail");
        assert!(matches!(
            &error,
            crate::compiler_driver::NativeProgramBuildError::Erasure(
                ErasureError::ExpressionLowering { lane, .. }
            ) if *lane == "checked_bracket_acquire_without_marker"
        ), "wrong deletion refusal: {error:?}");
    }
    crate::compiler_driver::prepare_native_program_sources(
        "rt-acquire-restored",
        vec![crate::compiler_driver::CompilerSource::new(
            "region.ken", template.replace(
                "__ROOT_BODY__", "(\\resource. leaf_body terminal resource)"
            ),
        )],
    ).expect("restored production acquisition binds again");
}

/// Promise class: durable counterexample to ordinal matching. This uses the
/// actual nested source, not a manufactured pair: preserving both marker and
/// region counts while swapping their binder payloads must still be refused.
#[test]
fn nested_two_ordinal_swap_does_not_bind_the_inner_binder_to_outer_finalizer() {
    let source = include_str!("../../tests/fixtures/rt_bracket_region_multileaf.ken").replace(
        "__ROOT_BODY__",
        "(\\resource. level_one_body cap terminal resource)",
    );
    let prepared = crate::compiler_driver::prepare_native_program_sources(
        "rt-bracket-region-ordinal",
        vec![crate::compiler_driver::CompilerSource::new(
            "region.ken",
            source,
        )],
    )
    .expect("nested checked source prepares");
    let regions = prepared.bracket_regions();
    assert_eq!(regions.len(), 2);
    assert_eq!(regions[0].owner, regions[1].owner);
    assert_ne!(
        regions[0].ok_resource_binder_path,
        regions[1].ok_resource_binder_path
    );
    assert_ne!(
        regions[0].finalizer_constructor_node_ids,
        regions[1].finalizer_constructor_node_ids
    );
    let markers: Vec<_> = regions
        .iter()
        .map(|region| ken_runtime::CheckedBracketRegionMarker {
            owner: region.owner.to_string(),
            role: ken_runtime::CheckedBracketMarkRole::Finalizer,
            constructor_node_id: region.finalizer_constructor_node_ids[0],
            acquire_vis_path: region.acquire_vis_path.clone(),
            acquire_response_binder_path: region.acquire_response_binder_path.clone(),
            ok_resource_binder_path: region.ok_resource_binder_path.clone(),
            finalizer_path: region.finalizer_paths[0].clone(),
        })
        .collect();
    // Keep both source-node ids and both owner keys fixed: corrupt only the
    // ordinally paired acquire/Ok/finalizer ports, so an id-only comparison
    // cannot be the reason the swapped-binder test reddens.
    let mut wrong_outer = markers[1].clone();
    wrong_outer.constructor_node_id = regions[0].finalizer_constructor_node_ids[0];
    let mut wrong_inner = markers[0].clone();
    wrong_inner.constructor_node_id = regions[1].finalizer_constructor_node_ids[0];
    let mut expected = BTreeMap::new();
    for region in regions {
        expected.insert(
            (region.owner.clone(), region.acquire_constructor_node_id),
            BoundBracketMark {
                marker: ken_runtime::CheckedBracketRegionMarker {
                    owner: region.owner.to_string(),
                    role: ken_runtime::CheckedBracketMarkRole::Acquire,
                    constructor_node_id: region.acquire_constructor_node_id,
                    acquire_vis_path: region.acquire_vis_path.clone(),
                    acquire_response_binder_path: region.acquire_response_binder_path.clone(),
                    ok_resource_binder_path: region.ok_resource_binder_path.clone(),
                    finalizer_path: region.finalizer_paths[0].clone(),
                },
                constructor: region.acquire_constructor.clone(),
            },
        );
    }
    for (region, wrong) in regions.iter().zip([wrong_outer, wrong_inner]) {
        expected.insert(
            (region.owner.clone(), region.finalizer_constructor_node_ids[0]),
            BoundBracketMark {
                marker: wrong,
                constructor: prepared.bracket_host_spine_for_test()
                    .bracket_finalizer_constructor.as_ref().unwrap().clone(),
            },
        );
    }
    assert_lane(
        super::bind_bracket_runtime_marker_locations(
            &regions[0].owner,
            prepared.runtime_program(),
            &expected,
            prepared
                .bracket_host_spine_for_test()
                .bracket_finalizer_constructor
                .as_ref()
                .unwrap(),
            &prepared.bracket_host_spine_for_test().operations,
        )
        .unwrap_err(),
        "checked_bracket_marker_mismatch",
    );
}

fn count_release_host_op_constructs(
    expr: &RuntimeExpr,
    mapped_release_symbols: &BTreeSet<String>,
    candidates: &mut usize,
    exact_markers: &mut usize,
) {
    match expr {
        RuntimeExpr::CheckedBracketFinalizer { region, body } => {
            if region.role == ken_runtime::CheckedBracketMarkRole::Finalizer {
                *exact_markers += 1;
            }
            count_release_host_op_constructs(
                body,
                mapped_release_symbols,
                candidates,
                exact_markers,
            );
        }
        RuntimeExpr::CheckedJoinSite { body, .. }
        | RuntimeExpr::CheckedSubcontinuationFrame { body, .. }
        | RuntimeExpr::CheckedRecursiveInvocation { body, .. }
        | RuntimeExpr::CheckedComputationalIHSlots { body, .. }
        | RuntimeExpr::CheckedComputationalIHInvocation { body, .. }
        | RuntimeExpr::Project { record: body, .. }
        | RuntimeExpr::Closure { body, .. } => count_release_host_op_constructs(
            body,
            mapped_release_symbols,
            candidates,
            exact_markers,
        ),
        RuntimeExpr::LexicalClosure { captures, body, .. } => {
            for capture in captures {
                count_release_host_op_constructs(
                    capture,
                    mapped_release_symbols,
                    candidates,
                    exact_markers,
                );
            }
            count_release_host_op_constructs(
                body,
                mapped_release_symbols,
                candidates,
                exact_markers,
            );
        }
        RuntimeExpr::Let { value, body } => {
            count_release_host_op_constructs(
                value,
                mapped_release_symbols,
                candidates,
                exact_markers,
            );
            count_release_host_op_constructs(
                body,
                mapped_release_symbols,
                candidates,
                exact_markers,
            );
        }
        RuntimeExpr::If {
            scrutinee,
            then_expr,
            else_expr,
        } => {
            for child in [scrutinee, then_expr, else_expr] {
                count_release_host_op_constructs(
                    child,
                    mapped_release_symbols,
                    candidates,
                    exact_markers,
                );
            }
        }
        RuntimeExpr::PrimitiveCall { args, .. } | RuntimeExpr::Effect { args, .. } => {
            for arg in args {
                count_release_host_op_constructs(
                    arg,
                    mapped_release_symbols,
                    candidates,
                    exact_markers,
                );
            }
        }
        RuntimeExpr::Construct { constructor, args } => {
            if mapped_release_symbols.contains(constructor) {
                *candidates += 1;
            }
            for arg in args {
                count_release_host_op_constructs(
                    arg,
                    mapped_release_symbols,
                    candidates,
                    exact_markers,
                );
            }
        }
        RuntimeExpr::Call { callee, args } => {
            count_release_host_op_constructs(
                callee,
                mapped_release_symbols,
                candidates,
                exact_markers,
            );
            for arg in args {
                count_release_host_op_constructs(
                    arg,
                    mapped_release_symbols,
                    candidates,
                    exact_markers,
                );
            }
        }
        RuntimeExpr::Match {
            scrutinee, cases, ..
        } => {
            count_release_host_op_constructs(
                scrutinee,
                mapped_release_symbols,
                candidates,
                exact_markers,
            );
            for case in cases {
                count_release_host_op_constructs(
                    &case.body,
                    mapped_release_symbols,
                    candidates,
                    exact_markers,
                );
            }
        }
        RuntimeExpr::ComputationalMatch {
            scrutinee, cases, ..
        } => {
            count_release_host_op_constructs(
                scrutinee,
                mapped_release_symbols,
                candidates,
                exact_markers,
            );
            for case in cases {
                count_release_host_op_constructs(
                    &case.body,
                    mapped_release_symbols,
                    candidates,
                    exact_markers,
                );
            }
        }
        RuntimeExpr::Record { fields } => {
            for (_, field) in fields {
                count_release_host_op_constructs(
                    field,
                    mapped_release_symbols,
                    candidates,
                    exact_markers,
                );
            }
        }
        RuntimeExpr::Value(_)
        | RuntimeExpr::Var(_)
        | RuntimeExpr::Trap(_)
        | RuntimeExpr::DeclarationRef { .. }
        | RuntimeExpr::ImportedDeclarationRef { .. } => {}
    }
}

/// Promise class: durable HostOp-overmark discriminator on the same prepared
/// source. The REAL host-op table selects three Constructs (including two
/// source-visible releases); exact checked-region binding selects only one.
#[test]
fn release_host_op_selector_overmarks_early_public_releases() {
    let prepared = crate::compiler_driver::prepare_native_program_sources(
        "rt-bracket-region-overmark",
        vec![crate::compiler_driver::CompilerSource::new(
            "region.ken",
            include_str!("../../tests/fixtures/rt_bracket_region_early_release.ken"),
        )],
    )
    .expect("early-release checked source prepares");
    let spine = prepared.bracket_host_spine_for_test();
    let mapped_release_symbols: BTreeSet<_> = spine
        .operations
        .iter()
        .filter(|(_, operation)| **operation == ken_host::HostOpV1::ResourceRelease)
        .map(|(symbol, _)| symbol.to_string())
        .collect();
    let private_constructor = spine
        .bracket_finalizer_constructor
        .as_ref()
        .expect("private identity")
        .to_string();
    assert!(mapped_release_symbols.contains(&private_constructor));
    assert!(
        mapped_release_symbols
            .iter()
            .any(|symbol| symbol != &private_constructor),
        "the checked HostOp table must contain a distinct public release identity"
    );
    let mut candidates = 0;
    let mut exact_markers = 0;
    for declaration in &prepared.runtime_program().declarations {
        if let RuntimeDeclarationKind::Transparent { body } = &declaration.kind {
            count_release_host_op_constructs(
                body,
                &mapped_release_symbols,
                &mut candidates,
                &mut exact_markers,
            );
        }
    }
    assert_eq!(prepared.bracket_regions().len(), 1);
    assert_eq!(exact_markers, 1);
    assert_eq!(
        candidates,
        exact_markers + 2,
        "HostOpV1::ResourceRelease also selects both source-visible early releases"
    );
}
