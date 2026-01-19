# Tutoriel 01 - Les Bases : Variables et Affichage

**Difficulté** : 🟢 FACILE  
**Durée estimée** : 30 minutes  
**Prérequis** : Installation de Dabara

---

## 📖 Objectifs de ce Tutoriel

À la fin de ce tutoriel, vous saurez :
- ✅ Déclarer des variables avec `naɗa`
- ✅ Afficher du texte et des valeurs avec `rubuta`
- ✅ Comprendre la structure d'un programme
- ✅ Gérer les erreurs simples

---

## 1️⃣ Structure d'un Programme Dabara

Chaque programme Dabara suit cette structure :

```hausa
fara
  # Votre code ici
ƙare
```

### Règles Importantes

1. **Tout programme commence par `fara`** (commencer)
2. **Tout programme se termine par `ƙare`** ou `kare` (terminer)
3. **Le code est indenté** (décalé vers la droite) pour la lisibilité
4. **Les commentaires commencent par `#`**

### ❌ Exemple Incorrect

```hausa
# Manque 'fara' au début
rubuta "Bonjour"
ƙare
```

**Erreur** : Le programme doit commencer par `fara`

### ✅ Exemple Correct

```hausa
fara
  rubuta "Bonjour"
ƙare
```

---

## 2️⃣ Afficher du Texte avec `rubuta`

Le mot-clé `rubuta` (qui signifie "écrire" en haoussa) permet d'afficher du texte à l'écran.

### Syntaxe de Base

```hausa
rubuta "votre texte ici"
```

### Exemples

**Exemple 1 : Afficher un message simple**
```hausa
fara
  rubuta "Sannu Duniya!"
ƙare
```

**Sortie** :
```
Sannu Duniya!
```

**Exemple 2 : Afficher plusieurs lignes**
```hausa
fara
  rubuta "Première ligne"
  rubuta "Deuxième ligne"
  rubuta "Troisième ligne"
ƙare
```

**Sortie** :
```
Première ligne
Deuxième ligne
Troisième ligne
```

### 🔍 Points Importants

- Les chaînes de caractères sont **toujours** entre guillemets doubles `" "`
- Chaque `rubuta` affiche sur une nouvelle ligne
- Vous pouvez utiliser des caractères Haoussa : `ƙ`, `ɗ`, `ɓ`, `ƴ`

### Exercice Pratique 1

Créez un fichier `salutation.ha` qui affiche :
```
Barka da safiya!
Yaya lafiya?
Sannu!
```

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  rubuta "Barka da safiya!"
  rubuta "Yaya lafiya?"
  rubuta "Sannu!"
ƙare
```
</details>

---

## 3️⃣ Les Variables avec `naɗa`

Une variable est comme une **boîte** qui contient une valeur. Le mot-clé `naɗa` (ou `nada`) signifie "créer/définir".

### Syntaxe

```hausa
naɗa nom_variable = valeur
```

### Exemple Simple

```hausa
fara
  naɗa sunan = "Ahmad"
  rubuta sunan
ƙare
```

**Sortie** :
```
Ahmad
```

### 🔍 Décortiquons

1. `naɗa sunan = "Ahmad"` → Crée une variable nommée `sunan` contenant `"Ahmad"`
2. `rubuta sunan` → Affiche le contenu de la variable `sunan`

### Règles de Nommage des Variables

#### ✅ Noms Valides
```hausa
sunan          # nom
lambar         # nombre
lambar1        # nombre1
total_jimla    # total_somme
ɗan_makaranta  # étudiant
```

#### ❌ Noms Invalides
```hausa
1lambar        # Ne peut pas commencer par un chiffre
suna-na        # Pas de tirets (utilisez _)
fara           # Mot-clé réservé
```

### Variables avec Différents Types

**Texte (Chaînes)**
```hausa
fara
  naɗa sunan = "Khadija"
  naɗa gari = "Kano"
  rubuta sunan
  rubuta gari
ƙare
```

**Nombres**
```hausa
fara
  naɗa lambar = 42
  naɗa shekarun = 25
  rubuta lambar
  rubuta shekarun
ƙare
```

**Booléens (Vrai/Faux)**
```hausa
fara
  naɗa gaskiya_ne = gaskiya  # vrai
  naɗa karya_ne = karya      # faux
  rubuta gaskiya_ne
  rubuta karya_ne
ƙare
```

### Exercice Pratique 2

Créez un fichier `bayani.ha` (informations) qui déclare et affiche :
- Votre nom
- Votre âge
- Votre ville

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa sunan = "Fatima"
  naɗa shekarun = 22
  naɗa gari = "Niamey"
  
  rubuta sunan
  rubuta shekarun
  rubuta gari
ƙare
```
</details>

---

## 4️⃣ Combiner Texte et Variables

Vous pouvez afficher du texte **et** des variables sur la même ligne en utilisant plusieurs `rubuta`.

### Méthode 1 : Affichage Séparé

```hausa
fara
  naɗa sunan = "Musa"
  naɗa shekarun = 30
  
  rubuta "Sunansa:"
  rubuta sunan
  rubuta "Shekarsa:"
  rubuta shekarun
ƙare
```

**Sortie** :
```
Sunassa:
Musa
Shekarsa:
30
```

### Méthode 2 : Concaténation avec `+`

```hausa
fara
  naɗa sunan = "Aisha"
  rubuta "Sannu " + sunan + "!"
ƙare
```

**Sortie** :
```
Sannu Aisha!
```

### 🔍 Important

- L'opérateur `+` fonctionne pour **combiner des chaînes** de texte
- Attention aux espaces : `"Sannu" + sunan` donne `"SannuAisha"` (sans espace)
- Utilisez `"Sannu " + sunan` pour avoir `"Sannu Aisha"` (avec espace)

### Exercice Pratique 3

Créez un programme qui affiche :
```
Sunanta: Zainab
Garinta: Sokoto
```

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa sunan = "Zainab"
  naɗa gari = "Sokoto"
  
  rubuta "Sunanta: " + sunan
  rubuta "Garinta: " + gari
ƙare
```
</details>

---

## 5️⃣ Modifier une Variable

Vous pouvez changer la valeur d'une variable après sa déclaration.

```hausa
fara
  naɗa lambar = 10
  rubuta lambar        # Affiche: 10
  
  lambar = 20          # Modification (sans 'naɗa')
  rubuta lambar        # Affiche: 20
ƙare
```

### 🔍 Règle Importante

- **Première déclaration** : utilisez `naɗa`
- **Modification** : utilisez juste le nom de la variable

### ❌ Erreur Commune

```hausa
fara
  lambar = 10          # ERREUR : variable non déclarée
  rubuta lambar
ƙare
```

**Erreur** : `Babu irin wannan mai canjin 'lambar'` (Variable 'lambar' n'existe pas)

### ✅ Version Correcte

```hausa
fara
  naɗa lambar = 10     # Déclaration avec naɗa
  rubuta lambar
ƙare
```

---

## 6️⃣ Programme Complet : Carte de Visite

Mettons tout ensemble pour créer un programme qui affiche une carte de visite.

**Fichier : `katin_ziyara.ha`**

```hausa
# Programme : Carte de visite
# Auteur : Débutant Dabara
# Date : Aujourd'hui

fara
  # Déclaration des informations
  naɗa sunan = "Abdullahi Ibrahim"
  naɗa aiki = "Mai shirye-shirye" # Programmeur
  naɗa gari = "Katsina"
  naɗa email = "abdullahi@example.com"
  
  # Affichage de la carte
  rubuta "================================"
  rubuta "         KATIN ZIYARA          "
  rubuta "================================"
  rubuta "Suna: " + sunan
  rubuta "Aiki: " + aiki
  rubuta "Gari: " + gari
  rubuta "Email: " + email
  rubuta "================================"
ƙare
```

**Sortie** :
```
================================
         KATIN ZIYARA          
================================
Suna: Abdullahi Ibrahim
Aiki: Mai shirye-shirye
Gari: Katsina
Email: abdullahi@example.com
================================
```

### Exercice Final

Créez votre propre carte de visite avec au moins 5 informations différentes.

---

## 7️⃣ Gestion des Erreurs

Voyons les erreurs courantes et comment les résoudre.

### Erreur 1 : Oubli de Guillemets

❌ **Code Incorrect**
```hausa
fara
  rubuta Sannu
ƙare
```

**Erreur** : `Babu irin wannan mai canjin 'Sannu'`  
**Raison** : Sans guillemets, Dabara pense que `Sannu` est une variable

✅ **Correction**
```hausa
fara
  rubuta "Sannu"
ƙare
```

### Erreur 2 : Variable Non Déclarée

❌ **Code Incorrect**
```hausa
fara
  rubuta sunan
ƙare
```

**Erreur** : `Babu irin wannan mai canjin 'sunan'`  
**Raison** : La variable `sunan` n'a pas été déclarée

✅ **Correction**
```hausa
fara
  naɗa sunan = "Ahmad"
  rubuta sunan
ƙare
```

### Erreur 3 : Oubli de `fara` ou `ƙare`

❌ **Code Incorrect**
```hausa
naɗa sunan = "Test"
rubuta sunan
```

**Erreur** : Le programme ne s'exécutera pas correctement

✅ **Correction**
```hausa
fara
  naɗa sunan = "Test"
  rubuta sunan
ƙare
```

---

## 📝 Résumé du Tutoriel

| Concept | Mot-clé | Exemple |
|---------|---------|---------|
| Début de programme | `fara` | `fara` |
| Fin de programme | `ƙare` ou `kare` | `ƙare` |
| Afficher | `rubuta` | `rubuta "Texte"` |
| Déclarer variable | `naɗa` ou `nada` | `naɗa x = 10` |
| Modifier variable | (aucun) | `x = 20` |
| Commentaire | `#` | `# Ceci est un commentaire` |
| Concaténation | `+` | `"Sannu " + sunan` |

---

## 🎯 Points Clés à Retenir

1. ✅ Chaque programme commence par `fara` et finit par `ƙare`
2. ✅ `rubuta` affiche du texte ou des variables
3. ✅ `naɗa` déclare une nouvelle variable
4. ✅ Les chaînes sont entre guillemets doubles `" "`
5. ✅ Les variables doivent être déclarées avant utilisation
6. ✅ `+` permet de combiner des chaînes

---

## 🚀 Projet Pratique : Présentation Personnelle

Créez un fichier `gabatar_da_kai.ha` qui affiche :
- Votre nom
- Votre âge
- Votre profession ou statut (étudiant, etc.)
- Votre ville
- Un message de bienvenue personnalisé

**Exemple de sortie attendue** :
```
===========================
GABATAR DA KAI
===========================
Sunana: Maryam Hassan
Shekaruna: 19
Aiki: Dalibi
Gari: Zinder
===========================
Sannu! Ina son koyan shirye-shirye!
===========================
```

<details>
<summary>💡 Voir une solution possible</summary>

```hausa
fara
  naɗa sunan = "Maryam Hassan"
  naɗa shekarun = 19
  naɗa aiki = "Dalibi"
  naɗa gari = "Zinder"
  naɗa sako = "Sannu! Ina son koyan shirye-shirye!"
  
  rubuta "==========================="
  rubuta "GABATAR DA KAI"
  rubuta "==========================="
  rubuta "Sunana: " + sunan
  rubuta "Shekaruna: "
  rubuta shekarun
  rubuta "Aiki: " + aiki
  rubuta "Gari: " + gari
  rubuta "==========================="
  rubuta sako
  rubuta "==========================="
ƙare
```
</details>

---

## 🎓 Quiz de Validation

Avant de passer au tutoriel suivant, assurez-vous de pouvoir répondre à ces questions :

1. Quelle est la différence entre `naɗa sunan = "Test"` et `sunan = "Test"` ?
2. Que fait le mot-clé `rubuta` ?
3. Comment afficher "Sannu Ahmad!" si `sunan = "Ahmad"` ?
4. Quels caractères spéciaux haoussa sont supportés ?
5. Pourquoi `fara` et `ƙare` sont-ils obligatoires ?

<details>
<summary>📖 Voir les réponses</summary>

1. `naɗa` déclare une **nouvelle** variable. Sans `naɗa`, on modifie une variable existante.
2. `rubuta` affiche du texte ou la valeur d'une variable à l'écran.
3. `rubuta "Sannu " + sunan + "!"`
4. `ƙ`, `ɗ`, `ɓ`, `ƴ` et leurs équivalents latins.
5. Ils définissent le début et la fin du programme, c'est la structure obligatoire de Dabara.
</details>

---

## ➡️ Prochaine Étape

Félicitations ! 🎉 Vous maîtrisez maintenant les bases de Dabara.

Passez au tutoriel suivant pour approfondir votre connaissance des types de données :

➡️ [**Tutoriel 02 - Les Types de Données**](TUTORIEL_02_TYPES.md)

---

**Bonne programmation en Dabara !** 💻✨
