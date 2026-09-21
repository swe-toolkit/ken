//! Typed, well-nested control graph for generated Cranelift units.
//!
//! This graph is deliberately produced from finished Cranelift functions. It
//! records opaque function-local node names only after every `Inst` has been
//! checked against the `Function` that owns it. No Cranelift entity, symbol,
//! declaration ordinal, or module function number crosses into the graph.

use std::collections::{BTreeMap, BTreeSet};

use cranelift_codegen::flowgraph::ControlFlowGraph;
use cranelift_codegen::ir::{ExternalName, Function, Inst, InstructionData, Opcode};
use cranelift_module::FuncId;

use super::planning::{
    ContinuationContextId, ContinuationSpecializationId, PredeclaredFunctionId,
    StaticContinuationFusionId, StaticOriginId, StaticResponseOwnerId,
};
use super::surface::{BackendFailure, CraneliftBackendError};

fn graph_error(reason: impl Into<String>) -> CraneliftBackendError {
    CraneliftBackendError::Backend(BackendFailure::Module(reason.into()))
}

/// The five sealed generated-unit families.
///
/// There is intentionally no open or numeric variant. Adding a sixth family
/// makes every exhaustive match over this type fail to compile until that
/// family's entry, exit, calls, returns and terminals are accounted for.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) enum GraftedSpineFunctionScope {
    Predeclared(PredeclaredFunctionId),
    Continuation(ContinuationSpecializationId),
    Response(StaticResponseOwnerId),
    ContinuationContext(ContinuationContextId),
    Fusion(StaticContinuationFusionId),
}

impl GraftedSpineFunctionScope {
    fn has_continuation_entry(self) -> bool {
        match self {
            Self::Predeclared(_) | Self::Response(_) | Self::Fusion(_) => false,
            Self::Continuation(_) | Self::ContinuationContext(_) => true,
        }
    }
}

/// An opaque node name meaningful only together with its typed function scope.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct GraftedSpineNode {
    scope: GraftedSpineFunctionScope,
    local: GraftedSpineLocalNodeId,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct GraftedSpineLocalNodeId(u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GraftedSpineNodeKind {
    FunctionEntry,
    ContinuationEntry,
    Body,
    CallerLocalCall,
    CallReturn { call: GraftedSpineLocalNodeId },
    FunctionExit,
    TerminalDispatch,
}

/// One generated-unit call observed at its actual `builder.ins().call` seam.
///
/// `inst` is transient: it is accepted only by `observe_emitted_function`,
/// validated against that one function, translated to an opaque local node,
/// and then dropped.
#[derive(Clone, Copy, Debug)]
pub(in crate::cranelift_backend) struct PendingGraftedSpineCall {
    inst: Inst,
    source: Option<StaticOriginId>,
}

impl PendingGraftedSpineCall {
    pub(in crate::cranelift_backend) fn new(inst: Inst, source: Option<StaticOriginId>) -> Self {
        Self { inst, source }
    }
}

/// One host terminal observed independently at the emitted dispatch call.
#[derive(Clone, Copy, Debug)]
pub(in crate::cranelift_backend) struct PendingGraftedSpineTerminal {
    inst: Inst,
    member: StaticOriginId,
    body: StaticOriginId,
    /// A value supplied from the existing `AbiSlotKind::Control` lane. The
    /// precursor never emits or transports one; `None` is the behavior-inert
    /// observation available before D0 installs its affine member words.
    control_word: Option<u64>,
}

impl PendingGraftedSpineTerminal {
    pub(in crate::cranelift_backend) fn new(
        inst: Inst,
        member: StaticOriginId,
        body: StaticOriginId,
    ) -> Self {
        Self {
            inst,
            member,
            body,
            control_word: None,
        }
    }

    /// Bind an observation to the word already carried in
    /// `AbiSlotKind::Control`. This records no second token and emits nothing.
    pub(in crate::cranelift_backend) fn with_control_word(
        inst: Inst,
        member: StaticOriginId,
        body: StaticOriginId,
        control_word: u64,
    ) -> Self {
        Self {
            inst,
            member,
            body,
            control_word: Some(control_word),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct IntraFunctionEdge {
    from: GraftedSpineLocalNodeId,
    to: GraftedSpineLocalNodeId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GraftedSpineCall {
    caller: GraftedSpineFunctionScope,
    source: Option<StaticOriginId>,
    call: GraftedSpineLocalNodeId,
    return_site: GraftedSpineLocalNodeId,
    target: GraftedSpineFunctionScope,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GraftedSpineTerminal {
    member: StaticOriginId,
    body: StaticOriginId,
    node: GraftedSpineNode,
    control_word: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GraftedSpineFunctionGraph {
    scope: GraftedSpineFunctionScope,
    entry: GraftedSpineLocalNodeId,
    continuation_entry: Option<GraftedSpineLocalNodeId>,
    exit: GraftedSpineLocalNodeId,
    nodes: BTreeMap<GraftedSpineLocalNodeId, GraftedSpineNodeKind>,
    flow: BTreeSet<IntraFunctionEdge>,
    calls: Vec<GraftedSpineCall>,
    terminals: Vec<GraftedSpineTerminal>,
}

impl GraftedSpineFunctionGraph {
    fn node(&self, local: GraftedSpineLocalNodeId) -> GraftedSpineNode {
        GraftedSpineNode {
            scope: self.scope,
            local,
        }
    }
}

/// One typed interprocedural edge. These edges are descriptive; realizability
/// is computed with matched call/return summaries rather than ordinary graph
/// transitive closure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) enum GraftedSpineTypedEdge {
    IntraFunction {
        from: GraftedSpineNode,
        to: GraftedSpineNode,
    },
    CallToEntry {
        call: GraftedSpineNode,
        entry: GraftedSpineNode,
    },
    CalleeExitToExactReturn {
        exit: GraftedSpineNode,
        call: GraftedSpineNode,
        return_site: GraftedSpineNode,
    },
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct GraftedSpineSeedEdge {
    pub(in crate::cranelift_backend) source: StaticOriginId,
    pub(in crate::cranelift_backend) call: GraftedSpineNode,
    pub(in crate::cranelift_backend) target: GraftedSpineFunctionScope,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::cranelift_backend) struct GraftedSpineRealizableSummary {
    pub(in crate::cranelift_backend) call: GraftedSpineNode,
    pub(in crate::cranelift_backend) return_site: GraftedSpineNode,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct GraftedSpineQuery {
    pub(in crate::cranelift_backend) seeds: Vec<GraftedSpineSeedEdge>,
    pub(in crate::cranelift_backend) terminal: GraftedSpineNode,
    pub(in crate::cranelift_backend) terminal_member: StaticOriginId,
    pub(in crate::cranelift_backend) terminal_body: StaticOriginId,
    pub(in crate::cranelift_backend) summaries: Vec<GraftedSpineRealizableSummary>,
}

#[cfg(any(test, feature = "px8-ds-test-support"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraftedSpineValidationMutation {
    Exact,
    SubstituteExpectedSource {
        member: u32,
        original: u32,
        replacement: u32,
    },
}

#[cfg(any(test, feature = "px8-ds-test-support"))]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GraftedSpineObservedTerminal {
    pub member: u32,
    pub body: u32,
    pub node: String,
    pub control_word: Option<u64>,
}

#[cfg(any(test, feature = "px8-ds-test-support"))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraftedSpineValidationObservation {
    pub member: u32,
    pub expected_source: u32,
    pub outcome: Result<(), String>,
}

#[cfg(any(test, feature = "px8-ds-test-support"))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraftedSpineActualLoweringObservation {
    pub topology: Vec<String>,
    pub terminals: Vec<GraftedSpineObservedTerminal>,
    pub validations: Vec<GraftedSpineValidationObservation>,
}

#[cfg(any(test, feature = "px8-ds-test-support"))]
thread_local! {
    static GRAFTED_SPINE_VALIDATION_MUTATION:
        std::cell::Cell<Option<GraftedSpineValidationMutation>> = const {
            std::cell::Cell::new(None)
        };
    static GRAFTED_SPINE_VALIDATION_MUTATION_APPLICATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
    static GRAFTED_SPINE_ACTUAL_LOWERING_OBSERVATIONS:
        std::cell::RefCell<Vec<GraftedSpineActualLoweringObservation>> = const {
            std::cell::RefCell::new(Vec::new())
        };
}

#[cfg(any(test, feature = "px8-ds-test-support"))]
pub fn with_grafted_spine_validation_mutation<T>(
    mutation: GraftedSpineValidationMutation,
    operation: impl FnOnce() -> T,
) -> (T, Vec<GraftedSpineActualLoweringObservation>, usize) {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            GRAFTED_SPINE_VALIDATION_MUTATION.with(|active| active.set(None));
            GRAFTED_SPINE_VALIDATION_MUTATION_APPLICATIONS.with(|count| count.set(0));
            GRAFTED_SPINE_ACTUAL_LOWERING_OBSERVATIONS
                .with(|observations| observations.borrow_mut().clear());
        }
    }

    GRAFTED_SPINE_ACTUAL_LOWERING_OBSERVATIONS
        .with(|observations| observations.borrow_mut().clear());
    GRAFTED_SPINE_VALIDATION_MUTATION_APPLICATIONS.with(|count| count.set(0));
    GRAFTED_SPINE_VALIDATION_MUTATION.with(|active| active.set(Some(mutation)));
    let reset = Reset;
    let result = operation();
    let observations = GRAFTED_SPINE_ACTUAL_LOWERING_OBSERVATIONS
        .with(|recorded| std::mem::take(&mut *recorded.borrow_mut()));
    let applications = GRAFTED_SPINE_VALIDATION_MUTATION_APPLICATIONS.with(std::cell::Cell::get);
    drop(reset);
    (result, observations, applications)
}

fn prepare_validation_inputs(
    validation_inputs: impl IntoIterator<Item = (StaticOriginId, StaticOriginId)>,
) -> Vec<(StaticOriginId, StaticOriginId)> {
    #[allow(unused_mut)]
    let mut inputs = validation_inputs
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    #[cfg(any(test, feature = "px8-ds-test-support"))]
    if let Some(mutation) = GRAFTED_SPINE_VALIDATION_MUTATION.with(std::cell::Cell::get) {
        for (member, expected_source) in &mut inputs {
            match mutation {
                GraftedSpineValidationMutation::Exact => {}
                GraftedSpineValidationMutation::SubstituteExpectedSource {
                    member: selected_member,
                    original,
                    replacement,
                } if member.observation_ordinal() == selected_member
                    && expected_source.observation_ordinal() == original =>
                {
                    *expected_source = StaticOriginId::for_validation_observation(replacement);
                    GRAFTED_SPINE_VALIDATION_MUTATION_APPLICATIONS.with(|count| {
                        count.set(
                            count
                                .get()
                                .checked_add(1)
                                .expect("the grafted-spine validation mutation count fits usize"),
                        )
                    });
                }
                GraftedSpineValidationMutation::SubstituteExpectedSource { .. } => {}
            }
        }
    }
    inputs
}

/// Artifact-wide graph builder. Its expected population is the declaration
/// bundle, while its observed population is the five body-definition passes.
#[derive(Default)]
pub(in crate::cranelift_backend) struct GraftedSpineControlGraphBuilder {
    expected: BTreeSet<GraftedSpineFunctionScope>,
    validation_inputs: Vec<(StaticOriginId, StaticOriginId)>,
    functions: BTreeMap<GraftedSpineFunctionScope, GraftedSpineFunctionGraph>,
}

impl GraftedSpineControlGraphBuilder {
    pub(in crate::cranelift_backend) fn new(
        expected: impl IntoIterator<Item = GraftedSpineFunctionScope>,
        validation_inputs: impl IntoIterator<Item = (StaticOriginId, StaticOriginId)>,
    ) -> Result<Self, CraneliftBackendError> {
        let mut closed = BTreeSet::new();
        for scope in expected {
            if !closed.insert(scope) {
                return Err(graph_error(format!(
                    "the grafted-spine declaration population repeats typed scope {scope:?}"
                )));
            }
        }
        Ok(Self {
            expected: closed,
            validation_inputs: prepare_validation_inputs(validation_inputs),
            functions: BTreeMap::new(),
        })
    }

    pub(in crate::cranelift_backend) fn observe_emitted_function(
        &mut self,
        scope: GraftedSpineFunctionScope,
        func: &Function,
        pending_calls: Vec<PendingGraftedSpineCall>,
        pending_terminals: Vec<PendingGraftedSpineTerminal>,
        resolve_target: impl Fn(FuncId) -> Option<GraftedSpineFunctionScope>,
    ) -> Result<(), CraneliftBackendError> {
        if !self.expected.contains(&scope) {
            return Err(graph_error(format!(
                "finished generated function {scope:?} was absent from the sealed declaration population"
            )));
        }
        if self.functions.contains_key(&scope) {
            return Err(graph_error(format!(
                "generated function {scope:?} was recorded twice in the grafted spine"
            )));
        }

        let layout_insts = func
            .layout
            .blocks()
            .flat_map(|block| func.layout.block_insts(block))
            .collect::<BTreeSet<_>>();
        let mut calls_by_inst = BTreeMap::new();
        for pending in pending_calls {
            if !layout_insts.contains(&pending.inst) {
                return Err(graph_error(
                    "a grafted-spine call instruction does not belong to its recorded Function",
                ));
            }
            if func.dfg.insts[pending.inst].opcode() != Opcode::Call {
                return Err(graph_error(
                    "a grafted-spine call observation does not name a direct call instruction",
                ));
            }
            if calls_by_inst.insert(pending.inst, pending).is_some() {
                return Err(graph_error(
                    "one caller-local instruction was recorded as two generated-unit calls",
                ));
            }
        }
        let mut terminals_by_inst = BTreeMap::new();
        for pending in pending_terminals {
            if !layout_insts.contains(&pending.inst) {
                return Err(graph_error(
                    "a grafted-spine terminal instruction does not belong to its recorded Function",
                ));
            }
            if func.dfg.insts[pending.inst].opcode() != Opcode::Call {
                return Err(graph_error(
                    "a grafted-spine terminal observation does not name a direct dispatch call",
                ));
            }
            terminals_by_inst
                .entry(pending.inst)
                .or_insert_with(Vec::new)
                .push(pending);
        }
        if calls_by_inst
            .keys()
            .any(|inst| terminals_by_inst.contains_key(inst))
        {
            return Err(graph_error(
                "one instruction cannot be both a generated-unit call and a host terminal",
            ));
        }

        let mut next = 0u32;
        let mut nodes = BTreeMap::new();
        let mut allocate = |kind, nodes: &mut BTreeMap<_, _>| {
            let id = GraftedSpineLocalNodeId(next);
            next = next.checked_add(1).ok_or_else(|| {
                graph_error("one generated function exhausted grafted-spine local node ids")
            })?;
            nodes.insert(id, kind);
            Ok::<_, CraneliftBackendError>(id)
        };
        let entry = allocate(GraftedSpineNodeKind::FunctionEntry, &mut nodes)?;
        let continuation_entry = scope
            .has_continuation_entry()
            .then(|| allocate(GraftedSpineNodeKind::ContinuationEntry, &mut nodes))
            .transpose()?;
        let exit = allocate(GraftedSpineNodeKind::FunctionExit, &mut nodes)?;

        let mut block_nodes = BTreeMap::new();
        for block in func.layout.blocks() {
            block_nodes.insert(block, allocate(GraftedSpineNodeKind::Body, &mut nodes)?);
        }
        let entry_block = func.layout.entry_block().ok_or_else(|| {
            graph_error("a finished generated function has no layout entry block")
        })?;
        let entry_body = *block_nodes.get(&entry_block).ok_or_else(|| {
            graph_error("a finished generated function's entry block was not translated")
        })?;
        let mut flow = BTreeSet::new();
        let entry_tail = continuation_entry.unwrap_or(entry);
        flow.insert(IntraFunctionEdge {
            from: entry,
            to: continuation_entry.unwrap_or(entry_body),
        });
        if continuation_entry.is_some() {
            flow.insert(IntraFunctionEdge {
                from: entry_tail,
                to: entry_body,
            });
        }

        let cfg = ControlFlowGraph::with_function(func);
        let mut calls = Vec::new();
        let mut terminals = Vec::new();
        for block in func.layout.blocks() {
            let block_node = block_nodes[&block];
            let mut prior = block_node;
            let mut ended_by_return = false;
            for inst in func.layout.block_insts(block) {
                let local = if let Some(pending) = calls_by_inst.remove(&inst) {
                    let InstructionData::Call { func_ref, .. } = func.dfg.insts[inst] else {
                        unreachable!("pending generated-unit calls were checked as direct calls")
                    };
                    let ExternalName::User(name_ref) = func.dfg.ext_funcs[func_ref].name else {
                        return Err(graph_error(
                            "a generated-unit call names a non-user endpoint",
                        ));
                    };
                    let user = &func.params.user_named_funcs()[name_ref];
                    if user.namespace != 0 {
                        return Err(graph_error(
                            "a generated-unit call names a foreign user-function namespace",
                        ));
                    }
                    let target = resolve_target(FuncId::from_u32(user.index)).ok_or_else(|| {
                        graph_error("a generated-unit call names an unknown typed endpoint")
                    })?;
                    let call = allocate(GraftedSpineNodeKind::CallerLocalCall, &mut nodes)?;
                    let return_site =
                        allocate(GraftedSpineNodeKind::CallReturn { call }, &mut nodes)?;
                    flow.insert(IntraFunctionEdge {
                        from: prior,
                        to: call,
                    });
                    calls.push(GraftedSpineCall {
                        caller: scope,
                        source: pending.source,
                        call,
                        return_site,
                        target,
                    });
                    prior = return_site;
                    continue;
                } else if let Some(observed) = terminals_by_inst.remove(&inst) {
                    let terminal = allocate(GraftedSpineNodeKind::TerminalDispatch, &mut nodes)?;
                    for pending in observed {
                        terminals.push(GraftedSpineTerminal {
                            member: pending.member,
                            body: pending.body,
                            node: GraftedSpineNode {
                                scope,
                                local: terminal,
                            },
                            control_word: pending.control_word,
                        });
                    }
                    terminal
                } else {
                    allocate(GraftedSpineNodeKind::Body, &mut nodes)?
                };
                flow.insert(IntraFunctionEdge {
                    from: prior,
                    to: local,
                });
                prior = local;
                if func.dfg.insts[inst].opcode() == Opcode::Return {
                    flow.insert(IntraFunctionEdge {
                        from: prior,
                        to: exit,
                    });
                    ended_by_return = true;
                }
            }
            if !ended_by_return {
                for successor in cfg.succ_iter(block) {
                    let successor = *block_nodes.get(&successor).ok_or_else(|| {
                        graph_error("a CFG successor was absent from the local node translation")
                    })?;
                    flow.insert(IntraFunctionEdge {
                        from: prior,
                        to: successor,
                    });
                }
            }
        }
        if !calls_by_inst.is_empty() || !terminals_by_inst.is_empty() {
            return Err(graph_error(
                "a locally validated grafted-spine observation was not consumed by layout translation",
            ));
        }

        let function = GraftedSpineFunctionGraph {
            scope,
            entry,
            continuation_entry,
            exit,
            nodes,
            flow,
            calls,
            terminals,
        };
        self.functions.insert(scope, function);
        Ok(())
    }

    pub(in crate::cranelift_backend) fn finish(
        self,
    ) -> Result<
        (
            GraftedSpineControlGraph,
            Vec<(StaticOriginId, StaticOriginId)>,
        ),
        CraneliftBackendError,
    > {
        let observed = self.functions.keys().copied().collect::<BTreeSet<_>>();
        if observed != self.expected {
            return Err(graph_error(format!(
                "grafted-spine body population does not close over the sealed declarations: expected {:?}, observed {:?}",
                self.expected, observed
            )));
        }
        let graph = GraftedSpineControlGraph {
            functions: self.functions,
        };
        graph.validate()?;
        Ok((graph, self.validation_inputs))
    }
}

/// Finished behavior-inert control graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::cranelift_backend) struct GraftedSpineControlGraph {
    functions: BTreeMap<GraftedSpineFunctionScope, GraftedSpineFunctionGraph>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ReachState {
    terminals: BTreeSet<GraftedSpineNode>,
    can_exit: bool,
}

impl ReachState {
    fn union_from(&mut self, other: &Self) -> bool {
        let old_len = self.terminals.len();
        self.terminals.extend(other.terminals.iter().copied());
        let old_exit = self.can_exit;
        self.can_exit |= other.can_exit;
        self.terminals.len() != old_len || self.can_exit != old_exit
    }
}

impl GraftedSpineControlGraph {
    #[cfg(any(test, feature = "px8-ds-test-support"))]
    pub(in crate::cranelift_backend) fn observe_actual_lowering_validation(
        &self,
        validation_inputs: impl IntoIterator<Item = (StaticOriginId, StaticOriginId)>,
    ) {
        if GRAFTED_SPINE_VALIDATION_MUTATION
            .with(std::cell::Cell::get)
            .is_none()
        {
            return;
        }
        // The sorted first planner row is stable across the two compilations.
        // Its expected source was already captured (and, for the second run,
        // mutated) before any emitted Function entered the graph builder.
        let validations = validation_inputs
            .into_iter()
            .next()
            .into_iter()
            .map(
                |(member, expected_source)| GraftedSpineValidationObservation {
                    member: member.observation_ordinal(),
                    expected_source: expected_source.observation_ordinal(),
                    outcome: self
                        .query(member, expected_source)
                        .map(|_| ())
                        .map_err(|error| error.to_string()),
                },
            )
            .collect();
        let terminals = self
            .functions
            .values()
            .flat_map(|function| {
                function
                    .terminals
                    .iter()
                    .map(|terminal| GraftedSpineObservedTerminal {
                        member: terminal.member.observation_ordinal(),
                        body: terminal.body.observation_ordinal(),
                        node: format!("{:?}", terminal.node),
                        control_word: terminal.control_word,
                    })
            })
            .collect();
        GRAFTED_SPINE_ACTUAL_LOWERING_OBSERVATIONS.with(|observations| {
            observations
                .borrow_mut()
                .push(GraftedSpineActualLoweringObservation {
                    topology: self
                        .typed_edges()
                        .into_iter()
                        .map(|edge| format!("{edge:?}"))
                        .collect(),
                    terminals,
                    validations,
                })
        });
    }

    fn validate(&self) -> Result<(), CraneliftBackendError> {
        for function in self.functions.values() {
            for call in &function.calls {
                let call_kind = function.nodes.get(&call.call);
                let return_kind = function.nodes.get(&call.return_site);
                if call_kind != Some(&GraftedSpineNodeKind::CallerLocalCall) {
                    return Err(graph_error(
                        "a grafted-spine call binding does not name a caller-local call node",
                    ));
                }
                if return_kind != Some(&GraftedSpineNodeKind::CallReturn { call: call.call }) {
                    return Err(graph_error(
                        "a grafted-spine call returns to a site owned by a different call",
                    ));
                }
                if !self.functions.contains_key(&call.target) {
                    return Err(graph_error(format!(
                        "a grafted-spine call names unknown typed endpoint {:?}",
                        call.target
                    )));
                }
            }
            for terminal in &function.terminals {
                if function.nodes.get(&terminal.node.local)
                    != Some(&GraftedSpineNodeKind::TerminalDispatch)
                {
                    return Err(graph_error(
                        "a terminal observation was moved to an entry or body node",
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_terminal_control_words(
        &self,
        node: GraftedSpineNode,
    ) -> Result<(), CraneliftBackendError> {
        let claims = self
            .functions
            .values()
            .flat_map(|function| function.terminals.iter())
            .filter(|terminal| terminal.node == node)
            .collect::<Vec<_>>();
        if claims.len() <= 1 {
            return Ok(());
        }
        let mut words = BTreeSet::new();
        for claim in claims {
            let Some(word) = claim.control_word else {
                return Err(graph_error(format!(
                    "terminal {node:?} serves several members without distinct words supplied from the existing Control slot"
                )));
            };
            if !words.insert(word) {
                return Err(graph_error(format!(
                    "terminal {node:?} repeats one existing Control-slot word"
                )));
            }
        }
        Ok(())
    }

    pub(in crate::cranelift_backend) fn typed_edges(&self) -> Vec<GraftedSpineTypedEdge> {
        let mut edges = Vec::new();
        for function in self.functions.values() {
            edges.extend(
                function
                    .flow
                    .iter()
                    .map(|edge| GraftedSpineTypedEdge::IntraFunction {
                        from: function.node(edge.from),
                        to: function.node(edge.to),
                    }),
            );
            for call in &function.calls {
                let callee = &self.functions[&call.target];
                edges.push(GraftedSpineTypedEdge::CallToEntry {
                    call: function.node(call.call),
                    entry: callee.node(callee.entry),
                });
                edges.push(GraftedSpineTypedEdge::CalleeExitToExactReturn {
                    exit: callee.node(callee.exit),
                    call: function.node(call.call),
                    return_site: function.node(call.return_site),
                });
            }
        }
        edges
    }

    fn reachability(
        &self,
    ) -> Result<
        (
            BTreeMap<GraftedSpineNode, ReachState>,
            Vec<GraftedSpineRealizableSummary>,
        ),
        CraneliftBackendError,
    > {
        let mut state = BTreeMap::new();
        let mut terminal_nodes = BTreeSet::new();
        for function in self.functions.values() {
            for local in function.nodes.keys().copied() {
                state.insert(function.node(local), ReachState::default());
            }
            state
                .get_mut(&function.node(function.exit))
                .expect("the exit was inserted above")
                .can_exit = true;
            for terminal in &function.terminals {
                terminal_nodes.insert(terminal.node);
                state
                    .get_mut(&terminal.node)
                    .expect("the terminal was inserted above")
                    .terminals
                    .insert(terminal.node);
            }
        }

        let node_count = state.len();
        let terminal_count = terminal_nodes.len();
        let mut changed = true;
        let mut iterations = 0usize;
        while changed {
            changed = false;
            iterations = iterations
                .checked_add(1)
                .ok_or_else(|| graph_error("grafted-spine summary iteration count overflowed"))?;
            let bound = node_count
                .checked_mul(terminal_count.saturating_add(1))
                .and_then(|n| n.checked_add(1))
                .ok_or_else(|| graph_error("grafted-spine summary bound overflowed"))?;
            if iterations > bound {
                return Err(graph_error(
                    "grafted-spine well-nested summaries did not converge",
                ));
            }
            for function in self.functions.values() {
                let calls = function
                    .calls
                    .iter()
                    .map(|call| (call.call, call))
                    .collect::<BTreeMap<_, _>>();
                let successors = function.flow.iter().fold(
                    BTreeMap::<GraftedSpineLocalNodeId, Vec<GraftedSpineLocalNodeId>>::new(),
                    |mut map, edge| {
                        map.entry(edge.from).or_default().push(edge.to);
                        map
                    },
                );
                for local in function.nodes.keys().copied().collect::<Vec<_>>() {
                    let node = function.node(local);
                    if terminal_nodes.contains(&node) || local == function.exit {
                        continue;
                    }
                    let mut next = ReachState::default();
                    if let Some(call) = calls.get(&local) {
                        let callee = &self.functions[&call.target];
                        let callee_state = state[&callee.node(callee.entry)].clone();
                        next.terminals
                            .extend(callee_state.terminals.iter().copied());
                        if callee_state.can_exit {
                            next.union_from(&state[&function.node(call.return_site)]);
                        }
                    } else {
                        for successor in successors.get(&local).into_iter().flatten() {
                            next.union_from(&state[&function.node(*successor)]);
                        }
                    }
                    let current = state.get_mut(&node).expect("the node was inserted above");
                    if current.union_from(&next) {
                        changed = true;
                    }
                }
            }
        }

        let summaries = self
            .functions
            .values()
            .flat_map(|function| {
                function.calls.iter().filter_map(|call| {
                    let callee = &self.functions[&call.target];
                    state[&callee.node(callee.entry)].can_exit.then_some(
                        GraftedSpineRealizableSummary {
                            call: function.node(call.call),
                            return_site: function.node(call.return_site),
                        },
                    )
                })
            })
            .collect();
        Ok((state, summaries))
    }

    /// Independently join a D0 member/source request to what lowering emitted.
    ///
    /// Neither `member` nor `expected_source` participates in graph
    /// construction. They can therefore alter this verdict and cannot alter
    /// topology or the independently observed terminal population.
    pub(in crate::cranelift_backend) fn query(
        &self,
        member: StaticOriginId,
        expected_source: StaticOriginId,
    ) -> Result<GraftedSpineQuery, CraneliftBackendError> {
        self.validate()?;
        let seeds = self
            .functions
            .values()
            .flat_map(|function| {
                function.calls.iter().filter_map(|call| {
                    (call.source == Some(expected_source)).then_some(GraftedSpineSeedEdge {
                        source: expected_source,
                        call: function.node(call.call),
                        target: call.target,
                    })
                })
            })
            .collect::<Vec<_>>();
        if seeds.is_empty() {
            return Err(graph_error(format!(
                "member {member:?} has no actual seed edge at expected source {expected_source:?}"
            )));
        }
        let terminals = self
            .functions
            .values()
            .flat_map(|function| function.terminals.iter())
            .filter(|terminal| terminal.member == member)
            .collect::<Vec<_>>();
        let [terminal] = terminals.as_slice() else {
            return Err(graph_error(format!(
                "member {member:?} has {} independently observed terminals instead of one",
                terminals.len()
            )));
        };
        self.validate_terminal_control_words(terminal.node)?;
        let (state, summaries) = self.reachability()?;
        for seed in &seeds {
            let reached = &state[&seed.call];
            if reached.can_exit {
                return Err(graph_error(format!(
                    "member {member:?} can leave a finite well-nested source path unconsumed"
                )));
            }
            if reached.terminals != BTreeSet::from([terminal.node]) {
                return Err(graph_error(format!(
                    "member {member:?} reaches terminals {:?} rather than its unique observed terminal {:?}",
                    reached.terminals, terminal.node
                )));
            }
        }
        Ok(GraftedSpineQuery {
            seeds,
            terminal: terminal.node,
            terminal_member: terminal.member,
            terminal_body: terminal.body,
            summaries,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn origin(id: u32) -> StaticOriginId {
        StaticOriginId::for_test(id)
    }

    fn scope(id: u32) -> GraftedSpineFunctionScope {
        GraftedSpineFunctionScope::Predeclared(PredeclaredFunctionId::for_test(id))
    }

    fn node(scope: GraftedSpineFunctionScope, local: u32) -> GraftedSpineNode {
        GraftedSpineNode {
            scope,
            local: GraftedSpineLocalNodeId(local),
        }
    }

    fn function(
        scope: GraftedSpineFunctionScope,
        nodes: &[(u32, GraftedSpineNodeKind)],
        flow: &[(u32, u32)],
        entry: u32,
        exit: u32,
    ) -> GraftedSpineFunctionGraph {
        GraftedSpineFunctionGraph {
            scope,
            entry: GraftedSpineLocalNodeId(entry),
            continuation_entry: None,
            exit: GraftedSpineLocalNodeId(exit),
            nodes: nodes
                .iter()
                .map(|(id, kind)| (GraftedSpineLocalNodeId(*id), *kind))
                .collect(),
            flow: flow
                .iter()
                .map(|(from, to)| IntraFunctionEdge {
                    from: GraftedSpineLocalNodeId(*from),
                    to: GraftedSpineLocalNodeId(*to),
                })
                .collect(),
            calls: Vec::new(),
            terminals: Vec::new(),
        }
    }

    fn two_path_graph() -> GraftedSpineControlGraph {
        let caller = scope(0);
        let callee = scope(1);
        let mut caller_fn = function(
            caller,
            &[
                (0, GraftedSpineNodeKind::FunctionEntry),
                (1, GraftedSpineNodeKind::CallerLocalCall),
                (
                    2,
                    GraftedSpineNodeKind::CallReturn {
                        call: GraftedSpineLocalNodeId(1),
                    },
                ),
                (3, GraftedSpineNodeKind::FunctionExit),
            ],
            &[(0, 1), (2, 3)],
            0,
            3,
        );
        caller_fn.calls.push(GraftedSpineCall {
            caller,
            source: Some(origin(328)),
            call: GraftedSpineLocalNodeId(1),
            return_site: GraftedSpineLocalNodeId(2),
            target: callee,
        });
        let mut callee_fn = function(
            callee,
            &[
                (0, GraftedSpineNodeKind::FunctionEntry),
                (1, GraftedSpineNodeKind::Body),
                (2, GraftedSpineNodeKind::Body),
                (3, GraftedSpineNodeKind::Body),
                (4, GraftedSpineNodeKind::TerminalDispatch),
                (5, GraftedSpineNodeKind::FunctionExit),
            ],
            &[(0, 1), (1, 2), (1, 3), (2, 4), (3, 4)],
            0,
            5,
        );
        callee_fn.terminals.push(GraftedSpineTerminal {
            member: origin(1092),
            body: origin(1076),
            node: node(callee, 4),
            control_word: None,
        });
        GraftedSpineControlGraph {
            functions: BTreeMap::from([(caller, caller_fn), (callee, callee_fn)]),
        }
    }

    fn nested_call_graph() -> GraftedSpineControlGraph {
        let caller = scope(0);
        let middle = scope(1);
        let leaf = scope(2);
        let mut caller_fn = function(
            caller,
            &[
                (0, GraftedSpineNodeKind::FunctionEntry),
                (1, GraftedSpineNodeKind::CallerLocalCall),
                (
                    2,
                    GraftedSpineNodeKind::CallReturn {
                        call: GraftedSpineLocalNodeId(1),
                    },
                ),
                (3, GraftedSpineNodeKind::FunctionExit),
            ],
            &[(0, 1), (2, 3)],
            0,
            3,
        );
        caller_fn.calls.push(GraftedSpineCall {
            caller,
            source: Some(origin(328)),
            call: GraftedSpineLocalNodeId(1),
            return_site: GraftedSpineLocalNodeId(2),
            target: middle,
        });
        let mut middle_fn = function(
            middle,
            &[
                (0, GraftedSpineNodeKind::FunctionEntry),
                (1, GraftedSpineNodeKind::CallerLocalCall),
                (
                    2,
                    GraftedSpineNodeKind::CallReturn {
                        call: GraftedSpineLocalNodeId(1),
                    },
                ),
                (3, GraftedSpineNodeKind::FunctionExit),
            ],
            &[(0, 1), (2, 3)],
            0,
            3,
        );
        middle_fn.calls.push(GraftedSpineCall {
            caller: middle,
            source: None,
            call: GraftedSpineLocalNodeId(1),
            return_site: GraftedSpineLocalNodeId(2),
            target: leaf,
        });
        let mut leaf_fn = function(
            leaf,
            &[
                (0, GraftedSpineNodeKind::FunctionEntry),
                (1, GraftedSpineNodeKind::TerminalDispatch),
                (2, GraftedSpineNodeKind::FunctionExit),
            ],
            &[(0, 1)],
            0,
            2,
        );
        leaf_fn.terminals.push(GraftedSpineTerminal {
            member: origin(1092),
            body: origin(1076),
            node: node(leaf, 1),
            control_word: None,
        });
        GraftedSpineControlGraph {
            functions: BTreeMap::from([(caller, caller_fn), (middle, middle_fn), (leaf, leaf_fn)]),
        }
    }

    #[test]
    fn branch_and_reconvergence_allows_two_realizable_paths_to_one_terminal() {
        let graph = two_path_graph();
        let query = graph.query(origin(1092), origin(328)).unwrap();
        assert_eq!(query.seeds.len(), 1);
        assert_eq!(query.terminal, node(scope(1), 4));
    }

    #[test]
    fn several_real_seed_alternatives_may_converge_on_one_terminal() {
        let mut graph = two_path_graph();
        let caller = scope(3);
        let mut alternate = function(
            caller,
            &[
                (0, GraftedSpineNodeKind::FunctionEntry),
                (1, GraftedSpineNodeKind::CallerLocalCall),
                (
                    2,
                    GraftedSpineNodeKind::CallReturn {
                        call: GraftedSpineLocalNodeId(1),
                    },
                ),
                (3, GraftedSpineNodeKind::FunctionExit),
            ],
            &[(0, 1), (2, 3)],
            0,
            3,
        );
        alternate.calls.push(GraftedSpineCall {
            caller,
            source: Some(origin(328)),
            call: GraftedSpineLocalNodeId(1),
            return_site: GraftedSpineLocalNodeId(2),
            target: scope(1),
        });
        graph.functions.insert(caller, alternate);
        let query = graph.query(origin(1092), origin(328)).unwrap();
        assert_eq!(query.seeds.len(), 2);
        assert_eq!(query.terminal, node(scope(1), 4));
    }

    #[test]
    fn deleted_intermediate_call_return_rejects_at_terminal_reachability_layer() {
        let mut graph = nested_call_graph();
        graph.functions.get_mut(&scope(1)).unwrap().calls.clear();
        let error = graph.query(origin(1092), origin(328)).unwrap_err();
        assert!(
            error.to_string().contains("reaches terminals {}"),
            "{error}"
        );
    }

    #[test]
    fn mismatched_return_rejects_at_exact_return_layer() {
        let mut graph = two_path_graph();
        graph.functions.get_mut(&scope(0)).unwrap().calls[0].return_site =
            GraftedSpineLocalNodeId(3);
        let error = graph.query(origin(1092), origin(328)).unwrap_err();
        assert!(
            error.to_string().contains("site owned by a different call"),
            "{error}"
        );
    }

    #[test]
    fn unknown_typed_endpoint_fails_closed() {
        let mut graph = two_path_graph();
        graph.functions.get_mut(&scope(0)).unwrap().calls[0].target = scope(99);
        let error = graph.query(origin(1092), origin(328)).unwrap_err();
        assert!(
            error.to_string().contains("unknown typed endpoint"),
            "{error}"
        );
    }

    #[test]
    fn terminal_moved_to_body_rejects_at_terminal_kind_layer() {
        let mut graph = two_path_graph();
        graph
            .functions
            .get_mut(&scope(1))
            .unwrap()
            .nodes
            .insert(GraftedSpineLocalNodeId(4), GraftedSpineNodeKind::Body);
        let error = graph.query(origin(1092), origin(328)).unwrap_err();
        assert!(
            error.to_string().contains("moved to an entry or body"),
            "{error}"
        );
    }

    #[test]
    fn duplicate_terminal_rejects_at_member_affinity_layer() {
        let mut graph = two_path_graph();
        let duplicate = GraftedSpineTerminal {
            member: origin(1092),
            body: origin(1076),
            node: node(scope(1), 4),
            control_word: None,
        };
        graph
            .functions
            .get_mut(&scope(1))
            .unwrap()
            .terminals
            .push(duplicate);
        let error = graph.query(origin(1092), origin(328)).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("independently observed terminals instead of one"),
            "{error}"
        );
    }

    #[test]
    fn shared_terminal_requires_injective_existing_control_words() {
        let mut graph = two_path_graph();
        let function = graph.functions.get_mut(&scope(1)).unwrap();
        function.terminals[0].control_word = Some(11);
        function.terminals.push(GraftedSpineTerminal {
            member: origin(1093),
            body: origin(1076),
            node: node(scope(1), 4),
            control_word: Some(12),
        });
        assert_eq!(
            graph.query(origin(1092), origin(328)).unwrap().terminal,
            node(scope(1), 4)
        );
        assert_eq!(
            graph.query(origin(1093), origin(328)).unwrap().terminal,
            node(scope(1), 4)
        );

        graph.functions.get_mut(&scope(1)).unwrap().terminals[1].control_word = Some(11);
        let error = graph.query(origin(1092), origin(328)).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("repeats one existing Control-slot word"),
            "{error}"
        );
    }

    #[test]
    fn query_inputs_do_not_mutate_a_finished_synthetic_graph() {
        let graph = two_path_graph();
        let before_edges = graph.typed_edges();
        let before_terminals = graph
            .functions
            .values()
            .flat_map(|function| function.terminals.clone())
            .collect::<Vec<_>>();
        let error = graph.query(origin(1092), origin(329)).unwrap_err();
        assert!(error.to_string().contains("no actual seed edge"), "{error}");
        assert_eq!(graph.typed_edges(), before_edges);
        assert_eq!(
            graph
                .functions
                .values()
                .flat_map(|function| function.terminals.clone())
                .collect::<Vec<_>>(),
            before_terminals
        );
    }
}
