//! Property 20 — 授予三条件守卫 (grant three-condition guard).
//!
//! This is the dedicated property-based test for **Property 20** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 11.6).
//!
//! > **Property 20: 授予三条件守卫** — For any boolean combination of the three
//! > conditions (a matching *valid* registration exists, an *associated* contribution
//! > record exists, and that record *passed* evaluation), the mint/grant action is
//! > triggered **iff** all three conditions are simultaneously true; when *any* single
//! > condition is false, **no** MeriToken is minted.
//!
//! **Validates: Requirements 9.5, 9.6, 9.8**
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 20: ...` and runs with `>= 100`
//! random iterations.
//!
//! ## How the three conditions are driven
//!
//! The授予 (grant) guard's integration entry point is
//! [`RegistrationService::can_grant`], which:
//!
//! 1. supplies **condition 1** itself — "a matching valid registration exists" — via
//!    `find_valid_registration(contributor, chain)`; and
//! 2. combines it with **conditions 2 & 3** — `has_linked_record()` /
//!    `evaluation_passed()` — read from a [`GrantContext`].
//!
//! So for each generated `(contributor, chain)` we exercise the **full 2×2×2 truth
//! table** of the three booleans: condition 1 is made true/false by registering (or
//! not) a matching `Valid` registration, and conditions 2/3 are supplied by a stub
//! [`GrantContext`]. We then assert the guard fires (and a mint would be triggered)
//! exactly when all three are true, and never otherwise (_Requirements 9.5, 9.6, 9.8_).

use gmc_core::registration::{GrantContext, RegistrationApplication, RegistrationService};
use gmc_core::types::{ChainId, FayID, Timestamp};
use proptest::prelude::*;

/// Test double for [`GrantContext`]: carries conditions 2 and 3 (whether an
/// associated contribution record exists, and whether it passed evaluation).
struct StubGrantContext {
    has_linked_record: bool,
    evaluation_passed: bool,
}

impl GrantContext for StubGrantContext {
    fn has_linked_record(&self) -> bool {
        self.has_linked_record
    }
    fn evaluation_passed(&self) -> bool {
        self.evaluation_passed
    }
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 20: 授予三条件守卫
    #[test]
    fn property_20_grant_three_condition_guard(
        // Arbitrary (non-empty) contributor and chain identifiers; the property must
        // hold for *any* such pair. (Non-empty so a matching registration is acceptable
        // when condition 1 is meant to be true — see Requirement 9.1 field rules.)
        contributor in "[a-z][a-z0-9]{0,7}",
        chain in "[a-z][a-z0-9]{0,7}",
    ) {
        let contributor_id = FayID::new(contributor);
        let chain_id = ChainId::new(chain);

        // Exercise the full boolean truth table of the three grant conditions.
        for has_valid_registration in [false, true] {
            for has_linked_record in [false, true] {
                for evaluation_passed in [false, true] {
                    // Condition 1 ("matching valid registration exists") is realised by
                    // registering (or not) a matching Valid registration in the service.
                    let mut service = RegistrationService::new();
                    if has_valid_registration {
                        service
                            .register(RegistrationApplication::new(
                                contributor_id.clone(),
                                chain_id.clone(),
                                "intended contribution",
                                Timestamp::from_secs(1_000),
                            ))
                            .expect("a complete, in-bounds application must be accepted");
                    }

                    // Conditions 2 & 3 come from the GrantContext.
                    let ctx = StubGrantContext {
                        has_linked_record,
                        evaluation_passed,
                    };

                    // The pure three-condition guard (design's canGrant).
                    let granted = service.can_grant(&contributor_id, &chain_id, &ctx);

                    // Model the grant/mint trigger: the integration layer invokes
                    // Minting_Service.mint exactly once iff the guard returns true; a
                    // false guard mints nothing (Requirements 9.5/9.6).
                    let mint_count = if granted { 1u32 } else { 0u32 };

                    // The grant is triggered iff ALL THREE conditions hold (Req 9.8).
                    let all_three =
                        has_valid_registration && has_linked_record && evaluation_passed;

                    prop_assert_eq!(
                        granted,
                        all_three,
                        "grant guard must fire iff all three conditions hold \
                         (valid_reg={}, linked_record={}, evaluation_passed={})",
                        has_valid_registration,
                        has_linked_record,
                        evaluation_passed
                    );

                    // No mint on any false condition; exactly one mint when all true.
                    prop_assert_eq!(mint_count, if all_three { 1 } else { 0 });
                }
            }
        }
    }
}
