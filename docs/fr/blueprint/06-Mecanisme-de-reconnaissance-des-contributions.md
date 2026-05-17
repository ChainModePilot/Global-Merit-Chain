# Chapitre 6 : Mécanisme de reconnaissance des contributions

## 6.1 Le défi central de la reconnaissance

La reconnaissance des contributions est le composant le plus critique et le plus difficile de GMC. Le défi central réside dans :

- Les contributions peuvent être objectives (quantifiables) ou subjectives (nécessitant une évaluation)
- La mesure objective est naturellement résistante à la fraude mais a une couverture étroite
- L'évaluation subjective a une couverture large mais est facilement manipulable (similaire aux faux avis en ligne)

## 6.2 Deux méthodes d'acquisition

### Méthode 1 : Mesure objective

Basée sur des métriques objectives vérifiables, le système frappe automatiquement du Merit :

| Dimension de mesure | Exemples | Caractéristiques |
|--------------------|----------|------------------|
| Par volume | Clients servis, propositions livrées | Auditable, résistant à la fraude |
| Par temps | Heures de service, durée en ligne | Les horodatages sont vérifiables |
| Par production | Commits de code, documentation produite | Traçable sur la chaîne |

Avantages : Automatique, efficace, difficulté élevée de fraude.
Limitations : Ne peut couvrir tous les types de contributions.

### Méthode 2 : Prime de tâche

Merit prédéfini pour une tâche spécifique ; à l'achèvement, les parties prenantes votent pour confirmer :

1. **Publication** : Définir l'objectif de la tâche, la récompense en Merit et la durée d'influence
2. **Exécution** : L'exécutant complète la tâche et soumet les résultats
3. **Vote** : Les parties prenantes votent pour déterminer si les critères sont remplis
4. **Frappe** : Après approbation, le système frappe des MeriToken

## 6.3 Mécanisme des parties prenantes

### Qui sont les parties prenantes

Les parties ayant un intérêt direct dans une contribution donnée. Par exemple :
- La contribution d'un coFay de consultation gouvernementale → votée collectivement par ses utilisateurs
- Une contribution à un projet open-source → votée par les utilisateurs et collaborateurs du projet

### Règle clé : Exclure les individus à haute intimité

Puisque GMC enregistre le réseau de relations sociales, le système peut :
1. Identifier les individus dont l'intimité avec le contributeur dépasse un seuil
2. Exclure ces individus du pool de votants
3. Sélectionner les votants parmi les parties prenantes restantes

C'est le mécanisme central pour empêcher les « initiés votant pour les initiés ».

### Conditions d'approbation par consensus

- Un seuil de proportion est fixé (par exemple, majorité des 2/3)
- Le poids du vote est lié au MeriToken propre du votant
- Une fois le seuil dépassé, le système frappe automatiquement

## 6.4 Détermination de la durée d'influence

Chaque reconnaissance de contribution doit également déterminer la durée d'influence :

| Méthode de détermination | Scénario applicable |
|--------------------------|---------------------|
| Prédéfinie par type de contribution | Mesure objective (par exemple, interaction de service client = 30 jours) |
| Fixée par le publieur de la tâche | Prime de tâche |
| Décidée collectivement par les votants | Consensus communautaire |

La durée d'influence détermine le taux de décroissance de ce lot de Merit.

## 6.5 Stratégies anti-fraude

> Question centrale en discussion : Le minage de Bitcoin est une mesure purement objective, naturellement résistante à la fraude. Mais GMC inclut l'évaluation subjective — comment empêcher les faux avis ?
>
> Approche : Non pas éliminer la subjectivité, mais rendre le coût de la fraude bien supérieur au bénéfice.

Combinaison de défenses :

1. **Exclusion par intimité** : Exclure les votants ayant des relations proches avec le sujet évalué
2. **Pondération par MeriToken** : Les votants à haute réputation ont plus de poids ; les fraudeurs doivent d'abord accumuler une réputation authentique substantielle
3. **Audit du comportement de vote** : Voter fréquemment en faveur d'un sujet spécifique → signalé comme anomalie
4. **Échantillonnage aléatoire** : Sélectionner aléatoirement les votants dans le pool de parties prenantes pour réduire la possibilité de collusion
5. **Responsabilité rétroactive** : Si une fraude est découverte, elle peut être traitée rétroactivement par le mécanisme de sanction

### Principe de conception

> Décomposer les contributions en composants objectivement mesurables autant que possible, réduisant la proportion d'évaluation subjective :
> - Prioriser la mesure objective (automatique, efficace, résistante à la fraude)
> - L'évaluation subjective n'est utilisée que pour les scénarios qui ne peuvent être objectivement quantifiés
> - L'évaluation subjective emploie plusieurs couches de défense pour réduire le risque de fraude

## 6.6 Notes de discussion

> Compromis de conception dans la reconnaissance des contributions :
> - Efficacité vs. équité : La mesure objective est efficace mais étroite ; l'évaluation subjective est complète mais susceptible de manipulation
> - Participation vs. qualité : Abaisser le seuil de vote augmente la participation mais peut réduire la qualité de l'évaluation
> - Approche actuelle : « Objectif d'abord + complément subjectif + défense multicouche »
> - Question étendue : Comment le Merit est-il créé à partir de rien ? → Voir le chapitre Modèle Économique
