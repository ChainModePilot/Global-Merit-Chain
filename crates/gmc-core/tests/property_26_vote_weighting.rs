//! Property 26 — 投票按 curMerit 占比加权 (vote weighting by `curMerit` share).
//!
//! **Validates: Requirements 11.5**
//!
//! Property 26 (design doc): *对任意* 投票者集合及其 curMerit 取值，每个投票者的投票
//! 权重等于其 `curMerit` 占该集合 `curMerit` 总和的比例，且所有投票者权重之和等于 1。
//!
//! In other words, for any voter set with a **positive total** `curMerit`:
//!
//! 1. each voter's weight equals `curMerit / Σ(curMerit)`, and
//! 2. all voters' weights sum to `1`.
//!
//! ## Why two generated voter sets in one property
//!
//! [`gmc_core::types::Decimal`] is a fixed-point type whose division truncates toward
//! zero. Summing the *independently rounded* per-voter weights can therefore fall
//! short of exactly `1` by up to `(n − 1)` units in the last place — a fixed-point
//! artefact, not a violation of the proportional rule. This mirrors the note in
//! `governance.rs` and the module's own `weights_are_curmerit_proportions_that_sum_to_one`
//! unit test, which asserts `Σ weights == 1` only for evenly-divisible inputs.
//!
//! To keep the "sum == 1 exactly" assertion robust while still exercising arbitrary
//! merit values, the single property test drives **two** generated voter sets:
//!
//! - **`even_merits`** — an *evenly-divisible* set whose raw `curMerit` values are
//!   `k · partᵢ` with `Σ partᵢ == 10⁶` (one [`Decimal`] unit, `ONE.raw`). Every
//!   `curMeritᵢ / total` is then exact at 6 dp (the `k` cancels and `total.raw` divides
//!   `curMeritᵢ.raw · 10⁶` with no remainder), so the rounded weights sum to **exactly**
//!   `Decimal::ONE`. Both halves of the property are asserted here.
//! - **`arb_merits`** — an *arbitrary* positive set (at least one positive `curMerit`,
//!   so the total is positive). Only the per-voter identity `weightᵢ == curMeritᵢ / total`
//!   is asserted here, since fixed-point truncation may make the sum fall just below `1`.

use std::collections::BTreeMap;

use gmc_core::governance::{GovernanceModule, Voter};
use gmc_core::types::{Decimal, FayID, Ratio};
use proptest::prelude::*;
use proptest::strategy::BoxedStrategy;

/// `ONE.raw` for [`Decimal`] (scale = 6 decimal digits): `10^6`. The evenly-divisible
/// generator builds `curMerit` "parts" that sum to this value, guaranteeing that every
/// `curMerit / total` division is exact at 6 dp (see module docs).
const ONE_RAW: i128 = 1_000_000;

/// Distinct, deterministic voter id for index `i`.
fn voter_id(i: usize) -> FayID {
    FayID::new(format!("voter-{i}"))
}

/// Builds [`Voter`]s with distinct ids from a list of `curMerit` values.
fn voters_from(merits: &[Decimal]) -> Vec<Voter> {
    merits
        .iter()
        .enumerate()
        .map(|(i, m)| Voter::new(voter_id(i), *m))
        .collect()
}

/// An **evenly-divisible** voter merit set: `1..=max_voters` non-negative `curMerit`
/// values whose weights (`curMeritᵢ / total`) are each exact at 6 dp and therefore sum
/// to **exactly** `Decimal::ONE`.
///
/// Construction: pick `n` and a scale `k`, generate `n` strictly-positive integer
/// `parts` that sum to `ONE_RAW` (via `n−1` distinct cut points), then set
/// `curMeritᵢ.raw = k · partᵢ`. Then `total.raw = k · ONE_RAW`, and
/// `weightᵢ = curMeritᵢ / total = partᵢ / ONE_RAW` exactly, so `Σ weightᵢ = 1`.
fn evenly_divisible_merits(max_voters: usize) -> BoxedStrategy<Vec<Decimal>> {
    (1usize..=max_voters, 1i128..=1_000i128)
        .prop_flat_map(|(n, k)| {
            let parts: BoxedStrategy<Vec<i128>> = if n == 1 {
                Just(vec![ONE_RAW]).boxed()
            } else {
                // `n - 1` distinct cut points in `1..ONE_RAW` partition `[0, ONE_RAW]`
                // into `n` strictly-positive parts that sum to exactly `ONE_RAW`.
                proptest::collection::btree_set(1i128..ONE_RAW, (n - 1)..=(n - 1))
                    .prop_map(move |cuts| {
                        let mut prev = 0i128;
                        let mut parts = Vec::with_capacity(n);
                        for cut in cuts {
                            parts.push(cut - prev);
                            prev = cut;
                        }
                        parts.push(ONE_RAW - prev);
                        parts
                    })
                    .boxed()
            };
            parts.prop_map(move |ps| ps.into_iter().map(|p| Decimal::from_raw(p * k)).collect())
        })
        .boxed()
}

/// An **arbitrary** positive voter merit set: `1..=max_voters` non-negative `curMerit`
/// values with at least one strictly positive, guaranteeing a positive total (so the
/// weighting is well-defined and `open_vote` accepts it).
fn arbitrary_positive_merits(max_voters: usize) -> impl Strategy<Value = Vec<Decimal>> {
    (1usize..=max_voters).prop_flat_map(|n| {
        (
            1i128..=1_000_000_000i128,
            proptest::collection::vec(0i128..=1_000_000_000i128, n - 1),
        )
            .prop_map(|(head, tail)| {
                let mut merits = Vec::with_capacity(tail.len() + 1);
                merits.push(Decimal::from_raw(head));
                merits.extend(tail.into_iter().map(Decimal::from_raw));
                merits
            })
    })
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 26: 投票按 curMerit 占比加权
    #[test]
    fn property_26_vote_weighting(
        even_merits in evenly_divisible_merits(8),
        arb_merits in arbitrary_positive_merits(8),
    ) {
        // ---- Part A: evenly-divisible set -- per-voter identity AND Σ weights == 1 ----
        let voters_a = voters_from(&even_merits);
        let merit_a: BTreeMap<FayID, Decimal> =
            voters_a.iter().map(|v| (v.id.clone(), v.cur_merit)).collect();

        let mut gov_a = GovernanceModule::new();
        let vote_a = gov_a
            .open_vote("property-26-even", Ratio::from_percent(50).unwrap(), voters_a.clone())
            .expect("evenly-divisible electorate has a positive total merit");

        let total_a = gov_a.total_merit(vote_a).expect("vote exists");
        let weights_a = gov_a.voter_weights(vote_a).expect("vote exists");

        // Every electorate member is reported exactly once.
        prop_assert_eq!(weights_a.len(), voters_a.len());

        // (1) Each weight equals curMerit / Σ(curMerit), per the module's fixed-point div.
        for (id, weight) in &weights_a {
            let cur_merit = merit_a.get(id).expect("weight id is in the electorate");
            let expected = cur_merit
                .checked_div(total_a)
                .expect("total merit is positive");
            prop_assert_eq!(weight.value(), expected);
        }

        // (2) All weights sum to exactly 1 (the even-divisibility construction makes
        //     every per-voter division exact, so no truncation accumulates).
        let sum_a = weights_a
            .iter()
            .try_fold(Decimal::ZERO, |acc, (_, w)| acc.checked_add(w.value()))
            .expect("weight sum does not overflow");
        prop_assert_eq!(sum_a, Decimal::ONE);

        // ---- Part B: arbitrary positive set -- per-voter identity holds for all inputs ----
        let voters_b = voters_from(&arb_merits);
        let merit_b: BTreeMap<FayID, Decimal> =
            voters_b.iter().map(|v| (v.id.clone(), v.cur_merit)).collect();

        let mut gov_b = GovernanceModule::new();
        let vote_b = gov_b
            .open_vote("property-26-arbitrary", Ratio::ZERO, voters_b.clone())
            .expect("at least one positive curMerit => positive total");

        let total_b = gov_b.total_merit(vote_b).expect("vote exists");
        let weights_b = gov_b.voter_weights(vote_b).expect("vote exists");

        prop_assert_eq!(weights_b.len(), voters_b.len());

        // Each weight equals curMerit / Σ(curMerit), even when fixed-point truncation
        // means the weights need not sum to exactly 1.
        for (id, weight) in &weights_b {
            let cur_merit = merit_b.get(id).expect("weight id is in the electorate");
            let expected = cur_merit
                .checked_div(total_b)
                .expect("total merit is positive");
            prop_assert_eq!(weight.value(), expected);
        }
    }
}
