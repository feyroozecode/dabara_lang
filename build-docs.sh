#!/bin/bash

# Script de génération de documentation locale
# Usage: ./build-docs.sh [option]

set -e

echo "🚀 Génération de la documentation Dabara"

# Fonction d'aide
show_help() {
    echo "Usage: $0 [OPTION]"
    echo ""
    echo "Options:"
    echo "  docs     Générer uniquement la documentation technique"
    echo "  ebook    Générer uniquement l'eBook d'apprentissage"
    echo "  all      Générer tout (par défaut)"
    echo "  serve    Générer et servir localement"
    echo "  clean    Nettoyer les fichiers générés"
    echo "  epub     Générer l'eBook en format EPUB"
    echo "  help     Afficher cette aide"
    echo ""
    echo "Exemples:"
    echo "  $0           # Génère tout"
    echo "  $0 docs      # Génère seulement la documentation"
    echo "  $0 serve     # Génère et sert sur http://localhost:3000"
    echo "  $0 epub      # Génère l'eBook en EPUB"
}

# Vérifier que mdBook est installé
check_mdbook() {
    if ! command -v mdbook &> /dev/null; then
        echo "❌ mdBook n'est pas installé."
        echo "💡 Installation : cargo install mdbook"
        exit 1
    fi
    echo "✅ mdBook détecté : $(mdbook --version)"
}

# Nettoyer les fichiers générés
clean_build() {
    echo "🧹 Nettoyage des fichiers générés..."
    rm -rf docs/book ebook/book
    rm -f ebook/*.epub
    echo "✅ Nettoyage terminé"
}

# Générer la documentation technique
build_docs() {
    echo "📚 Génération de la documentation technique..."
    cd docs
    mdbook build
    echo "✅ Documentation générée dans : docs/book/"
    cd ..
}

# Générer l'eBook
build_ebook() {
    echo "📖 Génération de l'eBook d'apprentissage..."
    cd ebook
    mdbook build
    echo "✅ eBook généré dans : ebook/book/"
    cd ..
}

# Servir localement
serve_docs() {
    echo "🌐 Démarrage du serveur local..."
    echo "📚 Documentation : http://localhost:3000"
    echo "📖 eBook : http://localhost:3001"
    echo ""
    echo "Appuyez sur Ctrl+C pour arrêter"
    
    # Démarrer les serveurs en arrière-plan
    cd docs && mdbook serve --port 3000 &
    DOCS_PID=$!
    
    cd ../ebook && mdbook serve --port 3001 &
    EBOOK_PID=$!
    
    cd ..
    
    # Fonction de nettoyage à l'arrêt
    cleanup() {
        echo ""
        echo "🛑 Arrêt des serveurs..."
        kill $DOCS_PID $EBOOK_PID 2>/dev/null
        exit 0
    }
    
    trap cleanup SIGINT SIGTERM
    wait
}

# Générer l'EPUB
build_epub() {
    echo "📕 Génération de l'eBook en format EPUB..."
    
    # Vérifier que pandoc est installé
    if ! command -v pandoc &> /dev/null; then
        echo "❌ Pandoc n'est pas installé."
        echo "💡 Installation sur macOS : brew install pandoc"
        echo "💡 Installation sur Ubuntu : sudo apt-get install pandoc"
        exit 1
    fi
    
    # Générer l'HTML d'abord
    build_ebook
    
    cd ebook
    
    # Créer un fichier de métadonnées EPUB
    cat > metadata.xml << EOF
<dc:title>Apprendre Dabara - Guide Complet</dc:title>
<dc:creator>Ahmad</dc:creator>
<dc:language>fr</dc:language>
<dc:subject>Programmation, Haoussa, Informatique</dc:subject>
<dc:description>Guide complet pour apprendre le langage de programmation Dabara en français</dc:description>
<dc:publisher>Projet Dabara</dc:publisher>
<dc:date>$(date +%Y-%m-%d)</dc:date>
EOF
    
    # Générer l'EPUB
    pandoc --from=html --to=epub3 \
           --epub-metadata=metadata.xml \
           --toc-depth=3 \
           --output=dabara-guide-complet.epub \
           book/index.html
    
    echo "✅ EPUB généré : ebook/dabara-guide-complet.epub"
    
    # Nettoyer
    rm -f metadata.xml
    cd ..
}

# Afficher les statistiques
show_stats() {
    echo ""
    echo "📊 Statistiques de génération :"
    
    if [ -d "docs/book" ]; then
        docs_size=$(du -sh docs/book | cut -f1)
        docs_files=$(find docs/book -name "*.html" | wc -l)
        echo "  📚 Documentation : $docs_size ($docs_files pages)"
    fi
    
    if [ -d "ebook/book" ]; then
        ebook_size=$(du -sh ebook/book | cut -f1)
        ebook_files=$(find ebook/book -name "*.html" | wc -l)
        echo "  📖 eBook HTML : $ebook_size ($ebook_files pages)"
    fi
    
    if [ -f "ebook/dabara-guide-complet.epub" ]; then
        epub_size=$(du -sh ebook/dabara-guide-complet.epub | cut -f1)
        echo "  📕 eBook EPUB : $epub_size"
    fi
    
    echo ""
}

# Main
main() {
    check_mdbook
    
    case "${1:-all}" in
        "docs")
            build_docs
            ;;
        "ebook")
            build_ebook
            ;;
        "all")
            build_docs
            build_ebook
            ;;
        "serve")
            build_docs
            build_ebook
            serve_docs
            ;;
        "clean")
            clean_build
            ;;
        "epub")
            build_epub
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            echo "❌ Option inconnue: $1"
            echo "💡 Utilisez '$0 help' pour voir les options disponibles"
            exit 1
            ;;
    esac
    
    show_stats
}

main "$@"