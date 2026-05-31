//! Property 28 — 碳积分铸造计入当前周期配额
//! (carbon-credit minting is charged to the current refresh period's quota).
//!
//! Dedicated property-based test for **Property 28** of the `gmc-core-protocol`
//! design's *Correctness Properties* section (task 16.4).
//!
//! > **Property 28: 碳积分铸造计入当前周期配额** — 对任意 环境保护链上经审核通过的
//! > 碳积分申报铸造，铸造数量被计入该链当前 `Refresh_Period` 的配额消耗，且计入后本
//! > 周期累计量仍不超过 `Quota`。
//! >
//! > For any approved carbon-credit declaration mint on the environmental-protection
//! > chain, the minted amount is charged to the chain's current `Refresh_Period`
//! > quota consumption (`mintedThisPeriod` increases by exactly the mint amount), and
//! > the resulting period total still does not exceed `Quota`.
//!
//! **Validates: Requirements 12.5**
//!
//! The approved mint is modeled by [`CarbonCreditVoucher::convert`], which — after a
//! retroactive vote approves the carbon declaration and the three-dimension
//! `Scoring_Engine` has computed `mint_amount` — charges that amount to the chain's
//! current period via [`QuotaLedger::consume_quota`] and marks the voucher converted.
//! Inputs are constrained so the mint fits within the remaining allowance (an
//! *approved* mint that the quota admits), then the period-accounting invariants are
//! asserted: the running total rises by exactly the minted amount and never exceeds
//! `Quota`.

use gmc_core::carbon::CarbonCreditVoucher;
use gmc_core::quota::{QuotaConfig, QuotaLedger, RefreshPeriod, TimeUnit};
use gmc_core::retroactive::EvidenceRef;
use gmc_core::types::{ChainId, Decimal, Timestamp};
use proptest::prelude::*;

/// A verifiable (replayable) carbon-credit voucher reference. `convert` does not
/// re-validate evidence, but we build a realistic, verifiable reference anyway.
fn verifiable_voucher() -> CarbonCreditVoucher {
    CarbonCreditVoucher::new(
        "voucher-prop-28",
        EvidenceRef::new("ipfs://carbon-cid", "0xcarbonhash", true),
    )
}

/// A periodic environmental-protection chain quota config with the given cap.
fn env_quota_cfg(quota: Decimal) -> QuotaConfig {
    QuotaConfig::new(
        quota,
        RefreshPeriod::Periodic {
            unit: TimeUnit::Day,
            value: Decimal::ONE,
        },
    )
    .expect("a strictly-positive quota with a valid periodic period is a valid config")
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 28: 碳积分铸造计入当前周期配额
    #[test]
    fn property_28_carbon_mint_charged_to_current_period_quota(
        // Pick a quota cap (raw micro-units), an amount already minted this period
        // (`pre_raw`, strictly below the cap), and an approved mint amount that fits in
        // the remaining allowance (`mint_raw` in 1..=remaining). This models an
        // approved carbon declaration whose mint the quota admits.
        (quota_raw, pre_raw, mint_raw) in (1i128..=1_000_000_000i128)
            .prop_flat_map(|quota_raw| {
                (0i128..quota_raw).prop_flat_map(move |pre_raw| {
                    let remaining = quota_raw - pre_raw;
                    (Just(quota_raw), Just(pre_raw), 1i128..=remaining)
                })
            }),
    ) {
        let quota = Decimal::from_raw(quota_raw);
        let pre = Decimal::from_raw(pre_raw);
        let mint = Decimal::from_raw(mint_raw);

        let cfg = env_quota_cfg(quota);
        let mut ledger = QuotaLedger::new(
            ChainId::new("carbon-reduction"),
            Timestamp::from_secs(0),
        );

        // Establish the period's pre-existing consumption (a prior approved mint).
        ledger
            .consume_quota(&cfg, pre)
            .expect("pre-consumption is within the quota by construction");
        let before = ledger.minted_this_period();
        prop_assert_eq!(before, pre);

        // A fresh (unconverted) voucher representing an approved carbon declaration.
        let mut voucher = verifiable_voucher();
        prop_assert!(!voucher.is_converted());

        // The approved mint: convert charges `mint` to the current Refresh_Period.
        voucher
            .convert("decl-prop-28", mint, &cfg, &mut ledger)
            .expect("an approved mint that fits the remaining quota must succeed");

        let after = ledger.minted_this_period();

        // (Req 12.5) The minted amount is counted in the current period: the running
        // total increases by exactly the mint amount.
        prop_assert_eq!(after, before.checked_add(mint).expect("no overflow by bound"));
        prop_assert_eq!(after.checked_sub(before).expect("monotonic"), mint);

        // (Req 12.5 / Property 28) The resulting period total never exceeds Quota.
        prop_assert!(after <= cfg.quota());

        // The conversion was recorded exactly once against its declaration.
        prop_assert!(voucher.is_converted());
        prop_assert_eq!(voucher.converted_declaration_id(), Some("decl-prop-28"));
    }
}
