//! Property 9 — 配额永不超限（含一次性耗尽不恢复）
//! (quota never exceeds its cap, incl. one-time exhaustion never recovers).
//!
//! This is the dedicated property-based test for **Property 9** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 6.4).
//!
//! > **Property 9: 配额永不超限（含一次性耗尽不恢复）** — For any sequence of mint
//! > requests against a single chain, the amount minted this period
//! > (`mintedThisPeriod`) never exceeds that chain's `Quota` at any point; any
//! > request that would push the running total over `Quota` is rejected and is *not*
//! > counted (the counter is unchanged); and a chain configured as one-time, once its
//! > quota is exhausted, rejects every subsequent request forever and never restores
//! > any available quota.
//!
//! **Validates: Requirements 4.2, 4.3, 4.4, 4.7**
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 9: ...` and runs with `>= 100`
//! random iterations. Because the property is about a *single* chain, the inputs are
//! built directly here: a strictly-positive `Quota`, a one-time vs. periodic refresh
//! configuration, and a sequence of strictly-positive mint amounts (the quota module
//! deliberately leaves `amount > 0` validation to the minting service, so a quota
//! property drives only positive requests). The ledger is then driven request by
//! request and the Property 9 invariants are asserted after every step.

use gmc_core::error::GmcError;
use gmc_core::quota::{QuotaConfig, QuotaLedger, RefreshPeriod, TimeUnit};
use gmc_core::types::{ChainId, Decimal, Timestamp};
use proptest::prelude::*;

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 9: 配额永不超限（含一次性耗尽不恢复）
    #[test]
    fn property_9_quota_never_exceeds(
        // A strictly-positive per-period cap (raw fixed-point units). Chosen to sit in
        // the same magnitude band as the amounts below so a sequence naturally mixes
        // accepted mints and over-quota rejections.
        quota_raw in 1i128..=5_000_000_000i128,
        // Whether the chain uses a one-time (non-renewing) allowance or a periodic one.
        one_time in any::<bool>(),
        // The mint-request sequence: each amount is strictly positive (the minting
        // pipeline guards `amount > 0` separately, Req 8.7).
        amount_raws in proptest::collection::vec(1i128..=1_000_000_000i128, 0..40usize),
    ) {
        let quota = Decimal::from_raw(quota_raw);
        let refresh_period = if one_time {
            RefreshPeriod::OneTime
        } else {
            RefreshPeriod::Periodic {
                unit: TimeUnit::Day,
                value: Decimal::ONE,
            }
        };
        let cfg = QuotaConfig::new(quota, refresh_period)
            .expect("a strictly-positive quota + valid refresh period is a valid config");

        let mut ledger =
            QuotaLedger::new(ChainId::new("chain-under-test"), Timestamp::from_secs(0));

        // The invariant holds for the clean initial state too: nothing minted yet.
        prop_assert_eq!(ledger.minted_this_period(), Decimal::ZERO);
        prop_assert!(ledger.minted_this_period() <= quota);
        prop_assert!(!ledger.is_exhausted());

        // Tracks whether the (one-time) chain has ever become exhausted, so we can
        // assert exhaustion is permanent (Req 4.7).
        let mut exhausted_seen = false;

        for raw in amount_raws {
            let amount = Decimal::from_raw(raw);

            let before = ledger.minted_this_period();
            let was_exhausted = ledger.is_exhausted();

            // (Req 4.3) `check_quota` answers "would this be allowed?" WITHOUT mutating
            // anything — confirm it leaves the counter untouched and predicts consume.
            let check = ledger.check_quota(&cfg, amount);
            prop_assert_eq!(
                ledger.minted_this_period(),
                before,
                "check_quota must not mutate the ledger"
            );

            let result = ledger.consume_quota(&cfg, amount);
            let after = ledger.minted_this_period();

            prop_assert_eq!(
                result.is_ok(),
                check.is_ok(),
                "check_quota must predict the outcome of consume_quota"
            );

            // (Req 4.2 & 4.7) Once a one-time chain is exhausted, every subsequent
            // request is rejected and no quota is ever restored.
            if was_exhausted {
                prop_assert!(
                    result.is_err(),
                    "an exhausted one-time chain rejects all further requests"
                );
                prop_assert_eq!(after, before, "an exhausted chain never restores quota");
            }

            match &result {
                Ok(()) => {
                    // (Req 4.4) a successful mint accumulates exactly `amount`.
                    let expected = before
                        .checked_add(amount)
                        .expect("running total stays within i128 over the tested range");
                    prop_assert_eq!(after, expected, "a successful mint accumulates `amount`");
                    // (Req 4.3) and the running total still respects the cap.
                    prop_assert!(after <= quota, "minted_this_period must never exceed Quota");
                }
                Err(err) => {
                    // The only rejection reason for quota accounting is QuotaExceeded.
                    prop_assert_eq!(err, &GmcError::QuotaExceeded);
                    // (Req 4.3) a rejected (over-quota) request is NOT counted.
                    prop_assert_eq!(after, before, "a rejected request leaves the counter unchanged");
                }
            }

            // (Req 4.2 / 4.3) the cap is never exceeded at ANY point in the sequence.
            prop_assert!(after <= quota, "minted_this_period must never exceed Quota");

            // (Req 4.4 / 4.7) within a single period (no reset is called here) the
            // counter is monotonically non-decreasing — quota is never restored.
            prop_assert!(after >= before, "quota is never restored within a period");

            // (Req 4.7) only one-time chains ever become exhausted, and once exhausted
            // they stay exhausted for the remainder of the sequence.
            if ledger.is_exhausted() {
                prop_assert!(
                    cfg.is_one_time(),
                    "only one-time chains can become exhausted"
                );
                exhausted_seen = true;
            }
            if exhausted_seen {
                prop_assert!(ledger.is_exhausted(), "exhaustion is permanent (never reverses)");
            }
        }
    }
}
