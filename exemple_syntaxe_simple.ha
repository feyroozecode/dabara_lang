fara
    # Test de la syntaxe simplifiée des fonctions (style Ruby/Python)
    
    # Syntaxe traditionnelle avec parenthèses
    aiki salutation(nom) {
        rubuta "Sannu " + nom + "!"
    }
    
    # Tests dappels de fonctions
    naɗa nom = "Ahmad"
    
    # Appel avec parenthèses (style traditionnel)
    salutation(nom)
    
    # Appel sans parenthèses (style Ruby/Python) - NOUVEAU !
    salutation nom
    
    # Exemple avec plusieurs arguments
    aiki tamba(nom, shekara) {
        rubuta nom + " yana da shekara " + shekara
    }
    
    # Appel traditionnel
    tamba("Fatima", 25)
    
    # Appel style Ruby/Python - NOUVEAU !
    tamba "Fatima" 25
    tamba "Ibrahim", 30
    
    # Test avec variables
    naɗa suna = "Aisha"
    naɗa umuri = 22
    tamba suna umuri
    
ƙare