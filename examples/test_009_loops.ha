# TEST 009: Loops (maimaita and ga)
# Demonstrates while loops and for loops in Dabara

fara
    rubuta "=== While Loop (maimaita) ==="
    naɗa lambar = 1
    maimaita (lambar <= 5) {
        rubuta lambar
        naɗa lambar = lambar + 1
    }

    rubuta ""
    rubuta "=== For Loop (ga...cikin) ==="
    naɗa sunaye = ["Ahmad", "Fatima", "Musa", "Aisha"]
    ga suna cikin sunaye {
        rubuta suna
    }

    rubuta ""
    rubuta "=== Nested Loops ==="
    naɗa jimla = 0
    ga i cikin [1, 2, 3] {
        ga j cikin [10, 20] {
            naɗa jimla = jimla + i + j
        }
    }
    rubuta "Jimla (Total): "
    rubuta jimla

    rubuta ""
    rubuta "=== Countdown with While ==="
    naɗa ƙidaya = 5
    maimaita (ƙidaya > 0) {
        rubuta ƙidaya
        naɗa ƙidaya = ƙidaya - 1
    }
    rubuta "Farawa! (Start!)"
ƙare
