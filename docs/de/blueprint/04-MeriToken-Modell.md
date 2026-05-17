# Kapitel 4: MeriToken-Modell

## 4.1 Modellübersicht

MeriToken ist die zentrale Messeinheit von GMC. Sein Design muss eine Schlüsselfrage beantworten: **Wie kann die Beitragsmessung aktuelle Aktivität widerspiegeln und gleichzeitig historische Beiträge respektieren?**

Die Antwort lautet: Exponentieller Verfall + Mindestwert ungleich null.

## 4.2 Zwei Schlüsselwerte

Jedes MeritPocket pflegt zwei Kernwerte:

- **curMerit** (aktuelles MeriToken): Der Echtzeit-Beitragsmesswert; verfällt über die Zeit und wächst mit neuen Beiträgen
- **minMerit** (Mindestwert): Die untere Grenze des Verfalls, repräsentiert das langfristige Sediment historischer Beiträge; steigt nur (außer bei Strafen)

```
curMerit ≥ minMerit ≥ e (Anfangswert)
```

## 4.3 Erwerb

MeriToken wird durch Beiträge erworben; das System prägt neue Token:

| Erwerbsmethode | Beschreibung | Auslösebedingung |
|----------------|--------------|------------------|
| Objektive Messung | Automatisch berechnet auf Basis verifizierbarer Metriken | System zeichnet automatisch Schwellenwertüberschreitung auf |
| Aufgabenprämie | Voreingestelltes Merit für eine bestimmte Aufgabe | Stakeholder stimmen bei Abschluss zu |
| Anfangszuteilung | Gewährt bei Netzwerkregistrierung | Identitätsregistrierung abgeschlossen |

Anfangswert = e ≈ 2,718 (die natürliche Konstante, natürlich abgestimmt auf das exponentielle Verfallsmodell).

## 4.4 Verfallsmodell

### Kernidee

Jede Merit-Erwerbscharge hat eine unabhängige **Einflussdauer**. Die Einflussdauer spiegelt die Aktualität dieses Beitrags wider — ein Beitrag mit 100 Tagen Einfluss hat sein Merit innerhalb von 100 Tagen vollständig verfallen.

### Einzelchargen-Verfallsformel

```
MeriToken_i(t) = (V_i - B_i) × e^(-λ_i × t) + B_i
```

- `V_i`: Anfänglicher Merit-Wert der Charge
- `B_i`: Beitrag der Charge zum Mindestwert
- `λ_i`: Verfallskoeffizient, bestimmt durch Einflussdauer T_i (λ_i = k / T_i, wobei k eine Konstante ist)
- `t`: Seit dem Erwerb verstrichene Zeit

### Gesamtes aktuelles MeriToken

```
curMerit = Σ MeriToken_i(t)  (Summe aller aktiven Chargen)
```

Wenn alle Chargen vollständig verfallen sind, nähert sich curMerit dem minMerit an.

## 4.5 Mindestwert (minMerit)

### Aktualisierungsregel

Bei jedem neuen Merit-Erwerb wird der Mindestwert aktualisiert:

Sei aktuelles curMerit = M, neu erworbenes Merit = x, aktueller Mindestwert = B, dann:

```
Neuer Mindestwert B' = (x + M) × B / M
```

Bedeutung: Der Mindestwert wächst proportional zum Anteil des neuen Merit am Gesamtwert.

### Eigenschaften

- Startwert = e ≈ 2,718
- Steigt nur (außer bei Strafen)
- Repräsentiert das unauslöschliche Sediment historischer Beiträge
- Selbst wenn Beiträge vollständig eingestellt werden, wird curMerit letztlich nie unter minMerit fallen

### Grenzfall

Wenn curMerit = minMerit (d.h. im Mindestzustand) und neues Merit x erworben wird:
```
B' = (x + B) × B / B = x + B
```
Der Mindestwert steigt direkt um x — das bedeutet, im Mindestzustand erworbenes Merit wird vollständig als Mindestwert hinterlegt.

## 4.6 Implementierung des unabhängigen Verfalls pro Charge

### Herausforderungen

- Jedes MeritPocket muss eine Liste von Merit-Chargen pflegen
- Die Abfrage des aktuellen Werts erfordert die Iteration über alle noch nicht vollständig verfallenen Chargen
- On-Chain-Speicher- und Berechnungskosten wachsen linear mit der Anzahl der Chargen

### Optimierungsstrategien

1. **Chargenzusammenführung**: Chargen mit ähnlicher Einflussdauer werden periodisch zusammengeführt, um die Anzahl aktiver Chargen zu reduzieren
2. **Off-Chain-Berechnung**: Rollup zur Off-Chain-Berechnung von Echtzeitwerten nutzen; nur Snapshots und Nachweise on-chain speichern
3. **Chargensedimentation**: Bei Überschreitung der maximalen aktiven Chargenanzahl werden die ältesten Chargen automatisch in den Mindestwert sedimentiert
4. **Verzögerte Berechnung**: Präzise Werte werden nur bei Bedarf berechnet (z.B. bei Abstimmungen oder Abfragen)

## 4.7 Designphilosophie

### Warum exponentieller Verfall?

- Fördert kontinuierliche Beiträge statt eines einzelnen großen Beitrags gefolgt von Inaktivität
- Spiegelt die Aktualität von Beiträgen wider — neuere Beiträge haben größeren Einfluss auf die aktuelle Reputation
- Simuliert natürlich den Verfall des sozialen Gedächtnisses
- Verfällt anfangs schnell und verlangsamt sich später, was der Intuition entspricht

### Warum ein Mindestwert ungleich null?

- Anerkennt den langfristigen Wert historischer Beiträge — vergangene Leistungen werden nicht vollständig auf null gesetzt
- Verhindert, dass langfristige Beitragende durch eine kurze Pause ihr gesamtes Stimmrecht verlieren
- Der Mindestwert wächst mit kumulativen Beiträgen und belohnt nachhaltige Teilnahme

### Warum unabhängige Einflussdauer pro Charge?

- Verschiedene Beiträge haben naturgemäß unterschiedliche Aktualität
- Eine einzelne Kundenservice-Interaktion mag nur 30 Tage Einfluss haben
- Die Pflege eines Open-Source-Projekts mag jahrelangen Einfluss haben
- Eine einheitliche Verfallsrate würde den Wert verschiedener Beitragsarten verzerren

## 4.8 Diskussionsnotizen

> Schlüsselentscheidungen im MeriToken-Modell:
> - Exponentieller Verfall + Mindestwert ungleich null: Findet eine Balance zwischen „Förderung kontinuierlicher Teilnahme" und „Respektierung historischer Beiträge"
> - Unabhängige Einflussdauer pro Charge: Erhöht die Implementierungskomplexität, spiegelt aber Unterschiede in der Beitragsaktualität genauer wider
> - Mindestwert steigt nur (außer bei Strafen): Schützt die Grundrechte langfristiger Beitragender
> - Anfangswert von e: Verbindet mathematische Eleganz mit praktischer Bedeutung
>
> Noch zu prüfen: Ob die Mindestwert-Aktualisierungsformel unter Extrembedingungen vernünftig funktioniert
