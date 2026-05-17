# Kapitel 6: Beitragsanerkennungsmechanismus

## 6.1 Die zentrale Herausforderung der Anerkennung

Die Beitragsanerkennung ist die kritischste und schwierigste Komponente von GMC. Die zentrale Herausforderung liegt in:

- Beiträge können objektiv (quantifizierbar) oder subjektiv (bewertungsbedürftig) sein
- Objektive Messung ist von Natur aus betrugsresistent, hat aber eine enge Abdeckung
- Subjektive Bewertung hat eine breite Abdeckung, ist aber leicht manipulierbar (ähnlich wie gefälschte Online-Bewertungen)

## 6.2 Zwei Erwerbsmethoden

### Methode 1: Objektive Messung

Basierend auf verifizierbaren objektiven Metriken prägt das System automatisch Merit:

| Messdimension | Beispiele | Merkmale |
|---------------|-----------|----------|
| Nach Volumen | Bediente Kunden, eingereichte Vorschläge | Prüfbar, betrugsresistent |
| Nach Zeit | Servicestunden, Online-Dauer | Zeitstempel sind verifizierbar |
| Nach Output | Code-Commits, erstellte Dokumentation | On-chain nachverfolgbar |

Vorteile: Automatisch, effizient, hohe Betrugsschwierigkeit.
Einschränkungen: Kann nicht alle Arten von Beiträgen abdecken.

### Methode 2: Aufgabenprämie

Voreingestelltes Merit für eine bestimmte Aufgabe; bei Abschluss stimmen Stakeholder zur Bestätigung ab:

1. **Veröffentlichen**: Aufgabenziel, Merit-Belohnung und Einflussdauer definieren
2. **Ausführen**: Der Ausführende erledigt die Aufgabe und reicht Ergebnisse ein
3. **Abstimmen**: Stakeholder stimmen darüber ab, ob die Kriterien erfüllt sind
4. **Prägen**: Bei Genehmigung prägt das System MeriToken

## 6.3 Stakeholder-Mechanismus

### Wer sind die Stakeholder

Parteien mit einem berechtigten Interesse an einem bestimmten Beitrag. Zum Beispiel:
- Beitrag eines Regierungsberatungs-coFay → wird kollektiv von dessen Nutzern bewertet
- Beitrag zu einem Open-Source-Projekt → wird von den Nutzern und Mitarbeitern des Projekts bewertet

### Schlüsselregel: Ausschluss von Personen mit hoher Vertrautheit

Da GMC das soziale Beziehungsnetzwerk erfasst, kann das System:
1. Personen identifizieren, deren Vertrautheit mit dem Beitragenden einen Schwellenwert überschreitet
2. Diese Personen aus dem Abstimmungspool ausschließen
3. Abstimmende aus den verbleibenden Stakeholdern auswählen

Dies ist der Kernmechanismus zur Verhinderung von „Insider stimmen für Insider".

### Konsens-Genehmigungsbedingungen

- Ein Anteilsschwellenwert wird festgelegt (z.B. 2/3-Mehrheit)
- Das Abstimmungsgewicht ist an das eigene MeriToken des Abstimmenden gebunden
- Sobald der Schwellenwert überschritten wird, prägt das System automatisch

## 6.4 Bestimmung der Einflussdauer

Jede Beitragsanerkennung muss auch die Einflussdauer bestimmen:

| Bestimmungsmethode | Anwendbares Szenario |
|--------------------|---------------------|
| Voreingestellt nach Beitragstyp | Objektive Messung (z.B. Kundenservice-Interaktion = 30 Tage) |
| Vom Aufgabenherausgeber festgelegt | Aufgabenprämie |
| Kollektiv von Abstimmenden entschieden | Gemeinschaftskonsens |

Die Einflussdauer bestimmt die Verfallsrate dieser Merit-Charge.

## 6.5 Anti-Betrugs-Strategien

> Kernfrage in der Diskussion: Bitcoin-Mining ist rein objektive Messung, von Natur aus betrugsresistent. Aber GMC beinhaltet subjektive Bewertung — wie verhindert man gefälschte Bewertungen?
>
> Ansatz: Nicht die Subjektivität eliminieren, sondern die Kosten des Betrugs weit über den Nutzen hinaus steigern.

Verteidigungskombination:

1. **Vertrautheitsausschluss**: Abstimmende mit engen Beziehungen zum Bewertungsgegenstand ausschließen
2. **MeriToken-Gewichtung**: Abstimmende mit hoher Reputation haben mehr Gewicht; Betrüger müssen zunächst erhebliche echte Reputation aufbauen
3. **Abstimmungsverhaltens-Audit**: Häufiges Abstimmen zugunsten einer bestimmten Person → als anomal markiert
4. **Zufallsstichprobe**: Zufällige Auswahl von Abstimmenden aus dem Stakeholder-Pool zur Reduzierung der Kollusionsmöglichkeit
5. **Rückwirkende Verantwortlichkeit**: Bei Entdeckung von Betrug kann dieser rückwirkend durch den Strafmechanismus adressiert werden

### Designprinzip

> Beiträge so weit wie möglich in objektiv messbare Komponenten zerlegen und den Anteil subjektiver Bewertung reduzieren:
> - Objektive Messung priorisieren (automatisch, effizient, betrugsresistent)
> - Subjektive Bewertung nur für Szenarien verwenden, die nicht objektiv quantifiziert werden können
> - Subjektive Bewertung setzt mehrere Verteidigungsschichten ein, um das Betrugsrisiko zu reduzieren

## 6.6 Diskussionsnotizen

> Design-Abwägungen bei der Beitragsanerkennung:
> - Effizienz vs. Fairness: Objektive Messung ist effizient, aber eng; subjektive Bewertung ist umfassend, aber manipulationsanfällig
> - Teilnahme vs. Qualität: Die Senkung der Abstimmungsschwelle erhöht die Teilnahme, kann aber die Bewertungsqualität verringern
> - Aktueller Ansatz: „Objektiv zuerst + subjektive Ergänzung + mehrschichtige Verteidigung"
> - Weiterführende Frage: Wie wird Merit aus dem Nichts geschaffen? → Siehe Kapitel Wirtschaftsmodell
