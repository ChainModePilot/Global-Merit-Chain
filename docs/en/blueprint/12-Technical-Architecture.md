# Chapter 12: Technical Architecture

## 12.1 Architecture Overview

```
┌─────────────────────────────────────────────┐
│  Application Layer (DApp, Fay Interface,     │
│  Governance UI)                              │
├─────────────────────────────────────────────┤
│  Layer 2: ZK Rollup (High-frequency         │
│  transaction processing)                     │
│  - Contribution records, intimacy updates,   │
│    daily interactions                        │
├─────────────────────────────────────────────┤
│  Layer 1: Substrate Dedicated Chain          │
│  (Settlement & Consensus)                    │
│  - State root anchoring, identity management,│
│    governance voting                         │
├─────────────────────────────────────────────┤
│  Identity Layer: DID + PKI + ZKP             │
└─────────────────────────────────────────────┘
```

## 12.2 Technology Selection Comparison

> Multiple approaches were evaluated during discussion:

| Approach | Advantages | Disadvantages | Conclusion |
|----------|-----------|---------------|------------|
| Ethereum Mainnet | Mature ecosystem, high security | High gas fees, low TPS (15–30) | Not suitable for high-frequency recording at population scale |
| Ethereum L2 | Reduced fees | Still constrained by Ethereum ecosystem | Alternative |
| DAG (IOTA/Nano) | High throughput, no fees | Weak consensus security | Insufficient security |
| **Substrate Custom Chain** | Fully customizable, no gas fees | Requires building own ecosystem | **Recommended** |

### The Gas Fee Problem

Gas fees are the computational cost per transaction on public chains like Ethereum. With the entire population generating large volumes of micro-contribution records daily, recording each one on-chain would be prohibitively expensive. GMC requires a free or extremely low-cost recording method.

### The Throughput Problem

Ethereum mainnet handles approximately 15–30 TPS. For contribution records from billions of users worldwide, this throughput is far from sufficient.

## 12.3 Substrate Dedicated Chain

### Why Substrate

1. **Fully customizable consensus**: design a consensus algorithm specifically suited for contribution recording
2. **No gas fees**: can be designed for fee-free transactions
3. **Customizable governance modules**: naturally suited for community consensus
4. **Polkadot interoperability**: can interoperate with other chains via relay chains
5. **Modular**: compose Runtime modules as needed

### Rationale

> GMC's unique requirements make general-purpose public chains unsuitable:
> - Population-wide participation = extremely high transaction volume
> - Micro-contribution records = high-frequency, low-value transactions
> - Cannot charge fees = recording contributions must not become a financial burden
> - Requires custom decay calculations and intimacy algorithms

## 12.4 ZK Rollup

### Core Concept

Off-chain execution, on-chain verification:
- Daily contribution records are processed at high speed on L2, with no fees and high throughput
- Zero-knowledge proofs of batch records are periodically submitted to L1
- L1 only stores compressed state roots

### ZK Rollup vs. Optimistic Rollup

| Feature | ZK Rollup | Optimistic Rollup |
|---------|-----------|-------------------|
| Verification method | Zero-knowledge proofs (mathematical guarantee) | Fraud proofs (challenge period) |
| Confirmation time | Fast | Slow (typically 7 days) |
| Security | Mathematical guarantee | Relies on honest validators |
| Computational cost | High | Low |

**Choice: ZK Rollup** — a reputation system requires fast confirmation and mathematically guaranteed security.

### Division of Responsibilities

- **L2 processing**: contribution record creation, real-time MeriToken calculation, intimacy updates
- **L1 anchoring**: state roots, identity registration/changes, governance voting results, penalty records

## 12.5 Data Storage

```
On-chain (L1): Identity registry, state roots, governance records, penalty records
Rollup (L2): MeriToken balances and batches, intimacy, contribution records
Off-chain (IPFS, etc.): Interaction details, contribution evidence, large files
```

## 12.6 Consensus Mechanism

- **Validator admission**: requires a certain amount of MeriToken (reputation collateral)
- **Validation incentives**: validation work itself is a contribution and can earn Merit
- **L1 consensus**: GRANDPA/BABE (Substrate defaults)
- **L2 consensus**: lightweight BFT

## 12.7 Performance Estimates

Assuming 1 billion users, each generating 5 records per day:
- Daily transaction volume: 5 billion records
- TPS requirement: ~58,000
- Requires: multiple parallel Rollup instances (sharding), efficient proof generation, distributed L2 nodes

## 12.8 Discussion Notes

> Core decisions in the technical architecture:
> - Dedicated chain rather than general-purpose public chain: GMC's requirements are too specialized
> - ZK Rollup rather than Optimistic: requires fast confirmation and mathematical guarantees
> - Layered storage: a balance between security and scalability
> - Performance is the greatest challenge: the scale of population-wide participation is unprecedented
>
> This is an architecture concept at the discussion draft stage; actual implementation will need to be adjusted based on technological developments.
