# Chapitre 5 : Système d'identité

## 5.1 Pourquoi un système d'identité dédié est nécessaire

L'identité dans GMC diffère des comptes internet traditionnels :

- Elle est liée à la réputation à vie d'une personne physique et ne peut être créée ou abandonnée arbitrairement
- Elle doit supporter la liaison permanente d'iFay et le transfert de propriété de coFay
- Elle doit être vérifiable dans un environnement décentralisé tout en protégeant la vie privée

## 5.2 Couches d'identité

```
┌─────────────────────────────────────┐
│  Couche 1 : Identité de la personne  │  ← Unique, à vie
│             physique (HumanID)        │
├─────────────────────────────────────┤
│  Couche 2 : Identité Fay (FayID)     │  ← Associé au HumanID
├─────────────────────────────────────┤
│  Couche 3 : Couche d'actifs          │  ← Lié au FayID
│             (MeritPocket)            │
└─────────────────────────────────────┘
```

### HumanID

- Unique au monde, identifie une personne physique
- Un HumanID peut correspondre à plusieurs FayID
- Valide à vie, ne peut être désinscrit (mais peut entrer en état de cimetière)

### FayID

- Unique au monde, identifie un Fay
- Chaque FayID est associé à un MeritPocket
- Le FayID d'un iFay est lié de manière permanente à un HumanID
- La propriété du FayID d'un coFay peut être transférée

## 5.3 Schéma de vérification sur la chaîne

### Comparaison des schémas

| Schéma | Principe | Avantages | Inconvénients | Scénarios applicables |
|--------|----------|-----------|---------------|----------------------|
| PKI (Paire de clés publique-privée) | Vérification par signature de paire de clés | Mature, efficace, décentralisé | Perte de clé privée = perte d'identité | Signatures de base |
| DID (Identité décentralisée) | Standard W3C, documents d'identité sur la chaîne | Standardisé, supporte la récupération de clés | Relativement complexe | Cartographie des relations |
| ZKP (Preuve à divulgation nulle) | Prouve l'identité sans révéler d'informations | Protection de la vie privée extrêmement forte | Coût de calcul élevé | Scénarios de confidentialité |

### Recommandation : Combinaison en couches

1. **Couche de base (vérification basique)** : PKI
   - Mécanisme de signature pour toutes les opérations sur la chaîne
   - Chaque HumanID et FayID possède une paire de clés

2. **Couche intermédiaire (gestion des relations)** : DID
   - Gère les relations de liaison HumanID ↔ FayID
   - Supporte la rotation des clés et la récupération sociale
   - Stocke les métadonnées d'identité

3. **Couche supérieure (scénarios de confidentialité)** : ZKP
   - Prouve l'identité lors du vote sans révéler qui vous êtes
   - Vérifie les relations lors de l'authentification d'héritage sans exposer les détails
   - Protège les lanceurs d'alerte lors des plaintes de sanction

### Justification

> Chaque schéma pris individuellement a ses limites :
> - Le PKI pur ne peut résoudre la perte de clés et manque de protection de la vie privée
> - Le DID pur a des performances insuffisantes pour la vérification à haute fréquence
> - Le ZKP pur a des coûts de calcul excessifs
>
> Une combinaison en couches permet à chaque couche de se concentrer sur les scénarios qu'elle gère le mieux.

## 5.4 Cycle de vie d'iFay

```
Création → Liaison à l'archétype humain → Fonctionnement normal → [Décès de l'archétype humain] → Tutelle / Cimetière numérique
```

### Fonctionnement normal

- iFay agit au nom de l'archétype humain
- Tous les MeriToken générés appartiennent à l'archétype humain
- L'archétype humain participe aux votes, à la reconnaissance des contributions, etc. via iFay

### Tutelle

Lorsque l'archétype humain décède :
- Un héritier peut demander à devenir le tuteur
- Le tuteur peut gérer au nom du défunt, mais **ne peut pas agir sous l'identité de l'archétype humain**
- Toutes les actions de tutelle doivent afficher les informations du tuteur
- Il existe un marqueur de tutelle explicite sur la chaîne

### Cimetière numérique

- Un iFay peut encore avoir des interactions passives après avoir été placé au cimetière
- Toutes les interactions sont étiquetées « depuis le cimetière numérique »
- Aucun nouveau MeriToken n'est activement généré
- Les MeriToken existants continuent de décroître normalement

## 5.5 Transfert de propriété de coFay

En tant qu'actif, coFay suit ces règles de transfert :

1. Le MeritPocket est transféré avec le coFay ; le MeriToken n'est pas atténué
2. Les enregistrements de transfert sont stockés sur la chaîne ; l'historique des changements de propriété est inviolable
3. Le transfert nécessite la confirmation par signature des deux parties
4. La continuité du pouvoir de vote du coFay n'est pas affectée par le transfert

## 5.6 Prévention des attaques Sybil

Le multi-comptes par une seule personne est une menace classique pour les systèmes d'identité décentralisés :

- L'inscription HumanID nécessite une preuve d'unicité (méthode spécifique à déterminer)
- Analyse du graphe social : Les vrais utilisateurs ont des réseaux sociaux naturels ; les faux comptes présentent des schémas anormaux
- Analyse des schémas comportementaux : Plusieurs comptes contrôlés par la même personne partagent des caractéristiques comportementales similaires
- Confiance progressive : Les permissions et l'influence des nouveaux utilisateurs sont libérées progressivement

## 5.7 Notes de discussion

> Compromis fondamentaux dans le système d'identité :
> - Sécurité vs. utilisabilité : La vérification à trois couches augmente la sécurité mais aussi la complexité
> - Vie privée vs. transparence : ZKP protège la vie privée ; les enregistrements sur la chaîne assurent la transparence
> - Permanence vs. flexibilité : La liaison permanente d'iFay garantit que la réputation est inséparable de la personne ; la transférabilité de coFay assure la flexibilité commerciale
> - La prévention des attaques Sybil est un défi éternel pour l'identité décentralisée et nécessite une combinaison de multiples approches
