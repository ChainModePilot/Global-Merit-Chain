//! Property 14 — 膨胀指数区间校验 (inflation-index range validation).
//!
//! This is the dedicated property-based test for **Property 14** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 8.5).
//!
//! > **Property 14: 膨胀指数区间校验** — *对任意* 膨胀指数配置，当且仅当 Thought 维度取值
//! > 落在 `(1.00, 10.00]`、Training 维度落在 `[0.95, 1.05]`、Technique 维度落在
//! > `[0.01, 1.00]`（均精确到两位小数）时被接受；任一维度越界则该配置被拒绝，且该维度原有
//! > 指数保持不变。
//!
//! **Validates: Requirements 7.1, 7.2, 7.3, 7.4, 7.8**
//!
//! The property is checked against [`InflationIndexConfig::set_inflation_index`]: each
//! generated `(Dimension, Decimal)` entry is applied to a running config, and an
//! implementation-independent oracle decides whether that value lies in the dimension's
//! valid band (to two-decimal precision). The test asserts both halves of the iff:
//!
//! - **Accept ⇔ in band.** A value is accepted exactly when the oracle says it is valid,
//!   and on acceptance the dimension's stored index becomes that value.
//! - **Reject ⇒ preserved.** An out-of-band value is rejected with
//!   [`GmcError::InflationIndexOutOfRange`] and the dimension keeps its prior index
//!   (validate-then-replace, Requirement 7.8). The other two dimensions never change on
//!   a single-dimension set.

mod common;

use common::generators;
use gmc_core::error::GmcError;
use gmc_core::scoring::InflationIndexConfig;
use gmc_core::types::{Decimal, Dimension};
use proptest::prelude::*;

/// Parses a two-decimal band literal into a [`Decimal`].
fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).expect("valid decimal band literal")
}

/// Implementation-independent oracle: `true` iff `value` carries at most two fractional
/// decimal digits (Requirement 7.1: 精确到两位小数).
fn at_most_two_decimals(value: Decimal) -> bool {
    let step = 10i128.pow(Decimal::SCALE_DIGITS - 2);
    value.raw() % step == 0
}

/// Implementation-independent oracle for the per-dimension valid band:
/// Thought `(1.00, 10.00]`, Training `[0.95, 1.05]`, Technique `[0.01, 1.00]`.
fn in_band(dim: Dimension, value: Decimal) -> bool {
    match dim {
        // (1.00, 10.00] — open lower bound (Requirement 7.2).
        Dimension::Thought => value > dec("1.00") && value <= dec("10.00"),
        // [0.95, 1.05] (Requirement 7.3).
        Dimension::Training => value >= dec("0.95") && value <= dec("1.05"),
        // [0.01, 1.00] (Requirement 7.4).
        Dimension::Technique => value >= dec("0.01") && value <= dec("1.00"),
    }
}

/// A value is a valid index iff it is in band AND has at most two decimals.
fn is_valid_index(dim: Dimension, value: Decimal) -> bool {
    in_band(dim, value) && at_most_two_decimals(value)
}

proptest! {
    // Run this numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 14: 膨胀指数区间校验
    #[test]
    fn property_14_inflation_index_range_validation(
        // A sequence of `(Dimension, value)` index updates whose values straddle each
        // dimension's valid band (so both accepted and rejected values arise).
        entries in proptest::collection::vec(generators::inflation_index_entry(), 0..30usize),
    ) {
        // Start from the default, in-range configuration.
        let mut cfg = InflationIndexConfig::default();

        for (dim, value) in entries {
            // Snapshot every dimension's index just before the attempted set, so we can
            // assert (a) the targeted dimension's preserve-on-reject behaviour and
            // (b) that the other two dimensions are never touched by a single-dim set.
            let prior_thought = cfg.get(Dimension::Thought);
            let prior_training = cfg.get(Dimension::Training);
            let prior_technique = cfg.get(Dimension::Technique);
            let prior_target = cfg.get(dim);

            let expected_accept = is_valid_index(dim, value);
            let result = cfg.set_inflation_index(dim, value);

            match result {
                Ok(()) => {
                    // Accepted ⇒ the oracle must agree the value is in band (iff, ⇐).
                    prop_assert!(
                        expected_accept,
                        "accepted an out-of-range index: dim={:?} value={}",
                        dim,
                        value
                    );
                    // ...and the targeted dimension now holds the new value.
                    prop_assert_eq!(cfg.get(dim), value);
                }
                Err(e) => {
                    // Rejected ⇒ the oracle must agree the value is invalid (iff, ⇒).
                    prop_assert!(
                        !expected_accept,
                        "rejected an in-range index: dim={:?} value={}",
                        dim,
                        value
                    );
                    // ...with the documented out-of-range error code.
                    prop_assert_eq!(e, GmcError::InflationIndexOutOfRange);
                    // ...and the dimension's prior index is preserved (Requirement 7.8).
                    prop_assert_eq!(cfg.get(dim), prior_target);
                }
            }

            // A single-dimension set never changes the other two dimensions, regardless
            // of accept/reject.
            if dim != Dimension::Thought {
                prop_assert_eq!(cfg.get(Dimension::Thought), prior_thought);
            }
            if dim != Dimension::Training {
                prop_assert_eq!(cfg.get(Dimension::Training), prior_training);
            }
            if dim != Dimension::Technique {
                prop_assert_eq!(cfg.get(Dimension::Technique), prior_technique);
            }
        }
    }
}
