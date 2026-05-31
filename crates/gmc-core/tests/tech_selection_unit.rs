//! Tech-selection determination unit tests (task 21.1).
//!
//! **Validates: Requirements 5.5**
//!
//! These are plain `#[test]` unit tests (NOT property tests), so they carry no
//! `Feature: ... Property N` label.
//!
//! Requirement 5.5 states the "免费或极低成本记录" (free or extremely-low-cost
//! recording) constraint for the 技术选型评估 (technology-selection evaluation):
//!
//! > IF 某个候选技术方案对每条贡献记录向贡献者收取交易手续费，或其单条贡献记录成本
//! > 超过协议规定的单条记录成本上限，THEN THE 技术选型评估 SHALL 将该方案标记为
//! > 不满足"免费或极低成本记录"约束。
//!
//! The `gmc-core` crate exposes no dedicated tech-selection predicate, so — as the
//! task allows — the determination is modelled here directly from the crate's public
//! fee/cost markers ([`Decimal`]). The **baseline candidate** (the Substrate L1 + ZK
//! Rollup L2 architecture) is grounded in the real public fee-free API of
//! [`L1Settlement`]: its per-transaction fee is [`Decimal::ZERO`] and
//! [`L1Settlement::is_fee_free`] is `true` (Requirement 13.4), so its per-record
//! transaction fee is zero.
//!
//! The rule below is the contrapositive of Requirement 5.5: a candidate **satisfies**
//! the constraint iff it charges **no** per-record transaction fee **and** its
//! per-record cost does **not** exceed the protocol per-record cost cap.

use gmc_core::l1_settlement::L1Settlement;
use gmc_core::types::Decimal;

/// The protocol's per-record cost cap — the upper bound of "极低成本" (extremely low
/// cost). The crate exposes no dedicated constant, so this is modelled from the public
/// [`Decimal`] marker as a small positive bound for the unit determination.
fn per_record_cost_cap() -> Decimal {
    // 0.001 cost units — a deliberately tiny ceiling for "extremely-low-cost".
    Decimal::from_str("0.001").expect("well-formed decimal literal")
}

/// Models the Requirement 5.5 determination: a candidate satisfies the
/// "免费或极低成本记录" constraint **iff** it charges no per-record transaction fee
/// (`per_record_fee` is zero) **and** its `per_record_cost` does not exceed the
/// protocol per-record cost cap.
///
/// Equivalently (Req 5.5 directly): the candidate is marked **not satisfying** when it
/// charges a per-record fee (`per_record_fee` is positive) **or** its per-record cost
/// exceeds the cap.
fn satisfies_free_or_low_cost(per_record_fee: Decimal, per_record_cost: Decimal) -> bool {
    !per_record_fee.is_positive() && per_record_cost <= per_record_cost_cap()
}

/// Requirement 5.5: a candidate that charges a per-record transaction fee to the
/// contributor is marked as **not satisfying** the "免费或极低成本记录" constraint.
#[test]
fn fee_charging_candidate_marked_not_satisfying() {
    // A candidate charging 0.05 per contribution record, with an otherwise tiny cost.
    let per_record_fee = Decimal::from_str("0.05").expect("well-formed decimal literal");
    let per_record_cost = Decimal::ZERO;

    assert!(
        per_record_fee.is_positive(),
        "this candidate is defined as charging a per-record fee"
    );
    assert!(
        !satisfies_free_or_low_cost(per_record_fee, per_record_cost),
        "Req 5.5: a fee-charging candidate must NOT satisfy the free/low-cost constraint"
    );
}

/// Requirement 5.5: a fee-free candidate (within the cost cap) satisfies the
/// "免费或极低成本记录" constraint. The fee is taken from the real fee-free baseline
/// candidate ([`L1Settlement`], Requirement 13.4) so the "free option" is grounded in
/// the crate's actual public fee marker rather than a fabricated value.
#[test]
fn fee_free_baseline_candidate_satisfies() {
    let baseline = L1Settlement::new();

    // The baseline candidate's public fee markers confirm it is fee-free (Req 13.4).
    assert!(
        baseline.is_fee_free(),
        "baseline candidate (Substrate L1) is configured fee-free"
    );
    assert!(
        L1Settlement::TRANSACTION_FEE.is_zero(),
        "the baseline candidate's per-transaction fee marker is zero"
    );

    let per_record_fee = baseline.transaction_fee(); // == Decimal::ZERO
    let per_record_cost = Decimal::ZERO; // well within the per-record cost cap

    assert!(
        !per_record_fee.is_positive(),
        "a free candidate charges no per-record fee"
    );
    assert!(
        satisfies_free_or_low_cost(per_record_fee, per_record_cost),
        "Req 5.5: a fee-free candidate within the cost cap satisfies the constraint"
    );
}

/// Requirement 5.5 (cost-cap branch): even a fee-free candidate is marked as **not
/// satisfying** when its per-record cost exceeds the protocol per-record cost cap.
#[test]
fn fee_free_candidate_over_cost_cap_marked_not_satisfying() {
    let per_record_fee = Decimal::ZERO; // charges no fee...
    let over_cap = Decimal::from_str("0.01").expect("well-formed decimal literal");

    assert!(
        over_cap > per_record_cost_cap(),
        "this per-record cost is deliberately above the cap"
    );
    assert!(
        !satisfies_free_or_low_cost(per_record_fee, over_cap),
        "Req 5.5: exceeding the per-record cost cap must NOT satisfy the constraint, \
         even with a zero fee"
    );
}
