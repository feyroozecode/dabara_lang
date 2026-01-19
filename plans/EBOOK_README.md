# Dabara Ebook - Guide Rapide de Programmation en Hausa

## 📘 Description

Ce repository contient les ressources pour créer un ebook de 45 pages sur le langage de programmation Dabara, conçu pour enseigner la programmation en haoussa et en français.

## 📁 Contenu

### Documents Principaux

- **`dabara_ebook.html`** - Version HTML interactive (recommandée)
- **`dabara_ebook.tex`** - Version LaTeX source (pour PDF professionnel)
- **`ebook_plan_45_pages.md`** - Plan détaillé de l'ebook
- **`build_ebook.sh`** - Script de compilation LaTeX

### Fichiers de Support

- **`README.md`** - Ce fichier
- **`LICENSE`** - Licence du projet

## 🚀 Utilisation

### Option 1 : HTML (Recommandée - Pas d'installation requise)

1. Ouvrez `dabara_ebook.html` dans votre navigateur
2. Utilisez les boutons "Voir la solution" pour les exercices
3. Imprimez en PDF depuis votre navigateur :
   - Chrome/Firefox : Ctrl+P → Enregistrer en PDF
   - Safari : Cmd+P → PDF → Enregistrer

### Option 2 : PDF via LaTeX (Qualité professionnelle)

**Prérequis :**
```bash
# Sur macOS
brew install --cask mactex

# Sur Ubuntu/Debian
sudo apt install texlive-latex-recommended texlive-fonts-recommended

# Sur Windows
# Télécharger et installer MiKTeX : https://miktex.org/download
```

**Compilation :**
```bash
chmod +x build_ebook.sh
./build_ebook.sh
```

Le PDF sera généré sous le nom `dabara_ebook.pdf`.

## 📖 Structure de l'Ebook (45 pages)

### Pages 1-2 : Introduction
- Présentation de Dabara
- Objectifs du guide
- Public cible

### Pages 3-5 : Installation (3 pages)
- Méthodes d'installation
- Vérification du fonctionnement
- Structure de base

### Pages 6-12 : Fondamentaux (7 pages)
- Variables et types de données
- Affichage avec `rubuta`
- Concaténation de textes
- Exercices pratiques

### Pages 13-18 : Mathématiques (6 pages)
- Opérations arithmétiques
- Priorité des opérations
- Applications pratiques
- Gestion des erreurs

### Pages 19-25 : Conditions (7 pages)
- Structures conditionnelles
- Opérateurs de comparaison
- Conditions imbriquées
- Projets : calculateur d'IMC, quiz

### Pages 26-32 : Fonctions (7 pages)
- Définition et appel
- Paramètres et retours
- Portée des variables
- Fonctions récursives

### Pages 33-37 : Interaction (5 pages)
- Lecture d'entrées utilisateur
- Validation des données
- Menus interactifs
- Calculatrice interactive

### Pages 38-41 : Collections (4 pages)
- Création de listes
- Boucles `don`
- Opérations sur les listes
- Gestionnaire de contacts

### Pages 42-43 : Projets (2 pages)
- Gestionnaire de budget
- Quiz multilingue

### Pages 44-45 : Références (2 pages)
- Tableau des mots-clés
- Commandes utiles
- Ressources supplémentaires

## 🎯 Caractéristiques

### Pour les Lecteurs
- ✅ Contenu condensé et essentiel (45 pages seulement)
- ✅ Exemples pratiques en haoussa
- ✅ Exercices avec solutions
- ✅ Code coloré et formaté
- ✅ Accessible aux débutants

### Pour les Formateurs
- ✅ Structure pédagogique progressive
- ✅ Projets concrets et applicables
- ✅ Références rapides incluses
- ✅ Format HTML + PDF disponibles

## 🛠️ Personnalisation

### Modifier le contenu HTML
1. Éditez `dabara_ebook.html`
2. Sauvegardez et ouvrez dans le navigateur

### Modifier le contenu LaTeX
1. Éditez `dabara_ebook.tex`
2. Compilez avec `./build_ebook.sh`

### Adapter pour d'autres langues
Remplacez les exemples haoussa par votre langue cible tout en conservant la structure.

## 🤝 Contribution

Suggestions d'amélioration :
- Ajout d'exercices supplémentaires
- Traduction dans d'autres langues locales
- Amélioration des exemples
- Correction d'erreurs

## 📄 Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.

## 🙏 Remerciements

Merci à la communauté Dabara pour leur travail innovant sur ce langage de programmation multilingue.

---

**Barka da zuwa!** (Bienvenue!)  
**Mu fara tafiya!** (Commençons le voyage!)

*Dabara Programming Language - Yaren shirye-shirye na Hausa* 🇳🇬 🇳🇪 💻