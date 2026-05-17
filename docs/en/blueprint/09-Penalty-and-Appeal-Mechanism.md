# Chapter 9: Penalty and Appeal Mechanism

## 9.1 Why Penalties Are Needed

Any reputation system requires the ability to correct errors. When contributions are incorrectly recognized or fraud exists, the system must be able to make corrections.

The penalty mechanism is the ultimate safeguard of GMC's credibility.

## 9.2 Penalty Types

| Type | Effect | Severity |
|------|--------|----------|
| Deduct curMerit | Reduces current MeriToken, affecting immediate voting power | Lighter |
| Deduct minMerit | Lowers the floor value, affecting long-term minimum reputation guarantee | Severe |

Deducting minMerit is a more severe penalty—it breaks the rule that "the floor value only increases, never decreases," meaning that the accumulation of historical contributions is partially revoked.

### Severity Reference

| Violation Level | Penalty Method | Example |
|-----------------|----------------|---------|
| Minor | Deduct partial curMerit | Exaggerated contributions |
| Moderate | Deduct significant curMerit | Duplicate submissions |
| Severe | curMerit + partial minMerit | Collusion to farm Merit |
| Extreme | Major deduction of both | Systematic fraud |

## 9.3 Trigger Process

```
Complaint filed → Stakeholder acceptance vote → [Rejected if not passed] → Penalty vote → Execution
```

### Rules

1. **Complaints must target a specific Merit acquisition batch**: vague complaints are not allowed; they must point to a specific event
2. **Stakeholder acceptance**: a certain proportion of relevant stakeholders must accept the complaint before a formal vote is initiated
3. **Penalty vote**: requires a higher passing threshold (e.g., 3/4 majority)
4. **Automatic execution**: once the vote passes, the system automatically applies the deduction

### Preventing Malicious Complaints

- Complainants must provide evidence or justification
- Frequent malicious complainants may be flagged
- Complaint records themselves are stored on-chain, ensuring transparency

## 9.4 Appeals

The penalized party has the right to appeal:

1. An appeal may be filed within a certain period after penalty execution
2. A broader group of community members re-votes (to avoid the same group judging repeatedly)
3. If the appeal succeeds, the penalty is revoked and MeriToken is restored

## 9.5 Interaction with Other Mechanisms

- **Penalties are the only mechanism that can reduce minMerit** (aside from natural decay)
- Penalty records are stored on-chain, including the penalized entity, reason, amount, and voting results
- Penalty history affects the entity's social reputation (viewable by others)

## 9.6 Discussion Notes

> Design philosophy of the penalty mechanism:
> - Must be evidence-based (targeting specific batches), preventing "baseless accusations"
> - Graduated penalties reflect the principle of proportionality
> - Complaints require a threshold (stakeholder acceptance), preventing malicious harassment
> - The right to appeal safeguards fairness; expanding the scope prevents echo chamber effects
> - The fact that minMerit can be reduced by penalties serves as the strongest deterrent against integrity violations
