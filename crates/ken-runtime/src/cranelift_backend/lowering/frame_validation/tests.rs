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
        .record(builder, kind, KEY)
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

// Token-shaped keys exercise the exact same finished-Function engine in a
// separate key space. Production supplies the opaque full
// ContinuationCallIdentity; the last field here stands for its recursive
// position, and must participate in ordering rather than be erased.
type TokenTestKey = (&'static str, u32);
const TOKEN: TokenTestKey = ("complete planner identity", 1);

fn token_fixture() -> (FrameEvents<TokenTestKey>, Function) {
    let (frames, func) = build(Shape::ExclusiveActivations);
    let tokens = FrameEvents {
        events: frames.events.into_iter().map(|event| FrameEvent {
            kind: event.kind, key: TOKEN, block: event.block, after: event.after,
        }).collect(),
        terminals: frames.terminals,
    };
    (tokens, func)
}

fn token_rule(events: &FrameEvents<TokenTestKey>, func: &Function, expected: &[FrameRule]) {
    let expected: BTreeSet<_> = expected.iter().copied().collect();
    let actual = events.rule_violations(func).expect("finished token Function");
    assert_eq!(actual.len(), 1, "no other identity may hide a rule");
    assert_eq!(actual.get(&TOKEN), Some(&expected));
    let checked = events.validate_named(func, "continuation call token");
    if expected.is_empty() {
        checked.expect("exclusive token arms must pass");
    } else {
        let reason = format!("{:?}", checked.expect_err("same-path token must refuse"));
        assert!(reason.contains("continuation call token violations:"));
        assert!(reason.contains(&format!("{expected:?}")), "wrong refusal: {reason}");
    }
}

#[test]
fn token_exclusive_arms_pass_and_flat_restoration_would_refuse() {
    let (mut events, func) = token_fixture();
    token_rule(&events, &func, &[]);
    let flat_activations = events.events.iter().filter(|event| {
        event.key == TOKEN && event.kind == FrameEventKind::Activation
    }).count();
    assert_eq!(flat_activations, 2, "both exclusive arms must claim");
    // Restoring the old compile-wide insertion test rejects the same key on
    // its second visit despite the CFG witness above admitting both paths.
    assert!(!events.events.iter().filter(|event| {
        event.key == TOKEN && event.kind == FrameEventKind::Activation
    }).map(|event| event.key).collect::<Vec<_>>().windows(2)
      .all(|window| window[0] != window[1]));
    // Population-side perturbation: one arm no longer discharges at all.
    events.events.remove(3);
    token_rule(&events, &func, &[FrameRule::N1]);
}

#[test]
fn token_e1_second_enter_on_one_arm() {
    let (mut events, func) = token_fixture();
    token_rule(&events, &func, &[]);
    events.events.insert(1, events.events[0].clone());
    token_rule(&events, &func, &[FrameRule::E1]);
}

#[test]
fn token_e2_enter_after_discharge_on_one_arm() {
    let (mut events, func) = token_fixture();
    token_rule(&events, &func, &[]);
    let mut enter = events.events[0].clone();
    enter.after = events.events[1].after;
    let mut receipt = enter.clone();
    receipt.kind = FrameEventKind::Receipt;
    events.events.extend([enter, receipt]);
    token_rule(&events, &func, &[FrameRule::E2]);
}

#[test]
fn token_r1_receipt_without_enter_on_one_arm() {
    let (mut events, func) = token_fixture();
    token_rule(&events, &func, &[]);
    events.events.remove(0);
    token_rule(&events, &func, &[FrameRule::R1]);
}

#[test]
fn token_r2_second_direct_receipt_on_one_arm() {
    let (mut events, func) = token_fixture();
    token_rule(&events, &func, &[]);
    events.events.insert(2, events.events[1].clone());
    token_rule(&events, &func, &[FrameRule::R2]);
}

#[test]
fn token_n1_normal_path_without_receipt() {
    let (mut events, func) = token_fixture();
    token_rule(&events, &func, &[]);
    events.events.remove(1);
    token_rule(&events, &func, &[FrameRule::N1]);
}

#[test]
fn token_recursive_position_remains_part_of_the_key() {
    let (mut events, func) = token_fixture();
    token_rule(&events, &func, &[]);
    let other = (TOKEN.0, TOKEN.1 + 1);
    events.events[1].key = other;
    let actual = events.rule_violations(&func).expect("finished token Function");
    assert_eq!(actual.get(&TOKEN), Some(&BTreeSet::from([FrameRule::N1])));
    assert_eq!(actual.get(&other), Some(&BTreeSet::from([FrameRule::R1])));
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
