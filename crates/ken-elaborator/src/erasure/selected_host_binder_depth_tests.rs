use super::*;

// Synthetic route input, not a checked-source/native-emission witness. The
// production eraser builds the Match/Let spine. The oracle does not consult
// leaf_dispatch's depth arguments: it follows the evaluator's case_env =
// args ++ env and body_env = [value] ++ env rules.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Resolved {
    Word(String),
    Int(RuntimeIntV1),
    Constructor(String, Vec<Self>),
}

fn resolve(expr: &RuntimeExpr, env: &[Resolved]) -> Resolved {
    match expr {
        RuntimeExpr::Var(index) => env[*index as usize].clone(),
        RuntimeExpr::Value(RuntimeValue::String(word)) => Resolved::Word(word.clone()),
        RuntimeExpr::Value(RuntimeValue::Int(value)) => Resolved::Int(value.clone()),
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

struct SelectedHostFixture {
    semantic: checked_core::CheckedCoreSemanticInputs,
    spine: CheckedHostSpineV1,
    in_l: StableSymbol,
    in_r: StableSymbol,
    fs_op: StableSymbol,
    console_op: StableSymbol,
    clock_op: StableSymbol,
    entropy_op: StableSymbol,
}

fn selected_host_fixture() -> SelectedHostFixture {
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
        option_none: unused.clone(),
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
        resource_trace_identity: unused.clone(),
        nat_zero: unused.clone(),
        nat_suc: unused.clone(),
        private_buffer_span: unused.clone(),
        private_transfer_count: unused.clone(),
        read_some: unused.clone(),
        read_eof: unused.clone(),
        wrote: unused.clone(),
        mk_instant: unused.clone(),
        mk_monotonic_instant: unused.clone(),
        mk_deadline: unused.clone(),
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
        directory_entries: unused.clone(),
        directory_end: unused.clone(),
        resource_mapping_limit: unused.clone(),
        px10_exit_symbols: std::array::from_fn(|_| unused.clone()),
        resource_kind_symbols: std::array::from_fn(|_| unused.clone()),
        px11_address_symbols: std::array::from_fn(|_| unused.clone()),
        process_take_pipe_write_constructor: unused.clone(),
        bracket_finalizer_constructor: None,
        operations,
    };
    SelectedHostFixture {
        semantic,
        spine,
        in_l,
        in_r,
        fs_op,
        console_op,
        clock_op,
        entropy_op,
    }
}

#[test]
fn selected_host_leaf_free_var_resolves_through_actual_match_and_let_environments() {
    let SelectedHostFixture {
        semantic,
        spine,
        in_l,
        in_r,
        fs_op,
        console_op,
        clock_op,
        entropy_op,
    } = selected_host_fixture();
    let id = |name: &str| StableSymbol::declaration("selected-host-depth", &[], name);
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

// Follow the selected constructor's actual Match binders before observing the
// emitted Effect. Reading the raw Var indices alone would not establish that
// the runtime-selected child operand survives those binders.
fn selected_effect(expr: &RuntimeExpr, env: &[Resolved]) -> (ken_host::HostOpV1, Vec<Resolved>) {
    match expr {
        RuntimeExpr::Match {
            scrutinee, cases, ..
        } => {
            let Resolved::Constructor(constructor, mut args) = resolve(scrutinee, env) else {
                panic!("selected operation must resolve to a constructor")
            };
            let case = cases
                .iter()
                .find(|case| case.constructor == constructor)
                .expect("selected constructor must have a checked case");
            assert_eq!(args.len(), case.binders);
            args.extend_from_slice(env);
            selected_effect(&case.body, &args)
        }
        RuntimeExpr::Let { value, .. } => {
            let RuntimeExpr::Effect {
                operation, args, ..
            } = value.as_ref()
            else {
                panic!("selected host case must produce an Effect")
            };
            (
                *operation,
                args.iter().map(|arg| resolve(arg, env)).collect(),
            )
        }
        other => panic!("selected operation did not reach its Effect: {other:?}"),
    }
}

/// Promise class: durable invariant. MEASURED: the production runtime-selected
/// host dispatcher matches the checked stdin constructor, preserves the child
/// through its case binders, and appends fd 0; the sibling read constructor
/// preserves its independently supplied nonzero index instead of being zeroed.
/// CLAIMED: both constructors sharing ProcessTakePipe use their canonical
/// two-operand host request shape. THE GAP: this is a synthetic selected-host
/// operation; the checked-source native fixture separately covers static take.
#[test]
fn runtime_selected_take_stdin_preserves_child_and_adds_fd_zero() {
    let SelectedHostFixture {
        mut semantic,
        mut spine,
        ..
    } = selected_host_fixture();
    let write = StableSymbol::constructor(&spine.fs_family, "PrivateProcessTakePipeWrite");
    let read = StableSymbol::constructor(&spine.fs_family, "PrivateProcessTakePipeRead");
    spine.process_take_pipe_write_constructor = write.clone();
    let fs_metadata = semantic
        .data_metadata
        .get_mut(&spine.fs_family)
        .expect("the checked FS family is in the selected-host fixture");
    for (constructor, argument_count) in [(&write, 1), (&read, 2)] {
        spine
            .operations
            .insert(constructor.clone(), ken_host::HostOpV1::ProcessTakePipe);
        fs_metadata
            .constructors
            .push(checked_core::ConstructorMetadata {
                symbol: constructor.clone(),
                argument_count,
                target_index_count: 0,
                recursive_positions: Vec::new(),
                lowerability: checked_core::LowerabilityStatus::Supported,
            });
    }
    let word = |text: &str| RuntimeExpr::Value(RuntimeValue::String(text.into()));
    let select = |constructor: &StableSymbol, arguments| {
        let selected = lower_runtime_selected_host_operation(
            chosen_operation(
                &[spine.in_l.to_string(), constructor.to_string()],
                arguments,
            ),
            RuntimeExpr::Value(RuntimeValue::String("continuation".into())),
            &semantic,
            &spine,
            &StableSymbol::declaration("selected-host-depth", &[], "main"),
        )
        .expect("runtime-selected host producer must build both checked cases");
        selected_effect(&selected, &[])
    };
    let (write_op, write_args) = select(&write, vec![word("stdin-child")]);
    assert_eq!(write_op, ken_host::HostOpV1::ProcessTakePipe);
    assert_eq!(
        write_args,
        [
            Resolved::Word("stdin-child".into()),
            Resolved::Int(0.into())
        ]
    );

    let (read_op, read_args) = select(
        &read,
        vec![
            word("stdout-child"),
            RuntimeExpr::Value(RuntimeValue::Int(2.into())),
        ],
    );
    assert_eq!(read_op, ken_host::HostOpV1::ProcessTakePipe);
    assert_eq!(
        read_args,
        [
            Resolved::Word("stdout-child".into()),
            Resolved::Int(2.into())
        ]
    );
}
