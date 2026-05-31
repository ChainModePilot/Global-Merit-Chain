//! Carbon-credit → MeriToken application scenario — **task 16.1**.
//!
//! This module implements the *model* and the *declaration-import* seam for the
//! environmental-protection ("环保") carbon-credit scenario (design *Data Models*
//! `CarbonCreditVoucher` + flow 4 "碳积分转 MeriToken").
//!
//! The core idea (Requirement 12.1): when the environmental-protection
//! `Nested_Merit_Chain` has the carbon-credit scenario enabled, a contribution
//! declaration whose evidence is a **verifiable carbon-credit voucher reference** is
//! *imported into the retroactive-declaration flow* rather than the standard
//! register → record → grant pipeline. Concretely, this module builds a
//! [`crate::retroactive::RetroactiveApplication`] from the voucher's evidence and
//! routes it through [`RetroactiveReviewModule::submit`]. Because import reuses the
//! retroactive intake, the stricter retroactive review/voting (task 15.2 — stakeholder
//! selection, `max(regular, 2/3)` threshold, ZK-private weighted voting) applies to
//! carbon declarations exactly as it does to any other retroactive claim.
//!
//! ## Requirements covered by task 16.1
//!
//! - **Requirement 12.1**: a carbon-credit voucher reference is accepted as evidence
//!   and imported into the retroactive-declaration flow (see
//!   [`CarbonCreditVoucher::import_to_retroactive_flow`]).
//! - **Requirement 12.4**: if the voucher reference fails replayability /
//!   verifiability validation (the [`EvidenceRef`] is not independently verifiable —
//!   i.e. not replayable, or missing its locator/hash, so it does not correspond to a
//!   traceable decarbonization action), the import is **rejected** with
//!   [`GmcError::EvidenceInvalid`]. Import routes through
//!   [`RetroactiveReviewModule::submit`], which performs exactly this check atomically:
//!   on rejection **no declaration is created**, so nothing is ever pushed into voting
//!   and — since minting/quota are strictly downstream of an approved vote — **nothing
//!   is minted and no quota is consumed**.
//! - **Requirements 12.2 / 12.3** (mint per the chain's `Evaluation_Mechanism` + the
//!   three-dimension scoring model, mapping decarbonization to dimensions and applying
//!   each dimension's inflation index, *after* the retroactive vote passes) are
//!   realized **downstream** by the retroactive vote (task 15.2) plus the already
//!   implemented `Scoring_Engine` / `Minting_Service`. Task 16.1 only ensures the
//!   carbon declaration carries enough (contributor / chain / description / occurrence
//!   time + verifiable evidence) to be scored and minted later; it does **not**
//!   re-implement scoring or minting.
//!
//! ## Scope boundary — conversion guard & quota accounting (task 16.2)
//!
//! The at-most-once conversion guard and quota accounting (Requirements 12.5 / 12.6 /
//! 12.7) are implemented by [`CarbonCreditVoucher::convert`]. A freshly constructed
//! voucher carries the [`converted`](CarbonCreditVoucher::is_converted) flag (initially
//! `false`) and the
//! [`converted_declaration_id`](CarbonCreditVoucher::converted_declaration_id)
//! (initially `None`); `convert` advances that state exactly once, atomically:
//!
//! - **Requirement 12.6** (double-conversion): converting an already-converted voucher
//!   returns [`GmcError::DoubleConversion`] and mints nothing, consumes no quota, and
//!   mutates neither the voucher nor the ledger.
//! - **Requirement 12.5** (quota accounting): a successful conversion charges the
//!   mint amount to the environmental-protection chain's current `Refresh_Period`
//!   quota via [`QuotaLedger::consume_quota`], so the running total never exceeds
//!   `Quota`.
//! - **Requirement 12.7** (at-most-once): only after quota is successfully consumed is
//!   the voucher marked `converted = true` (recording the declaration id), so it can
//!   never be converted again.
//!
//! > Note: the MeriToken minting into the pocket itself is `Minting_Service`'s job
//! > (task 9.3). [`CarbonCreditVoucher::convert`] handles the carbon-specific concerns
//! > — the double-conversion guard, the chain's quota charge, and marking the voucher
//! > converted — given the already-computed `mint_amount` from the three-dimension
//! > scoring/minting pipeline.
//!
//! ## Scenario gating (Requirement 12.1 `WHERE` clause)
//!
//! Requirement 12.1 is conditioned on the environmental-protection chain having the
//! carbon scenario *enabled*. That gate depends on the target chain's configuration in
//! `Chain_Registry`, which this module deliberately does not reach into. The end-to-end
//! wiring (task 20.1) is responsible for checking the scenario is enabled before
//! calling [`CarbonCreditVoucher::import_to_retroactive_flow`]; this module assumes that
//! precondition has already been satisfied by its caller.

use crate::error::{GmcError, GmcResult};
use crate::quota::{QuotaConfig, QuotaLedger};
use crate::retroactive::{
    DeclarationId, EvidenceRef, RetroactiveApplication, RetroactiveReviewModule,
};
use crate::types::{ChainId, Decimal, FayID, Timestamp};

/// A carbon-credit voucher and its conversion state (design data model
/// `CarbonCreditVoucher`).
///
/// A voucher pairs a unique [`voucher_id`](CarbonCreditVoucher::voucher_id) with a
/// **verifiable carbon-credit voucher reference** ([`EvidenceRef`], reused from the
/// retroactive module so it shares the exact replayability semantics reviewers rely
/// on). The [`converted`](CarbonCreditVoucher::is_converted) flag and
/// [`converted_declaration_id`](CarbonCreditVoucher::converted_declaration_id) record
/// whether — and via which declaration — this voucher was turned into MeriToken.
///
/// Fields are private with read-only accessors so the conversion state can only be
/// advanced through the controlled flow ([`convert`](CarbonCreditVoucher::convert),
/// task 16.2). A freshly constructed voucher is always **unconverted**:
/// `converted == false` and `converted_declaration_id == None`.
///
/// > **Invariant (enforced by [`convert`](CarbonCreditVoucher::convert)):** each
/// > `voucher_id` is converted at most once — once `converted` becomes `true` it never
/// > reverts, and any further conversion attempt is rejected with
/// > [`crate::error::GmcError::DoubleConversion`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarbonCreditVoucher {
    voucher_id: String,
    evidence_ref: EvidenceRef,
    /// `true` once this voucher has been converted into MeriToken (set by
    /// [`convert`](CarbonCreditVoucher::convert)).
    converted: bool,
    /// The declaration that consumed this voucher, set alongside `converted` by
    /// [`convert`](CarbonCreditVoucher::convert).
    converted_declaration_id: Option<String>,
}

impl CarbonCreditVoucher {
    /// Builds an **unconverted** carbon-credit voucher from its id and evidence
    /// reference.
    ///
    /// The conversion state starts cleared: [`is_converted`](Self::is_converted) is
    /// `false` and [`converted_declaration_id`](Self::converted_declaration_id) is
    /// `None` (Requirements 12.6 / 12.7 initial state). Advancing that state is task
    /// 16.2's responsibility.
    pub fn new(voucher_id: impl Into<String>, evidence_ref: EvidenceRef) -> Self {
        CarbonCreditVoucher {
            voucher_id: voucher_id.into(),
            evidence_ref,
            converted: false,
            converted_declaration_id: None,
        }
    }

    /// The voucher's unique identifier.
    pub fn voucher_id(&self) -> &str {
        &self.voucher_id
    }

    /// The verifiable carbon-credit voucher reference used as declaration evidence.
    pub fn evidence_ref(&self) -> &EvidenceRef {
        &self.evidence_ref
    }

    /// Whether this voucher has already been converted into MeriToken.
    ///
    /// Always `false` for a freshly [`new`](Self::new) voucher; set by
    /// [`convert`](Self::convert) on the first successful conversion.
    pub fn is_converted(&self) -> bool {
        self.converted
    }

    /// The id of the declaration that converted this voucher, or `None` if unconverted.
    pub fn converted_declaration_id(&self) -> Option<&str> {
        self.converted_declaration_id.as_deref()
    }

    /// Imports a carbon-credit contribution declaration into the retroactive flow
    /// (Requirement 12.1), using this voucher's reference as the evidence.
    ///
    /// This builds a [`RetroactiveApplication`] whose single evidence reference is the
    /// voucher's [`evidence_ref`](Self::evidence_ref) and submits it to `module`
    /// ([`RetroactiveReviewModule::submit`]). On success a retroactive declaration is
    /// created with status `Pending`, ready for the stricter retroactive review/voting
    /// (task 15.2); its [`DeclarationId`] is returned.
    ///
    /// Because intake is reused, validation is handled by `submit` **atomically**:
    ///
    /// - **Requirement 12.4 / 10.8:** if the voucher reference is not replayable —
    ///   i.e. not independently verifiable, so it does not correspond to a traceable
    ///   decarbonization action — `submit` returns [`GmcError::EvidenceInvalid`] and
    ///   **creates no declaration**. The claim is therefore never pushed into voting,
    ///   and since minting/quota are downstream of an approved vote, **nothing is
    ///   minted and no quota is consumed**.
    /// - **Requirement 10.1:** a missing core field (empty contributor / chain id, or
    ///   blank description) is rejected with [`GmcError::FieldValidation`], again
    ///   without creating any record.
    ///
    /// > **Conversion seam:** this method does **not** consult or update the voucher's
    /// > [`converted`](Self::is_converted) state. The double-conversion guard
    /// > ([`GmcError::DoubleConversion`]) and marking the voucher converted + charging
    /// > the environmental-protection chain's quota happen in
    /// > [`convert`](Self::convert), after a vote approves the declaration and minting
    /// > succeeds.
    ///
    /// [`GmcError::EvidenceInvalid`]: crate::error::GmcError::EvidenceInvalid
    /// [`GmcError::FieldValidation`]: crate::error::GmcError::FieldValidation
    /// [`GmcError::DoubleConversion`]: crate::error::GmcError::DoubleConversion
    pub fn import_to_retroactive_flow(
        &self,
        module: &mut RetroactiveReviewModule,
        contributor_id: FayID,
        chain_id: ChainId,
        description: impl Into<String>,
        occurred_at: Timestamp,
    ) -> GmcResult<DeclarationId> {
        let application = RetroactiveApplication::new(
            contributor_id,
            chain_id,
            description,
            occurred_at,
            vec![self.evidence_ref.clone()],
        );
        module.submit(application)
    }

    /// Converts this carbon-credit voucher into MeriToken **at most once**, atomically
    /// charging the mint amount to the environmental-protection chain's current
    /// `Refresh_Period` quota and marking the voucher converted (Requirements 12.5 /
    /// 12.6 / 12.7).
    ///
    /// This is invoked *after* a retroactive vote approves the carbon declaration and
    /// the three-dimension `Scoring_Engine` has computed `mint_amount`. It handles the
    /// carbon-specific concerns only — the double-conversion guard, the chain's quota
    /// charge, and marking the voucher converted. The MeriToken batch creation in the
    /// contributor's pocket is `Minting_Service`'s responsibility (task 9.3); the
    /// `mint_amount` passed here is the already-computed amount being charged to quota.
    ///
    /// ## Ordering & atomicity
    ///
    /// The three steps run in a fixed order so the operation is all-or-nothing:
    ///
    /// 1. **Double-conversion guard (Requirement 12.6).** If the voucher
    ///    [`is_converted`](Self::is_converted), return [`GmcError::DoubleConversion`]
    ///    **before** touching the ledger. Nothing is minted, no quota is consumed, and
    ///    neither the voucher nor `ledger` is mutated.
    /// 2. **Quota accounting (Requirement 12.5).** Charge `mint_amount` to the chain's
    ///    current period via [`QuotaLedger::consume_quota`]. That call is atomic: on
    ///    [`GmcError::QuotaExceeded`] the ledger counter is left **unchanged**, so when
    ///    we propagate the error the voucher also stays unconverted — no partial state.
    /// 3. **Mark converted (Requirement 12.7).** Only after quota was successfully
    ///    consumed do we set `converted = true` and record `declaration_id`, so this
    ///    voucher can never be converted again.
    ///
    /// This guarantees: a rejected double-conversion changes nothing; a quota failure
    /// leaves the voucher unconverted and the ledger untouched; and a success both
    /// consumes quota and marks the voucher converted exactly once. Because
    /// `consume_quota` enforces `minted_this_period + mint_amount <= Quota`, the
    /// converted amount is counted in the current period and the running total never
    /// exceeds `Quota` (Requirement 12.5 / Property 28).
    ///
    /// [`GmcError::DoubleConversion`]: crate::error::GmcError::DoubleConversion
    /// [`GmcError::QuotaExceeded`]: crate::error::GmcError::QuotaExceeded
    pub fn convert(
        &mut self,
        declaration_id: impl Into<String>,
        mint_amount: Decimal,
        config: &QuotaConfig,
        ledger: &mut QuotaLedger,
    ) -> GmcResult<()> {
        // (1) Double-conversion guard (Requirement 12.6): reject before any side
        // effect — no mint, no quota consumption, no mutation of voucher or ledger.
        if self.converted {
            return Err(GmcError::DoubleConversion);
        }

        // (2) Quota accounting (Requirement 12.5): charge the mint amount to the
        // chain's current Refresh_Period. consume_quota is atomic — on QuotaExceeded
        // the ledger counter is unchanged, so propagating the error here leaves the
        // voucher unconverted with no partial state.
        ledger.consume_quota(config, mint_amount)?;

        // (3) Mark converted (Requirement 12.7): only reached once quota was charged
        // successfully, so the voucher is converted exactly once and never again.
        self.converted = true;
        self.converted_declaration_id = Some(declaration_id.into());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::GmcError;
    use crate::quota::{QuotaConfig, QuotaLedger, RefreshPeriod, TimeUnit};
    use crate::retroactive::ReviewStatus;
    use crate::types::Decimal;

    /// A verifiable (replayable) carbon-credit voucher reference.
    fn verifiable_ref() -> EvidenceRef {
        EvidenceRef::new("ipfs://carbon-cid-001", "0xcarbonhash", true)
    }

    fn voucher_with(evidence: EvidenceRef) -> CarbonCreditVoucher {
        CarbonCreditVoucher::new("voucher-001", evidence)
    }

    // --- Requirement 12.6/12.7 initial state: fresh voucher is unconverted ---

    #[test]
    fn freshly_constructed_voucher_is_unconverted() {
        let voucher = voucher_with(verifiable_ref());
        assert_eq!(voucher.voucher_id(), "voucher-001");
        assert_eq!(voucher.evidence_ref(), &verifiable_ref());
        assert!(!voucher.is_converted());
        assert_eq!(voucher.converted_declaration_id(), None);
    }

    // --- Requirement 12.1: verifiable voucher imports into the retroactive flow ---

    #[test]
    fn verifiable_voucher_imports_as_pending_declaration() {
        let mut module = RetroactiveReviewModule::new();
        let voucher = voucher_with(verifiable_ref());

        let id = voucher
            .import_to_retroactive_flow(
                &mut module,
                FayID::new("eco-alice"),
                ChainId::new("carbon-reduction"),
                "Restored 5 hectares of wetland in 2023, certified carbon credits.",
                Timestamp::from_secs(1_650_000_000),
            )
            .expect("a verifiable voucher must import into the retroactive flow");

        // A Pending declaration was created, carrying the voucher's evidence.
        let declaration = module.get(&id).expect("declaration stored");
        assert_eq!(declaration.review_status(), ReviewStatus::Pending);
        assert_eq!(declaration.contributor_id(), &FayID::new("eco-alice"));
        assert_eq!(declaration.chain_id(), &ChainId::new("carbon-reduction"));
        assert_eq!(declaration.evidence_refs(), &[verifiable_ref()]);
        assert_eq!(module.len(), 1);

        // Task 16.1 does not touch conversion state — that is task 16.2's seam.
        assert!(!voucher.is_converted());
        assert_eq!(voucher.converted_declaration_id(), None);
    }

    // --- Requirement 12.4: invalid voucher → EvidenceInvalid, no declaration ---

    #[test]
    fn non_replayable_voucher_is_rejected_with_evidence_invalid_and_creates_nothing() {
        let mut module = RetroactiveReviewModule::new();
        // Flagged not replayable: the reference cannot be independently verified, so it
        // does not correspond to a traceable decarbonization action.
        let voucher = voucher_with(EvidenceRef::new("ipfs://carbon-cid", "0xhash", false));

        let err = voucher
            .import_to_retroactive_flow(
                &mut module,
                FayID::new("eco-bob"),
                ChainId::new("carbon-reduction"),
                "Claimed offsets with an unverifiable voucher.",
                Timestamp::from_secs(1_650_000_000),
            )
            .expect_err("a non-verifiable voucher must be rejected");

        assert_eq!(err, GmcError::EvidenceInvalid);
        // No declaration created → never pushed into voting → nothing minted, no quota
        // consumed (minting/quota are strictly downstream of an approved vote).
        assert!(module.is_empty());
    }

    #[test]
    fn voucher_ref_missing_hash_is_not_verifiable() {
        let mut module = RetroactiveReviewModule::new();
        // Flagged replayable but missing the hash → cannot be verified (R12.4 / 10.8).
        let voucher = voucher_with(EvidenceRef::new("ipfs://carbon-cid", "", true));

        let err = voucher
            .import_to_retroactive_flow(
                &mut module,
                FayID::new("eco-carol"),
                ChainId::new("carbon-reduction"),
                "Voucher reference without a verifiable hash.",
                Timestamp::from_secs(1_650_000_000),
            )
            .expect_err("a voucher reference with no hash must be rejected");

        assert_eq!(err, GmcError::EvidenceInvalid);
        assert!(module.is_empty());
    }

    // --- Requirements 12.5/12.6/12.7: convert() guard + quota accounting ----

    /// A periodic environmental-protection chain quota config with `quota` cap.
    fn env_quota_cfg(quota: &str) -> QuotaConfig {
        QuotaConfig::new(
            Decimal::from_str(quota).unwrap(),
            RefreshPeriod::Periodic {
                unit: TimeUnit::Day,
                value: Decimal::ONE,
            },
        )
        .expect("valid periodic env-chain quota config")
    }

    fn env_ledger() -> QuotaLedger {
        QuotaLedger::new(ChainId::new("carbon-reduction"), Timestamp::from_secs(0))
    }

    fn dec(s: &str) -> Decimal {
        Decimal::from_str(s).unwrap()
    }

    #[test]
    fn first_conversion_succeeds_marks_converted_and_consumes_quota() {
        // Requirements 12.5 + 12.7: first conversion of an unconverted voucher
        // succeeds, marks converted=true with the declaration id, and consumes quota
        // by mint_amount.
        let cfg = env_quota_cfg("100");
        let mut ledger = env_ledger();
        let mut voucher = voucher_with(verifiable_ref());

        voucher
            .convert("decl-001", dec("30"), &cfg, &mut ledger)
            .expect("first conversion of an unconverted voucher must succeed");

        // Voucher is now converted exactly once, recording the declaration.
        assert!(voucher.is_converted());
        assert_eq!(voucher.converted_declaration_id(), Some("decl-001"));
        // The minted amount is counted in the chain's current period quota.
        assert_eq!(ledger.minted_this_period(), dec("30"));
        // And the running total stays within Quota (Requirement 12.5 / Property 28).
        assert!(ledger.minted_this_period() <= cfg.quota());
    }

    #[test]
    fn second_conversion_returns_double_conversion_and_changes_nothing() {
        // Requirement 12.6: a second conversion attempt on the now-converted voucher
        // returns DoubleConversion and does NOT change the voucher or consume more
        // quota.
        let cfg = env_quota_cfg("100");
        let mut ledger = env_ledger();
        let mut voucher = voucher_with(verifiable_ref());

        voucher
            .convert("decl-001", dec("30"), &cfg, &mut ledger)
            .expect("first conversion succeeds");
        assert_eq!(ledger.minted_this_period(), dec("30"));

        // Second attempt is rejected; the guard fires before any side effect.
        let err = voucher
            .convert("decl-002", dec("10"), &cfg, &mut ledger)
            .expect_err("a converted voucher must reject a second conversion");
        assert_eq!(err, GmcError::DoubleConversion);

        // Voucher unchanged (still bound to the first declaration), no extra quota.
        assert!(voucher.is_converted());
        assert_eq!(voucher.converted_declaration_id(), Some("decl-001"));
        assert_eq!(ledger.minted_this_period(), dec("30"));
    }

    #[test]
    fn conversion_exceeding_quota_is_rejected_and_leaves_voucher_unconverted() {
        // Requirement 12.5 (atomicity): a conversion that would exceed quota returns
        // QuotaExceeded and leaves the voucher unconverted with no quota consumed.
        let cfg = env_quota_cfg("100");
        let mut ledger = env_ledger();
        // Pre-consume so only 20 remains in this period.
        ledger.consume_quota(&cfg, dec("80")).expect("80 <= 100");

        let mut voucher = voucher_with(verifiable_ref());
        // 80 + 30 = 110 > 100 -> over quota.
        let err = voucher
            .convert("decl-001", dec("30"), &cfg, &mut ledger)
            .expect_err("a conversion that would exceed quota must be rejected");
        assert_eq!(err, GmcError::QuotaExceeded);

        // No partial state: the voucher stays unconverted and quota is unchanged.
        assert!(!voucher.is_converted());
        assert_eq!(voucher.converted_declaration_id(), None);
        assert_eq!(ledger.minted_this_period(), dec("80"));
    }

    #[test]
    fn voucher_rejected_by_quota_can_still_convert_once_quota_allows() {
        // The QuotaExceeded path leaves no partial state, so the same (still
        // unconverted) voucher converts cleanly once the amount fits.
        let cfg = env_quota_cfg("100");
        let mut ledger = env_ledger();
        ledger.consume_quota(&cfg, dec("80")).expect("80 <= 100");

        let mut voucher = voucher_with(verifiable_ref());
        assert_eq!(
            voucher.convert("decl-001", dec("30"), &cfg, &mut ledger),
            Err(GmcError::QuotaExceeded)
        );
        assert!(!voucher.is_converted());

        // 80 + 20 == 100 fits exactly; conversion now succeeds and is counted.
        voucher
            .convert("decl-001", dec("20"), &cfg, &mut ledger)
            .expect("20 fits within the remaining quota");
        assert!(voucher.is_converted());
        assert_eq!(voucher.converted_declaration_id(), Some("decl-001"));
        assert_eq!(ledger.minted_this_period(), dec("100"));
        assert!(ledger.minted_this_period() <= cfg.quota());
    }
}
