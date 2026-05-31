//! Property 13 — 维度占比和为 100% (dimension proportions sum to 100%).
//!
//! This is the dedicated property-based test for **Property 13** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 8.4).
//!
//! > **Property 13: 维度占比和为 100%** — 对任意 被成功归类的贡献，`Scoring_Engine`
//! > 输出的维度集合规模在 1 至 3 之间，每个适用维度占比落在 (0, 1] 内，且所有适用
//! > 维度占比之和恰好等于 1（即 100%）。
//!
//! **Validates: Requirements 6.1, 6.5**
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 13: ...` and runs with `>= 100`
//! random iterations.
//!
//! The property quantifies over *successfully classified* contributions, so the test
//! drives [`ScoringEngine::classify`] with the **mixed** weight generator
//! (`generators::dimension_weights`), which produces both `Σ == 1` and `Σ != 1`
//! samples. Inputs that `classify` rejects are skipped (they are not "successfully
//! classified"); for every input it *accepts*, the three Property-13 invariants are
//! asserted on the returned [`DimensionWeights`]:
//!
//! 1. the dimension set size is in `1..=3`,
//! 2. each applicable dimension's proportion lies in `(0, 1]`, and
//! 3. all applicable proportions sum to exactly `1` (100%).

mod common;
use common::generators;

use gmc_core::scoring::ScoringEngine;
use gmc_core::types::{Decimal, Ratio};
use proptest::prelude::*;

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 13: 维度占比和为 100%
    #[test]
    fn property_13_dimension_weights_sum_to_one(
        // Mixed weight maps: some sum to exactly 1, some do not. This lets us observe
        // the engine's *accept* decision rather than presupposing it.
        proposed in generators::dimension_weights(),
    ) {
        let engine = ScoringEngine::new();

        // Only "被成功归类的贡献" (successfully classified contributions) are in scope.
        // A rejected classification is not a counterexample to Property 13.
        if let Ok(classified) = engine.classify(proposed) {
            // (1) The output dimension set size is in 1..=3.
            let size = classified.len();
            prop_assert!(
                (1..=3).contains(&size),
                "dimension set size {size} is outside 1..=3",
            );

            // (2) Each applicable dimension's proportion lies in (0, 1]:
            //     strictly greater than 0 and at most 1.
            for (dimension, ratio) in classified.iter() {
                prop_assert!(
                    !ratio.is_zero(),
                    "dimension {dimension:?} has a zero proportion (must be > 0)",
                );
                prop_assert!(
                    ratio.value().is_positive(),
                    "dimension {dimension:?} proportion is not strictly positive",
                );
                prop_assert!(
                    ratio <= Ratio::ONE,
                    "dimension {dimension:?} proportion exceeds 1",
                );
            }

            // (3) All applicable proportions sum to exactly 1 (100%).
            let sum = classified
                .weight_sum()
                .expect("an accepted classification has a representable weight sum");
            prop_assert_eq!(
                sum,
                Decimal::ONE,
                "applicable proportions must sum to exactly 1 (100%)",
            );
        }
    }
}
