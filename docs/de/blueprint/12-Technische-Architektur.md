# Kapitel 12: Technische Architektur

## 12.1 Architekturübersicht

```
┌─────────────────────────────────────────────┐
│  Anwendungsschicht (DApp, Fay-Schnittstelle, │
│  Governance-UI)                              │
├─────────────────────────────────────────────┤
│  Layer 2: ZK Rollup (Hochfrequente           │
│  Transaktionsverarbeitung)                   │
│  - Beitragsaufzeichnungen, Vertrautheits-    │
│    aktualisierungen, tägliche Interaktionen  │
├─────────────────────────────────────────────┤
│  Layer 1: Substrate-Dedizierte Chain         │
│  (Settlement & Konsens)                      │
│  - State-Root-Verankerung, Identitäts-       │
│    verwaltung, Governance-Abstimmungen       │
├─────────────────────────────────────────────┤
│  Identitätsschicht: DID + PKI + ZKP          │
└─────────────────────────────────────────────┘
```

## 12.2 Technologieauswahl-Vergleich

> Während der Diskussion wurden mehrere Ansätze evaluiert:

| Ansatz | Vorteile | Nachteile | Schlussfolgerung |
|--------|----------|-----------|------------------|
| Ethereum Mainnet | Ausgereiftes Ökosystem, hohe Sicherheit | Hohe Gas-Gebühren, niedriger TPS (15–30) | Nicht geeignet für hochfrequente Aufzeichnung im Bevölkerungsmaßstab |
| Ethereum L2 | Reduzierte Gebühren | Weiterhin durch Ethereum-Ökosystem eingeschränkt | Alternative |
| DAG (IOTA/Nano) | Hoher Durchsatz, keine Gebühren | Schwache Konsenssicherheit | Unzureichende Sicherheit |
| **Substrate Custom Chain** | Vollständig anpassbar, keine Gas-Gebühren | Erfordert Aufbau eines eigenen Ökosystems | **Empfohlen** |

### Das Gas-Gebühren-Problem

Gas-Gebühren sind die Rechenkosten pro Transaktion auf öffentlichen Chains wie Ethereum. Wenn die gesamte Bevölkerung täglich große Mengen an Mikro-Beitragsaufzeichnungen generiert, wäre die Aufzeichnung jeder einzelnen on-chain unerschwinglich teuer. GMC erfordert eine kostenlose oder extrem kostengünstige Aufzeichnungsmethode.

### Das Durchsatz-Problem

Das Ethereum-Mainnet verarbeitet ungefähr 15–30 TPS. Für Beitragsaufzeichnungen von Milliarden Nutzern weltweit ist dieser Durchsatz bei weitem nicht ausreichend.

## 12.3 Substrate-Dedizierte Chain

### Warum Substrate

1. **Vollständig anpassbarer Konsens**: Entwurf eines Konsensalgorithmus, der speziell für Beitragsaufzeichnung geeignet ist
2. **Keine Gas-Gebühren**: Kann für gebührenfreie Transaktionen konzipiert werden
3. **Anpassbare Governance-Module**: Natürlich geeignet für Gemeinschaftskonsens
4. **Polkadot-Interoperabilität**: Kann über Relay-Chains mit anderen Chains interoperieren
5. **Modular**: Runtime-Module nach Bedarf zusammenstellen

### Begründung

> Die einzigartigen Anforderungen von GMC machen allgemeine öffentliche Chains ungeeignet:
> - Bevölkerungsweite Teilnahme = extrem hohes Transaktionsvolumen
> - Mikro-Beitragsaufzeichnungen = hochfrequente, niedrigwertige Transaktionen
> - Keine Gebühren erheben = Beitragsaufzeichnung darf keine finanzielle Belastung werden
> - Erfordert benutzerdefinierte Verfallsberechnungen und Vertrautheitsalgorithmen

## 12.4 ZK Rollup

### Kernkonzept

Off-Chain-Ausführung, On-Chain-Verifizierung:
- Tägliche Beitragsaufzeichnungen werden mit hoher Geschwindigkeit auf L2 verarbeitet, ohne Gebühren und mit hohem Durchsatz
- Zero-Knowledge-Beweise von Batch-Aufzeichnungen werden periodisch an L1 übermittelt
- L1 speichert nur komprimierte State-Roots

### ZK Rollup vs. Optimistic Rollup

| Eigenschaft | ZK Rollup | Optimistic Rollup |
|-------------|-----------|-------------------|
| Verifizierungsmethode | Zero-Knowledge-Beweise (mathematische Garantie) | Fraud-Proofs (Challenge-Periode) |
| Bestätigungszeit | Schnell | Langsam (typischerweise 7 Tage) |
| Sicherheit | Mathematische Garantie | Abhängig von ehrlichen Validatoren |
| Rechenkosten | Hoch | Niedrig |

**Wahl: ZK Rollup** — ein Reputationssystem erfordert schnelle Bestätigung und mathematisch garantierte Sicherheit.

### Aufgabenteilung

- **L2-Verarbeitung**: Erstellung von Beitragsaufzeichnungen, Echtzeit-MeriToken-Berechnung, Vertrautheitsaktualisierungen
- **L1-Verankerung**: State-Roots, Identitätsregistrierung/-änderungen, Governance-Abstimmungsergebnisse, Strafaufzeichnungen

## 12.5 Datenspeicherung

```
On-chain (L1): Identitätsregister, State-Roots, Governance-Aufzeichnungen, Strafaufzeichnungen
Rollup (L2): MeriToken-Salden und -Chargen, Vertrautheit, Beitragsaufzeichnungen
Off-chain (IPFS usw.): Interaktionsdetails, Beitragsnachweise, große Dateien
```

## 12.6 Konsensmechanismus

- **Validator-Zulassung**: Erfordert eine bestimmte Menge an MeriToken (Reputationssicherheit)
- **Validierungsanreize**: Validierungsarbeit selbst ist ein Beitrag und kann Merit verdienen
- **L1-Konsens**: GRANDPA/BABE (Substrate-Standards)
- **L2-Konsens**: Leichtgewichtiges BFT

## 12.7 Leistungsschätzungen

Angenommen 1 Milliarde Nutzer, jeder generiert 5 Aufzeichnungen pro Tag:
- Tägliches Transaktionsvolumen: 5 Milliarden Aufzeichnungen
- TPS-Anforderung: ~58.000
- Erfordert: Mehrere parallele Rollup-Instanzen (Sharding), effiziente Beweisgenerierung, verteilte L2-Knoten

## 12.8 Diskussionsnotizen

> Kernentscheidungen in der technischen Architektur:
> - Dedizierte Chain statt allgemeiner öffentlicher Chain: Die Anforderungen von GMC sind zu spezialisiert
> - ZK Rollup statt Optimistic: Erfordert schnelle Bestätigung und mathematische Garantien
> - Geschichtete Speicherung: Eine Balance zwischen Sicherheit und Skalierbarkeit
> - Leistung ist die größte Herausforderung: Der Maßstab bevölkerungsweiter Teilnahme ist beispiellos
>
> Dies ist ein Architekturkonzept im Stadium des Diskussionsentwurfs; die tatsächliche Implementierung muss basierend auf technologischen Entwicklungen angepasst werden.
