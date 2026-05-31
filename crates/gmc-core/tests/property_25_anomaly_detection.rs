//! Property 25 — 异常投票行为检测 (anomalous voting-behaviour detection).
//!
//! **Validates: Requirements 11.4**
//!
//! Property 25 (design doc): *对任意* 投票者在最近 30 天评估窗口内对同一对象的投票历史，
//! **当且仅当** 其赞成投票次数不少于 5 次（`approvals >= 5`）**且** 赞成票占其对该对象
//! 全部投票的比例超过 80%（`approvals / total > 0.8`，严格大于）时，该投票行为被标记为
//! 异常并记入待审计条目。
//!
//! The single proptest below drives [`gmc_core::antifraud::detect_anomaly`] (and the
//! stateful [`gmc_core::antifraud::AntiFraudEngine`]) with randomly generated voting
//! histories and asserts the **iff** relationship between the anomaly criteria and the
//! flagged-as-anomalous outcome.
//!
//! ## How the generated histories sample the criteria space
//!
//! For one fixed `(voter, target)` pair we generate a configurable number of approve
//! and reject votes, all timestamped **inside** the most recent 30-day window. The two
//! counts (`0..=20` each) range over both sides of every threshold:
//!
//! - the `approvals >= 5` count floor, and
//! - the `approvals / total > 0.8` (4/5) ratio threshold, including the exact-80%
//!   boundary (e.g. 8 approvals + 2 rejects) which must **not** be flagged.
//!
//! ## Why the expected ratio test is integer arithmetic
//!
//! The implementation compares `approvals` against `0.8 * total` using the fixed-point
//! [`Decimal`] type. Because `0.8 == 4/5`, that comparison is *exactly* equivalent to
//! the integer test `5 * approvals > 4 * total` (the fixed-point multiply
//! `total * 800_000` carries no truncation remainder). The test computes its
//! expectation with that exact integer test, so it never relies on float rounding.
//!
//! ## Noise that must be ignored
//!
//! Each case also injects votes that the rule must disregard: votes toward a *different
//! object*, votes by a *different voter*, and votes *older than 30 days* (outside the
//! window). None of these may change the tally or the flag — exercising the
//! "对同一对象" + "最近 30 天评估窗口" scoping of Requirement 11.4.

use gmc_core::antifraud::{
    detect_anomaly, AntiFraudEngine, VoteEvent, ANOMALY_MIN_APPROVALS, ANOMALY_WINDOW_SECS,
};
use gmc_core::types::{FayID, Timestamp};
use proptest::prelude::*;

/// Fixed "now" for the evaluation window: 100 days, comfortably larger than the 30-day
/// window so in-window timestamps (`now - offset`) never underflow.
const NOW_SECS: u64 = 100 * 86_400;

/// The voter and object under evaluation.
const VOTER: &str = "voter-under-test";
const TARGET: &str = "obj-under-test";

/// Returns `true` iff the tally meets the Requirement 11.4 anomaly criteria:
/// `approvals >= 5` **and** `approvals / total > 0.8`. The ratio test is expressed as
/// the exact integer equivalent `5 * approvals > 4 * total` (`0.8 == 4/5`).
fn expected_anomalous(approvals: u64, total: u64) -> bool {
    approvals >= ANOMALY_MIN_APPROVALS && total > 0 && 5 * approvals > 4 * total
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(256))]

    // Feature: gmc-core-protocol, Property 25: 异常投票行为检测
    #[test]
    fn property_25_anomaly_detection(
        // In-window approve / reject votes by VOTER toward TARGET.
        approvals in 0u64..=20,
        rejects in 0u64..=20,
        // Noise that MUST be ignored by the rule.
        other_target_votes in 0u64..=6,   // same voter, different object (in window)
        other_voter_votes in 0u64..=6,    // different voter, same object (in window)
        out_of_window_votes in 0u64..=6,  // same voter & object, older than 30 days
    ) {
        let now = Timestamp::from_secs(NOW_SECS);
        let voter = FayID::new(VOTER);

        // --- Build the voting history. ---
        let mut history: Vec<VoteEvent> = Vec::new();
        // A monotonically growing "seconds ago" offset, kept strictly inside the window
        // (offset <= ANOMALY_WINDOW_SECS) for the relevant votes.
        let mut offset: u64 = 0;
        let mut next_in_window = || {
            offset += 1;
            // Stay within [now - 30d, now]: offsets here never exceed the vote budget
            // (<= ~44), which is far below ANOMALY_WINDOW_SECS (2_592_000).
            Timestamp::from_secs(NOW_SECS - offset)
        };

        // The relevant in-window votes by VOTER toward TARGET.
        for _ in 0..approvals {
            history.push(VoteEvent::new(VOTER, TARGET, true, next_in_window()));
        }
        for _ in 0..rejects {
            history.push(VoteEvent::new(VOTER, TARGET, false, next_in_window()));
        }

        // Noise #1: same voter, a DIFFERENT object (in window) -> ignored.
        for _ in 0..other_target_votes {
            history.push(VoteEvent::new(VOTER, "some-other-object", true, next_in_window()));
        }
        // Noise #2: a DIFFERENT voter, same object (in window) -> ignored.
        for _ in 0..other_voter_votes {
            history.push(VoteEvent::new("another-voter", TARGET, true, next_in_window()));
        }
        // Noise #3: same voter & object but OLDER than 30 days -> outside window, ignored.
        for i in 0..out_of_window_votes {
            let secs = NOW_SECS - (ANOMALY_WINDOW_SECS + 1 + i);
            history.push(VoteEvent::new(VOTER, TARGET, true, Timestamp::from_secs(secs)));
        }

        let total = approvals + rejects;
        let should_flag = expected_anomalous(approvals, total);

        // --- detect_anomaly: flagged iff criteria met. ---
        let result = detect_anomaly(&voter, TARGET, &history, now);
        prop_assert_eq!(
            result.is_some(),
            should_flag,
            "approvals={}, total={}: detect_anomaly disagreed with the iff criteria",
            approvals,
            total
        );

        // When flagged, the audit entry must capture exactly the in-window tally.
        if let Some(entry) = result {
            prop_assert_eq!(&entry.voter, &voter);
            prop_assert_eq!(entry.target.as_str(), TARGET);
            prop_assert_eq!(entry.approval_count, approvals);
            prop_assert_eq!(entry.total_count, total);
            prop_assert_eq!(entry.window_end, now);
        }

        // --- AntiFraudEngine: a pending-audit entry is recorded iff anomalous. ---
        let mut engine = AntiFraudEngine::new();
        let recorded = engine.record_if_anomalous(&voter, TARGET, &history, now).is_some();
        prop_assert_eq!(recorded, should_flag);
        prop_assert_eq!(engine.pending_audit().len(), usize::from(should_flag));
        if should_flag {
            let logged = &engine.pending_audit()[0];
            prop_assert_eq!(logged.approval_count, approvals);
            prop_assert_eq!(logged.total_count, total);
            prop_assert_eq!(&logged.voter, &voter);
            prop_assert_eq!(logged.target.as_str(), TARGET);
        }
    }
}
