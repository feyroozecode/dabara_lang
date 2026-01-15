#!/bin/bash

# Script de compilation du ebook Dabara
# Nécessite: pdflatex, latexmk ou texlive

echo "=== Compilation du ebook Dabara ==="

# Vérifier si pdflatex est installé
if ! command -v pdflatex &> /dev/null; then
    echo "Erreur: pdflatex n'est pas installé"
    echo "Installez TeX Live ou MacTeX"
    echo "Sur macOS: brew install --cask mactex"
    echo "Sur Ubuntu: sudo apt install texlive-latex-recommended texlive-fonts-recommended"
    exit 1
fi

# Compiler le document
echo "Compilation en cours..."
pdflatex -interaction=nonstopmode dabara_ebook.tex

# Compiler une deuxième fois pour les références croisées
echo "Finalisation..."
pdflatex -interaction=nonstopmode dabara_ebook.tex

# Nettoyer les fichiers temporaires
echo "Nettoyage..."
rm -f dabara_ebook.aux dabara_ebook.log dabara_ebook.toc dabara_ebook.out

echo "=== Compilation terminée ==="
echo "Le fichier PDF est disponible: dabara_ebook.pdf"

# Optionnel: ouvrir le PDF
if command -v open &> /dev/null; then
    echo "Ouverture du PDF..."
    open dabara_ebook.pdf
elif command -v xdg-open &> /dev/null; then
    echo "Ouverture du PDF..."
    xdg-open dabara_ebook.pdf
fi