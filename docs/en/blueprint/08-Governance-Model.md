# Chapter 8: Governance Model

## 8.1 The Logic of Voting Power

In the post-currency era, voting power in social governance cannot be based on wealth (currency has become ineffective), nor should it be based on authority (which violates decentralization principles).

GMC's answer: **Voting power derives from one's share of contributions within a community.**

This means:
- The more you contribute and the higher your reputation, the greater your influence
- Voting power is dynamic, fluctuating with MeriToken decay and growth
- Without sustained contributions, influence naturally fades—there are no permanent privileges

## 8.2 Weighted Voting Mechanism

```
Individual effective votes = Base votes × (Individual MeriToken / Community total MeriToken)
```

Everyone has the right to vote (base votes = 1), but the weight is proportional to one's MeriToken share.

### Example

A community has 3 members:

| Member | MeriToken | Share | Effective Votes |
|--------|-----------|-------|-----------------|
| A | 100 | 50% | 0.5 |
| B | 60 | 30% | 0.3 |
| C | 40 | 20% | 0.2 |

A + C vote in favor, B votes against: In favor 0.7 > Against 0.3 → Passed.

## 8.3 Governance Scenarios

| Scenario | Voters | Passing Condition | Notes |
|----------|--------|-------------------|-------|
| Contribution recognition | Stakeholders (excluding high-intimacy) | 2/3 majority | Routine operation |
| Penalty decision | Affected stakeholders | 3/4 majority | Severe behavior requires a higher threshold |
| Rule change | All community members | 2/3 absolute majority | Affects everyone |

## 8.4 Communities

Communities are the governance units in GMC:

- A person can belong to multiple communities
- Communities can be nested (sub-communities)
- Voting power is calculated independently in each community
- The same person may have entirely different levels of influence in different communities

## 8.5 Anti-Monopoly

MeriToken share determines voting power, but extreme concentration must be prevented:

- **The decay mechanism itself is anti-monopoly**: without sustained contributions, voting power is lost
- **Community layering**: in large communities, individual shares are naturally diluted
- **Share rather than absolute value**: increases in total supply do not affect governance fairness

## 8.6 Human-AI Collaborative Governance

- An iFay's vote represents the will of its human archetype
- A coFay's vote represents the will of its affiliated organization
- All voting behavior is transparent and auditable on-chain
- Humans and Fays operate within the same governance framework

## 8.7 Discussion Notes

> Design choices for the governance model:
> - "Share-weighted" rather than "one person, one vote": the core principle is "contributions determine voting power"
> - "Share" rather than "absolute value": prevents early participants from permanently monopolizing influence
> - Decay is a natural safeguard for governance fairness
> - A "voting power cap" mechanism may be needed in the future to prevent absolute control by a single entity in small communities
