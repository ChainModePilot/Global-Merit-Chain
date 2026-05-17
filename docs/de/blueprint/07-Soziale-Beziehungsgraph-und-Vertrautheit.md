# Kapitel 7: Sozialer Beziehungsgraph und Vertrautheit

## 7.1 Warum GMC einen sozialen Beziehungsgraphen benötigt

GMC erfasst nicht nur Beiträge — es erfasst auch Beziehungen zwischen Menschen. Dies ist keine Zusatzfunktion, sondern die Grundlage zentraler Mechanismen:

| Vom Beziehungsgraphen abhängiger Mechanismus | Zweck |
|----------------------------------------------|-------|
| Vererbungsmechanismus | Bestimmt das Abschwächungsverhältnis (höhere Vertrautheit = weniger Abschwächung) |
| Stakeholder-Ausschluss | Schließt Personen aus, die zu eng mit dem Beitragenden verwandt sind, bei Abstimmungen |
| Anti-Betrug | Identifiziert abnormale Beziehungsmuster und Kollusionsverhalten |
| Gemeinschafts-Governance | Definiert Gemeinschaftsgrenzen und Mitgliedschaftsbeziehungen |

Ohne den Beziehungsgraphen kann keiner der oben genannten Mechanismen funktionieren.

## 7.2 Quellen der Vertrautheit

Vertrautheit leitet sich aus Fay-zu-Fay-Interaktionen und dem sozialen Beziehungsnetzwerk ab:

- **Interaktionshäufigkeit**: Kommunikations- und Zusammenarbeitshäufigkeit zwischen zwei Fay
- **Interaktionstiefe**: Komplexität und Dauer gemeinsamer Projekte
- **Beziehungsdeklarationen**: Von Nutzern aktiv deklarierte Beziehungen (Familie, Kollegen usw.)
- **Gemeinsame Teilnahme**: Gemeinsam besuchte Gemeinschaften, Projekte und Abstimmungen

## 7.3 On-Chain-Strategie

### Warum On-Chain-Speicherung notwendig ist

> Schlussfolgerung aus Diskussionen: Soziale Beziehungen müssen on-chain gespeichert werden, um die Authentizität der Beziehungen sicherzustellen und Fälschungen zu verhindern.
>
> Wenn Beziehungsdaten gefälscht werden können, versagen Mechanismen wie Vererbungsabschwächung und Abstimmungsausschluss.

### Geschichtete Speicherung

| Datentyp | Speicherort | Begründung |
|----------|-------------|------------|
| Beziehungsexistenz | On-chain | Gewährleistet Unfälschbarkeit |
| Vertrautheitswerte | On-chain | Dient als Grundlage für Vererbung und Ausschluss |
| Vertrautheitsberechnungsnachweise | On-chain (ZK-Beweise) | Stellt sicher, dass die Berechnung prüfbar ist |
| Interaktionsdetails | Off-chain | Großes Datenvolumen, betrifft Privatsphäre |

### Off-Chain-zu-On-Chain-Verankerung

- Interaktionsdetails werden off-chain gespeichert
- Statistische Ergebnisse werden periodisch per Hash auf der Chain verankert
- ZK-Beweise werden bei Aktualisierung der Vertrautheit eingereicht
- Jeder kann über den Hash verifizieren, dass Off-Chain-Daten nicht manipuliert wurden

## 7.4 Vertrautheitsmodell

### Berechnungseingaben

```
Vertrautheit = f(Interaktionshäufigkeit, Interaktionstiefe, Beziehungsdeklarationen, gemeinsame Teilnahme, Zeitverfall)
```

### Eigenschaften

- Hat eine maximale Obergrenze
- Verfällt bei längerem Ausbleiben von Interaktionen
- Berechnungsprozess ist über On-Chain-Nachweise prüfbar
- Symmetrie noch zu bestimmen (ob A→B gleich B→A ist)

### Vertrautheit-zu-Funktion-Zuordnung

| Vertrautheitsbereich | Vererbungsabschwächung | Abstimmungsausschluss |
|---------------------|------------------------|----------------------|
| Sehr hoch (> 0,9) | Niedrigste | Muss ausgeschlossen werden |
| Hoch (0,7–0,9) | Niedrig | Ausschluss empfohlen |
| Mittel (0,4–0,7) | Moderat | Nicht ausgeschlossen |
| Niedrig (0,1–0,4) | Hoch | Nicht ausgeschlossen |
| Sehr niedrig (< 0,1) | Sehr hoch oder nicht erlaubt | Nicht ausgeschlossen |

## 7.5 Beziehungstypen

- **Blutsverwandtschaft**: Eltern, Kinder, Geschwister
- **Rechtliche Beziehungen**: Ehepartner, Vormund
- **Soziale Beziehungen**: Freunde, Kollegen, Mentor-Schüler
- **Organisatorische Beziehungen**: Beschäftigung, Geschäftspartner

Verschiedene Beziehungstypen können unterschiedliche Vertrautheits-Basiswerte und Verfallsraten haben.

## 7.6 Fälschungsschutz

- Beziehungsdeklarationen erfordern die Bestätigung beider Parteien (bilaterale Signaturen)
- Interaktionsaufzeichnungen werden automatisch vom System generiert, nicht manuell eingegeben
- Ein großes Volumen an Interaktionen innerhalb kurzer Zeit wird als anomal behandelt
- Isolierte hochfrequente Interaktionen zwischen zwei Parteien (ohne gemeinsamen sozialen Kreis) werden als verdächtig behandelt
- Beziehungen müssen bereits on-chain sein, bevor ein Ereignis eintritt (rückwirkende Erfassung für Vererbungszwecke ist nicht erlaubt)

## 7.7 Privatsphärenschutz

- Beziehungsexistenz ist öffentlich (wird für öffentliche Funktionen wie Abstimmungsausschluss verwendet)
- Spezifische Vertrautheitswerte können selektiv offengelegt werden
- Interaktionsdetails sind streng vertraulich
- ZKP wird verwendet, um Berechtigung nachzuweisen, ohne spezifische Beziehungen offenzulegen

## 7.8 Diskussionsnotizen

> Designüberlegungen zum sozialen Beziehungsgraphen:
> - Dies ist das Schlüsselmerkmal, das GMC von einem reinen Token-System unterscheidet
> - Das Datenvolumen ist die größte Herausforderung — ein globaler Sozialgraph ist enorm groß im Umfang
> - Geschichtete Speicherung (On-Chain-Beziehungen + Off-Chain-Details + Verankerungsnachweise) ist der aktuelle ausgewogene Ansatz
> - Die Symmetriefrage bei der Vertrautheit erfordert weitere Diskussion
> - Der Beziehungsgraph selbst erfordert ebenfalls Fälschungsschutzmechanismen
