# Tutoriel Dabara - Introduction au Langage

## 🌍 Bienvenue dans Dabara

**Dabara** (prononcé *dah-BAH-rah*) est le premier langage de programmation utilisant la syntaxe **Haoussa**. Ce tutoriel vous guidera pas à pas dans l'apprentissage de ce langage révolutionnaire.

## 📖 Pour Qui est ce Tutoriel?

Ce tutoriel est conçu pour :
- ✅ **Débutants complets** : Aucune expérience en programmation nécessaire
- ✅ **Développeurs juniors** : Comprendre les mécanismes internes d'un langage
- ✅ **Locuteurs Hausa** : Apprendre la programmation dans votre langue maternelle
- ✅ **Curieux** : Découvrir comment fonctionne un langage de programmation

## 🎯 Objectifs du Tutoriel

À la fin de ce tutoriel, vous serez capable de :
1. **Comprendre** la syntaxe Dabara et les mots-clés Haoussa
2. **Écrire** vos propres programmes en Dabara
3. **Résoudre** des problèmes avec les structures de contrôle
4. **Créer** des fonctions et organiser votre code
5. **Comprendre** comment le langage fonctionne en interne

## 📚 Structure du Tutoriel

Le tutoriel est divisé en plusieurs sections progressives :

### **Niveau 1 : Les Fondamentaux** (Débutants)
- [**Tutoriel 01**](TUTORIEL_01_BASES.md) - Les Bases : Variables et Affichage
- [**Tutoriel 02**](TUTORIEL_02_TYPES.md) - Les Types de Données
- [**Tutoriel 03**](TUTORIEL_03_OPERATIONS.md) - Opérations Arithmétiques

### **Niveau 2 : La Logique** (Intermédiaire)
- [**Tutoriel 04**](TUTORIEL_04_CONDITIONS.md) - Conditions et Comparaisons
- [**Tutoriel 05**](TUTORIEL_05_LISTES.md) - Listes et Collections
- [**Tutoriel 06**](TUTORIEL_06_ENTREE.md) - Interaction avec l'Utilisateur

### **Niveau 3 : L'Organisation** (Avancé)
- [**Tutoriel 07**](TUTORIEL_07_FONCTIONS.md) - Fonctions et Réutilisation
- [**Tutoriel 08**](TUTORIEL_08_RECURSION.md) - Récursivité et Portée
- [**Tutoriel 09**](TUTORIEL_09_PROJET.md) - Projet Complet

### **Niveau 4 : Les Mécanismes** (Experts)
- [**Tutoriel 10**](TUTORIEL_10_INTERNALS.md) - Comment Dabara Fonctionne
- [**Tutoriel 11**](TUTORIEL_11_LEXER.md) - Le Lexer : Tokenisation
- [**Tutoriel 12**](TUTORIEL_12_PARSER.md) - Le Parser : Analyse Syntaxique
- [**Tutoriel 13**](TUTORIEL_13_INTERPRETER.md) - L'Interpréteur : Exécution

## 🚀 Comment Utiliser ce Tutoriel

### 1. **Lisez dans l'ordre**
Chaque tutoriel s'appuie sur les précédents. Suivez l'ordre numérique pour une progression optimale.

### 2. **Pratiquez les exemples**
Chaque section contient des exemples de code. **Tapez-les vous-même** au lieu de copier-coller pour mieux apprendre.

### 3. **Faites les exercices**
Des exercices sont proposés à la fin de chaque tutoriel. Essayez de les résoudre avant de consulter les solutions.

### 4. **Expérimentez**
N'hésitez pas à modifier les exemples et à créer vos propres variations.

## ⚙️ Prérequis : Installation

Avant de commencer, vous devez installer Dabara sur votre ordinateur.

### Option A : Télécharger le Binaire (Recommandé pour Débutants)

1. Allez sur la page [Releases](https://github.com/feyroozecode/dabara/releases)
2. Téléchargez le fichier correspondant à votre système :
   - **Windows** : `dabara-x86_64-pc-windows-msvc.zip`
   - **macOS (Intel)** : `dabara-x86_64-apple-darwin.tar.gz`
   - **macOS (Apple Silicon)** : `dabara-aarch64-apple-darwin.tar.gz`
   - **Linux** : `dabara-x86_64-unknown-linux-gnu.tar.gz`

3. Extrayez l'archive et déplacez le fichier `dabara` dans un dossier de votre PATH

### Option B : Compiler depuis les Sources (Pour Développeurs)

```bash
# Installer Rust (si nécessaire)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Cloner le repository
git clone https://github.com/feyroozecode/dabara.git
cd dabara

# Compiler
cargo build --release

# Installer
cargo install --path .
```

### Vérifier l'Installation

Ouvrez un terminal et tapez :
```bash
dabara --version
```

Vous devriez voir :
```
dabara 0.2.0
```

## 📝 Votre Premier Programme

Créez un fichier nommé `sannu.ha` avec ce contenu :

```hausa
fara
  rubuta "Sannu Duniya!"
ƙare
```

Exécutez-le :
```bash
dabara sannu.ha
```

Résultat :
```
Sannu Duniya!
```

**🎉 Félicitations !** Vous venez d'exécuter votre premier programme Dabara !

## 🔍 Comprendre ce Premier Programme

Décortiquons ligne par ligne :

```hausa
fara                        # Début du programme (keyword "commencer")
  rubuta "Sannu Duniya!"    # Afficher "Bonjour le Monde!" (keyword "écrire")
ƙare                        # Fin du programme (keyword "terminer")
```

### Mots-clés Essentiels

| Hausa | Signification | Français | Utilisation |
|-------|---------------|----------|-------------|
| `fara` | commencer | begin | Début de programme |
| `ƙare` ou `kare` | terminer | end | Fin de programme |
| `rubuta` | écrire | print | Afficher du texte |

> 💡 **Note** : Vous pouvez utiliser `ƙare` (avec le caractère spécial) ou `kare` (version latine). Les deux sont acceptés !

## 🎨 Les Commentaires

En Dabara, les commentaires commencent par `#` :

```hausa
# Ceci est un commentaire - il est ignoré par l'interpréteur
fara
  # Afficher un message de bienvenue
  rubuta "Sannu!"
ƙare
```

Les commentaires sont essentiels pour :
- Expliquer ce que fait votre code
- Désactiver temporairement du code
- Documenter votre logique

## 📋 Conventions de Nommage

### Extensions de Fichiers
Tous les fichiers Dabara utilisent l'extension `.ha` (pour **Ha**usa) :
- ✅ `programme.ha`
- ✅ `calculatrice.ha`
- ❌ `programme.txt`
- ❌ `code.dab`

### Noms de Variables
Les noms de variables peuvent contenir :
- Lettres latines : `a-z`, `A-Z`
- Lettres haoussa : `ƙ`, `ɗ`, `ɓ`, `ƴ`
- Chiffres : `0-9` (mais pas en première position)
- Underscore : `_`

Exemples valides :
```hausa
sunan           # "nom"
lambar1         # "nombre1"
ɗan_makaranta   # "étudiant"
total_jimla     # "total_somme"
```

## 🎓 Conseils pour Bien Apprendre

### ✅ À Faire
- **Tapez** les exemples au lieu de copier-coller
- **Expérimentez** : modifiez les exemples pour voir ce qui se passe
- **Faites des erreurs** : les erreurs sont vos meilleures enseignantes
- **Prenez des notes** : notez ce qui vous semble important
- **Pratiquez régulièrement** : 30 minutes par jour vaut mieux que 3 heures par semaine

### ❌ À Éviter
- **Ne sautez pas** les sections fondamentales
- **N'abandonnez pas** après une première erreur
- **Ne comparez pas** votre progression à celle des autres
- **Ne copiez pas** sans comprendre

## 🆘 Obtenir de l'Aide

Si vous rencontrez des difficultés :

1. **Relisez la section** concernée attentivement
2. **Vérifiez les erreurs** : les messages d'erreur donnent souvent des indices
3. **Consultez les exemples** dans le dossier `examples/`
4. **Posez des questions** sur le repository GitHub
5. **Participez** à la communauté Dabara

## 📊 Système de Notation

Dans ce tutoriel, nous utilisons des symboles pour indiquer la difficulté :

- 🟢 **FACILE** : Concepts de base, pour débutants
- 🟡 **MOYEN** : Nécessite de comprendre les bases
- 🔴 **AVANCÉ** : Pour développeurs expérimentés
- ⚫ **EXPERT** : Mécanismes internes du langage

## 🗺️ Prochaine Étape

Maintenant que vous avez compris l'introduction, passez au premier tutoriel :

➡️ [**Tutoriel 01 - Les Bases : Variables et Affichage**](TUTORIEL_01_BASES.md)

---

## 📖 Glossaire Rapide

| Terme | Définition |
|-------|------------|
| **Token** | Plus petite unité significative du code (mot-clé, nombre, symbole) |
| **Variable** | Espace mémoire nommé pour stocker une valeur |
| **Type** | Catégorie de données (nombre, texte, booléen) |
| **Fonction** | Bloc de code réutilisable |
| **Condition** | Test qui détermine quel code exécuter |

---

**🎉 Bonne chance dans votre apprentissage de Dabara !**

*Sannu da zuwa cikin yaren Dabara!* (Bienvenue dans le langage Dabara!)
