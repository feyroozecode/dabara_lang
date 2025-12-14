# TEST 013: Floating Point Numbers
# Demonstrates float arithmetic and mixed operations

fara
    rubuta "=== Float Literals ==="
    naɗa pi = 3.14159
    naɗa e = 2.71828
    rubuta "Pi: "
    rubuta pi
    rubuta "E: "
    rubuta e

    rubuta ""
    rubuta "=== Float Arithmetic ==="
    naɗa jimla = pi + e
    rubuta "Pi + E = "
    rubuta jimla

    naɗa bambanci = pi - e
    rubuta "Pi - E = "
    rubuta bambanci

    naɗa ninkawa = pi * 2.0
    rubuta "Pi * 2 = "
    rubuta ninkawa

    naɗa rabo = pi / 2.0
    rubuta "Pi / 2 = "
    rubuta rabo

    rubuta ""
    rubuta "=== Mixed Int/Float Operations ==="
    naɗa lambar = 10
    naɗa float_num = 3.5

    rubuta "10 + 3.5 = "
    rubuta lambar + float_num

    rubuta "10 * 3.5 = "
    rubuta lambar * float_num

    rubuta "10 / 3.5 = "
    rubuta lambar / float_num

    rubuta ""
    rubuta "=== Float Comparisons ==="
    naɗa a = 5.5
    naɗa b = 3.2

    maimaita (a > b) {
        rubuta "5.5 is greater than 3.2"
        naɗa a = 0.0
    }

    rubuta ""
    rubuta "=== Practical Example: Circle Area ==="
    naɗa radius = 5.0
    naɗa area = 3.14159 * radius * radius
    rubuta "Radius: "
    rubuta radius
    rubuta "Area: "
    rubuta area
ƙare
