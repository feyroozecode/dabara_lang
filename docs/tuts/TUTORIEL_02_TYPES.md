# Tutoriel 02 - Les Types de Données

**Difficulté** : 🟢 FACILE  
**Durée estimée** : 30 minutes  
**Prérequis** : [Tutoriel 01](TUTORIEL_01_BASES.md)

---

## 📖 Objectifs de ce Tutoriel

À la fin de ce tutoriel, vous saurez :
- ✅ Comprendre les trois types de base : nombres, chaînes, booléens
- ✅ Convertir entre différents types
- ✅ Reconnaître et corriger les erreurs de type
- ✅ Utiliser les listes (collections)

---

## 1️⃣ Les Trois Types Fondamentaux

En Dabara, il existe trois types de données de base :

| Type | Hausa | Exemples | Utilisation |
|------|-------|----------|-------------|
| **Nombre** | Lambar | `42`, `0`, `-5` | Calculs mathématiques |
| **Chaîne** | Jimla/Rubutu | `"Sannu"`, `"123"` | Texte et messages |
| **Booléen** | Gaskiya/Karya | `gaskiya`, `karya` | Vrai/Faux, conditions |

---

## 2️⃣ Les Nombres (Lambar)

Les nombres sont utilisés pour représenter des valeurs numériques.

### Déclaration de Nombres

```hausa
fara
  naɗa lambar1 = 42
  naɗa lambar2 = 0
  naɗa lambar3 = -15
  
  rubuta lambar1
  rubuta lambar2
  rubuta lambar3
ƙare
```

**Sortie** :
```
42
0
-15
```

### 🔍 Caractéristiques

- ✅ Nombres entiers (pas de décimales pour l'instant)
- ✅ Peuvent être positifs, négatifs ou zéro
- ✅ Utilisés pour les calculs arithmétiques

### Exemples Pratiques

**Compter l'âge**
```hausa
fara
  naɗa shekarun = 25
  rubuta "Shekaruna: "
  rubuta shekarun
ƙare
```

**Quantités**
```hausa
fara
  naɗa littattafai = 12  # nombre de livres
  naɗa dalibai = 30      # nombre d'étudiants
  
  rubuta "Littattafai: "
  rubuta littattafai
  rubuta "Dalibai: "
  rubuta dalibai
ƙare
```

### ❌ Erreurs Courantes

**Erreur : Utiliser des décimales**
```hausa
fara
  naɗa lambar = 3.14  # ❌ Pas encore supporté
ƙare
```

**Correction** : Pour l'instant, utilisez uniquement des entiers
```hausa
fara
  naɗa lambar = 314  # ✅ Représentez 3.14 comme 314
ƙare
```

### Exercice 1

Créez un programme qui déclare :
- Le nombre d'élèves dans votre classe
- Votre note sur 100
- L'année en cours

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa dalibai = 35
  naɗa daraja = 87
  naɗa shekara = 2025
  
  rubuta "Yawan dalibai: "
  rubuta dalibai
  rubuta "Daraja: "
  rubuta daraja
  rubuta "Shekara: "
  rubuta shekara
ƙare
```
</details>

---

## 3️⃣ Les Chaînes de Caractères (Jimla)

Les chaînes représentent du **texte**. Elles sont toujours entre guillemets doubles.

### Déclaration de Chaînes

```hausa
fara
  naɗa sallama = "Sannu!"
  naɗa sunan = "Fatima"
  naɗa gari = "Kano"
  
  rubuta sallama
  rubuta sunan
  rubuta gari
ƙare
```

**Sortie** :
```
Sannu!
Fatima
Kano
```

### Chaînes Spéciales

**Chaîne vide**
```hausa
fara
  naɗa babu_kome = ""
  rubuta babu_kome  # Affiche une ligne vide
ƙare
```

**Chaînes avec caractères haoussa**
```hausa
fara
  naɗa jimla = "Ƙarfi da ƙwazo"
  rubuta jimla
ƙare
```

**Chaînes avec chiffres**
```hausa
fara
  naɗa jimla = "Shekaruna 25"  # C'est du TEXTE, pas un nombre
  rubuta jimla
ƙare
```

### 🔍 Important : Nombre vs Chaîne

```hausa
fara
  naɗa lambar = 42      # Nombre (sans guillemets)
  naɗa jimla = "42"     # Chaîne (avec guillemets)
  
  rubuta lambar
  rubuta jimla
ƙare
```

**Sortie identique** :
```
42
42
```

Mais ce sont des **types différents** ! Cela a de l'importance pour les opérations.

### Concaténation (Combiner des Chaînes)

L'opérateur `+` combine des chaînes :

```hausa
fara
  naɗa sunan_farko = "Ahmad"
  naɗa sunan_iyali = "Ibrahim"
  naɗa cikakken_suna = sunan_farko + " " + sunan_iyali
  
  rubuta cikakken_suna
ƙare
```

**Sortie** :
```
Ahmad Ibrahim
```

### Exemples de Concaténation

**Message de bienvenue**
```hausa
fara
  naɗa sunan = "Aisha"
  naɗa sallama = "Sannu " + sunan + "! Ina kwana?"
  rubuta sallama
ƙare
```

**Sortie** :
```
Sannu Aisha! Ina kwana?
```

**Adresse complète**
```hausa
fara
  naɗa gida = "Gida Na 42"
  naɗa titi = "Titin Zinder"
  naɗa gari = "Kano"
  naɗa adireshi = gida + ", " + titi + ", " + gari
  
  rubuta adireshi
ƙare
```

**Sortie** :
```
Gida Na 42, Titin Zinder, Kano
```

### Exercice 2

Créez un programme qui construit une phrase complète à partir de :
- Un prénom
- Un nom de famille
- Une ville
- Et affiche : "Je m'appelle [Prénom Nom] et j'habite à [Ville]"

En Haoussa : "Sunana [Prénom Nom] kuma ina zaune a [Ville]"

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa suna_farko = "Musa"
  naɗa suna_iyali = "Garba"
  naɗa gari = "Maradi"
  
  naɗa jimla = "Sunana " + suna_farko + " " + suna_iyali + " kuma ina zaune a " + gari
  
  rubuta jimla
ƙare
```

**Sortie** :
```
Sunana Musa Garba kuma ina zaune a Maradi
```
</details>

---

## 4️⃣ Les Booléens (Gaskiya/Karya)

Les booléens représentent **vrai** ou **faux**. C'est essentiel pour les conditions.

### Les Deux Valeurs

| Hausa | Français | Valeur |
|-------|----------|--------|
| `gaskiya` | vérité/vrai | `true` |
| `karya` | mensonge/faux | `false` |

### Déclaration de Booléens

```hausa
fara
  naɗa gaskiya_ne = gaskiya
  naɗa karya_ne = karya
  
  rubuta gaskiya_ne
  rubuta karya_ne
ƙare
```

**Sortie** :
```
gaskiya
karya
```

### Utilisation Pratique

**Statut**
```hausa
fara
  naɗa an_gama = gaskiya
  naɗa akwai_kuskure = karya
  
  rubuta "An gama: "
  rubuta an_gama
  rubuta "Akwai kuskure: "
  rubuta akwai_kuskure
ƙare
```

**Permissions**
```hausa
fara
  naɗa yana_da_izini = gaskiya
  naɗa ya_biya = karya
  
  rubuta "Yana da izini: "
  rubuta yana_da_izini
  rubuta "Ya biya: "
  rubuta ya_biya
ƙare
```

### 🔍 Pourquoi les Booléens ?

Les booléens sont essentiels pour :
- Prendre des décisions (conditions)
- Vérifier des états
- Contrôler le flux du programme

Nous les utiliserons beaucoup dans le [Tutoriel 04 - Conditions](TUTORIEL_04_CONDITIONS.md).

---

## 5️⃣ Les Listes (Jerin)

Les listes permettent de stocker **plusieurs valeurs** dans une seule variable.

### Syntaxe

```hausa
naɗa jerin = [valeur1, valeur2, valeur3]
```

### Exemples de Listes

**Liste de nombres**
```hausa
fara
  naɗa lambobi = [1, 2, 3, 4, 5]
  rubuta lambobi
ƙare
```

**Sortie** :
```
[1, 2, 3, 4, 5]
```

**Liste de chaînes**
```hausa
fara
  naɗa sunaye = ["Ahmad", "Fatima", "Musa"]
  rubuta sunaye
ƙare
```

**Sortie** :
```
[Ahmad, Fatima, Musa]
```

**Liste vide**
```hausa
fara
  naɗa jerin_babu = []
  rubuta jerin_babu
ƙare
```

**Sortie** :
```
[]
```

### Listes Mixtes

Vous pouvez mélanger différents types (mais c'est rarement une bonne idée) :

```hausa
fara
  naɗa hade = [1, "Ahmad", gaskiya, 42]
  rubuta hade
ƙare
```

**Sortie** :
```
[1, Ahmad, gaskiya, 42]
```

### 🔍 Utilisations Pratiques

**Liste de courses**
```hausa
fara
  naɗa kayan_kasuwa = ["Shinkafa", "Miya", "Nama", "Kayan marmari"]
  rubuta "Kayan da za mu saya:"
  rubuta kayan_kasuwa
ƙare
```

**Notes d'examens**
```hausa
fara
  naɗa darajoji = [85, 92, 78, 88, 95]
  rubuta "Darajoji:"
  rubuta darajoji
ƙare
```

**Villes visitées**
```hausa
fara
  naɗa birane = ["Kano", "Zinder", "Sokoto", "Maradi"]
  rubuta "Biranen da na ziyarta:"
  rubuta birane
ƙare
```

### Exercice 3

Créez trois listes :
1. Une liste de vos 5 films préférés
2. Une liste de 5 nombres (vos notes)
3. Une liste vide

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa fina_finan = ["Film 1", "Film 2", "Film 3", "Film 4", "Film 5"]
  naɗa darajoji = [87, 92, 78, 85, 90]
  naɗa babu_kome = []
  
  rubuta "Fina-finai:"
  rubuta fina_finan
  rubuta "Darajoji:"
  rubuta darajoji
  rubuta "Jerin babu:"
  rubuta babu_kome
ƙare
```
</details>

---

## 6️⃣ Comparaison des Types

Voyons comment différencier visuellement les types :

### Tableau Récapitulatif

```hausa
fara
  # Nombres (sans guillemets)
  naɗa lambar = 42
  
  # Chaînes (avec guillemets)
  naɗa jimla = "42"
  
  # Booléens (mots-clés spéciaux)
  naɗa boolean = gaskiya
  
  # Listes (entre crochets)
  naɗa jerin = [1, 2, 3]
  
  rubuta "Lambar:"
  rubuta lambar
  rubuta "Jimla:"
  rubuta jimla
  rubuta "Boolean:"
  rubuta boolean
  rubuta "Jerin:"
  rubuta jerin
ƙare
```

**Sortie** :
```
Lambar:
42
Jimla:
42
Boolean:
gaskiya
Jerin:
[1, 2, 3]
```

### 🎯 Comment Reconnaître le Type

| Si vous voyez... | C'est un... | Exemple |
|------------------|-------------|---------|
| Un nombre sans guillemets | **Nombre** | `42`, `-5`, `0` |
| Du texte entre `" "` | **Chaîne** | `"Sannu"`, `"123"` |
| `gaskiya` ou `karya` | **Booléen** | `gaskiya`, `karya` |
| Des crochets `[ ]` | **Liste** | `[1, 2, 3]` |

---

## 7️⃣ Erreurs de Type Courantes

### Erreur 1 : Oublier les Guillemets

❌ **Incorrect**
```hausa
fara
  naɗa sallama = Sannu  # ❌ Dabara pense que "Sannu" est une variable
ƙare
```

**Erreur** : `Babu irin wannan mai canjin 'Sannu'`

✅ **Correct**
```hausa
fara
  naɗa sallama = "Sannu"  # ✅ Chaîne avec guillemets
ƙare
```

### Erreur 2 : Mélanger les Types

❌ **Peut causer des problèmes**
```hausa
fara
  naɗa lambar = 42
  naɗa jimla = "Le nombre est " + lambar  # ⚠️ Peut ne pas fonctionner
ƙare
```

Pour l'instant, convertissez manuellement :
```hausa
fara
  naɗa lambar = 42
  naɗa jimla = "Le nombre est 42"  # ✅ Tout en texte
ƙare
```

### Erreur 3 : Virgules dans les Listes

❌ **Incorrect**
```hausa
fara
  naɗa jerin = [1 2 3]  # ❌ Manque les virgules
ƙare
```

✅ **Correct**
```hausa
fara
  naɗa jerin = [1, 2, 3]  # ✅ Virgules entre les éléments
ƙare
```

---

## 8️⃣ Programme Complet : Profil Étudiant

Créons un programme qui utilise tous les types de données.

**Fichier : `profil_dalibi.ha`**

```hausa
# Programme : Profil d'étudiant
# Utilise tous les types de données

fara
  # Informations personnelles (chaînes)
  naɗa sunan = "Hauwa Abdullahi"
  naɗa gari = "Niamey"
  naɗa daraja_aji = "Aji na 10"
  
  # Statistiques (nombres)
  naɗa shekarun = 16
  naɗa lambar_dalibi = 2025042
  naɗa yawan_darajoji = 8
  
  # Statuts (booléens)
  naɗa yana_zuwa = gaskiya
  naɗa ya_biya_kudi = gaskiya
  
  # Matières (liste)
  naɗa darussa = ["Lissafi", "Kimiyya", "Harshe", "Tarihi", "Geography"]
  
  # Affichage du profil
  rubuta "======================================"
  rubuta "       PROFIL DALIBI / STUDENT       "
  rubuta "======================================"
  rubuta "Suna: " + sunan
  rubuta "Gari: " + gari
  rubuta "Daraja: " + daraja_aji
  rubuta ""
  rubuta "Shekarun: "
  rubuta shekarun
  rubuta "Lambar dalibi: "
  rubuta lambar_dalibi
  rubuta "Yawan darajoji: "
  rubuta yawan_darajoji
  rubuta ""
  rubuta "Yana zuwa: "
  rubuta yana_zuwa
  rubuta "Ya biya kudi: "
  rubuta ya_biya_kudi
  rubuta ""
  rubuta "Darussa:"
  rubuta darussa
  rubuta "======================================"
ƙare
```

**Sortie** :
```
======================================
       PROFIL DALIBI / STUDENT       
======================================
Suna: Hauwa Abdullahi
Gari: Niamey
Daraja: Aji na 10

Shekarun: 16
Lambar dalibi: 2025042
Yawan darajoji: 8

Yana zuwa: gaskiya
Ya biya kudi: gaskiya

Darussa:
[Lissafi, Kimiyya, Harshe, Tarihi, Geography]
======================================
```

---

## 📝 Résumé du Tutoriel

| Type | Déclaration | Exemple | Utilisation |
|------|-------------|---------|-------------|
| **Nombre** | `naɗa x = 42` | `100`, `-5` | Calculs |
| **Chaîne** | `naɗa s = "texte"` | `"Sannu"` | Texte |
| **Booléen** | `naɗa b = gaskiya` | `gaskiya`, `karya` | Vrai/Faux |
| **Liste** | `naɗa l = [1,2,3]` | `["a", "b"]` | Collections |

---

## 🎯 Points Clés à Retenir

1. ✅ **Nombres** : pas de guillemets → `42`
2. ✅ **Chaînes** : entre guillemets → `"Sannu"`
3. ✅ **Booléens** : `gaskiya` ou `karya`
4. ✅ **Listes** : entre crochets → `[1, 2, 3]`
5. ✅ Utilisez `+` pour combiner des chaînes
6. ✅ Les virgules séparent les éléments d'une liste

---

## 🚀 Projet Pratique : Inventaire Personnel

Créez un fichier `kayan_gida.ha` (inventaire de la maison) qui déclare :

1. **Liste d'objets** : au moins 5 objets dans votre chambre
2. **Leurs quantités** : liste de nombres correspondant
3. **Propriétaire** : votre nom (chaîne)
4. **Inventaire complet** : booléen indiquant si c'est complet

Affichez toutes ces informations de manière organisée.

<details>
<summary>💡 Voir une solution possible</summary>

```hausa
fara
  # Informations
  naɗa mai_shi = "Ibrahim Yusuf"
  naɗa an_gama = gaskiya
  
  # Inventaire
  naɗa kayayyaki = ["Littattafai", "Alƙalami", "Kwamfuta", "Tebur", "Kujera"]
  naɗa adadi = [15, 20, 1, 1, 2]
  
  # Affichage
  rubuta "================================"
  rubuta "   KAYAN GIDA / INVENTAIRE"
  rubuta "================================"
  rubuta "Mai shi: " + mai_shi
  rubuta "An gama: "
  rubuta an_gama
  rubuta ""
  rubuta "Kayayyaki:"
  rubuta kayayyaki
  rubuta ""
  rubuta "Adadi:"
  rubuta adadi
  rubuta "================================"
ƙare
```
</details>

---

## 🎓 Quiz de Validation

1. Quelle est la différence entre `42` et `"42"` ?
2. Comment créer une liste vide ?
3. Quels sont les deux valeurs booléennes en Dabara ?
4. Comment combiner deux chaînes ?
5. Peut-on avoir des nombres négatifs ?

<details>
<summary>📖 Voir les réponses</summary>

1. `42` est un **nombre**, `"42"` est une **chaîne** de texte
2. `naɗa jerin = []`
3. `gaskiya` (vrai) et `karya` (faux)
4. Avec l'opérateur `+` : `"Sannu " + sunan`
5. Oui : `naɗa x = -10`
</details>

---

## ➡️ Prochaine Étape

Maintenant que vous connaissez les types de données, apprenons à faire des calculs !

➡️ [**Tutoriel 03 - Opérations Arithmétiques**](TUTORIEL_03_OPERATIONS.md)

---

**Ci gaba da koyo!** (Continuez à apprendre!) 📚✨
