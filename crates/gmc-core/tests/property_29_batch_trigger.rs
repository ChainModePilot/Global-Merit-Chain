//! Property 29 — 批量证明触发条件 (batch-proof trigger condition).
//!
//! This is the dedicated property-based test for **Property 29** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 19.3).
//!
//! > **Property 29: 批量证明触发条件** — For any contribution-record arrival sequence
//! > and time advance, the `L2_Rollup` submits a batch zero-knowledge proof to
//! > `L1_Settlement` **iff** the records accumulated since the last batch reached
//! > **1,000** OR **60 seconds** have elapsed since the last batch submission
//! > (whichever comes first).
//!
//! **Validates: Requirements 13.3**
//!
//! The trigger arithmetic under test lives in
//! [`gmc_core::l2_rollup::L2Rollup`]: `should_submit_batch` / `batch_trigger` /
//! `try_submit_batch`, with the protocol thresholds
//! [`BATCH_MAX_RECORDS`] (= 1,000) and [`BATCH_MAX_INTERVAL_SECS`] (= 60).
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 29: ...` and runs with `>= 100`
//! random iterations. The test drives a single rollup with a generated sequence of
//! `(records-arriving, time-advance)` steps and, at every step, compares the rollup's
//! decision against an **independent** model of "records since last batch" and
//! "seconds since last batch", asserting the biconditional in both directions.

mod common;

use gmc_core::l2_rollup::{
    BatchTrigger, BufferedRecord, L2Rollup, BATCH_MAX_INTERVAL_SECS, BATCH_MAX_RECORDS,
};
use gmc_core::types::{ChainId, Decimal, FayID, Timestamp};
use proptest::prelude::*;

/// One step of the arrival/time sequence: ingest `record_count` fresh records, then
/// advance the clock by `time_delta` seconds (the step's records arrive at the
/// post-advance time `now`).
///
/// `record_count` reaches above [`BATCH_MAX_RECORDS`] so a single burst can cross the
/// 1,000-record threshold (and, combined with a `>= 60 s` delta, exercise the
/// "whichever comes first" precedence); `time_delta` straddles
/// [`BATCH_MAX_INTERVAL_SECS`] so the 60-second interval trigger is regularly reached.
fn step() -> impl Strategy<Value = (usize, u64)> {
    let count = prop_oneof![
        // Mostly small bursts so records accumulate slowly and the 60 s interval
        // trigger gets exercised before the record threshold is reached.
        6 => 0usize..=50,
        // Medium bursts that accumulate toward 1,000 across a few steps.
        3 => 51usize..=600,
        // Large bursts that can cross 1,000 in a single step.
        1 => 601usize..=1_100,
    ];
    // Deltas straddle the 60 s interval boundary (including 0 for same-instant ingests).
    let delta = 0u64..=90;
    (count, delta)
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 29: 批量证明触发条件
    #[test]
    fn property_29_batch_proof_trigger_condition(
        steps in proptest::collection::vec(step(), 1..=24usize),
    ) {
        const GENESIS_SECS: u64 = 1_000;

        // The rollup under test, with its batch timer anchored at the genesis time.
        let mut rollup = L2Rollup::new(Timestamp::from_secs(GENESIS_SECS));

        // --- Independent model of the trigger inputs (not read from the rollup). ---
        // Records accumulated since the last batch submission (or since genesis).
        let mut model_buffered: usize = 0;
        // On-chain seconds of the last batch submission (or the genesis time).
        let mut model_last_batch_secs: u64 = GENESIS_SECS;
        // Monotonic clock and a global counter for unique record ids.
        let mut now_secs: u64 = GENESIS_SECS;
        let mut next_record: u64 = 0;

        for (record_count, time_delta) in steps {
            // Advance the (monotonic, non-decreasing) clock, then ingest this step's
            // records at the post-advance time.
            now_secs = now_secs.saturating_add(time_delta);
            let now = Timestamp::from_secs(now_secs);

            for _ in 0..record_count {
                rollup.record_contribution(BufferedRecord::new(
                    gmc_core::l2_rollup::RollupRecordId::new(format!("rec-{next_record}")),
                    FayID::new("contributor-1"),
                    ChainId::new("chain-1"),
                    Decimal::from_int(1),
                    now,
                ));
                next_record += 1;
            }
            model_buffered += record_count;

            // --- Independently recompute the Requirement 13.3 trigger condition. ---
            let elapsed = now_secs.saturating_sub(model_last_batch_secs);
            let record_threshold_reached = model_buffered >= BATCH_MAX_RECORDS;
            let interval_elapsed = elapsed >= BATCH_MAX_INTERVAL_SECS;
            // A batch is due iff at least one record is buffered AND (1,000-record
            // threshold OR 60 s elapsed) — whichever comes first.
            let expected_should_submit =
                model_buffered > 0 && (record_threshold_reached || interval_elapsed);
            // When both hold, the record threshold is the "first" trigger.
            let expected_trigger = if !expected_should_submit {
                None
            } else if record_threshold_reached {
                Some(BatchTrigger::RecordThreshold)
            } else {
                Some(BatchTrigger::IntervalElapsed)
            };

            // The model's buffered count must mirror the rollup's buffer exactly.
            prop_assert_eq!(rollup.buffered_len(), model_buffered);

            // (==>) and (<==): the rollup's decision matches the biconditional exactly.
            prop_assert_eq!(rollup.should_submit_batch(now), expected_should_submit);
            prop_assert_eq!(rollup.batch_trigger(now), expected_trigger);

            // Now actually attempt the submission and check the effect.
            let submitted = rollup.try_submit_batch(now);
            match submitted {
                Some(batch) => {
                    // A proof is submitted ONLY when a trigger fired...
                    prop_assert!(expected_should_submit);
                    // ...carrying the correct "whichever comes first" reason...
                    prop_assert_eq!(batch.trigger, expected_trigger.expect("a trigger fired"));
                    // ...and proving exactly the records accumulated since the last batch.
                    prop_assert_eq!(batch.len(), model_buffered);
                    prop_assert_eq!(batch.proof.record_count(), model_buffered);
                    prop_assert_eq!(batch.submitted_at, now);

                    // The rollup resets per-batch state: buffer drained, interval window
                    // restarted at `now`. Mirror that in the model.
                    prop_assert_eq!(rollup.buffered_len(), 0);
                    prop_assert_eq!(rollup.last_batch_at(), now);
                    model_buffered = 0;
                    model_last_batch_secs = now_secs;
                }
                None => {
                    // No proof submitted ONLY when no trigger was due...
                    prop_assert!(!expected_should_submit);
                    // ...and the state is left completely unchanged.
                    prop_assert_eq!(rollup.buffered_len(), model_buffered);
                    prop_assert_eq!(rollup.last_batch_at().as_secs(), model_last_batch_secs);
                }
            }
        }
    }
}
