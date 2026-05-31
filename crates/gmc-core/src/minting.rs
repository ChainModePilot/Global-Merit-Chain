//! `Minting_Service` — the minting pipeline, quota metering and `minMerit` updates
//! (design *Minting_Service*; _Requirements 8.6, 8.7_).
//!
//! This module wires together the three stable building blocks implemented by the
//! sibling modules into the single, **atomic** mint pipeline the design mandates:
//!
//! - [`crate::merit`] — [`MeritPocket`] / [`MeritBatch`] (per-batch decay + the
//!   `minMerit` floor-update rule `B' = (x + M)·B / M`),
//! - [`crate::quota`] — [`QuotaConfig`] / [`QuotaLedger`] (per-chain quota check +
//!   consumption), and
//! - [`crate::types`] — the shared fixed-point [`Decimal`] and identity primitives.
//!
//! The mint amount itself (`= V`, the batch's initial value) is produced upstream by
//! `Scoring_Engine::compute_mint_amount` (task 8.3); this service does **not**
//! recompute it. It receives the already-scored amount in a [`MintRequest`] and is
//! responsible only for the *授予 (grant)* mechanics: validate, meter quota, create
//! the decay batch, raise the floor, and account the consumption — in that order,
//! with no partial writes on any failure.
//!
//! # Pipeline order (design: 校验 amount>0 → 检查配额 → 创建批次 → 更新 minMerit → 累计配额消耗)
//!
//! [`MintingService::mint`] executes the steps below. Every step that can fail is a
//! pure check performed **before** any state is mutated, so a rejected request leaves
//! both the [`MeritPocket`] and the [`QuotaLedger`] byte-for-byte unchanged
//! (design *Error Handling*: "validate up front, fail atomically, leave state
//! unchanged").
//!
//! 1. **Validate `amount > 0`** (_Requirement 8.7_). A non-positive amount returns
//!    [`GmcError::InvalidMintAmount`] and touches nothing — `curMerit` / `minMerit`
//!    and the ledger are all left as-is.
//! 2. **Validate the influence duration & derive `λ`.** A batch needs a strictly
//!    positive `influenceDuration` to define `λ = k / influenceDuration`
//!    (_Requirement 8.1_). This is derived up front via
//!    [`MeritBatch::lambda_from_duration`] so that, once we begin mutating, batch
//!    creation can no longer fail. A non-positive duration is reported as
//!    [`GmcError::InvalidMintAmount`] (it makes the requested mint un-mintable).
//! 3. **Check quota** via [`QuotaLedger::check_quota`] (_Requirements 4.2, 4.3_).
//!    This is read-only: an over-quota / exhausted request returns
//!    [`GmcError::QuotaExceeded`] without mutating the pocket or the ledger.
//!
//!    *— all fallible validation now done; the steps below form the mutation phase —*
//!
//! 4. **Snapshot `M = pocket.cur_merit(acquired_at)`** *before* adding the new batch.
//!    The floor-update rule requires the **pre-mint** `curMerit` (see
//!    [`MeritPocket::update_min_merit`]); its monotonicity proof relies on `M ≥ B`,
//!    which holds because the `curMerit ≥ minMerit` invariant held before this mint.
//! 5. **Update the floor & derive `B`.** We call
//!    [`MeritPocket::update_min_merit(amount, M)`](MeritPocket::update_min_merit) —
//!    reusing the canonical floor math rather than recomputing it by hand — and take
//!    `B = newFloor − oldFloor`, this batch's contribution to the floor. On the
//!    (practically unreachable) fixed-point overflow it returns
//!    [`GmcError::InvalidMintAmount`] and leaves `min_merit` untouched, so the mint
//!    aborts before any batch is added.
//! 6. **Create the batch** with `V = amount`, `B = floor increment`, the `λ` derived
//!    in step 2, and the request's duration / acquisition time / source chain, then
//!    append it with [`MeritPocket::add_batch`].
//! 7. **Accumulate quota** via [`QuotaLedger::consume_quota`] (_Requirement 4.4_).
//!
//! ## Why steps 5 & 6 compute the floor *before* building the batch
//!
//! The design lists "创建批次 → 更新 minMerit" (create batch → update floor), but the
//! batch's floor contribution `B` *is* the floor increment, so the increment has to
//! be known to build a batch that keeps the `curMerit ≥ minMerit` invariant. We
//! therefore compute the floor first (which yields the increment), then build the
//! batch with `B = increment`. This is exactly the ordering proven correct by the
//! `merit.rs` test `invariant_holds_through_a_simulated_mint_pipeline`: with each
//! batch's floor `B_i` set to its mint's floor increment, the batch floors telescope
//! so that `Σ_i B_i == minMerit`, and since every batch value is `≥ B_i` at all times,
//! `curMerit(t) ≥ Σ_i B_i ≥ minMerit` (_Requirement 8.5_). Both sub-steps are part of
//! the same logical "grant" and neither is observable until the whole mint succeeds.
//!
//! ## Atomicity of the mutation phase (no half-updates)
//!
//! Steps 4–7 mutate state, but the only fallible call among them,
//! [`MeritPocket::update_min_merit`] in step 5, is atomic on its own (it leaves
//! `min_merit` unchanged on error and no batch has been added yet). The final
//! [`QuotaLedger::consume_quota`] in step 7 is **guaranteed to succeed**: step 3's
//! [`check_quota`](QuotaLedger::check_quota) proved the ledger is not exhausted and
//! `minted_this_period + amount ≤ quota`, and nothing between step 3 and step 7
//! touches the ledger (we hold it by `&mut`, so no other code can mutate it, and the
//! pocket mutations are independent of the ledger). `consume_quota` re-runs the same
//! deterministic check on the same inputs and therefore cannot fail here. The `?` on
//! that call is thus defensive only; its error branch is unreachable in practice, so
//! the pipeline never leaves a half-updated pocket-with-unaccounted-quota state.
//!
//! # L2 / L1 seams (_Requirement 8.6_)
//!
//! Per the design, MeriToken is computed in real time on **L2_Rollup** and the
//! resulting state root is anchored to **L1_Settlement**. Those integrations are
//! owned by later tasks (L2: task 19.1; L1: task 18.1). This module exposes them as
//! explicit, documented no-op seams — [`MintingService::compute_meritoken_l2`] and
//! [`MintingService::anchor_state_root_to_l1`] — invoked at the end of a successful
//! mint so the end-to-end flow reads completely while keeping this crate free of any
//! chain-runtime dependency.

use crate::error::{GmcError, GmcResult};
use crate::merit::{MeritBatch, MeritPocket};
use crate::quota::{QuotaConfig, QuotaLedger};
use crate::types::{ChainId, Decimal, Timestamp};

/// The inputs of a single mint that are specific to *this* contribution.
///
/// The stateful collaborators a mint operates on — the target [`MeritPocket`], the
/// chain's [`QuotaConfig`] and its [`QuotaLedger`] — are passed to
/// [`MintingService::mint`] by reference rather than embedded here, so this struct
/// only carries the per-mint, by-value parameters.
///
/// `amount` is the value produced by `Scoring_Engine::compute_mint_amount` (task 8.3)
/// and equals the new batch's initial Merit value `V`. It is expected to be strictly
/// positive; the pipeline re-validates that (_Requirement 8.7_) so this entry point is
/// safe to call with an unvalidated amount.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MintRequest {
    /// Stable identifier to assign to the created [`MeritBatch`].
    pub batch_id: String,
    /// The amount to mint (`= V`, the batch's initial Merit value). Must be `> 0`.
    pub amount: Decimal,
    /// The contribution's influence duration (`> 0`); drives `λ = k / duration`.
    pub influence_duration: Decimal,
    /// On-chain time of acquisition; used both as the batch's `acquiredAt` and as the
    /// time at which the pre-mint `curMerit` snapshot `M` is taken.
    pub acquired_at: Timestamp,
    /// The chain that is minting this batch.
    pub source_chain_id: ChainId,
}

impl MintRequest {
    /// Convenience constructor for a [`MintRequest`].
    pub fn new(
        batch_id: impl Into<String>,
        amount: Decimal,
        influence_duration: Decimal,
        acquired_at: Timestamp,
        source_chain_id: ChainId,
    ) -> Self {
        MintRequest {
            batch_id: batch_id.into(),
            amount,
            influence_duration,
            acquired_at,
            source_chain_id,
        }
    }
}

/// The outcome of a successful mint.
///
/// Returned by [`MintingService::mint`] on success. It reports the created batch's id
/// alongside the floor transition so callers (and tests) can assert the pipeline's
/// effects without re-reading the pocket.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MintReceipt {
    /// Id of the [`MeritBatch`] that was created and appended to the pocket.
    pub batch_id: String,
    /// The amount minted (`= V`, the batch's initial value).
    pub minted_amount: Decimal,
    /// This batch's contribution `B` to the floor (the floor increment).
    pub floor_increment: Decimal,
    /// The pocket's `minMerit` after the mint.
    pub new_min_merit: Decimal,
}

/// The minting service: drives the atomic *授予 (grant)* pipeline.
///
/// Stateless — it owns no mutable state of its own and instead operates on the
/// [`MeritPocket`] and [`QuotaLedger`] handed to [`mint`](MintingService::mint). This
/// mirrors `Scoring_Engine` (also stateless) and keeps the per-chain economic state
/// where it belongs (the pocket and the ledger), so the service can be freely shared
/// across chains and layers.
#[derive(Debug, Clone, Copy, Default)]
pub struct MintingService;

impl MintingService {
    /// Creates a new minting service.
    pub fn new() -> Self {
        MintingService
    }

    /// Runs the atomic mint pipeline for one contribution.
    ///
    /// See the module-level docs for the full step-by-step ordering and the
    /// atomicity argument. In short, the pipeline is:
    ///
    /// 1. validate `request.amount > 0` (else [`GmcError::InvalidMintAmount`]);
    /// 2. validate `request.influence_duration > 0` and derive `λ` (else
    ///    [`GmcError::InvalidMintAmount`]);
    /// 3. [`check_quota`](QuotaLedger::check_quota) (else [`GmcError::QuotaExceeded`]);
    /// 4. snapshot `M = pocket.cur_merit(acquired_at)` *before* mutating;
    /// 5. [`update_min_merit`](MeritPocket::update_min_merit) and derive `B = ΔminMerit`;
    /// 6. create the batch (`V = amount`, `B`, `λ`) and
    ///    [`add_batch`](MeritPocket::add_batch);
    /// 7. [`consume_quota`](QuotaLedger::consume_quota).
    ///
    /// On **any** failure in steps 1–3 neither the pocket nor the ledger is mutated
    /// (_Requirements 8.7, 4.3_). On success the pocket has one new batch, a
    /// non-decreasing `minMerit`, the ledger's `minted_this_period` has grown by
    /// `amount`, and a [`MintReceipt`] describing the transition is returned.
    ///
    /// # Errors
    ///
    /// - [`GmcError::InvalidMintAmount`] — `amount ≤ 0`, or `influence_duration ≤ 0`,
    ///   or fixed-point overflow while updating the floor. No state is changed (the
    ///   floor update is itself atomic, so an overflow there aborts before any batch
    ///   is added).
    /// - [`GmcError::QuotaExceeded`] — minting `amount` would exceed the chain's
    ///   period quota, or the (one-time) chain is exhausted. No state is changed.
    pub fn mint(
        &self,
        pocket: &mut MeritPocket,
        config: &QuotaConfig,
        ledger: &mut QuotaLedger,
        request: MintRequest,
    ) -> GmcResult<MintReceipt> {
        // --- Validation phase: pure checks, no mutation (atomic abort on failure) ---

        // (1) amount must be strictly positive (Requirement 8.7). A non-positive mint
        // never creates a batch and never moves curMerit/minMerit or the ledger.
        if !request.amount.is_positive() {
            return Err(GmcError::InvalidMintAmount);
        }

        // (2) influence_duration must be > 0 so λ = k / duration is defined
        // (Requirement 8.1). Deriving λ here (before any mutation) means batch
        // creation in the mutation phase below can no longer fail.
        let lambda = MeritBatch::lambda_from_duration(request.influence_duration)
            .ok_or(GmcError::InvalidMintAmount)?;

        // (3) Quota check — read-only. An over-quota / exhausted request is rejected
        // with QuotaExceeded and leaves the ledger's counter unchanged (Req 4.2/4.3).
        ledger.check_quota(config, request.amount)?;

        // --- Mutation phase: all fallible validation passed (see module docs) ---

        // (4) Snapshot the PRE-mint curMerit M before adding the new batch. The
        // floor-update rule needs the pre-mint curMerit (its M ≥ B precondition holds
        // because the curMerit ≥ minMerit invariant held before this mint).
        let m = pocket.cur_merit(request.acquired_at);

        // (5) Raise the floor via the canonical rule B' = (x + M)·B / M and take this
        // batch's floor contribution B = newFloor − oldFloor. update_min_merit leaves
        // min_merit untouched on the (unreachable) overflow path, so a failure here
        // aborts the mint before any batch is added — still atomic.
        let old_floor = pocket.min_merit();
        let new_floor = pocket.update_min_merit(request.amount, m)?;
        let floor_increment = new_floor.checked_sub(old_floor).unwrap_or(Decimal::ZERO);

        // (6) Create the decay batch with V = amount and B = floor increment so the
        // batch floors telescope to exactly minMerit (keeps curMerit ≥ minMerit at all
        // t; see module docs / merit.rs invariant_holds_through_a_simulated_mint_pipeline).
        let batch = MeritBatch::new(
            request.batch_id.clone(),
            request.amount,  // V
            floor_increment, // B
            lambda,
            request.influence_duration,
            request.acquired_at,
            request.source_chain_id.clone(),
        );
        pocket.add_batch(batch);

        // (7) Accumulate quota consumption. Guaranteed to succeed: check_quota in
        // step 3 already proved it, and nothing has touched the ledger since (we hold
        // it by &mut). The `?` is defensive; its error branch is unreachable here, so
        // the pocket is never left with unaccounted quota.
        ledger.consume_quota(config, request.amount)?;

        // L2 real-time MeriToken computation + L1 state-root anchoring seams (Req 8.6).
        self.compute_meritoken_l2(pocket, request.acquired_at);
        self.anchor_state_root_to_l1();

        Ok(MintReceipt {
            batch_id: request.batch_id,
            minted_amount: request.amount,
            floor_increment,
            new_min_merit: new_floor,
        })
    }

    /// Seam for "L2 computes MeriToken in real time" (_Requirement 8.6_).
    ///
    /// In the integrated system the L2_Rollup derives each pocket's live `curMerit`
    /// from its decay batches and serves it to clients within the design's latency
    /// budget. The concrete L2 wiring is task 19.1; this no-op keeps the call site
    /// explicit so the mint flow reads end-to-end here.
    fn compute_meritoken_l2(&self, _pocket: &MeritPocket, _now: Timestamp) {
        // Intentionally empty: see task 19.1 for the real L2 computation.
    }

    /// Seam for "anchor the post-mint state root to L1_Settlement" (_Requirement 8.6_).
    ///
    /// In the integrated system the L2 batches mint results into a ZK proof and
    /// anchors the resulting state root to L1. The concrete L1 anchoring is task 18.1;
    /// this no-op marks where it plugs in.
    fn anchor_state_root_to_l1(&self) {
        // Intentionally empty: see task 18.1 for the real L1 anchoring.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::merit::E;
    use crate::quota::{QuotaConfig, QuotaLedger, RefreshPeriod};
    use crate::types::{ChainId, FayID, Timestamp};

    fn dec(s: &str) -> Decimal {
        Decimal::from_str(s).expect("valid decimal literal")
    }

    fn ts(secs: u64) -> Timestamp {
        Timestamp::from_secs(secs)
    }

    fn periodic_cfg(quota: &str) -> QuotaConfig {
        QuotaConfig::new(
            dec(quota),
            RefreshPeriod::Periodic {
                unit: crate::quota::TimeUnit::Day,
                value: Decimal::ONE,
            },
        )
        .expect("valid periodic config")
    }

    fn fresh_ledger() -> QuotaLedger {
        QuotaLedger::new(ChainId::from("chain-1"), ts(0))
    }

    /// A pocket whose initial floor `E` is *backed* by a slowly-decaying batch with
    /// `B = E`, mirroring the registration grant in
    /// `merit.rs::invariant_holds_through_a_simulated_mint_pipeline`. This makes
    /// `Σ B_i` start equal to `minMerit`, so the `curMerit ≥ minMerit` invariant is
    /// well-defined and holds through subsequent mints.
    fn backed_pocket() -> MeritPocket {
        let mut pocket = MeritPocket::new(FayID::from("fay-1"));
        pocket.add_batch(MeritBatch::new(
            "reg-grant",
            Decimal::from_int(100), // V
            E,                      // B = initial floor, so Σ B_i starts == minMerit
            dec("0.001"),
            Decimal::from_int(1_000),
            ts(0),
            ChainId::from("chain-1"),
        ));
        pocket
    }

    #[test]
    fn successful_mint_creates_batch_raises_floor_and_consumes_quota() {
        let service = MintingService::new();
        let cfg = periodic_cfg("1000");
        let mut ledger = fresh_ledger();
        let mut pocket = backed_pocket();

        let now = ts(100);
        let batches_before = pocket.batches.len();
        let floor_before = pocket.min_merit();
        let cur_before = pocket.cur_merit(now);

        let amount = Decimal::from_int(50);
        let receipt = service
            .mint(
                &mut pocket,
                &cfg,
                &mut ledger,
                MintRequest::new(
                    "m1",
                    amount,
                    Decimal::from_int(1_000),
                    now,
                    ChainId::from("chain-1"),
                ),
            )
            .expect("mint within quota with valid inputs must succeed");

        // A new batch was created and appended.
        assert_eq!(pocket.batches.len(), batches_before + 1);
        assert_eq!(receipt.batch_id, "m1");
        assert_eq!(receipt.minted_amount, amount);

        // curMerit increased: at the acquisition time the new batch contributes its
        // full V (decay factor 1), so curMerit grows by exactly `amount`.
        let cur_after = pocket.cur_merit(now);
        assert_eq!(cur_after, cur_before.checked_add(amount).unwrap());
        assert!(cur_after > cur_before);

        // minMerit is non-decreasing and matches the receipt.
        assert!(pocket.min_merit() >= floor_before);
        assert_eq!(pocket.min_merit(), receipt.new_min_merit);
        assert_eq!(
            receipt.floor_increment,
            pocket.min_merit().checked_sub(floor_before).unwrap()
        );

        // Quota consumed by exactly `amount`.
        assert_eq!(ledger.minted_this_period(), amount);

        // Invariant curMerit >= minMerit holds at a spread of time points.
        for t in [0u64, 100, 1_000, 10_000, 1_000_000, 100_000_000] {
            assert!(
                pocket.invariant_holds(ts(t)),
                "invariant violated at t={t}: cur={}, min={}",
                pocket.cur_merit(ts(t)),
                pocket.min_merit()
            );
        }
    }

    #[test]
    fn sequence_of_mints_keeps_invariant_and_accumulates_quota() {
        let service = MintingService::new();
        let cfg = periodic_cfg("1000");
        let mut ledger = fresh_ledger();
        let mut pocket = backed_pocket();

        let mints = [("m1", 50i64, 100u64), ("m2", 30, 500), ("m3", 80, 1_500)];
        let mut prev_floor = pocket.min_merit();
        let mut expected_minted = Decimal::ZERO;

        for (id, x, now) in mints {
            let amount = Decimal::from_int(x);
            service
                .mint(
                    &mut pocket,
                    &cfg,
                    &mut ledger,
                    MintRequest::new(
                        id,
                        amount,
                        Decimal::from_int(1_000),
                        ts(now),
                        ChainId::from("chain-1"),
                    ),
                )
                .expect("mint must succeed");
            // Floor is non-decreasing across the sequence.
            assert!(pocket.min_merit() >= prev_floor);
            prev_floor = pocket.min_merit();
            expected_minted = expected_minted.checked_add(amount).unwrap();
        }

        // Quota accumulated across all mints.
        assert_eq!(ledger.minted_this_period(), expected_minted);

        // Invariant holds across time, including the far future.
        for t in [0u64, 100, 500, 1_500, 5_000, 50_000, 1_000_000, 100_000_000] {
            assert!(
                pocket.invariant_holds(ts(t)),
                "invariant violated at t={t}: cur={}, min={}",
                pocket.cur_merit(ts(t)),
                pocket.min_merit()
            );
        }
    }

    #[test]
    fn non_positive_amount_is_rejected_and_leaves_pocket_and_ledger_unchanged() {
        // Requirement 8.7: amount <= 0 returns InvalidMintAmount and changes nothing.
        let service = MintingService::new();
        let cfg = periodic_cfg("1000");
        let mut ledger = fresh_ledger();
        let mut pocket = backed_pocket();

        let batches_before = pocket.batches.len();
        let floor_before = pocket.min_merit();
        let minted_before = ledger.minted_this_period();
        let pocket_snapshot = pocket.clone();
        let ledger_snapshot = ledger.clone();

        for bad in [Decimal::ZERO, Decimal::from_int(-5)] {
            let err = service
                .mint(
                    &mut pocket,
                    &cfg,
                    &mut ledger,
                    MintRequest::new(
                        "bad",
                        bad,
                        Decimal::from_int(1_000),
                        ts(100),
                        ChainId::from("chain-1"),
                    ),
                )
                .unwrap_err();
            assert_eq!(err, GmcError::InvalidMintAmount);
            // No batch added, floor unchanged, quota counter unchanged.
            assert_eq!(pocket.batches.len(), batches_before);
            assert_eq!(pocket.min_merit(), floor_before);
            assert_eq!(ledger.minted_this_period(), minted_before);
            // Whole-state equality: provably no partial write.
            assert_eq!(pocket, pocket_snapshot);
            assert_eq!(ledger, ledger_snapshot);
        }
    }

    #[test]
    fn non_positive_influence_duration_is_rejected_and_leaves_state_unchanged() {
        // A non-positive influence duration cannot define λ, so the mint is rejected
        // up front (before any mutation), leaving pocket and ledger unchanged.
        let service = MintingService::new();
        let cfg = periodic_cfg("1000");
        let mut ledger = fresh_ledger();
        let mut pocket = backed_pocket();

        let pocket_snapshot = pocket.clone();
        let ledger_snapshot = ledger.clone();

        let err = service
            .mint(
                &mut pocket,
                &cfg,
                &mut ledger,
                MintRequest::new(
                    "bad-dur",
                    Decimal::from_int(50),
                    Decimal::ZERO, // invalid duration
                    ts(100),
                    ChainId::from("chain-1"),
                ),
            )
            .unwrap_err();
        assert_eq!(err, GmcError::InvalidMintAmount);
        assert_eq!(pocket, pocket_snapshot);
        assert_eq!(ledger, ledger_snapshot);
    }

    #[test]
    fn over_quota_mint_is_rejected_and_leaves_pocket_and_ledger_unchanged() {
        // Requirement 4.3 / 8.x: an over-quota mint returns QuotaExceeded with no
        // batch added, no minMerit change and the quota counter unchanged.
        let service = MintingService::new();
        let cfg = periodic_cfg("100");
        let mut ledger = fresh_ledger();
        let mut pocket = backed_pocket();

        // Pre-consume 80 of the 100 quota directly on the ledger.
        ledger
            .consume_quota(&cfg, Decimal::from_int(80))
            .expect("80 <= 100");

        let batches_before = pocket.batches.len();
        let floor_before = pocket.min_merit();
        let pocket_snapshot = pocket.clone();
        let ledger_snapshot = ledger.clone();

        // 80 + 30 = 110 > 100 -> rejected.
        let err = service
            .mint(
                &mut pocket,
                &cfg,
                &mut ledger,
                MintRequest::new(
                    "over",
                    Decimal::from_int(30),
                    Decimal::from_int(1_000),
                    ts(100),
                    ChainId::from("chain-1"),
                ),
            )
            .unwrap_err();
        assert_eq!(err, GmcError::QuotaExceeded);

        // No batch added; minMerit unchanged; quota counter still 80.
        assert_eq!(pocket.batches.len(), batches_before);
        assert_eq!(pocket.min_merit(), floor_before);
        assert_eq!(ledger.minted_this_period(), Decimal::from_int(80));
        // Whole-state equality: provably no partial write to either collaborator.
        assert_eq!(pocket, pocket_snapshot);
        assert_eq!(ledger, ledger_snapshot);
    }

    #[test]
    fn exhausted_one_time_chain_rejects_mint_without_mutation() {
        // A one-time chain that has consumed its full quota is exhausted and rejects
        // further mints, leaving state unchanged (Requirements 4.2, 4.7).
        let service = MintingService::new();
        let cfg = QuotaConfig::new(Decimal::from_int(100), RefreshPeriod::OneTime)
            .expect("valid one-time config");
        let mut ledger = fresh_ledger();
        ledger
            .consume_quota(&cfg, Decimal::from_int(100))
            .expect("fully consume the one-time quota");
        assert!(ledger.is_exhausted());

        let mut pocket = backed_pocket();
        let pocket_snapshot = pocket.clone();
        let ledger_snapshot = ledger.clone();

        let err = service
            .mint(
                &mut pocket,
                &cfg,
                &mut ledger,
                MintRequest::new(
                    "after-exhaust",
                    Decimal::from_int(1),
                    Decimal::from_int(1_000),
                    ts(100),
                    ChainId::from("chain-1"),
                ),
            )
            .unwrap_err();
        assert_eq!(err, GmcError::QuotaExceeded);
        assert_eq!(pocket, pocket_snapshot);
        assert_eq!(ledger, ledger_snapshot);
    }

    #[test]
    fn mint_up_to_exactly_quota_succeeds_then_next_is_rejected() {
        // Boundary: consuming up to exactly the cap is allowed; the next positive
        // mint is rejected and leaves state unchanged.
        let service = MintingService::new();
        let cfg = periodic_cfg("100");
        let mut ledger = fresh_ledger();
        let mut pocket = backed_pocket();

        service
            .mint(
                &mut pocket,
                &cfg,
                &mut ledger,
                MintRequest::new(
                    "fill",
                    Decimal::from_int(100),
                    Decimal::from_int(1_000),
                    ts(100),
                    ChainId::from("chain-1"),
                ),
            )
            .expect("100 == quota is allowed");
        assert_eq!(ledger.minted_this_period(), Decimal::from_int(100));

        let pocket_snapshot = pocket.clone();
        let ledger_snapshot = ledger.clone();

        let err = service
            .mint(
                &mut pocket,
                &cfg,
                &mut ledger,
                MintRequest::new(
                    "overflow",
                    dec("0.000001"),
                    Decimal::from_int(1_000),
                    ts(200),
                    ChainId::from("chain-1"),
                ),
            )
            .unwrap_err();
        assert_eq!(err, GmcError::QuotaExceeded);
        assert_eq!(pocket, pocket_snapshot);
        assert_eq!(ledger, ledger_snapshot);
    }
}
