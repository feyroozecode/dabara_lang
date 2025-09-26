# Nouvelles Fonctionnalités de Dabara v0.2

## Aperçu des améliorations

Cette version étend significativement les capacités du langage Dabara avec de nouveaux opérateurs mathématiques, le support des fonctions, l'entrée utilisateur, et des exemples concrets en Hausa.

## Nouveaux Opérateurs Mathématiques

### 1. Multiplication (`ninka`)
- **Mot-clé Hausa** : `ninka`
- **Utilisation** : `lambar1 ninka lambar2`
- **Exemple** :
```hausa
naɗa sakamako = 5 ninka 3  // Résultat: 15
```

### 2. Division (`raba`)  
- **Mot-clé Hausa** : `raba`
- **Utilisation** : `lambar1 raba lambar2`
- **Gestion de la division par zéro** : Message d'erreur en Hausa
- **Exemple** :
```hausa
naɗa sakamako = 20 raba 4  // Résultat: 5
```

### 3. Priorité des opérateurs
- Les opérateurs suivent la priorité mathématique standard
- Multiplication et division avant addition et soustraction
- **Exemple** :
```hausa
naɗa sakamako = 2 ƙara 3 ninka 4  // Résultat: 14 (2 + (3 * 4))
```

## Support des Fonctions (en préparation)

### 1. Définition de fonction
- **Mot-clé Hausa** : `aiki`
- **Syntaxe** : `aiki nom(paramètres) { corps }`
- **Exemple** :
```hausa
aiki gaishe(suna) {
    rubuta "Sannu " ƙara suna
}
```

### 2. Appel de fonction  
- **Syntaxe** : `nom(arguments)`
- **Exemple** :
```hausa
gaishe("Ahmad")
```

## Entrée Utilisateur

### 1. Récupération d'entrée
- **Mot-clé Hausa** : `karɓa`
- **Fonctionnalité** : Lit l'entrée utilisateur depuis la console
- **Détection automatique** : Nombre ou chaîne de caractères
- **Exemple** :
```hausa
rubuta "Rubuta sunanka: "
naɗa suna = karɓa
rubuta "Sannu " ƙara suna
```

## Support Unicode Hausa Renforcé

### 1. Identifiants avec chiffres
- Support des identifiants contenant des chiffres après les lettres
- Compatible avec les caractères Unicode haoussa
- **Exemples valides** :
  - `lambar1`, `ɗan_makaranta2`, `ƙarfin3`

### 2. Caractères spéciaux supportés
- `ɓ`, `ɗ`, `ƙ`, `ƴ`, `ʔ`
- Intégration complète dans les identifiants

## Support des Commentaires

### 1. Commentaires de ligne
- **Syntaxe** : `# commentaire`
- Les commentaires sont ignorés lors de l'analyse
- **Exemple** :
```hausa
# Wannan shi ne misali
naɗa lambar = 42  # Lambar ta musamman
```

## Exemples Pratiques Fournis

### 1. Calculatrice simple (`hasaban_lissafi.ha`)
- Démontre tous les opérateurs mathématiques
- Calculs combinés avec priorité des opérateurs
- Interface entièrement en Hausa

### 2. Interaction utilisateur (`haduwa_da_mutum.ha`)
- Collecte du nom et de l'âge
- Calcul de l'année de naissance
- Messages personnalisés en Hausa

### 3. Jeux de mots (`wasanni_kalmomi.ha`)
- Table de multiplication
- Calculs de temps (années→mois→jours→heures)
- Inventaire d'objets de maison
- Exemples éducatifs amusants

### 4. Jeu éducatif (`wasan_lissafi.ha`)
- Quiz de calcul mental interactif
- Questions en Hausa avec réponses
- Encouragements et messages éducatifs

### 5. Test Unicode (`unicode_test.ha`)
- Validation du support Unicode complet
- Identifiants avec caractères haoussa et chiffres
- Opérations complexes avec noms Unicode

## Tests Unitaires Complets

### 1. Nouveaux tests ajoutés
- Tests des opérateurs de multiplication et division
- Tests de parsing des fonctions
- Tests de l'entrée utilisateur
- Tests des expressions arithmétiques complexes
- Tests du support Unicode étendu

### 2. Validation
- 29 tests passent avec succès
- Couverture complète des nouvelles fonctionnalités
- Tests d'erreurs (division par zéro, tokens invalides)

## Utilisation

### Exécuter les exemples
```bash
cargo run examples/hasaban_lissafi.ha
cargo run examples/wasanni_kalmomi.ha
cargo run examples/wasan_lissafi.ha
```

### Lancer les tests
```bash
cargo test
```

## Philosophie de Développement

Cette implémentation suit la philosophie de "coder en parlant Hausa" :

1. **Simplicité** : Syntaxe intuitive proche du langage naturel haoussa
2. **Clarté** : Mots-clés explicites et compréhensibles
3. **Accessibilité** : Messages d'erreur en haoussa
4. **Éducation** : Exemples pratiques et jeux éducatifs
5. **Culture** : Respect des spécificités linguistiques haoussa

## Évolutions Futures

- Implémentation complète des fonctions avec environnement local
- Structures de contrôle (conditions, boucles)
- Support des tableaux et structures de données
- Bibliothèque standard de fonctions utilitaires en Hausa
- Interface graphique éventuelle pour l'apprentissage

---

**Dabara** continue d'évoluer comme un langage de programmation accessible, éducatif et culturellement adapté à la communauté haoussa.