# Kapitel 5: Identitätssystem

## 5.1 Warum ein dediziertes Identitätssystem benötigt wird

Identität in GMC unterscheidet sich von traditionellen Internet-Konten:

- Sie ist an die lebenslange Reputation einer natürlichen Person gebunden und kann nicht willkürlich erstellt oder verworfen werden
- Sie muss die permanente Bindung von iFay und die Eigentumsübertragung von coFay unterstützen
- Sie muss in einer dezentralen Umgebung verifizierbar sein und gleichzeitig die Privatsphäre schützen

## 5.2 Identitätsschichten

```
┌─────────────────────────────────────┐
│  Schicht 1: Natürliche Person        │  ← Eindeutig, lebenslang
│             (HumanID)                │
├─────────────────────────────────────┤
│  Schicht 2: Fay-Identität (FayID)    │  ← Gepaart mit HumanID
├─────────────────────────────────────┤
│  Schicht 3: Vermögensschicht         │  ← An FayID gebunden
│             (MeritPocket)            │
└─────────────────────────────────────┘
```

### HumanID

- Global eindeutig, identifiziert eine natürliche Person
- Eine HumanID kann mehreren FayIDs entsprechen
- Lebenslang gültig, kann nicht abgemeldet werden (kann aber in den Friedhofszustand übergehen)

### FayID

- Global eindeutig, identifiziert einen Fay
- Jede FayID ist mit einem MeritPocket verknüpft
- Die FayID eines iFay ist permanent an eine HumanID gebunden
- Die FayID-Eigentümerschaft eines coFay kann übertragen werden

## 5.3 On-Chain-Verifizierungsschema

### Schemavergleich

| Schema | Prinzip | Vorteile | Nachteile | Anwendbare Szenarien |
|--------|---------|----------|-----------|---------------------|
| PKI (Public-Private-Key-Paar) | Schlüsselpaar-Signaturverifizierung | Ausgereift, effizient, dezentral | Verlust des privaten Schlüssels = Identitätsverlust | Grundlegende Signaturen |
| DID (Dezentrale Identität) | W3C-Standard, On-Chain-Identitätsdokumente | Standardisiert, unterstützt Schlüsselwiederherstellung | Relativ komplex | Beziehungszuordnung |
| ZKP (Zero-Knowledge-Beweis) | Beweist Identität ohne Informationspreisgabe | Extrem starker Privatsphärenschutz | Hoher Rechenaufwand | Privatsphäre-Szenarien |

### Empfehlung: Geschichtete Kombination

1. **Basisschicht (grundlegende Verifizierung)**: PKI
   - Signaturmechanismus für alle On-Chain-Operationen
   - Jede HumanID und FayID hat ein Schlüsselpaar

2. **Mittlere Schicht (Beziehungsmanagement)**: DID
   - Verwaltet HumanID ↔ FayID-Bindungsbeziehungen
   - Unterstützt Schlüsselrotation und soziale Wiederherstellung
   - Speichert Identitätsmetadaten

3. **Obere Schicht (Privatsphäre-Szenarien)**: ZKP
   - Beweist Identität bei Abstimmungen, ohne preiszugeben, wer man ist
   - Verifiziert Beziehungen bei der Erbschaftsauthentifizierung, ohne Details offenzulegen
   - Schützt Hinweisgeber bei Strafbeschwerden

### Begründung

> Jedes einzelne Schema hat Einschränkungen:
> - Reines PKI kann Schlüsselverlust nicht lösen und bietet keinen Privatsphärenschutz
> - Reines DID hat unzureichende Leistung für hochfrequente Verifizierung
> - Reines ZKP hat übermäßige Rechenkosten
>
> Eine geschichtete Kombination lässt jede Schicht sich auf die Szenarien konzentrieren, die sie am besten bewältigt.

## 5.4 iFay-Lebenszyklus

```
Erstellung → Bindung an menschlichen Archetyp → Normalbetrieb → [Menschlicher Archetyp verstirbt] → Vormundschaft / Digitaler Friedhof
```

### Normalbetrieb

- iFay handelt im Auftrag des menschlichen Archetyps
- Alles generierte MeriToken gehört dem menschlichen Archetyp
- Der menschliche Archetyp nimmt über iFay an Abstimmungen, Beitragsanerkennung usw. teil

### Vormundschaft

Wenn der menschliche Archetyp verstirbt:
- Ein Erbe kann die Vormundschaft beantragen
- Der Vormund kann im Namen des Verstorbenen verwalten, aber **nicht in der Identität des menschlichen Archetyps handeln**
- Alle Vormundschaftshandlungen müssen die Informationen des Vormunds anzeigen
- Es gibt eine explizite Vormundschaftsmarkierung on-chain

### Digitaler Friedhof

- Ein iFay kann nach der Überführung in den Friedhof noch passive Interaktionen haben
- Alle Interaktionen werden als „vom digitalen Friedhof" gekennzeichnet
- Es wird kein neues MeriToken aktiv generiert
- Bestehendes MeriToken verfällt weiterhin normal

## 5.5 coFay-Eigentumsübertragung

Als Vermögenswert folgt coFay diesen Übertragungsregeln:

1. Das MeritPocket wird mit dem coFay übertragen; MeriToken wird nicht abgeschwächt
2. Übertragungsaufzeichnungen werden on-chain gespeichert; die Eigentumsänderungshistorie ist manipulationssicher
3. Die Übertragung erfordert eine beidseitige Signaturbestätigung
4. Die Stimmrecht-Kontinuität des coFay wird durch die Übertragung nicht beeinträchtigt

## 5.6 Sybil-Angriff-Prävention

Ein-Person-mehrere-Konten ist eine klassische Bedrohung für dezentrale Identitätssysteme:

- Die HumanID-Registrierung erfordert einen Eindeutigkeitsnachweis (spezifische Methode noch zu bestimmen)
- Sozialgraph-Analyse: Echte Nutzer haben natürliche soziale Netzwerke; gefälschte Konten zeigen abnormale Muster
- Verhaltensmusteranalyse: Mehrere von derselben Person kontrollierte Konten teilen ähnliche Verhaltensmerkmale
- Progressive Vertrauensbildung: Berechtigungen und Einfluss neuer Nutzer werden schrittweise freigegeben

## 5.7 Diskussionsnotizen

> Kern-Abwägungen im Identitätssystem:
> - Sicherheit vs. Benutzerfreundlichkeit: Dreischichtige Verifizierung erhöht die Sicherheit, aber auch die Komplexität
> - Privatsphäre vs. Transparenz: ZKP schützt die Privatsphäre; On-Chain-Aufzeichnungen gewährleisten Transparenz
> - Permanenz vs. Flexibilität: Die permanente Bindung von iFay stellt sicher, dass Reputation untrennbar von der Person ist; die Übertragbarkeit von coFay gewährleistet kommerzielle Flexibilität
> - Sybil-Angriff-Prävention ist eine ewige Herausforderung für dezentrale Identität und erfordert eine Kombination mehrerer Ansätze
