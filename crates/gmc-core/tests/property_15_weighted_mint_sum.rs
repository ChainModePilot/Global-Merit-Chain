//! Property 15 — 铸造数量按维度加权求和且为正
//! (single-mint amount is the per-dimension weighted sum, and is strictly positive).
//!
//! This is the dedicated property-based test for **Property 15** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 8.6).
//!
//! > **Property 15: 铸造数量按维度加权求和且为正** — 对任意 含 1 至 3 个适用维度的合法贡献
//! > （各维度基础分非负且至少一维大于零、占比之和为 1、各维度指数落在规定区间），
//! > `Scoring_Engine` 计算的单次铸造数量等于 Σ_dim (占比_dim × 基础分_dim × 膨胀指数_dim)，
//! > 且该结果严格大于零。
//!
//! **Validates: Requirements 7.5, 7.6, 8.3**
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 15: ...` and runs with `>= 100`
//! random iterations. Inputs are assembled from the shared generators so each sample
//! is a *legal* contribution: `dimension_weights_sum_one` yields 1..=3 dimensions whose
//! proportions sum to exactly 1, `inflation_index_in_range` yields a per-dimension index
//! inside its valid band, and `base_score` yields non-negative base scores. To keep the
//! "at least one base > 0 ⇒ result strictly > 0" guarantee robust under fixed-point
//! truncation, the first present dimension's base is lifted by a positive floor so its
//! contribution always survives the `SCALE_DIGITS = 6` rounding.
//!
//! The expected amount is recomputed independently with the same fixed-point [`Decimal`]
//! operations the engine uses (so the SCALE_DIGITS=6 truncation is accounted for and the
//! comparison is exact), then asserted equal to the engine's output and strictly > 0.

mod common;

use common::generators;
use gmc_core::scoring::{BaseScores, InflationIndexConfig, ScoringEngine};
use gmc_core::types::{Decimal, Dimension};
use proptest::prelude::*;

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(256))]

    // Feature: gmc-core-protocol, Property 15: 铸造数量按维度加权求和且为正
    #[test]
    fn property_15_weighted_mint_sum(
        // 1..=3 distinct dimensions, each weight in (0, 1], Σ == 1 (a valid classification).
        weights in generators::dimension_weights_sum_one(),
        // Non-negative base scores per dimension (only present dimensions are used; some
        // may be zero — the property only requires *at least one* base > 0).
        thought_base in generators::base_score(),
        training_base in generators::base_score(),
        technique_base in generators::base_score(),
        // Inflation indices inside each dimension's valid band (two-decimal precision).
        thought_index in generators::inflation_index_in_range(Dimension::Thought),
        training_index in generators::inflation_index_in_range(Dimension::Training),
        technique_index in generators::inflation_index_in_range(Dimension::Technique),
    ) {
        let engine = ScoringEngine::new();

        // Present dimensions, in the stable (BTreeMap) order shared by `iter`/`dimensions`.
        let present: Vec<Dimension> = weights.dimensions().collect();
        prop_assert!(!present.is_empty() && present.len() <= 3);

        // Look up the generated per-dimension operands.
        let generated_base = |d: Dimension| match d {
            Dimension::Thought => thought_base,
            Dimension::Training => training_base,
            Dimension::Technique => technique_base,
        };
        let in_range_index = |d: Dimension| match d {
            Dimension::Thought => thought_index,
            Dimension::Training => training_index,
            Dimension::Technique => technique_index,
        };

        // Inflation-index config: set each present dimension to its in-range value. The
        // generators emit two-decimal values inside the band, so the setter accepts them.
        let mut indices = InflationIndexConfig::default();
        for &d in &present {
            indices
                .set_inflation_index(d, in_range_index(d))
                .expect("generated inflation index is in range to two decimals");
        }

        // Base scores: every present dimension carries a non-negative base. The first
        // present dimension is lifted by a positive floor (>= 1000) so that — even at the
        // smallest possible weight (1e-6) and index (0.01) — its weighted contribution
        // does not truncate to zero. This realises the legal-contribution precondition
        // "各维度基础分非负且至少一维大于零" while keeping the strictly-positive result
        // robust against SCALE_DIGITS=6 fixed-point truncation.
        let primary = present[0];
        let floor = Decimal::from_int(1000);
        let mut base_scores = BaseScores::new();
        for &d in &present {
            let mut base = generated_base(d);
            if d == primary {
                base = base.checked_add(floor).expect("base + floor does not overflow");
            }
            base_scores.set(d, base);
        }

        // Independent reference: Σ_dim (占比_dim × 基础分_dim × 膨胀指数_dim), evaluated with
        // the same fixed-point Decimal ops (same per-term truncation, same summation) the
        // engine performs, so equality with the engine output is exact.
        let mut expected = Decimal::ZERO;
        for (d, ratio) in weights.iter() {
            let base = base_scores.get(d).expect("present dimension has a base score");
            let index = indices.get(d);
            let contribution = ratio
                .value()
                .checked_mul(base)
                .and_then(|weighted_base| weighted_base.checked_mul(index))
                .expect("bounded operands do not overflow");
            expected = expected
                .checked_add(contribution)
                .expect("the weighted sum does not overflow");
        }

        // The reference weighted sum is strictly positive for a legal contribution.
        prop_assert!(
            expected.is_positive(),
            "weighted sum should be strictly positive, got {expected}"
        );

        // The engine computes exactly the weighted sum and accepts it (strictly > 0).
        let amount = engine
            .compute_mint_amount(&weights, &base_scores, &indices)
            .expect("a legal contribution mints a positive amount");
        prop_assert_eq!(amount, expected);
        prop_assert!(amount.is_positive());
    }
}
