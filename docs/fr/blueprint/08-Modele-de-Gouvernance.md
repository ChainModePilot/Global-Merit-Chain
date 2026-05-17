# Chapitre 8 : Modèle de Gouvernance

## 8.1 La logique du pouvoir de vote

Dans l'ère post-monétaire, le pouvoir de vote dans la gouvernance sociale ne peut être basé sur la richesse (la monnaie est devenue inefficace), ni sur l'autorité (ce qui viole les principes de décentralisation).

La réponse de GMC : **Le pouvoir de vote découle de la part de contributions au sein d'une communauté.**

Cela signifie :
- Plus vous contribuez et plus votre réputation est élevée, plus votre influence est grande
- Le pouvoir de vote est dynamique, fluctuant avec la décroissance et la croissance de MeriToken
- Sans contributions soutenues, l'influence s'estompe naturellement — il n'y a pas de privilèges permanents

## 8.2 Mécanisme de vote pondéré

```
Votes effectifs individuels = Votes de base × (MeriToken individuel / MeriToken total de la communauté)
```

Chacun a le droit de voter (votes de base = 1), mais le poids est proportionnel à la part de MeriToken.

### Exemple

Une communauté a 3 membres :

| Membre | MeriToken | Part | Votes effectifs |
|--------|-----------|------|-----------------|
| A | 100 | 50% | 0,5 |
| B | 60 | 30% | 0,3 |
| C | 40 | 20% | 0,2 |

A + C votent pour, B vote contre : Pour 0,7 > Contre 0,3 → Adopté.

## 8.3 Scénarios de gouvernance

| Scénario | Votants | Condition d'adoption | Notes |
|----------|---------|---------------------|-------|
| Reconnaissance de contribution | Parties prenantes (excluant haute intimité) | Majorité des 2/3 | Opération courante |
| Décision de sanction | Parties prenantes affectées | Majorité des 3/4 | Un comportement grave nécessite un seuil plus élevé |
| Changement de règles | Tous les membres de la communauté | Majorité absolue des 2/3 | Affecte tout le monde |

## 8.4 Communautés

Les communautés sont les unités de gouvernance dans GMC :

- Une personne peut appartenir à plusieurs communautés
- Les communautés peuvent être imbriquées (sous-communautés)
- Le pouvoir de vote est calculé indépendamment dans chaque communauté
- La même personne peut avoir des niveaux d'influence entièrement différents dans différentes communautés

## 8.5 Anti-monopole

La part de MeriToken détermine le pouvoir de vote, mais la concentration extrême doit être empêchée :

- **Le mécanisme de décroissance est lui-même anti-monopole** : sans contributions soutenues, le pouvoir de vote est perdu
- **Stratification communautaire** : dans les grandes communautés, les parts individuelles sont naturellement diluées
- **Part plutôt que valeur absolue** : les augmentations de l'offre totale n'affectent pas l'équité de la gouvernance

## 8.6 Gouvernance collaborative humain-IA

- Le vote d'un iFay représente la volonté de son archétype humain
- Le vote d'un coFay représente la volonté de son organisation affiliée
- Tout comportement de vote est transparent et auditable sur la chaîne
- Les humains et les Fay opèrent au sein du même cadre de gouvernance

## 8.7 Notes de discussion

> Choix de conception pour le modèle de gouvernance :
> - « Pondéré par la part » plutôt que « une personne, un vote » : le principe central est « les contributions déterminent le pouvoir de vote »
> - « Part » plutôt que « valeur absolue » : empêche les premiers participants de monopoliser l'influence de manière permanente
> - La décroissance est une protection naturelle pour l'équité de la gouvernance
> - Un mécanisme de « plafond de pouvoir de vote » pourrait être nécessaire à l'avenir pour empêcher le contrôle absolu par une seule entité dans les petites communautés
