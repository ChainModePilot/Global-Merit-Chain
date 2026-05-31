//! Harness smoke test for the property-based testing infrastructure (task 1.3).
//!
//! This is **NOT** one of the numbered design properties (Property 1–30). It exists
//! only to prove the test harness wiring works end to end:
//!
//! - the shared module include path (`mod common;`) resolves,
//! - the reusable generators in [`common::generators`] produce values, and
//! - the `>= 100` iteration `ProptestConfig` convention compiles and runs.
//!
//! Each numbered property gets its own dedicated test file in a later task, labelled
//! `Feature: gmc-core-protocol, Property N: ...` per the convention in
//! `tests/common/mod.rs`. This smoke test deliberately carries no such label.

mod common;

use common::generators;
use proptest::prelude::*;

proptest! {
    // Mirror the >= 100-iteration convention every numbered property test uses.
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// HARNESS SMOKE TEST (not a numbered property): a derivation-tree request
    /// sequence generator yields a vector no longer than the requested bound.
    #[test]
    fn smoke_derivation_sequence_respects_length_bound(
        ops in generators::derivation_sequence(16),
    ) {
        prop_assert!(ops.len() <= 16);
    }

    /// HARNESS SMOKE TEST (not a numbered property): every stakeholder's intimacy is
    /// a valid `Ratio` in `[0, 1]`, confirming the normalized-intimacy generator.
    #[test]
    fn smoke_stakeholder_intimacy_is_normalized(
        pool in generators::stakeholder_pool(12),
    ) {
        for s in &pool {
            prop_assert!(s.intimacy >= gmc_core::types::Ratio::ZERO);
            prop_assert!(s.intimacy <= gmc_core::types::Ratio::ONE);
        }
    }

    /// HARNESS SMOKE TEST (not a numbered property): every carbon declaration in a
    /// generated sequence references the same voucher id.
    #[test]
    fn smoke_carbon_sequence_shares_one_voucher(
        decls in generators::carbon_declaration_sequence(8),
    ) {
        if let Some(first) = decls.first() {
            for d in &decls {
                prop_assert_eq!(&d.voucher_id, &first.voucher_id);
            }
        }
    }
}
