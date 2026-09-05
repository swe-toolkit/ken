//! Host-internal revocation lineage for the current process authority domain.
//!
//! The domain stores only opaque node identities and parent links. Effective
//! liveness is computed by walking the addressed node and every ancestor; no
//! validity cell, pointer, or reference crosses the host boundary.

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct RevocationNodeId(u64);

struct RevocationNode {
    id: RevocationNodeId,
    parent: Option<RevocationNodeId>,
    locally_live: bool,
}

#[derive(Default)]
pub(crate) struct RevocationDomain {
    nodes: Vec<RevocationNode>,
    next_node_id: u64,
}

#[allow(dead_code)] // D0 substrate; dispatch consumption lands in ABI-REVOKE-D1.
impl RevocationDomain {
    pub(crate) fn mint_root(&mut self) -> RevocationNodeId {
        self.mint_node(None)
    }

    pub(crate) fn attenuate(&mut self, parent: RevocationNodeId) -> Option<RevocationNodeId> {
        self.node(parent)?;
        Some(self.mint_node(Some(parent)))
    }

    pub(crate) fn copy(&self, node: RevocationNodeId) -> Option<RevocationNodeId> {
        self.node(node).map(|_| node)
    }

    pub(crate) fn revoke(&mut self, node: RevocationNodeId) -> bool {
        let Some(node) = self.nodes.iter_mut().find(|candidate| candidate.id == node) else {
            return false;
        };
        node.locally_live = false;
        true
    }

    pub(crate) fn is_admissible(&self, node: RevocationNodeId) -> bool {
        let mut current = Some(node);
        while let Some(id) = current {
            let Some(node) = self.node(id) else {
                return false;
            };
            if !node.locally_live {
                return false;
            }
            current = node.parent;
        }
        true
    }

    fn mint_node(&mut self, parent: Option<RevocationNodeId>) -> RevocationNodeId {
        self.next_node_id = self
            .next_node_id
            .checked_add(1)
            .expect("revocation node identity exhausted");
        let id = RevocationNodeId(self.next_node_id);
        self.nodes.push(RevocationNode {
            id,
            parent,
            locally_live: true,
        });
        id
    }

    fn node(&self, id: RevocationNodeId) -> Option<&RevocationNode> {
        self.nodes.iter().find(|node| node.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;

    /// Promise class: durable invariant. MEASURED: copying a live node returns
    /// the same nominal identity and does not change the domain population.
    /// CLAIMED: capability copy preserves lineage and mints no node. THE GAP:
    /// capability-slot wiring is deliberately outside D0 and begins in D1.
    #[test]
    fn copy_preserves_node_identity_without_minting() {
        let mut domain = RevocationDomain::default();
        let root = domain.mint_root();
        let population_before = domain.nodes.len();

        let copied = domain.copy(root).expect("a minted root is copyable");

        assert!(copied == root, "copy must preserve the exact node identity");
        assert_eq!(
            domain.nodes.len(),
            population_before,
            "copy must not mint a revocation node"
        );
    }

    /// Promise class: durable invariant. MEASURED: attenuation adds one child
    /// whose stored parent is the supplied live root. CLAIMED: attenuation
    /// creates the lineage edge used for transitive revocation. THE GAP:
    /// authority narrowing and capability-slot lineage are D1 concerns.
    #[test]
    fn attenuation_creates_a_child_with_the_supplied_parent_link() {
        let mut domain = RevocationDomain::default();
        let root = domain.mint_root();

        let child = domain
            .attenuate(root)
            .expect("a minted root accepts an attenuated child");
        let child_node = domain.node(child).expect("the child remains in the domain");

        assert!(child_node.parent == Some(root));
        assert!(domain.is_admissible(child));
    }

    /// Promise class: durable invariant. MEASURED: revoking one branch makes
    /// that node and a depth-two descendant inadmissible while its parent and
    /// sibling remain admissible. CLAIMED: revoke closes exactly the addressed
    /// subtree, to arbitrary parent-linked depth. THE GAP: this isolated tree
    /// test does not claim that host dispatch consults the domain before D1.
    #[test]
    fn revoke_closes_the_addressed_subtree_but_not_parent_or_sibling() {
        let mut domain = RevocationDomain::default();
        let root = domain.mint_root();
        let branch = domain.attenuate(root).expect("branch");
        let leaf = domain.attenuate(branch).expect("leaf");
        let sibling = domain.attenuate(root).expect("sibling");

        assert!(domain.revoke(branch));

        assert!(domain.is_admissible(root), "the parent must remain live");
        assert!(
            domain.is_admissible(sibling),
            "a sibling outside the subtree must remain live"
        );
        assert!(!domain.is_admissible(branch));
        assert!(!domain.is_admissible(leaf));
    }

    /// Promise class: durable invariant. MEASURED: root and child identities
    /// increase strictly, and minting after revoke does not reuse either value;
    /// Rust TypeId also distinguishes the node from both host token types.
    /// CLAIMED: revocation identities are monotonic, never reused, and occupy a
    /// distinct nominal id space. THE GAP: numeric exhaustion fails closed by
    /// panic and is not made recoverable by this bounded synchronous substrate.
    #[test]
    fn node_ids_are_monotonic_never_reused_and_nominally_distinct() {
        let mut domain = RevocationDomain::default();
        let first = domain.mint_root();
        let second = domain.attenuate(first).expect("second node");
        assert!(domain.revoke(first));
        let third = domain.mint_root();

        assert!(first.0 < second.0 && second.0 < third.0);
        assert!(first != third && second != third);
        assert_ne!(
            TypeId::of::<RevocationNodeId>(),
            TypeId::of::<crate::CapabilityTokenV1>()
        );
        assert_ne!(
            TypeId::of::<RevocationNodeId>(),
            TypeId::of::<crate::ResourceTokenV1>()
        );
    }

    /// Promise class: durable invariant plus transition control. MEASURED: a
    /// leaf is observed admissible, then revoking its root makes the same leaf
    /// inadmissible without directly revoking the leaf. CLAIMED: admission
    /// re-walks every ancestor and cannot trust a cached leaf-live bit. THE GAP:
    /// D0 proves the domain query only; D1 wires it into host-op admission.
    #[test]
    fn admissibility_rechecks_ancestors_after_leaf_was_observed_live() {
        let mut domain = RevocationDomain::default();
        let root = domain.mint_root();
        let child = domain.attenuate(root).expect("child");
        let leaf = domain.attenuate(child).expect("leaf");

        assert!(domain.is_admissible(leaf));
        assert!(domain.revoke(root));
        assert!(
            domain
                .node(leaf)
                .expect("leaf remains allocated")
                .locally_live,
            "the control requires the leaf-local cache to remain live"
        );
        assert!(!domain.is_admissible(leaf));
    }
}
