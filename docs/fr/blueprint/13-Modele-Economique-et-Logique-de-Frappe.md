# Chapitre 13 : Modèle Économique et Logique de Frappe

## 13.1 MeriToken n'est pas une monnaie

Pour réitérer le positionnement économique de MeriToken :

- Non-échangeable, non-convertible
- Aucune valeur spéculative
- N'est pas un moyen d'échange
- Purement une mesure de contribution et un porteur de pouvoir de vote

Par conséquent, les contraintes de l'économie monétaire traditionnelle (contrôle de l'inflation, politique monétaire) ne s'appliquent pas à MeriToken.

## 13.2 Sélection de l'approche de frappe

> Trois approches ont été évaluées lors des discussions :

| Approche | Description | Avantages | Inconvénients |
|----------|-------------|-----------|---------------|
| Offre fixe | Plafond prédéfini | Simple | Difficulté croissante pour les arrivants tardifs, inéquitable |
| Quota périodique | Montant de frappe fixe par période | Contrôle l'offre totale | Les contributions deviennent un jeu à somme nulle |
| **Sans plafond + auto-équilibrage par décroissance** | Frappe à la demande, la décroissance brûle automatiquement | Équitable, pas de désavantage pour les arrivants tardifs | Nécessite un modèle de décroissance précis |

### Choix : Frappe sans plafond + Auto-équilibrage par décroissance

Justification :
- Le Merit n'est pas une monnaie ; il n'a pas besoin de rareté pour maintenir sa valeur
- Il représente le « niveau de contribution active actuel » ; la décroissance le garantit
- Évite les désavantages inéquitables pour les arrivants tardifs
- Le pouvoir de vote est basé sur la part ; les changements de l'offre totale n'affectent pas l'équité de la gouvernance

## 13.3 Pourquoi la sur-émission ne se produira pas

> Question clé soulevée lors des discussions : Le Merit est créé à partir de rien — ne sera-t-il pas sur-émis ?

Réponse :
1. **La décroissance est un mécanisme de brûlage naturel** : les anciens MeriToken décroissent continuellement
2. **Équilibre dynamique** : lorsque le taux de frappe = le taux de décroissance, l'offre totale tend vers la stabilité
3. **La part détermine le pouvoir de vote** : même si l'offre totale augmente, le pouvoir de vote individuel dépend de la part plutôt que de la valeur absolue
4. **Analogie** : les comptages de citations académiques n'ont pas de plafond, mais l'influence des articles plus anciens décroît naturellement — le système s'auto-équilibre

## 13.4 Équilibre dynamique

### État stationnaire

Lorsque le nombre d'utilisateurs est stable : MeriToken total du réseau ≈ constant

### Phase de croissance

Nouveaux utilisateurs en augmentation → l'offre totale croît → mais la moyenne par habitant tend vers la stabilité → les parts de pouvoir de vote sont naturellement diluées

### Phase de déclin

Utilisateurs actifs en diminution → la frappe diminue tandis que la décroissance continue → l'offre totale baisse → les parts des utilisateurs actifs restants augmentent

## 13.5 Allocation initiale

- L'inscription accorde MeriToken = e ≈ 2,718
- minMerit initial = e
- Garantit que chaque nouvel utilisateur a une capacité de participation de base
- e est suffisamment petit pour ne pas diluer significativement les utilisateurs existants, mais suffisamment grand pour garantir les droits de participation de base

## 13.6 Analyse des incitations

MeriToken est non-échangeable, mais les incitations qu'il fournit sont :

| Incitation | Description |
|------------|-------------|
| Pouvoir de vote | Influence dans la prise de décision communautaire |
| Reconnaissance sociale | MeriToken élevé = haute crédibilité |
| Accès prioritaire | Allocation préférentielle de certaines ressources ou opportunités |
| Valeur patrimoniale | Peut être partiellement transmis aux descendants |

Dans l'ère post-monétaire, la reconnaissance sociale et le pouvoir de vote sont eux-mêmes les incitations les plus fortes.

## 13.7 Notes de discussion

> Perspectives fondamentales du modèle économique :
> - MeriToken n'est pas une monnaie et n'a pas besoin des contraintes de l'économie monétaire
> - La décroissance est le mécanisme de « brûlage » le plus élégant — aucune intervention humaine nécessaire, auto-équilibrage naturel
> - Le pouvoir de vote basé sur la part signifie que les changements de l'offre totale n'affectent pas l'équité de la gouvernance
> - L'avantage central de ce modèle : simplicité, auto-équilibrage, équité
> - Aucune « politique monétaire » complexe n'est nécessaire pour maintenir la stabilité
