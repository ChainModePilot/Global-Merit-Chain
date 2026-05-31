//! Property 3 (层级深度上界) property-based test for `Chain_Registry`.
//!
//! Design property (Property 3, _Validates: Requirements 1.7_):
//!
//! > *对任意* 派生请求序列，注册表中每条链的 `depth` 都不超过 16（以 `GMC_Base`
//! > 为 depth=0）；任何会使深度超过 16 的派生请求都被拒绝，且被拒后注册表状态
//! > 保持不变。
//!
//! Per the suite convention (`tests/common/mod.rs`) every numbered property is
//! implemented by **exactly one** proptest test, run with **>= 100** iterations and
//! labelled `Feature: gmc-core-protocol, Property N: ...`. This file owns Property 3.
//!
//! The single property test below drives a [`ChainRegistry`] with a generated
//! sequence of derive requests that deliberately tries to build **deep** chains
//! (mostly extending the current deepest chain, occasionally branching elsewhere).
//! Because every request uses a globally-unique proposed id and a globally-unique
//! domain and always targets an existing parent, the *only* reason a derive can fail
//! is the depth bound — which isolates Property 3 cleanly. After each operation the
//! test asserts:
//!
//! - every chain in the registry has `depth() <= MAX_DEPTH (16)`; and
//! - any [`GmcError::DepthExceeded`] rejection leaves the registry unchanged
//!   (snapshot equality of the full chain set before/after — `ChainRegistry` itself
//!   is not `PartialEq`, but `Vec<NestedMeritChain>` is).
//!
//! A separate deterministic unit test (not a numbered property) builds a straight
//! chain to exactly depth 16 and asserts the 17th derive is rejected with
//! `DepthExceeded`, pinning the boundary the random search may or may not hit.

use gmc_core::error::GmcError;
use gmc_core::registry::{ChainRegistry, DeriveRequest, NestedMeritChain, OriginType, MAX_DEPTH};
use gmc_core::types::{ChainId, FayID, Timestamp};
use proptest::prelude::*;

/// The fixed `GMC_Base` root id used by every registry in this file.
const ROOT_ID: &str = "gmc-base";

/// Builds a fresh registry seeded with the `GMC_Base` depth-0 root.
fn new_registry() -> ChainRegistry {
    let root = NestedMeritChain::root(
        ChainId::new(ROOT_ID),
        "root",
        vec![FayID::new("founder")],
        Timestamp::from_secs(0),
    );
    ChainRegistry::with_root(root).expect("GMC_Base is a valid depth-0 root")
}

/// One step in a generated derive sequence: where to attach the next new chain.
#[derive(Debug, Clone)]
enum DeriveMove {
    /// Derive under the *current deepest* chain (pushes the tree deeper, so the
    /// depth bound is exercised quickly).
    ExtendDeepest,
    /// Derive under an existing chain selected by index (taken modulo the registry
    /// size), so the sequence also branches instead of only going straight down.
    BranchAt(usize),
}

/// A single move, weighted toward `ExtendDeepest` so generated runs reliably reach
/// (and then try to exceed) the depth-16 bound.
fn derive_move() -> impl Strategy<Value = DeriveMove> {
    prop_oneof![
        3 => Just(DeriveMove::ExtendDeepest),
        1 => (0usize..64).prop_map(DeriveMove::BranchAt),
    ]
}

/// A sequence of `0..=max_len` derive moves.
fn derive_moves(max_len: usize) -> impl Strategy<Value = Vec<DeriveMove>> {
    proptest::collection::vec(derive_move(), 0..=max_len)
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 3: 层级深度上界
    #[test]
    fn property_3_depth_bound(moves in derive_moves(40)) {
        let mut registry = new_registry();
        // Monotonic counter giving every derive a globally-unique id + domain, so the
        // only possible rejection cause across the whole run is the depth bound.
        let mut counter: u64 = 0;

        for mv in &moves {
            // Pick an existing parent and clone its id, so the immutable borrow of the
            // registry ends before the mutable `derive` call below.
            let parent_id: ChainId = match mv {
                DeriveMove::ExtendDeepest => registry
                    .iter()
                    .max_by_key(|chain| chain.depth())
                    .map(|chain| chain.id().clone())
                    .expect("registry always holds at least the GMC_Base root"),
                DeriveMove::BranchAt(k) => {
                    let len = registry.len();
                    registry
                        .iter()
                        .nth(k % len)
                        .map(|chain| chain.id().clone())
                        .expect("index is taken modulo a non-zero len")
                }
            };

            counter += 1;
            let req = DeriveRequest::new(
                ChainId::new(format!("gen-{counter}")),
                parent_id,
                format!("dom-{counter}"),
                vec![FayID::new("steward")],
                OriginType::StewardInitiated,
                Timestamp::from_secs(counter),
            );

            // Snapshot the full chain set before the derive so a depth-bound rejection
            // can be checked to leave the registry untouched.
            let before: Vec<NestedMeritChain> = registry.iter().cloned().collect();

            match registry.derive(req) {
                Ok(_) => {}
                Err(GmcError::DepthExceeded) => {
                    // After rejection the registry state must be unchanged.
                    let after: Vec<NestedMeritChain> = registry.iter().cloned().collect();
                    prop_assert_eq!(before, after);
                }
                Err(other) => {
                    // Unique ids/domains + always-existing parent ⇒ depth is the only
                    // legitimate failure here; anything else is a test/logic bug.
                    prop_assert!(false, "unexpected derive error: {:?}", other);
                }
            }

            // Core invariant: after every operation, no chain ever exceeds MAX_DEPTH.
            for chain in registry.iter() {
                prop_assert!(
                    chain.depth() <= MAX_DEPTH,
                    "chain {} reached depth {} > MAX_DEPTH {}",
                    chain.id(),
                    chain.depth(),
                    MAX_DEPTH
                );
            }
        }
    }
}

/// Deterministic boundary check (NOT a numbered property): build a straight chain to
/// exactly depth 16, then assert the 17th derive is rejected with `DepthExceeded` and
/// leaves the registry unchanged. This pins the exact boundary regardless of whether
/// any random run reaches it.
#[test]
fn building_to_depth_16_then_17th_derive_is_rejected() {
    let mut registry = new_registry();
    let mut parent = ChainId::new(ROOT_ID);

    // Derive depths 1..=16 in a single straight line (each child is parent.depth + 1).
    for d in 1..=MAX_DEPTH {
        let id = ChainId::new(format!("depth-{d}"));
        let req = DeriveRequest::new(
            id.clone(),
            parent.clone(),
            format!("domain-{d}"),
            vec![FayID::new("steward")],
            OriginType::StewardInitiated,
            Timestamp::from_secs(d as u64),
        );
        registry
            .derive(req)
            .expect("a derive that stays within the depth bound succeeds");
        parent = id;
    }

    // The deepest chain now sits exactly at MAX_DEPTH (16).
    let deepest = registry
        .iter()
        .map(|chain| chain.depth())
        .max()
        .expect("registry is non-empty");
    assert_eq!(deepest, MAX_DEPTH);

    // A 17th derive under the depth-16 chain would be depth 17 > 16 ⇒ rejected.
    let before: Vec<NestedMeritChain> = registry.iter().cloned().collect();
    let req = DeriveRequest::new(
        ChainId::new("depth-17"),
        parent,
        "domain-17".to_string(),
        vec![FayID::new("steward")],
        OriginType::StewardInitiated,
        Timestamp::from_secs(17),
    );
    assert_eq!(registry.derive(req), Err(GmcError::DepthExceeded));

    // Rejection left the registry unchanged, and the invariant still holds.
    let after: Vec<NestedMeritChain> = registry.iter().cloned().collect();
    assert_eq!(before, after);
    assert!(registry.iter().all(|chain| chain.depth() <= MAX_DEPTH));
}
