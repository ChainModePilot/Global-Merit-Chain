//! Property 17 — curMerit 永不低于 minMerit（含批次独立衰减）
//! (`curMerit` never drops below `minMerit`, with per-batch independent decay).
//!
//! This is the dedicated property-based test for **Property 17** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 9.5).
//!
//! > **Property 17: curMerit 永不低于 minMerit（含批次独立衰减）** — 对任意 MeritPocket、
//! > 任意铸造批次组合及任意时间点 `t`，每个批次按
//! > `MeriToken_i(t) = (V_i - B_i)·e^(-λ_i·t) + B_i` 独立衰减（对 `t` 非增、下限为 `B_i`），
//! > 且 `curMerit(t) = Σ_i MeriToken_i(t)` 恒大于或等于 `minMerit`。
//!
//! **Validates: Requirements 8.1, 8.4, 8.5**
//!
//! ## How the pocket is constructed
//!
//! The `curMerit ≥ minMerit` invariant only makes sense for a *valid* pocket — one
//! built the way `Minting_Service` builds it (design *Minting_Service*; reproduced by
//! `merit.rs::invariant_holds_through_a_simulated_mint_pipeline`): the floor starts at
//! `E` *backed* by a batch whose `B = E`, and every subsequent mint raises the floor
//! via `B' = (x + M)·B / M` while creating a batch whose `B_i` equals that floor
//! increment. The per-batch floors therefore telescope to exactly `minMerit`
//! (`Σ_i B_i == minMerit`), and since every batch value stays `≥ B_i`, the sum
//! `curMerit(t) = Σ_i MeriToken_i(t) ≥ Σ_i B_i == minMerit` at every `t`.
//!
//! This test reproduces exactly that construction directly through the `merit.rs`
//! API (no quota machinery) from a random sequence of positive mint amounts /
//! positive influence durations / acquisition times, then asserts Property 17 at a
//! random ascending set of evaluation time points (plus `t = 0` and the far future).
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 17: ...` and runs with `>= 100`
//! random iterations.

mod common;

use common::generators;
use gmc_core::merit::{MeritBatch, MeritPocket, E};
use gmc_core::types::{ChainId, Decimal, FayID, Timestamp};
use proptest::prelude::*;

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 17: curMerit 永不低于 minMerit（含批次独立衰减）
    #[test]
    fn property_17_cur_merit_never_below_min_merit(
        // Each element is one successful mint: (V = amount > 0, influenceDuration > 0).
        mints in proptest::collection::vec(
            (generators::mint_amount_positive(), generators::influence_duration()),
            0..12usize,
        ),
        // Per-mint acquisition times (a non-decreasing sequence of on-chain times).
        acquisition_times in generators::ascending_timestamps(12),
        // Time points `t` at which Property 17 is evaluated (ascending).
        evaluation_times in generators::ascending_timestamps(16),
    ) {
        // Fixed-point exp is non-increasing only up to truncation at the 10^-6
        // resolution, so monotonicity / floor / invariant checks carry a small
        // tolerance (mirrors the unit tests in `merit.rs`).
        let tol = Decimal::from_str("0.0001").expect("valid tolerance literal");
        let chain = ChainId::from("chain-17");

        // --- Build a valid MeritPocket the way Minting_Service builds it. ---
        // Seed: floor `E` backed by a (genuinely decaying) batch with B = E, so that
        // Σ_i B_i starts equal to minMerit. V = 100 (> E) gives a non-zero amplitude.
        let mut pocket = MeritPocket::new(FayID::from("fay-17"));
        pocket.add_batch(
            MeritBatch::with_influence_duration(
                "reg-grant",
                Decimal::from_int(100), // V > E
                E,                      // B = initial floor ⇒ Σ B_i starts == minMerit
                Decimal::from_int(1_000),
                Timestamp::from_secs(0),
                chain.clone(),
            )
            .expect("positive influence duration yields a backing batch"),
        );

        // Replay the mint sequence, telescoping each batch's floor B_i onto minMerit.
        for (i, (amount, influence_duration)) in mints.iter().enumerate() {
            let acquired_at = acquisition_times
                .get(i)
                .copied()
                .unwrap_or_else(|| Timestamp::from_secs(1_000 + i as u64));

            // Snapshot the PRE-mint curMerit M (>= minMerit > 0 here) before mutating.
            let m = pocket.cur_merit(acquired_at);
            let old_floor = pocket.min_merit();

            // Raise the floor via the canonical rule and take B = floor increment.
            let new_floor = pocket
                .update_min_merit(*amount, m)
                .expect("amount > 0 and M > 0 ⇒ floor update succeeds");
            let b = new_floor
                .checked_sub(old_floor)
                .expect("floor increment does not overflow");

            // The batch's V is the mint amount; its B is the floor increment, so the
            // per-batch floors telescope to exactly minMerit. B <= V (since M >= B),
            // so the amplitude (V - B) is non-negative.
            pocket.add_batch(
                MeritBatch::with_influence_duration(
                    format!("mint-{i}"),
                    *amount,
                    b,
                    *influence_duration,
                    acquired_at,
                    chain.clone(),
                )
                .expect("positive influence duration yields a batch"),
            );
        }

        // --- Assemble the ascending set of time points `t` to evaluate at. ---
        // t = 0, every random evaluation time, then the far future (factor -> 0).
        let mut time_points: Vec<Timestamp> = Vec::with_capacity(evaluation_times.len() + 3);
        time_points.push(Timestamp::from_secs(0));
        time_points.extend(evaluation_times.iter().copied());
        time_points.push(Timestamp::from_secs(10_000_000_000));
        time_points.push(Timestamp::from_secs(u64::MAX));

        // --- Part 1: each batch decays non-increasingly in t, with floor B_i. ---
        for batch in &pocket.batches {
            let mut prev = batch.merit_at(time_points[0]);
            for &t in &time_points {
                let value = batch.merit_at(t);

                // Lower bound: MeriToken_i(t) >= B_i (allowing fixed-point slack).
                prop_assert!(
                    value.checked_add(tol).expect("no overflow") >= batch.b,
                    "batch {} dropped below its floor at t={}: value={}, B={}",
                    batch.batch_id,
                    t.as_secs(),
                    value,
                    batch.b,
                );

                // Non-increasing in t: value(t_{k+1}) <= value(t_k) (+ slack).
                prop_assert!(
                    value <= prev.checked_add(tol).expect("no overflow"),
                    "batch {} decay increased at t={}: {} > {}",
                    batch.batch_id,
                    t.as_secs(),
                    value,
                    prev,
                );
                prev = value;
            }
        }

        // --- Part 2: curMerit(t) = Σ_i MeriToken_i(t) and curMerit(t) >= minMerit. ---
        let min_merit = pocket.min_merit();
        for &t in &time_points {
            // curMerit is exactly the sum of the per-batch decayed values (Req 8.4).
            let mut expected_sum = Decimal::ZERO;
            for batch in &pocket.batches {
                expected_sum = expected_sum
                    .checked_add(batch.merit_at(t))
                    .expect("batch-sum does not overflow for realistic inputs");
            }
            prop_assert_eq!(
                pocket.cur_merit(t),
                expected_sum,
                "curMerit != Σ batch values at t={}",
                t.as_secs()
            );

            // The floor invariant (Req 8.5): curMerit(t) >= minMerit (+ slack).
            prop_assert!(
                pocket
                    .cur_merit(t)
                    .checked_add(tol)
                    .expect("no overflow")
                    >= min_merit,
                "curMerit dropped below minMerit at t={}: cur={}, min={}",
                t.as_secs(),
                pocket.cur_merit(t),
                min_merit,
            );
            // And the library helper agrees.
            prop_assert!(
                pocket.invariant_holds(t),
                "invariant_holds() false at t={}: cur={}, min={}",
                t.as_secs(),
                pocket.cur_merit(t),
                min_merit,
            );
        }
    }
}
