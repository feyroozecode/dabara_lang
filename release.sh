#!/bin/bash

# Script de création de release locale pour Dabara
# Usage: ./release.sh [version]

set -e

VERSION=${1:-"v0.0.1"}
PROJECT_NAME="dabara"
BUILD_DIR="release-builds"

echo "🚀 Création de la release $VERSION pour $PROJECT_NAME"

# Nettoyer le répertoire de build
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Fonction pour créer une archive
create_archive() {
    local target=$1
    local os_name=$2
    local extension=$3
    
    echo "📦 Construction pour $target..."
    
    # Construire pour la cible spécifiée
    cargo build --release --target "$target"
    
    # Créer le répertoire de sortie
    local output_dir="$BUILD_DIR/$PROJECT_NAME-$target"
    mkdir -p "$output_dir"
    
    # Copier le binaire
    local binary_name="$PROJECT_NAME$extension"
    cp "target/$target/release/$binary_name" "$output_dir/"
    
    # Copier les fichiers de documentation
    cp README.md "$output_dir/"
    cp LICENSE "$output_dir/"
    
    # Créer l'archive
    cd "$BUILD_DIR"
    if [[ "$os_name" == "windows" ]]; then
        zip -r "$PROJECT_NAME-$target.zip" "$PROJECT_NAME-$target"
        echo "✅ Créé: $PROJECT_NAME-$target.zip"
    else
        tar -czf "$PROJECT_NAME-$target.tar.gz" "$PROJECT_NAME-$target"
        echo "✅ Créé: $PROJECT_NAME-$target.tar.gz"
    fi
    cd ..
}

# Installer les cibles cross-compilation si nécessaire
echo "🔧 Installation des cibles de cross-compilation..."

# Cibles supportées
targets=(
    "x86_64-unknown-linux-gnu:linux:"
    "x86_64-pc-windows-msvc:windows:.exe"
    "x86_64-apple-darwin:macos:"
)

# Installer les cibles
for target_info in "${targets[@]}"; do
    IFS=':' read -r target os extension <<< "$target_info"
    rustup target add "$target" || echo "⚠️  Cible $target déjà installée"
done

# Créer les archives pour chaque cible
echo "🏗️  Construction des binaires..."
for target_info in "${targets[@]}"; do
    IFS=':' read -r target os extension <<< "$target_info"
    
    # Vérifier si la cible est supportée sur cette plateforme
    if [[ "$target" == "x86_64-pc-windows-msvc" ]] && [[ "$OSTYPE" != "msys" ]] && [[ "$OSTYPE" != "win32" ]]; then
        echo "⚠️  Ignore la cible Windows sur cette plateforme: $target"
        continue
    fi
    
    create_archive "$target" "$os" "$extension"
done

echo ""
echo "🎉 Release créée avec succès !"
echo "📁 Fichiers générés dans: $BUILD_DIR/"
echo ""
echo "📦 Archives créées:"
ls -la "$BUILD_DIR"/*.{tar.gz,zip} 2>/dev/null || true
echo ""
echo "🚀 Pour créer un tag et pousser sur GitHub:"
echo "   git tag $VERSION"
echo "   git push origin $VERSION"
echo ""
echo "📋 Instructions d'installation pour les utilisateurs:"
echo "   1. Télécharger l'archive correspondant à votre système"
echo "   2. Extraire: tar -xzf $PROJECT_NAME-*.tar.gz (Linux/macOS) ou décompresser le ZIP (Windows)"
echo "   3. Déplacer le binaire dans votre PATH"