# Hello World# Hello World

Le programme "Hello World" traditionnel en Dabara est simple et élégant. Ce chapitre vous guide à travers vos premiers pas.

## Programme de base

### Version simple

```hausa
fara
  rubuta "Sannu duniya!"
ƙare
```

**Sortie :**
```
Sannu duniya!
```

### Explication ligne par ligne

1. `fara` - Commence le programme (comme `{` en C/Java)
2. `rubuta "Sannu duniya!"` - Affiche le texte "Sannu duniya!"
3. `ƙare` - Termine le programme (comme `}` en C/Java)

## Variations

### Avec variables

```hausa
fara
  naɗa gaisuwa = "Sannu duniya!"
  rubuta gaisuwa
ƙare
```

### Avec concaténation

```hausa
fara
  naɗa kalmar_gaisuwa = "Sannu"
  naɗa kalmar_duniya = "duniya"
  naɗa jimla_cikakke = kalmar_gaisuwa + " " + kalmar_duniya + "!"
  
  rubuta jimla_cikakke
ƙare
```

### Version interactive

```hausa
fara
  naɗa sunan_daskaraci = "Ahmad"
  naɗa gaisuwa = "Sannu " + sunan_daskaraci + "!"
  naɗa maganar_maraba = "Barka da zuwa cikin duniyar Dabara"
  
  rubuta gaisuwa
  rubuta maganar_maraba
ƙare
```

**Sortie :**
```
Sannu Ahmad!
Barka da zuwa cikin duniyar Dabara
```

## Exemples multiculturels

### Salutations en différentes langues

```hausa
fara
  rubuta "Sannu duniya!"        // Haoussa
  rubuta "Bonjour le monde!"     // Français
  rubuta "Hello World!"          // Anglais
  rubuta "Hola Mundo!"           // Espagnol
  rubuta "السلام عليكم!"              // Arabe
ƙare
```

### Messages personnalisés

```hausa
fara
  naɗa sunan = "Khadija"
  naɗa wuri = "Kano"
  naɗa shekarun = 25
  
  rubuta "Sannu " + sunan + "!"
  rubuta "Kin zo daga " + wuri + " ko?"
  rubuta "Kina da shekaru " + shekarun + " ko?"
ƙare
```

**Sortie :**
```
Sannu Khadija!
Kin zo daga Kano ko?
Kina da shekaru 25 ko?
```

## Programmes avec calculs

### Hello World avec mathématiques

```hausa
fara
  naɗa lambar_gaisuwa = 2024
  naɗa lambar_shekarun_dabara = 1
  naɗa jimla = lambar_gaisuwa ƙara lambar_shekarun_dabara
  
  rubuta "Sannu daga shekara " + jimla + "!"
  rubuta "Dabara yana da shekara " + lambar_shekarun_dabara
ƙare
```

### Compteur simple

```hausa
fara
  naɗa ƙidaya = 1
  
  rubuta "Gaisuwa ta " + ƙidaya + ": Sannu!"
  ƙidaya = ƙidaya ƙara 1
  rubuta "Gaisuwa ta " + ƙidaya + ": Barka da zuwa!"
  
  ƙidaya = ƙidaya ƙara 1
  rubuta "Gaisuwa ta " + ƙidaya + ": Mu ci gaba!"
ƙare
```

**Sortie :**
```
Gaisuwa ta 1: Sannu!
Gaisuwa ta 2: Barka da zuwa!
Gaisuwa ta 3: Mu ci gaba!
```

## Débogage et tests

### Version avec informations de débogage

```hausa
fara
  naɗa debug_mode = gaskiya
  
  rubuta "=== Fara shirin Dabara ==="
  
  // Affichage principal
  rubuta "Sannu duniya!"
  
  // Informations de débogage
  rubuta "Debug: " + debug_mode
  rubuta "Harshe: Dabara v0.0.1"
  
  rubuta "=== ƙare shirin ==="
ƙare
```

### Test de différents types

```hausa
fara
  // Test de chaînes
  naɗa jimla = "Sannu duniya!"
  rubuta jimla
  
  // Test de nombres
  naɗa lambar = 2024
  rubuta lambar
  
  // Test de booléens
  naɗa sahihi = gaskiya
  rubuta sahihi
  
  naɗa karami = karya
  rubuta karami
ƙare
```

## Exercices pratiques

### Exercice 1: Présentation personnelle

Créez un programme qui affiche vos informations personnelles :

```hausa
fara
  // Remplissez avec vos informations
  naɗa sunan_ku = "VOTRE_NOM"
  naɗa shekarun_ku = VOTRE_AGE
  naɗa wuri_ku = "VOTRE_VILLE"
  
  rubuta "Sannu! Sunana " + sunan_ku
  rubuta "Ina da shekaru " + shekarun_ku
  rubuta "Na fito daga " + wuri_ku
ƙare
```

### Exercice 2: Calculatrice simple

Créez un programme qui fait des calculs simples :

```hausa
fara
  naɗa lambar1 = 10
  naɗa lambar2 = 5
  
  // Votre code ici
  // Calculez et affichez la somme et la différence
  
ƙare
```

**Solution :**
```hausa
fara
  naɗa lambar1 = 10
  naɗa lambar2 = 5
  
  naɗa jimla = lambar1 ƙara lambar2
  naɗa bambanci = lambar1 rage lambar2
  
  rubuta "Jimla: " + jimla
  rubuta "Bambanci: " + bambanci
ƙare
```

### Exercice 3: Créateur de messages

Créez des messages automatiques :

```hausa
fara
  naɗa sunan_aboki = "Fatima"
  naɗa lokacin_rana = "safiya"  // safiya, rana, magarib
  
  // Créez un message personnalisé basé sur le moment de la journée
  
ƙare
```

## Erreurs communes

### 1. Oubli des guillemets

```hausa
// ❌ Incorrect
fara
  rubuta Sannu duniya!
ƙare

// ✅ Correct
fara
  rubuta "Sannu duniya!"
ƙare
```

### 2. Oubli de fara/ƙare

```hausa
// ❌ Incorrect
rubuta "Sannu duniya!"

// ✅ Correct
fara
  rubuta "Sannu duniya!"
ƙare
```

### 3. Mauvaise indentation

```hausa
// ❌ Incorrect
fara
rubuta "Sannu duniya!"
ƙare

// ✅ Correct
fara
  rubuta "Sannu duniya!"
ƙare
```

## Versions alternatives

### Avec variantes latines

```hausa
fara
  nada gaisuwa = "Sannu duniya!"
  rubuta gaisuwa
kare
```

### Version minimaliste

```hausa
fara
rubuta "Sannu!"
ƙare
```

### Version exhaustive

```hausa
fara
  // En-tête du programme
  rubuta "================================"
  rubuta "    Programme Dabara v0.0.1    "
  rubuta "================================"
  
  // Variables du programme
  naɗa nom_programme = "Hello World"
  naɗa version = "1.0"
  naɗa auteur = "Votre nom"
  
  // Message principal
  naɗa gaisuwa_principal = "Sannu duniya daga Dabara!"
  rubuta gaisuwa_principal
  
  // Informations sur le programme
  rubuta "Nom: " + nom_programme
  rubuta "Version: " + version
  rubuta "Auteur: " + auteur
  
  // Pied de page
  rubuta "================================"
  rubuta "       Ƙare da aminci!         "
  rubuta "================================"
ƙare
```

## Conseils pour débutants

1. **Commencez simple** : Un programme Hello World basique d'abord
2. **Testez fréquemment** : Exécutez votre code après chaque modification
3. **Lisez les erreurs** : Les messages d'erreur en haoussa vous guident
4. **Expérimentez** : Modifiez les exemples pour voir ce qui se passe
5. **Soyez patient** : La programmation demande de la pratique

## Prochaines étapes

Maintenant que vous maîtrisez Hello World :

1. [Variables et calculs](./variables.md) - Apprenez à stocker des données
2. [Programmes complexes](./complex.md) - Créez des applications plus avancées
3. [Syntaxe de base](../language/syntax.md) - Approfondissez vos connaissances

---

**Mu ci gaba!** (Continuons !) 🚀
