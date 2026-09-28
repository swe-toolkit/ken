//! Finished-function checked-frame accounting. Compiler traversal order is not
//! runtime path order: distinct successors may consume the same checked key.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use cranelift_codegen::flowgraph::ControlFlowGraph;
use cranelift_codegen::ir::{Block, Function, Inst, Opcode};
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
}

#[derive(Default)]
pub(super) struct FrameEvents {
    pub events: Vec<FrameEvent>,
    pub terminals: Vec<FrameTerminal>,
}

fn refusal(reason: &str) -> CraneliftBackendError {
    unsupported("OrientedSubcontinuationPlanV1", reason)
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

    pub fn terminal(
        &mut self,
        builder: &FunctionBuilder<'_>,
        kind: FrameTerminalKind,
    ) -> Result<(), CraneliftBackendError> {
        let block = builder.current_block().ok_or_else(|| {
            refusal("checked Runtime frame terminal has no current Function block")
        })?;
        let inst = builder.func.layout.last_inst(block).ok_or_else(|| {
            refusal("checked Runtime frame terminal has no emitted instruction")
        })?;
        self.terminals.push(FrameTerminal { block, inst, kind });
        Ok(())
    }

    pub fn validate(&self, func: &Function) -> Result<(), CraneliftBackendError> {
        if self.events.is_empty() {
            return Ok(());
        }
        let cfg = ControlFlowGraph::with_function(func);
        let entry = func.layout.entry_block().ok_or_else(|| {
            refusal("checked Runtime frame Function has no entry block")
        })?;
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
                    return Err(refusal("checked Runtime frame event position is not in its block"));
                }
            }
            events.entry(event.block).or_default().push((position, sequence, event));
        }
        for block_events in events.values_mut() {
            block_events.sort_by_key(|(position, sequence, _)| (*position, *sequence));
        }
        let mut terminals = BTreeMap::new();
        for terminal in &self.terminals {
            if func.layout.last_inst(terminal.block) != Some(terminal.inst)
                || !matches!(func.dfg.insts[terminal.inst].opcode(), Opcode::Return | Opcode::Trap)
                || terminals.insert(terminal.block, terminal.kind).is_some()
            {
                return Err(refusal("checked Runtime frame terminal registration is not unique or terminal"));
            }
        }
        let keys: BTreeSet<_> = self.events.iter().map(|event| event.key).collect();
        for key in keys {
            // 0: no activation on this path, 1: active with no receipt,
            // 2: active with one receipt. An activation closes the preceding
            // segment before starting the next, including a loop iteration.
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
                                if state == 1 {
                                    return Err(refusal("checked Runtime frame marker was skipped before reactivation"));
                                }
                                state = 1;
                            }
                            FrameEventKind::Receipt => {
                                if state == 0 {
                                    return Err(refusal("checked Runtime frame receipt has no activation"));
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
                        let Some(kind) = terminals.get(&block) else {
                            return Err(refusal("checked Runtime frame path has an unregistered terminal"));
                        };
                        if state == 1 {
                            return Err(refusal("checked Runtime frame marker was skipped on a terminal path"));
                        }
                        let _ = kind; // Both terminal kinds close an already-consumed segment.
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
