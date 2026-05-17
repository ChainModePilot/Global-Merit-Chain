# Chapitre 12 : Architecture Technique

## 12.1 Vue d'ensemble de l'architecture

```
┌─────────────────────────────────────────────┐
│  Couche applicative (DApp, Interface Fay,    │
│  UI de gouvernance)                          │
├─────────────────────────────────────────────┤
│  Couche 2 : ZK Rollup (Traitement des       │
│  transactions à haute fréquence)             │
│  - Enregistrements de contributions, mises   │
│    à jour d'intimité, interactions           │
│    quotidiennes                              │
├─────────────────────────────────────────────┤
│  Couche 1 : Chaîne dédiée Substrate         │
│  (Règlement & Consensus)                     │
│  - Ancrage de la racine d'état, gestion      │
│    d'identité, vote de gouvernance           │
├─────────────────────────────────────────────┤
│  Couche d'identité : DID + PKI + ZKP        │
└─────────────────────────────────────────────┘
```

## 12.2 Comparaison des choix technologiques

> Plusieurs approches ont été évaluées lors des discussions :

| Approche | Avantages | Inconvénients | Conclusion |
|----------|-----------|---------------|------------|
| Réseau principal Ethereum | Écosystème mature, haute sécurité | Frais de gas élevés, faible TPS (15–30) | Non adapté à l'enregistrement à haute fréquence à l'échelle de la population |
| Ethereum L2 | Frais réduits | Toujours contraint par l'écosystème Ethereum | Alternative |
| DAG (IOTA/Nano) | Haut débit, sans frais | Sécurité de consensus faible | Sécurité insuffisante |
| **Chaîne personnalisée Substrate** | Entièrement personnalisable, sans frais de gas | Nécessite de construire son propre écosystème | **Recommandé** |

### Le problème des frais de gas

Les frais de gas sont le coût de calcul par transaction sur les chaînes publiques comme Ethereum. Avec l'ensemble de la population générant quotidiennement de grands volumes de micro-enregistrements de contributions, enregistrer chacun sur la chaîne serait prohibitivement coûteux. GMC nécessite une méthode d'enregistrement gratuite ou à coût extrêmement faible.

### Le problème du débit

Le réseau principal Ethereum gère environ 15–30 TPS. Pour les enregistrements de contributions de milliards d'utilisateurs dans le monde, ce débit est loin d'être suffisant.

## 12.3 Chaîne dédiée Substrate

### Pourquoi Substrate

1. **Consensus entièrement personnalisable** : concevoir un algorithme de consensus spécifiquement adapté à l'enregistrement des contributions
2. **Sans frais de gas** : peut être conçu pour des transactions sans frais
3. **Modules de gouvernance personnalisables** : naturellement adapté au consensus communautaire
4. **Interopérabilité Polkadot** : peut interopérer avec d'autres chaînes via des chaînes relais
5. **Modulaire** : composer les modules Runtime selon les besoins

### Justification

> Les exigences uniques de GMC rendent les chaînes publiques généralistes inadaptées :
> - Participation de toute la population = volume de transactions extrêmement élevé
> - Micro-enregistrements de contributions = transactions à haute fréquence et faible valeur
> - Impossibilité de facturer des frais = l'enregistrement des contributions ne doit pas devenir un fardeau financier
> - Nécessite des calculs de décroissance personnalisés et des algorithmes d'intimité

## 12.4 ZK Rollup

### Concept central

Exécution hors chaîne, vérification sur la chaîne :
- Les enregistrements de contributions quotidiens sont traités à haute vitesse sur L2, sans frais et à haut débit
- Des preuves à divulgation nulle des enregistrements par lots sont périodiquement soumises à L1
- L1 ne stocke que les racines d'état compressées

### ZK Rollup vs. Optimistic Rollup

| Caractéristique | ZK Rollup | Optimistic Rollup |
|-----------------|-----------|-------------------|
| Méthode de vérification | Preuves à divulgation nulle (garantie mathématique) | Preuves de fraude (période de contestation) |
| Temps de confirmation | Rapide | Lent (typiquement 7 jours) |
| Sécurité | Garantie mathématique | Repose sur des validateurs honnêtes |
| Coût de calcul | Élevé | Faible |

**Choix : ZK Rollup** — un système de réputation nécessite une confirmation rapide et une sécurité garantie mathématiquement.

### Répartition des responsabilités

- **Traitement L2** : création d'enregistrements de contributions, calcul de MeriToken en temps réel, mises à jour d'intimité
- **Ancrage L1** : racines d'état, inscription/modifications d'identité, résultats de votes de gouvernance, enregistrements de sanctions

## 12.5 Stockage des données

```
Sur la chaîne (L1) : Registre d'identité, racines d'état, enregistrements de gouvernance, enregistrements de sanctions
Rollup (L2) : Soldes et lots de MeriToken, intimité, enregistrements de contributions
Hors chaîne (IPFS, etc.) : Détails des interactions, preuves de contributions, fichiers volumineux
```

## 12.6 Mécanisme de consensus

- **Admission des validateurs** : nécessite un certain montant de MeriToken (garantie de réputation)
- **Incitations à la validation** : le travail de validation est lui-même une contribution et peut rapporter du Merit
- **Consensus L1** : GRANDPA/BABE (valeurs par défaut de Substrate)
- **Consensus L2** : BFT léger

## 12.7 Estimations de performance

En supposant 1 milliard d'utilisateurs, chacun générant 5 enregistrements par jour :
- Volume de transactions quotidien : 5 milliards d'enregistrements
- Exigence TPS : ~58 000
- Nécessite : plusieurs instances Rollup parallèles (sharding), génération efficace de preuves, nœuds L2 distribués

## 12.8 Notes de discussion

> Décisions fondamentales de l'architecture technique :
> - Chaîne dédiée plutôt que chaîne publique généraliste : les exigences de GMC sont trop spécialisées
> - ZK Rollup plutôt qu'Optimistic : nécessite une confirmation rapide et des garanties mathématiques
> - Stockage en couches : un équilibre entre sécurité et évolutivité
> - La performance est le plus grand défi : l'échelle de la participation de toute la population est sans précédent
>
> Ceci est un concept d'architecture au stade de document de discussion ; l'implémentation réelle devra être ajustée en fonction des évolutions technologiques.
