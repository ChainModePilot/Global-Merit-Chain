# Kapitel 9: Straf- und Einspruchsmechanismus

## 9.1 Warum Strafen benötigt werden

Jedes Reputationssystem erfordert die Fähigkeit, Fehler zu korrigieren. Wenn Beiträge fälschlicherweise anerkannt werden oder Betrug vorliegt, muss das System Korrekturen vornehmen können.

Der Strafmechanismus ist die ultimative Absicherung der Glaubwürdigkeit von GMC.

## 9.2 Straftypen

| Typ | Wirkung | Schweregrad |
|-----|---------|-------------|
| curMerit abziehen | Reduziert aktuelles MeriToken, beeinflusst unmittelbares Stimmrecht | Leichter |
| minMerit abziehen | Senkt den Mindestwert, beeinflusst langfristige Mindest-Reputationsgarantie | Schwer |

Das Abziehen von minMerit ist eine schwerere Strafe — es bricht die Regel, dass „der Mindestwert nur steigt, nie sinkt", was bedeutet, dass die Akkumulation historischer Beiträge teilweise widerrufen wird.

### Schweregrad-Referenz

| Verstoßebene | Strafmethode | Beispiel |
|--------------|--------------|----------|
| Geringfügig | Teilweises curMerit abziehen | Übertriebene Beiträge |
| Moderat | Erhebliches curMerit abziehen | Doppelte Einreichungen |
| Schwer | curMerit + teilweises minMerit | Kollusion zum Merit-Farming |
| Extrem | Erheblicher Abzug beider | Systematischer Betrug |

## 9.3 Auslöseprozess

```
Beschwerde eingereicht → Stakeholder-Annahmevotum → [Abgelehnt wenn nicht bestanden] → Strafabstimmung → Vollstreckung
```

### Regeln

1. **Beschwerden müssen auf eine bestimmte Merit-Erwerbscharge abzielen**: Vage Beschwerden sind nicht erlaubt; sie müssen auf ein bestimmtes Ereignis verweisen
2. **Stakeholder-Annahme**: Ein bestimmter Anteil relevanter Stakeholder muss die Beschwerde annehmen, bevor eine formelle Abstimmung eingeleitet wird
3. **Strafabstimmung**: Erfordert eine höhere Annahmeschwelle (z.B. 3/4-Mehrheit)
4. **Automatische Vollstreckung**: Sobald die Abstimmung bestanden ist, wendet das System den Abzug automatisch an

### Verhinderung böswilliger Beschwerden

- Beschwerdeführer müssen Beweise oder Begründungen vorlegen
- Häufig böswillige Beschwerdeführer können markiert werden
- Beschwerdeaufzeichnungen selbst werden on-chain gespeichert und gewährleisten Transparenz

## 9.4 Einsprüche

Die bestrafte Partei hat das Recht auf Einspruch:

1. Ein Einspruch kann innerhalb eines bestimmten Zeitraums nach der Strafvollstreckung eingereicht werden
2. Eine breitere Gruppe von Gemeinschaftsmitgliedern stimmt erneut ab (um zu vermeiden, dass dieselbe Gruppe wiederholt urteilt)
3. Wenn der Einspruch erfolgreich ist, wird die Strafe aufgehoben und MeriToken wiederhergestellt

## 9.5 Interaktion mit anderen Mechanismen

- **Strafen sind der einzige Mechanismus, der minMerit reduzieren kann** (abgesehen vom natürlichen Verfall)
- Strafaufzeichnungen werden on-chain gespeichert, einschließlich der bestraften Entität, des Grundes, des Betrags und der Abstimmungsergebnisse
- Die Strafhistorie beeinflusst die soziale Reputation der Entität (für andere einsehbar)

## 9.6 Diskussionsnotizen

> Designphilosophie des Strafmechanismus:
> - Muss evidenzbasiert sein (auf bestimmte Chargen abzielend), verhindert „grundlose Anschuldigungen"
> - Abgestufte Strafen spiegeln das Verhältnismäßigkeitsprinzip wider
> - Beschwerden erfordern eine Schwelle (Stakeholder-Annahme), verhindert böswillige Belästigung
> - Das Einspruchsrecht sichert Fairness; die Erweiterung des Kreises verhindert Echokammer-Effekte
> - Die Tatsache, dass minMerit durch Strafen reduziert werden kann, dient als stärkste Abschreckung gegen Integritätsverletzungen
