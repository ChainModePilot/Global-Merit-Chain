//! `GMC_Base` — root node (depth = 0) and the monetary-exchange rejection choke point.
//!
//! `GMC_Base` is the single root of the whole derivation hierarchy. Per the design's
//! *GMC_Base* component it has two responsibilities:
//!
//! 1. **Root of the derivation tree (Requirement 1.1).** It sits at depth `0` under a
//!    fixed [`ChainId`] and records the *top-level contribution behavior categories*
//!    that domain branches (`Nested_Merit_Chain`s) later derive from.
//! 2. **Monetary-exchange choke point (Requirement 11.8).** It is the single
//!    interception point for the protocol's "no buying recognition" rule: any request
//!    that funds currency to exchange for MeriToken, or that tries to purchase
//!    contribution recognition, is rejected unconditionally — nothing is minted, no
//!    recognition result is changed, and [`GmcError::OperationNotAllowed`] is returned.
//!
//! This is pure logic with no L1/L2 wiring; anchoring the root config to L1 and the
//! Substrate/rollup integration happen in later integration tasks.

use std::collections::BTreeSet;

use crate::error::{GmcError, GmcResult};
use crate::types::ChainId;

/// The kind of disallowed monetary request intercepted by [`GmcBase`].
///
/// Both variants map to the same outcome (rejection); the distinction exists only so
/// callers and audit logs can record *why* a request was disallowed. Per Requirement
/// 11.8 the rejection is unconditional regardless of variant, requester or amount.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MonetaryRequestKind {
    /// Funding currency in order to exchange for / mint MeriToken.
    FundForMeriToken,
    /// Paying money to purchase a contribution-recognition result.
    PurchaseRecognition,
}

/// A monetary-funding / purchase-of-recognition request presented to [`GmcBase`].
///
/// The protocol forbids *any* path that turns money into MeriToken or into a
/// recognition result. This struct models such a request purely so the rejection can
/// be expressed as a typed operation; its contents never influence the outcome —
/// [`GmcBase::reject_monetary_request`] always rejects it (Requirement 11.8).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MonetaryRequest {
    /// Why the request is considered a monetary exchange.
    pub kind: MonetaryRequestKind,
    /// The chain the request targets (e.g. where it hoped to mint / buy recognition).
    pub target_chain: ChainId,
    /// Opaque description of the offered funding (free-form; never trusted/credited).
    pub offered_funding: String,
}

impl MonetaryRequest {
    /// Convenience constructor for a "fund currency to obtain MeriToken" request.
    pub fn fund_for_meritoken(
        target_chain: impl Into<ChainId>,
        offered_funding: impl Into<String>,
    ) -> Self {
        MonetaryRequest {
            kind: MonetaryRequestKind::FundForMeriToken,
            target_chain: target_chain.into(),
            offered_funding: offered_funding.into(),
        }
    }

    /// Convenience constructor for a "purchase contribution recognition" request.
    pub fn purchase_recognition(
        target_chain: impl Into<ChainId>,
        offered_funding: impl Into<String>,
    ) -> Self {
        MonetaryRequest {
            kind: MonetaryRequestKind::PurchaseRecognition,
            target_chain: target_chain.into(),
            offered_funding: offered_funding.into(),
        }
    }
}

/// The GMC root node (`GMC_Base`), at derivation depth `0`.
///
/// Holds the registry of top-level contribution behavior categories (Requirement 1.1)
/// and exposes the unconditional monetary-request rejection (Requirement 11.8).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GmcBase {
    /// Top-level contribution behavior categories recorded at the root. A
    /// `BTreeSet` keeps them unique and in a stable, deterministic order.
    categories: BTreeSet<String>,
}

impl GmcBase {
    /// The fixed identifier of the root chain (`GMC_Base`).
    pub const ROOT_CHAIN_ID: &'static str = "gmc-base";

    /// The derivation depth of the root node. `GMC_Base` is always depth `0`.
    pub const ROOT_DEPTH: u32 = 0;

    /// Creates an empty `GMC_Base` root node with no recorded categories.
    pub fn new() -> Self {
        GmcBase {
            categories: BTreeSet::new(),
        }
    }

    /// Returns the fixed root chain identifier (depth `0`).
    ///
    /// This is the anchor every derivation `path` starts from (Requirement 1.1). It is
    /// an associated function — the root id is a protocol constant, independent of any
    /// particular `GmcBase` instance.
    pub fn root_chain_id() -> ChainId {
        ChainId::new(Self::ROOT_CHAIN_ID)
    }

    /// Records a top-level contribution behavior category at the root (Requirement 1.1).
    ///
    /// Returns `true` if the category was newly recorded, or `false` if an identical
    /// category had already been recorded (recording is idempotent). Whitespace-only
    /// or empty names are not recorded and return `false`.
    pub fn record_top_level_category(&mut self, category: impl Into<String>) -> bool {
        let category = category.into();
        if category.trim().is_empty() {
            return false;
        }
        self.categories.insert(category)
    }

    /// Returns `true` if `category` has been recorded at the root.
    pub fn has_top_level_category(&self, category: &str) -> bool {
        self.categories.contains(category)
    }

    /// Number of distinct top-level categories recorded at the root.
    pub fn category_count(&self) -> usize {
        self.categories.len()
    }

    /// Iterates over the recorded top-level categories in stable, sorted order.
    pub fn top_level_categories(&self) -> impl Iterator<Item = &str> + '_ {
        self.categories.iter().map(String::as_str)
    }

    /// Rejects any monetary-funding / purchase-of-recognition request (Requirement 11.8).
    ///
    /// This **always** returns `Err(`[`GmcError::OperationNotAllowed`]`)`, regardless of
    /// the request's kind, target chain or offered funding. It takes `&self` (an
    /// immutable borrow), which makes the "mint nothing, change no recognition result"
    /// guarantee structural: the method cannot mutate any root state — including the
    /// recorded categories — even in principle.
    pub fn reject_monetary_request(&self, _request: &MonetaryRequest) -> GmcResult<()> {
        Err(GmcError::OperationNotAllowed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_chain_id_is_the_fixed_depth_zero_root() {
        // Requirement 1.1: GMC_Base is the depth-0 root under a fixed identifier.
        assert_eq!(GmcBase::root_chain_id(), ChainId::new("gmc-base"));
        assert_eq!(GmcBase::root_chain_id().as_str(), GmcBase::ROOT_CHAIN_ID);
        assert_eq!(GmcBase::ROOT_DEPTH, 0);
    }

    #[test]
    fn recorded_top_level_category_is_retrievable() {
        // Requirement 1.1: top-level contribution behavior categories are recorded.
        let mut base = GmcBase::new();
        assert_eq!(base.category_count(), 0);

        assert!(base.record_top_level_category("academic"));
        assert!(base.record_top_level_category("public-good"));

        assert!(base.has_top_level_category("academic"));
        assert!(base.has_top_level_category("public-good"));
        assert!(!base.has_top_level_category("environment"));
        assert_eq!(base.category_count(), 2);

        let recorded: Vec<&str> = base.top_level_categories().collect();
        assert_eq!(recorded, vec!["academic", "public-good"]);
    }

    #[test]
    fn recording_is_idempotent_and_ignores_empty_names() {
        let mut base = GmcBase::new();
        assert!(base.record_top_level_category("art"));
        // Re-recording the same category does not duplicate it.
        assert!(!base.record_top_level_category("art"));
        // Empty / whitespace-only names are not recorded.
        assert!(!base.record_top_level_category(""));
        assert!(!base.record_top_level_category("   "));
        assert_eq!(base.category_count(), 1);
    }

    #[test]
    fn reject_monetary_request_always_returns_operation_not_allowed() {
        // Requirement 11.8: any monetary-funding / purchase-of-recognition request is
        // rejected with OperationNotAllowed.
        let base = GmcBase::new();

        let fund = MonetaryRequest::fund_for_meritoken("academic", "1000 USD");
        assert_eq!(
            base.reject_monetary_request(&fund),
            Err(GmcError::OperationNotAllowed)
        );

        let purchase = MonetaryRequest::purchase_recognition("public-good", "5 BTC");
        assert_eq!(
            base.reject_monetary_request(&purchase),
            Err(GmcError::OperationNotAllowed)
        );
    }

    #[test]
    fn rejected_monetary_request_mutates_no_state() {
        // Requirement 11.8: a rejected request mints nothing and changes no recognition
        // result. Here we assert the recorded top-level categories are untouched by a
        // rejected monetary request (it neither adds, removes, nor alters categories).
        let mut base = GmcBase::new();
        base.record_top_level_category("academic");
        base.record_top_level_category("environment");

        let before = base.clone();

        let req = MonetaryRequest::fund_for_meritoken("academic", "huge pile of money");
        let result = base.reject_monetary_request(&req);

        assert_eq!(result, Err(GmcError::OperationNotAllowed));
        // State is byte-for-byte identical: no category added, removed or changed.
        assert_eq!(base, before);
        assert_eq!(base.category_count(), 2);
        assert!(base.has_top_level_category("academic"));
        assert!(base.has_top_level_category("environment"));
    }
}
