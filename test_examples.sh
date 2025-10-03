#!/bin/bash

# Script de test automatique pour les exemples Dabara
# Teste tous les exemples fonctionnels

echo "=== Test automatique des exemples Dabara ==="
echo ""

# Couleurs pour l'affichage
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Compteurs
total=0
success=0
failed=0

# Fonction pour tester un exemple
test_example() {
    local file=$1
    local description=$2
    
    echo "🧪 Test: $description"
    echo "   Fichier: $file"
    
    if cargo run --quiet "examples/$file" > /dev/null 2>&1; then
        echo -e "   ${GREEN}✅ RÉUSSI${NC}"
        ((success++))
    else
        echo -e "   ${RED}❌ ÉCHEC${NC}"
        ((failed++))
    fi
    
    ((total++))
    echo ""
}

# Aller dans le répertoire du projet
cd "$(dirname "$0")"

echo "📁 Répertoire de travail: $(pwd)"
echo ""

# Tests des exemples fonctionnels
echo "=== TESTS DES EXEMPLES FONCTIONNELS ==="
echo ""

test_example "test_001_variables_et_affichage.ha" "Variables et affichage de base"
test_example "test_001b_listes.ha" "Listes et collections"
test_example "test_001c_arithmetique.ha" "Opérations arithmétiques"
test_example "test_002_entree_utilisateur_simple.ha" "Guide entrée utilisateur"
test_example "test_003_conditions.ha" "Conditions if/else"
test_example "test_004_fonctions_attente.ha" "Guide fonctions (en attente)"
test_example "test_005_unicode_haoussa.ha" "Support Unicode Haoussa"
test_example "test_006_programme_complet.ha" "Programme complet (calculatrice)"

# Résumé
echo "=== RÉSUMÉ DES TESTS ==="
echo "Total des tests: $total"
echo -e "Réussites: ${GREEN}$success${NC}"
echo -e "Échecs: ${RED}$failed${NC}"

if [ $failed -eq 0 ]; then
    echo -e "${GREEN}🎉 Tous les tests ont réussi !${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  Certains tests ont échoué.${NC}"
    exit 1
fi