//! Finished-function checked-frame accounting. Compiler traversal order is not
//! runtime path order: distinct successors may consume the same checked key.

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use cranelift_codegen::flowgraph::ControlFlowGraph;
use cranelift_codegen::ir::{types, Block, Function, Inst, InstructionData, Opcode, ValueDef};
use cranelift_frontend::FunctionBuilder;

use super::{unsupported, CraneliftBackendError, StaticOriginId};

pub(super) type FrameKey = (u64, u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum FrameEventKind {
    Activation,
    Receipt,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct FrameEvent {
    pub kind: FrameEventKind,
    pub key: FrameKey,
    pub block: Block,
    pub after: Option<Inst>,
    pub origin: Option<StaticOriginId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum FrameTerminalKind {
    Normal,
    Abort,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct FrameTerminal {
    pub block: Block,
    pub inst: Inst,
    pub kind: FrameTerminalKind,
    pub emitter: &'static std::panic::Location<'static>,
}

#[derive(Default)]
pub(super) struct FrameEvents {
    pub events: Vec<FrameEvent>,
    pub terminals: Vec<FrameTerminal>,
}

fn refusal(reason: impl Into<String>) -> CraneliftBackendError {
    unsupported("OrientedSubcontinuationPlanV1", reason)
}

// The generated-unit ABI returns exactly one I64 status. A nonzero constant
// return is an abort regardless of which emitter produced it; dynamic status
// returns and forgotten zero-success returns must still be registered.
fn classify_unregistered_terminal(
    func: &Function,
    block: Block,
) -> Result<FrameTerminalKind, CraneliftBackendError> {
    let inst = func.layout.last_inst(block).ok_or_else(|| {
        refusal(format!(
            "checked Runtime frame path ends in {block} without a terminal instruction",
        ))
    })?;
    match func.dfg.insts[inst].opcode() {
        Opcode::Trap => Ok(FrameTerminalKind::Abort),
        Opcode::Return => {
            let [status] = func.dfg.inst_args(inst) else {
                return Err(refusal(
                    "checked Runtime frame return has no single status operand",
                ));
            };
            let ValueDef::Result(def, _) = func.dfg.value_def(*status) else {
                return Err(refusal(format!(
                    "checked Runtime frame has unregistered dynamic-status return in {block}",
                )));
            };
            match func.dfg.insts[def] {
                InstructionData::UnaryImm {
                    opcode: Opcode::Iconst,
                    imm,
                } if imm.bits() != 0 => Ok(FrameTerminalKind::Abort),
                InstructionData::UnaryImm {
                    opcode: Opcode::Iconst,
                    imm,
                } if imm.bits() == 0 => Err(refusal(format!(
                    "checked Runtime frame has unregistered zero-status return in {block}",
                ))),
                _ => Err(refusal(format!(
                    "checked Runtime frame has unregistered dynamic-status return in {block}",
                ))),
            }
        }
        _ => Err(refusal(format!(
            "checked Runtime frame path has an unregistered terminal in {block}",
        ))),
    }
}

impl FrameEvents {
    pub fn record(
        &mut self,
        builder: &FunctionBuilder<'_>,
        kind: FrameEventKind,
        key: FrameKey,
        origin: Option<StaticOriginId>,
    ) -> Result<(), CraneliftBackendError> {
        let block = builder.current_block().ok_or_else(|| {
            refusal("checked Runtime frame marker event has no current Function block")
        })?;
        self.events.push(FrameEvent {
            kind,
            key,
            block,
            after: builder.func.layout.last_inst(block),
            origin,
        });
        Ok(())
    }

    #[track_caller]
    pub fn terminal(
        &mut self,
        builder: &FunctionBuilder<'_>,
        kind: FrameTerminalKind,
    ) -> Result<(), CraneliftBackendError> {
        let block = builder.current_block().ok_or_else(|| {
            refusal("checked Runtime frame terminal has no current Function block")
        })?;
        let inst =
            builder.func.layout.last_inst(block).ok_or_else(|| {
                refusal("checked Runtime frame terminal has no emitted instruction")
            })?;
        self.terminals.push(FrameTerminal {
            block,
            inst,
            kind,
            emitter: std::panic::Location::caller(),
        });
        Ok(())
    }

    // Diagnostic only: count generated Function sites reached from CFG entry,
    // including sites with no checked events. The registered class comes from
    // the emitting call, never from the status word's runtime value.
    fn trace_dynamic_status_returns(&self, func: &Function) {
        let Some(entry) = func.layout.entry_block() else {
            return;
        };
        let cfg = ControlFlowGraph::with_function(func);
        let mut seen = BTreeSet::new();
        let mut queue = VecDeque::from([entry]);
        let mut count = 0usize;
        while let Some(block) = queue.pop_front() {
            if !seen.insert(block) {
                continue;
            }
            if let Some(inst) = func.layout.last_inst(block) {
                if func.dfg.insts[inst].opcode() == Opcode::Return {
                    let dynamic = match func.dfg.inst_args(inst) {
                        [status] => match func.dfg.value_def(*status) {
                            ValueDef::Result(def, _) => !matches!(
                                func.dfg.insts[def],
                                InstructionData::UnaryImm {
                                    opcode: Opcode::Iconst,
                                    ..
                                }
                            ),
                            _ => true,
                        },
                        _ => true,
                    };
                    if dynamic {
                        count += 1;
                        let registration = self
                            .terminals
                            .iter()
                            .find(|terminal| terminal.block == block)
                            .map(|terminal| (terminal.kind, terminal.emitter));
                        eprintln!("KEN_FRAME_DYNAMIC_STATUS function={} block={block} registration={registration:?}", func.name);
                    }
                }
            }
            queue.extend(cfg.succ_iter(block));
        }
        eprintln!(
            "KEN_FRAME_DYNAMIC_STATUS_SUM function={} sites={count}",
            func.name
        );
    }

    pub fn validate(&self, func: &Function) -> Result<(), CraneliftBackendError> {
        if func.signature.returns.len() != 1 || func.signature.returns[0].value_type != types::I64 {
            return Err(refusal(
                "checked Runtime frame Function must return exactly one I64 status",
            ));
        }
        if std::env::var_os("KEN_FRAME_TERMINAL_CENSUS").is_some() {
            self.trace_dynamic_status_returns(func);
        }
        if self.events.is_empty() {
            return Ok(());
        }
        let cfg = ControlFlowGraph::with_function(func);
        let entry = func
            .layout
            .entry_block()
            .ok_or_else(|| refusal("checked Runtime frame Function has no entry block"))?;
        let mut events: BTreeMap<Block, Vec<(usize, usize, FrameEvent)>> = BTreeMap::new();
        for (sequence, event) in self.events.iter().copied().enumerate() {
            let mut position = 0;
            if let Some(anchor) = event.after {
                let mut found = false;
                for inst in func.layout.block_insts(event.block) {
                    position += 1;
                    if inst == anchor {
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Err(refusal(
                        "checked Runtime frame event position is not in its block",
                    ));
                }
            }
            events
                .entry(event.block)
                .or_default()
                .push((position, sequence, event));
        }
        for block_events in events.values_mut() {
            block_events.sort_by_key(|(position, sequence, _)| (*position, *sequence));
        }
        let mut terminals = BTreeMap::new();
        for terminal in &self.terminals {
            if func.layout.last_inst(terminal.block) != Some(terminal.inst)
                || !matches!(
                    func.dfg.insts[terminal.inst].opcode(),
                    Opcode::Return | Opcode::Trap
                )
                || terminals.insert(terminal.block, terminal.kind).is_some()
            {
                return Err(refusal(
                    "checked Runtime frame terminal registration is not unique or terminal",
                ));
            }
        }
        let keys: BTreeSet<_> = self.events.iter().map(|event| event.key).collect();
        for key in keys {
            // 0: no activation on this path, 1: active with no receipt,
            // 2: active with one receipt. A second activation or receipt on
            // the same path refuses, including revisiting its event on a cycle.
            let mut queue = VecDeque::from([(entry, 0u8)]);
            let mut visited = BTreeSet::new();
            while let Some((block, mut state)) = queue.pop_front() {
                if !visited.insert((block, state)) {
                    continue;
                }
                if let Some(block_events) = events.get(&block) {
                    for (_, _, event) in block_events {
                        if event.key != key {
                            continue;
                        }
                        match event.kind {
                            FrameEventKind::Activation => {
                                if state != 0 {
                                    return Err(refusal("checked Runtime frame marker was activated more than once on one path"));
                                }
                                state = 1;
                            }
                            FrameEventKind::Receipt => {
                                if state == 0 {
                                    return Err(refusal(
                                        "checked Runtime frame receipt has no activation",
                                    ));
                                }
                                if state == 2 {
                                    return Err(refusal("checked Runtime frame marker was consumed more than once on one path"));
                                }
                                state = 2;
                            }
                        }
                    }
                }
                let successors: Vec<_> = cfg.succ_iter(block).collect();
                if successors.is_empty() {
                    if state != 0 {
                        let kind = match terminals.get(&block) {
                            Some(kind) => *kind,
                            None => classify_unregistered_terminal(func, block)?,
                        };
                        match kind {
                            FrameTerminalKind::Normal if state == 1 => {
                                return Err(refusal(
                                    "checked Runtime frame marker was skipped on a normal return",
                                ));
                            }
                            FrameTerminalKind::Normal | FrameTerminalKind::Abort => {}
                        }
                    }
                } else {
                    for successor in successors {
                        queue.push_back((successor, state));
                    }
                }
            }
        }
        Ok(())
    }
}
