//! Property 30 — 证明验证失败保留前一状态根
//! (failed proof verification retains the previous confirmed state root).
//!
//! This is the dedicated property-based test for **Property 30** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 18.3).
//!
//! > **Property 30: 证明验证失败保留前一状态根** — 对任意提交到 L1 的批次证明序列（含有效与
//! > 无效证明），L1 的已确认状态根仅在证明验证通过时更新为该批次状态根；任何验证失败的批次都
//! > 被拒绝更新，状态根保留为上一已确认值。
//!
//! **Validates: Requirements 13.8**
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 30: ...` and runs with `>= 100`
//! random iterations. Each generated step drives one
//! [`L1Settlement::submit_batch_proof`] call against a freshly built [`BatchProof`]:
//!
//! - a **verifying** proof (valid *and* committing to exactly the submitted batch
//!   root) — the batch is accepted and the confirmed root must advance to that batch
//!   root; or
//! - a **non-verifying** proof — either an invalid proof committing to the submitted
//!   root, or a *valid* proof committing to a **different** root, or a structurally
//!   invalid placeholder — the batch must be rejected with
//!   [`GmcError::ProofVerificationFailed`] and the confirmed root (and version) must be
//!   left exactly as before.
//!
//! The test models the expected confirmed root independently of the implementation
//! (driven by each step's *intent*, not by re-using the production `verifies` logic),
//! then asserts the implementation matches it after every submission.

use gmc_core::error::GmcError;
use gmc_core::l1_settlement::{BatchProof, L1Settlement, StateRoot};
use proptest::prelude::*;

/// The four kinds of batch proof a step can submit. Only [`ProofKind::Verifying`] is
/// expected to advance the confirmed state root; the other three must be rejected.
#[derive(Debug, Clone, Copy)]
enum ProofKind {
    /// Valid proof committing to exactly the submitted batch root → verifies.
    Verifying,
    /// Invalid proof committing to the submitted batch root → fails verification.
    InvalidForRoot,
    /// Valid proof committing to a *different* root than the one submitted → fails
    /// verification (a valid proof for another root must not authorise this batch).
    ValidForOtherRoot,
    /// Structurally invalid placeholder proof → fails verification.
    Invalid,
}

/// Builds the 32-byte model batch root for the given seed (a distinct seed yields a
/// distinct root, so `other_root` below is always `!= batch_root`).
fn root_from(seed: u8) -> StateRoot {
    StateRoot::from_bytes([seed; 32])
}

/// A single step in the submitted batch-proof sequence: the batch root being settled
/// and which kind of proof accompanies it.
fn step_strategy() -> impl Strategy<Value = (u8, ProofKind)> {
    (
        any::<u8>(),
        prop_oneof![
            Just(ProofKind::Verifying),
            Just(ProofKind::InvalidForRoot),
            Just(ProofKind::ValidForOtherRoot),
            Just(ProofKind::Invalid),
        ],
    )
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 30: 证明验证失败保留前一状态根
    #[test]
    fn property_30_failed_proof_retains_previous_state_root(
        steps in proptest::collection::vec(step_strategy(), 0..40usize),
    ) {
        let mut l1 = L1Settlement::new();

        // Independent model of the confirmed state root + settlement version. These are
        // updated *only* on a step whose proof is intended to verify, mirroring the
        // Property 30 invariant we are checking the implementation against.
        let mut expected_root = StateRoot::GENESIS;
        let mut expected_version: u64 = 0;
        prop_assert_eq!(l1.state_root(), expected_root);
        prop_assert_eq!(l1.version(), expected_version);

        for (seed, kind) in steps {
            let batch_root = root_from(seed);
            // Distinct seed ⇒ distinct root, used by the `ValidForOtherRoot` case.
            let other_root = root_from(seed.wrapping_add(1));
            prop_assert_ne!(batch_root, other_root);

            // The confirmed root *before* this submission. Used to assert retention on
            // rejection.
            let root_before = l1.state_root();
            let version_before = l1.version();
            prop_assert_eq!(root_before, expected_root);
            prop_assert_eq!(version_before, expected_version);

            // Build the proof and the ground-truth expectation for this step.
            let (proof, should_verify) = match kind {
                ProofKind::Verifying => (BatchProof::valid_for(batch_root), true),
                ProofKind::InvalidForRoot => (BatchProof::invalid_for(batch_root), false),
                ProofKind::ValidForOtherRoot => (BatchProof::valid_for(other_root), false),
                ProofKind::Invalid => (BatchProof::invalid(), false),
            };

            let result = l1.submit_batch_proof(batch_root, proof);

            if should_verify {
                // Verification passed ⇒ confirmed root updates to *this batch's* root.
                let returned = result.expect("a verifying proof is accepted");
                prop_assert_eq!(returned, batch_root);
                prop_assert_eq!(l1.state_root(), batch_root);
                // A settled update advances the version exactly once.
                prop_assert_eq!(l1.version(), version_before + 1);

                expected_root = batch_root;
                expected_version = version_before + 1;
            } else {
                // Verification failed ⇒ batch rejected, previous confirmed root retained.
                prop_assert_eq!(result, Err(GmcError::ProofVerificationFailed));
                // The confirmed state root is left exactly as it was before the batch.
                prop_assert_eq!(l1.state_root(), root_before);
                // ... and the settlement version is unchanged (no state transition).
                prop_assert_eq!(l1.version(), version_before);

                // Model is unchanged on a rejected batch.
                prop_assert_eq!(expected_root, root_before);
                prop_assert_eq!(expected_version, version_before);
            }

            // After every step the implementation matches the independent model: the
            // confirmed root is whatever the last *verifying* batch settled (or genesis
            // if none has), never a value introduced by a rejected batch.
            prop_assert_eq!(l1.state_root(), expected_root);
            prop_assert_eq!(l1.version(), expected_version);
        }
    }
}
