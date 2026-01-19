# Tutoriel 04 - Conditions et Comparaisons

**Difficulté** : 🟡 MOYEN  
**Durée estimée** : 45 minutes  
**Prérequis** : [Tutoriel 01-03](TUTORIEL_01_BASES.md)

---

## 📖 Objectifs de ce Tutoriel

À la fin de ce tutoriel, vous saurez :
- ✅ Utiliser les conditions `idan`, `amma`, `ammaina`
- ✅ Comparer des valeurs avec `==`, `!=`, `<`, `>`, `<=`, `>=`
- ✅ Prendre des décisions dans vos programmes
- ✅ Créer des programmes interactifs

---

## 1️⃣ Les Conditions : Prendre des Décisions

Les conditions permettent à votre programme de **prendre des décisions** basées sur des critères.

### Structure de Base

```hausa
idan condition {
  # Code exécuté si la condition est vraie
}
```

### Exemple Simple

```hausa
fara
  naɗa lambar = 10
  
  idan lambar > 5 {
    rubuta "Lambar ta fi 5"
  }
ƙare
```

**Sortie** :
```
Lambar ta fi 5
```

### 🔍 Comment ça Marche ?

1. **Évaluation** : `lambar > 5` → `10 > 5` → `gaskiya` (vrai)
2. **Décision** : Si vrai, exécuter le code entre `{ }`
3. **Résultat** : Le message est affiché

---

## 2️⃣ Les Mots-Clés de Condition

| Hausa | Français | Utilisation |
|-------|----------|-------------|
| `idan` | si | Condition principale |
| `amma` | sinon | Alternative (else) |
| `ammaina` | sinon si | Condition supplémentaire (else if) |

### Structure Complète

```hausa
idan condition1 {
  # Si condition1 est vraie
} ammaina condition2 {
  # Sinon, si condition2 est vraie
} amma {
  # Sinon (aucune condition vraie)
}
```

---

## 3️⃣ Les Opérateurs de Comparaison

### Tableau des Opérateurs

| Opérateur | Signification | Exemple | Résultat |
|-----------|---------------|---------|----------|
| `==` | égal à | `5 == 5` | `gaskiya` |
| `!=` | différent de | `5 != 3` | `gaskiya` |
| `<` | inférieur à | `3 < 5` | `gaskiya` |
| `>` | supérieur à | `10 > 7` | `gaskiya` |
| `<=` | inférieur ou égal | `5 <= 5` | `gaskiya` |
| `>=` | supérieur ou égal | `8 >= 3` | `gaskiya` |

---

## 4️⃣ Égalité (==)

Teste si deux valeurs sont **identiques**.

### Avec Nombres

```hausa
fara
  naɗa lambar = 10
  
  idan lambar == 10 {
    rubuta "Lambar daidai da 10"
  }
ƙare
```

**Sortie** :
```
Lambar daidai da 10
```

### Avec Chaînes

```hausa
fara
  naɗa sunan = "Ahmad"
  
  idan sunan == "Ahmad" {
    rubuta "Sannu Ahmad!"
  }
ƙare
```

**Sortie** :
```
Sannu Ahmad!
```

### Avec Booléens

```hausa
fara
  naɗa sahihi = gaskiya
  
  idan sahihi == gaskiya {
    rubuta "Gaskiya ne!"
  }
ƙare
```

**Sortie** :
```
Gaskiya ne!
```

### Exercice 1

Créez un programme qui vérifie si votre âge est exactement 18 ans.

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa shekarun = 18
  
  idan shekarun == 18 {
    rubuta "Kana da shekara 18 daidai!"
  }
ƙare
```
</details>

---

## 5️⃣ Différence (!=)

Teste si deux valeurs sont **différentes**.

### Exemple

```hausa
fara
  naɗa lambar = 7
  
  idan lambar != 10 {
    rubuta "Lambar ba 10 ba ce"
  }
ƙare
```

**Sortie** :
```
Lambar ba 10 ba ce
```

### Usage Pratique

```hausa
fara
  naɗa sunan = "Fatima"
  
  idan sunan != "Ahmad" {
    rubuta "Ba Ahmad ba ne"
  }
ƙare
```

**Sortie** :
```
Ba Ahmad ba ne
```

---

## 6️⃣ Inférieur (<) et Supérieur (>)

### Inférieur à (<)

```hausa
fara
  naɗa daraja = 65
  
  idan daraja < 70 {
    rubuta "Daraja ba ta kai 70 ba"
  }
ƙare
```

**Sortie** :
```
Daraja ba ta kai 70 ba
```

### Supérieur à (>)

```hausa
fara
  naɗa shekarun = 25
  
  idan shekarun > 18 {
    rubuta "Ka wuce shekara 18"
  }
ƙare
```

**Sortie** :
```
Ka wuce shekara 18
```

### Exercice 2

Créez un programme qui vérifie si un nombre est :
- Inférieur à 0 : "Nombre négatif"
- Supérieur à 0 : "Nombre positif"

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa lambar = -5
  
  idan lambar < 0 {
    rubuta "Lambar mara kyau (negative)"
  }
  
  idan lambar > 0 {
    rubuta "Lambar mai kyau (positive)"
  }
ƙare
```
</details>

---

## 7️⃣ Inférieur ou Égal (<=) et Supérieur ou Égal (>=)

### Inférieur ou Égal (<=)

```hausa
fara
  naɗa daraja = 70
  
  idan daraja <= 70 {
    rubuta "Daraja ƙasa ko daidai da 70"
  }
ƙare
```

**Sortie** :
```
Daraja ƙasa ko daidai da 70
```

### Supérieur ou Égal (>=)

```hausa
fara
  naɗa daraja = 90
  
  idan daraja >= 90 {
    rubuta "Excellent! A grade!"
  }
ƙare
```

**Sortie** :
```
Excellent! A grade!
```

---

## 8️⃣ La Structure If-Else (idan-amma)

Exécute un code **OU** un autre, jamais les deux.

### Syntaxe

```hausa
idan condition {
  # Si vrai
} amma {
  # Si faux
}
```

### Exemple

```hausa
fara
  naɗa lambar = 15
  
  idan lambar > 10 {
    rubuta "Lambar ta fi 10"
  } amma {
    rubuta "Lambar ba ta kai 10 ba"
  }
ƙare
```

**Sortie** :
```
Lambar ta fi 10
```

### Exemple avec Notes

```hausa
fara
  naɗa daraja = 55
  
  idan daraja >= 60 {
    rubuta "Ka ci jarrabawa!"
  } amma {
    rubuta "Ka kasa. Sake ƙoƙari."
  }
ƙare
```

**Sortie** :
```
Ka kasa. Sake ƙoƙari.
```

### Exercice 3

Créez un programme qui vérifie si un nombre est pair ou impair.
(Astuce : utilisez la division par 2)

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa lambar = 7
  naɗa rabo = lambar / 2
  naɗa sake_ninka = rabo * 2
  
  idan sake_ninka == lambar {
    rubuta "Lambar biyu-biyu (pair)"
  } amma {
    rubuta "Lambar ba biyu-biyu ba (impair)"
  }
ƙare
```
</details>

---

## 9️⃣ La Structure If-ElseIf-Else (idan-ammaina-amma)

Permet de tester **plusieurs conditions** successivement.

### Syntaxe

```hausa
idan condition1 {
  # Si condition1 vraie
} ammaina condition2 {
  # Sinon, si condition2 vraie
} ammaina condition3 {
  # Sinon, si condition3 vraie
} amma {
  # Sinon (aucune vraie)
}
```

### Exemple : Système de Notes

```hausa
fara
  naɗa daraja = 85
  
  idan daraja >= 90 {
    rubuta "Grade: A (Excellent!)"
  } ammaina daraja >= 80 {
    rubuta "Grade: B (Sosai!)"
  } ammaina daraja >= 70 {
    rubuta "Grade: C (Ba shi da kyau)"
  } ammaina daraja >= 60 {
    rubuta "Grade: D (Raunana)"
  } amma {
    rubuta "Grade: F (Ka kasa)"
  }
ƙare
```

**Sortie** :
```
Grade: B (Sosai!)
```

### 🔍 Comment ça Marche ?

1. Teste `daraja >= 90` → `85 >= 90` → `karya` (faux) → Continue
2. Teste `daraja >= 80` → `85 >= 80` → `gaskiya` (vrai) → **Exécute et ARRÊTE**
3. Ne teste pas les conditions suivantes

### Exemple : Classification d'Âge

```hausa
fara
  naɗa shekarun = 35
  
  idan shekarun < 13 {
    rubuta "Yaro/Yarinya (Enfant)"
  } ammaina shekarun < 20 {
    rubuta "Matashi (Adolescent)"
  } ammaina shekarun < 60 {
    rubuta "Babba (Adulte)"
  } amma {
    rubuta "Tsoho (Ancien)"
  }
ƙare
```

**Sortie** :
```
Babba (Adulte)
```

### Exercice 4

Créez un programme de classification de température :
- < 0 : "Très froid"
- 0-15 : "Froid"
- 16-25 : "Agréable"
- 26-35 : "Chaud"
- > 35 : "Très chaud"

<details>
<summary>💡 Voir la solution</summary>

```hausa
fara
  naɗa zafin_iska = 28
  
  rubuta "Zafin iska: "
  rubuta zafin_iska
  rubuta "Matsayi:"
  
  idan zafin_iska < 0 {
    rubuta "Sanyi sosai"
  } ammaina zafin_iska <= 15 {
    rubuta "Sanyi"
  } ammaina zafin_iska <= 25 {
    rubuta "Lafiya"
  } ammaina zafin_iska <= 35 {
    rubuta "Zafi"
  } amma {
    rubuta "Zafi sosai"
  }
ƙare
```

**Sortie** :
```
Matsayi: Zafi
```
</details>

---

## 🔟 Conditions Imbriquées

Vous pouvez mettre des conditions **à l'intérieur** d'autres conditions.

### Exemple

```hausa
fara
  naɗa shekarun = 20
  naɗa an_yi_rajista = gaskiya
  
  idan shekarun >= 18 {
    rubuta "Kana da isasshen shekara"
    
    idan an_yi_rajista == gaskiya {
      rubuta "Kuna iya zabe!"
    } amma {
      rubuta "Dole ka yi rajista"
    }
  } amma {
    rubuta "Ba ka da isasshen shekara"
  }
ƙare
```

**Sortie** :
```
Kana da isasshen shekara
Kuna iya zabe!
```

### 🔍 Décomposition

1. Vérifie si `shekarun >= 18` → Vrai
2. Affiche "Kana da isasshen shekara"
3. **Ensuite** vérifie `an_yi_rajista == gaskiya` → Vrai
4. Affiche "Kuna iya zabe!"

---

## 1️⃣1️⃣ Programme Complet : Système d'Admission

```hausa
# Programme : Vérification d'admission universitaire
# Critères : Note >= 70 ET Âge >= 17

fara
  # Données de l'étudiant
  naɗa sunan = "Aisha Mohammed"
  naɗa daraja = 85
  naɗa shekarun = 18
  
  # Affichage des informations
  rubuta "===================================="
  rubuta "   NEMAN SHIGAR DA JAMI'A         "
  rubuta "===================================="
  rubuta "Suna: " + sunan
  rubuta "Daraja: "
  rubuta daraja
  rubuta "Shekara: "
  rubuta shekarun
  rubuta "------------------------------------"
  
  # Vérification des critères
  idan daraja >= 70 {
    idan shekarun >= 17 {
      rubuta "SAKAMAKO: AN KARƁA!"
      rubuta "Congratulations! Ka cancanci shigarwa."
    } amma {
      rubuta "SAKAMAKO: AN ƘI"
      rubuta "Dalili: Ba ka da isasshen shekara"
    }
  } amma {
    rubuta "SAKAMAKO: AN ƘI"
    rubuta "Dalili: Daraja ba ta isa ba"
  }
  
  rubuta "===================================="
ƙare
```

**Sortie** :
```
====================================
   NEMAN SHIGAR DA JAMI'A         
====================================
Suna: Aisha Mohammed
Daraja: 85
Shekara: 18
------------------------------------
SAKAMAKO: AN KARƁA!
Congratulations! Ka cancanci shigarwa.
====================================
```

---

## 1️⃣2️⃣ Comparaisons de Chaînes

Vous pouvez aussi comparer des chaînes de caractères.

### Égalité de Chaînes

```hausa
fara
  naɗa kalmar_sirri = "dabara123"
  naɗa shigarwa = "dabara123"
  
  idan shigarwa == kalmar_sirri {
    rubuta "Kalmar sirri daidai! Barka da zuwa."
  } amma {
    rubuta "Kalmar sirri ba daidai ba!"
  }
ƙare
```

**Sortie** :
```
Kalmar sirri daidai! Barka da zuwa.
```

### Vérification d'Utilisateur

```hausa
fara
  naɗa sunan_mai_amfani = "ahmad"
  
  idan sunan_mai_amfani == "admin" {
    rubuta "Sannu Admin! Kana da cikakken iko."
  } ammaina sunan_mai_amfani == "ahmad" {
    rubuta "Sannu Ahmad! Kana da ikon mai amfani."
  } amma {
    rubuta "Sunan mai amfani ba a sani ba"
  }
ƙare
```

**Sortie** :
```
Sannu Ahmad! Kana da ikon mai amfani.
```

---

## 📝 Résumé du Tutoriel

### Mots-Clés

| Hausa | Français | Utilisation |
|-------|----------|-------------|
| `idan` | si | Condition principale |
| `amma` | sinon | Alternative |
| `ammaina` | sinon si | Condition supplémentaire |

### Opérateurs

| Opérateur | Signification | Exemple |
|-----------|---------------|---------|
| `==` | égal | `a == b` |
| `!=` | différent | `a != b` |
| `<` | inférieur | `a < b` |
| `>` | supérieur | `a > b` |
| `<=` | inférieur ou égal | `a <= b` |
| `>=` | supérieur ou égal | `a >= b` |

---

## 🎯 Points Clés à Retenir

1. ✅ `idan` teste une condition
2. ✅ `amma` s'exécute si la condition est fausse
3. ✅ `ammaina` teste une condition supplémentaire
4. ✅ Seul **un bloc** s'exécute (jamais tous)
5. ✅ Les conditions peuvent être **imbriquées**
6. ✅ On peut comparer nombres, chaînes et booléens

---

## 🚀 Projet Final : Calculateur d'IMC (Indice de Masse Corporelle)

Créez un programme `imc.ha` qui :
1. Définit le poids (en kg) et la taille (en cm)
2. Calcule l'IMC : `poids * 10000 / (taille * taille)`
3. Affiche la catégorie :
   - < 18 : "Insuffisance pondérale"
   - 18-24 : "Poids normal"
   - 25-29 : "Surpoids"
   - >= 30 : "Obésité"

<details>
<summary>💡 Voir une solution possible</summary>

```hausa
fara
  # Données
  naɗa nauyi = 70  # kg
  naɗa tsawo = 170 # cm
  
  # Calcul IMC
  naɗa imc = nauyi * 10000 / (tsawo * tsawo)
  
  # Affichage
  rubuta "==================================="
  rubuta "  ƘIDIDDIGAR IMC                 "
  rubuta "==================================="
  rubuta "Nauyi (kg): "
  rubuta nauyi
  rubuta "Tsawo (cm): "
  rubuta tsawo
  rubuta "IMC: "
  rubuta imc
  rubuta "-----------------------------------"
  rubuta "Sakamako:"
  
  # Classification
  idan imc < 18 {
    rubuta "Raunana (Insuffisance pondérale)"
  } ammaina imc < 25 {
    rubuta "Daidai (Poids normal)"
  } ammaina imc < 30 {
    rubuta "Nauyin wuce gona da iri (Surpoids)"
  } amma {
    rubuta "Kiba (Obésité)"
  }
  
  rubuta "==================================="
ƙare
```

**Sortie** :
```
IMC: 24
Sakamako: Daidai (Poids normal)
```
</details>

---

## 🎓 Quiz de Validation

1. Quelle est la différence entre `==` et `=` ?
2. Que fait `ammaina` ?
3. Peut-on avoir plusieurs `ammaina` ?
4. Que se passe-t-il si aucune condition n'est vraie ?
5. Comment vérifier si un nombre est entre 10 et 20 ?

<details>
<summary>📖 Voir les réponses</summary>

1. `==` compare, `=` assigne une valeur
2. `ammaina` teste une condition supplémentaire (else if)
3. Oui, autant qu'on veut
4. Le bloc `amma` s'exécute (s'il existe)
5. `idan lambar >= 10 { idan lambar <= 20 { ... } }`
</details>

---

## ➡️ Prochaine Étape

Maintenant que vous savez prendre des décisions, découvrons les listes et collections !

➡️ [**Tutoriel 05 - Listes et Collections**](TUTORIEL_05_LISTES.md)

---

**Ka sani yanke hukunci yanzu!** (Vous savez prendre des décisions maintenant!) ✅🎯
