# Chapitre 9 : Mécanisme de Sanction et d'Appel

## 9.1 Pourquoi les sanctions sont nécessaires

Tout système de réputation nécessite la capacité de corriger les erreurs. Lorsque des contributions sont incorrectement reconnues ou qu'une fraude existe, le système doit pouvoir effectuer des corrections.

Le mécanisme de sanction est la garantie ultime de la crédibilité de GMC.

## 9.2 Types de sanctions

| Type | Effet | Sévérité |
|------|-------|----------|
| Déduction de curMerit | Réduit le MeriToken actuel, affectant le pouvoir de vote immédiat | Plus légère |
| Déduction de minMerit | Abaisse la valeur plancher, affectant la garantie de réputation minimale à long terme | Sévère |

La déduction de minMerit est une sanction plus sévère — elle enfreint la règle selon laquelle « la valeur plancher ne fait qu'augmenter, jamais diminuer », ce qui signifie que l'accumulation des contributions historiques est partiellement révoquée.

### Référence de sévérité

| Niveau de violation | Méthode de sanction | Exemple |
|--------------------|---------------------|---------|
| Mineure | Déduction partielle de curMerit | Contributions exagérées |
| Modérée | Déduction significative de curMerit | Soumissions en double |
| Grave | curMerit + minMerit partiel | Collusion pour accumuler du Merit |
| Extrême | Déduction majeure des deux | Fraude systématique |

## 9.3 Processus de déclenchement

```
Plainte déposée → Vote d'acceptation des parties prenantes → [Rejeté si non adopté] → Vote de sanction → Exécution
```

### Règles

1. **Les plaintes doivent cibler un lot d'acquisition de Merit spécifique** : les plaintes vagues ne sont pas autorisées ; elles doivent pointer vers un événement spécifique
2. **Acceptation par les parties prenantes** : une certaine proportion de parties prenantes concernées doit accepter la plainte avant qu'un vote formel ne soit initié
3. **Vote de sanction** : nécessite un seuil d'adoption plus élevé (par exemple, majorité des 3/4)
4. **Exécution automatique** : une fois le vote adopté, le système applique automatiquement la déduction

### Prévention des plaintes malveillantes

- Les plaignants doivent fournir des preuves ou une justification
- Les plaignants malveillants fréquents peuvent être signalés
- Les enregistrements de plaintes eux-mêmes sont stockés sur la chaîne, garantissant la transparence

## 9.4 Appels

La partie sanctionnée a le droit de faire appel :

1. Un appel peut être déposé dans un certain délai après l'exécution de la sanction
2. Un groupe plus large de membres de la communauté re-vote (pour éviter que le même groupe juge à répétition)
3. Si l'appel réussit, la sanction est révoquée et le MeriToken est restauré

## 9.5 Interaction avec les autres mécanismes

- **Les sanctions sont le seul mécanisme pouvant réduire minMerit** (en dehors de la décroissance naturelle)
- Les enregistrements de sanctions sont stockés sur la chaîne, incluant l'entité sanctionnée, la raison, le montant et les résultats du vote
- L'historique des sanctions affecte la réputation sociale de l'entité (visible par les autres)

## 9.6 Notes de discussion

> Philosophie de conception du mécanisme de sanction :
> - Doit être basé sur des preuves (ciblant des lots spécifiques), empêchant les « accusations sans fondement »
> - Les sanctions graduées reflètent le principe de proportionnalité
> - Les plaintes nécessitent un seuil (acceptation par les parties prenantes), empêchant le harcèlement malveillant
> - Le droit d'appel protège l'équité ; élargir le périmètre empêche les effets de chambre d'écho
> - Le fait que minMerit puisse être réduit par les sanctions sert de dissuasion la plus forte contre les violations d'intégrité
