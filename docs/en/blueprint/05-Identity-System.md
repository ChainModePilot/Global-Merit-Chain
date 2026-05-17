# Chapter 5: Identity System

## 5.1 Why a Dedicated Identity System Is Needed

Identity in GMC differs from traditional internet accounts:

- It binds to a natural person's lifelong reputation and cannot be arbitrarily created or discarded
- It must support the permanent binding of iFay and the ownership transfer of coFay
- It must be verifiable in a decentralized environment while protecting privacy

## 5.2 Identity Layers

```
┌─────────────────────────────────────┐
│  Layer 1: Natural Person Identity    │  ← Unique, lifelong
│           (HumanID)                  │
├─────────────────────────────────────┤
│  Layer 2: Fay Identity (FayID)       │  ← Paired with HumanID
├─────────────────────────────────────┤
│  Layer 3: Asset Layer (MeritPocket)  │  ← Bound to FayID
└─────────────────────────────────────┘
```

### HumanID

- Globally unique, identifies a natural person
- One HumanID can correspond to multiple FayIDs
- Valid for life, cannot be deregistered (but can enter cemetery state)

### FayID

- Globally unique, identifies a Fay
- Each FayID is associated with one MeritPocket
- An iFay's FayID is permanently bound to a HumanID
- A coFay's FayID ownership can be transferred

## 5.3 On-Chain Verification Scheme

### Scheme Comparison

| Scheme | Principle | Advantages | Disadvantages | Applicable Scenarios |
|--------|-----------|------------|---------------|---------------------|
| PKI (Public-Private Key Pair) | Key pair signature verification | Mature, efficient, decentralized | Private key loss = identity loss | Basic signatures |
| DID (Decentralized Identity) | W3C standard, on-chain identity documents | Standardized, supports key recovery | Relatively complex | Relationship mapping |
| ZKP (Zero-Knowledge Proof) | Proves identity without revealing information | Extremely strong privacy protection | High computational overhead | Privacy scenarios |

### Recommendation: Layered Combination

1. **Base layer (basic verification)**: PKI
   - Signature mechanism for all on-chain operations
   - Every HumanID and FayID has a key pair

2. **Middle layer (relationship management)**: DID
   - Manages HumanID ↔ FayID binding relationships
   - Supports key rotation and social recovery
   - Stores identity metadata

3. **Upper layer (privacy scenarios)**: ZKP
   - Proves identity during voting without revealing who you are
   - Verifies relationships during inheritance authentication without exposing details
   - Protects whistleblowers during penalty complaints

### Rationale

> Every single scheme has limitations:
> - Pure PKI cannot solve key loss and lacks privacy protection
> - Pure DID has insufficient performance for high-frequency verification
> - Pure ZKP has excessive computational costs
>
> A layered combination lets each layer focus on the scenarios it handles best.

## 5.4 iFay Lifecycle

```
Creation → Binding to human archetype → Normal operation → [Human archetype passes away] → Guardianship / Digital cemetery
```

### Normal Operation

- iFay acts on behalf of the human archetype
- All MeriToken generated belongs to the human archetype
- The human archetype participates in voting, contribution recognition, etc. through iFay

### Guardianship

When the human archetype passes away:
- An heir may apply to become the guardian
- The guardian may manage on behalf of the deceased, but **cannot act in the identity of the human archetype**
- All guardianship actions must display the guardian's information
- There is an explicit guardianship marker on-chain

### Digital Cemetery

- An iFay may still have passive interactions after being placed in the cemetery
- All interactions are labeled "from the digital cemetery"
- No new MeriToken is actively generated
- Existing MeriToken continues to decay normally

## 5.5 coFay Ownership Transfer

As an asset, coFay follows these transfer rules:

1. The MeritPocket transfers with the coFay; MeriToken is not attenuated
2. Transfer records are stored on-chain; ownership change history is tamper-proof
3. Transfer requires dual-party signature confirmation
4. The coFay's voting power continuity is unaffected by transfer

## 5.6 Sybil Attack Prevention

One-person-multiple-accounts is a classic threat to decentralized identity systems:

- HumanID registration requires a uniqueness proof (specific method TBD)
- Social graph analysis: Real users have natural social networks; fake accounts exhibit abnormal patterns
- Behavioral pattern analysis: Multiple accounts controlled by the same person share similar behavioral characteristics
- Progressive trust: New users' permissions and influence are released gradually

## 5.7 Discussion Notes

> Core trade-offs in the identity system:
> - Security vs. usability: Three-layer verification increases security but also increases complexity
> - Privacy vs. transparency: ZKP protects privacy; on-chain records ensure transparency
> - Permanence vs. flexibility: iFay's permanent binding ensures reputation is inseparable from the person; coFay's transferability ensures commercial flexibility
> - Sybil attack prevention is an eternal challenge for decentralized identity and requires a combination of multiple approaches
