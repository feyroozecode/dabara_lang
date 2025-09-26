# Démarrage rapide avec Dabara

Ce guide vous permettra d'écrire votre premier programme Dabara en quelques minutes !

## Votre premier programme

### Créer un fichier

Créez un nouveau fichier avec l'extension `.ha` :

```bash
touch sannu.ha
```

### Écrire le code

Ouvrez le fichier et ajoutez :

```hausa
fara
  rubuta "Sannu duniya!"
ƙare
```

### Exécuter

```bash
dabara sannu.ha
```

**Résultat :**
```
Sannu duniya!
```

🎉 **Félicitations !** Vous venez d'exécuter votre premier programme Dabara !

## Structure d'un programme

Tout programme Dabara suit cette structure :

```hausa
fara
  // Votre code ici
ƙare
```

- `fara` : Marque le début du programme
- `ƙare` : Marque la fin du programme
- Tout le code doit être entre ces deux mots-clés

## Exemples de base

### 1. Affichage de texte

```hausa
fara
  rubuta "Barka da zuwa!"
  rubuta "Ina kwana?"
ƙare
```

### 2. Variables

```hausa
fara
  naɗa sunan = "Khadija"
  naɗa shekarun = 25
  rubuta "Sunanta: " + sunan
  rubuta shekarun
ƙare
```

### 3. Calculs simples

```hausa
fara
  naɗa a = 10
  naɗa b = 5
  naɗa jimla = a ƙara b
  rubuta "Jimla: "
  rubuta jimla
ƙare
```

## Concepts de base

### Types de données
- **Nombres** : `42`, `0`, `999`
- **Texte** : `"Sannu"`, `"Barka da zuwa"`
- **Booléens** : `gaskiya` (vrai), `karya` (faux)

### Mots-clés essentiels
- `fara` : début de programme
- `ƙare` : fin de programme  
- `rubuta` : afficher/imprimer
- `naɗa` : créer une variable
- `ƙara` : addition
- `rage` : soustraction

### Support clavier
Si vous ne pouvez pas taper les caractères spéciaux, utilisez les variants latins :
- `ƙare` → `kare`
- `naɗa` → `nada`
- `ƙara` → `kara`

## Exemples pratiques

### Calculatrice simple

```hausa
fara
  naɗa prix1 = 1500
  naɗa prix2 = 2000
  naɗa total = prix1 ƙara prix2
  
  rubuta "Prix 1: "
  rubuta prix1
  rubuta "Prix 2: "
  rubuta prix2
  rubuta "Total: "
  rubuta total
ƙare
```

### Présentation personnelle

```hausa
fara
  naɗa sunan = "Ahmad"
  naɗa shekarun = 30
  naɗa garin = "Kano"
  
  rubuta "Sannu! Ni " + sunan
  rubuta "Ina da shekaru " 
  rubuta shekarun
  rubuta "Daga " + garin + " nake"
ƙare
```

### Programme avec booléens

```hausa
fara
  naɗa yana_da_mota = gaskiya
  naɗa yana_da_gida = karya
  
  rubuta "Yana da mota: "
  rubuta yana_da_mota
  rubuta "Yana da gida: "
  rubuta yana_da_gida
ƙare
```

## Mode debug

Pour voir ce qui se passe dans votre programme :

```bash
export DABARA_DEBUG=1
dabara votre_programme.ha
```

Cela affichera :
- Le code source
- Les tokens générés
- L'arbre syntaxique (AST)

## Gestion des erreurs

Dabara fournit des messages d'erreur clairs en haoussa :

### Erreur de syntaxe
```hausa
fara
  rubuta "Oubli des guillemets
ƙare
```

**Erreur :**
```
Kuskure: Ana tsammanin alamar '"' a layi 2
```

### Variable non définie
```hausa
fara
  rubuta sunan_bai_wanzu
ƙare
```

**Erreur :**
```
Kuskure: Babu irin wannan mai canjin 'sunan_bai_wanzu'
```

## Bonnes pratiques

### 1. Nommage des variables
```hausa
// ✅ Bon
naɗa sunan_makaranta = "Université de Kano"
naɗa yawan_dalibai = 5000

// ❌ Éviter
naɗa x = "Université de Kano"
naɗa n = 5000
```

### 2. Organisation du code
```hausa
fara
  // Déclaration des variables
  naɗa sunan = "Ahmad"
  naɗa shekarun = 25
  
  // Calculs
  naɗa shekarun_bana = shekarun ƙara 1
  
  // Affichage des résultats
  rubuta "Sannu " + sunan
  rubuta "Shekarunka bana: "
  rubuta shekarun_bana
ƙare
```

### 3. Commentaires (bientôt disponible)
```hausa
fara
  // Wannan shi ne sunan mutum
  naɗa sunan = "Fatima"
  rubuta sunan
ƙare
```

## Prochaines étapes

Maintenant que vous maîtrisez les bases :

1. **Explorez la [référence du langage](./language/syntax.md)** pour découvrir toutes les fonctionnalités
2. **Consultez plus d'[exemples](./examples/hello-world.md)** pour des cas d'usage avancés  
3. **Rejoignez la [communauté](./development/contributing.md)** pour partager vos créations

## Aide et support

- **Documentation complète** : Parcourez toute la documentation
- **Exemples** : Section dédiée aux cas pratiques
- **FAQ** : Questions fréquemment posées
- **GitHub** : Signaler des bugs ou demander de l'aide

---

**Bonne programmation avec Dabara !** 🚀

> *Premier programme écrit ? Félicitations ! Vous faites maintenant partie de la communauté Dabara. Continuez à explorer et n'hésitez pas à partager vos créations !*