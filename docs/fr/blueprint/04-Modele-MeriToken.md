# Chapitre 4 : Modèle MeriToken

## 4.1 Vue d'ensemble du modèle

MeriToken est l'unité de mesure centrale de GMC. Sa conception doit répondre à une question clé : **Comment la mesure de contribution peut-elle refléter l'activité actuelle tout en respectant les contributions historiques ?**

La réponse est : décroissance exponentielle + valeur plancher non nulle.

## 4.2 Deux valeurs clés

Chaque MeritPocket maintient deux valeurs fondamentales :

- **curMerit** (MeriToken actuel) : La valeur de mesure de contribution en temps réel ; décroît au fil du temps et croît avec les nouvelles contributions
- **minMerit** (valeur plancher) : La borne inférieure de décroissance, représentant le sédiment à long terme des contributions historiques ; ne fait qu'augmenter (sauf en cas de sanctions)

```
curMerit ≥ minMerit ≥ e (valeur initiale)
```

## 4.3 Acquisition

MeriToken est acquis par les contributions ; le système frappe de nouveaux Tokens :

| Méthode d'acquisition | Description | Condition de déclenchement |
|-----------------------|-------------|---------------------------|
| Mesure objective | Calculé automatiquement sur la base de métriques vérifiables | Le système enregistre automatiquement le seuil atteint |
| Prime de tâche | Merit prédéfini pour une tâche spécifique | Les parties prenantes votent pour approuver à l'achèvement |
| Allocation initiale | Accordé lors de l'inscription au réseau | Inscription d'identité complétée |

Valeur initiale = e ≈ 2,718 (la constante naturelle, naturellement alignée avec le modèle de décroissance exponentielle).

## 4.4 Modèle de décroissance

### Idée centrale

Chaque lot d'acquisition de Merit possède une **durée d'influence** indépendante. La durée d'influence reflète la temporalité de cette contribution — une contribution avec 100 jours d'influence voit son Merit entièrement décroître en 100 jours.

### Formule de décroissance par lot

```
MeriToken_i(t) = (V_i - B_i) × e^(-λ_i × t) + B_i
```

- `V_i` : Valeur initiale de Merit du lot
- `B_i` : Contribution du lot à la valeur plancher
- `λ_i` : Coefficient de décroissance, déterminé par la durée d'influence T_i (λ_i = k / T_i, où k est une constante)
- `t` : Temps écoulé depuis l'acquisition

### MeriToken actuel total

```
curMerit = Σ MeriToken_i(t)  (somme de tous les lots actifs)
```

Lorsque tous les lots ont entièrement décru, curMerit tend vers minMerit.

## 4.5 Valeur plancher (minMerit)

### Règle de mise à jour

Chaque fois qu'un nouveau Merit est acquis, la valeur plancher est mise à jour :

Soit curMerit actuel = M, Merit nouvellement acquis = x, valeur plancher actuelle = B, alors :

```
Nouvelle valeur plancher B' = (x + M) × B / M
```

Signification : La valeur plancher croît proportionnellement à la part du nouveau Merit dans le total.

### Propriétés

- Valeur de départ = e ≈ 2,718
- Ne fait qu'augmenter (sauf en cas de sanctions)
- Représente le sédiment indélébile des contributions historiques
- Même si les contributions cessent entièrement, curMerit ne tombera jamais en dessous de minMerit

### Cas limite

Lorsque curMerit = minMerit (c'est-à-dire à l'état plancher) et qu'un nouveau Merit x est acquis :
```
B' = (x + B) × B / B = x + B
```
La valeur plancher augmente directement de x — ce qui signifie que le Merit acquis à l'état plancher est entièrement déposé en tant que valeur plancher.

## 4.6 Implémentation de la décroissance indépendante par lot

### Défis

- Chaque MeritPocket doit maintenir une liste de lots de Merit
- Interroger la valeur actuelle nécessite d'itérer sur tous les lots qui n'ont pas entièrement décru
- Les coûts de stockage et de calcul sur la chaîne croissent linéairement avec le nombre de lots

### Stratégies d'optimisation

1. **Fusion de lots** : Les lots avec des durées d'influence similaires sont périodiquement fusionnés pour réduire le nombre de lots actifs
2. **Calcul hors chaîne** : Utiliser Rollup pour calculer les valeurs en temps réel hors chaîne ; ne stocker que les instantanés et les preuves sur la chaîne
3. **Sédimentation de lots** : Lorsque le nombre maximum de lots actifs est dépassé, les lots les plus anciens sont automatiquement sédimentés dans la valeur plancher
4. **Calcul paresseux** : Les valeurs précises ne sont calculées que lorsque nécessaire (par exemple, lors de votes ou de requêtes)

## 4.7 Philosophie de conception

### Pourquoi la décroissance exponentielle ?

- Incite à la contribution continue plutôt qu'à une seule grande contribution suivie d'inactivité
- Reflète la temporalité des contributions — les contributions plus récentes ont un impact plus important sur la réputation actuelle
- Simule naturellement la décroissance de la mémoire sociale
- Décroît rapidement au début puis ralentit, en accord avec l'intuition

### Pourquoi un plancher non nul ?

- Reconnaît la valeur à long terme des contributions historiques — les efforts passés ne s'annulent pas complètement
- Empêche les contributeurs de longue date de perdre tout pouvoir de vote en raison d'une brève pause
- La valeur plancher croît avec les contributions cumulées, récompensant la participation soutenue

### Pourquoi une durée d'influence indépendante par lot ?

- Différentes contributions ont naturellement des temporalités différentes
- Une seule interaction de service client peut avoir une influence de seulement 30 jours
- Maintenir un projet open-source peut avoir une influence durant des années
- Un taux de décroissance uniforme déformerait la valeur des différents types de contributions

## 4.8 Notes de discussion

> Décisions clés dans le modèle MeriToken :
> - Décroissance exponentielle + plancher non nul : Trouve un équilibre entre « inciter à la participation continue » et « respecter les contributions historiques »
> - Durée d'influence indépendante par lot : Augmente la complexité d'implémentation mais reflète plus précisément les différences de temporalité des contributions
> - La valeur plancher ne fait qu'augmenter (sauf en cas de sanctions) : Protège les droits fondamentaux des contributeurs de longue date
> - Valeur initiale de e : Combine élégance mathématique et signification pratique
>
> À examiner plus en détail : Si la formule de mise à jour de la valeur plancher se comporte raisonnablement dans des conditions extrêmes
