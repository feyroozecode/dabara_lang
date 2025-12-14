# Tutoriel 07 - Fonctions et Réutilisation

**Difficulté** : 🟡 MOYEN  
**Durée estimée** : 50 minutes  
**Prérequis** : [Tutoriels 01-06](TUTORIEL_01_BASES.md)

---

## 📖 Objectifs de ce Tutoriel

À la fin de ce tutoriel, vous saurez :
- ✅ Créer des fonctions avec `aiki`
- ✅ Passer des paramètres aux fonctions
- ✅ Retourner des valeurs avec `mayar`
- ✅ Réutiliser votre code efficacement
- ✅ Organiser vos programmes

---

## 1️⃣ Qu'est-ce qu'une Fonction ?

Une fonction est un **bloc de code réutilisable** qui effectue une tâche spécifique.

### Pourquoi utiliser des fonctions ?

- ✅ **Réutilisation** : Écrire une fois, utiliser plusieurs fois
- ✅ **Organisation** : Diviser un grand programme en petites parties
- ✅ **Lisibilité** : Code plus facile à comprendre
- ✅ **Maintenance** : Plus facile à modifier et déboguer

### Analogie

Pensez à une fonction comme une **recette de cuisine** :
- **Nom** : "Préparer du riz"
- **Ingrédients** : Riz, eau, sel (paramètres)
- **Instructions** : Étapes à suivre (code)
- **Résultat** : Riz cuit (valeur de retour)

---

## 2️⃣ Créer une Fonction Simple

### Syntaxe de Base

```hausa
aiki nom_fonction(parametres) {
  # Code de la fonction
  mayar valeur_de_retour
}
```

### Exemple Simple

```hausa
fara
  aiki gaisuwa() {
    mayar "Sannu Duniya!"
  }
  
  naɗa sako = gaisuwa()
  rubuta sako
ƙare
```

**Sortie** :
```
Sannu Duniya!
```

### 🔍 Décortiquons

1. `aiki gaisuwa()` → Définit une fonction nommée `gaisuwa`
2. `mayar "Sannu Duniya!"` → Retourne une chaîne
3. `gaisuwa()` → Appelle la fonction
4. `naɗa sako = ...` → Stocke le résultat

---

## 3️⃣ Le Mot-Clé `aiki`

`aiki` signifie "travail" ou "tâche" en haoussa. C'est le mot-clé pour définir une fonction.

### Structure

```hausa
aiki nom_de_la_fonction(parametre1, parametre2) {
  # Instructions
  mayar resultat
}
```

### Règles de Nommage

✅ **Noms Valides**
```hausa
aiki jimla() { ... }
aiki ƙidaya() { ... }
aiki lissafi_daraja() { ... }
aiki darabaƙarami() { ... }
```

❌ **Noms Invalides**
```hausa
aiki 1fonction() { ... }    # Ne peut pas commencer par un chiffre
aiki ma-fonction() { ... }   # Pas de tirets
aiki fara() { ... }          # Mot-clé réservé
```

---

## 4️⃣ Fonctions Sans Paramètres

Les fonctions les plus simples ne prennent aucun paramètre.

### Exemple 1 : Afficher un Message

```hausa
fara
  aiki maraba() {
    mayar "Barka da zuwa cikin Dabara!"
  }
  
  naɗa sako = maraba()
  rubuta sako
ƙare
```

**Sortie** :
```
Barka da zuwa cikin Dabara!
```

### Exemple 2 : Retourner un Nombre

```hausa
fara
  aiki lambar_magic() {
    mayar 42
  }
  
  naɗa lambar = lambar_magic()
  rubuta lambar
ƙare
```

**Sortie** :
```
42
```

### Exercice 1

Créez une fonction `obtenir_annee()` qui retourne l'année en cours (2025).

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  aiki obtenir_annee() {
    mayar 2025
  }
  
  naɗa shekara = obtenir_annee()
  rubuta "Shekara: "
  rubuta shekara
ƙare
```
</details>

---

## 5️⃣ Fonctions Avec Paramètres

Les paramètres permettent de **passer des valeurs** à une fonction.

### Syntaxe

```hausa
aiki nom_fonction(param1, param2) {
  # Utiliser param1 et param2
  mayar resultat
}
```

### Exemple : Addition de Deux Nombres

```hausa
fara
  aiki jimla(a, b) {
    mayar a + b
  }
  
  naɗa resultat = jimla(5, 3)
  rubuta resultat
ƙare
```

**Sortie** :
```
8
```

### 🔍 Comment ça Marche ?

1. Appel : `jimla(5, 3)`
2. `a` reçoit `5`, `b` reçoit `3`
3. Calcul : `a + b` → `5 + 3` → `8`
4. Retour : `mayar 8`
5. Résultat stocké dans `resultat`

### Exemple : Salutation Personnalisée

```hausa
fara
  aiki gaisuwa(suna) {
    mayar "Sannu " + suna + "!"
  }
  
  naɗa sako1 = gaisuwa("Ahmad")
  naɗa sako2 = gaisuwa("Fatima")
  
  rubuta sako1
  rubuta sako2
ƙare
```

**Sortie** :
```
Sannu Ahmad!
Sannu Fatima!
```

### Exercice 2

Créez une fonction `darabawa(a, b)` qui multiplie deux nombres.

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  aiki darabawa(a, b) {
    mayar a * b
  }
  
  naɗa resultat1 = darabawa(4, 7)
  naɗa resultat2 = darabawa(10, 3)
  
  rubuta resultat1  # 28
  rubuta resultat2  # 30
ƙare
```
</details>

---

## 6️⃣ Le Mot-Clé `mayar` (Return)

`mayar` signifie "retourner" en haoussa. Il permet à une fonction de **renvoyer une valeur**.

### Syntaxe

```hausa
mayar valeur
```

### Exemple : Fonction avec Retour

```hausa
fara
  aiki square(x) {
    mayar x * x
  }
  
  naɗa resultat = square(5)
  rubuta resultat
ƙare
```

**Sortie** :
```
25
```

### 🔍 Important

- `mayar` **termine** l'exécution de la fonction immédiatement
- Le code après `mayar` n'est **jamais exécuté**
- Une fonction peut avoir **plusieurs `mayar`** (avec conditions)

### Exemple : Retour Conditionnel

```hausa
fara
  aiki maximum(a, b) {
    idan a > b {
      mayar a
    } amma {
      mayar b
    }
  }
  
  naɗa max_val = maximum(10, 5)
  rubuta max_val
ƙare
```

**Sortie** :
```
10
```

---

## 7️⃣ Fonctions avec Plusieurs Paramètres

Vous pouvez passer autant de paramètres que nécessaire.

### Exemple : Calculer une Moyenne

```hausa
fara
  aiki matsakaici(a, b, c) {
    naɗa jimla = a + b + c
    mayar jimla / 3
  }
  
  naɗa daraja = matsakaici(85, 90, 78)
  rubuta "Matsakaicin daraja: "
  rubuta daraja
ƙare
```

**Sortie** :
```
Matsakaicin daraja: 84
```

### Exemple : Informations Complètes

```hausa
fara
  aiki cikakken_suna(suna_farko, suna_iyali, taken) {
    mayar taken + " " + suna_farko + " " + suna_iyali
  }
  
  naɗa suna = cikakken_suna("Ahmad", "Ibrahim", "Alhaji")
  rubuta suna
ƙare
```

**Sortie** :
```
Alhaji Ahmad Ibrahim
```

### Exercice 3

Créez une fonction `yanki_rectangle(tsawo, faɗi)` qui calcule l'aire d'un rectangle.

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  aiki yanki_rectangle(tsawo, faɗi) {
    mayar tsawo * faɗi
  }
  
  naɗa yanki = yanki_rectangle(10, 5)
  rubuta "Yankin rectangle: "
  rubuta yanki
ƙare
```

**Sortie** :
```
Yankin rectangle: 50
```
</details>

---

## 8️⃣ Appeler des Fonctions Depuis d'Autres Fonctions

Les fonctions peuvent s'appeler entre elles.

### Exemple

```hausa
fara
  aiki double(x) {
    mayar x * 2
  }
  
  aiki quadruple(x) {
    mayar double(double(x))
  }
  
  naɗa resultat = quadruple(3)
  rubuta resultat
ƙare
```

**Sortie** :
```
12
```

### 🔍 Décomposition

1. `quadruple(3)` appelle `double(double(3))`
2. `double(3)` retourne `6`
3. `double(6)` retourne `12`
4. Résultat final : `12`

---

## 9️⃣ Fonctions Récursives

Une fonction récursive **s'appelle elle-même**. C'est très puissant mais attention à bien définir une condition d'arrêt !

### Exemple : Factorielle

```hausa
fara
  aiki factorial(n) {
    idan n == 0 {
      mayar 1
    } amma {
      mayar n * factorial(n - 1)
    }
  }
  
  naɗa fact5 = factorial(5)
  rubuta "Factorial de 5: "
  rubuta fact5
ƙare
```

**Sortie** :
```
Factorial de 5: 120
```

### 🔍 Comment ça Marche ?

```
factorial(5)
  → 5 * factorial(4)
    → 4 * factorial(3)
      → 3 * factorial(2)
        → 2 * factorial(1)
          → 1 * factorial(0)
            → 1
          → 1 * 1 = 1
        → 2 * 1 = 2
      → 3 * 2 = 6
    → 4 * 6 = 24
  → 5 * 24 = 120
```

### ⚠️ Attention

Une fonction récursive DOIT avoir :
1. **Cas de base** : condition d'arrêt (`n == 0`)
2. **Appel récursif** : qui se rapproche du cas de base (`n - 1`)

Sans cas de base, la fonction s'appellerait **infiniment** !

### Exercice 4

Créez une fonction récursive `jimla_zuwa_n(n)` qui calcule la somme de 1 à n.
Par exemple : `jimla_zuwa_n(5)` → `1 + 2 + 3 + 4 + 5 = 15`

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  aiki jimla_zuwa_n(n) {
    idan n == 0 {
      mayar 0
    } amma {
      mayar n + jimla_zuwa_n(n - 1)
    }
  }
  
  naɗa resultat = jimla_zuwa_n(5)
  rubuta "Jimla 1 zuwa 5: "
  rubuta resultat
ƙare
```

**Sortie** :
```
Jimla 1 zuwa 5: 15
```
</details>

---

## 🔟 Portée des Variables (Scope)

Les variables définies **à l'intérieur** d'une fonction sont **locales** à cette fonction.

### Variables Locales

```hausa
fara
  naɗa x = 100  # Variable globale
  
  aiki canja() {
    naɗa x = 999  # Variable LOCALE (différente)
    mayar x
  }
  
  naɗa local_x = canja()
  
  rubuta "x global: "
  rubuta x          # 100
  rubuta "x local: "
  rubuta local_x    # 999
ƙare
```

**Sortie** :
```
x global: 100
x local: 999
```

### 🔍 Important

- La variable `x` dans `canja()` est **différente** de la variable `x` globale
- Elles ont le **même nom** mais sont **isolées**
- Chaque fonction a son propre **espace de variables**

---

## 1️⃣1️⃣ Programme Complet : Bibliothèque de Calculs

```hausa
# Programme : Bibliothèque de fonctions mathématiques
# Auteur : Dabara Tutorial

fara
  # Fonction : Addition
  aiki jimla(a, b) {
    mayar a + b
  }
  
  # Fonction : Soustraction
  aiki bambanci(a, b) {
    mayar a - b
  }
  
  # Fonction : Multiplication
  aiki ninka(a, b) {
    mayar a * b
  }
  
  # Fonction : Division
  aiki raba(a, b) {
    mayar a / b
  }
  
  # Fonction : Maximum
  aiki max(a, b) {
    idan a > b {
      mayar a
    } amma {
      mayar b
    }
  }
  
  # Fonction : Minimum
  aiki min(a, b) {
    idan a < b {
      mayar a
    } amma {
      mayar b
    }
  }
  
  # Tests
  rubuta "==================================="
  rubuta "  LABURAREN LISSAFI / MATH LIB"
  rubuta "==================================="
  
  naɗa a = 15
  naɗa b = 7
  
  rubuta "Lambobi: "
  rubuta a
  rubuta b
  rubuta "-----------------------------------"
  
  rubuta "Jimla: "
  rubuta jimla(a, b)
  
  rubuta "Bambanci: "
  rubuta bambanci(a, b)
  
  rubuta "Ninka: "
  rubuta ninka(a, b)
  
  rubuta "Raba: "
  rubuta raba(a, b)
  
  rubuta "Maximum: "
  rubuta max(a, b)
  
  rubuta "Minimum: "
  rubuta min(a, b)
  
  rubuta "==================================="
ƙare
```

**Sortie** :
```
=================================
  LABURAREN LISSAFI / MATH LIB
=================================
Lambobi: 15 7
-----------------------------------
Jimla: 22
Bambanci: 8
Ninka: 105
Raba: 2
Maximum: 15
Minimum: 7
=================================
```

---

## 📝 Résumé du Tutoriel

### Mots-Clés

| Mot-Clé | Signification | Utilisation |
|---------|---------------|-------------|
| `aiki` | travail/fonction | Définir une fonction |
| `mayar` | retourner | Retourner une valeur |

### Structure d'une Fonction

```hausa
aiki nom(param1, param2) {
  # Code
  mayar valeur
}
```

### Appel de Fonction

```hausa
naɗa resultat = nom(valeur1, valeur2)
```

---

## 🎯 Points Clés à Retenir

1. ✅ `aiki` définit une fonction
2. ✅ `mayar` retourne une valeur
3. ✅ Les paramètres sont entre parenthèses
4. ✅ Les fonctions peuvent s'appeler entre elles
5. ✅ La récursion nécessite un cas de base
6. ✅ Les variables locales sont isolées

---

## 🚀 Projet Final : Calculateur de Notes

Créez un programme `ƙididdigan_daraja.ha` avec les fonctions :

1. `jimlar_darajoji(d1, d2, d3, d4)` - Somme de 4 notes
2. `matsakaici(jimla, yawa)` - Moyenne
3. `daraja_harfi(matsakaici)` - Convertit en lettre (A, B, C, D, F)
4. Utilisez ces fonctions pour calculer et afficher :
   - 4 notes d'examens
   - Leur somme
   - Leur moyenne
   - La note en lettre

<details>
<summary>💡 Voir une solution possible</summary>

```hausa
fara
  # Fonctions
  aiki jimlar_darajoji(d1, d2, d3, d4) {
    mayar d1 + d2 + d3 + d4
  }
  
  aiki matsakaici(jimla, yawa) {
    mayar jimla / yawa
  }
  
  aiki daraja_harfi(mat) {
    idan mat >= 90 {
      mayar "A"
    } ammaina mat >= 80 {
      mayar "B"
    } ammaina mat >= 70 {
      mayar "C"
    } ammaina mat >= 60 {
      mayar "D"
    } amma {
      mayar "F"
    }
  }
  
  # Données
  naɗa daraja1 = 85
  naɗa daraja2 = 92
  naɗa daraja3 = 78
  naɗa daraja4 = 88
  
  # Calculs
  naɗa jimla = jimlar_darajoji(daraja1, daraja2, daraja3, daraja4)
  naɗa mat = matsakaici(jimla, 4)
  naɗa harfi = daraja_harfi(mat)
  
  # Affichage
  rubuta "================================"
  rubuta "  ƘIDIDDIGAN DARAJA"
  rubuta "================================"
  rubuta "Darajoji:"
  rubuta daraja1
  rubuta daraja2
  rubuta daraja3
  rubuta daraja4
  rubuta "--------------------------------"
  rubuta "Jimla: "
  rubuta jimla
  rubuta "Matsakaici: "
  rubuta mat
  rubuta "Daraja (harfi): " + harfi
  rubuta "================================"
ƙare
```

**Sortie** :
```
Jimla: 343
Matsakaici: 85
Daraja (harfi): B
```
</details>

---

## 🎓 Quiz de Validation

1. Quelle est la différence entre définir et appeler une fonction ?
2. À quoi sert `mayar` ?
3. Combien de paramètres peut avoir une fonction ?
4. Qu'est-ce qu'une fonction récursive ?
5. Qu'est-ce qu'une variable locale ?

<details>
<summary>📖 Voir les réponses</summary>

1. **Définir** = créer la fonction avec `aiki`. **Appeler** = l'utiliser avec `nom()`
2. `mayar` retourne une valeur de la fonction
3. Autant que nécessaire (0, 1, 2, 3...)
4. Une fonction qui s'appelle elle-même
5. Variable créée à l'intérieur d'une fonction, non accessible dehors
</details>

---

## ➡️ Prochaine Étape

Maintenant que vous maîtrisez les fonctions, approfondissons avec la récursivité et la portée !

➡️ [**Tutoriel 08 - Récursivité et Portée Avancée**](TUTORIEL_08_RECURSION.md)

---

**Ka iya yin ayyuka yanzu!** (Vous pouvez créer des fonctions maintenant!) 🎯📦
