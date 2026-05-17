# Chapitre 11 : Sécurité et Anti-fraude

## 11.1 Modèle de menaces

| Menace | Description | Impact |
|--------|-------------|--------|
| Accumulation frauduleuse de Merit | Obtenir des MeriToken par de fausses contributions | Pouvoir de vote gonflé |
| Vote collusoire | Plusieurs parties conspirant pour manipuler les votes de reconnaissance | Acquisition illégitime de Merit |
| Fabrication d'intimité | Fabriquer des interactions pour augmenter l'intimité | Contournement des exclusions, réduction de l'atténuation d'héritage |
| Falsification d'identité | Créer de faux HumanID | Identités multiples acquérant plusieurs parts de Merit |
| Attaque Sybil | Une personne contrôlant plusieurs identités | Manipulation des votes |

## 11.2 Prévention de l'accumulation frauduleuse de Merit

### Protections pour la mesure objective

- Le système enregistre automatiquement, laissant peu de place à la manipulation humaine
- La vérification croisée est possible (par exemple, comparer les heures de travail vs. la production)
- Détection statistique des anomalies

### Protections pour l'évaluation subjective

> Principe central : rendre le coût de la fraude bien supérieur au bénéfice.

1. **Exclusion par intimité** : exclure les votants ayant des relations proches
2. **Pondération par MeriToken** : les votants à haute réputation ont plus de poids ; les fraudeurs doivent d'abord accumuler une réputation authentique substantielle
3. **Audit comportemental** : voter fréquemment en faveur d'un individu spécifique → signalé comme anomalie
4. **Échantillonnage aléatoire** : sélectionner aléatoirement les votants pour réduire la possibilité de collusion
5. **Responsabilité rétroactive** : une fois la fraude découverte, tous les participants sont sanctionnés

## 11.3 Prévention de la fabrication d'intimité

- Évaluation de la qualité des interactions (pas seulement la fréquence)
- Les interactions unidirectionnelles sont invalides (doivent être bidirectionnelles)
- De grands volumes d'interactions dans un court laps de temps sont traités comme anomalies
- Des interactions isolées à haute fréquence entre deux individus (sans cercle social partagé) sont traitées comme suspectes

## 11.4 Sécurité des clés

- Schémas multi-signatures : les opérations critiques nécessitent la confirmation de plusieurs clés
- Rotation des clés : remplacement périodique
- Récupération sociale : les contacts de confiance assistent à la récupération

## 11.5 Protection de la vie privée

- Le contenu des votes n'est pas public (ZKP) ; seuls les résultats sont divulgués
- Les valeurs d'intimité peuvent être divulguées de manière sélective
- Le contenu des interactions n'est pas stocké sur la chaîne
- La participation anonyme est supportée (ZKP prouve l'éligibilité sans révéler l'identité)

## 11.6 Notes de discussion

> Philosophie de conception du mécanisme de sécurité :
> - Il n'existe pas de solution anti-fraude parfaite ; l'objectif est de rendre le coût de la fraude bien supérieur au bénéfice
> - Les défenses multicouches sont plus efficaces que tout mécanisme unique
> - Mesures préventives + responsabilité rétroactive forment une boucle fermée
> - L'anti-fraude est un processus adversarial continu ; le système doit pouvoir évoluer
