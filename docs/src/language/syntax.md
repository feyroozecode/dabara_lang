# Syntaxe de base# Syntaxe de base

La syntaxe de Dabara est conçue pour être intuitive et naturelle pour les locuteurs haoussa. Ce chapitre couvre les éléments fondamentaux de la syntaxe.

## Structure d'un programme

Tout programme Dabara commence par `fara` (commencer) et se termine par `ƙare` ou `kare` (finir) :

```hausa
fara
  // Votre code ici
ƙare
```

### Exemple minimal
```hausa
fara
  rubuta "Sannu duniya!"
ƙare
```

## Règles de syntaxe

### 1. Indentation
Dabara utilise l'indentation pour structurer le code :
- Utilisez 2 espaces par niveau d'indentation
- Soyez cohérent dans tout votre programme

```hausa
fara
  naɗa lambar = 5
  rubuta lambar
ƙare
```

### 2. Mots-clés (Kalmomin mahimmanci)

Les mots-clés sont réservés et ne peuvent pas être utilisés comme noms de variables :

| Hausa (Unicode) | Hausa (Latin) | Français | Usage |
|-----------------|---------------|----------|---------|
| `fara` | `fara` | commencer | Début de programme |
| `ƙare` | `kare` | finir | Fin de programme |
| `rubuta` | `rubuta` | écrire | Affichage |
| `naɗa` | `nada` | créer | Déclaration variable |
| `gaskiya` | `gaskiya` | vrai | Booléen vrai |
| `karya` | `karya` | faux | Booléen faux |
| `ƙara` | `kara` | ajouter | Addition |
| `rage` | `rage` | retirer | Soustraction |

### 3. Identificateurs (Sunaye)

Les noms de variables doivent :
- Commencer par une lettre (a-z, A-Z) ou caractère haoussa
- Contenir uniquement des lettres, chiffres et underscores
- Être significatifs et descriptifs

**Valides :**
```hausa
sunan
lambari
shekarun_makaranta
mayaƙi
ƙasa_name
```

**Invalides :**
```hausa
2sunan     // commence par un chiffre
fara       // mot-clé réservé
su-nan     // contient un tiret
```

### 4. Commentaires

Dabara supporte les commentaires sur une ligne avec `//` :

```hausa
fara
  // Wannan sharhi ne (Ceci est un commentaire)
  naɗa sunan = "Ahmad"  // Suna a nan (nom ici)
  rubuta sunan
ƙare
```

### 5. Chaînes de caractères (Jimla)

Les chaînes sont délimitées par des guillemets doubles :

```hausa
fara
  naɗa gaisuwa = "Sannu!"
  naɗa jimla_dogowa = "Wannan jimla ce mai tsawo sosai"
  rubuta gaisuwa
ƙare
```

#### Caractères d'échappement
```hausa
naɗa jimla = "Yana cewa: \"Sannu!\""
naɗa layi_sabo = "Layi farko\nLayi na biyu"
```

### 6. Nombres (Lambobi)

Dabara supporte les nombres entiers :

```hausa
fara
  naɗa lambar_ƙarami = 5
  naɗa lambar_babba = 1000
  naɗa lambar_sifiri = 0
ƙare
```

### 7. Opérateurs

#### Arithmétiques
- `ƙara` ou `kara` : addition (+)
- `rage` : soustraction (-)
- `+` : concaténation de chaînes

```hausa
fara
  naɗa a = 10
  naɗa b = 5
  naɗa jimla = a ƙara b      // 15
  naɗa bambanci = a rage b   // 5
  
  naɗa maganar = "Sannu " + "duniya"  // "Sannu duniya"
ƙare
```

### 8. Assignation

Utilisez `=` pour assigner des valeurs :

```hausa
fara
  naɗa lambar = 42
  naɗa sunan = "Khadija"
  naɗa sahihi = gaskiya
ƙare
```

## Règles de style

### Conventions de nommage
- **Variables** : `snake_case` ou `camelCase`
- **Constantes** : `UPPERCASE_WITH_UNDERSCORES`
- **Noms significatifs** : préférez `shekarun_makaranta` à `sm`

### Formatage
- Une instruction par ligne
- Espaces autour des opérateurs
- Ligne vide pour séparer les sections logiques

**Bon style :**
```hausa
fara
  naɗa sunan_ɗalibi = "Ahmad"
  naɗa shekarun = 20
  
  naɗa jimla_shekaru = shekarun ƙara 5
  
  rubuta "Sunan: " + sunan_ɗalibi
  rubuta "Shekaru: "
  rubuta jimla_shekaru
ƙare
```

## Cas particuliers

### Support Unicode
Dabara supporte pleinement les caractères haoussa :
- `ƙ` (k avec crochet)
- `ɗ` (d avec crochet) 
- `ɓ` (b avec crochet)
- `ƴ` (y avec crochet)
- Et tous les autres caractères spéciaux haoussa

### Flexibilité de saisie
Pour l'accessibilité, vous pouvez utiliser les variantes latines :
- `ƙare` = `kare`
- `naɗa` = `nada`
- `ƙara` = `kara`

Les deux formes sont équivalentes et peuvent être mélangées dans le même programme.

## Erreurs courantes

### 1. Oubli de `fara`/`ƙare`
```hausa
// ❌ Incorrect
rubuta "Sannu"

// ✅ Correct
fara
  rubuta "Sannu"
ƙare
```

### 2. Indentation incorrecte
```hausa
// ❌ Incorrect
fara
rubuta "Sannu"
ƙare

// ✅ Correct
fara
  rubuta "Sannu"
ƙare
```

### 3. Guillemets manquants
```hausa
// ❌ Incorrect
naɗa sunan = Ahmad

// ✅ Correct
naɗa sunan = "Ahmad"
```

## Prochaines étapes

- [Types de données](./types.md)
- [Variables](./variables.md)
- [Opérateurs](./operators.md)
- [Expressions](./expressions.md)
