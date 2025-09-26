# Guide de Publication sur GitHub

Ce guide explique comment publier votre projet Dabara sur GitHub avec des exécutables automatiquement compilés.

## 🚀 Étapes de Publication

### 1. Créer un Repository GitHub

1. Allez sur [GitHub](https://github.com)
2. Cliquez sur "New repository"
3. Nommez votre repository (ex: `dabara`)
4. Laissez-le public pour que les releases soient accessibles
5. Ne pas initialiser avec README (nous avons déjà le nôtre)

### 2. Pousser le Code

```bash
# Dans le dossier de votre projet
cd /Users/ahmad/dev/rust_projects/dabara

# Initialiser git si pas encore fait
git init

# Ajouter tous les fichiers
git add .

# Premier commit
git commit -m "Initial commit: Dabara Programming Language v0.0.1"

# Ajouter l'origin remote (remplacez USERNAME par votre nom d'utilisateur GitHub)
git remote add origin https://github.com/USERNAME/dabara.git

# Pousser vers GitHub
git push -u origin main
```

### 3. Créer une Release

#### Option A: Via l'interface GitHub (Recommandé)

1. Allez sur votre repository GitHub
2. Cliquez sur "Releases" dans la barre latérale droite
3. Cliquez sur "Create a new release"
4. Dans "Tag version", tapez `v0.0.1`
5. Dans "Release title", tapez `Dabara v0.0.1 - Initial Release`
6. Dans la description, ajoutez:
   ```markdown
   ## 🎉 Première release de Dabara !
   
   Dabara est un langage de programmation utilisant des mots-clés en haoussa.
   
   ### ✨ Fonctionnalités
   - Syntaxe en haoussa
   - Variables et opérations arithmétiques
   - Messages d'erreur localisés
   - Support Unicode complet
   
   ### 📦 Installation
   
   Téléchargez le binaire correspondant à votre système:
   - **Linux**: `dabara-x86_64-unknown-linux-gnu.tar.gz`
   - **Windows**: `dabara-x86_64-pc-windows-msvc.zip`
   - **macOS**: `dabara-x86_64-apple-darwin.tar.gz`
   
   ### 🚀 Utilisation
   ```bash
   dabara programme.ha
   ```
   
   Voir le [README](https://github.com/USERNAME/dabara#readme) pour plus de détails.
   ```
7. Cliquez sur "Publish release"

#### Option B: Via la ligne de commande

```bash
# Créer et pousser un tag
git tag v0.0.1
git push origin v0.0.1
```

Le workflow GitHub Actions se déclenchera automatiquement et créera les binaires.

### 4. Vérification

Après quelques minutes, vérifiez:

1. **Actions Tab**: Les workflows de compilation doivent être verts ✅
2. **Releases**: Votre release doit contenir les binaires pour chaque plateforme
3. **Téléchargement**: Testez le téléchargement d'un binaire

## 🔧 Configuration Avancée

### Variables d'Environnement GitHub

Si vous voulez personnaliser les builds, vous pouvez ajouter des secrets dans votre repository:

1. Allez dans Settings > Secrets and variables > Actions
2. Ajoutez des variables selon vos besoins

### Modifier le Workflow

Le fichier `.github/workflows/release.yml` peut être modifié pour:
- Ajouter d'autres plateformes
- Changer les options de compilation
- Ajouter des tests avant release

### Release Automatique

Pour automatiser les releases à chaque tag:

```bash
# Créer une nouvelle version
git tag v0.0.2
git push origin v0.0.2
```

Le workflow créera automatiquement une release avec les binaires.

## 📋 Checklist de Publication

- [ ] Repository créé sur GitHub
- [ ] Code poussé sur main/master
- [ ] Workflows GitHub Actions fonctionnels
- [ ] Tag créé (v0.0.1)
- [ ] Release publiée
- [ ] Binaires téléchargeables
- [ ] README mis à jour avec les liens de téléchargement
- [ ] Tests des binaires sur différentes plateformes

## 🎯 Prochaines Étapes

1. **Marketing**: Partagez votre projet sur les réseaux sociaux
2. **Documentation**: Ajoutez plus d'exemples
3. **Communauté**: Créez des issues templates
4. **CI/CD**: Améliorez les workflows
5. **Tests**: Ajoutez plus de tests automatisés

## 🆘 Dépannage

### Problème: "No such file or directory" lors du build
- Vérifiez que tous les fichiers source sont présents
- Assurez-vous que Cargo.toml est correct

### Problème: Workflow failed
- Vérifiez les logs dans l'onglet Actions
- Vérifiez que les dépendances sont correctes

### Problème: Binaires manquants dans la release
- Vérifiez que le tag commence par 'v' (v0.0.1)
- Assurez-vous que le workflow release s'est bien exécuté

## 📞 Support

Pour toute question:
1. Créez une issue sur votre repository
2. Consultez la documentation GitHub Actions
3. Vérifiez les logs des workflows