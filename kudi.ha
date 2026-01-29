fara
# Variables globales
naɗa kudin_shiga = []
naɗa kudin_fitowa = []
# Fonction pour ajouter revenu
aiki kara_shiga(kudi, bayani) {
kudin_shiga.kara(kudi)
}
# Fonction pour ajouter dépense
aiki kara_fitowa(kudi, bayani) {
kudin_fitowa.kara(kudi)
}
# Ajout de revenus
kara_shiga(50000, "Mai")
kara_shiga(25000, "Kasuwanci")
# Ajout de dépenses
kara_fitowa(15000, "Abinci")
kara_fitowa(8000, "Transport")
# Calculs et affichage
rubuta "Jimlar shiga: " + kudin_shiga
rubuta "Jimlar fitowa: " + kudin_fitowa
kare
