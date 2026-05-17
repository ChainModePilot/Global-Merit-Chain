# Chapitre 7 : Graphe des relations sociales et Intimité

## 7.1 Pourquoi GMC a besoin d'un graphe de relations sociales

GMC ne se contente pas d'enregistrer les contributions — il enregistre également les relations entre les personnes. Ce n'est pas une fonctionnalité accessoire mais le fondement des mécanismes centraux :

| Mécanisme dépendant du graphe de relations | Objectif |
|--------------------------------------------|----------|
| Mécanisme d'héritage | Détermine le ratio d'atténuation (intimité plus élevée = moins d'atténuation) |
| Exclusion des parties prenantes | Exclut les individus trop proches du contributeur lors du vote |
| Anti-fraude | Identifie les schémas de relations anormaux et les comportements de collusion |
| Gouvernance communautaire | Définit les frontières communautaires et les relations d'appartenance |

Sans le graphe de relations, aucun des mécanismes ci-dessus ne peut fonctionner.

## 7.2 Sources de l'intimité

L'intimité découle des interactions entre Fay et du réseau de relations sociales :

- **Fréquence d'interaction** : Fréquence de communication et de collaboration entre deux Fay
- **Profondeur d'interaction** : Complexité et durée des projets collaboratifs
- **Déclarations de relations** : Relations activement déclarées par les utilisateurs (famille, collègues, etc.)
- **Participation conjointe** : Communautés, projets et votes auxquels ils participent ensemble

## 7.3 Stratégie de stockage sur la chaîne

### Pourquoi le stockage sur la chaîne est nécessaire

> Conclusion des discussions : Les relations sociales doivent être stockées sur la chaîne pour garantir l'authenticité des relations et empêcher la fabrication.
>
> Si les données de relations peuvent être falsifiées, les mécanismes tels que l'atténuation d'héritage et l'exclusion de vote échoueront tous.

### Stockage en couches

| Type de données | Emplacement de stockage | Justification |
|-----------------|------------------------|---------------|
| Existence de la relation | Sur la chaîne | Garantit l'infalsifiabilité |
| Valeurs d'intimité | Sur la chaîne | Sert de base pour l'héritage et l'exclusion |
| Preuves de calcul d'intimité | Sur la chaîne (preuves ZK) | Garantit que le calcul est auditable |
| Détails des interactions | Hors chaîne | Volume de données important, implique la vie privée |

### Ancrage hors chaîne vers la chaîne

- Les détails des interactions sont stockés hors chaîne
- Les résultats statistiques sont périodiquement ancrés par hash sur la chaîne
- Des preuves ZK sont soumises lors de la mise à jour de l'intimité
- Quiconque peut vérifier que les données hors chaîne n'ont pas été altérées via le hash

## 7.4 Modèle d'intimité

### Entrées de calcul

```
Intimité = f(fréquence d'interaction, profondeur d'interaction, déclarations de relations, participation conjointe, décroissance temporelle)
```

### Propriétés

- Possède une borne supérieure maximale
- Décroît en cas d'absence prolongée d'interaction
- Le processus de calcul est auditable via des preuves sur la chaîne
- Symétrie à déterminer (si A→B est égal à B→A)

### Correspondance intimité-fonction

| Plage d'intimité | Atténuation d'héritage | Exclusion de vote |
|------------------|------------------------|-------------------|
| Très élevée (> 0,9) | La plus faible | Doit exclure |
| Élevée (0,7–0,9) | Faible | Recommandé d'exclure |
| Moyenne (0,4–0,7) | Modérée | Non exclu |
| Faible (0,1–0,4) | Élevée | Non exclu |
| Très faible (< 0,1) | Très élevée ou interdite | Non exclu |

## 7.5 Types de relations

- **Relations de sang** : Parents, enfants, frères et sœurs
- **Relations légales** : Conjoint, tuteur
- **Relations sociales** : Amis, collègues, mentor-élève
- **Relations organisationnelles** : Emploi, partenaires commerciaux

Différents types de relations peuvent avoir des niveaux de base d'intimité et des taux de décroissance différents.

## 7.6 Anti-falsification

- Les déclarations de relations nécessitent la confirmation des deux parties (signatures bilatérales)
- Les enregistrements d'interaction sont automatiquement générés par le système, non saisis manuellement
- Un grand volume d'interactions dans un court laps de temps est traité comme anomalie
- Des interactions isolées à haute fréquence entre deux parties (sans cercle social partagé) sont traitées comme suspectes
- Les relations doivent déjà être sur la chaîne avant qu'un événement ne se produise (l'enregistrement rétroactif à des fins d'héritage n'est pas autorisé)

## 7.7 Protection de la vie privée

- L'existence des relations est publique (utilisée pour des fonctions publiques telles que l'exclusion de vote)
- Les valeurs d'intimité spécifiques peuvent être divulguées de manière sélective
- Les détails des interactions sont strictement confidentiels
- ZKP est utilisé pour prouver l'éligibilité sans révéler les relations spécifiques

## 7.8 Notes de discussion

> Considérations de conception pour le graphe de relations sociales :
> - C'est la caractéristique clé qui distingue GMC d'un système de Token pur
> - Le volume de données est le plus grand défi — un graphe social mondial est d'une échelle énorme
> - Le stockage en couches (relations sur la chaîne + détails hors chaîne + preuves d'ancrage) est l'approche équilibrée actuelle
> - La question de la symétrie de l'intimité nécessite une discussion plus approfondie
> - Le graphe de relations lui-même nécessite également des mécanismes anti-falsification
