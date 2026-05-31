//! Property 21 — 事后申报受理证据校验 (retroactive-declaration intake & evidence check).
//!
//! Dedicated property-based test for **Property 21** of the `gmc-core-protocol`
//! design's *Correctness Properties* section (task 15.3).
//!
//! > **Property 21: 事后申报受理证据校验** — 对任意事后申报，当且仅当其包含贡献者标识、
//! > 所属功勋链标识、已发生贡献描述、贡献发生时间，且至少附带一条可被审核者独立访问与
//! > 核验（可复盘）的证据引用时被受理并标记为"待审核"；否则被拒绝且不推入投票流程。
//!
//! **Validates: Requirements 10.1, 10.2, 10.8**
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 21: ...` and runs with `>= 100`
//! random iterations.
//!
//! The generator independently toggles each required field's presence
//! (`contributorId`, `chainId`, `description` — `occurredAt` is value-typed via
//! [`Timestamp`] and therefore always present) and builds a list of evidence
//! references that may or may not be independently *replayable* (a reference counts
//! only when it is flagged replayable **and** carries a non-empty locator **and** a
//! non-empty hash, per [`EvidenceRef::is_replayable`]). The test then asserts the
//! iff: a declaration is accepted as [`ReviewStatus::Pending`] exactly when all
//! required fields are present **and** at least one evidence reference is replayable;
//! otherwise [`RetroactiveReviewModule::submit`] rejects it (with
//! [`GmcError::EvidenceInvalid`] when only the evidence is lacking, or
//! [`GmcError::FieldValidation`] when a required field is missing) and stores nothing
//! — so the rejected declaration is never pushed into the voting flow.

use gmc_core::error::GmcError;
use gmc_core::retroactive::{
    EvidenceRef, RetroactiveApplication, RetroactiveReviewModule, ReviewStatus,
};
use gmc_core::types::{ChainId, FayID, Timestamp};
use proptest::prelude::*;

/// A toggled-presence spec for one evidence reference: `(replayable, uri_present,
/// hash_present)`. A reference is independently verifiable (replayable) iff all three
/// flags are set, mirroring [`EvidenceRef::is_replayable`].
type EvidenceSpec = (bool, bool, bool);

/// Builds an [`EvidenceRef`] from a presence spec, choosing concrete non-empty values
/// for present locator/hash and empty strings otherwise.
fn build_evidence(spec: &EvidenceSpec) -> EvidenceRef {
    let (replayable, uri_present, hash_present) = *spec;
    let uri = if uri_present { "ipfs://cid-abc" } else { "" };
    let hash = if hash_present { "0xhash" } else { "" };
    EvidenceRef::new(uri, hash, replayable)
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 21: 事后申报受理证据校验
    #[test]
    fn property_21_retro_evidence(
        // Presence of each required, string-backed field (occurredAt is value-typed
        // and therefore always present).
        contributor_present in any::<bool>(),
        chain_present in any::<bool>(),
        description_present in any::<bool>(),
        // The contribution occurrence time is always present; vary its value.
        occurred_secs in 1u64..=2_000_000_000u64,
        // 0..6 evidence references, each with independently toggled
        // replayable / locator-present / hash-present flags.
        evidence_specs in proptest::collection::vec(
            (any::<bool>(), any::<bool>(), any::<bool>()),
            0..6usize,
        ),
    ) {
        // --- Build the declaration from the toggled presence flags. ---
        let contributor_id = FayID::new(if contributor_present { "alice" } else { "" });
        let chain_id = ChainId::new(if chain_present { "carbon-reduction" } else { "" });
        let description = if description_present {
            "Planted 1,000 trees in 2023, verified by local registry."
        } else {
            ""
        };
        let occurred_at = Timestamp::from_secs(occurred_secs);
        let evidence_refs: Vec<EvidenceRef> = evidence_specs.iter().map(build_evidence).collect();

        let application = RetroactiveApplication::new(
            contributor_id,
            chain_id,
            description,
            occurred_at,
            evidence_refs,
        );

        // --- Compute the expected outcome straight from the property definition. ---
        // All required fields present (occurredAt always present as a value type).
        let fields_present = contributor_present && chain_present && description_present;
        // At least one evidence reference is independently accessible AND verifiable.
        let has_replayable_evidence = evidence_specs
            .iter()
            .any(|(replayable, uri_present, hash_present)| {
                *replayable && *uri_present && *hash_present
            });
        // Acceptance holds iff both halves of the conjunction hold.
        let expected_accept = fields_present && has_replayable_evidence;

        // --- Submit and assert the iff. ---
        let mut module = RetroactiveReviewModule::new();
        match module.submit(application) {
            Ok(id) => {
                // Accepted only when every condition is met.
                prop_assert!(expected_accept);

                // ... and the accepted declaration is marked "待审核" (Pending) ...
                let declaration = module.get(&id).expect("an accepted declaration is stored");
                prop_assert_eq!(declaration.review_status(), ReviewStatus::Pending);

                // ... awaiting (not yet pushed into) the voting flow: no vote handle yet.
                prop_assert_eq!(declaration.vote_id(), None);

                // Exactly one record was created by this single submission.
                prop_assert_eq!(module.len(), 1);
            }
            Err(err) => {
                // Rejected exactly when some condition fails.
                prop_assert!(!expected_accept);

                // Rejection is side-effect free: nothing is stored, so the rejected
                // declaration is never pushed into the voting flow.
                prop_assert!(module.is_empty());

                // The error code reflects the up-front validation order: required
                // fields first (FieldValidation), then evidence replayability
                // (EvidenceInvalid) once all fields are present.
                if fields_present {
                    // Fields all present, so the only possible failure is the evidence
                    // check (Requirements 10.2 / 10.8).
                    prop_assert_eq!(err, GmcError::EvidenceInvalid);
                } else {
                    // A required field was missing (Requirement 10.1).
                    prop_assert_eq!(err, GmcError::FieldValidation);
                }
            }
        }
    }
}
