# Chapter 6: Contribution Recognition Mechanism

## 6.1 The Core Challenge of Recognition

Contribution recognition is the most critical and most difficult component of GMC. The core challenge lies in:

- Contributions can be objective (quantifiable) or subjective (requiring evaluation)
- Objective measurement is naturally resistant to fraud but has narrow coverage
- Subjective evaluation has broad coverage but is easily manipulated (similar to fake online reviews)

## 6.2 Two Acquisition Methods

### Method 1: Objective Measurement

Based on verifiable objective metrics, the system automatically mints Merit:

| Measurement Dimension | Examples | Characteristics |
|-----------------------|----------|-----------------|
| By volume | Customers served, proposals delivered | Auditable, fraud-resistant |
| By time | Service hours, online duration | Timestamps are verifiable |
| By output | Code commits, documentation produced | Traceable on-chain |

Advantages: Automatic, efficient, high difficulty of fraud.
Limitations: Cannot cover all types of contributions.

### Method 2: Task Bounty

Preset Merit for a specific task; upon completion, stakeholders vote to confirm:

1. **Publish**: Define the task objective, Merit reward, and influence duration
2. **Execute**: The executor completes the task and submits results
3. **Vote**: Stakeholders vote on whether the criteria are met
4. **Mint**: Upon approval, the system mints MeriToken

## 6.3 Stakeholder Mechanism

### Who Are the Stakeholders

Parties with a vested interest in a given contribution. For example:
- A government consultation coFay's contribution → voted on collectively by its users
- An open-source project contribution → voted on by the project's users and collaborators

### Key Rule: Exclude High-Intimacy Individuals

Since GMC records the social relationship network, the system can:
1. Identify individuals whose intimacy with the contributor exceeds a threshold
2. Exclude these individuals from the voting pool
3. Select voters from the remaining stakeholders

This is the core mechanism for preventing "insiders voting for insiders."

### Consensus Approval Conditions

- A proportion threshold is set (e.g., 2/3 majority)
- Voting weight is tied to the voter's own MeriToken
- Once the threshold is exceeded, the system automatically mints

## 6.4 Determining Influence Duration

Each contribution recognition must also determine the influence duration:

| Determination Method | Applicable Scenario |
|---------------------|---------------------|
| Preset by contribution type | Objective measurement (e.g., customer service interaction = 30 days) |
| Set by task publisher | Task bounty |
| Decided collectively by voters | Community consensus |

The influence duration determines the decay rate of that Merit batch.

## 6.5 Anti-Fraud Strategies

> Core question under discussion: Bitcoin mining is purely objective measurement, naturally fraud-resistant. But GMC includes subjective evaluation — how do we prevent fake reviews?
>
> Approach: Not to eliminate subjectivity, but to make the cost of fraud far exceed the benefit.

Defense combination:

1. **Intimacy exclusion**: Exclude voters with close relationships to the subject being evaluated
2. **MeriToken weighting**: High-reputation voters carry more weight; fraudsters must first accumulate substantial genuine reputation
3. **Voting behavior audit**: Frequently voting in favor of a specific subject → flagged as anomalous
4. **Random sampling**: Randomly select voters from the stakeholder pool to reduce the possibility of collusion
5. **Retroactive accountability**: If fraud is discovered, it can be addressed retroactively through the penalty mechanism

### Design Principle

> Decompose contributions into objectively measurable components as much as possible, reducing the proportion of subjective evaluation:
> - Prioritize objective measurement (automatic, efficient, fraud-resistant)
> - Subjective evaluation is used only for scenarios that cannot be objectively quantified
> - Subjective evaluation employs multiple layers of defense to reduce fraud risk

## 6.6 Discussion Notes

> Design trade-offs in contribution recognition:
> - Efficiency vs. fairness: Objective measurement is efficient but narrow; subjective evaluation is comprehensive but susceptible to manipulation
> - Participation vs. quality: Lowering the voting threshold increases participation but may reduce evaluation quality
> - Current approach: "Objective first + subjective supplement + multi-layered defense"
> - Extended question: How is Merit created from nothing? → See the Economic Model chapter
