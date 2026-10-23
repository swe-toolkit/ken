//! Refined-parameter recursion is checked on carrier-typed private workers.
//!
//! This is an elaborator-only adapter: the kernel checks both the worker SCT
//! group and the original public signatures' non-recursive wrappers. A group
//! already accepted by SCT bypasses the adapter byte-for-byte. Every failed
//! worker attempt falls back to admission of the original bodies.

use super::*;
use ken_kernel::subst::subst_var;

struct RefinedMember {
    id: GlobalId,
    level_params: Vec<LevelVar>,
    original_type: Term,
    original_body: Term,
    /// Leading parameter positions through the last subset-Σ position.
    positions: Vec<Option<(Term, Term)>>,
    worker_type: Term,
}

struct RefinedWorkerPlan {
    members: Vec<RefinedMember>,
}

/// Replace a refined binder under `depth` later binders with its split pair.
/// Old outer variables shift up twice before substitution removes the old
/// binder; in the resulting context its carrier and proof have indices
/// `depth + 1` and `depth` respectively.
fn split_refined_under(term: &Term, depth: usize) -> Term {
    let pair = Term::pair(Term::var(depth + 1), Term::var(depth));
    subst_var(&shift(term, 2, depth + 1), depth, &pair)
}

fn split_refined_binder(body: &Term) -> Term {
    split_refined_under(body, 0)
}

/// Cached later subset domains were classified under the original telescope.
/// Rebase both components for each later domain: its carrier is under one
/// binder per intervening parameter, and its predicate has one more binder
/// for the subset carrier. Without this, a dependent second refinement still
/// refers to the removed original binder even after its Pi/Lam body is split.
fn split_later_positions(positions: &[Option<(Term, Term)>]) -> Vec<Option<(Term, Term)>> {
    positions
        .iter()
        .enumerate()
        .map(|(depth, position)| {
            position.as_ref().map(|(carrier, predicate)| {
                (
                    split_refined_under(carrier, depth),
                    split_refined_under(predicate, depth + 1),
                )
            })
        })
        .collect()
}

fn subset_domain(env: &GlobalEnv, ctx: &Context, domain: &Term) -> Option<(Term, Term)> {
    let Term::Sigma(carrier, predicate) = whnf(env, ctx, domain) else {
        return None;
    };
    let carrier_sort = kernel_infer_raw(env, ctx, &carrier).ok()?;
    if !matches!(whnf(env, ctx, &carrier_sort), Term::Type(_)) {
        return None;
    }
    let mut under_carrier = ctx.clone();
    under_carrier.push((*carrier).clone());
    let predicate_sort = kernel_infer_raw(env, &under_carrier, &predicate).ok()?;
    matches!(whnf(env, &under_carrier, &predicate_sort), Term::Omega(_))
        .then(|| ((*carrier).clone(), (*predicate).clone()))
}

fn worker_type(ty: &Term, positions: &[Option<(Term, Term)>]) -> Option<Term> {
    let Some((position, remaining)) = positions.split_first() else {
        return Some(ty.clone());
    };
    let Term::Pi(domain, codomain) = ty else {
        return None;
    };
    match position {
        Some((carrier, predicate)) => {
            // The current domain may be an alias; its checked WHNF is the
            // classified subset. Later domains and the result retain their
            // original meaning by replacing the old pair binder.
            let rest = split_refined_binder(codomain);
            let later = split_later_positions(remaining);
            let rest = worker_type(&rest, &later)?;
            Some(Term::pi(carrier.clone(), Term::pi(predicate.clone(), rest)))
        }
        None => Some(Term::pi(
            (**domain).clone(),
            worker_type(codomain, remaining)?,
        )),
    }
}

/// Split exactly the declared leading lambdas whose Pi domains are in the
/// plan. Do not eta-expand a missing lambda: leave that shape on the existing
/// admission path instead of manufacturing an SCT argument position.
fn split_body(body: &Term, positions: &[Option<(Term, Term)>]) -> Option<Term> {
    let Some((position, remaining)) = positions.split_first() else {
        return Some(body.clone());
    };
    let Term::Lam(domain, inner) = body else {
        return None;
    };
    match position {
        Some((carrier, predicate)) => {
            let later = split_later_positions(remaining);
            let inner = split_body(&split_refined_binder(inner), &later)?;
            Some(Term::lam(
                carrier.clone(),
                Term::lam(predicate.clone(), inner),
            ))
        }
        None => Some(Term::lam((**domain).clone(), split_body(inner, remaining)?)),
    }
}

fn strip_ascript(mut term: &Term) -> &Term {
    while let Term::Ascript(inner, _) = term {
        term = inner;
    }
    term
}

fn unwrap_pair(term: &Term) -> Option<(&Term, &Term)> {
    match strip_ascript(term) {
        Term::Pair(a, b) => Some((a, b)),
        _ => None,
    }
}

fn projection_pair(term: &Term) -> Option<(&Term, &Term)> {
    unwrap_pair(term)
}

/// What a literal-pair projection may do during the rewrite.
enum ProjectionRedexes {
    /// Pre-check: admit only a projection whose operand is a refined argument
    /// of a saturated member call whose value is in flight: in that call's
    /// result ascription or in the type and scope of a let that binds it.
    /// Authority is inherited inward and never reset.
    Induced(Vec<Term>),
    /// Worker construction: reduce every literal-pair projection after the
    /// original type and body have passed the pre-check.
    All,
}

impl ProjectionRedexes {
    fn under_binder(&self) -> Self {
        match self {
            Self::Induced(args) => Self::Induced(args.iter().map(|arg| shift(arg, 1, 0)).collect()),
            Self::All => Self::All,
        }
    }

    fn admits(&self, operand: &Term) -> bool {
        match self {
            Self::Induced(args) => args.iter().any(|arg| arg == strip_ascript(operand)),
            Self::All => true,
        }
    }

    /// Add the refined arguments of a saturated member call to the inherited set.
    fn with_call(
        &self,
        call: &Term,
        members: &HashMap<GlobalId, (&[Option<(Term, Term)>], GlobalId)>,
    ) -> Self {
        match self {
            Self::Induced(args) => Self::Induced(
                args.iter()
                    .cloned()
                    .chain(member_call_refined_args(call, members))
                    .collect(),
            ),
            Self::All => Self::All,
        }
    }
}

/// The refined arguments of a saturated member call; otherwise none.
fn member_call_refined_args(
    term: &Term,
    members: &HashMap<GlobalId, (&[Option<(Term, Term)>], GlobalId)>,
) -> Vec<Term> {
    let (head, args) = peel_app(term);
    match head {
        Term::Const { id, .. } => members
            .get(&id)
            .filter(|(positions, _)| args.len() >= positions.len())
            .map(|(positions, _)| {
                positions
                    .iter()
                    .zip(&args)
                    .filter(|(position, _)| position.is_some())
                    .map(|(_, arg)| strip_ascript(arg).clone())
                    .collect()
            })
            .unwrap_or_default(),
        _ => vec![],
    }
}

/// One total, binder-preserving core walk. Group Const occurrences are valid
/// only as heads of saturated application spines through the last refined
/// position. The same walk, with placeholder worker IDs, validates the entire
/// body and type before any worker is staged. It recurses into method/motive
/// terms and ascriptions, not only expression leaves.
fn rewrite_core(
    term: &Term,
    members: &HashMap<GlobalId, (&[Option<(Term, Term)>], GlobalId)>,
    mode: &ProjectionRedexes,
) -> Option<Term> {
    let walk = |term: &Term| rewrite_core(term, members, mode);
    let under_binder = |term: &Term| rewrite_core(term, members, &mode.under_binder());
    Some(match term {
        Term::App(..) => {
            let (head, args) = peel_app(term);
            if let Term::Const { id, level_args } = &head {
                if let Some((positions, worker)) = members.get(id) {
                    if args.len() < positions.len() {
                        return None;
                    }
                    let mut call = Term::const_(*worker, level_args.clone());
                    for (index, arg) in args.iter().enumerate() {
                        let arg = walk(arg)?;
                        if positions.get(index).is_some_and(Option::is_some) {
                            let (carrier, proof) = match unwrap_pair(&arg) {
                                Some((carrier, proof)) => (carrier.clone(), proof.clone()),
                                None => (Term::proj1(arg.clone()), Term::proj2(arg)),
                            };
                            call = Term::app(Term::app(call, carrier), proof);
                        } else {
                            call = Term::app(call, arg);
                        }
                    }
                    return Some(call);
                }
            }
            let mut call = walk(&head)?;
            for arg in &args {
                call = Term::app(call, walk(arg)?);
            }
            call
        }
        Term::Const { id, .. } if members.contains_key(id) => return None,
        Term::Pi(a, b) => Term::pi(walk(a)?, under_binder(b)?),
        Term::Lam(a, b) => Term::lam(walk(a)?, under_binder(b)?),
        Term::Sigma(a, b) => Term::sigma(walk(a)?, under_binder(b)?),
        Term::Pair(a, b) => Term::pair(walk(a)?, walk(b)?),
        Term::Proj1(inner) => {
            let inner = walk(inner)?;
            match projection_pair(&inner) {
                Some((carrier, _)) if mode.admits(&inner) => carrier.clone(),
                Some(_) => return None,
                None => Term::proj1(inner),
            }
        }
        Term::Proj2(inner) => {
            let inner = walk(inner)?;
            match projection_pair(&inner) {
                Some((_, proof)) if mode.admits(&inner) => proof.clone(),
                Some(_) => return None,
                None => Term::proj2(inner),
            }
        }
        Term::Let { ty, val, body } => {
            let bound = mode.with_call(val, members);
            Term::Let {
                ty: Box::new(rewrite_core(ty, members, &bound)?),
                val: Box::new(walk(val)?),
                body: Box::new(rewrite_core(body, members, &bound.under_binder())?),
            }
        }
        Term::Ascript(body, ty) => match mode {
            ProjectionRedexes::All => Term::Ascript(Box::new(walk(body)?), Box::new(walk(ty)?)),
            ProjectionRedexes::Induced(_) => Term::Ascript(
                Box::new(walk(body)?),
                Box::new(rewrite_core(ty, members, &mode.with_call(body, members))?),
            ),
        },
        Term::Eq(a, b, c) => Term::Eq(Box::new(walk(a)?), Box::new(walk(b)?), Box::new(walk(c)?)),
        Term::Refl(a) => Term::Refl(Box::new(walk(a)?)),
        Term::Cast(a, b, c, d) => Term::Cast(
            Box::new(walk(a)?),
            Box::new(walk(b)?),
            Box::new(walk(c)?),
            Box::new(walk(d)?),
        ),
        Term::J(a, b, c) => Term::J(Box::new(walk(a)?), Box::new(walk(b)?), Box::new(walk(c)?)),
        Term::Quot(a, b, c) => {
            Term::Quot(Box::new(walk(a)?), Box::new(walk(b)?), Box::new(walk(c)?))
        }
        Term::QuotClass(a) => Term::QuotClass(Box::new(walk(a)?)),
        Term::Trunc(a) => Term::Trunc(Box::new(walk(a)?)),
        Term::TruncProj(a) => Term::TruncProj(Box::new(walk(a)?)),
        Term::Absurd(a, b) => Term::Absurd(Box::new(walk(a)?), Box::new(walk(b)?)),
        Term::QuotElim {
            motive,
            method,
            respect,
            scrut,
        } => Term::QuotElim {
            motive: Box::new(walk(motive)?),
            method: Box::new(walk(method)?),
            respect: Box::new(walk(respect)?),
            scrut: Box::new(walk(scrut)?),
        },
        Term::Elim {
            fam,
            level_args,
            params,
            motive,
            methods,
            indices,
            scrut,
        } => Term::Elim {
            fam: *fam,
            level_args: level_args.clone(),
            params: params.iter().map(walk).collect::<Option<Vec<_>>>()?,
            motive: Box::new(walk(motive)?),
            methods: methods.iter().map(walk).collect::<Option<Vec<_>>>()?,
            indices: indices.iter().map(walk).collect::<Option<Vec<_>>>()?,
            scrut: Box::new(walk(scrut)?),
        },
        Term::Var(_)
        | Term::Const { .. }
        | Term::Type(_)
        | Term::Omega(_)
        | Term::IndFormer { .. }
        | Term::Constructor { .. }
        | Term::IntLit(_) => term.clone(),
    })
}

impl RefinedWorkerPlan {
    fn new(env: &GlobalEnv, ids: &[GlobalId], bodies: &[Term]) -> Option<Self> {
        if ids.len() != bodies.len() || ids.is_empty() {
            return None;
        }
        let mut members = Vec::with_capacity(ids.len());
        let mut has_refined_position = false;
        for (&id, body) in ids.iter().zip(bodies) {
            let Some(Decl::Opaque {
                ty, level_params, ..
            }) = env.lookup(id)
            else {
                return None;
            };
            let mut ctx = Context::new();
            let mut current = ty;
            let mut all_positions = Vec::new();
            while let Term::Pi(domain, codomain) = current {
                let refinement = subset_domain(env, &ctx, domain);
                if refinement.is_some() {
                    has_refined_position = true;
                }
                all_positions.push(refinement);
                ctx.push((**domain).clone());
                current = codomain;
            }
            let arity = all_positions
                .iter()
                .rposition(Option::is_some)
                .map_or(0, |index| index + 1);
            all_positions.truncate(arity);
            let worker_ty = worker_type(ty, &all_positions)?;
            // A worker must be able to reproduce every group member, including
            // an unrefined one in a mutual cycle: otherwise its edges through
            // an opaque public member would escape the worker SCT group.
            split_body(body, &all_positions)?;
            members.push(RefinedMember {
                id,
                level_params: level_params.clone(),
                original_type: ty.clone(),
                original_body: body.clone(),
                positions: all_positions,
                worker_type: worker_ty,
            });
        }
        if !has_refined_position {
            return None;
        }
        // Fail closed on unsaturated, higher-order or type/motive mentions.
        // Placeholder IDs suffice here: rewriting does not infer their types.
        let by_id = members
            .iter()
            .map(|member| (member.id, (member.positions.as_slice(), GlobalId(u32::MAX))))
            .collect();
        for member in &members {
            // Permit only a refined call argument projected where that call's
            // value is in flight: its result ascription, or a let that binds it.
            // Source literal-pair projections and every other shape retain the
            // original admission path.
            rewrite_core(
                &member.original_type,
                &by_id,
                &ProjectionRedexes::Induced(vec![]),
            )?;
            rewrite_core(
                &member.original_body,
                &by_id,
                &ProjectionRedexes::Induced(vec![]),
            )?;
        }
        Some(Self { members })
    }

    fn worker_specs(&self) -> Vec<(String, Vec<LevelVar>, Term)> {
        self.members
            .iter()
            .map(|member| {
                (
                    format!("\0ken.refined.worker.{}", member.id.0),
                    member.level_params.clone(),
                    member.worker_type.clone(),
                )
            })
            .collect()
    }

    fn worker_bodies(&self, ids: &[GlobalId]) -> Option<Vec<Term>> {
        let by_id: HashMap<GlobalId, (&[Option<(Term, Term)>], GlobalId)> = self
            .members
            .iter()
            .zip(ids)
            .map(|(member, worker)| (member.id, (member.positions.as_slice(), *worker)))
            .collect();
        self.members
            .iter()
            .map(|member| {
                let split = split_body(&member.original_body, &member.positions)?;
                rewrite_core(&split, &by_id, &ProjectionRedexes::All)
            })
            .collect()
    }

    fn wrapper_bodies(&self, ids: &[GlobalId]) -> Option<Vec<Term>> {
        self.members
            .iter()
            .zip(ids)
            .map(|(member, worker)| {
                let mut domains = Vec::with_capacity(member.positions.len());
                let mut current = &member.original_type;
                for _ in &member.positions {
                    let Term::Pi(domain, codomain) = current else {
                        return None;
                    };
                    domains.push((**domain).clone());
                    current = codomain;
                }
                let levels = member
                    .level_params
                    .iter()
                    .copied()
                    .map(Level::Var)
                    .collect();
                let mut call = Term::const_(*worker, levels);
                for (index, refined) in member.positions.iter().enumerate() {
                    let arg = Term::var(member.positions.len() - 1 - index);
                    if refined.is_some() {
                        call =
                            Term::app(Term::app(call, Term::proj1(arg.clone())), Term::proj2(arg));
                    } else {
                        call = Term::app(call, arg);
                    }
                }
                Some(
                    domains
                        .into_iter()
                        .rev()
                        .fold(call, |body, domain| Term::lam(domain, body)),
                )
            })
            .collect()
    }
}

/// Return the public IDs and the corresponding optional private worker IDs.
/// The caller keeps the public name and obligation identity. The worker IDs
/// travel only through `ElabResult::auxiliary_ids` to artifact emission.
pub(super) fn admit_pending_refined(
    env: &mut GlobalEnv,
    pending: ken_kernel::PendingAdmission,
    bodies: Vec<Term>,
) -> Result<(Vec<GlobalId>, Vec<Option<GlobalId>>), (ken_kernel::KernelError, Vec<Decl>)> {
    let ids = pending.ids().to_vec();
    let original = ids
        .iter()
        .copied()
        .zip(bodies.iter().cloned())
        .collect::<Vec<_>>();
    if !matches!(
        ken_kernel::sct::sct_check(env, &original),
        Err(ken_kernel::KernelError::NotTerminating(_))
    ) {
        return ken_kernel::admit_pending(env, pending, bodies)
            .map(|ids| (ids, vec![None; original.len()]));
    }
    let Some(plan) = RefinedWorkerPlan::new(env, &ids, &bodies) else {
        return ken_kernel::admit_pending(env, pending, bodies)
            .map(|ids| (ids, vec![None; original.len()]));
    };
    let workers = match ken_kernel::stage_placeholders(env, plan.worker_specs()) {
        Ok(workers) => workers,
        Err(_) => {
            return ken_kernel::admit_pending(env, pending, bodies)
                .map(|ids| (ids, vec![None; original.len()]))
        }
    };
    let worker_ids = workers.ids().to_vec();
    let Some((worker_bodies, wrappers)) = plan
        .worker_bodies(&worker_ids)
        .zip(plan.wrapper_bodies(&worker_ids))
    else {
        let _ = ken_kernel::rollback_pending(env, workers);
        return ken_kernel::admit_pending(env, pending, bodies)
            .map(|ids| (ids, vec![None; original.len()]));
    };
    match ken_kernel::admit_pending_with_companions(env, workers, worker_bodies, pending, wrappers)
    {
        Ok(_) => Ok((ids, worker_ids.into_iter().map(Some).collect())),
        Err(refusal) => ken_kernel::admit_pending(env, refusal.companions, bodies)
            .map(|ids| (ids, vec![None; original.len()])),
    }
}
