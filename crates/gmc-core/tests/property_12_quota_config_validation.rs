//! Property 12 — 配额与刷新周期配置校验 (quota & refresh-period config validation).
//!
//! This is the dedicated property-based test for **Property 12** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 6.7).
//!
//! > **Property 12: 配额与刷新周期配置校验** — 对任意 链配置，当且仅当 `Quota` 为大于零的
//! > 有限数值，且 `Refresh_Period` 为"一次性"或带显式时间单位（秒/小时/天）且取值大于零的
//! > 有限间隔时被接受；否则配置被拒绝。
//!
//! **Validates: Requirements 4.1, 4.8**
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 12: ...` and runs with `>= 100`
//! random iterations. Inputs are built directly here: a `quota` [`Decimal`] (biased to
//! cover positive / zero / negative) and a [`RefreshPeriod`] that is either `OneTime`
//! or `Periodic` with an explicit [`TimeUnit`] and an interval `value` (likewise biased
//! across positive / zero / negative). The property then asserts that
//! [`QuotaConfig::new`] accepts a config **iff** the quota is strictly positive *and*
//! the refresh period is `OneTime` or a `Periodic` with `value > 0`; every rejection is
//! the [`GmcError::QuotaConfigInvalid`] validation error.
//!
//! > Note on "finite": [`Decimal`] is an `i128`-backed fixed-point type, so every
//! > representable value is inherently finite (no NaN/∞). The "finite" clause of
//! > Requirements 4.1/4.8 is therefore satisfied by construction, and the meaningful
//! > acceptance test reduces to the strict-positivity checks exercised below.

use gmc_core::error::GmcError;
use gmc_core::quota::{QuotaConfig, RefreshPeriod, TimeUnit};
use gmc_core::types::Decimal;
use proptest::prelude::*;

/// A [`Decimal`] biased to land on the acceptance boundary: it is explicitly drawn
/// from {zero, strictly-positive, strictly-negative} so that the `> 0` predicate is
/// exercised on both sides (and exactly at zero) far more often than uniform sampling
/// of a wide range would manage.
fn boundary_decimal() -> impl Strategy<Value = Decimal> {
    prop_oneof![
        1 => Just(Decimal::ZERO),
        3 => (1i128..=1_000_000_000i128).prop_map(Decimal::from_raw),
        3 => (-1_000_000_000i128..=-1i128).prop_map(Decimal::from_raw),
    ]
}

/// One of the three explicit [`TimeUnit`]s (second / hour / day).
fn time_unit() -> impl Strategy<Value = TimeUnit> {
    prop_oneof![
        Just(TimeUnit::Second),
        Just(TimeUnit::Hour),
        Just(TimeUnit::Day),
    ]
}

/// A [`RefreshPeriod`] that is either the non-renewing `OneTime` variant or a
/// `Periodic` with an explicit unit and a (possibly invalid, i.e. `<= 0`) interval
/// value, so both the accepted and rejected period shapes arise.
fn refresh_period() -> impl Strategy<Value = RefreshPeriod> {
    prop_oneof![
        1 => Just(RefreshPeriod::OneTime),
        3 => (time_unit(), boundary_decimal())
            .prop_map(|(unit, value)| RefreshPeriod::Periodic { unit, value }),
    ]
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 12: 配额与刷新周期配置校验
    #[test]
    fn property_12_quota_config_validation(
        quota in boundary_decimal(),
        refresh_period in refresh_period(),
    ) {
        // Independently compute whether this config *should* be accepted, straight from
        // the property text:
        //   - the quota is a strictly-positive finite value, AND
        //   - the refresh period is "一次性" (OneTime), OR a Periodic with an explicit
        //     time unit and a strictly-positive (finite) interval value.
        // (Decimal is fixed-point, so "finite" holds by construction.)
        let quota_ok = quota.is_positive();
        let period_ok = match refresh_period {
            RefreshPeriod::OneTime => true,
            RefreshPeriod::Periodic { value, .. } => value.is_positive(),
        };
        let expected_accept = quota_ok && period_ok;

        match QuotaConfig::new(quota, refresh_period) {
            Ok(config) => {
                // Accepted iff the config satisfies the acceptance rule (Req 4.1).
                prop_assert!(
                    expected_accept,
                    "config was accepted but violates the rule: quota={quota}, period={refresh_period:?}"
                );
                // An accepted config round-trips its inputs faithfully.
                prop_assert_eq!(config.quota(), quota);
                prop_assert_eq!(config.refresh_period(), refresh_period);
                prop_assert_eq!(config.is_one_time(), refresh_period.is_one_time());
            }
            Err(err) => {
                // Rejected iff the config breaks the rule, with the validation error (Req 4.8).
                prop_assert!(
                    !expected_accept,
                    "config was rejected but satisfies the rule: quota={quota}, period={refresh_period:?}"
                );
                prop_assert_eq!(err, GmcError::QuotaConfigInvalid);
            }
        }
    }
}
