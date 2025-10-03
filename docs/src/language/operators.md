# Opérateurs# Opérateurs

Les opérateurs en Dabara permettent d'effectuer des opérations sur les données. Ce chapitre détaille tous les opérateurs disponibles et leur utilisation.

## Opérateurs arithmétiques

### Addition (`ƙara` / `kara`)

L'opérateur d'addition combine deux nombres :

```hausa
fara
  naɗa a = 10
  naɗa b = 5
  naɗa jimla = a ƙara b    // 15
  
  // Variante latine
  naɗa jimla2 = a kara b   // 15
  
  rubuta jimla      // Affiche: 15
  rubuta jimla2     // Affiche: 15
ƙare
```

#### Caractéristiques
- **Types supportés** : Lambar (nombres)
- **Résultat** : Lambar
- **Variantes** : `ƙara` (Unicode) ou `kara` (Latin)

#### Exemples pratiques
```hausa
fara
  // Calculs simples
  naɗa lambar1 = 25
  naɗa lambar2 = 17
  naɗa total = lambar1 ƙara lambar2
  
  rubuta "Total: " + total    // Total: 42
  
  // Addition avec littéraux
  naɗa resultat = 100 ƙara 200
  rubuta resultat             // 300
  
  // Additions multiples
  naɗa a = 5
  naɗa b = 10
  naɗa c = 15
  naɗa somme = a ƙara b ƙara c  // Attention: évaluation gauche-droite
  rubuta somme                     // 30
ƙare
```

### Soustraction (`rage`)

L'opérateur de soustraction retranche un nombre d'un autre :

```hausa
fara
  naɗa a = 20
  naɗa b = 7
  naɗa bambanci = a rage b   // 13
  
  rubuta bambanci              // Affiche: 13
ƙare
```

#### Caractéristiques
- **Types supportés** : Lambar (nombres)
- **Résultat** : Lambar (peut être négatif)
- **Une seule forme** : `rage`

#### Exemples pratiques
```hausa
fara
  // Calculs de différences
  naɗa shekarun_yanzu = 2024
  naɗa shekarun_haihuwa = 1995
  naɗa shekarun_rayuwa = shekarun_yanzu rage shekarun_haihuwa
  
  rubuta "Shekaru: " + shekarun_rayuwa  // Shekaru: 29
  
  // Soustraction avec résultats négatifs
  naɗa ƙarami = 5
  naɗa babba = 10
  naɗa bambanci_korau = ƙarami rage babba
  rubuta bambanci_korau                 // -5
  
  // Décrémentation
  naɗa ƙidaya = 10
  ƙidaya = ƙidaya rage 1
  rubuta ƙidaya                        // 9
ƙare
```

## Opérateur de concaténation (`+`)

L'opérateur `+` concatène les chaînes de caractères :

```hausa
fara
  naɗa farko = "Sannu"
  naɗa biyu = "duniya"
  naɗa jimla = farko + " " + biyu + "!"
  
  rubuta jimla    // Affiche: Sannu duniya!
ƙare
```

### Caractéristiques
- **Types supportés** : Jimla (chaînes)
- **Résultat** : Jimla
- **Conversion automatique** : Nombres et booléens vers chaînes

### Concaténation avec conversion

```hausa
fara
  naɗa sunan = "Ahmad"
  naɗa shekarun = 25
  naɗa yana_aiki = gaskiya
  
  // Conversion automatique
  naɗa bayanai = "Sunan: " + sunan + ", Shekaru: " + shekarun + ", Ana aiki: " + yana_aiki
  
  rubuta bayanai
  // Affiche: Sunan: Ahmad, Shekaru: 25, Ana aiki: gaskiya
ƙare
```

### Exemples avancés

```hausa
fara
  // Construction de messages dynamiques
  naɗa sunan_aboki = "Fatima"
  naɗa yawan_littattafai = 15
  naɗa gidan_karatu = "Gidan karatu na jami'a"
  
  naɗa sakon = "Sannu " + sunan_aboki + "! " +
                "Kina da littattafai " + yawan_littattafai + " " +
                "a " + gidan_karatu + "."
  
  rubuta sakon
  // Affiche: Sannu Fatima! Kina da littattafai 15 a Gidan karatu na jami'a.
  
  // Concaténation avec caractères spéciaux
  naɗa layi1 = "Layi farko"
  naɗa layi2 = "Layi na biyu"
  naɗa jimla_layuka = layi1 + "\n" + layi2
  
  rubuta jimla_layuka
  // Affiche sur deux lignes
ƙare
```

## Opérateur d'assignation (`=`)

L'opérateur d'assignation attribue une valeur à une variable :

```hausa
fara
  // Déclaration et assignation
  naɗa sunan = "Khadija"
  naɗa lambar = 42
  
  // Réassignation
  sunan = "Aisha"
  lambar = 100
  
  rubuta sunan     // Affiche: Aisha
  rubuta lambar    // Affiche: 100
ƙare
```

### Caractéristiques
- **Syntaxe** : `variable = valeur`
- **Types** : Tous types supportés
- **Changement de type** : Autorisé (typage dynamique)

### Assignations multiples

```hausa
fara
  naɗa a = 10
  naɗa b = 5
  
  // Utilisation de variables dans l'assignation
  a = a ƙara b     // a devient 15
  b = a rage b      // b devient 10
  
  rubuta "a: " + a   // a: 15
  rubuta "b: " + b   // b: 10
  
  // Changement de type
  a = "Yanzu na zama jimla"  // a devient une chaîne
  rubuta a                    // Yanzu na zama jimla
ƙare
```

## Priorité des opérateurs

### Ordre d'évaluation

Dabara évalue les opérateurs de gauche à droite avec les priorités suivantes :

1. **Parenthèses** : `()` (plus haute priorité)
2. **Opérateurs arithmétiques** : `ƙara`, `rage`
3. **Concaténation** : `+`
4. **Assignation** : `=` (plus basse priorité)

### Exemples de priorité

```hausa
fara
  // Évaluation gauche-droite
  naɗa resultat1 = 10 ƙara 5 rage 3    // (10 + 5) - 3 = 12
  
  // Avec parenthèses
  naɗa resultat2 = 10 ƙara (5 rage 3)  // 10 + (5 - 3) = 12
  
  // Mélange arithmétique et concaténation
  naɗa a = 5
  naɗa b = 3
  naɗa message = "Résultat: " + (a ƙara b)  // "Résultat: 8"
  
  rubuta resultat1   // 12
  rubuta resultat2   // 12
  rubuta message     // Résultat: 8
ƙare
```

### Attention aux types

```hausa
fara
  naɗa lambar = 10
  naɗa jimla = "5"
  
  // ❌ Erreur - types incompatibles
  // naɗa kuskure = lambar ƙara jimla
  
  // ✅ Correct - concaténation
  naɗa sahihi = "Lambar: " + lambar  // "Lambar: 10"
  
  rubuta sahihi
ƙare
```

## Exemples pratiques complexes

### 1. Calculatrice de taxes

```hausa
fara
  naɗa farashi_asali = 1000
  naɗa haraji_kashi = 175  // 17.5% comme entier
  
  // Calcul de la taxe (simplifié)
  naɗa adadin_haraji = farashi_asali ƙara haraji_kashi
  naɗa farashi_gaba_daya = adadin_haraji
  
  rubuta "Farashi asali: " + farashi_asali
  rubuta "Haraji: " + haraji_kashi
  rubuta "Farashi gaba daya: " + farashi_gaba_daya
ƙare
```

### 2. Système de notation

```hausa
fara
  naɗa sunan_ɗalibi = "Ahmad"
  naɗa maki_farko = 85
  naɗa maki_biyu = 92
  naɗa maki_uku = 78
  
  // Calcul de la moyenne (simplifié)
  naɗa jimlar_maki = maki_farko ƙara maki_biyu ƙara maki_uku
  naɗa matsakaitar = jimlar_maki rage 50  // Approximation
  
  naɗa sakon_sakamako = "\u0186alibi: " + sunan_ɗalibi + "\n" +
                          "Maki farko: " + maki_farko + "\n" +
                          "Maki biyu: " + maki_biyu + "\n" +
                          "Maki uku: " + maki_uku + "\n" +
                          "Jimla: " + jimlar_maki + "\n" +
                          "Matsakaicin: " + matsakaitar
  
  rubuta sakon_sakamako
ƙare
```

### 3. Gestionnaire d'inventaire

```hausa
fara
  naɗa abu_sunan = "Littattafai"
  naɗa adadin_farko = 500
  naɗa adadin_sayar_wata = 150
  naɗa adadin_shigo_wata = 75
  
  // Calculs mensuels
  naɗa adadin_bayan_sayar = adadin_farko rage adadin_sayar_wata
  naɗa adadin_ƙarshe = adadin_bayan_sayar ƙara adadin_shigo_wata
  naɗa canjin_adadi = adadin_ƙarshe rage adadin_farko
  
  // Rapport
  naɗa rahoto = "=== RAHOTON INVENTORY ===\n" +
                 "Abu: " + abu_sunan + "\n" +
                 "Adadin farko: " + adadin_farko + "\n" +
                 "An sayar: " + adadin_sayar_wata + "\n" +
                 "An shigo: " + adadin_shigo_wata + "\n" +
                 "Adadin ƙarshe: " + adadin_ƙarshe + "\n" +
                 "Canji: " + canjin_adadi + "\n" +
                 "=========================="
  
  rubuta rahoto
ƙare
```

## Gestion d'erreurs d'opérateurs

### Erreurs de types

```hausa
// Ces opérations causeront des erreurs :

fara
  naɗa lambar = 10
  naɗa jimla = "Sannu"
  
  // ❌ Erreur: types incompatibles
  // naɗa kuskure1 = lambar ƙara jimla
  // Message: "Kuskure: Ba za a iya ƙara lambar da jimla ba"
  
  // ❌ Erreur: opération impossible
  // naɗa kuskure2 = jimla rage lambar
  // Message: "Kuskure: Ba za a iya rage lambar daga jimla ba"
ƙare
```

### Prévention d'erreurs

```hausa
fara
  naɗa lambar1 = 15
  naɗa lambar2 = 8
  
  // ✅ Vérifiez les types avant les opérations
  naɗa jimla = lambar1 ƙara lambar2     // OK - deux nombres
  naɗa bambanci = lambar1 rage lambar2  // OK - deux nombres
  
  // ✅ Utilisez la concaténation pour combiner avec du texte
  naɗa sakon = "Jimla: " + jimla + ", Bambanci: " + bambanci
  
  rubuta sakon  // Jimla: 23, Bambanci: 7
ƙare
```

## Bonnes pratiques

### 1. Noms d'opérateurs clairs

```hausa
// ✅ Bon - utilisez les noms haoussa appropriés
naɗa jimla = a ƙara b      // Addition claire
naɗa bambanci = a rage b   // Soustraction claire

// ❌ À éviter - mélange incohérent
naɗa jimla = a kara b      // Acceptable mais mélangez pas
naɗa bambanci = a ƙara b   // Erreur de logique
```

### 2. Regroupement avec parenthèses

```hausa
// ✅ Bon - intentions claires
naɗa resultat = (a ƙara b) rage (c ƙara d)

// ❌ Confus - ordre d'évaluation ambigu
naɗa resultat = a ƙara b rage c ƙara d
```

### 3. Variables intermédiaires

```hausa
// ✅ Bon - étapes claires
naɗa jimla_farko = a ƙara b
naɗa jimla_biyu = c ƙara d
naɗa bambanci_ƙarshe = jimla_farko rage jimla_biyu

// ❌ Difficile à lire
naɗa bambanci_ƙarshe = (a ƙara b) rage (c ƙara d)
```

### 4. Commentaires explicatifs

```hausa
fara
  naɗa farashi = 1000
  naɗa rangwame = 50      // Rangwame 5%
  
  // Lissafin farashi tare da rangwame
  naɗa jimlar_rangwame = farashi ƙara rangwame
  
  rubuta "Farashi da rangwame: " + jimlar_rangwame
ƙare
```

## Limitations actuelles

### Version 0.0.1
- Pas d'opérateurs de comparaison (`>`, `<`, `==`)
- Pas d'opérateurs logiques (`da`, `ko`, `ba`)
- Pas de multiplication ni division
- Pas d'opérateurs composés (`+=`, `-=`)

### Futures extensions (v0.1.0+)
```hausa
// Prévisions pour les futures versions :
naɗa sakamako = a ninka b       // Multiplication
naɗa rabu = a raba b            // Division
naɗa sahihi = a fi b girma      // Comparaison
naɗa sakon = gaskiya da karya   // Logique
```

## Prochaines étapes

- [Expressions](./expressions.md) - Combiner opérateurs et variables
- [Variables](./variables.md) - Revoir la gestion des variables
- [Exemples pratiques](../examples/variables.md) - Plus d'exemples d'usage
