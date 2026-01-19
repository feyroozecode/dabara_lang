#!/bin/bash

# Script pour générer le PDF du ebook Dabara
# Utilise le navigateur système pour l'impression en PDF

echo "=== Génération du PDF du ebook Dabara ==="

# Vérifier si le fichier HTML existe
if [ ! -f "dabara_ebook_print.html" ]; then
    echo "Erreur: Le fichier dabara_ebook_print.html n'existe pas"
    exit 1
fi

# Vérifier si bookcover.png existe
if [ ! -f "bookcover.png" ]; then
    echo "⚠️  Avertissement: bookcover.png n'a pas été trouvé"
    echo "Le ebook fonctionnera sans image de couverture"
fi

# Ouvrir le fichier HTML dans le navigateur par défaut
echo "Ouverture du fichier HTML dans votre navigateur..."
if command -v open &> /dev/null; then
    open dabara_ebook_print.html
elif command -v xdg-open &> /dev/null; then
    xdg-open dabara_ebook_print.html
else
    echo "Impossible d'ouvrir automatiquement le navigateur"
    echo "Veuillez ouvrir manuellement le fichier:"
    echo "$(pwd)/dabara_ebook_print.html"
fi

echo ""
echo "=== Instructions pour générer le PDF ==="
echo "1. Une fois le fichier ouvert dans votre navigateur:"
echo "2. Appuyez sur Ctrl+P (ou Cmd+P sur Mac)"
echo "3. Dans les options d'impression:"
echo "   - Destination: Enregistrer au format PDF"
echo "   - Disposition: Portrait"
echo "   - Marges: Personnalisées (minimum 1cm)"
echo "   - Options: Cocher 'Arrière-plans graphiques'"
echo "4. Cliquez sur 'Enregistrer'"
echo "5. Nommez le fichier: dabara_ebook.pdf"
echo ""
echo "💡 Conseils pour un meilleur résultat:"
echo "- Utilisez Chrome/Edge pour la meilleure qualité"
echo "- Vérifiez l'aperçu avant d'imprimer"
echo "- Assurez-vous que toutes les pages sont incluses"
echo ""

# Attendre un moment pour que l'utilisateur puisse lire les instructions
sleep 3

# Optionnel: ouvrir le finder/explorateur pour faciliter l'accès
if command -v open &> /dev/null; then
    echo "Ouverture du dossier contenant les fichiers..."
    open .
fi

echo "=== Processus terminé ==="
echo "Le PDF sera généré dans le même dossier que ce script"