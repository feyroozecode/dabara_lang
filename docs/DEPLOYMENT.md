# Guide de déploiement de la documentation Dabara

Ce guide vous explique comment déployer la documentation Dabara sur différentes plateformes d'hébergement statique.

## 📋 Prérequis

- [mdBook](https://rust-lang.github.io/mdBook/) installé
- Git configuré
- Compte sur la plateforme de déploiement choisie

### Installation de mdBook

```bash
# Via Cargo (Rust)
cargo install mdbook

# Via Homebrew (macOS)
brew install mdbook

# Via apt (Ubuntu/Debian)
sudo apt install mdbook

# Ou téléchargement direct depuis GitHub
# https://github.com/rust-lang/mdBook/releases
```

## 🏗️ Construction locale

### 1. Construire la documentation

```bash
cd docs
mdbook build
```

Cette commande génère le site statique dans le dossier `book/`.

### 2. Prévisualisation locale

```bash
cd docs
mdbook serve
```

Ouvrez http://localhost:3000 pour prévisualiser.

### 3. Vérification du contenu

```bash
# Structure générée
docs/book/
├── index.html
├── css/
├── js/
├── FontAwesome/
├── fonts/
├── part1/
├── part2/
├── part3/
├── part4/
├── annexes/
└── ...
```

## 🚀 Déploiement sur Vercel (Recommandé)

### Option 1: Interface Web Vercel

1. **Connectez votre repository**
   - Allez sur [vercel.com](https://vercel.com)
   - Cliquez "New Project"
   - Importez votre repository GitHub

2. **Configuration du build**
   ```json
   {
     "buildCommand": "cd docs && mdbook build",
     "outputDirectory": "docs/book",
     "installCommand": "curl -L https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz | tar xz && chmod +x mdbook"
   }
   ```

3. **Variables d'environnement** (optionnel)
   - `MDBOOK_VERSION`: Version spécifique de mdBook

4. **Déploiement**
   - Cliquez "Deploy"
   - Votre site sera disponible sur une URL `.vercel.app`

### Option 2: CLI Vercel

```bash
# Installation de Vercel CLI
npm i -g vercel

# Dans le dossier racine du projet
cd /path/to/dabara

# Configuration initiale
vercel

# Créer vercel.json
cat > vercel.json << 'EOF'
{
  "builds": [
    {
      "src": "docs/**",
      "use": "@vercel/static-build",
      "config": {
        "distDir": "docs/book"
      }
    }
  ],
  "routes": [
    {
      "src": "/(.*)",
      "dest": "/docs/book/$1"
    }
  ],
  "buildCommand": "cd docs && curl -L https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz | tar xz && chmod +x mdbook && ./mdbook build"
}
EOF

# Déploiement
vercel --prod
```

## 📄 Déploiement sur Netlify

### Option 1: Interface Web Netlify

1. **Connectez votre repository**
   - Allez sur [netlify.com](https://netlify.com)
   - "New site from Git"
   - Choisissez votre repository

2. **Configuration de build**
   - **Build command**: `cd docs && curl -L https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz | tar xz && chmod +x mdbook && ./mdbook build`
   - **Publish directory**: `docs/book`
   - **Base directory**: (laisser vide)

3. **Variables d'environnement**
   - Aucune nécessaire pour le build de base

### Option 2: netlify.toml

Créez `netlify.toml` à la racine :

```toml
[build]
  command = "cd docs && curl -L https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz | tar xz && chmod +x mdbook && ./mdbook build"
  publish = "docs/book"

[build.environment]
  MDBOOK_VERSION = "0.4.36"

[[headers]]
  for = "/*"
  [headers.values]
    X-Frame-Options = "DENY"
    X-XSS-Protection = "1; mode=block"
    X-Content-Type-Options = "nosniff"

[[redirects]]
  from = "/docs/*"
  to = "/:splat"
  status = 301
```

## 🐙 Déploiement sur GitHub Pages

### 1. Créer le workflow GitHub Actions

Créez `.github/workflows/deploy-docs.yml` :

```yaml
name: Deploy Documentation

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  deploy:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup mdBook
      uses: peaceiris/actions-mdbook@v1
      with:
        mdbook-version: '0.4.36'
    
    - name: Build documentation
      run: |
        cd docs
        mdbook build
    
    - name: Deploy to GitHub Pages
      if: github.ref == 'refs/heads/main'
      uses: peaceiris/actions-gh-pages@v3
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: ./docs/book
        cname: dabara-docs.your-domain.com  # Optionnel
```

### 2. Configuration GitHub Pages

1. Allez dans Settings > Pages de votre repository
2. Source: "Deploy from a branch"
3. Branch: `gh-pages` / `/ (root)`
4. Votre documentation sera disponible sur `username.github.io/repository-name`

## 🌐 Déploiement sur CloudFlare Pages

### 1. Interface Web CloudFlare

1. **Connectez votre repository**
   - Dashboard CloudFlare > Pages > "Create a project"
   - "Connect to Git" > Choisissez votre repository

2. **Configuration de build**
   - **Framework preset**: None
   - **Build command**: `cd docs && curl -L https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz | tar xz && chmod +x mdbook && ./mdbook build`
   - **Build output directory**: `docs/book`
   - **Root directory**: (laisser vide)

### 2. Variables d'environnement

```
MDBOOK_VERSION=0.4.36
```

## 🔧 Script de déploiement automatisé

Créez `deploy.sh` à la racine :

```bash
#!/bin/bash

set -e

echo "🚀 Déploiement de la documentation Dabara"

# Configuration
PLATFORM=${1:-"vercel"}  # vercel, netlify, github, cloudflare
ENVIRONMENT=${2:-"production"}

echo "📋 Plateforme: $PLATFORM"
echo "🌍 Environnement: $ENVIRONMENT"

# Construction de la documentation
echo "🏗️ Construction de la documentation..."
cd docs
mdbook build
cd ..

# Validation du contenu
echo "✅ Validation du contenu..."
if [ ! -f "docs/book/index.html" ]; then
    echo "❌ Erreur: docs/book/index.html non trouvé"
    exit 1
fi

echo "📊 Taille du site généré:"
du -sh docs/book/

# Déploiement selon la plateforme
case $PLATFORM in
    "vercel")
        echo "🚀 Déploiement sur Vercel..."
        if [ "$ENVIRONMENT" = "production" ]; then
            vercel --prod
        else
            vercel
        fi
        ;;
    "netlify")
        echo "🚀 Déploiement sur Netlify..."
        if command -v netlify-cli >/dev/null 2>&1; then
            if [ "$ENVIRONMENT" = "production" ]; then
                netlify deploy --prod --dir=docs/book
            else
                netlify deploy --dir=docs/book
            fi
        else
            echo "❌ netlify-cli non installé. Installez avec: npm install -g netlify-cli"
            exit 1
        fi
        ;;
    "github")
        echo "🚀 Déploiement sur GitHub Pages..."
        # Le déploiement se fait via GitHub Actions
        git add .
        git commit -m "📚 Mise à jour de la documentation" || echo "Rien à commiter"
        git push origin main
        echo "✅ Poussé vers GitHub. Le déploiement se fera automatiquement."
        ;;
    "cloudflare")
        echo "🚀 Déploiement sur CloudFlare Pages..."
        if command -v wrangler >/dev/null 2>&1; then
            wrangler pages publish docs/book --project-name dabara-docs
        else
            echo "❌ wrangler non installé. Installez avec: npm install -g wrangler"
            exit 1
        fi
        ;;
    *)
        echo "❌ Plateforme non supportée: $PLATFORM"
        echo "Plateformes supportées: vercel, netlify, github, cloudflare"
        exit 1
        ;;
esac

echo "✅ Déploiement terminé!"
```

Rendez le script exécutable :

```bash
chmod +x deploy.sh
```

### Utilisation du script

```bash
# Déploiement sur Vercel (par défaut)
./deploy.sh

# Déploiement sur Netlify
./deploy.sh netlify

# Déploiement sur GitHub Pages
./deploy.sh github

# Déploiement en mode preview
./deploy.sh vercel preview
```

## 📱 Configuration responsive et PWA

### 1. Mettre à jour book.toml

```toml
[book]
authors = ["Ahmad"]
language = "fr"
src = "src"
title = "Documentation Dabara"
description = "Documentation technique complète du langage de programmation Dabara"

[build]
build-dir = "book"
create-missing = true

[output.html]
default-theme = "light"
preferred-dark-theme = "navy"
smart-punctuation = true
mathjax-support = false
copy-fonts = true
no-section-label = false
site-url = "https://dabara-docs.vercel.app/"
git-repository-url = "https://github.com/votre-username/dabara"
git-repository-icon = "fa-github"
edit-url-template = "https://github.com/votre-username/dabara/edit/main/docs/src/{path}"

[output.html.search]
enable = true
limit-results = 30
teaser-word-count = 30
use-boolean-and = true
boost-title = 2
boost-hierarchy = 1
boost-paragraph = 1
expand = true
heading-split-level = 3
copy-js = true

[output.html.print]
enable = true

[output.html.fold]
enable = true
level = 1

[output.html.playground]
runnable = false

[preprocessor.links]
```

### 2. Ajout de PWA (optionnel)

Créez `docs/src/manifest.json` :

```json
{
  "name": "Documentation Dabara",
  "short_name": "Dabara Docs",
  "description": "Documentation du langage de programmation Dabara",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#ffffff",
  "theme_color": "#2196f3",
  "icons": [
    {
      "src": "icon-192.png",
      "sizes": "192x192",
      "type": "image/png"
    },
    {
      "src": "icon-512.png",
      "sizes": "512x512",
      "type": "image/png"
    }
  ]
}
```

## 🔍 Optimisations SEO

### 1. Meta tags personnalisés

Créez `docs/theme/head.hbs` :

```html
<!-- SEO Meta Tags -->
<meta name="description" content="Documentation officielle du langage de programmation Dabara - Le premier langage de programmation en haoussa">
<meta name="keywords" content="dabara, haoussa, programmation, afrique, documentation, tutoriel">
<meta name="author" content="Ahmad">
<meta name="robots" content="index, follow">

<!-- Open Graph Meta Tags -->
<meta property="og:title" content="{{title}} - Documentation Dabara">
<meta property="og:description" content="Documentation officielle du langage de programmation Dabara">
<meta property="og:type" content="website">
<meta property="og:url" content="https://dabara-docs.vercel.app">
<meta property="og:image" content="https://dabara-docs.vercel.app/images/og-image.png">

<!-- Twitter Meta Tags -->
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:title" content="{{title}} - Documentation Dabara">
<meta name="twitter:description" content="Documentation officielle du langage de programmation Dabara">
<meta name="twitter:image" content="https://dabara-docs.vercel.app/images/og-image.png">

<!-- Canonical URL -->
<link rel="canonical" href="https://dabara-docs.vercel.app/{{path}}">

<!-- Additional Meta -->
<meta name="language" content="French">
<meta name="revisit-after" content="7 days">
```

### 2. Sitemap automatique

Créez `docs/src/sitemap.xml` :

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://dabara-docs.vercel.app/</loc>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>https://dabara-docs.vercel.app/introduction.html</loc>
    <changefreq>monthly</changefreq>
    <priority>0.9</priority>
  </url>
  <url>
    <loc>https://dabara-docs.vercel.app/installation.html</loc>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
  <!-- Ajoutez d'autres URLs importantes -->
</urlset>
```

## 📊 Analytics et monitoring

### 1. Google Analytics

Ajoutez dans `docs/theme/head.hbs` :

```html
<!-- Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=GA_MEASUREMENT_ID"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'GA_MEASUREMENT_ID');
</script>
```

### 2. Monitoring des performances

Utilisez [Lighthouse](https://lighthouse-ci.com/) pour vérifier :
- Performance
- Accessibilité  
- SEO
- PWA (si applicable)

## 🚨 Résolution de problèmes

### Erreurs courantes

1. **mdBook non trouvé lors du build**
   ```bash
   # Solution: Installer mdBook globalement
   cargo install mdbook
   ```

2. **Permission denied sur le script de déploiement**
   ```bash
   chmod +x deploy.sh
   ```

3. **Build qui échoue sur la plateforme**
   - Vérifiez la commande de build
   - Vérifiez le répertoire de sortie
   - Consultez les logs de déploiement

4. **CSS/JS non chargé**
   - Vérifiez le `site-url` dans `book.toml`
   - Assurez-vous que les assets sont dans le bon répertoire

### Support et maintenance

- **Mise à jour de mdBook** : Modifiez la version dans les configs
- **Surveillance des déploiements** : Configurez des notifications
- **Backup régulier** : Sauvegardez votre repository

## 🎯 Recommandations finales

### Pour la production

1. **Utilisez Vercel** - Plus simple et plus rapide
2. **Configurez un domaine personnalisé** - `docs.dabara.dev`
3. **Activez HTTPS** - Automatique sur la plupart des plateformes
4. **Configurez un CDN** - Pour des performances globales
5. **Surveillez les performances** - Lighthouse CI

### URLs suggérées

- **Vercel**: `https://dabara-docs.vercel.app`
- **Netlify**: `https://dabara-docs.netlify.app`  
- **GitHub Pages**: `https://username.github.io/dabara`
- **CloudFlare**: `https://dabara-docs.pages.dev`

### Commandes de déploiement rapides

```bash
# Vercel
vercel --prod

# Netlify
netlify deploy --prod --dir=docs/book

# GitHub Pages (automatique via Actions)
git push origin main

# CloudFlare Pages
wrangler pages publish docs/book
```

---

**La documentation Dabara est maintenant prête pour le monde ! 🌍🚀**