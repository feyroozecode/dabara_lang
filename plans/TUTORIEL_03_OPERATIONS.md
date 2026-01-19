# Tutoriel 03 - Opérations Arithmétiques

**Difficulté** : 🟢 FACILE  
**Durée estimée** : 35 minutes  
**Prérequis** : [Tutoriel 01](TUTORIEL_01_BASES.md), [Tutoriel 02](TUTORIEL_02_TYPES.md)

---

## 📖 Objectifs de ce Tutoriel

À la fin de ce tutoriel, vous saurez :
- ✅ Effectuer les quatre opérations de base (+, -, *, /)
- ✅ Comprendre la priorité des opérations
- ✅ Créer des calculatrices simples
- ✅ Combiner calculs et variables

---

## 1️⃣ Les Quatre Opérations de Base

Dabara supporte les opérations arithmétiques standards avec les symboles mathématiques.

### Tableau des Opérateurs

| Opération | Symbole | Mot Haoussa | Exemple | Résultat |
|-----------|---------|-------------|---------|----------|
| Addition | `+` | ƙara | `5 + 3` | `8` |
| Soustraction | `-` | rage | `10 - 4` | `6` |
| Multiplication | `*` | ninka | `6 * 7` | `42` |
| Division | `/` | raba | `20 / 4` | `5` |

---

## 2️⃣ Addition (+)

L'addition combine deux nombres pour obtenir leur somme.

### Exemple Simple

```hausa
fara
  naɗa jimla = 5 + 3
  rubuta jimla
ƙare
```

**Sortie** :
```
8
```

### Avec Variables

```hausa
fara
  naɗa lambar1 = 10
  naɗa lambar2 = 25
  naɗa jimla = lambar1 + lambar2
  
  rubuta "Lambar 1: "
  rubuta lambar1
  rubuta "Lambar 2: "
  rubuta lambar2
  rubuta "Jimla: "
  rubuta jimla
ƙare
```

**Sortie** :
```
Lambar 1: 10
Lambar 2: 25
Jimla: 35
```

### Additionner Plusieurs Nombres

```hausa
fara
  naɗa a = 5
  naɗa b = 10
  naɗa c = 15
  naɗa jimla = a + b + c
  
  rubuta jimla
ƙare
```

**Sortie** :
```
30
```

### 🔍 Applications Pratiques

**Calculer un total**
```hausa
fara
  naɗa kudin_littafi = 500
  naɗa kudin_alkalam = 100
  naɗa kudin_jaka = 1200
  naɗa jimlar_kudi = kudin_littafi + kudin_alkalam + kudin_jaka
  
  rubuta "Jimlar kudi: "
  rubuta jimlar_kudi
ƙare
```

**Compteur**
```hausa
fara
  naɗa lambar = 0
  lambar = lambar + 1  # Incrémente de 1
  rubuta lambar        # Affiche: 1
  
  lambar = lambar + 1
  rubuta lambar        # Affiche: 2
ƙare
```

### Exercice 1

Créez un programme qui calcule le total de vos dépenses de la semaine :
- Lundi : 200
- Mardi : 150
- Mercredi : 300
- Jeudi : 175
- Vendredi : 400

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa litinin = 200
  naɗa talata = 150
  naɗa laraba = 300
  naɗa alhamis = 175
  naɗa jumma = 400
  
  naɗa jimla = litinin + talata + laraba + alhamis + jumma
  
  rubuta "Kashe kudi na mako:"
  rubuta "Litinin: "
  rubuta litinin
  rubuta "Talata: "
  rubuta talata
  rubuta "Laraba: "
  rubuta laraba
  rubuta "Alhamis: "
  rubuta alhamis
  rubuta "Jumma'a: "
  rubuta jumma
  rubuta "Jimla: "
  rubuta jimla
ƙare
```

**Sortie** :
```
Jimla: 1225
```
</details>

---

## 3️⃣ Soustraction (-)

La soustraction retire un nombre d'un autre.

### Exemple Simple

```hausa
fara
  naɗa bambanci = 10 - 3
  rubuta bambanci
ƙare
```

**Sortie** :
```
7
```

### Avec Variables

```hausa
fara
  naɗa duka = 100
  naɗa kashewa = 35
  naɗa sauran = duka - kashewa
  
  rubuta "Duka: "
  rubuta duka
  rubuta "Kashewa: "
  rubuta kashewa
  rubuta "Sauran: "
  rubuta sauran
ƙare
```

**Sortie** :
```
Duka: 100
Kashewa: 35
Sauran: 65
```

### Nombres Négatifs

```hausa
fara
  naɗa sakamako = 5 - 10
  rubuta sakamako
ƙare
```

**Sortie** :
```
-5
```

### 🔍 Applications Pratiques

**Calculer le reste**
```hausa
fara
  naɗa kudin_asali = 5000
  naɗa kashe = 1200
  naɗa saura = kudin_asali - kashe
  
  rubuta "Kudin da ya saura: "
  rubuta saura
ƙare
```

**Différence d'âge**
```hausa
fara
  naɗa shekarun_babba = 30
  naɗa shekarun_karami = 18
  naɗa bambanci = shekarun_babba - shekarun_karami
  
  rubuta "Bambancin shekaru: "
  rubuta bambanci
ƙare
```

### Exercice 2

Vous avez 1000 dans votre porte-monnaie. Vous achetez :
- Un livre : 250
- Un stylo : 50
- Un cahier : 100

Calculez combien il vous reste.

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa kudin_asali = 1000
  naɗa littafi = 250
  naɗa alkalam = 50
  naɗa littafin_rubutu = 100
  
  naɗa jimlar_kashe = littafi + alkalam + littafin_rubutu
  naɗa saura = kudin_asali - jimlar_kashe
  
  rubuta "Kudin asali: "
  rubuta kudin_asali
  rubuta "Jimlar kashewa: "
  rubuta jimlar_kashe
  rubuta "Kudin da ya saura: "
  rubuta saura
ƙare
```

**Sortie** :
```
Kudin da ya saura: 600
```
</details>

---

## 4️⃣ Multiplication (*)

La multiplication répète l'addition d'un nombre.

### Exemple Simple

```hausa
fara
  naɗa sakamako = 6 * 7
  rubuta sakamako
ƙare
```

**Sortie** :
```
42
```

### Avec Variables

```hausa
fara
  naɗa farashin = 50
  naɗa adadi = 4
  naɗa jimla = farashin * adadi
  
  rubuta "Farashin kowane: "
  rubuta farashin
  rubuta "Adadi: "
  rubuta adadi
  rubuta "Jimlar kudi: "
  rubuta jimla
ƙare
```

**Sortie** :
```
Farashin kowane: 50
Adadi: 4
Jimlar kudi: 200
```

### Table de Multiplication

```hausa
fara
  naɗa lambar = 5
  
  rubuta "Teburin ninka na 5:"
  rubuta lambar * 1
  rubuta lambar * 2
  rubuta lambar * 3
  rubuta lambar * 4
  rubuta lambar * 5
ƙare
```

**Sortie** :
```
Teburin ninka na 5:
5
10
15
20
25
```

### 🔍 Applications Pratiques

**Calculer une surface (rectangle)**
```hausa
fara
  naɗa tsawo = 10
  naɗa faɗi = 5
  naɗa yanki = tsawo * faɗi
  
  rubuta "Tsawo: "
  rubuta tsawo
  rubuta "Faɗi: "
  rubuta faɗi
  rubuta "Yanki: "
  rubuta yanki
ƙare
```

**Salaire mensuel**
```hausa
fara
  naɗa albashi_rana = 2000
  naɗa ranakun_aiki = 22
  naɗa jimlar_albashi = albashi_rana * ranakun_aiki
  
  rubuta "Albashi na wata: "
  rubuta jimlar_albashi
ƙare
```

### Exercice 3

Un livre coûte 350. Vous voulez acheter 7 livres.
Calculez le coût total.

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa farashin_littafi = 350
  naɗa yawan_littattafai = 7
  naɗa jimlar_farashi = farashin_littafi * yawan_littattafai
  
  rubuta "Farashin littafi ɗaya: "
  rubuta farashin_littafi
  rubuta "Yawan littattafai: "
  rubuta yawan_littattafai
  rubuta "Jimlar farashi: "
  rubuta jimlar_farashi
ƙare
```

**Sortie** :
```
Jimlar farashi: 2450
```
</details>

---

## 5️⃣ Division (/)

La division partage un nombre en parties égales.

### Exemple Simple

```hausa
fara
  naɗa sakamako = 20 / 4
  rubuta sakamako
ƙare
```

**Sortie** :
```
5
```

### Avec Variables

```hausa
fara
  naɗa jimlar_kudi = 1000
  naɗa yawan_mutane = 5
  naɗa rabo = jimlar_kudi / yawan_mutane
  
  rubuta "Jimla: "
  rubuta jimlar_kudi
  rubuta "Yawan mutane: "
  rubuta yawan_mutane
  rubuta "Rabo kowane: "
  rubuta rabo
ƙare
```

**Sortie** :
```
Jimla: 1000
Yawan mutane: 5
Rabo kowane: 200
```

### ⚠️ Division Entière

**Important** : Dabara effectue une division **entière** (sans décimales).

```hausa
fara
  naɗa sakamako = 10 / 3
  rubuta sakamako  # Affiche 3, pas 3.333...
ƙare
```

**Sortie** :
```
3
```

Le reste est ignoré ! `10 / 3 = 3` (et non 3.33...)

### 🔍 Applications Pratiques

**Partager équitablement**
```hausa
fara
  naɗa kayan_abinci = 30
  naɗa dalibai = 6
  naɗa rabo_kowane = kayan_abinci / dalibai
  
  rubuta "Kowane dalibi ya samu: "
  rubuta rabo_kowane
ƙare
```

**Moyenne (approximative)**
```hausa
fara
  naɗa daraja1 = 85
  naɗa daraja2 = 90
  naɗa daraja3 = 78
  naɗa jimla = daraja1 + daraja2 + daraja3
  naɗa matsakaici = jimla / 3
  
  rubuta "Matsakaicin daraja: "
  rubuta matsakaici
ƙare
```

**Sortie** :
```
Matsakaicin daraja: 84
```
(253 / 3 = 84, le reste est ignoré)

### Exercice 4

Un sac de riz de 50 kg doit être partagé équitablement entre 8 personnes.
Combien de kg chacun reçoit-il ? (Division entière)

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa jimlar_shinkafa = 50
  naɗa yawan_mutane = 8
  naɗa rabo_kowane = jimlar_shinkafa / yawan_mutane
  
  rubuta "Jimlar shinkafa (kg): "
  rubuta jimlar_shinkafa
  rubuta "Yawan mutane: "
  rubuta yawan_mutane
  rubuta "Rabo kowane (kg): "
  rubuta rabo_kowane
ƙare
```

**Sortie** :
```
Rabo kowane (kg): 6
```
(50 / 8 = 6, avec un reste de 2 kg)
</details>

---

## 6️⃣ Combiner les Opérations

Vous pouvez combiner plusieurs opérations dans une même expression.

### Expressions Composées

```hausa
fara
  naɗa sakamako = 10 + 5 * 2
  rubuta sakamako
ƙare
```

**Sortie** :
```
20
```

### 🔍 Priorité des Opérations

Dabara suit les règles mathématiques standard (PEMDAS) :

1. **Parenthèses** `( )`
2. **Multiplication** `*` et **Division** `/` (gauche à droite)
3. **Addition** `+` et **Soustraction** `-` (gauche à droite)

### Exemples de Priorité

**Sans parenthèses**
```hausa
fara
  naɗa resultat = 2 + 3 * 4
  rubuta resultat  # 2 + 12 = 14 (multiplication d'abord)
ƙare
```

**Avec parenthèses**
```hausa
fara
  naɗa resultat = (2 + 3) * 4
  rubuta resultat  # 5 * 4 = 20 (parenthèses d'abord)
ƙare
```

**Comparaison**
```hausa
fara
  naɗa sans = 10 + 20 / 2
  naɗa avec = (10 + 20) / 2
  
  rubuta "Sans parenthèses: "
  rubuta sans   # 10 + 10 = 20
  rubuta "Avec parenthèses: "
  rubuta avec   # 30 / 2 = 15
ƙare
```

**Sortie** :
```
Sans parenthèses: 20
Avec parenthèses: 15
```

### Exercice 5

Calculez les expressions suivantes et vérifiez avec Dabara :
1. `5 + 3 * 2`
2. `(5 + 3) * 2`
3. `20 - 10 / 2`
4. `(20 - 10) / 2`

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa exp1 = 5 + 3 * 2
  naɗa exp2 = (5 + 3) * 2
  naɗa exp3 = 20 - 10 / 2
  naɗa exp4 = (20 - 10) / 2
  
  rubuta "5 + 3 * 2 = "
  rubuta exp1  # 11
  rubuta "(5 + 3) * 2 = "
  rubuta exp2  # 16
  rubuta "20 - 10 / 2 = "
  rubuta exp3  # 15
  rubuta "(20 - 10) / 2 = "
  rubuta exp4  # 5
ƙare
```

**Sortie** :
```
11
16
15
5
```
</details>

---

## 7️⃣ Programme Complet : Calculatrice Simple

Créons une calculatrice qui effectue toutes les opérations de base.

**Fichier : `ƙididdiga.ha`** (calculatrice)

```hausa
# Programme : Calculatrice simple
# Effectue les 4 opérations de base

fara
  # Définir les nombres
  naɗa lambar1 = 20
  naɗa lambar2 = 5
  
  # Calculer
  naɗa jimla = lambar1 + lambar2
  naɗa bambanci = lambar1 - lambar2
  naɗa ninka = lambar1 * lambar2
  naɗa raba = lambar1 / lambar2
  
  # Afficher les résultats
  rubuta "================================"
  rubuta "      ƘIDIDDIGA / CALCULATRICE"
  rubuta "================================"
  rubuta "Lambar 1: "
  rubuta lambar1
  rubuta "Lambar 2: "
  rubuta lambar2
  rubuta "--------------------------------"
  rubuta "Jimla (+): "
  rubuta jimla
  rubuta "Bambanci (-): "
  rubuta bambanci
  rubuta "Ninka (*): "
  rubuta ninka
  rubuta "Raba (/): "
  rubuta raba
  rubuta "================================"
ƙare
```

**Sortie** :
```
================================
      ƘIDIDDIGA / CALCULATRICE
================================
Lambar 1: 20
Lambar 2: 5
--------------------------------
Jimla (+): 25
Bambanci (-): 15
Ninka (*): 100
Raba (/): 4
================================
```

---

## 8️⃣ Applications Pratiques

### Budget Mensuel

```hausa
fara
  # Revenus
  naɗa albashi = 50000
  
  # Dépenses
  naɗa haya = 15000
  naɗa abinci = 12000
  naɗa motoci = 5000
  naɗa wasu = 8000
  
  # Calculs
  naɗa jimlar_kashewa = haya + abinci + motoci + wasu
  naɗa ajiya = albashi - jimlar_kashewa
  
  # Affichage
  rubuta "=== KASAFIN KUDI NA WATA ==="
  rubuta "Albashi: "
  rubuta albashi
  rubuta "Jimlar kashewa: "
  rubuta jimlar_kashewa
  rubuta "Ajiya: "
  rubuta ajiya
ƙare
```

### Convertisseur de Distance

```hausa
fara
  # Convertir kilomètres en mètres
  naɗa kilomita = 5
  naɗa mita = kilomita * 1000
  
  rubuta "Kilomita: "
  rubuta kilomita
  rubuta "Mita: "
  rubuta mita
ƙare
```

### Calculateur de Note Finale

```hausa
fara
  # Notes sur différents examens
  naɗa gwaji1 = 85
  naɗa gwaji2 = 90
  naɗa gwaji3 = 78
  naɗa aikin_gida = 95
  
  # Calcul de la moyenne
  naɗa jimla = gwaji1 + gwaji2 + gwaji3 + aikin_gida
  naɗa matsakaici = jimla / 4
  
  rubuta "=== DARAJA NA ƘARSHE ==="
  rubuta "Gwaji 1: "
  rubuta gwaji1
  rubuta "Gwaji 2: "
  rubuta gwaji2
  rubuta "Gwaji 3: "
  rubuta gwaji3
  rubuta "Aikin gida: "
  rubuta aikin_gida
  rubuta "Matsakaici: "
  rubuta matsakaici
ƙare
```

---

## 📝 Résumé du Tutoriel

| Opération | Symbole | Exemple | Résultat | Priorité |
|-----------|---------|---------|----------|----------|
| Addition | `+` | `5 + 3` | `8` | 3 (faible) |
| Soustraction | `-` | `10 - 4` | `6` | 3 (faible) |
| Multiplication | `*` | `6 * 7` | `42` | 2 (haute) |
| Division | `/` | `20 / 4` | `5` | 2 (haute) |
| Parenthèses | `( )` | `(2 + 3) * 4` | `20` | 1 (très haute) |

---

## 🎯 Points Clés à Retenir

1. ✅ **Quatre opérations** : `+`, `-`, `*`, `/`
2. ✅ **Division entière** : pas de décimales
3. ✅ **Priorité** : `*` et `/` avant `+` et `-`
4. ✅ **Parenthèses** : modifient la priorité
5. ✅ **Combiner** : plusieurs opérations dans une expression
6. ✅ **Variables** : peuvent être utilisées dans les calculs

---

## 🚀 Projet Final : Gestionnaire de Budget

Créez un programme `kasafin_kudi.ha` qui :

1. Définit votre salaire mensuel
2. Liste au moins 5 dépenses différentes
3. Calcule le total des dépenses
4. Calcule combien il vous reste
5. Calcule combien vous économisez par jour (reste / 30)

Affichez tous les résultats de manière claire et organisée.

<details>
<summary>💡 Voir une solution possible</summary>

```hausa
fara
  # Revenus
  naɗa albashi_wata = 60000
  
  # Dépenses
  naɗa hayan_gida = 20000
  naɗa abinci = 15000
  naɗa motoci = 8000
  naɗa wutar_lantarki = 3000
  naɗa ruwa = 2000
  naɗa sadarwa = 5000
  
  # Calculs
  naɗa jimlar_kashewa = hayan_gida + abinci + motoci + wutar_lantarki + ruwa + sadarwa
  naɗa saura = albashi_wata - jimlar_kashewa
  naɗa ajiyar_rana = saura / 30
  
  # Affichage
  rubuta "======================================"
  rubuta "   KASAFIN KUDI NA WATA / BUDGET     "
  rubuta "======================================"
  rubuta "SAMUN KUDI:"
  rubuta "Albashi: "
  rubuta albashi_wata
  rubuta ""
  rubuta "KASHEWA:"
  rubuta "Hayan gida: "
  rubuta hayan_gida
  rubuta "Abinci: "
  rubuta abinci
  rubuta "Motoci: "
  rubuta motoci
  rubuta "Wutar lantarki: "
  rubuta wutar_lantarki
  rubuta "Ruwa: "
  rubuta ruwa
  rubuta "Sadarwa: "
  rubuta sadarwa
  rubuta "---"
  rubuta "Jimlar kashewa: "
  rubuta jimlar_kashewa
  rubuta ""
  rubuta "AJIYA:"
  rubuta "Saura: "
  rubuta saura
  rubuta "Ajiyar rana: "
  rubuta ajiyar_rana
  rubuta "======================================"
ƙare
```

**Sortie** :
```
Saura: 7000
Ajiyar rana: 233
```
</details>

---

## 🎓 Quiz de Validation

1. Quel est le résultat de `10 + 5 * 2` ?
2. Quel est le résultat de `(10 + 5) * 2` ?
3. Que se passe-t-il avec `10 / 3` en Dabara ?
4. Quelle opération a la priorité la plus élevée ?
5. Comment calculer la moyenne de 3 nombres ?

<details>
<summary>📖 Voir les réponses</summary>

1. `20` (multiplication d'abord : 10 + 10)
2. `30` (parenthèses d'abord : 15 * 2)
3. Division entière : donne `3` (reste ignoré)
4. Les parenthèses `( )`, puis `*` et `/`
5. `(a + b + c) / 3`
</details>

---

## ➡️ Prochaine Étape

Maintenant que vous maîtrisez les calculs, apprenons à prendre des décisions !

➡️ [**Tutoriel 04 - Conditions et Comparaisons**](TUTORIEL_04_CONDITIONS.md)

---

**Gaya da lissafi!** (En avant avec les mathématiques!) ➕➖✖️➗
