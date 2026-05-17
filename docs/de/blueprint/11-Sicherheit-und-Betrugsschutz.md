# Kapitel 11: Sicherheit und Betrugsschutz

## 11.1 Bedrohungsmodell

| Bedrohung | Beschreibung | Auswirkung |
|-----------|--------------|------------|
| Merit-Farming | Erlangung von MeriToken durch gefälschte Beiträge | Aufgeblähtes Stimmrecht |
| Kollusives Abstimmen | Mehrere Parteien verschwören sich zur Manipulation von Anerkennungsabstimmungen | Unrechtmäßiger Merit-Erwerb |
| Vertrautheits-Farming | Fälschung von Interaktionen zur Steigerung der Vertrautheit | Umgehung von Ausschlüssen, Reduzierung der Vererbungsabschwächung |
| Identitätsfälschung | Erstellung gefälschter HumanIDs | Mehrere Identitäten erwerben mehrere Merit-Anteile |
| Sybil-Angriff | Eine Person kontrolliert mehrere Identitäten | Manipulation von Abstimmungen |

## 11.2 Verhinderung von Merit-Farming

### Schutzmaßnahmen für objektive Messung

- Das System zeichnet automatisch auf, was wenig Raum für menschliche Manipulation lässt
- Kreuzverifizierung ist möglich (z.B. Vergleich von Arbeitsstunden vs. Output)
- Statistische Anomalieerkennung

### Schutzmaßnahmen für subjektive Bewertung

> Kernprinzip: Die Kosten des Betrugs weit über den Nutzen hinaus steigern.

1. **Vertrautheitsausschluss**: Abstimmende mit engen Beziehungen ausschließen
2. **MeriToken-Gewichtung**: Abstimmende mit hoher Reputation haben mehr Gewicht; Betrüger müssen zunächst erhebliche echte Reputation aufbauen
3. **Verhaltens-Audit**: Häufiges Abstimmen zugunsten einer bestimmten Person → als anomal markiert
4. **Zufallsstichprobe**: Zufällige Auswahl von Abstimmenden zur Reduzierung der Kollusionsmöglichkeit
5. **Rückwirkende Verantwortlichkeit**: Bei Entdeckung von Betrug werden alle Beteiligten bestraft

## 11.3 Verhinderung von Vertrautheits-Farming

- Bewertung der Interaktionsqualität (nicht nur Häufigkeit)
- Einseitige Interaktionen sind ungültig (müssen bidirektional sein)
- Große Mengen an Interaktionen in kurzer Zeit werden als anomal behandelt
- Isolierte hochfrequente Interaktionen zwischen zwei Personen (ohne gemeinsamen sozialen Kreis) werden als verdächtig behandelt

## 11.4 Schlüsselsicherheit

- Multi-Signatur-Schemata: Kritische Operationen erfordern Bestätigung durch mehrere Schlüssel
- Schlüsselrotation: Periodischer Austausch
- Soziale Wiederherstellung: Vertrauenswürdige Kontakte unterstützen bei der Wiederherstellung

## 11.5 Privatsphärenschutz

- Abstimmungsinhalte sind nicht öffentlich (ZKP); nur Ergebnisse werden offengelegt
- Vertrautheitswerte können selektiv offengelegt werden
- Interaktionsinhalte werden nicht on-chain gespeichert
- Anonyme Teilnahme wird unterstützt (ZKP beweist Berechtigung ohne Identitätspreisgabe)

## 11.6 Diskussionsnotizen

> Designphilosophie des Sicherheitsmechanismus:
> - Es gibt keine perfekte Anti-Betrugs-Lösung; das Ziel ist, die Kosten des Betrugs weit über den Nutzen hinaus zu steigern
> - Mehrschichtige Verteidigungen sind wirksamer als jeder einzelne Mechanismus
> - Präventive Maßnahmen + rückwirkende Verantwortlichkeit bilden einen geschlossenen Kreislauf
> - Betrugsschutz ist ein kontinuierlicher adversarialer Prozess; das System muss sich weiterentwickeln können
