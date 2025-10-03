#!/bin/bash

# Script de construction et déploiement de la documentation Dabara
# Usage: ./build-and-deploy.sh [platform]
# Platforms supportées: vercel, netlify, github

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCS_DIR="$SCRIPT_DIR"
PLATFORM=${1:-"local"}

echo "🚀 Construction et déploiement de la documentation Dabara"
echo "📁 Répertoire: $DOCS_DIR"
echo "🌍 Plateforme: $PLATFORM"

# Vérifier que mdbook est installé
if ! command -v mdbook &> /dev/null; then
    echo "❌ mdBook n'est pas installé."
    echo "📥 Installation automatique..."
    
    # Tentative d'installation via cargo
    if command -v cargo &> /dev/null; then
        echo "🦀 Installation via Cargo..."
        cargo install mdbook
    else
        echo "📦 Téléchargement du binaire mdBook..."
        MDBOOK_VERSION="0.4.36"
        OS=$(uname -s | tr '[:upper:]' '[:lower:]')
        ARCH=$(uname -m)
        
        if [[ "$ARCH" == "x86_64" ]]; then
            ARCH="x86_64"
        elif [[ "$ARCH" == "arm64" || "$ARCH" == "aarch64" ]]; then
            ARCH="aarch64"
        fi
        
        if [[ "$OS" == "darwin" ]]; then
            DOWNLOAD_URL="https://github.com/rust-lang/mdBook/releases/download/v${MDBOOK_VERSION}/mdbook-v${MDBOOK_VERSION}-${ARCH}-apple-darwin.tar.gz"
        elif [[ "$OS" == "linux" ]]; then
            DOWNLOAD_URL="https://github.com/rust-lang/mdBook/releases/download/v${MDBOOK_VERSION}/mdbook-v${MDBOOK_VERSION}-${ARCH}-unknown-linux-gnu.tar.gz"
        else
            echo "❌ Système non supporté: $OS"
            exit 1
        fi
        
        echo "📥 Téléchargement depuis: $DOWNLOAD_URL"
        curl -L "$DOWNLOAD_URL" | tar xz
        chmod +x mdbook
        sudo mv mdbook /usr/local/bin/ 2>/dev/null || mv mdbook ~/.local/bin/ 2>/dev/null || {
            echo "⚠️  Impossible d'installer mdbook globalement. Utilisation locale."
            export PATH="$PWD:$PATH"
        }
    fi
fi

# Vérifier la version de mdbook
echo "📖 Version mdBook: $(mdbook --version)"

# Aller dans le répertoire docs
cd "$DOCS_DIR"

# Nettoyer les builds précédents
if [ -d "book" ]; then
    echo "🧹 Nettoyage du build précédent..."
    rm -rf book
fi

# Construire la documentation
echo "🏗️  Construction de la documentation..."
mdbook build

# Vérifier que la construction a réussi
if [ ! -f "book/index.html" ]; then
    echo "❌ Erreur: La construction a échoué"
    exit 1
fi

echo "✅ Construction réussie!"
echo "📊 Taille du site généré: $(du -sh book | cut -f1)"
echo "📄 Fichiers générés: $(find book -type f | wc -l | tr -d ' ')"

# Déploiement selon la plateforme
case $PLATFORM in
    "local")
        echo "🖥️  Mode local - Lancement du serveur de développement..."
        echo "📖 Ouvrez http://localhost:3000 dans votre navigateur"
        echo "⏹️  Utilisez Ctrl+C pour arrêter le serveur"
        mdbook serve --open
        ;;
        
    "vercel")
        echo "🚀 Déploiement sur Vercel..."
        
        # Vérifier si vercel CLI est installé
        if ! command -v vercel &> /dev/null; then
            echo "❌ Vercel CLI n'est pas installé."
            echo "📥 Installation via npm..."
            npm install -g vercel
        fi
        
        # Créer ou mettre à jour vercel.json dans le répertoire racine
        cd ..
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
  "buildCommand": "cd docs && mdbook build"
}
EOF
        
        echo "📝 Configuration Vercel créée"
        vercel --prod
        echo "✅ Déployé sur Vercel!"
        ;;
        
    "netlify")
        echo "🚀 Déploiement sur Netlify..."
        
        # Vérifier si netlify CLI est installé
        if ! command -v netlify &> /dev/null; then
            echo "❌ Netlify CLI n'est pas installé."
            echo "📥 Installation via npm..."
            npm install -g netlify-cli
        fi
        
        # Créer netlify.toml dans le répertoire racine
        cd ..
        cat > netlify.toml << 'EOF'
[build]
  command = "cd docs && mdbook build"
  publish = "docs/book"

[build.environment]
  MDBOOK_VERSION = "0.4.36"

[[headers]]
  for = "/*"
  [headers.values]
    X-Frame-Options = "DENY"
    X-XSS-Protection = "1; mode=block"
    X-Content-Type-Options = "nosniff"
    Referrer-Policy = "strict-origin-when-cross-origin"
EOF
        
        echo "📝 Configuration Netlify créée"
        netlify deploy --prod --dir=docs/book
        echo "✅ Déployé sur Netlify!"
        ;;
        
    "github")
        echo "🚀 Déploiement sur GitHub Pages..."
        
        # Créer le workflow GitHub Actions si il n'existe pas
        mkdir -p ../.github/workflows
        cat > ../.github/workflows/deploy-docs.yml << 'EOF'
name: Deploy Documentation

on:
  push:
    branches: [ main, master ]
  pull_request:
    branches: [ main, master ]

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
      if: github.ref == 'refs/heads/main' || github.ref == 'refs/heads/master'
      uses: peaceiris/actions-gh-pages@v3
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: ./docs/book
EOF
        
        echo "📝 Workflow GitHub Actions créé"
        echo "🔄 Commitez et poussez pour déclencher le déploiement automatique"
        
        # Ajouter et committer automatiquement
        cd ..
        git add .
        git commit -m "📚 Mise à jour de la documentation et configuration de déploiement" || echo "Rien à commiter"
        
        echo "📤 Poussez avec: git push origin main"
        echo "✅ Le déploiement se fera automatiquement sur GitHub Pages"
        ;;
        
    *)
        echo "❌ Plateforme non supportée: $PLATFORM"
        echo "Plateformes supportées: local, vercel, netlify, github"
        echo ""
        echo "Exemples d'usage:"
        echo "  ./build-and-deploy.sh local     # Serveur local"
        echo "  ./build-and-deploy.sh vercel    # Déploiement Vercel"
        echo "  ./build-and-deploy.sh netlify   # Déploiement Netlify"
        echo "  ./build-and-deploy.sh github    # GitHub Pages"
        exit 1
        ;;
esac

echo ""
echo "🎉 Processus terminé avec succès!"
echo ""

# Afficher les informations utiles
if [ "$PLATFORM" != "local" ]; then
    echo "📋 Prochaines étapes:"
    echo "  1. Vérifiez votre déploiement"
    echo "  2. Configurez un domaine personnalisé si souhaité"
    echo "  3. Activez les analytics (Google Analytics, etc.)"
    echo ""
    echo "🔗 Liens utiles:"
    case $PLATFORM in
        "vercel")
            echo "  • Dashboard Vercel: https://vercel.com/dashboard"
            echo "  • Documentation: https://vercel.com/docs"
            ;;
        "netlify")
            echo "  • Dashboard Netlify: https://app.netlify.com/"
            echo "  • Documentation: https://docs.netlify.com/"
            ;;
        "github")
            echo "  • Actions GitHub: https://github.com/your-username/dabara/actions"
            echo "  • Pages GitHub: https://github.com/your-username/dabara/settings/pages"
            ;;
    esac
fi