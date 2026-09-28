use cranelift_codegen::ir::{types, AbiParam, Function, InstBuilder};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};

use super::*;

const KEY: FrameKey = (0, 0);

#[derive(Clone, Copy)]
enum Shape {
    ExclusiveArms,
    ZeroReceiptAbortArm,
    SameBlockDuplicate,
    SequentialBlockDuplicate,
    OneArmSkip,
    ReceiptWithoutActivation,
    UnregisteredZeroStatus,
    UnregisteredDynamicStatus,
    UnregisteredNonzeroStatus,
    UnregisteredTrap,
    InvalidSignature,
    ActivationCycle,
    ReceiptCycle,
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

fn exercise(shape: Shape) -> Result<(), CraneliftBackendError> {
    let mut func = Function::new();
    if matches!(shape, Shape::UnregisteredDynamicStatus) {
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
        if matches!(shape, Shape::UnregisteredDynamicStatus) {
            builder.append_block_params_for_function_params(entry);
        }
        builder.switch_to_block(entry);
        match shape {
            Shape::ExclusiveArms | Shape::ZeroReceiptAbortArm | Shape::OneArmSkip => {
                event(&builder, &mut events, FrameEventKind::Activation);
                let left = builder.create_block();
                let right = builder.create_block();
                let cond = builder.ins().iconst(types::I8, 1);
                builder.ins().brif(cond, left, &[], right, &[]);
                builder.switch_to_block(left);
                event(&builder, &mut events, FrameEventKind::Receipt);
                ret(&mut builder, &mut events, Some(FrameTerminalKind::Normal));
                builder.switch_to_block(right);
                match shape {
                    Shape::ExclusiveArms => event(&builder, &mut events, FrameEventKind::Receipt),
                    Shape::ZeroReceiptAbortArm | Shape::OneArmSkip => {}
                    _ => unreachable!(),
                }
                ret(
                    &mut builder,
                    &mut events,
                    Some(if matches!(shape, Shape::ZeroReceiptAbortArm) {
                        FrameTerminalKind::Abort
                    } else {
                        FrameTerminalKind::Normal
                    }),
                );
            }
            Shape::SameBlockDuplicate => {
                event(&builder, &mut events, FrameEventKind::Activation);
                event(&builder, &mut events, FrameEventKind::Receipt);
                event(&builder, &mut events, FrameEventKind::Receipt);
                ret(&mut builder, &mut events, Some(FrameTerminalKind::Normal));
            }
            Shape::SequentialBlockDuplicate => {
                event(&builder, &mut events, FrameEventKind::Activation);
                event(&builder, &mut events, FrameEventKind::Receipt);
                let second = builder.create_block();
                builder.ins().jump(second, &[]);
                builder.switch_to_block(second);
                event(&builder, &mut events, FrameEventKind::Receipt);
                ret(&mut builder, &mut events, Some(FrameTerminalKind::Normal));
            }
            Shape::ReceiptWithoutActivation => {
                event(&builder, &mut events, FrameEventKind::Receipt);
                ret(&mut builder, &mut events, Some(FrameTerminalKind::Normal));
            }
            Shape::UnregisteredZeroStatus => {
                event(&builder, &mut events, FrameEventKind::Activation);
                event(&builder, &mut events, FrameEventKind::Receipt);
                ret(&mut builder, &mut events, None);
            }
            Shape::UnregisteredDynamicStatus => {
                event(&builder, &mut events, FrameEventKind::Activation);
                let status = builder.block_params(entry)[0];
                builder.ins().return_(&[status]);
            }
            Shape::UnregisteredNonzeroStatus => {
                event(&builder, &mut events, FrameEventKind::Activation);
                let failure = builder.ins().iconst(types::I64, -1);
                builder.ins().return_(&[failure]);
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
                    .expect("normal return position");
            }
            Shape::ActivationCycle | Shape::ReceiptCycle => {
                let cycle = builder.create_block();
                let exit = builder.create_block();
                if matches!(shape, Shape::ReceiptCycle) {
                    event(&builder, &mut events, FrameEventKind::Activation);
                }
                builder.ins().jump(cycle, &[]);
                builder.switch_to_block(cycle);
                if matches!(shape, Shape::ActivationCycle) {
                    event(&builder, &mut events, FrameEventKind::Activation);
                }
                event(&builder, &mut events, FrameEventKind::Receipt);
                let cond = builder.ins().iconst(types::I8, 1);
                builder.ins().brif(cond, cycle, &[], exit, &[]);
                builder.switch_to_block(exit);
                ret(&mut builder, &mut events, Some(FrameTerminalKind::Normal));
            }
        }
        builder.seal_all_blocks();
        builder.finalize();
    }
    events.validate(&func)
}

#[test]
fn exclusive_successors_each_consume_once() {
    exercise(Shape::ExclusiveArms).expect("separate receipts are not a same-path duplicate");
}

#[test]
fn abort_arm_without_receipt_and_normal_arm_with_receipt_pass() {
    exercise(Shape::ZeroReceiptAbortArm).expect("abort may abandon the checked continuation");
}

#[test]
fn same_block_duplicate_receipts_refuse() {
    assert!(
        format!("{:?}", exercise(Shape::SameBlockDuplicate).unwrap_err())
            .contains("consumed more than once")
    );
}

#[test]
fn reachable_sequential_receipts_refuse() {
    assert!(format!(
        "{:?}",
        exercise(Shape::SequentialBlockDuplicate).unwrap_err()
    )
    .contains("consumed more than once"));
}

#[test]
fn one_arm_skip_to_normal_return_refuses() {
    assert!(format!("{:?}", exercise(Shape::OneArmSkip).unwrap_err())
        .contains("skipped on a normal return"));
}

#[test]
fn receipt_without_activation_refuses() {
    assert!(format!(
        "{:?}",
        exercise(Shape::ReceiptWithoutActivation).unwrap_err()
    )
    .contains("receipt has no activation"));
}

#[test]
fn unregistered_zero_status_return_refuses() {
    assert!(
        format!("{:?}", exercise(Shape::UnregisteredZeroStatus).unwrap_err())
            .contains("unregistered zero-status return")
    );
}

#[test]
fn unregistered_dynamic_status_return_refuses() {
    assert!(format!(
        "{:?}",
        exercise(Shape::UnregisteredDynamicStatus).unwrap_err()
    )
    .contains("unregistered dynamic-status return"));
}

#[test]
fn unregistered_nonzero_status_return_without_receipt_is_abort() {
    exercise(Shape::UnregisteredNonzeroStatus).expect("nonzero status is an ABI abort");
}

#[test]
fn unregistered_clif_trap_without_receipt_is_abort() {
    exercise(Shape::UnregisteredTrap).expect("CLIF trap is an abort");
}

#[test]
fn non_i64_status_signature_refuses() {
    assert!(
        format!("{:?}", exercise(Shape::InvalidSignature).unwrap_err())
            .contains("exactly one I64 status")
    );
}

#[test]
fn activation_on_cycle_refuses() {
    assert!(
        format!("{:?}", exercise(Shape::ActivationCycle).unwrap_err())
            .contains("activated more than once")
    );
}

#[test]
fn receipt_on_cycle_refuses() {
    assert!(format!("{:?}", exercise(Shape::ReceiptCycle).unwrap_err())
        .contains("consumed more than once"));
}
