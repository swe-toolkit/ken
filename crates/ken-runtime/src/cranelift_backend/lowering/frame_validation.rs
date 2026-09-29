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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum FrameRule {
    E1,
    E2,
    R1,
    R2,
    N1,
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
        let started = std::time::Instant::now();
        let violations = self.rule_violations(func)?;
        if std::env::var_os("KEN_FRAME_CENSUS").is_some() {
            let cfg = ControlFlowGraph::with_function(func);
            let blocks = func.layout.blocks().count();
            let edges: usize = func
                .layout
                .blocks()
                .map(|block| cfg.succ_iter(block).count())
                .sum();
            let activations = self
                .events
                .iter()
                .filter(|event| event.kind == FrameEventKind::Activation)
                .count();
            let receipts = self.events.len() - activations;
            let mut normal = 0usize;
            let mut abort = 0usize;
            let mut unclassified = 0usize;
            for block in func.layout.blocks() {
                let Some(inst) = func.layout.last_inst(block) else {
                    continue;
                };
                if !matches!(func.dfg.insts[inst].opcode(), Opcode::Return | Opcode::Trap) {
                    continue;
                }
                let kind = self
                    .terminals
                    .iter()
                    .find(|terminal| terminal.block == block)
                    .map(|terminal| terminal.kind)
                    .map(Ok)
                    .unwrap_or_else(|| classify_unregistered_terminal(func, block));
                match kind {
                    Ok(FrameTerminalKind::Normal) => normal += 1,
                    Ok(FrameTerminalKind::Abort) => abort += 1,
                    Err(_) => unclassified += 1,
                }
            }
            eprintln!("KEN_FRAME_METRICS function={} keys={} activations={activations} receipts={receipts} blocks={blocks} edges={edges} normal_terminals={normal} abort_terminals={abort} unclassified_terminals={unclassified} validator_micros={}",
                func.name, violations.len(), started.elapsed().as_micros());
        }
        if violations.values().all(BTreeSet::is_empty) {
            Ok(())
        } else {
            Err(refusal(format!(
                "checked Runtime frame violations: {violations:?}"
            )))
        }
    }

    // Forward possible-state dataflow, per unchanged checked frame key. Each
    // element represents a path that can arrive at a program point: Inactive,
    // Active, or Discharged. Union at joins preserves skipped-arm obligations.
    pub(super) fn rule_violations(
        &self,
        func: &Function,
    ) -> Result<BTreeMap<FrameKey, BTreeSet<FrameRule>>, CraneliftBackendError> {
        if func.signature.returns.len() != 1 || func.signature.returns[0].value_type != types::I64 {
            return Err(refusal(
                "checked Runtime frame Function must return exactly one I64 status",
            ));
        }
        if std::env::var_os("KEN_FRAME_TERMINAL_CENSUS").is_some()
            || std::env::var_os("KEN_FRAME_CENSUS").is_some()
        {
            self.trace_dynamic_status_returns(func);
        }
        if self.events.is_empty() {
            return Ok(BTreeMap::new());
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
        const I: u8 = 0b001;
        const A: u8 = 0b010;
        const D: u8 = 0b100;
        let keys: BTreeSet<_> = self.events.iter().map(|event| event.key).collect();
        let mut violations = BTreeMap::new();
        let mut keys_needing_extra_passes = 0usize;
        for key in keys {
            let mut rules = BTreeSet::new();
            let mut incoming = BTreeMap::from([(entry, I)]);
            let mut queue = VecDeque::from([entry]);
            let mut queued = BTreeSet::from([entry]);
            let mut processed = BTreeSet::new();
            let mut extra_pass = false;
            while let Some(block) = queue.pop_front() {
                queued.remove(&block);
                if !processed.insert(block) {
                    extra_pass = true;
                }
                let mut state = incoming[&block];
                if let Some(block_events) = events.get(&block) {
                    for (_, _, event) in block_events {
                        if event.key != key {
                            continue;
                        }
                        match event.kind {
                            FrameEventKind::Activation => {
                                if state & A != 0 {
                                    rules.insert(FrameRule::E1);
                                }
                                if state & D != 0 {
                                    rules.insert(FrameRule::E2);
                                }
                                state = A;
                            }
                            FrameEventKind::Receipt => {
                                if state & I != 0 {
                                    rules.insert(FrameRule::R1);
                                }
                                if state & D != 0 {
                                    rules.insert(FrameRule::R2);
                                }
                                state = D;
                            }
                        }
                    }
                }
                let successors: Vec<_> = cfg.succ_iter(block).collect();
                if successors.is_empty() {
                    if state & (A | D) != 0 {
                        let kind = match terminals.get(&block) {
                            Some(kind) => *kind,
                            None => classify_unregistered_terminal(func, block)?,
                        };
                        // No checked source route activates a frame before a
                        // fanout at this base (measured: paired activation/
                        // receipt sites in px7n and a bypass variant). N1 is
                        // pinned by a hand-built finished-Function negative
                        // and its mutation. A future lowering may separate
                        // these events; its normal-return skip must refuse.
                        if kind == FrameTerminalKind::Normal && state & A != 0 {
                            rules.insert(FrameRule::N1);
                        }
                    }
                } else {
                    for successor in successors {
                        let previous = incoming.get(&successor).copied().unwrap_or(0);
                        let joined = previous | state;
                        if joined != previous {
                            incoming.insert(successor, joined);
                            if queued.insert(successor) {
                                queue.push_back(successor);
                            }
                        }
                    }
                }
            }
            if extra_pass {
                keys_needing_extra_passes += 1;
            }
            violations.insert(key, rules);
        }
        if std::env::var_os("KEN_FRAME_TERMINAL_CENSUS").is_some()
            || std::env::var_os("KEN_FRAME_CENSUS").is_some()
        {
            eprintln!("KEN_FRAME_FIXPOINT function={} keys_needing_extra_passes={keys_needing_extra_passes} total_keys={}", func.name, violations.len());
        }
        Ok(violations)
    }
}
