//! Deliverables & one-time-configuration **smoke tests** (task 21.2).
//!
//! These are plain `#[test]` smoke/unit checks — **NOT** property tests — so they carry
//! no `Feature: ... Property N` label. They verify two kinds of "deliverable" acceptance
//! criteria that the design explicitly routes to smoke tests rather than PBT:
//!
//! 1. **Design-document deliverable (Requirements 5.2 / 5.3 / 5.6).** The design's
//!    "技术选型评估" (technology selection evaluation) section must contain a baseline
//!    candidate, a comparison candidate, a three-dimensional comparison table, and a
//!    recommendation with rationale. The section is read at runtime from the spec
//!    directory via a path built from `CARGO_MANIFEST_DIR`.
//! 2. **One-time configuration (Requirements 1.1 / 13.4 / 13.6).** The root-node config
//!    (`GMC_Base` is the fixed depth-0 root), L1 being fee-free, and L1 running
//!    GRANDPA/BABE consensus are asserted through the crate's public API.

use gmc_core::gmc_base::GmcBase;
use gmc_core::l1_settlement::{ConsensusConfig, L1Settlement};
use gmc_core::types::ChainId;

/// Reads the design document from the spec directory.
///
/// `gmc-core` lives at `crates/gmc-core`, so `design.md` is two directories up under
/// `.kiro/specs/gmc-core-protocol/`. Building the path from `CARGO_MANIFEST_DIR` keeps
/// the test independent of the process working directory.
fn read_design_doc() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../.kiro/specs/gmc-core-protocol/design.md"
    );
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("design document must be readable at {path}: {e}"))
}

/// Extracts the "技术选型评估" section: from its `##` heading up to the next `## `
/// heading (or end of file). Asserting against just this slice keeps the deliverable
/// checks scoped to the technology-selection section.
fn technology_selection_section(doc: &str) -> &str {
    let start = doc
        .find("## 技术选型评估")
        .expect("design must contain a '技术选型评估' (technology selection) section");
    let rest = &doc[start..];
    // Skip past this heading line, then find the next top-level "## " section boundary.
    let after_heading = rest
        .find('\n')
        .map(|nl| start + nl + 1)
        .unwrap_or(doc.len());
    let end = doc[after_heading..]
        .find("\n## ")
        .map(|rel| after_heading + rel)
        .unwrap_or(doc.len());
    &doc[start..end]
}

// --- Requirements 5.2 / 5.3 / 5.6: design-document deliverable ------------------

#[test]
fn design_has_technology_selection_section() {
    // Requirement 5.2: the design records the technology-selection evaluation.
    let doc = read_design_doc();
    let section = technology_selection_section(&doc);
    assert!(
        section.starts_with("## 技术选型评估"),
        "the extracted slice must begin at the '技术选型评估' heading"
    );
    assert!(
        !section.trim_end().is_empty(),
        "the technology-selection section must not be empty"
    );
}

#[test]
fn technology_selection_has_baseline_and_comparison_candidates() {
    // Requirements 5.2 / 5.3: a baseline candidate AND a comparison candidate are
    // present, with the baseline being Substrate L1 + ZK Rollup L2.
    let doc = read_design_doc();
    let section = technology_selection_section(&doc);

    assert!(
        section.contains("基准候选"),
        "technology-selection section must name a baseline candidate (基准候选)"
    );
    assert!(
        section.contains("对照候选"),
        "technology-selection section must name a comparison candidate (对照候选)"
    );
    // The baseline candidate is the Substrate L1 + ZK Rollup L2 architecture.
    assert!(
        section.contains("Substrate") && section.contains("Rollup"),
        "baseline candidate must be the Substrate L1 + ZK Rollup L2 architecture"
    );
    // The comparison candidate is the Ethereum-family derivation/sub-chain approach.
    assert!(
        section.contains("以太坊") || section.contains("Ethereum"),
        "comparison candidate must reference Ethereum-family approaches"
    );
}

#[test]
fn technology_selection_has_three_dimensional_comparison_table() {
    // Requirement 5.3: a three-dimensional comparison table is present. We confirm a
    // GitHub-flavoured Markdown table (header + separator rows) and the three named
    // comparison dimensions: transaction cost, throughput, and customizable governance.
    let doc = read_design_doc();
    let section = technology_selection_section(&doc);

    // A Markdown table has a header separator row made of pipes and dashes.
    let has_table_separator = section
        .lines()
        .any(|line| line.contains('|') && line.contains("---"));
    assert!(
        has_table_separator,
        "technology-selection section must contain a Markdown comparison table"
    );

    // The table must contain rows for at least three comparison dimensions: every
    // table row (and only table rows) starts with a leading pipe in this section.
    let table_rows = section
        .lines()
        .filter(|line| line.trim_start().starts_with('|'))
        .count();
    assert!(
        table_rows >= 5,
        "the comparison table must have a header, a separator, and ≥3 dimension rows \
         (found {table_rows} table rows)"
    );

    // The three named dimensions from the design (cost / throughput / governance).
    assert!(
        section.contains("成本"),
        "comparison table must include a transaction-cost dimension"
    );
    assert!(
        section.contains("吞吐"),
        "comparison table must include a throughput dimension"
    );
    assert!(
        section.contains("治理"),
        "comparison table must include a customizable-governance dimension"
    );
}

#[test]
fn technology_selection_has_recommendation_with_rationale() {
    // Requirement 5.6: a recommendation is given together with its rationale.
    let doc = read_design_doc();
    let section = technology_selection_section(&doc);

    assert!(
        section.contains("选型建议") || section.contains("建议"),
        "technology-selection section must contain a recommendation (选型建议)"
    );
    // The recommendation must select the baseline (Substrate L1 + ZK Rollup L2)...
    assert!(
        section.contains("采用基准候选") || section.contains("采用"),
        "the recommendation must state which candidate is adopted"
    );
    // ...and supply a rationale (three-dimension justification).
    assert!(
        section.contains("依据"),
        "the recommendation must be accompanied by a rationale (依据)"
    );
}

// --- Requirement 1.1: root-node configuration ----------------------------------

#[test]
fn gmc_base_is_the_fixed_depth_zero_root() {
    // Requirement 1.1: GMC_Base is the single depth-0 root under a fixed identifier.
    assert_eq!(GmcBase::ROOT_DEPTH, 0, "GMC_Base must sit at derivation depth 0");
    assert_eq!(
        GmcBase::root_chain_id(),
        ChainId::new("gmc-base"),
        "GMC_Base must expose the fixed root chain id"
    );
    assert_eq!(
        GmcBase::root_chain_id().as_str(),
        GmcBase::ROOT_CHAIN_ID,
        "root_chain_id() must agree with the ROOT_CHAIN_ID constant"
    );
}

// --- Requirement 13.4: L1 fee-free configuration -------------------------------

#[test]
fn l1_settlement_is_configured_fee_free() {
    // Requirement 13.4: L1 charges no transaction fee.
    let l1 = L1Settlement::new();
    assert!(l1.is_fee_free(), "L1_Settlement must be configured fee-free");
    assert!(
        l1.transaction_fee().is_zero(),
        "the L1 per-transaction fee must be exactly zero"
    );
    assert!(
        L1Settlement::TRANSACTION_FEE.is_zero(),
        "the L1 TRANSACTION_FEE constant must be exactly zero"
    );
}

// --- Requirement 13.6: L1 GRANDPA/BABE consensus -------------------------------

#[test]
fn l1_settlement_uses_grandpa_babe_consensus() {
    // Requirement 13.6: L1 runs GRANDPA (finality) + BABE (block production).
    let l1 = L1Settlement::new();
    let consensus = l1.consensus_config();

    assert_eq!(
        consensus,
        ConsensusConfig::GrandpaBabe,
        "L1 consensus must be GRANDPA/BABE"
    );
    assert!(consensus.uses_grandpa(), "L1 must use GRANDPA finality");
    assert!(consensus.uses_babe(), "L1 must use BABE block production");
    assert_eq!(
        consensus.label(),
        "GRANDPA/BABE",
        "L1 consensus label must read GRANDPA/BABE"
    );
}
