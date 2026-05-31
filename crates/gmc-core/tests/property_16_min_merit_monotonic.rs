//! Property 16 — minMerit 单调非减 (minMerit is monotonically non-decreasing).
//!
//! This is the dedicated property-based test for **Property 16** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 9.4).
//!
//! > **Property 16: minMerit 单调非减** — 对任意 MeritPocket 及任意成功铸造请求序列
//! > （非惩罚场景），每次铸造后的 `minMerit` 都大于或等于铸造前的 `minMerit`
//! > （由 `B' = (x+M)×B/M`，在 `x>0` 且 `M≥B` 下恒有 `B'≥B`）。
//!
//! **Validates: Requirements 8.2**
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 16: ...` and runs with `>= 100`
//! random iterations.
//!
//! The test drives the real `Minting_Service` mint pipeline — the faithful
//! representation of a "成功铸造请求序列" (successful, non-penalty mint sequence) —
//! over an arbitrary starting [`MeritPocket`]. Each generated mint uses a strictly
//! positive amount and a strictly positive influence duration, against a quota large
//! enough that no request is ever rejected, so every mint succeeds. After each
//! successful mint the test asserts the floor never decreased
//! (`minMerit_after >= minMerit_before`), exactly the Property 16 invariant.

mod common;
use common::generators;

use gmc_core::merit::{MeritBatch, MeritPocket};
use gmc_core::minting::{MintRequest, MintingService};
use gmc_core::quota::{QuotaConfig, QuotaLedger, RefreshPeriod};
use gmc_core::types::{ChainId, Decimal, FayID, Timestamp};
use proptest::prelude::*;

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 16: minMerit 单调非减
    #[test]
    fn property_16_min_merit_monotonic(
        // "对任意 MeritPocket": vary the pocket's starting floor B0 (a positive
        // Decimal). A backing batch with B == B0 keeps curMerit >= minMerit, so the
        // floor-update rule's M >= B precondition holds for a realistic, non-penalty
        // pocket.
        initial_floor_raw in 1_000_000i128..=1_000_000_000i128, // value 1.0 ..= 1000.0
        // "任意成功铸造请求序列": each step is a successful mint with a strictly
        // positive amount (x > 0), a strictly positive influence duration (> 0), and
        // a time offset (advancing the acquisition time across the sequence).
        mints in proptest::collection::vec(
            (
                generators::mint_amount_positive(),
                generators::influence_duration(),
                0u64..=1_000_000u64,
            ),
            1..30usize,
        ),
    ) {
        // --- Build an arbitrary starting MeritPocket (non-penalty). ---
        let initial_floor = Decimal::from_raw(initial_floor_raw);
        let mut pocket = MeritPocket::new(FayID::new("fay-1"));
        pocket.min_merit = initial_floor;
        // A slowly-decaying backing batch whose floor contribution B == B0 makes
        // Σ B_i start equal to minMerit, so curMerit >= minMerit holds throughout
        // (mirrors the minting.rs backed-pocket pattern).
        pocket.add_batch(MeritBatch::new(
            "backing",
            initial_floor.checked_add(Decimal::from_int(100)).expect("no overflow"), // V > B
            initial_floor,                                                            // B == B0
            Decimal::from_str("0.0001").expect("valid lambda"),
            Decimal::from_int(1_000_000),
            Timestamp::from_secs(0),
            ChainId::from("chain-1"),
        ));

        // Quota large enough that no mint in the sequence is ever rejected: a
        // one-time allowance far above the maximum possible total of the sequence
        // (< 30 mints × max amount 1000.0). Every request therefore succeeds.
        let service = MintingService::new();
        let config = QuotaConfig::new(Decimal::from_int(1_000_000_000), RefreshPeriod::OneTime)
            .expect("valid one-time quota config");
        let mut ledger = QuotaLedger::new(ChainId::from("chain-1"), Timestamp::from_secs(0));

        // --- Drive the successful mint sequence; assert monotonic non-decrease. ---
        let mut now: u64 = 0;
        for (step, (amount, influence_duration, delta)) in mints.into_iter().enumerate() {
            now = now.saturating_add(delta);
            let floor_before = pocket.min_merit();

            let receipt = service
                .mint(
                    &mut pocket,
                    &config,
                    &mut ledger,
                    MintRequest::new(
                        format!("m{step}"),
                        amount,
                        influence_duration,
                        Timestamp::from_secs(now),
                        ChainId::from("chain-1"),
                    ),
                )
                .expect("each generated mint (x > 0, duration > 0, within quota) must succeed");

            let floor_after = pocket.min_merit();

            // Property 16: every mint leaves minMerit >= the pre-mint minMerit.
            prop_assert!(
                floor_after >= floor_before,
                "minMerit decreased on mint {step}: before={floor_before}, after={floor_after} \
                 (amount={amount})"
            );
            // The receipt's reported floor agrees with the pocket and is non-decreasing.
            prop_assert_eq!(receipt.new_min_merit, floor_after);
            prop_assert!(receipt.new_min_merit >= floor_before);
        }
    }
}
