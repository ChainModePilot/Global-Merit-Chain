//! Property 22 — 事后申报阈值严格更高 (retroactive threshold is strictly higher).
//!
//! This is the dedicated property-based test for **Property 22** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 15.4).
//!
//! > **Property 22: 事后申报阈值严格更高** — For any chain's regular contribution
//! > recognition pass threshold `regular ∈ (0, 1)`, the corresponding retroactive
//! > declaration pass threshold satisfies `retro == max(regular, 2/3)`, is strictly
//! > greater than `regular`, and is never below two-thirds of the total weighted
//! > votes cast (the `2/3` floor).
//!
//! **Validates: Requirements 10.3**
//!
//! ## Reconciling the property's two conditions
//!
//! The property names two conditions that are in tension when `regular >= 2/3`:
//! `retro == max(regular, 2/3)` would force `retro == regular`, yet `retro` must be
//! **strictly** greater than `regular`. The implementation
//! ([`retro_threshold`]) resolves this exactly the way the design documents it — it
//! bumps by one ulp when `regular >= 2/3` so the three self-consistent conditions all
//! hold for every `regular ∈ (0, 1)`:
//!
//! 1. `retro >= 2/3` floor — it is never below [`RETRO_TWO_THIRDS_FLOOR`];
//! 2. `retro >= max(regular, 2/3)`; and
//! 3. `retro > regular` (strictly higher).
//!
//! Equality `retro == max(regular, 2/3)` holds **exactly** on the `regular < 2/3`
//! branch (where the max is the `2/3` floor and `retro` saturates onto it). The
//! `2/3` value is the non-representable-in-6dp truncation handled by the source's
//! [`RETRO_TWO_THIRDS_FLOOR`] constant (`0.666667`); the test compares against that
//! same constant so equality on the boundary holds exactly.
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 22: ...` and runs with `>= 100`
//! random iterations.

use gmc_core::retroactive::{retro_threshold, RETRO_TWO_THIRDS_FLOOR};
use gmc_core::types::{Decimal, Ratio};
use proptest::prelude::*;

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 22: 事后申报阈值严格更高
    #[test]
    fn property_22_retro_threshold_is_strictly_higher(
        // A regular threshold strictly inside the open interval (0, 1): the 6-dp
        // fixed-point raw value ranges over [0.000001, 0.999999], which excludes both
        // endpoints. This is exactly Property 22's domain `regular ∈ (0, 1)`.
        regular_raw in 1i128..Decimal::ONE.raw(),
    ) {
        let regular = Ratio::new(Decimal::from_raw(regular_raw))
            .expect("a raw value in (0, 1) is a valid Ratio");

        // The retroactive threshold derived from the regular threshold (R10.3).
        let retro = retro_threshold(regular);

        // The 2/3 lower bound the retro threshold must clear. Use the *source's*
        // constant (0.666667) rather than recomputing 2/3, so the truncation
        // direction is unambiguous and boundary equality holds exactly.
        let floor = RETRO_TWO_THIRDS_FLOOR;
        let regular_dec = regular.value();

        // max(regular, 2/3 floor).
        let max_regular_floor = if regular_dec >= floor { regular_dec } else { floor };

        // (1) retro is never below the 2/3 floor (i.e. >= two-thirds of total
        //     weighted votes).
        prop_assert!(
            retro.value() >= floor,
            "retro {} must be >= the 2/3 floor {}",
            retro.value(),
            floor
        );

        // (2) retro is strictly greater than the regular threshold.
        prop_assert!(
            retro.value() > regular_dec,
            "retro {} must be strictly > regular {}",
            retro.value(),
            regular_dec
        );

        // (3) retro is at least max(regular, 2/3).
        prop_assert!(
            retro.value() >= max_regular_floor,
            "retro {} must be >= max(regular, 2/3) {}",
            retro.value(),
            max_regular_floor
        );

        // Boundary: when regular is below the 2/3 floor, max(regular, 2/3) IS the
        // floor and retro equals it exactly — `retro == max(regular, 2/3)` holds.
        if regular_dec < floor {
            prop_assert_eq!(retro.value(), floor);
            prop_assert_eq!(retro.value(), max_regular_floor);
        }
    }
}
