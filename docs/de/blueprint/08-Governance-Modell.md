# Kapitel 8: Governance-Modell

## 8.1 Die Logik des Stimmrechts

In der Post-Währungs-Ära kann Stimmrecht in der sozialen Governance nicht auf Reichtum basieren (Währung ist unwirksam geworden), noch sollte es auf Autorität basieren (was Dezentralisierungsprinzipien verletzt).

Die Antwort von GMC: **Stimmrecht leitet sich aus dem eigenen Beitragsanteil innerhalb einer Gemeinschaft ab.**

Das bedeutet:
- Je mehr beigetragen wird und je höher die Reputation, desto größer der Einfluss
- Stimmrecht ist dynamisch und schwankt mit dem Verfall und Wachstum von MeriToken
- Ohne nachhaltige Beiträge schwindet der Einfluss natürlich — es gibt keine dauerhaften Privilegien

## 8.2 Gewichteter Abstimmungsmechanismus

```
Individuelle effektive Stimmen = Basisstimmen × (Individuelles MeriToken / Gesamtes MeriToken der Gemeinschaft)
```

Jeder hat das Recht abzustimmen (Basisstimmen = 1), aber das Gewicht ist proportional zum eigenen MeriToken-Anteil.

### Beispiel

Eine Gemeinschaft hat 3 Mitglieder:

| Mitglied | MeriToken | Anteil | Effektive Stimmen |
|----------|-----------|--------|-------------------|
| A | 100 | 50% | 0,5 |
| B | 60 | 30% | 0,3 |
| C | 40 | 20% | 0,2 |

A + C stimmen dafür, B stimmt dagegen: Dafür 0,7 > Dagegen 0,3 → Angenommen.

## 8.3 Governance-Szenarien

| Szenario | Abstimmende | Annahmebedingung | Anmerkungen |
|----------|-------------|------------------|-------------|
| Beitragsanerkennung | Stakeholder (ohne hohe Vertrautheit) | 2/3-Mehrheit | Routinebetrieb |
| Strafentscheidung | Betroffene Stakeholder | 3/4-Mehrheit | Schweres Verhalten erfordert höhere Schwelle |
| Regeländerung | Alle Gemeinschaftsmitglieder | 2/3 absolute Mehrheit | Betrifft alle |

## 8.4 Gemeinschaften

Gemeinschaften sind die Governance-Einheiten in GMC:

- Eine Person kann mehreren Gemeinschaften angehören
- Gemeinschaften können verschachtelt sein (Untergemeinschaften)
- Stimmrecht wird in jeder Gemeinschaft unabhängig berechnet
- Dieselbe Person kann in verschiedenen Gemeinschaften völlig unterschiedliche Einflussniveaus haben

## 8.5 Anti-Monopol

Der MeriToken-Anteil bestimmt das Stimmrecht, aber extreme Konzentration muss verhindert werden:

- **Der Verfallsmechanismus selbst ist anti-monopolistisch**: Ohne nachhaltige Beiträge geht Stimmrecht verloren
- **Gemeinschaftsschichtung**: In großen Gemeinschaften werden individuelle Anteile natürlich verwässert
- **Anteil statt Absolutwert**: Erhöhungen des Gesamtangebots beeinträchtigen die Governance-Fairness nicht

## 8.6 Mensch-KI-kollaborative Governance

- Die Stimme eines iFay repräsentiert den Willen seines menschlichen Archetyps
- Die Stimme eines coFay repräsentiert den Willen seiner angeschlossenen Organisation
- Alles Abstimmungsverhalten ist transparent und on-chain prüfbar
- Menschen und Fays operieren innerhalb desselben Governance-Rahmens

## 8.7 Diskussionsnotizen

> Designentscheidungen für das Governance-Modell:
> - „Anteilsgewichtet" statt „eine Person, eine Stimme": Das Kernprinzip ist „Beiträge bestimmen Stimmrecht"
> - „Anteil" statt „Absolutwert": Verhindert, dass frühe Teilnehmer dauerhaft den Einfluss monopolisieren
> - Verfall ist ein natürlicher Schutz für Governance-Fairness
> - Ein „Stimmrecht-Obergrenze"-Mechanismus könnte in Zukunft benötigt werden, um absolute Kontrolle durch eine einzelne Entität in kleinen Gemeinschaften zu verhindern
