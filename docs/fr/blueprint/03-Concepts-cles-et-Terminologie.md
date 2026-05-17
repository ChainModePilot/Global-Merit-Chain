# Chapitre 3 : Concepts clés et Terminologie

## 3.1 Vue d'ensemble des relations entre entités

```
Personne physique (archétype humain)
  └── HumanID (unique au monde)
        ├── iFay-1 (lié de manière permanente)
        │     ├── FayID
        │     └── MeritPocket → MeriToken (lots multiples)
        ├── iFay-2 (lié de manière permanente)
        │     ├── FayID
        │     └── MeritPocket → MeriToken (lots multiples)
        └── ...

Organisation / Individu
  └── coFay (relation de propriété, transférable)
        ├── FayID
        └── MeritPocket → MeriToken (lots multiples)
```

## 3.2 Terminologie fondamentale

### MeriToken

L'unité quantitative de contribution. Représente la crédibilité sociale et le pouvoir de vote d'une entité.

- Non-échangeable, non-transférable
- Décroît au fil du temps selon une courbe exponentielle
- Possède une valeur plancher qui ne peut tomber à zéro
- Peut être hérité sous des règles strictes (avec atténuation)

### MeritPocket

Le conteneur de MeriToken, analogue à un portefeuille. Chaque Fay est lié à un MeritPocket.

### iFay (Fay personnel)

Un agent IA personnel — l'« armure numérique ». Lié de manière permanente à une personne physique ; ne peut être délié.

- Essence : Une extension de la personne, pas un actif
- Les MeriToken générés par un iFay appartiennent à son archétype humain
- Une personne peut avoir plusieurs iFay

### coFay (Fay organisationnel)

Un agent IA organisationnel ou commercial. Appartient à un individu ou une organisation.

- Essence : Un actif, transférable
- Les MeriToken générés par un coFay appartiennent à son propriétaire actuel
- Lors d'un transfert, le MeritPocket est transféré avec lui ; le MeriToken n'est pas atténué

### Archétype humain

La personne physique à laquelle un iFay est lié de manière permanente. Chaque archétype humain possède un HumanID unique.

### HumanID / FayID

- HumanID : Un identifiant d'identité humaine unique au monde
- FayID : Un identifiant d'identité Fay unique au monde
- Un HumanID peut correspondre à plusieurs FayID
- HumanID et FayID apparaissent par paires

### Lot de Merit

L'unité d'enregistrement pour chaque acquisition de contribution, contenant : la quantité acquise, la durée d'influence, les paramètres de décroissance et le moment d'acquisition.

### Parties prenantes

Les parties ayant un intérêt direct dans une contribution donnée. Responsables du vote de consensus sur les contributions ; sélectionnées en excluant les individus ayant une intimité excessivement élevée avec le contributeur.

### Cimetière numérique

L'état dans lequel un iFay peut être placé après le décès de son archétype humain. Un iFay dans le cimetière numérique peut encore avoir des interactions passives, mais toutes les actions sont étiquetées « depuis le cimetière numérique ».

## 3.3 Différences essentielles entre iFay et coFay

| Dimension | iFay | coFay |
|-----------|------|-------|
| Essence | Extension de la personne | Actif |
| Relation de liaison | Lié de manière permanente, ne peut être délié | Relation de propriété, transférable |
| Après le décès de l'archétype humain | Entre en tutelle ou au cimetière numérique | Hérité/transféré en tant qu'actif |
| MeriToken lors d'un transfert | Ne peut être transféré | Transféré avec le coFay, sans atténuation |
| Nombre de propriétaires | Appartient à une seule personne physique | Appartient à un individu ou une organisation |

## 3.4 MeriToken et Soulbound Token (SBT)

SBT est un concept proposé par Vitalik Buterin en 2022 — un Token non-transférable lié à une identité spécifique, utilisé pour représenter des attributs qui ne devraient pas être échangés (accréditations, réputation, réalisations).

MeriToken est une version améliorée de SBT :

| Caractéristique | SBT standard | MeriToken |
|-----------------|--------------|-----------|
| Non-transférable | ✓ | ✓ |
| Méthode de liaison | Lié à l'adresse du portefeuille | Lié à iFay → MeritPocket → archétype humain |
| Dimension temporelle | Aucune (valide en permanence) | Oui (décroissance exponentielle) |
| Héritable | Non | Oui (avec atténuation) |
| Méthode de quantification | Typiquement booléen (possède/ne possède pas) | Valeur numérique continue |
| Garantie de plancher | Aucune | Oui (minMerit) |

## 3.5 Notes de discussion

> Logique de conception du système terminologique :
> - La liaison à trois couches (archétype humain → iFay → MeritPocket) isole les couches identité, agent et actif
> - iFay en tant qu'« extension de la personne » est non-transférable, garantissant que la réputation est inséparable de la personne
> - coFay en tant qu'« actif » est transférable, garantissant la flexibilité opérationnelle organisationnelle
> - MeriToken s'inspire de SBT mais ajoute la décroissance temporelle et l'héritabilité, le rendant mieux adapté aux scénarios de mesure dynamique des contributions
