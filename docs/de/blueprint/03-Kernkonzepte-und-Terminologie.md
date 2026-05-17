# Kapitel 3: Kernkonzepte und Terminologie

## 3.1 Überblick über Entitätsbeziehungen

```
Natürliche Person (menschlicher Archetyp)
  └── HumanID (global eindeutig)
        ├── iFay-1 (permanent gebunden)
        │     ├── FayID
        │     └── MeritPocket → MeriToken (mehrere Chargen)
        ├── iFay-2 (permanent gebunden)
        │     ├── FayID
        │     └── MeritPocket → MeriToken (mehrere Chargen)
        └── ...

Organisation / Einzelperson
  └── coFay (Eigentumsverhältnis, übertragbar)
        ├── FayID
        └── MeritPocket → MeriToken (mehrere Chargen)
```

## 3.2 Kernterminologie

### MeriToken

Die quantitative Einheit des Beitrags. Repräsentiert die soziale Glaubwürdigkeit und das Stimmrecht einer Entität.

- Nicht handelbar, nicht übertragbar
- Verfällt über die Zeit gemäß einer Exponentialkurve
- Hat einen Mindestwert, der nicht auf null fallen kann
- Kann unter strengen Regeln vererbt werden (mit Abschwächung)

### MeritPocket

Der Behälter für MeriToken, analog zu einer Wallet. Jeder Fay ist an ein MeritPocket gebunden.

### iFay (Persönlicher Fay)

Ein persönlicher KI-Agent — die „digitale Rüstung". Permanent an eine natürliche Person gebunden; kann nicht entbunden werden.

- Wesen: Eine Erweiterung der Persönlichkeit, kein Vermögenswert
- Von einem iFay generiertes MeriToken gehört seinem menschlichen Archetyp
- Eine Person kann mehrere iFay besitzen

### coFay (Organisatorischer Fay)

Ein organisatorischer oder kommerzieller KI-Agent. Gehört einer Einzelperson oder Organisation.

- Wesen: Ein Vermögenswert, übertragbar
- Von einem coFay generiertes MeriToken gehört seinem aktuellen Eigentümer
- Bei Übertragung wird das MeritPocket mitübertragen; MeriToken wird nicht abgeschwächt

### Menschlicher Archetyp

Die natürliche Person, an die ein iFay permanent gebunden ist. Jeder menschliche Archetyp besitzt eine eindeutige HumanID.

### HumanID / FayID

- HumanID: Ein global eindeutiger Identifikator für menschliche Identität
- FayID: Ein global eindeutiger Identifikator für Fay-Identität
- Eine HumanID kann mehreren FayIDs entsprechen
- HumanID und FayID treten paarweise auf

### Merit-Charge

Die Aufzeichnungseinheit für jeden Beitragserwerb, enthaltend: erworbene Menge, Einflussdauer, Verfallsparameter und Erwerbszeitpunkt.

### Stakeholder

Parteien mit einem berechtigten Interesse an einem bestimmten Beitrag. Verantwortlich für die Konsensabstimmung über Beiträge; ausgewählt durch Ausschluss von Personen mit übermäßig hoher Vertrautheit zum Beitragenden.

### Digitaler Friedhof

Der Zustand, in den ein iFay nach dem Tod seines menschlichen Archetyps versetzt werden kann. Ein iFay im digitalen Friedhof kann noch passive Interaktionen haben, aber alle Aktionen werden als „vom digitalen Friedhof" gekennzeichnet.

## 3.3 Wesentliche Unterschiede zwischen iFay und coFay

| Dimension | iFay | coFay |
|-----------|------|-------|
| Wesen | Erweiterung der Persönlichkeit | Vermögenswert |
| Bindungsverhältnis | Permanent gebunden, kann nicht entbunden werden | Eigentumsverhältnis, übertragbar |
| Nach Tod des menschlichen Archetyps | Geht in Vormundschaft oder digitalen Friedhof über | Wird als Vermögenswert vererbt/übertragen |
| MeriToken bei Übertragung | Kann nicht übertragen werden | Wird mit dem coFay übertragen, keine Abschwächung |
| Anzahl der Eigentümer | Gehört nur einer natürlichen Person | Gehört einer Einzelperson oder Organisation |

## 3.4 MeriToken und Soulbound Token (SBT)

SBT ist ein von Vitalik Buterin 2022 vorgeschlagenes Konzept — ein nicht übertragbarer Token, der an eine bestimmte Identität gebunden ist und zur Darstellung von Eigenschaften dient, die nicht gehandelt werden sollten (Berechtigungsnachweise, Reputation, Errungenschaften).

MeriToken ist eine erweiterte Version von SBT:

| Eigenschaft | Standard-SBT | MeriToken |
|-------------|--------------|-----------|
| Nicht übertragbar | ✓ | ✓ |
| Bindungsmethode | An Wallet-Adresse gebunden | An iFay → MeritPocket → menschlichen Archetyp gebunden |
| Zeitdimension | Keine (dauerhaft gültig) | Ja (exponentieller Verfall) |
| Vererbbar | Nein | Ja (mit Abschwächung) |
| Quantifizierungsmethode | Typischerweise boolesch (hat/hat nicht) | Kontinuierlicher numerischer Wert |
| Mindestgarantie | Keine | Ja (minMerit) |

## 3.5 Diskussionsnotizen

> Designlogik des Terminologiesystems:
> - Die dreischichtige Bindung (menschlicher Archetyp → iFay → MeritPocket) isoliert die Identitäts-, Agenten- und Vermögensschicht
> - iFay als „Erweiterung der Persönlichkeit" ist nicht übertragbar und stellt sicher, dass Reputation untrennbar von der Person ist
> - coFay als „Vermögenswert" ist übertragbar und gewährleistet organisatorische Betriebsflexibilität
> - MeriToken referenziert SBT, fügt aber Zeitverfall und Vererbbarkeit hinzu, wodurch es besser für dynamische Beitragsmessungsszenarien geeignet ist
