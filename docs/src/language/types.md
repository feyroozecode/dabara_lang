# Types de données# Types de données

Dabara supporte plusieurs types de données fondamentaux avec des noms en haoussa. Ce chapitre détaille chaque type et son utilisation.

## Types primitifs

### 1. Lambar (Nombres)

Les nombres en Dabara sont des entiers signés :

```hausa
fara
  naɗa lambar_ƙarami = 5
  naɗa lambar_babba = 1000
  naɗa lambar_sifiri = 0
  naɗa lambar_ɢangare = -42
  
  rubuta lambar_ƙarami    // Affiche: 5
  rubuta lambar_babba      // Affiche: 1000
  rubuta lambar_sifiri     // Affiche: 0
  rubuta lambar_ɢangare   // Affiche: -42
ƙare
```

#### Caractéristiques
- **Plage** : -2,147,483,648 à 2,147,483,647 (32 bits signé)
- **Notation** : Décimale uniquement
- **Opérations** : Addition (`ƙara`), soustraction (`rage`)

#### Exemples d'usage
```hausa
fara
  naɗa shekarun = 25
  naɗa lambar_yara = 3
  naɗa jimla = shekarun ƙara lambar_yara  // 28
  
  rubuta "Jimla shekaru: "
  rubuta jimla
ƙare
```

### 2. Jimla (Chaînes de caractères)

Les chaînes représentent du texte en Dabara :

```hausa
fara
  naɗa sunan = "Ahmad"
  naɗa gaisuwa = "Sannu!"
  naɗa maganar_dogowa = "Wannan jimla ce mai tsawo sosai da kuma bayanai da yawa"
  
  rubuta sunan         // Affiche: Ahmad
  rubuta gaisuwa       // Affiche: Sannu!
  rubuta maganar_dogowa
ƙare
```

#### Caractéristiques
- **Délimitation** : Guillemets doubles `""`
- **Support Unicode** : Tous les caractères haoussa supportés
- **Opérations** : Concaténation avec `+`

#### Concaténation
```hausa
fara
  naɗa sunan_farko = "Ahmad"
  naɗa sunan_iyali = "Bello"
  naɗa sunan_ciki = sunan_farko + " " + sunan_iyali
  
  rubuta sunan_ciki    // Affiche: Ahmad Bello
ƙare
```

#### Caractères d'échappement
```hausa
fara
  naɗa jimla_da_quotes = "Yana cewa: \"Sannu duniya!\""
  naɗa jimla_da_newline = "Layi farko\nLayi na biyu"
  naɗa jimla_da_tab = "Kalma\tKalma ta biyu"
  
  rubuta jimla_da_quotes   // Affiche: Yana cewa: "Sannu duniya!"
  rubuta jimla_da_newline  // Affiche sur deux lignes
  rubuta jimla_da_tab      // Affiche avec tabulation
ƙare
```

#### Caractères spéciaux supportés
- `\n` : Nouvelle ligne
- `\t` : Tabulation
- `\"` : Guillemet double
- `\\` : Antislash littéral

### 3. Boolean (Gaskiya/Karya)

Les valeurs booléennes représentent vrai ou faux :

```hausa
fara
  naɗa sahihi = gaskiya      // true
  naɗa kuskure = karya       // false
  naɗa yana_kwana = gaskiya
  
  rubuta sahihi        // Affiche: gaskiya
  rubuta kuskure       // Affiche: karya
  rubuta yana_kwana    // Affiche: gaskiya
ƙare
```

#### Caractéristiques
- **Valeurs** : `gaskiya` (vrai) et `karya` (faux)
- **Usage** : Logique, conditions, états
- **Affichage** : Les valeurs sont affichées en haoussa

## Conversion de types

### Conversion implicite

Dabara effectue certaines conversions automatiques :

```hausa
fara
  naɗa lambar = 42
  naɗa jimla = "Lambar: " + lambar
  
  rubuta jimla    // Affiche: Lambar: 42
ƙare
```

### Règles de conversion

1. **Nombres vers chaînes** : Automatique lors de la concaténation
2. **Booléens vers chaînes** : Automatique (`gaskiya`/`karya`)
3. **Autres conversions** : Non supportées actuellement

## Vérification de types

### Types dynamiques

Dabara utilise un typage dynamique - les types sont déterminés à l'exécution :

```hausa
fara
  naɗa abu = 42           // Lambar
  rubuta abu
  
  abu = "Sannu"           // Maintenant Jimla
  rubuta abu
  
  abu = gaskiya          // Maintenant Boolean
  rubuta abu
ƙare
```

### Erreurs de types

Certaines opérations peuvent échouer si les types ne sont pas compatibles :

```hausa
fara
  naɗa lambar = 10
  naɗa jimla = "Sannu"
  
  // Ceci causera une erreur :
  naɗa kuskure = lambar ƙara jimla
ƙare
```

**Message d'erreur :**
```
Kuskure: Ba za a iya ƙara lambar da jimla ba
```

## Exemples pratiques

### 1. Calculs mathématiques
```hausa
fara
  naɗa lambar1 = 15
  naɗa lambar2 = 7
  
  naɗa jimla = lambar1 ƙara lambar2
  naɗa bambanci = lambar1 rage lambar2
  
  rubuta "Jimla: "
  rubuta jimla        // 22
  rubuta "Bambanci: "
  rubuta bambanci     // 8
ƙare
```

### 2. Gestion de texte
```hausa
fara
  naɗa sunan = "Fatima"
  naɗa shekarun = 30
  
  naɗa bayanai = "Sunanta: " + sunan + ", Shekarunta: " + shekarun
  rubuta bayanai
  // Affiche: Sunanta: Fatima, Shekarunta: 30
ƙare
```

### 3. États booléens
```hausa
fara
  naɗa yana_yin_aiki = gaskiya
  naɗa yana_hutawa = karya
  
  rubuta "Ana aiki: " + yana_yin_aiki    // Ana aiki: gaskiya
  rubuta "Ana hutawa: " + yana_hutawa    // Ana hutawa: karya
ƙare
```

## Limites actuelles

### Types non supportés (v0.0.1)
- Nombres à virgule flottante
- Tableaux/listes
- Objets/structures
- Pointeurs/références

### Prévisions futures (v0.1.0+)
```hausa
// Futurs types planifiés :
naɗa lambar_ƙarami = 3.14159    // Nombres décimaux
naɗa jerin_lambobi = [1, 2, 3]   // Listes
naɗa abu_hadaka = {sunan: "Ahmad", shekarun: 25}  // Objets
```

## Bonnes pratiques

### 1. Nommage clair
```hausa
// ✅ Bon
naɗa shekarun_ɗalibi = 20
naɗa sunan_makaranta = "Université de Niamey"

// ❌ À éviter
naɗa s = 20
naɗa x = "Université"
```

### 2. Types cohérents
```hausa
// ✅ Bon - utilisez le même type pour des données similaires
naɗa lambar_farko = 10
naɗa lambar_biyu = 20
naɗa jimla = lambar_farko ƙara lambar_biyu

// ❌ À éviter - mélange de types sans raison
naɗa abu = 10
abu = "Sannu"  // Changement de type inutile
```

### 3. Validation des types
```hausa
// Toujours vérifier la compatibilité avant les opérations
naɗa lambar1 = 10
naɗa lambar2 = 5
// Assurez-vous que les deux sont des nombres avant l'addition
naɗa resultat = lambar1 ƙara lambar2
```

## Messages d'erreur liés aux types

| Erreur | Message en Haoussa | Cause |
|--------|-------------------|-------|
| Type incompatible | `Kuskure: Ba za a iya ƙara lambar da jimla ba` | Opération entre types incompatibles |
| Variable non définie | `Kuskure: Babu irin wannan mai canjin 'sunan'` | Utilisation d'une variable non déclarée |
| Type attendu | `Kuskure: Ana tsammanin 'lambar', amma an samu 'jimla'` | Type incorrect pour l'opération |

## Prochaines étapes

- [Variables](./variables.md) - Comment déclarer et utiliser les variables
- [Opérateurs](./operators.md) - Opérations sur les types
- [Expressions](./expressions.md) - Combiner types et opérateurs
