//! Property 19 — 贡献记录须匹配有效登记
//! (a contribution record requires a matching valid registration).
//!
//! This is the dedicated property-based test for **Property 19** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 11.5).
//!
//! > **Property 19: 贡献记录须匹配有效登记** — For any registration table and
//! > contribution-record request, when **not** flowing through the
//! > retroactive-declaration path, a contribution record is created **iff** there
//! > exists a matching registration (same `contributorId`, same `chainId`, status
//! > `Valid`); otherwise the request is rejected and returns a "not registered" error.
//!
//! **Validates: Requirements 9.3, 9.4**
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 19: ...` and runs with `>= 100`
//! random iterations.
//!
//! ## How the property is driven against the *real* logic
//!
//! The registration table is the **real** [`RegistrationService`]: every registered
//! pair therefore has status `Valid` by construction (the only status `register`
//! produces). A small local [`RegistrationLookup`] adapter delegates to the service's
//! real [`RegistrationService::find_valid_registration`] matching — exactly what the
//! integration layer (task 20.1) does — so the recording side exercises the genuine
//! `(contributorId, chainId, status == Valid)` match rule.
//!
//! Inputs are built directly here over small contributor/chain pools so that record
//! requests frequently both *do* and *do not* match a registered pair. The
//! independent oracle for "a matching valid registration exists" is simply "this
//! `(contributor, chain)` pair was registered" — computed from the model set, not
//! from the function under test — so the `iff` is a real check, not a tautology.

use std::collections::BTreeSet;

use gmc_core::error::GmcError;
use gmc_core::recording::{
    ContributionRequest, EvaluationStatus, EvidenceRef, RecordingService, RegistrationLookup,
};
use gmc_core::registration::{RegistrationApplication, RegistrationService};
use gmc_core::types::{ChainId, FayID, Timestamp};
use proptest::prelude::*;

/// Number of distinct contributors in the generated pool.
const CONTRIBUTORS: usize = 4;
/// Number of distinct chains in the generated pool.
const CHAINS: usize = 3;

/// Stable contributor id for pool index `c`.
fn contributor(c: usize) -> FayID {
    FayID::new(format!("fay-{c}"))
}

/// Stable chain id for pool index `ch`.
fn chain(ch: usize) -> ChainId {
    ChainId::new(format!("chain-{ch}"))
}

/// Local adapter implementing the recording side's [`RegistrationLookup`] seam over
/// the real [`RegistrationService`] (delegating to its `find_valid_registration`).
///
/// This mirrors the production integration (task 20.1): a "matching valid
/// registration" is whatever the registration service reports as `Valid` for the
/// `(contributorId, chainId)` pair, and its id is what the new record links to.
struct ServiceLookup<'a> {
    service: &'a RegistrationService,
}

impl RegistrationLookup for ServiceLookup<'_> {
    fn find_valid_registration(
        &self,
        contributor_id: &FayID,
        chain_id: &ChainId,
    ) -> Option<String> {
        self.service
            .find_valid_registration(contributor_id, chain_id)
            .map(|reg| reg.id().as_str().to_owned())
    }
}

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(100))]

    // Feature: gmc-core-protocol, Property 19: 贡献记录须匹配有效登记
    #[test]
    fn property_19_record_needs_registration(
        // The registration table: each (contributor, chain) pair is registered (and
        // so becomes a `Valid` registration). Some pairs may repeat; that is fine —
        // the matching rule keys on the pair, not on multiplicity.
        registered_pairs in proptest::collection::vec(
            (0usize..CONTRIBUTORS, 0usize..CHAINS),
            0..(CONTRIBUTORS * CHAINS),
        ),
        // The contribution-record requests to submit (non-retroactive). Drawn from the
        // same pools so requests frequently both match and miss the registered set.
        request_pairs in proptest::collection::vec(
            (0usize..CONTRIBUTORS, 0usize..CHAINS),
            1..24usize,
        ),
    ) {
        // --- Build the real registration table (every entry is `Valid`). ---
        let mut registrations = RegistrationService::new();
        // Independent oracle: the exact set of `(contributor, chain)` pairs that have a
        // matching *valid* registration. Computed from the model, not the SUT.
        let mut valid_set: BTreeSet<(usize, usize)> = BTreeSet::new();

        for (c, ch) in &registered_pairs {
            registrations
                .register(RegistrationApplication::new(
                    contributor(*c),
                    chain(*ch),
                    "intended contribution",
                    Timestamp::from_secs(1_000),
                ))
                .expect("a complete, in-bounds application must be accepted");
            valid_set.insert((*c, *ch));
        }

        let lookup = ServiceLookup { service: &registrations };

        // `service` accumulates the standard (non-retroactive) records; `retro_service`
        // is used only to contrast the "未走事后申报流程时" precondition of the property.
        let mut service = RecordingService::new();
        let mut retro_service = RecordingService::new();
        // Expected number of stored records: one per accepted (matching) request.
        let mut expected_len = 0usize;

        for (c, ch) in request_pairs {
            let contributor_id = contributor(c);
            let chain_id = chain(ch);
            // Oracle: does a matching *valid* registration exist for this pair?
            let has_match = valid_set.contains(&(c, ch));

            let req = ContributionRequest::new(
                contributor_id.clone(),
                chain_id.clone(),
                vec![EvidenceRef::new("ipfs://cid", "0xhash")],
                Timestamp::from_secs(2_000),
            );

            let before = service.len();
            // The property is about the **non-retroactive** path (`is_retroactive = false`).
            let result = service.record(req.clone(), &lookup, false);

            // (Req 9.3 / 9.4) Created IFF a matching valid registration exists.
            prop_assert_eq!(
                result.is_ok(),
                has_match,
                "record created iff a matching valid registration exists"
            );

            match result {
                Ok(id) => {
                    expected_len += 1;
                    // (Req 9.3) The new record is stored, linked to the registration,
                    // starts Pending, and carries the requested contributor/chain.
                    let record = service.get(&id).expect("an accepted record is stored");
                    prop_assert!(
                        record.is_linked(),
                        "a standard-flow record links to its valid registration"
                    );
                    prop_assert!(record.registration_id().is_some());
                    prop_assert_eq!(record.evaluation_status(), EvaluationStatus::Pending);
                    prop_assert_eq!(record.contributor_id(), &contributor_id);
                    prop_assert_eq!(record.chain_id(), &chain_id);
                    // Exactly one record was written (atomic create).
                    prop_assert_eq!(service.len(), before + 1);

                    // The linked id is a real `Valid` registration for this pair.
                    let linked = registrations
                        .find_valid_registration(&contributor_id, &chain_id)
                        .expect("a match must back an accepted record");
                    prop_assert_eq!(record.registration_id(), Some(linked.id().as_str()));
                }
                Err(err) => {
                    // (Req 9.4) No match + not retroactive → rejected with NotRegistered,
                    // and nothing is written (the store is left unchanged).
                    prop_assert_eq!(err, GmcError::NotRegistered);
                    prop_assert_eq!(service.len(), before, "a rejected record writes nothing");

                    // The property's precondition is "未走事后申报流程时" — confirm that the
                    // *same* unmatched request IS accepted on the retroactive path, as an
                    // unlinked record. This delimits exactly where the `iff` applies.
                    let retro_id = retro_service
                        .record(req, &lookup, true)
                        .expect("the retroactive path accepts a record without a registration");
                    let retro_record = retro_service
                        .get(&retro_id)
                        .expect("the retroactive record is stored");
                    prop_assert!(
                        !retro_record.is_linked(),
                        "a retroactive record is not linked to a registration"
                    );
                    prop_assert_eq!(retro_record.registration_id(), None);
                }
            }

            // The standard store always holds exactly the accepted-so-far count.
            prop_assert_eq!(service.len(), expected_len);
        }
    }
}
