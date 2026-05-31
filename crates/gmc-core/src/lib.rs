//! # gmc-core — GMC (Global Merit Chain) pure-logic core
//!
//! `gmc-core` is the deterministic, runtime-agnostic logic core of the GMC protocol.
//! It contains no chain-runtime dependencies so the very same logic can be reused by
//! both a future **Substrate L1 pallet** and a **ZK Rollup L2** implementation. This
//! crate is where the deterministic core logic (derivation-tree invariants, scoring
//! math, quota accounting, the MeriToken decay model, voter selection, etc.) lives,
//! and it is the target of the property-based tests (Property 1–30).
//!
//! ## Tech-stack decision (recorded per task 1.1, _Requirements: 5.2_)
//!
//! **Default direction (chosen):** Rust within the **Substrate ecosystem** as the
//! **L1 settlement** layer, paired with a **ZK Rollup L2** for high-frequency
//! processing, and **[`proptest`]** as the property-based testing framework. This
//! matches the blueprint's recommended `Substrate L1 + ZK Rollup L2` architecture
//! and is the baseline evaluated in the design's "technology selection" section.
//!
//! **Open alternative (kept on the table):** a **TypeScript reference model** paired
//! with **fast-check** for property-based testing. If the project ever pivots to the
//! TypeScript reference model, the property mapping and task structure stay the same;
//! only the test framework changes (`proptest` → `fast-check`).
//!
//! [`proptest`]: https://docs.rs/proptest
//!
//! ## Module layout
//!
//! Each protocol module from the design document has a dedicated home below. They are
//! empty placeholders at this stage (task 1.1, scaffolding only); shared types and
//! error codes (task 1.2) and the property-test generator skeleton (task 1.3) are
//! filled in by later tasks.

// --- Shared primitives & error vocabulary (task 1.2) ---

/// Shared primitive types: fixed-point `Decimal`/`Ratio`, `ChainId`/`FayID`/`Timestamp`,
/// and `Dimension`/`DimensionWeights`.
pub mod types;

/// Unified, machine-identifiable error codes (`GmcError`) returned across all modules.
pub mod error;

// --- Protocol module skeleton (placeholders to be implemented by later tasks) ---

/// `Chain_Registry` — derivation tree, lifecycle, and `(parentId, domain)` uniqueness.
pub mod registry;

/// `Evaluation_Mechanism` — per-chain evaluation config and change governance.
pub mod mechanism;

/// Quota & refresh-period accounting (per-chain isolation, one-time vs periodic).
pub mod quota;

/// `Scoring_Engine` — three-dimensional classification, inflation index, weighted sum.
pub mod scoring;

/// MeritPocket / Merit batch model: exponential decay and `minMerit` floor value.
pub mod merit;

/// `Minting_Service` — minting pipeline, quota metering, `minMerit` updates.
pub mod minting;

/// `Registration_Service` — merit registration and grant-trigger guards.
pub mod registration;

/// `Recording_Service` — contribution recording and registration matching.
pub mod recording;

/// `Retroactive_Review_Module` — retroactive declaration intake, evidence checks, voting.
pub mod retroactive;

/// `AntiFraud_Engine` — high-intimacy exclusion, sampling, anomaly detection, clawback.
pub mod antifraud;

/// `Governance_Module` — weighted voting, threshold decisions, proposal handling.
pub mod governance;

/// Carbon-credit → MeriToken application scenario (voucher state, single conversion).
pub mod carbon;

/// `GMC_Base` — root node (depth = 0) and the monetary-exchange rejection choke point.
pub mod gmc_base;

/// `L1_Settlement` — pure-logic model of the Substrate L1 layer: stores registration
/// records, identities, governance vote results, penalties and the state root, and is
/// the concrete home for the modules' L1 anchoring seams (fee-free; GRANDPA/BABE).
pub mod l1_settlement;

/// `L2_Rollup` — pure-logic model of the ZK Rollup L2 layer: high-frequency per-record
/// processing (Req 13.2), the 1,000-record / 60-second batch-proof trigger (Req 13.3),
/// the `submitRollupBatch` L2→L1 seam (Req 9.7), and BFT consensus markers (Req 13.7).
pub mod l2_rollup;

/// Nested merit-chain creation channels (`ChainCreationService`): the three Requirement 2
/// initiation paths — vote-initiated (Req 2.1), steward-initiated (Req 2.2/2.7), and
/// institution-applied (Req 2.3/2.8) — each recording the matching `originType` and
/// surfacing `MissingField` for a missing parent/domain (Req 2.5).
pub mod chain_creation;

/// End-to-end flow wiring (task 20.1): composes the protocol modules into the design's
/// four key flows — chain derivation, register→record→grant, retroactive review/voting,
/// and carbon→MeriToken — joining the cross-module trait seams (`RegistrationLookup`,
/// `GrantContext`, `L1ProofSink`) so no module is left isolated/un-integrated.
pub mod flows;
