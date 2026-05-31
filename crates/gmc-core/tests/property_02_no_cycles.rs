//! Property 2 of the `gmc-core-protocol` design: **派生树永不成环**.
//!
//! > *对任意* 包含派生与重挂接（re-parent）操作的请求序列，功勋链注册表在任意时刻
//! > 都不存在环路（不存在某链成为自身祖先）；任何会形成环路的请求都被拒绝
//! > （`CycleConflict`），且被拒后注册表状态保持不变。
//!
//! **Validates: Requirements 1.5**
//!
//! `Chain_Registry` exposes no public `reparent` and its `detect_cycle` guard is
//! private, so this test drives everything through the public
//! [`ChainRegistry::derive`]. Re-parenting / cycle attempts are expressed as
//! `derive` requests that name an **already-existing** chain as the `proposed_id`:
//!
//! - a *self-parent* request (`proposed_id == parent_id`), and
//! - a *re-derive under a descendant* request (`proposed_id` is some existing chain,
//!   `parent_id` lives in that chain's subtree),
//!
//! both of which `derive` must reject with [`GmcError::CycleConflict`] (the cycle
//! guard runs before the depth / `(parent, domain)` checks). The generated
//! [`DerivationOp`] sequence supplies both shapes: `Derive` ops can collide
//! (`proposed_id == parent_id`, or an already-derived id), and `Reparent` ops map to
//! re-deriving an existing id beneath a new parent — the canonical cycle risk.

mod common;

use std::collections::BTreeSet;

use common::generators::{self, DerivationOp};
use gmc_core::error::GmcError;
use gmc_core::registry::{ChainRegistry, DeriveRequest, NestedMeritChain, OriginType};
use gmc_core::types::{ChainId, FayID, Timestamp};
use proptest::prelude::*;

/// Root id `chain-0` — deliberately drawn from the generator's `chain-0..chain-15`
/// pool so generated `Derive` ops can name it as a parent and actually grow the tree
/// (a root outside the pool would never be a valid `parent_id`, leaving the tree
/// stuck at a single node).
fn root_id() -> ChainId {
    ChainId::new("chain-0")
}

/// A fresh registry holding only the `GMC_Base` depth-0 root (`chain-0`).
fn registry_with_root() -> ChainRegistry {
    let root = NestedMeritChain::root(
        root_id(),
        "root-domain",
        vec![FayID::new("founder")],
        Timestamp::from_secs(1_000),
    );
    ChainRegistry::with_root(root).expect("chain-0 is a valid depth-0 root")
}

/// Maps a generated [`DerivationOp`] onto a real [`DeriveRequest`].
///
/// `Reparent { chain_id, new_parent_id }` is expressed as "re-derive the existing
/// `chain_id` under `new_parent_id`" — the parent-pointer change the cycle guard must
/// reject when `new_parent_id` sits in `chain_id`'s subtree (or equals it). A
/// non-empty domain is synthesised so the request clears the `MissingField` check and
/// actually reaches the cycle guard.
fn op_to_request(op: &DerivationOp, seq: usize) -> DeriveRequest {
    let stewards = vec![FayID::new("steward-1")];
    let created_at = Timestamp::from_secs(2_000 + seq as u64);
    match op {
        DerivationOp::Derive {
            proposed_id,
            parent_id,
            domain,
        } => DeriveRequest::new(
            proposed_id.clone(),
            parent_id.clone(),
            domain.clone(),
            stewards,
            OriginType::StewardInitiated,
            created_at,
        ),
        DerivationOp::Reparent {
            chain_id,
            new_parent_id,
        } => DeriveRequest::new(
            chain_id.clone(),
            new_parent_id.clone(),
            format!("reparent-{chain_id}"),
            stewards,
            OriginType::StewardInitiated,
            created_at,
        ),
    }
}

/// Independent acyclicity oracle: walks every chain's `parentId` pointers up toward
/// the root and confirms the tree contains no cycle.
///
/// For each chain we follow parent links, recording every node we visit. The walk is
/// well-formed (and the tree acyclic) iff:
///
/// - no node is visited twice along a single walk (revisiting ⇒ a chain is its own
///   ancestor ⇒ cycle),
/// - the walk never takes more steps than there are chains (a belt-and-braces guard
///   against an undetected loop), and
/// - the parent chain terminates at the unique parentless node, the root.
///
/// Returns `true` exactly when the whole registry is a single acyclic tree rooted at
/// `root_id`.
fn tree_is_acyclic(registry: &ChainRegistry) -> bool {
    let root = registry.root_id();
    for chain in registry.iter() {
        let mut visited: BTreeSet<ChainId> = BTreeSet::new();
        let mut cursor = Some(chain.id().clone());
        let mut steps = 0usize;
        while let Some(current) = cursor {
            if !visited.insert(current.clone()) {
                // Revisited a node => the parent chain loops back on itself.
                return false;
            }
            steps += 1;
            if steps > registry.len() + 1 {
                // Walked more nodes than exist => an undetected cycle.
                return false;
            }
            match registry.get(&current) {
                Some(node) => cursor = node.parent_id().cloned(),
                // A dangling parent pointer would also be a structural defect.
                None => return false,
            }
        }
        // The parentless terminus of every walk must be the registry root.
        if !visited.contains(root) {
            return false;
        }
    }
    true
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 2: 派生树永不成环
    #[test]
    fn property_2_derivation_tree_never_cycles(
        ops in generators::derivation_sequence(32),
    ) {
        let mut registry = registry_with_root();
        // The invariant holds before any operation (root-only tree).
        prop_assert!(tree_is_acyclic(&registry), "fresh root-only registry must be acyclic");

        for (seq, op) in ops.iter().enumerate() {
            // Snapshot the full chain set (deterministic id order) before the op.
            // `ChainRegistry` is not `PartialEq`, but `NestedMeritChain` is, so the
            // collected vector is a faithful, comparable view of registry state.
            let before: Vec<NestedMeritChain> = registry.iter().cloned().collect();

            let result = registry.derive(op_to_request(op, seq));

            // A request that would form a cycle is rejected, and the rejection leaves
            // the registry byte-for-byte unchanged (validate up front, fail atomically).
            if matches!(result, Err(GmcError::CycleConflict)) {
                let after: Vec<NestedMeritChain> = registry.iter().cloned().collect();
                prop_assert_eq!(
                    before,
                    after,
                    "registry state must be unchanged after a CycleConflict rejection"
                );
            }

            // Regardless of whether the op was accepted or rejected (and for whatever
            // reason), the tree must never contain a cycle at any point in time.
            prop_assert!(
                tree_is_acyclic(&registry),
                "derivation tree must remain acyclic after every operation (op #{seq}: {op:?})"
            );
        }
    }
}
