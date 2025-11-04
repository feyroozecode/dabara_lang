# Opérateurs Arithmétiques

Dabara supporte les opérations arithmétiques de base avec des mots-clés en langue haoussa, rendant le langage plus accessible aux locuteurs natifs.

## Opérateurs de Base

### Addition (ƙara / kara)

L'opérateur d'addition permet de combiner deux nombres ou de concaténer des chaînes de caractères.

```dabara
fara
  naɗa a = 5
  naɗa b = 3
  naɗa c = a ƙara b  # Résultat: 8
  
  # Concaténation de chaînes
  naɗa nom = "Ahmad"
  naɗa sallama = "Sannu " ƙara nom  # Résultat: "Sannu Ahmad"
  
  # Alternative en latin
  naɗa d = a kara b  # Résultat: 8
ƙare
```

### Soustraction (rage)

L'opérateur de soustraction permet de calculer la différence entre deux nombres.

```dabara
fara
  naɗa a = 10
  naɗa b = 4
  naɗa c = a rage b  # Résultat: 6
ƙare
```

### Multiplication (ninka)

L'opérateur de multiplication permet de multiplier deux nombres.

```dabara
fara
  naɗa a = 6
  naɗa b = 7
  naɗa c = a ninka b  # Résultat: 42
ƙare
```

### Division (raba)

L'opérateur de division permet de diviser un nombre par un autre.

```dabara
fara
  naɗa a = 20
  naɗa b = 4
  naɗa c = a raba b  # Résultat: 5
ƙare
```

## Priorité des Opérateurs

Les opérateurs suivent l'ordre de priorité mathématique standard :

1. Multiplication (`ninka`) et Division (`raba`) - priorité élevée
2. Addition (`ƙara` / `kara`) et Soustraction (`rage`) - priorité normale

```dabara
fara
  naɗa resultat = 2 ƙara 3 ninka 4 rage 1
  # Équivalent à: 2 + (3 * 4) - 1 = 13
  rubuta resultat
ƙare
```

## Types Supportés

### Nombres Entiers

Tous les opérateurs arithmétiques fonctionnent avec les nombres entiers :

```dabara
fara
  naɗa a = 15
  naɗa b = 3
  rubuta a ƙara b   # 18
  rubuta a rage b   # 12
  rubuta a ninka b  # 45
  rubuta a raba b   # 5
ƙare
```

### Chaînes de Caractères

L'opérateur d'addition (`ƙara` / `kara`) peut concaténer des chaînes :

```dabara
fara
  naɗa prenom = "Ahmad"
  naɗa nom = "Ali"
  naɗa nom_complet = prenom ƙara " " ƙara nom
  rubuta nom_complet  # "Ahmad Ali"
ƙare
```

### Booléens

Les booléens peuvent être concaténés avec des chaînes :

```dabara
fara
  naɗa vrai = gaskiya
  naɗa faux = karya
  rubuta "Résultat: " ƙara vrai   # "Résultat: gaskiya"
  rubuta "Résultat: " ƙara faux   # "Résultat: karya"
ƙare
```

## Gestion des Erreurs

### Division par Zéro

Une erreur sera générée si vous tentez de diviser par zéro :

```dabara
fara
  naɗa a = 10
  naɗa b = 0
  # Ceci générera une erreur à l'exécution
  naɗa c = a raba b
ƙare
```

### Types Incompatibles

Certaines combinaisons de types génèrent des erreurs :

```dabara
fara
  naɗa nombre = 5
  naɗa chaine = "texte"
  # Ceci générera une erreur car la soustraction n'est pas définie pour ces types
  naɗa resultat = nombre rage chaine
ƙare
```

## Tableau de Correspondance

| Symbole | Hausa | Latin | Opération |
|---------|-------|-------|-----------|
| + | ƙara | kara | Addition |
| - | rage | rage | Soustraction |
| * | ninka | ninka | Multiplication |
| / | raba | raba | Division |

## Exemples Pratiques

### Calculatrice Simple

```dabara
fara
  rubuta "=== Mini Calculatrice ==="
  
  naɗa a = 12
  naɗa b = 4
  
  rubuta "a = "
  rubuta a
  rubuta "b = "
  rubuta b
  
  rubuta "Addition (a ƙara b): "
  rubuta a ƙara b
  
  rubuta "Soustraction (a rage b): "
  rubuta a rage b
  
  rubuta "Multiplication (a ninka b): "
  rubuta a ninka b
  
  rubuta "Division (a raba b): "
  rubuta a raba b
ƙare
```

### Calcul d'Âge

```dabara
fara
  naɗa annee_naissance = 1990
  naɗa annee_actuelle = 2024
  naɗa age = annee_actuelle rage annee_naissance
  rubuta "Vous avez "
  rubuta age
  rubuta " ans"
ƙare
```

## Bonnes Pratiques

1. **Utilisez des parenthèses** pour clarifier l'ordre des opérations lorsque nécessaire :
   ```dabara
   # Claire
   naɗa resultat = (a ƙara b) ninka c
   
   # Moins claire
   naɗa resultat = a ƙara b ninka c
   ```

2. **Soyez conscient des types** lors des opérations :
   ```dabara
   # Bon usage
   naɗa nom = "Ahmad"
   naɗa message = "Sannu " ƙara nom
   
   # Évitez les opérations non définies
   # naɗa erreur = "texte" rage "autre"  # Génère une erreur
   ```

3. **Gérez les divisions par zéro** dans vos programmes :
   ```dabara
   fara
     naɗa denominateur = karɓa  # Entrée utilisateur
     
     idan denominateur != 0 {
       naɗa resultat = 100 raba denominateur
       rubuta resultat
     } amma {
       rubuta "Erreur: Division par zéro impossible"
     }
   ƙare
   ```