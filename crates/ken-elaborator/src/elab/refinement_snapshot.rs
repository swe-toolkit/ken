//! Transactional branch retry for subset-Σ evidence demand.
//!
//! A failed first pass may have solved an older level metavariable, admitted
//! an obligation body, or allocated a literal. A retry must restore the whole
//! owned elaboration state and kernel suffix, not merely truncate vectors.

use super::*;
use ken_kernel::{env_mark, rollback_to_mark, EnvMark};

pub(super) struct Snapshot {
    env_mark: EnvMark,
    recursive_group: HashSet<GlobalId>,
    ctx: Context,
    assumptions: Vec<Assumption>,
    metas: MetaCtx,
    preconditions: HashMap<GlobalId, (usize, usize)>,
    premise_holes: PremiseHoles,
    obligations: Vec<Obligation>,
    obl_counter: u32,
    type_introductions: bool,
    refinement_slot: RefinementSlot,
    path_conditions: Vec<PathFrame>,
    next_split_id: u64,
    active_splits: Vec<u64>,
    convoy_splits: HashSet<u64>,
    ih_consuming_splits: HashSet<u64>,
    result_predicates: Vec<ResultPredicate>,
    provenance: Option<Vec<crate::classes::InstanceResolution>>,
    local_dicts: HashMap<String, (Term, Term, usize)>,
    var_refinements: HashMap<usize, (Term, Term, usize)>,
    active_index_refinements: Vec<ActiveIndexRefinement>,
    match_frames: Vec<MatchFrame>,
    next_aux_sentinel_region: usize,
    hidden_positions: Vec<usize>,
    matrix_virtual_surface_positions: Vec<usize>,
    lift_bindings: HashMap<usize, LiftBinding>,
    result_refinements: Vec<ResultRefinement>,
    active_index_premise_frames: Vec<ActiveIndexPremiseFrame>,
    space_state: Option<(usize, Vec<Term>)>,
    space_pre_state: Option<(usize, Vec<Term>)>,
    pattern_alias_type_frames: Vec<PatternAliasTypeFrame>,
    active_pattern_aliases: Vec<Vec<ActivePatternAlias>>,
    indexed_match_roots: Vec<IndexedMatchRootFrame>,
    matrix_entries: Vec<MatrixEntry>,
}

impl Snapshot {
    /// Allocate the retry state outside the ordinary recursive match frame.
    #[inline(never)]
    pub(super) fn capture_boxed(cx: &ElabCtx<'_>) -> Box<Self> {
        Box::new(Self::capture(cx))
    }

    pub(super) fn capture(cx: &ElabCtx<'_>) -> Self {
        Self {
            env_mark: env_mark(cx.env),
            recursive_group: cx.recursive_group.clone(),
            ctx: cx.ctx.clone(),
            assumptions: cx.assumptions.clone(),
            metas: cx.metas.clone(),
            preconditions: cx.preconditions.clone(),
            premise_holes: cx.premise_holes,
            obligations: cx.obligations.clone(),
            obl_counter: cx.obl_counter,
            type_introductions: cx.type_introductions,
            refinement_slot: cx.refinement_slot,
            path_conditions: cx.path_conditions.clone(),
            next_split_id: cx.next_split_id,
            active_splits: cx.active_splits.clone(),
            convoy_splits: cx.convoy_splits.clone(),
            ih_consuming_splits: cx.ih_consuming_splits.clone(),
            result_predicates: cx.result_predicates.clone(),
            provenance: cx.provenance.as_ref().map(|log| (**log).clone()),
            local_dicts: cx.local_dicts.clone(),
            var_refinements: cx.var_refinements.clone(),
            active_index_refinements: cx.active_index_refinements.clone(),
            match_frames: cx.match_frames.clone(),
            next_aux_sentinel_region: cx.next_aux_sentinel_region,
            hidden_positions: cx.hidden_positions.clone(),
            matrix_virtual_surface_positions: cx.matrix_virtual_surface_positions.clone(),
            lift_bindings: cx.lift_bindings.clone(),
            result_refinements: cx.result_refinements.clone(),
            active_index_premise_frames: cx.active_index_premise_frames.clone(),
            space_state: cx.space_state.clone(),
            space_pre_state: cx.space_pre_state.clone(),
            pattern_alias_type_frames: cx.pattern_alias_type_frames.clone(),
            active_pattern_aliases: cx.active_pattern_aliases.clone(),
            indexed_match_roots: cx.indexed_match_roots.clone(),
            matrix_entries: cx.matrix_entries.clone(),
        }
    }

    pub(super) fn rollback(self, cx: &mut ElabCtx<'_>) -> Result<(), ElabError> {
        let removed = rollback_to_mark(cx.env, self.env_mark).map_err(|error| {
            ElabError::Internal(format!("refinement branch rollback failed: {error}"))
        })?;
        let removed_ids: HashSet<_> = removed.iter().map(Decl::id).collect();
        cx.num_values.retain(|id, _| !removed_ids.contains(id));
        if let (Some(saved), Some(log)) = (self.provenance, cx.provenance.as_mut()) {
            **log = saved;
        }
        cx.recursive_group = self.recursive_group;
        cx.ctx = self.ctx;
        cx.assumptions = self.assumptions;
        cx.metas = self.metas;
        cx.preconditions = self.preconditions;
        cx.premise_holes = self.premise_holes;
        cx.obligations = self.obligations;
        cx.obl_counter = self.obl_counter;
        cx.type_introductions = self.type_introductions;
        cx.refinement_slot = self.refinement_slot;
        cx.path_conditions = self.path_conditions;
        cx.next_split_id = self.next_split_id;
        cx.active_splits = self.active_splits;
        cx.convoy_splits = self.convoy_splits;
        cx.ih_consuming_splits = self.ih_consuming_splits;
        cx.result_predicates = self.result_predicates;
        cx.local_dicts = self.local_dicts;
        cx.var_refinements = self.var_refinements;
        cx.active_index_refinements = self.active_index_refinements;
        cx.match_frames = self.match_frames;
        cx.next_aux_sentinel_region = self.next_aux_sentinel_region;
        cx.hidden_positions = self.hidden_positions;
        cx.matrix_virtual_surface_positions = self.matrix_virtual_surface_positions;
        cx.lift_bindings = self.lift_bindings;
        cx.result_refinements = self.result_refinements;
        cx.active_index_premise_frames = self.active_index_premise_frames;
        cx.space_state = self.space_state;
        cx.space_pre_state = self.space_pre_state;
        cx.pattern_alias_type_frames = self.pattern_alias_type_frames;
        cx.active_pattern_aliases = self.active_pattern_aliases;
        cx.indexed_match_roots = self.indexed_match_roots;
        cx.matrix_entries = self.matrix_entries;
        Ok(())
    }
}
