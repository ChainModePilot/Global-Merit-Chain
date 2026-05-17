# Chapter 4: MeriToken Model

## 4.1 Model Overview

MeriToken is the core measurement unit of GMC. Its design must answer a key question: **How can contribution measurement reflect current activity while also respecting historical contributions?**

The answer is: exponential decay + non-zero floor value.

## 4.2 Two Key Values

Each MeritPocket maintains two core values:

- **curMerit** (current MeriToken): The real-time contribution measurement value; decays over time and grows with new contributions
- **minMerit** (floor value): The lower bound of decay, representing the long-term sediment of historical contributions; only increases (except under penalties)

```
curMerit ≥ minMerit ≥ e (initial value)
```

## 4.3 Acquisition

MeriToken is acquired through contributions; the system mints new Tokens:

| Acquisition Method | Description | Trigger Condition |
|--------------------|-------------|-------------------|
| Objective measurement | Automatically calculated based on verifiable metrics | System automatically records threshold met |
| Task bounty | Preset Merit for a specific task | Stakeholders vote to approve upon completion |
| Initial allocation | Granted upon network registration | Identity registration completed |

Initial value = e ≈ 2.718 (the natural constant, naturally aligned with the exponential decay model).

## 4.4 Decay Model

### Core Idea

Each Merit acquisition batch has an independent **influence duration**. The influence duration reflects the timeliness of that contribution — a contribution with 100 days of influence has its Merit fully decayed within 100 days.

### Single-Batch Decay Formula

```
MeriToken_i(t) = (V_i - B_i) × e^(-λ_i × t) + B_i
```

- `V_i`: Initial Merit value of the batch
- `B_i`: The batch's contribution to the floor value
- `λ_i`: Decay coefficient, determined by influence duration T_i (λ_i = k / T_i, where k is a constant)
- `t`: Time elapsed since acquisition

### Total Current MeriToken

```
curMerit = Σ MeriToken_i(t)  (sum of all active batches)
```

When all batches have fully decayed, curMerit approaches minMerit.

## 4.5 Floor Value (minMerit)

### Update Rule

Each time new Merit is acquired, the floor value is updated:

Let current curMerit = M, newly acquired Merit = x, current floor value = B, then:

```
New floor value B' = (x + M) × B / M
```

Meaning: The floor value grows in proportion to the new Merit's share of the total.

### Properties

- Starting value = e ≈ 2.718
- Only increases (except under penalties)
- Represents the indelible sediment of historical contributions
- Even if contributions cease entirely, curMerit will ultimately never fall below minMerit

### Edge Case

When curMerit = minMerit (i.e., at the floor state) and new Merit x is acquired:
```
B' = (x + B) × B / B = x + B
```
The floor value increases directly by x — meaning Merit acquired while at the floor state is entirely deposited as floor value.

## 4.6 Implementation of Per-Batch Independent Decay

### Challenges

- Each MeritPocket must maintain a list of Merit batches
- Querying the current value requires iterating over all batches that have not fully decayed
- On-chain storage and computation costs grow linearly with the number of batches

### Optimization Strategies

1. **Batch merging**: Batches with similar influence durations are periodically merged to reduce active batch count
2. **Off-chain computation**: Use Rollup to compute real-time values off-chain; only store snapshots and proofs on-chain
3. **Batch sedimentation**: When the maximum active batch count is exceeded, the oldest batches are automatically sedimented into the floor value
4. **Lazy computation**: Precise values are only calculated when needed (e.g., during voting or queries)

## 4.7 Design Philosophy

### Why Exponential Decay?

- Incentivizes continuous contribution rather than a single large contribution followed by inactivity
- Reflects the timeliness of contributions — more recent contributions have greater impact on current reputation
- Naturally simulates the decay of social memory
- Decays rapidly at first and slows later, aligning with intuition

### Why a Non-Zero Floor?

- Acknowledges the long-term value of historical contributions — past efforts do not completely zero out
- Prevents long-term contributors from losing all voting power due to a brief pause
- The floor value grows with cumulative contributions, rewarding sustained participation

### Why Independent Influence Duration Per Batch?

- Different contributions naturally have different timeliness
- A single customer service interaction may have an influence of only 30 days
- Maintaining an open-source project may have an influence lasting years
- A uniform decay rate would distort the value of different types of contributions

## 4.8 Discussion Notes

> Key decisions in the MeriToken model:
> - Exponential decay + non-zero floor: Strikes a balance between "incentivizing continuous participation" and "respecting historical contributions"
> - Independent influence duration per batch: Increases implementation complexity but more accurately reflects differences in contribution timeliness
> - Floor value only increases (except under penalties): Protects the fundamental rights of long-term contributors
> - Initial value of e: Combines mathematical elegance with practical significance
>
> To be further examined: Whether the floor value update formula behaves reasonably under extreme conditions
