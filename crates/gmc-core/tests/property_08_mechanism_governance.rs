//! Property 8 — 未达治理阈值则配置不变 (governance-threshold-gated mechanism change).
//!
//! This is one of the design document's 30 numbered correctness properties. It pins
//! down the governance gate on an [`EvaluationMechanism`] change
//! ([`EvaluationMechanismSlot::propose_change`]):
//!
//! - When the chain's governance threshold is **not** reached
//!   ([`GovernanceDecision::ThresholdNotReached`]), the change is rejected with
//!   [`GmcError::GovernanceThresholdNotMet`] **and** the slot's existing config is
//!   preserved byte-for-byte (Requirement 3.7 — the focus of this property).
//! - When the threshold **is** reached, the change is applied and the slot now holds
//!   the proposed config (Requirement 3.4 — included so the gate's two sides are
//!   exercised together).
//!
//! Both the current config and the proposed config are drawn as *valid*
//! [`EvaluationMechanism`] values (≥1 acquisition mode, threshold in `(0, 1]`), so the
//! only thing under test is the governance gate, never config validity.
//!
//! Per the project's property-test conventions this property is implemented by exactly
//! one `proptest` test, run with ≥ 100 random cases, and labelled with the
//! `Feature: gmc-core-protocol, Property N: ...` comment directly above the test fn.

use gmc_core::error::GmcError;
use gmc_core::mechanism::{
    AcquisitionMode, EvaluationMechanism, EvaluationMechanismSlot, GovernanceDecision,
};
use gmc_core::types::{Decimal, Ratio};
use proptest::prelude::*;

/// `10^Decimal::SCALE_DIGITS` — the raw scaling factor of the fixed-point `Decimal`.
/// A raw value of `SCALE` equals `1.0`; `1` equals the smallest positive step.
const SCALE: i128 = 1_000_000;

/// A non-empty set of [`AcquisitionMode`]s (Requirement 3.3 requires ≥ 1).
///
/// There are only two modes, so the three possibilities are: each alone, or both.
fn acquisition_modes() -> impl Strategy<Value = Vec<AcquisitionMode>> {
    prop_oneof![
        Just(vec![AcquisitionMode::ObjectiveMetering]),
        Just(vec![AcquisitionMode::BountyTask]),
        Just(vec![
            AcquisitionMode::ObjectiveMetering,
            AcquisitionMode::BountyTask,
        ]),
    ]
}

/// A consensus threshold strictly inside `(0, 1]` (Requirements 3.3/3.6): raw values
/// `1..=SCALE` exclude exactly `0` and include the inclusive upper bound `1.0`.
fn threshold() -> impl Strategy<Value = Ratio> {
    (1i128..=SCALE).prop_map(|raw| {
        Ratio::new(Decimal::from_raw(raw)).expect("raw in 1..=SCALE lies in (0, 1]")
    })
}

/// A valid [`EvaluationMechanism`]: ≥ 1 acquisition mode, threshold in `(0, 1]`, and an
/// arbitrary high-intimacy-exclusion flag. Construction must always succeed.
fn valid_mechanism() -> impl Strategy<Value = EvaluationMechanism> {
    (acquisition_modes(), threshold(), any::<bool>()).prop_map(|(modes, thr, exclude)| {
        EvaluationMechanism::new(modes, thr, exclude).expect("generated config is valid")
    })
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 8: 未达治理阈值则配置不变
    //
    // Validates: Requirements 3.7
    #[test]
    fn property_8_governance_threshold_not_met_preserves_config(
        current in valid_mechanism(),
        proposed in valid_mechanism(),
        threshold_reached in any::<bool>(),
    ) {
        // Each case starts from a slot already holding a valid current config.
        let mut slot = EvaluationMechanismSlot::with_config(current.clone());

        // The governance outcome is an input: true -> ThresholdReached, else NotReached.
        let decision = GovernanceDecision::from_passed(threshold_reached);

        let result = slot.propose_change(proposed.clone(), decision);

        if threshold_reached {
            // Requirement 3.4: a change that reached the threshold takes effect, and the
            // stored config becomes the proposed one.
            prop_assert!(
                result.is_ok(),
                "threshold-reached change must be applied, got {result:?}"
            );
            prop_assert_eq!(slot.current(), Some(&proposed));
        } else {
            // Requirement 3.7 (the property's focus): a not-reached change is rejected
            // with GovernanceThresholdNotMet and the existing config is preserved
            // unchanged for EVERY not-reached case.
            prop_assert_eq!(
                result.expect_err("not-reached change must be rejected"),
                GmcError::GovernanceThresholdNotMet
            );
            prop_assert_eq!(slot.current(), Some(&current));
        }
    }
}
