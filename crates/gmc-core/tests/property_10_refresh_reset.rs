//! Property 10 — 刷新周期到期重置 (refresh-period expiry reset).
//!
//! This is the dedicated property-based test for **Property 10** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 6.5).
//!
//! > **Property 10: 刷新周期到期重置** — 对任意 非一次性链及任意随时间推进的铸造
//! > 序列，每当跨越一个 `Refresh_Period` 边界，`mintedThisPeriod` 在新周期起点被
//! > 重置为 0。
//!
//! **Validates: Requirements 4.5**
//!
//! The test drives a single **non-one-time** (`Periodic`) chain's [`QuotaLedger`] with
//! a time-advancing mint sequence: at each advancing timestamp it rolls the ledger
//! over via [`QuotaLedger::reset_if_elapsed`] and then attempts a mint. The invariant
//! checked at every step is that crossing a `Refresh_Period` boundary (a full period
//! having elapsed since the period start) resets `mintedThisPeriod` to exactly `0` at
//! the new period start — and, conversely, that no reset happens while still inside a
//! period (accumulation is preserved). Per the harness convention
//! (`tests/common/mod.rs`), the single proptest below is labelled
//! `Feature: gmc-core-protocol, Property 10: ...` and runs with `>= 100` iterations.

mod common;

use common::generators;
use gmc_core::quota::{QuotaConfig, QuotaLedger, RefreshPeriod, TimeUnit};
use gmc_core::types::{ChainId, Decimal, Timestamp};
use proptest::prelude::*;

/// A **non-one-time** (`Periodic`) quota config with a usable, strictly-positive
/// refresh-period length. The interval `value` is a whole integer count of units, so
/// `period_length_secs()` is always `Some(len)` with `len > 0`, giving a real boundary
/// to cross. The quota is large relative to the per-step mint amounts below, so mints
/// generally accumulate (making a reset observable as `minted -> 0`).
fn periodic_config() -> impl Strategy<Value = QuotaConfig> {
    let unit = prop_oneof![
        Just(TimeUnit::Second),
        Just(TimeUnit::Hour),
        Just(TimeUnit::Day),
    ];
    (unit, 1i64..=30i64, 1i64..=1_000_000i64).prop_map(|(unit, value, quota)| {
        QuotaConfig::new(
            Decimal::from_int(quota),
            RefreshPeriod::Periodic {
                unit,
                value: Decimal::from_int(value),
            },
        )
        .expect("positive quota + positive periodic interval is a valid config")
    })
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 10: 刷新周期到期重置
    #[test]
    fn property_10_refresh_period_expiry_resets(
        config in periodic_config(),
        // A time-advancing (non-decreasing) sequence of on-chain timestamps at which
        // the chain is evaluated/minted against.
        times in generators::ascending_timestamps(40),
        // A small positive mint amount per step (zipped with `times`), so each period
        // generally accumulates a non-zero `mintedThisPeriod` before a boundary.
        amounts in proptest::collection::vec(1i128..=1_000_000i128, 40),
    ) {
        // The (validated, integer) period length: always present and > 0 here.
        let len = config
            .refresh_period()
            .period_length_secs()
            .expect("a periodic chain with an integer interval has a usable period length");
        prop_assert!(len > 0);

        // A fresh ledger for a periodic chain; first period starts at t = 0, and every
        // generated timestamp is >= 0, so `now >= period_start` initially.
        let mut ledger = QuotaLedger::new(ChainId::new("periodic-chain"), Timestamp::from_secs(0));

        for (now, amount_raw) in times.into_iter().zip(amounts) {
            // State *before* the rollover decision.
            let before_start = ledger.period_start();
            let before_minted = ledger.minted_this_period();
            let elapsed = now.saturating_elapsed_since(before_start);

            // Roll over to the current period if a full Refresh_Period has elapsed.
            let did_reset = ledger.reset_if_elapsed(&config, now);

            // A reset happens *iff* a Refresh_Period boundary was crossed (a full
            // period elapsed since the period start). A periodic chain is never
            // exhausted, so the only gate is the elapsed-time boundary.
            prop_assert_eq!(did_reset, elapsed >= len);

            if did_reset {
                // ---- The core Property 10 assertion: at the new period's start,
                // mintedThisPeriod has been reset to exactly 0. ----
                prop_assert_eq!(ledger.minted_this_period(), Decimal::ZERO);

                // The new period start lands on a Refresh_Period boundary: it is
                // `before_start + k*len` for some k >= 1, so the advance is an exact
                // multiple of `len`, it never overshoots `now`, and `now` falls within
                // the freshly-started period (`now - new_start < len`).
                let new_start = ledger.period_start();
                prop_assert!(new_start.as_secs() >= before_start.as_secs());
                let advanced = new_start.as_secs() - before_start.as_secs();
                prop_assert_eq!(advanced % len, 0);
                prop_assert!(advanced >= len);
                prop_assert!(new_start.as_secs() <= now.as_secs());
                prop_assert!(now.saturating_elapsed_since(new_start) < len);
            } else {
                // No boundary crossed: still inside the current period, so neither the
                // accumulated counter nor the period start may move (accumulation is
                // preserved until a real boundary is reached).
                prop_assert_eq!(ledger.minted_this_period(), before_minted);
                prop_assert_eq!(ledger.period_start(), before_start);
            }

            // Advance the minting sequence: mint within the (post-rollover) period when
            // it fits, so `mintedThisPeriod` generally grows and the next reset is
            // observable. A request that would exceed the quota is simply skipped; the
            // refresh-reset invariant above does not depend on it succeeding.
            let amount = Decimal::from_raw(amount_raw);
            if ledger.check_quota(&config, amount).is_ok() {
                ledger
                    .consume_quota(&config, amount)
                    .expect("a request that passed check_quota must consume successfully");
            }
        }
    }
}
