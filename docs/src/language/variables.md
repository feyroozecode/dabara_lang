# Variables# Variables

Les variables en Dabara permettent de stocker et manipuler des données. Ce chapitre explique comment déclarer, utiliser et gérer les variables.

## Déclaration de variables

### Syntaxe de base

En Dabara, les variables sont déclarées avec le mot-clé `naɗa` (ou `nada`) :

```hausa
fara
  naɗa sunan = "Ahmad"
  naɗa shekarun = 25
  naɗa yana_aiki = gaskiya
ƙare
```

### Variantes de déclaration

Pour l'accessibilité, vous pouvez utiliser les deux formes :

```hausa
fara
  // Forme Unicode (recommandée)
  naɗa sunan_unicode = "Khadija"
  
  // Forme latine (alternative)
  nada sunan_latin = "Fatima"
  
  // Les deux sont équivalentes
  rubuta sunan_unicode
  rubuta sunan_latin
ƙare
```

## Règles de nommage

### Identificateurs valides

Les noms de variables doivent respecter ces règles :

1. **Commencer par une lettre** ou un caractère haoussa
2. **Contenir uniquement** : lettres, chiffres, underscores (`_`)
3. **Être sensibles à la casse** : `Sunan` ≠ `sunan`
4. **Ne pas être un mot-clé** réservé

### Exemples valides

```hausa
fara
  // Noms simples
  naɗa sunan = "Ahmad"
  naɗa lambar = 42
  
  // Avec underscores
  naɗa sunan_farko = "Hassan"
  naɗa lambar_yara = 3
  
  // Avec chiffres
  naɗa wuri1 = "Kano"
  naɗa wuri2 = "Lagos"
  
  // Caractères haoussa
  naɗa ƙasa = "Nigeria"
  naɗa maƙaranta = "Université"
ƙare
```

### Exemples invalides

```hausa
// ❌ Ces noms sont invalides :
naɗa 2sunan = "Ahmad"        // Commence par un chiffre
naɗa su-nan = "Khadija"      // Contient un tiret
naɗa fara = "début"          // Mot-clé réservé
naɗa rubuta = "action"       // Mot-clé réservé
naɗa su nan = "espace"       // Contient un espace
```

## Types de variables

### Variables dynamiques

Dabara utilise un typage dynamique - une variable peut changer de type :

```hausa
fara
  naɗa abu = 42           // Nombre
  rubuta abu              // Affiche: 42
  
  abu = "Sannu"           // Maintenant une chaîne
  rubuta abu              // Affiche: Sannu
  
  abu = gaskiya          // Maintenant un booléen
  rubuta abu              // Affiche: gaskiya
ƙare
```

### Types supportés

1. **Lambar** (Nombres) : `42`, `0`, `-15`
2. **Jimla** (Chaînes) : `"Sannu"`, `"Ina kwana?"`
3. **Boolean** : `gaskiya`, `karya`

## Assignation de valeurs

### Assignation simple

```hausa
fara
  naɗa sunan = "Aisha"
  naɗa shekarun = 22
  naɗa sahihi = gaskiya
  
  rubuta sunan
  rubuta shekarun
  rubuta sahihi
ƙare
```

### Réassignation

```hausa
fara
  naɗa lambar = 10
  rubuta lambar          // Affiche: 10
  
  lambar = 20           // Nouvelle valeur
  rubuta lambar          // Affiche: 20
  
  lambar = lambar ƙara 5  // Utilise l'ancienne valeur
  rubuta lambar          // Affiche: 25
ƙare
```

### Assignation avec expressions

```hausa
fara
  naɗa a = 10
  naɗa b = 5
  
  naɗa jimla = a ƙara b
  naɗa bambanci = a rage b
  naɗa maganar = "Lambar: " + a
  
  rubuta jimla      // 15
  rubuta bambanci   // 5
  rubuta maganar    // Lambar: 10
ƙare
```

## Portée des variables (Scope)

### Portée globale

Toutes les variables déclarées dans le bloc principal sont globales :

```hausa
fara
  naɗa sunan_global = "Ahmad"
  naɗa lambar_global = 42
  
  // Ces variables sont accessibles partout dans le programme
  rubuta sunan_global
  rubuta lambar_global
ƙare
```

### Variables temporaires

```hausa
fara
  naɗa a = 10
  naɗa b = 5
  
  // Variable temporaire pour le calcul
  naɗa temp = a ƙara b
  naɗa resultat = temp ƙara 3
  
  rubuta resultat    // 18
ƙare
```

## Manipulation de variables

### Opérations arithmétiques

```hausa
fara
  naɗa lambar = 100
  
  // Addition
  lambar = lambar ƙara 50    // 150
  rubuta lambar
  
  // Soustraction
  lambar = lambar rage 25     // 125
  rubuta lambar
  
  // Avec d'autres variables
  naɗa bonus = 10
  lambar = lambar ƙara bonus  // 135
  rubuta lambar
ƙare
```

### Concaténation de chaînes

```hausa
fara
  naɗa sunan = "Ahmad"
  naɗa kari = " Bello"
  
  // Concaténation directe
  sunan = sunan + kari
  rubuta sunan           // Ahmad Bello
  
  // Avec littéraux
  sunan = sunan + " (Mai aiki)"
  rubuta sunan           // Ahmad Bello (Mai aiki)
ƙare
```

### Combinaison de types

```hausa
fara
  naɗa sunan = "Fatima"
  naɗa shekarun = 28
  
  // Combinaison automatique
  naɗa bayanai = sunan + " yana da shekaru " + shekarun
  rubuta bayanai         // Fatima yana da shekaru 28
ƙare
```

## Exemples pratiques

### 1. Calculatrice simple

```hausa
fara
  naɗa lambar1 = 25
  naɗa lambar2 = 17
  
  naɗa jimla = lambar1 ƙara lambar2
  naɗa bambanci = lambar1 rage lambar2
  
  rubuta "Lambobi: " + lambar1 + " da " + lambar2
  rubuta "Jimla: " + jimla
  rubuta "Bambanci: " + bambanci
ƙare
```

### 2. Profil utilisateur

```hausa
fara
  naɗa sunan_farko = "Khadija"
  naɗa sunan_iyali = "Ibrahim"
  naɗa shekarun = 24
  naɗa yana_aiki = gaskiya
  
  naɗa sunan_ciki = sunan_farko + " " + sunan_iyali
  naɗa bayanai = "Sunan: " + sunan_ciki + ", Shekaru: " + shekarun
  
  rubuta bayanai
  rubuta "Ana aiki: " + yana_aiki
ƙare
```

### 3. Gestion d'inventaire

```hausa
fara
  naɗa abu_sunan = "Littattafai"
  naɗa adadin_farko = 100
  naɗa adadin_sayar = 25
  naɗa adadin_shigo = 15
  
  naɗa adadin_yanzu = adadin_farko rage adadin_sayar
  adadin_yanzu = adadin_yanzu ƙara adadin_shigo
  
  rubuta "Abu: " + abu_sunan
  rubuta "Adadin farko: " + adadin_farko
  rubuta "An sayar: " + adadin_sayar
  rubuta "An shigo: " + adadin_shigo
  rubuta "Adadin yanzu: " + adadin_yanzu
ƙare
```

## Erreurs courantes

### 1. Variable non déclarée

```hausa
fara
  // ❌ Erreur : 'lambar' n'est pas déclarée
  rubuta lambar
ƙare
```

**Message d'erreur :**
```
Kuskure: Babu irin wannan mai canjin 'lambar'
```

### 2. Syntaxe de déclaration incorrecte

```hausa
fara
  // ❌ Erreur : manque 'naɗa'
  sunan = "Ahmad"
  
  // ✅ Correct
  naɗa sunan = "Ahmad"
ƙare
```

### 3. Opérations incompatibles

```hausa
fara
  naɗa lambar = 10
  naɗa jimla = "Sannu"
  
  // ❌ Erreur : ne peut pas additionner nombre et chaîne
  naɗa kuskure = lambar ƙara jimla
ƙare
```

## Bonnes pratiques

### 1. Noms descriptifs

```hausa
// ✅ Bon - noms clairs et descriptifs
naɗa sunan_ɗalibi = "Ahmad"
naɗa shekarun_makaranta = 20
naɗa adadin_littattafai = 15

// ❌ À éviter - noms vagues
naɗa s = "Ahmad"
naɗa n = 20
naɗa x = 15
```

### 2. Initialisation immédiate

```hausa
// ✅ Bon - initialiser lors de la déclaration
naɗa sunan = "Fatima"
naɗa lambar = 0

// ❌ À éviter - déclaration puis assignation
naɗa sunan
sunan = "Fatima"  // Pas supporté actuellement
```

### 3. Cohérence des types

```hausa
// ✅ Bon - utilisez des types cohérents
naɗa lambar1 = 10
naɗa lambar2 = 20
naɗa jimla_lambobi = lambar1 ƙara lambar2

// ❌ Attention - changements de type fréquents
naɗa abu = 10
abu = "Sannu"
abu = gaskiya  // Peut prêter à confusion
```

### 4. Commentaires explicatifs

```hausa
fara
  // Bayanin ɗalibi
  naɗa sunan_ɗalibi = "Ahmad"
  naɗa shekarun = 22
  
  // Lissafin jimla shekaru a makaranta
  naɗa shekarun_makaranta = shekarun rage 6
  
  rubuta "Shekaru a makaranta: " + shekarun_makaranta
ƙare
```

## Conventions de style

### CamelCase vs Snake_case

```hausa
// Style recommandé : snake_case
naɗa sunan_farko = "Ahmad"
naɗa lambar_yara = 3
naɗa yana_yin_aiki = gaskiya

// Alternative acceptable : camelCase
naɗa sunanFarko = "Ahmad"
naɗa lambarYara = 3
naɗa yanaYinAiki = gaskiya
```

### Constantes

```hausa
// Pour les valeurs qui ne changent pas, utilisez UPPERCASE
naɗa SHEKARUN_A_KOWACE_KARNI = 100
naɗa SUNAN_KASA = "Nigeria"
naɗa ADADIN_WATANNI = 12
```

## Limitations actuelles

### Version 0.0.1
- Pas de déclaration sans assignation
- Pas de constantes (toutes les variables sont mutables)
- Pas de variables locales (fonctions)
- Typage dynamique uniquement

### Futures améliorations (v0.1.0+)
```hausa
// Prévisions pour les futures versions :
kadai LAMBAR_MAX = 1000     // Constantes
daidai lambar = 42          // Variables immuables
jerinlambobi = [1, 2, 3]    // Types complexes
```

## Prochaines étapes

- [Opérateurs](./operators.md) - Opérations sur les variables
- [Expressions](./expressions.md) - Combiner variables et opérateurs
- [Exemples pratiques](../examples/variables.md) - Plus d'exemples d'usage
