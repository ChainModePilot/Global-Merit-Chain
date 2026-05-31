//! Property 27 — 碳积分凭证至多转化一次 (a carbon-credit voucher is converted at most once).
//!
//! This is the dedicated property-based test for **Property 27** of the
//! `gmc-core-protocol` design's *Correctness Properties* section (task 16.3).
//!
//! > **Property 27: 碳积分凭证至多转化一次** — 对任意 碳积分凭证及针对该凭证的任意申报
//! > 请求序列，至多有一次申报成功铸造 MeriToken；一旦凭证被标记为"已转化"，其后所有针对
//! > 该凭证的申报都被拒绝（返回重复转化错误），不铸造且不消耗任何配额。
//!
//! **Validates: Requirements 12.6, 12.7**
//!
//! Per the harness convention (`tests/common/mod.rs`), the single proptest below is
//! labelled `Feature: gmc-core-protocol, Property 27: ...` and runs with `>= 100`
//! random iterations. The input is a [`generators::carbon_declaration_sequence`] —
//! `1..=N` declarations that **all** target the *same* voucher id, with a per-
//! declaration `valid_evidence` flag so mixed (valid / invalid-evidence) sequences are
//! exercised against one shared voucher.
//!
//! Each declaration is driven through the carbon scenario's two gates against the
//! single shared [`CarbonCreditVoucher`]:
//!
//! 1. **Evidence gate (Requirement 12.4).** A declaration whose voucher reference is
//!    not replayable is rejected at import with [`GmcError::EvidenceInvalid`], so it
//!    never reaches the mint/convert step — it neither mints nor consumes quota.
//! 2. **At-most-once conversion guard (Requirements 12.6 / 12.7).** A valid declaration
//!    attempts [`CarbonCreditVoucher::convert`] on the shared voucher. The *first*
//!    success marks the voucher `converted` and consumes quota exactly once; every
//!    later attempt on the now-converted voucher returns [`GmcError::DoubleConversion`]
//!    and mints nothing, consumes no quota, and mutates neither the voucher nor the
//!    ledger.
//!
//! Quota is configured far above the total possible mint so it never blocks the first
//! conversion: Property 27 isolates the at-most-once conversion guard (the quota-
//! counting side is Property 28's concern). The invariant asserted across the whole
//! sequence is that **at most one** declaration ever mints, and once converted the
//! voucher stays converted forever.

mod common;

use common::generators;

use gmc_core::carbon::CarbonCreditVoucher;
use gmc_core::error::GmcError;
use gmc_core::quota::{QuotaConfig, QuotaLedger, RefreshPeriod, TimeUnit};
use gmc_core::retroactive::{EvidenceRef, RetroactiveReviewModule};
use gmc_core::types::{ChainId, Decimal, Timestamp};
use proptest::prelude::*;

proptest! {
    // Run the numbered property with >= 100 random iterations.
    #![proptest_config(ProptestConfig::with_cases(256))]

    // Feature: gmc-core-protocol, Property 27: 碳积分凭证至多转化一次
    #[test]
    fn property_27_voucher_single_conversion(
        // 1..=N declarations, all referencing the SAME voucher id (generator invariant).
        declarations in generators::carbon_declaration_sequence(24),
    ) {
        // The generator always yields at least one declaration, all sharing one id.
        let voucher_id = declarations[0].voucher_id.clone();

        // The single shared voucher whose conversion state must persist across the
        // entire declaration sequence (Requirements 12.6 / 12.7).
        let mut voucher = CarbonCreditVoucher::new(
            voucher_id.clone(),
            EvidenceRef::new(format!("ipfs://{voucher_id}"), "0xvoucherhash", true),
        );

        // A generous environmental-protection chain quota so quota never blocks the
        // first conversion — Property 27 isolates the at-most-once conversion guard,
        // while Property 28 covers the quota-counting side. The cap (1,000,000) dwarfs
        // the most that can ever be minted here (a single 10-unit conversion).
        let cfg = QuotaConfig::new(
            Decimal::from_int(1_000_000),
            RefreshPeriod::Periodic { unit: TimeUnit::Day, value: Decimal::ONE },
        )
        .expect("a positive quota + valid periodic period is a valid config");
        let mut ledger =
            QuotaLedger::new(ChainId::new("carbon-reduction"), Timestamp::from_secs(0));

        // A fixed, strictly-positive mint amount charged on an approved conversion.
        let mint_amount = Decimal::from_int(10);

        // Routes each declaration's import through the real evidence gate (an invalid
        // reference -> EvidenceInvalid, never reaching the mint/convert step).
        let mut module = RetroactiveReviewModule::new();

        // A freshly constructed voucher is unconverted (R12.6/12.7 initial state).
        prop_assert!(!voucher.is_converted());
        prop_assert_eq!(voucher.converted_declaration_id(), None);

        // Counts declarations that successfully mint — must never exceed 1.
        let mut success_count = 0usize;

        for (i, decl) in declarations.iter().enumerate() {
            // The whole sequence targets the same voucher id.
            prop_assert_eq!(&decl.voucher_id, &voucher_id);

            // Snapshot the pre-attempt state so we can prove "no side effects" on the
            // rejection paths.
            let minted_before = ledger.minted_this_period();
            let converted_before = voucher.is_converted();
            let converted_id_before =
                voucher.converted_declaration_id().map(|s| s.to_owned());

            // 1) Evidence gate (Requirement 12.4): import this declaration's voucher
            //    reference. A non-replayable reference is rejected with EvidenceInvalid.
            let evidence = EvidenceRef::new(
                format!("ipfs://{voucher_id}"),
                "0xvoucherhash",
                decl.valid_evidence,
            );
            let declaration_voucher = CarbonCreditVoucher::new(voucher_id.clone(), evidence);
            let import = declaration_voucher.import_to_retroactive_flow(
                &mut module,
                decl.contributor_id.clone(),
                ChainId::new("carbon-reduction"),
                format!("carbon declaration #{i} for {voucher_id}"),
                Timestamp::from_secs(1_650_000_000 + i as u64),
            );

            if !decl.valid_evidence {
                // Rejected upstream: never reaches the mint/convert step, so it mints
                // nothing, consumes no quota, and leaves the shared voucher untouched.
                prop_assert_eq!(import, Err(GmcError::EvidenceInvalid));
                prop_assert_eq!(ledger.minted_this_period(), minted_before);
                prop_assert_eq!(voucher.is_converted(), converted_before);
                prop_assert_eq!(
                    voucher.converted_declaration_id().map(|s| s.to_owned()),
                    converted_id_before
                );
                continue;
            }

            // Valid evidence: a Pending declaration is created and we proceed to the
            // mint/convert step on the SAME shared voucher.
            let decl_id = import.expect("a replayable reference imports into the flow");

            // 2) At-most-once conversion guard (Requirements 12.6 / 12.7).
            let result = voucher.convert(decl_id.to_string(), mint_amount, &cfg, &mut ledger);

            if converted_before {
                // The voucher was already converted by an earlier declaration: this one
                // is rejected with DoubleConversion and changes nothing (R12.6).
                prop_assert_eq!(result, Err(GmcError::DoubleConversion));
                prop_assert_eq!(ledger.minted_this_period(), minted_before);
                prop_assert!(voucher.is_converted());
                prop_assert_eq!(
                    voucher.converted_declaration_id().map(|s| s.to_owned()),
                    converted_id_before
                );
            } else {
                // The first (and only possible) successful conversion of this voucher:
                // quota is generous, so the sole gate is the conversion guard (R12.7).
                prop_assert_eq!(result, Ok(()));
                success_count += 1;

                // The voucher is now permanently converted, bound to this declaration.
                prop_assert!(voucher.is_converted());
                prop_assert_eq!(voucher.converted_declaration_id(), Some(decl_id.as_str()));

                // The mint amount was charged to this period exactly once.
                prop_assert_eq!(
                    ledger.minted_this_period(),
                    minted_before
                        .checked_add(mint_amount)
                        .expect("running total stays within range")
                );
            }

            // INVARIANT: at no point can more than one declaration have minted.
            prop_assert!(success_count <= 1);
        }

        // Across the whole sequence at most one declaration ever minted (Property 27),
        // and the voucher is converted iff exactly one conversion succeeded.
        prop_assert!(success_count <= 1);
        prop_assert_eq!(voucher.is_converted(), success_count == 1);
    }
}
