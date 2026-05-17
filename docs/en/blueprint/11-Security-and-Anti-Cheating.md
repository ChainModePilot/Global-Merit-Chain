# Chapter 11: Security and Anti-Cheating

## 11.1 Threat Model

| Threat | Description | Impact |
|--------|-------------|--------|
| Merit farming | Obtaining MeriToken through fake contributions | Inflated voting power |
| Collusive voting | Multiple parties conspiring to manipulate recognition votes | Illegitimate Merit acquisition |
| Intimacy farming | Fabricating interactions to boost intimacy | Bypassing exclusions, reducing inheritance attenuation |
| Identity forgery | Creating fake HumanIDs | Multiple identities acquiring multiple Merit shares |
| Sybil attack | One person controlling multiple identities | Manipulating votes |

## 11.2 Preventing Merit Farming

### Safeguards for Objective Measurement

- System records automatically, leaving little room for human manipulation
- Cross-verification is possible (e.g., comparing work hours vs. output)
- Statistical anomaly detection

### Safeguards for Subjective Evaluation

> Core principle: make the cost of cheating far exceed the benefit.

1. **Intimacy exclusion**: exclude voters with close relationships
2. **MeriToken weighting**: high-reputation voters carry more weight; cheaters must first accumulate substantial genuine reputation
3. **Behavioral auditing**: frequently voting in favor of a specific individual → flagged as anomalous
4. **Random sampling**: randomly selecting voters to reduce the possibility of collusion
5. **Retroactive accountability**: once cheating is discovered, all participants are penalized

## 11.3 Preventing Intimacy Farming

- Interaction quality assessment (not just frequency)
- One-way interactions are invalid (must be bidirectional)
- Large volumes of interactions in a short period are treated as anomalous
- Isolated high-frequency interactions between two individuals (with no shared social circle) are treated as suspicious

## 11.4 Key Security

- Multi-signature schemes: critical operations require confirmation from multiple keys
- Key rotation: periodic replacement
- Social recovery: trusted contacts assist in recovery

## 11.5 Privacy Protection

- Voting content is not public (ZKP); only results are disclosed
- Intimacy values can be selectively disclosed
- Interaction content is not stored on-chain
- Anonymous participation is supported (ZKP proves eligibility without revealing identity)

## 11.6 Discussion Notes

> Design philosophy of the security mechanism:
> - There is no perfect anti-cheating solution; the goal is to make the cost of cheating far exceed the benefit
> - Multi-layered defenses are more effective than any single mechanism
> - Preventive measures + retroactive accountability form a closed loop
> - Anti-cheating is a continuous adversarial process; the system must be able to evolve
