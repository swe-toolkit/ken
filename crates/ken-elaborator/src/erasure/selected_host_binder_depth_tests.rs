use super::*;

// Synthetic route input, not a checked-source/native-emission witness. The
// production eraser builds the Match/Let spine. The oracle does not consult
// leaf_dispatch's depth arguments: it follows the evaluator's case_env =
// args ++ env and body_env = [value] ++ env rules.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Resolved {
    Word(String),
    Constructor(String, Vec<Self>),
}

fn resolve(expr: &RuntimeExpr, env: &[Resolved]) -> Resolved {
    match expr {
        RuntimeExpr::Var(index) => env[*index as usize].clone(),
        RuntimeExpr::Value(RuntimeValue::String(word)) => Resolved::Word(word.clone()),
        RuntimeExpr::Construct { constructor, args } => Resolved::Constructor(
            constructor.clone(),
            args.iter().map(|arg| resolve(arg, env)).collect(),
        ),
        RuntimeExpr::Match {
            scrutinee, cases, ..
        } => {
            let Resolved::Constructor(constructor, mut args) = resolve(scrutinee, env) else {
                panic!("the selected host operation must be a constructor");
            };
            let case = cases
                .iter()
                .find(|case| case.constructor == constructor)
                .expect("the selected host constructor has a case");
            assert_eq!(args.len(), case.binders);
            args.extend_from_slice(env);
            resolve(&case.body, &args)
        }
        RuntimeExpr::Let { value, body } => {
            assert!(matches!(&**value, RuntimeExpr::Effect { .. }));
            let mut body_env = vec![Resolved::Word("host-response".into())];
            body_env.extend_from_slice(env);
            resolve(body, &body_env)
        }
        other => panic!("unexpected selected-host expression: {other:?}"),
    }
}

fn chosen_operation(constructors: &[String], arguments: Vec<RuntimeExpr>) -> RuntimeExpr {
    let mut constructors = constructors.iter().rev();
    let mut body = RuntimeExpr::Construct {
        constructor: constructors.next().expect("one operation").clone(),
        args: arguments,
    };
    for constructor in constructors {
        body = RuntimeExpr::Construct {
            constructor: constructor.clone(),
            args: vec![body],
        };
    }
    body
}

#[test]
fn selected_host_leaf_free_var_resolves_through_actual_match_and_let_environments() {
    let id = |name: &str| StableSymbol::declaration("selected-host-depth", &[], name);
    let coproduct = id("Coproduct");
    let in_l = StableSymbol::constructor(&coproduct, "InL");
    let in_r = StableSymbol::constructor(&coproduct, "InR");
    let fs_family = id("FSOp");
    let console_family = id("ConsoleOp");
    let clock_family = id("ClockOp");
    let entropy_family = id("EntropyOp");
    let fs_op = StableSymbol::constructor(&fs_family, "FsReadFile");
    let console_op = StableSymbol::constructor(&console_family, "ConsoleFlush");
    let clock_op = StableSymbol::constructor(&clock_family, "WallNow");
    let entropy_op = StableSymbol::constructor(&entropy_family, "RandomBytes");
    let mut semantic = checked_core::CheckedCoreSemanticInputs::default();
    let mut operations = BTreeMap::new();
    for (family, op, arguments, host_op) in [
        (&fs_family, &fs_op, 2, ken_host::HostOpV1::FsReadFile),
        (
            &console_family,
            &console_op,
            1,
            ken_host::HostOpV1::ConsoleFlush,
        ),
        (
            &clock_family,
            &clock_op,
            0,
            ken_host::HostOpV1::ClockWallNow,
        ),
        (
            &entropy_family,
            &entropy_op,
            1,
            ken_host::HostOpV1::EntropyRandomBytes,
        ),
    ] {
        operations.insert(op.clone(), host_op);
        semantic.data_metadata.insert(
            family.clone(),
            checked_core::DataMetadata {
                parameter_count: 0,
                index_count: 0,
                constructors: vec![checked_core::ConstructorMetadata {
                    symbol: op.clone(),
                    argument_count: arguments,
                    target_index_count: 0,
                    recursive_positions: Vec::new(),
                    lowerability: checked_core::LowerabilityStatus::Supported,
                }],
                eliminator: checked_core::LowerabilityStatus::Supported,
                lowerability: checked_core::LowerabilityStatus::Supported,
            },
        );
    }
    let unused = id("UnusedRole");
    let spine = CheckedHostSpineV1 {
        ret: unused.clone(),
        vis: unused.clone(),
        in_l: in_l.clone(),
        in_r: in_r.clone(),
        fs_family,
        console_family,
        clock_family,
        entropy_family,
        capability: unused.clone(),
        result_err: unused.clone(),
        result_ok: unused.clone(),
        option_some: unused.clone(),
        file_error: unused.clone(),
        file_operation_read: unused.clone(),
        file_operation_write: unused.clone(),
        file_operation_change_mode: unused.clone(),
        io_errors: Vec::new(),
        resource_host_io: unused.clone(),
        resource_closed: unused.clone(),
        resource_malformed: unused.clone(),
        resource_right_not_held: unused.clone(),
        resource_release_failed: unused.clone(),
        resource_kind_mismatch: unused.clone(),
        resource_buffer_limit: unused.clone(),
        resource_allocation_failed: unused.clone(),
        resource_invalid_offset: unused.clone(),
        resource_invalid_bounds: unused.clone(),
        resource_no_progress: unused.clone(),
        resource_kind_fs_handle: unused.clone(),
        resource_kind_buffer: unused.clone(),
        resource_trace_identity: unused.clone(),
        nat_zero: unused.clone(),
        nat_suc: unused.clone(),
        private_buffer_span: unused.clone(),
        private_transfer_count: unused.clone(),
        read_some: unused.clone(),
        read_eof: unused.clone(),
        wrote: unused.clone(),
        mk_instant: unused.clone(),
        read_chunk: unused.clone(),
        read_result_eof: unused.clone(),
        unit: unused.clone(),
        bool_false: unused.clone(),
        bool_true: unused.clone(),
        file_operation_append: unused.clone(),
        file_operation_metadata: unused.clone(),
        file_metadata: unused.clone(),
        file_kind_file: unused.clone(),
        file_kind_directory: unused.clone(),
        file_kind_symlink: unused.clone(),
        file_kind_other: unused.clone(),
        file_operation_rename: unused.clone(),
        file_operation_read_directory: unused.clone(),
        file_operation_create_directory: unused.clone(),
        file_operation_remove_file: unused.clone(),
        file_operation_remove_directory: unused.clone(),
        dir_entry: unused.clone(),
        file_operation_seek: unused.clone(),
        file_operation_set_length: unused.clone(),
        file_operation_sync: unused.clone(),
        file_operation_get_inheritance: unused.clone(),
        file_operation_set_inheritance: unused.clone(),
        file_operation_duplicate: unused.clone(),
        resource_mapping_limit: unused.clone(),
        resource_kind_mapping: unused,
        operations,
    };
    let word = |text: &str| RuntimeExpr::Value(RuntimeValue::String(text.into()));
    let outer = Resolved::Word("outer-variable".into());
    let neighbour = Resolved::Word("neighbour-variable".into());
    let root_env = [outer.clone(), neighbour];
    let mut wrong_tail_reads = Vec::new();
    for (leaf, path, args) in [
        (
            "fs",
            vec![in_l.to_string(), fs_op.to_string()],
            vec![word("fs-cap"), word("fs-path")],
        ),
        (
            "console",
            vec![in_r.to_string(), in_l.to_string(), console_op.to_string()],
            vec![word("console-stream")],
        ),
        (
            "clock",
            vec![
                in_r.to_string(),
                in_r.to_string(),
                in_l.to_string(),
                clock_op.to_string(),
            ],
            vec![],
        ),
        (
            "entropy",
            vec![
                in_r.to_string(),
                in_r.to_string(),
                in_r.to_string(),
                entropy_op.to_string(),
            ],
            vec![word("entropy-count")],
        ),
    ] {
        let ir = lower_runtime_selected_host_operation(
            chosen_operation(&path, args),
            RuntimeExpr::Var(1),
            &semantic,
            &spine,
            &id("main"),
        )
        .expect("production selected-host erasure builds the runtime Match spine");
        let actual = resolve(&ir, &root_env);
        if leaf == "fs" || leaf == "console" {
            assert_eq!(actual, outer, "the {leaf} positive control must resolve");
        } else if actual != outer {
            wrong_tail_reads.push((leaf, actual));
        }
    }
    assert!(
        wrong_tail_reads.is_empty(),
        "each tail leaf must resolve its outer Var instead of an intervening binder: {wrong_tail_reads:?}"
    );
}
