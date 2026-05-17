# Chapitre 10 : Héritage et Tutelle

## 10.1 Contexte

Après le décès d'un archétype humain, ses MeriToken accumulés et son iFay doivent être correctement gérés. La tension centrale :

- Respecter les contributions historiques du défunt
- Empêcher des individus non liés d'obtenir un pouvoir de vote disproportionné
- Maintenir le principe selon lequel « la réputation ne peut être héritée par droit de naissance »

## 10.2 Règles d'héritage

### Héritable vs. Non-héritable

| Héritable | Non-héritable |
|-----------|---------------|
| MeriToken (avec atténuation) | Identité d'iFay |
| Tutelle du MeritPocket | Le droit d'agir sous l'identité du défunt |
| Propriété de coFay | La liaison entre iFay et l'archétype humain |

### Mécanisme d'atténuation

```
MeriToken hérité = curMerit du défunt × Coefficient d'héritage
Coefficient d'héritage = f(intimité)  ← Intimité plus élevée signifie moins d'atténuation
```

- Les individus avec une intimité extrêmement faible ne sont pas autorisés à hériter
- Les MeriToken hérités décroissent également normalement
- L'héritage augmente le minMerit de l'héritier (mais l'augmentation est également soumise à l'atténuation)

### Pourquoi l'atténuation est nécessaire

- MeriToken représente des contributions personnelles ; l'héritier n'est pas le créateur
- L'héritage sans atténuation conduirait à une « réputation par droit de naissance », violant les principes fondateurs de GMC
- Le ratio d'atténuation est lié à l'intimité : les relations proches reflètent elles-mêmes une contribution sociale
- MeriToken décroît déjà naturellement ; l'atténuation d'héritage en plus de cela garantit que l'influence s'estompe rapidement

## 10.3 Vérification de l'identité de l'héritier

1. **Vérification de la relation** : validée par le graphe de relations sociales sur la chaîne
2. **Confirmation de l'intimité** : confirmer la valeur et calculer le ratio d'atténuation
3. **Témoignage multipartite** : les contacts mutuels témoignent et confirment
4. **Période de refroidissement** : permet les objections

### Prévention de la fraude à l'héritage

- Les relations doivent avoir été enregistrées sur la chaîne du vivant du défunt
- Les ajouts rétroactifs ne sont pas autorisés
- L'intimité est basée sur les données d'interaction historiques et ne peut être fabriquée à court terme

## 10.4 Tutelle

La tutelle ≠ hériter de l'identité. Un tuteur peut gérer un iFay mais ne peut pas agir sous l'identité du défunt.

| Un tuteur peut | Un tuteur ne peut pas |
|----------------|----------------------|
| Gérer les opérations quotidiennes d'iFay | Faire des déclarations sous l'identité du défunt |
| Décider de déplacer iFay vers le cimetière numérique | Voter sous l'identité du défunt |
| Gérer les affaires inachevées | Acquérir du Merit sous l'identité du défunt |

Toutes les actions de tutelle sont marquées sur la chaîne avec l'opérateur identifié comme le tuteur.

## 10.5 Cimetière numérique

- Après le décès d'un archétype humain, son iFay peut être déplacé dans le cimetière numérique
- Des interactions passives peuvent encore se produire, mais sont étiquetées « depuis le cimetière numérique »
- Aucun nouveau MeriToken n'est activement généré
- Les MeriToken existants continuent de décroître, tendant finalement vers minMerit

## 10.6 Héritage de coFay

En tant qu'actif, coFay suit la logique d'héritage des actifs :
- La propriété est transférée à l'héritier
- Le MeriToken n'est pas atténué (car les contributions ont été générées par le coFay lui-même)
- La distinction fondamentale : ce qui est hérité est la « propriété d'un actif », pas la « réputation personnelle »

## 10.7 Notes de discussion

> Philosophie de conception du mécanisme d'héritage :
> - Tension centrale : respecter les contributions du défunt vs. empêcher la réputation par droit de naissance
> - Solution : permettre l'héritage mais imposer l'atténuation, avec le ratio d'atténuation déterminé par l'intimité objective
> - La non-transférabilité d'iFay garantit le principe selon lequel « la personne ne peut être héritée »
> - Le cimetière numérique fournit un cadre pour gérer l'« héritage numérique » à l'ère de l'IA
> - L'héritage de coFay n'a pas d'atténuation car coFay est un actif, pas une personne
