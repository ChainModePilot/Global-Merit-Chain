# Chapter 13: Economic Model and Minting Logic

## 13.1 MeriToken Is Not a Currency

To reiterate MeriToken's economic positioning:

- Non-tradable, non-transferable, non-cashable (no secondary market)
- No speculative value
- Not a medium of exchange
- Purely a measure of contribution and a carrier of voting power

> The sole inbound monetary touchpoint is the optional, one-way, capped cold-start bootstrap defined in §13.5. It is strictly one-way (no reputation→fiat exit) and, like all Merit, decays to the floor value — so it never confers persistent voice. This refines, not reverses, the positioning above.

Therefore, the constraints of traditional monetary economics (inflation control, monetary policy) do not apply to MeriToken.

## 13.2 Minting Approach Selection

> Three approaches were evaluated during discussion:

| Approach | Description | Advantages | Disadvantages |
|----------|-------------|-----------|---------------|
| Fixed supply | Preset cap | Simple | Increasing difficulty for latecomers, unfair |
| Periodic quota | Fixed minting amount per period | Controls total supply | Contributions become a zero-sum game |
| **Uncapped + decay self-balancing** | Mint on demand, decay automatically burns | Fair, no latecomer disadvantage | Requires precise decay model |

### Choice: Uncapped Minting + Decay Self-Balancing

Rationale:
- Merit is not a currency; it does not need scarcity to maintain value
- It represents "current active contribution level"; decay guarantees this
- Avoids unfair disadvantages for latecomers
- Voting power is based on share; changes in total supply do not affect governance fairness

## 13.3 Why Over-Issuance Will Not Occur

> Key question raised during discussion: Merit is created from nothing—won't it be over-issued?

Answer:
1. **Decay is a natural burn mechanism**: old MeriToken continuously decays
2. **Dynamic equilibrium**: when minting rate = decay rate, total supply tends toward stability
3. **Share determines voting power**: even if total supply increases, individual voting power depends on share rather than absolute value
4. **Analogy**: academic citation counts have no cap, but the influence of older papers naturally decays—the system self-balances

## 13.4 Dynamic Equilibrium

### Steady State

When user count is stable: total network MeriToken ≈ constant

### Growth Phase

New users increase → total supply grows → but per-capita tends toward stability → voting power shares are naturally diluted

### Decline Phase

Active users decrease → minting decreases while decay continues → total supply drops → remaining active users' shares increase

## 13.5 Initial Allocation

- Registration grants MeriToken = e ≈ 2.718
- Initial minMerit = e
- Ensures every new user has basic participation capability
- e is small enough not to significantly dilute existing users, yet large enough to guarantee basic participation rights

### Optional Cold-Start Bootstrap (one-way, capped, decaying)

To address present-moment voice and the cold-start problem, a participant MAY optionally make a one-way fiat payment to receive an above-baseline initial reputation grant, **up to a hard cap**. This is the system's **only** inbound monetary touchpoint, and it is strictly one-way:

- **No exit**: there is no reputation→fiat path — no resale, no cash-out, no secondary market, no transfer.
- **Decays like all Merit**: purchased initial reputation is subject to the same decay and collapses toward the floor value (`minMerit`). It MAY be configured to decay faster and/or be non-renewable, to prevent "keep paying = permanent power."
- **Net effect**: a purchase buys only a temporary head start. Without subsequent genuine contribution, the purchased reputation necessarily decays to the floor value — money cannot buy persistent voice. At steady state, voice is determined by sustained contribution.

> ⚠️ Parameters to be specified (this is qualitative alignment only): the purchase cap; whether purchased reputation decays faster or is non-renewable; the exact coupling to the decay/floor model. See iFay ADR `decisions/0001` (Decaying Bootstrap) for the cross-system ruling. This **refines**, not reverses, MeriToken's "measure of contribution, not financial asset" positioning.

## 13.6 Incentive Analysis

MeriToken is non-tradable, but the incentives it provides are:

| Incentive | Description |
|-----------|-------------|
| Voting power | Influence in community decision-making |
| Social recognition | High MeriToken = high credibility |
| Priority access | Preferential allocation of certain resources or opportunities |
| Legacy value | Can be partially passed on to descendants |

In the post-currency era, social recognition and voting power are themselves the strongest incentives.

## 13.7 Discussion Notes

> Core insights of the economic model:
> - MeriToken is not a currency and does not need the constraints of monetary economics
> - Decay is the most elegant "burn" mechanism—no human intervention needed, naturally self-balancing
> - Voting power based on share means changes in total supply do not affect governance fairness
> - The core advantage of this model: simplicity, self-balancing, fairness
> - No complex "monetary policy" is needed to maintain stability
