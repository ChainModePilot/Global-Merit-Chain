//! Property 24 — 投票者选取排除高亲密度且规模合规 (voter selection excludes
//! high-intimacy stakeholders and yields a size-compliant set).
//!
//! Dedicated property-based test for **Property 24** of the `gmc-core-protocol`
//! design's *Correctness Properties* section (task 12.3).
//!
//! > **Property 24: 投票者选取排除高亲密度且规模合规** — 对任意贡献者及其干系人集合
//! > （各带归一化亲密度 ∈ [0,1]），当排除亲密度大于 0.9 的全部实体后剩余干系人不少于
//! > 7 名时，所选投票者集合中每个成员与贡献者的亲密度都不超过 0.9，且集合规模不少于
//! > 7 名且不超过剩余干系人总数。
//!
//! **Validates: Requirements 11.1, 11.2**
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 24: ...` and runs with `>= 100`
//! random iterations. The [`generators::stakeholder_pool`] strategy yields pools with
//! unique ids and intimacy uniformly drawn from `[0, 1]`, so values above the `0.9`
//! exclusion threshold (Requirement 11.1) arise naturally and the post-exclusion
//! remaining count spans both `< 7` (precondition fails, skipped) and `>= 7` (the
//! interesting case asserted here).
//!
//! Voter selection is deterministic at this pure-logic layer: `select_voters` is
//! driven by a caller-supplied `u64` seed through an inline `SplitMix64` PRNG (no
//! `rand` dependency), so the same `(stakeholders, sample_size, seed)` always yields
//! the same voter set — which is asserted below as part of the property.

mod common;

use common::generators;
use gmc_core::antifraud::{
    select_voters, Stakeholder, INTIMACY_EXCLUSION_THRESHOLD, MIN_VOTER_SET_SIZE,
};
use proptest::prelude::*;

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 24: 投票者选取排除高亲密度且规模合规
    #[test]
    fn property_24_voter_selection_excludes_high_intimacy_and_is_size_compliant(
        // A pool large enough that, after ~10% of members exceed 0.9, the remaining
        // set frequently reaches the >= 7 precondition that triggers the assertions.
        pool in generators::stakeholder_pool(40),
        // Exercises sample_size clamping on both ends: < 7 (floored to 7) and
        // > remaining (capped to remaining), plus values in range.
        sample_size in 0usize..=60,
        seed in any::<u64>(),
    ) {
        // Map the generated plain-data pool onto the real `AntiFraud_Engine`
        // `Stakeholder` shape (same fields: unique id + normalized intimacy in [0, 1]).
        let stakeholders: Vec<Stakeholder> = pool
            .iter()
            .map(|s| Stakeholder::new(s.id.clone(), s.intimacy))
            .collect();

        // The post-exclusion remaining set: stakeholders with intimacy <= 0.9
        // (i.e. NOT strictly greater than the 0.9 exclusion threshold, Requirement 11.1).
        let remaining: Vec<&Stakeholder> = stakeholders
            .iter()
            .filter(|s| s.intimacy.value() <= INTIMACY_EXCLUSION_THRESHOLD)
            .collect();
        let remaining_total = remaining.len();

        // Property 24 only constrains the case where >= 7 stakeholders remain after
        // exclusion. (The < 7 case is Requirement 11.3 and covered elsewhere.)
        prop_assume!(remaining_total >= MIN_VOTER_SET_SIZE);

        // Selection must succeed when enough stakeholders remain.
        let voters = select_voters(&stakeholders, sample_size, seed)
            .expect("with >= 7 remaining stakeholders, selection succeeds");

        // (Requirement 11.2) The selected set size is at least 7 and at most the
        // remaining (post-exclusion) total.
        prop_assert!(
            voters.len() >= MIN_VOTER_SET_SIZE,
            "selected {} voters, below the minimum {}",
            voters.len(),
            MIN_VOTER_SET_SIZE
        );
        prop_assert!(
            voters.len() <= remaining_total,
            "selected {} voters, above the remaining total {}",
            voters.len(),
            remaining_total
        );

        // (Requirement 11.1) Every selected voter maps back to a stakeholder whose
        // intimacy with the contributor is <= 0.9 (no excluded high-intimacy entity
        // appears), and selected voters are distinct.
        let mut seen = std::collections::HashSet::new();
        for v in &voters {
            prop_assert!(seen.insert(v.clone()), "voter {v} selected more than once");
            let s = stakeholders
                .iter()
                .find(|s| &s.id == v)
                .expect("every selected voter comes from the input pool");
            prop_assert!(
                s.intimacy.value() <= INTIMACY_EXCLUSION_THRESHOLD,
                "selected voter {v} has intimacy > 0.9 and should have been excluded"
            );
        }

        // Determinism: the same (stakeholders, sample_size, seed) reproduces the same
        // voter set, since selection is seed-driven with no external randomness.
        let voters_again = select_voters(&stakeholders, sample_size, seed)
            .expect("deterministic re-run also succeeds");
        prop_assert_eq!(voters, voters_again, "selection must be reproducible for a fixed seed");
    }
}
