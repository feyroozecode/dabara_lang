# 🎉 Dabara: Le Premier Langage de Programmation en Haoussa est Maintenant Open Source!

## Yaren Shirye-Shirye Na Farko Na Hausa - A Buɗe Yanzu! 

![Dabara Logo](https://img.shields.io/badge/Dabara-v0.2.0-brightgreen) ![License](https://img.shields.io/badge/license-MIT-blue) ![Language](https://img.shields.io/badge/language-Hausa-orange)

---

## 🌍 Une Révolution pour l'Afrique Francophone et Haoussa

Aujourd'hui, je suis fier d'annoncer la sortie en **open source** de **Dabara** (_prononcé dah-BAH-rah_), le **premier langage de programmation** utilisant la syntaxe **Haoussa** ! 

Le Haoussa est parlé par plus de **100 millions de personnes** en Afrique de l'Ouest (Nigeria, Niger, Ghana, Cameroun...), pourtant aucun langage de programmation n'existait dans cette langue. **Jusqu'à aujourd'hui.**

### 🎯 La Mission

> **Démocratiser la programmation en supprimant la barrière linguistique.**

Pourquoi un enfant à Kano, Niamey ou Katsina devrait-il d'abord maîtriser l'anglais pour apprendre à coder? Avec Dabara, vous pouvez écrire du code dans **votre langue maternelle**.

---

## ✨ Ce que Vous Pouvez Faire Maintenant

### Exemple 1: Hello World
```hausa
fara
  rubuta "Sannu Duniya!"
ƙare
```

### Exemple 2: Fonction Récursive (Factorielle)
```hausa
fara
  aiki factorial(n) {
    idan n == 0 {
      mayar 1
    } amma {
      mayar n * factorial(n - 1)
    }
  }
  
  rubuta factorial(5)  # Sortie: 120
ƙare
```

### Exemple 3: Scope Local et Variables
```hausa
fara
  naɗa x = 100
  
  aiki canja() {
    naɗa x = 999  # Variable locale
    mayar x
  }
  
  rubuta x              # 100 (global)
  rubuta canja()        # 999 (local)
ƙare
```

---

## 🚀 Version 0.2.0 - Fonctions Complètement Opérationnelles!

Après des mois de développement, nous avons atteint une **étape majeure**:

### ✅ Fonctionnalités Implémentées

| Fonctionnalité | Statut | Description |
|---------------|--------|-------------|
| **Variables** | ✅ | `naɗa nom = valeur` |
| **Affichage** | ✅ | `rubuta "texte"` |
| **Arithmétique** | ✅ | `+`, `-`, `*`, `/` |
| **Conditions** | ✅ | `idan ... amma ...` |
| **Fonctions** | ✅ | `aiki nom(params) { ... }` |
| **Récursion** | ✅ | Fonctions s'appelant elles-mêmes |
| **Return** | ✅ | `mayar valeur` |
| **Scope Local** | ✅ | Isolation des variables |
| **Listes** | ✅ | `[1, 2, 3]` |
| **Booleans** | ✅ | `gaskiya`, `karya` |
| **Comparaisons** | ✅ | `==`, `!=`, `<`, `>`, `<=`, `>=` |

### 🎊 Résultats des Tests

```bash
$ cargo test
running 38 tests
test result: ok. 38 passed; 0 failed
```

**100% des tests passent!** Dabara est stable et prêt pour la production.

---

## 🛠️ Technologies Utilisées

- **Langage**: Rust 🦀 (performance + sécurité)
- **Architecture**: Lexer → Parser → AST → Interpreter
- **Support Unicode**: Complet pour les caractères haoussa (ƙ, ɗ, ɓ, ʔ)
- **Tests**: 38 tests unitaires et d'intégration

---

## 📊 Performance Benchmark

```bash
$ time dabara examples/benchmark.ha

=== Dabara v0.2.0 Performance Benchmark ===

Test 3: Recursive Factorial
factorial(5) = 120
factorial(10) = 3628800

Test 4: Nested Function Calls
octuple(5) = 40

=== Benchmark Complete! ===

Temps total: 0.16s
```

**Résultats**: Execution rapide même pour des calculs récursifs complexes!

---

## 🌟 Pourquoi C'est Important

### 1. **Inclusion Linguistique**
- Permet aux locuteurs haoussa d'apprendre la programmation **sans barrière linguistique**
- Valorise la culture et la langue africaine dans la tech

### 2. **Éducation Accessible**
- Les écoles au Nigeria et Niger peuvent enseigner la programmation en haoussa
- Les enfants comprennent mieux les concepts dans leur langue maternelle

### 3. **Représentation**
- Montre que les langues africaines ont leur place dans la technologie moderne
- Inspire d'autres projets similaires pour d'autres langues

### 4. **Open Source**
- Code entièrement ouvert et modifiable
- Communauté internationale peut contribuer
- Licence MIT permissive

---

## 🤝 Comment Contribuer

Nous recherchons activement des contributeurs! Voici comment vous pouvez aider:

### 🔧 Développeurs
- **Implémenter des nouvelles fonctionnalités** (loops, modules, file I/O)
- **Ajouter des tests**
- **Améliorer la performance**
- **Créer des outils** (VS Code extension, REPL, debugger)

### 📝 Non-Développeurs
- **Traduire la documentation** en haoussa, français, anglais
- **Créer des tutoriels** et exemples
- **Tester le langage** et signaler des bugs
- **Partager sur les réseaux sociaux**
- **Écrire des articles de blog**

### 📚 Éducateurs
- **Créer des cours** en utilisant Dabara
- **Tester dans des écoles** au Nigeria/Niger
- **Donner du feedback** sur l'usage pédagogique

---

## 🎓 Roadmap - Prochaines Étapes

### Version 0.3.0 (Q1 2026)
- [ ] Loops (`maimaita` - while, `kullum` - for)
- [ ] Boolean operators (`da` - and, `ko` - or, `ba` - not)
- [ ] Standard library (string/list methods)
- [ ] Comments support
- [ ] List indexing (`jeri[0]`)

### Version 0.4.0 (Q2 2026)
- [ ] File I/O operations
- [ ] Module system
- [ ] Exception handling
- [ ] Better error messages with line numbers

### Version 1.0.0 (Q4 2026)
- [ ] REPL interactive mode
- [ ] Package manager
- [ ] Stdlib complète
- [ ] Production-ready

---

## 📦 Installation

### Option 1: Télécharger le Binaire
```bash
# Voir les releases sur GitHub
https://github.com/feyroozecode/dabara_lang/releases
```

### Option 2: Compiler depuis les Sources
```bash
# Installer Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Cloner et compiler
git clone https://github.com/feyroozecode/dabara_lang.git
cd dabara_lang
cargo build --release
cargo install --path .

# Tester
dabara examples/benchmark.ha
```

---

## 🔗 Liens Utiles

- **GitHub**: [github.com/feyroozecode/dabara_lang](https://github.com/feyroozecode/dabara_lang)
- **Documentation**: [À venir]
- **VS Code Extension**: [github.com/feyroozecode/dabara-vscode-extension](https://github.com/feyroozecode/dabara-vscode-extension)
- **Discord Community**: [À venir]
- **Twitter**: [@feyroozecode](https://twitter.com/feyroozecode)

---

## 💬 Traduction en Haoussa

### 🎉 Dabara: Yaren Shirye-Shirye Na Farko Na Hausa!

**A yau**, ina farin ciki in sanar da cewa **Dabara** yanzu yana **buɗe** ga kowa! Wannan shi ne **yaren shirye-shirye na farko** da aka rubuta da **Hausanci**.

#### Me kuke iya yi a yanzu:

```hausa
# Misali 1: Gaisuwa ga Duniya
fara
  rubuta "Sannu Duniya!"
ƙare

# Misali 2: Aikin Ƙidaya (Factorial)
fara
  aiki factorial(n) {
    idan n == 0 {
      mayar 1
    } amma {
      mayar n * factorial(n - 1)
    }
  }
  rubuta factorial(5)
ƙare
```

#### 🎯 Manufarmu

> **A sauƙaƙa koyan shirye-shirye ga duk mutane ta hanyar amfani da yarensu na gida.**

#### 🤝 Ku taimaka mu!

Muna neman **gudummawar ku**:
- **Masu Shirye-Shirye**: Ku ƙara sabbin ayyuka
- **Malamai**: Ku gwada a makarantu
- **Kowa**: Ku raba wannan labari!

---

## 🎬 Conclusion

**Dabara** n'est pas juste un langage de programmation. C'est un **mouvement** pour:
- ✊ L'inclusion linguistique dans la tech
- 🌍 La représentation africaine dans la programmation
- 📚 L'éducation accessible pour tous
- 🚀 L'innovation africaine

### 💪 Rejoignez le mouvement!

Si vous croyez qu'**apprendre à coder ne devrait pas nécessiter de maîtriser l'anglais**, alors Dabara est fait pour vous.

**Star le repo**, **partagez cet article**, et **contribuez** pour aider des millions de jeunes Africains à découvrir la programmation dans leur langue!

---

## 🙏 Remerciements

Merci à:
- La communauté Rust pour l'excellent tooling
- Tous ceux qui ont testé les versions précoces
- Ma famille et amis pour le support
- Et **VOUS** pour avoir lu jusqu'ici! 🎉

---

### 📧 Contact

- **Email**: feyroozecode@gmail.com
- **GitHub**: [@feyroozecode](https://github.com/feyroozecode)
- **Twitter**: [@feyroozecode](https://twitter.com/feyroozecode)

---

**Sannu da zuwa!** 🎊 **Bienvenue dans l'ère de la programmation multilingue!**

#Dabara #Hausa #Programming #OpenSource #Africa #Tech #Diversity #Inclusion #Nigeria #Niger #Education
