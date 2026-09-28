use cranelift_codegen::ir::{types, AbiParam, Function, InstBuilder};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};

use super::*;

const KEY: FrameKey = (0, 0);

#[derive(Clone, Copy)]
enum Shape {
    ExclusiveReceipts,
    ReceiptOrAbort,
    ActiveAbort,
    ExclusiveActivations,
    DoubleEnter,
    EnterAfterDischarge,
    ReceiptWithoutEnter,
    DoubleReceipt,
    OneArmSkip,
    EnterCycle,
    ReceiptCycle,
    UnregisteredZero,
    UnregisteredDynamic,
    UnregisteredNonzero,
    UnregisteredTrap,
    InvalidSignature,
}

fn event(builder: &FunctionBuilder<'_>, events: &mut FrameEvents, kind: FrameEventKind) {
    events
        .record(builder, kind, KEY, None)
        .expect("event position");
}

fn ret(
    builder: &mut FunctionBuilder<'_>,
    events: &mut FrameEvents,
    kind: Option<FrameTerminalKind>,
) {
    let zero = builder.ins().iconst(types::I64, 0);
    builder.ins().return_(&[zero]);
    if let Some(kind) = kind {
        events.terminal(builder, kind).expect("return position");
    }
}

fn abort(builder: &mut FunctionBuilder<'_>, events: &mut FrameEvents) {
    let failure = builder.ins().iconst(types::I64, -1);
    builder.ins().return_(&[failure]);
    // Exercise ABI classification rather than emitter registration.
    let _ = events;
}

fn build(shape: Shape) -> (FrameEvents, Function) {
    let mut func = Function::new();
    if matches!(shape, Shape::UnregisteredDynamic) {
        func.signature.params.push(AbiParam::new(types::I64));
    }
    func.signature
        .returns
        .push(AbiParam::new(if matches!(shape, Shape::InvalidSignature) {
            types::I32
        } else {
            types::I64
        }));
    let mut context = FunctionBuilderContext::new();
    let mut events = FrameEvents::default();
    {
        let mut builder = FunctionBuilder::new(&mut func, &mut context);
        let entry = builder.create_block();
        if matches!(shape, Shape::UnregisteredDynamic) {
            builder.append_block_params_for_function_params(entry);
        }
        builder.switch_to_block(entry);
        match shape {
            Shape::ExclusiveReceipts | Shape::ReceiptOrAbort | Shape::OneArmSkip => {
                event(&builder, &mut events, FrameEventKind::Activation);
                let left = builder.create_block();
                let right = builder.create_block();
                let merge =
                    (!matches!(shape, Shape::ReceiptOrAbort)).then(|| builder.create_block());
                let cond = builder.ins().iconst(types::I8, 1);
                builder.ins().brif(cond, left, &[], right, &[]);
                builder.switch_to_block(left);
                event(&builder, &mut events, FrameEventKind::Receipt);
                if matches!(shape, Shape::ReceiptOrAbort) {
                    ret(&mut builder, &mut events, Some(FrameTerminalKind::Normal));
                } else {
                    builder.ins().jump(merge.expect("merging branch"), &[]);
                }
                builder.switch_to_block(right);
                match shape {
                    Shape::ExclusiveReceipts => {
                        event(&builder, &mut events, FrameEventKind::Receipt);
                        builder.ins().jump(merge.expect("merging branch"), &[]);
                    }
                    Shape::ReceiptOrAbort => abort(&mut builder, &mut events),
                    Shape::OneArmSkip => {
                        builder.ins().jump(merge.expect("merging branch"), &[]);
                    }
                    _ => unreachable!(),
                }
                if let Some(merge) = merge {
                    builder.switch_to_block(merge);
                    ret(&mut builder, &mut events, Some(FrameTerminalKind::Normal));
                }
            }
            Shape::ActiveAbort | Shape::UnregisteredNonzero => {
                event(&builder, &mut events, FrameEventKind::Activation);
                abort(&mut builder, &mut events);
            }
            Shape::ExclusiveActivations => {
                let left = builder.create_block();
                let right = builder.create_block();
                let merge = builder.create_block();
                let cond = builder.ins().iconst(types::I8, 1);
                builder.ins().brif(cond, left, &[], right, &[]);
                for arm in [left, right] {
                    builder.switch_to_block(arm);
                    event(&builder, &mut events, FrameEventKind::Activation);
                    event(&builder, &mut events, FrameEventKind::Receipt);
                    builder.ins().jump(merge, &[]);
                }
                builder.switch_to_block(merge);
                ret(&mut builder, &mut events, Some(FrameTerminalKind::Normal));
            }
            Shape::DoubleEnter | Shape::EnterAfterDischarge | Shape::DoubleReceipt => {
                event(&builder, &mut events, FrameEventKind::Activation);
                match shape {
                    Shape::DoubleEnter => event(&builder, &mut events, FrameEventKind::Activation),
                    Shape::EnterAfterDischarge | Shape::DoubleReceipt => {
                        event(&builder, &mut events, FrameEventKind::Receipt);
                    }
                    _ => unreachable!(),
                }
                if matches!(shape, Shape::EnterAfterDischarge) {
                    event(&builder, &mut events, FrameEventKind::Activation);
                }
                if matches!(shape, Shape::DoubleReceipt) {
                    event(&builder, &mut events, FrameEventKind::Receipt);
                }
                if !matches!(shape, Shape::DoubleReceipt) {
                    event(&builder, &mut events, FrameEventKind::Receipt);
                }
                ret(&mut builder, &mut events, Some(FrameTerminalKind::Normal));
            }
            Shape::ReceiptWithoutEnter => {
                event(&builder, &mut events, FrameEventKind::Receipt);
                ret(&mut builder, &mut events, Some(FrameTerminalKind::Normal));
            }
            Shape::EnterCycle | Shape::ReceiptCycle => {
                let cycle = builder.create_block();
                let exit = builder.create_block();
                if matches!(shape, Shape::ReceiptCycle) {
                    event(&builder, &mut events, FrameEventKind::Activation);
                }
                builder.ins().jump(cycle, &[]);
                builder.switch_to_block(cycle);
                if matches!(shape, Shape::EnterCycle) {
                    event(&builder, &mut events, FrameEventKind::Activation);
                } else {
                    event(&builder, &mut events, FrameEventKind::Receipt);
                }
                let cond = builder.ins().iconst(types::I8, 1);
                builder.ins().brif(cond, cycle, &[], exit, &[]);
                builder.switch_to_block(exit);
                if matches!(shape, Shape::EnterCycle) {
                    event(&builder, &mut events, FrameEventKind::Receipt);
                }
                ret(&mut builder, &mut events, Some(FrameTerminalKind::Normal));
            }
            Shape::UnregisteredZero => {
                event(&builder, &mut events, FrameEventKind::Activation);
                event(&builder, &mut events, FrameEventKind::Receipt);
                ret(&mut builder, &mut events, None);
            }
            Shape::UnregisteredDynamic => {
                event(&builder, &mut events, FrameEventKind::Activation);
                let status = builder.block_params(entry)[0];
                builder.ins().return_(&[status]);
            }
            Shape::UnregisteredTrap => {
                event(&builder, &mut events, FrameEventKind::Activation);
                builder
                    .ins()
                    .trap(cranelift_codegen::ir::TrapCode::unwrap_user(73));
            }
            Shape::InvalidSignature => {
                event(&builder, &mut events, FrameEventKind::Activation);
                let zero = builder.ins().iconst(types::I32, 0);
                builder.ins().return_(&[zero]);
                events
                    .terminal(&builder, FrameTerminalKind::Normal)
                    .expect("return position");
            }
        }
        builder.seal_all_blocks();
        builder.finalize();
    }
    (events, func)
}

fn expect_rules(shape: Shape, expected: &[FrameRule]) {
    let (events, func) = build(shape);
    let actual = events
        .rule_violations(&func)
        .expect("classified finished Function");
    let expected_set: BTreeSet<_> = expected.iter().copied().collect();
    assert_eq!(actual.get(&KEY), Some(&expected_set));
    assert_eq!(events.validate(&func).is_ok(), expected.is_empty());
}

#[test]
fn p1_exclusive_receipts_join() {
    expect_rules(Shape::ExclusiveReceipts, &[]);
}
#[test]
fn p2_receipt_or_abort() {
    expect_rules(Shape::ReceiptOrAbort, &[]);
}
#[test]
fn p3_active_abort() {
    expect_rules(Shape::ActiveAbort, &[]);
}
#[test]
fn p4_exclusive_activations_join() {
    expect_rules(Shape::ExclusiveActivations, &[]);
}
#[test]
fn n_e1_double_enter() {
    expect_rules(Shape::DoubleEnter, &[FrameRule::E1]);
}
#[test]
fn n_e2_enter_after_discharge() {
    expect_rules(Shape::EnterAfterDischarge, &[FrameRule::E2]);
}
#[test]
fn n_r1_receipt_without_enter() {
    expect_rules(Shape::ReceiptWithoutEnter, &[FrameRule::R1]);
}
#[test]
fn n_r2_double_receipt() {
    expect_rules(Shape::DoubleReceipt, &[FrameRule::R2]);
}
#[test]
fn n_n1_one_arm_skip() {
    expect_rules(Shape::OneArmSkip, &[FrameRule::N1]);
}
#[test]
fn n_e1c_activation_cycle() {
    expect_rules(Shape::EnterCycle, &[FrameRule::E1]);
}
#[test]
fn n_r2c_receipt_cycle() {
    expect_rules(Shape::ReceiptCycle, &[FrameRule::R2]);
}

#[test]
fn unregistered_zero_status_return_refuses() {
    let (events, func) = build(Shape::UnregisteredZero);
    assert!(format!("{:?}", events.validate(&func).unwrap_err())
        .contains("unregistered zero-status return"));
}
#[test]
fn unregistered_dynamic_status_return_refuses() {
    let (events, func) = build(Shape::UnregisteredDynamic);
    assert!(format!("{:?}", events.validate(&func).unwrap_err())
        .contains("unregistered dynamic-status return"));
}
#[test]
fn unregistered_nonzero_status_return_without_receipt_is_abort() {
    expect_rules(Shape::UnregisteredNonzero, &[]);
}
#[test]
fn unregistered_clif_trap_without_receipt_is_abort() {
    expect_rules(Shape::UnregisteredTrap, &[]);
}
#[test]
fn non_i64_status_signature_refuses() {
    let (events, func) = build(Shape::InvalidSignature);
    assert!(format!("{:?}", events.validate(&func).unwrap_err()).contains("exactly one I64 status"));
}
