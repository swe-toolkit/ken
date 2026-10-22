//! Compiler-owned bracket roles on the normalized checked Term, before erasure.
//! No source occurrence, host operation, or Runtime expression selects a region.

use std::collections::BTreeMap;

use ken_kernel::{GlobalId, Term};

use crate::checked_core::StableSymbol;
use crate::prelude::PreludeEnv;

use super::{term_application_spine_exact, CompilerDriverError};

/// Paths address children in one normalized declaration, not runtime values.
/// The Ok resource binder is the region key. The current linear bind places
/// one static finalizer per binder even if several dynamic Ret leaves reach
/// it; a future normalizer may copy that continuation. This is compiler-only
/// preparation, not a host or runtime ABI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NormalizedBracketRegion {
    pub owner: StableSymbol,
    pub acquire_vis_path: Vec<usize>,
    pub acquire_response_binder_path: Vec<usize>,
    pub ok_resource_binder_path: Vec<usize>,
    pub finalizer_paths: Vec<Vec<usize>>,
}

struct Binder<'a> {
    path: Vec<usize>,
    ty: &'a Term,
}

struct Acquire<'a> {
    vis_path: Vec<usize>,
    response_binder_path: Vec<usize>,
    response_ty: &'a Term,
}

struct OkArm {
    resource_binder_path: Vec<usize>,
    acquire_vis_path: Vec<usize>,
    acquire_response_binder_path: Vec<usize>,
}

struct Collector<'a> {
    owner: &'a StableSymbol,
    prelude: &'a PreludeEnv,
    resource_id: GlobalId,
    ok_method: usize,
    regions: BTreeMap<Vec<usize>, NormalizedBracketRegion>,
}

fn constructor_head(term: &Term) -> Option<(GlobalId, Vec<&Term>)> {
    let (head, args) = term_application_spine_exact(term);
    match head {
        Term::Constructor { id, .. } => Some((*id, args)),
        _ => None,
    }
}

impl<'a> Collector<'a> {
    fn invalid(&self, path: &[usize], reason: &'static str) -> CompilerDriverError {
        CompilerDriverError::InvalidBracketRegion {
            symbol: self.owner.clone(),
            path: path.to_vec(),
            reason,
        }
    }

    fn is_resource_type(&self, term: &Term) -> bool {
        let (head, args) = term_application_spine_exact(term);
        matches!(head, Term::Const { id, .. } if *id == self.resource_id) && args.len() == 1
    }

    fn acquire<'t>(
        &self,
        term: &'t Term,
        path: &[usize],
    ) -> Result<Option<Acquire<'t>>, CompilerDriverError> {
        let Some((vis, args)) = constructor_head(term) else {
            return Ok(None);
        };
        if vis != self.prelude.vis_id || args.len() != 5 {
            return Ok(None);
        }
        // The checked InL operand has exactly one FSOp constructor at its
        // head. A nested resemblance in an argument is not an acquisition.
        let Some((in_l, injection)) = constructor_head(args[3]) else {
            return Ok(None);
        };
        if in_l != self.prelude.inl_id || injection.len() != 3 {
            return Ok(None);
        }
        let Some((op, _)) = constructor_head(injection[2]) else {
            return Ok(None);
        };
        if ![
            self.prelude.private_fs_open_id,
            self.prelude.private_buffer_allocate_id,
            self.prelude.private_mapping_allocate_id,
            self.prelude.private_mapping_acquire_file_id,
        ]
        .contains(&op)
        {
            return Ok(None);
        }
        // A normalized continuation may be directly a Lam or a wrapped
        // application whose final argument is that Lam. Its binder path is
        // computed from the actual Term rather than a guessed static offset.
        let mut response_path = vec![1];
        let mut response = args[4];
        while let Term::App(_, arg) = response {
            response_path.push(1);
            response = arg;
        }
        let Term::Lam(response_ty, _) = response else {
            return Err(self.invalid(path, "acquire Vis has no response continuation binder"));
        };
        let (head, result_args) = term_application_spine_exact(response_ty);
        if !matches!(head, Term::IndFormer { id, .. } if *id == self.prelude.result_id)
            && !matches!(head, Term::Const { id, .. } if *id == self.prelude.result_id)
        {
            return Err(self.invalid(path, "acquire response binder is not Result"));
        }
        if result_args.len() != 2 || !self.is_resource_type(result_args[1]) {
            return Err(self.invalid(path, "acquire response is not Result of Resource"));
        }
        let mut response_binder_path = path.to_vec();
        response_binder_path.extend(response_path);
        Ok(Some(Acquire {
            vis_path: path.to_vec(),
            response_binder_path,
            response_ty,
        }))
    }

    fn ok_arm(
        &self,
        term: &Term,
        index: usize,
        child: &Term,
        path: &[usize],
        binders: &[Binder<'_>],
        acquires: &[Acquire<'_>],
    ) -> Result<Option<OkArm>, CompilerDriverError> {
        let Term::Elim {
            fam,
            params,
            methods,
            scrut,
            ..
        } = term
        else {
            return Ok(None);
        };
        if *fam != self.prelude.result_id
            || index != params.len() + 1 + self.ok_method
            || methods.len() <= self.ok_method
        {
            return Ok(None);
        }
        let Term::Var(scrut_index) = scrut.as_ref() else {
            return Ok(None);
        };
        let Some(scrut_binder) = binders.iter().rev().nth(*scrut_index) else {
            return Ok(None);
        };
        let Some(acquire) = acquires.iter().rev().find(|acquire| {
            acquire.response_binder_path == scrut_binder.path
                && acquire.response_ty == scrut_binder.ty
        }) else {
            return Ok(None);
        };
        let Term::Lam(resource_ty, _) = child else {
            return Err(self.invalid(path, "acquire Ok method has no resource binder"));
        };
        let (_, response_args) = term_application_spine_exact(acquire.response_ty);
        if !self.is_resource_type(resource_ty) || resource_ty.as_ref() != response_args[1] {
            return Err(self.invalid(path, "acquire Ok binder differs from response Resource"));
        }
        let mut resource_binder_path = path.to_vec();
        resource_binder_path.push(index);
        Ok(Some(OkArm {
            resource_binder_path,
            acquire_vis_path: acquire.vis_path.clone(),
            acquire_response_binder_path: acquire.response_binder_path.clone(),
        }))
    }

    fn finalizer(
        &mut self,
        term: &Term,
        path: &[usize],
        binders: &[Binder<'_>],
        ok_arms: &[OkArm],
    ) -> Result<(), CompilerDriverError> {
        let Some((head, args)) = constructor_head(term) else {
            return Ok(());
        };
        if head != self.prelude.private_bracket_release_id || args.len() != 3 {
            return Ok(());
        }
        let Term::Var(index) = args[2] else {
            return Err(self.invalid(path, "finalizer operand is not an Ok resource variable"));
        };
        let Some(binder) = binders.iter().rev().nth(*index) else {
            return Err(self.invalid(path, "finalizer resource variable has no binder"));
        };
        let Some(ok) = ok_arms
            .iter()
            .rev()
            .find(|ok| ok.resource_binder_path == binder.path)
        else {
            return Err(self.invalid(path, "finalizer operand lacks its acquire Ok binder"));
        };
        let (_, kind) = term_application_spine_exact(binder.ty);
        if !self.is_resource_type(binder.ty) || kind[0] != args[1] {
            return Err(self.invalid(path, "finalizer kind differs from Ok resource"));
        }
        // A lexical binder path identifies exactly one Ok arm in this term.
        // It is not a count, constructor identity, enclosing declaration or
        // HostOpV1 key; the key would also group copied finalizer leaves.
        let region = self
            .regions
            .entry(ok.resource_binder_path.clone())
            .or_insert_with(|| NormalizedBracketRegion {
                owner: self.owner.clone(),
                acquire_vis_path: ok.acquire_vis_path.clone(),
                acquire_response_binder_path: ok.acquire_response_binder_path.clone(),
                ok_resource_binder_path: ok.resource_binder_path.clone(),
                finalizer_paths: Vec::new(),
            });
        region.finalizer_paths.push(path.to_vec());
        Ok(())
    }

    fn visit<'t>(
        &mut self,
        term: &'t Term,
        path: &mut Vec<usize>,
        binders: &mut Vec<Binder<'t>>,
        acquires: &mut Vec<Acquire<'t>>,
        ok_arms: &mut Vec<OkArm>,
    ) -> Result<(), CompilerDriverError> {
        let acquire = self.acquire(term, path)?;
        let pushed_acquire = acquire.is_some();
        if let Some(acquire) = acquire {
            acquires.push(acquire);
        }
        self.finalizer(term, path, binders, ok_arms)?;
        for (index, child) in term.children().into_iter().enumerate() {
            let binder_ty = match term {
                Term::Pi(ty, _) | Term::Lam(ty, _) | Term::Sigma(ty, _) if index == 1 => {
                    Some(ty.as_ref())
                }
                Term::Let { ty, .. } if index == 2 => Some(ty.as_ref()),
                _ => None,
            };
            let ok = self.ok_arm(term, index, child, path, binders, acquires)?;
            let pushed_ok = ok.is_some();
            if let Some(ok) = ok {
                ok_arms.push(ok);
            }
            if let Some(ty) = binder_ty {
                binders.push(Binder {
                    path: path.clone(),
                    ty,
                });
            }
            path.push(index);
            self.visit(child, path, binders, acquires, ok_arms)?;
            path.pop();
            if binder_ty.is_some() {
                binders.pop();
            }
            if pushed_ok {
                ok_arms.pop();
            }
        }
        if pushed_acquire {
            acquires.pop();
        }
        Ok(())
    }
}

pub(super) fn collect_normalized_bracket_regions(
    owner: &StableSymbol,
    term: &Term,
    prelude: &PreludeEnv,
    resource_id: GlobalId,
    ok_method: usize,
) -> Result<Vec<NormalizedBracketRegion>, CompilerDriverError> {
    let mut collector = Collector {
        owner,
        prelude,
        resource_id,
        ok_method,
        regions: BTreeMap::new(),
    };
    collector.visit(
        term,
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
    )?;
    Ok(collector.regions.into_values().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::empty_prelude_env;

    fn app(head: Term, arguments: impl IntoIterator<Item = Term>) -> Term {
        arguments.into_iter().fold(head, Term::app)
    }

    /// Promise class: collector's structural grouping rule, not a claim that
    /// this hand-built Term is reachable by a source program. Its two release
    /// applications share one Ok binder. With the identity omitted from the
    /// selector, both applications cease to create a region.
    #[test]
    fn identical_ok_binder_groups_two_leaves_only_with_confined_identity() {
        let mut prelude = empty_prelude_env();
        prelude.result_id = GlobalId(101);
        prelude.ok_id = GlobalId(102);
        prelude.vis_id = GlobalId(103);
        prelude.inl_id = GlobalId(104);
        prelude.private_fs_open_id = GlobalId(105);
        prelude.private_bracket_release_id = GlobalId(106);
        prelude.resource_id = GlobalId(107);
        let resource = app(
            Term::const_(prelude.resource_id, vec![]),
            [Term::constructor(GlobalId(108), vec![])],
        );
        let response = app(
            Term::indformer(prelude.result_id, vec![]),
            [Term::const_(GlobalId(109), vec![]), resource.clone()],
        );
        let release = app(
            Term::constructor(prelude.private_bracket_release_id, vec![]),
            [
                Term::const_(GlobalId(110), vec![]),
                Term::constructor(GlobalId(108), vec![]),
                Term::var(0),
            ],
        );
        let selected = Term::Elim {
            fam: prelude.result_id,
            level_args: vec![],
            params: vec![Term::const_(GlobalId(109), vec![]), resource.clone()],
            motive: Box::new(Term::ty(ken_kernel::Level::Zero)),
            methods: vec![
                Term::var(0),
                Term::lam(resource.clone(), Term::pair(release.clone(), release)),
            ],
            indices: vec![],
            scrut: Box::new(Term::var(0)),
        };
        let operation = app(
            Term::constructor(prelude.inl_id, vec![]),
            [
                Term::ty(ken_kernel::Level::Zero),
                Term::ty(ken_kernel::Level::Zero),
                app(
                    Term::constructor(prelude.private_fs_open_id, vec![]),
                    [Term::const_(GlobalId(110), vec![])],
                ),
            ],
        );
        let tree = app(
            Term::constructor(prelude.vis_id, vec![]),
            [
                Term::ty(ken_kernel::Level::Zero),
                Term::ty(ken_kernel::Level::Zero),
                Term::ty(ken_kernel::Level::Zero),
                operation,
                Term::lam(response, selected),
            ],
        );
        let owner = StableSymbol::new(
            crate::checked_core::SymbolNamespace::Declaration,
            vec!["synthetic-collector".to_string()],
        );
        let grouped =
            collect_normalized_bracket_regions(&owner, &tree, &prelude, prelude.resource_id, 1)
                .expect("both leaves have one Ok binder");
        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped[0].finalizer_paths.len(), 2);
        prelude.private_bracket_release_id = GlobalId(111);
        let absent =
            collect_normalized_bracket_regions(&owner, &tree, &prelude, prelude.resource_id, 1)
                .expect("omitting the constructor yields no region");
        assert!(absent.is_empty());
    }
}
