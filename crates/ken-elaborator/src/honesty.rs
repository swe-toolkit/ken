//! Transitive proof honesty (`21 §5.4`, `23 §1.3`).
use std::collections::{BTreeSet, HashSet};

use ken_kernel::{postulates_reachable, Decl, GlobalEnv, GlobalId, Term};

/// Every postulate the verification layer minted as an obligation hole,
/// recorded by identity when minted. Never derived from a declaration's
/// label, type or `trusted_base()` membership: the kernel records holes and
/// audited contract axioms identically (`18 §5`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ObligationHoles(BTreeSet<GlobalId>);

impl ObligationHoles {
    pub fn record(&mut self, id: GlobalId) {
        self.0.insert(id);
    }

    pub(crate) fn forget(&mut self, removed: &HashSet<GlobalId>) {
        self.0.retain(|id| !removed.contains(id));
    }

    /// The open obligation holes `term` relies on.
    pub fn reached_open(&self, env: &GlobalEnv, term: &Term) -> BTreeSet<GlobalId> {
        postulates_reachable(env, term)
            .into_iter()
            .filter(|id| self.0.contains(id))
            .collect()
    }

    /// A claim (an obligation hole, or a `theorem`/`proof` definition) reads
    /// `proved` only when it is a checked transparent definition whose
    /// closure reaches no open obligation hole.
    pub fn claim_proved(&self, env: &GlobalEnv, claim: GlobalId) -> bool {
        matches!(env.lookup(claim), Some(Decl::Transparent { .. }))
            && self
                .reached_open(env, &Term::const_(claim, vec![]))
                .is_empty()
    }
}
