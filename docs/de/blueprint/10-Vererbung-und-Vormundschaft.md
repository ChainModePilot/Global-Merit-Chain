# Kapitel 10: Vererbung und Vormundschaft

## 10.1 Hintergrund

Nach dem Tod eines menschlichen Archetyps müssen dessen angesammeltes MeriToken und iFay angemessen behandelt werden. Die zentrale Spannung:

- Respektierung der historischen Beiträge des Verstorbenen
- Verhinderung, dass nicht verwandte Personen unverhältnismäßiges Stimmrecht erlangen
- Aufrechterhaltung des Prinzips, dass „Reputation nicht durch Geburtsrecht vererbt werden kann"

## 10.2 Vererbungsregeln

### Vererbbar vs. Nicht vererbbar

| Vererbbar | Nicht vererbbar |
|-----------|-----------------|
| MeriToken (mit Abschwächung) | Identität des iFay |
| Vormundschaft über MeritPocket | Das Recht, unter der Identität des Verstorbenen zu handeln |
| Eigentum an coFay | Die Bindung zwischen iFay und menschlichem Archetyp |

### Abschwächungsmechanismus

```
Vererbtes MeriToken = curMerit des Verstorbenen × Vererbungskoeffizient
Vererbungskoeffizient = f(Vertrautheit)  ← Höhere Vertrautheit bedeutet weniger Abschwächung
```

- Personen mit extrem niedriger Vertrautheit dürfen nicht erben
- Vererbtes MeriToken verfällt ebenfalls normal
- Vererbung erhöht das minMerit des Erben (aber die Erhöhung unterliegt ebenfalls der Abschwächung)

### Warum Abschwächung notwendig ist

- MeriToken repräsentiert persönliche Beiträge; der Erbe ist nicht der Urheber
- Vererbung ohne Abschwächung würde zu „Reputation durch Geburtsrecht" führen, was die Gründungsprinzipien von GMC verletzt
- Das Abschwächungsverhältnis ist an die Vertrautheit gekoppelt: Enge Beziehungen selbst spiegeln sozialen Beitrag wider
- MeriToken verfällt bereits natürlich; Vererbungsabschwächung darüber hinaus stellt sicher, dass der Einfluss schnell schwindet

## 10.3 Identitätsverifizierung des Erben

1. **Beziehungsverifizierung**: Validiert durch den On-Chain-Sozialen-Beziehungsgraphen
2. **Vertrautheitsbestätigung**: Wert bestätigen und Abschwächungsverhältnis berechnen
3. **Mehrseitige Bezeugung**: Gemeinsame Kontakte bezeugen und bestätigen
4. **Abkühlungsperiode**: Ermöglicht Einsprüche

### Verhinderung von Vererbungsbetrug

- Beziehungen müssen zu Lebzeiten des Verstorbenen on-chain erfasst worden sein
- Rückwirkende Ergänzungen sind nicht erlaubt
- Vertrautheit basiert auf historischen Interaktionsdaten und kann nicht kurzfristig gefälscht werden

## 10.4 Vormundschaft

Vormundschaft ≠ Identitätsvererbung. Ein Vormund kann einen iFay verwalten, aber nicht unter der Identität des Verstorbenen handeln.

| Ein Vormund kann | Ein Vormund kann nicht |
|------------------|----------------------|
| Den täglichen Betrieb des iFay verwalten | Aussagen unter der Identität des Verstorbenen machen |
| Entscheiden, ob der iFay in den digitalen Friedhof überführt wird | Unter der Identität des Verstorbenen abstimmen |
| Unerledigte Angelegenheiten bearbeiten | Merit unter der Identität des Verstorbenen erwerben |

Alle Vormundschaftshandlungen werden on-chain markiert, wobei der Operator als Vormund identifiziert wird.

## 10.5 Digitaler Friedhof

- Nach dem Tod eines menschlichen Archetyps kann dessen iFay in den digitalen Friedhof überführt werden
- Passive Interaktionen können weiterhin stattfinden, werden aber als „vom digitalen Friedhof" gekennzeichnet
- Es wird kein neues MeriToken aktiv generiert
- Bestehendes MeriToken verfällt weiterhin und nähert sich schließlich dem minMerit an

## 10.6 Vererbung von coFay

Als Vermögenswert folgt coFay der Vermögensvererbungslogik:
- Das Eigentum wird auf den Erben übertragen
- MeriToken wird nicht abgeschwächt (weil die Beiträge vom coFay selbst generiert wurden)
- Der fundamentale Unterschied: Was vererbt wird, ist „Vermögenseigentum", nicht „persönliche Reputation"

## 10.7 Diskussionsnotizen

> Designphilosophie des Vererbungsmechanismus:
> - Zentrale Spannung: Respektierung der Beiträge des Verstorbenen vs. Verhinderung von Reputation durch Geburtsrecht
> - Lösung: Vererbung erlauben, aber Abschwächung durchsetzen, wobei das Abschwächungsverhältnis durch objektive Vertrautheit bestimmt wird
> - Die Nicht-Übertragbarkeit von iFay garantiert das Prinzip, dass „Persönlichkeit nicht vererbt werden kann"
> - Der digitale Friedhof bietet einen Rahmen für den Umgang mit „digitalem Erbe" im KI-Zeitalter
> - coFay-Vererbung hat keine Abschwächung, weil coFay ein Vermögenswert ist, keine Persönlichkeit
