# TEST 012: String Operations
# Demonstrates string methods and character access

fara
    naɗa rubutu = "Sannu Duniya"

    rubuta "=== String Methods ==="
    rubuta "Original: "
    rubuta rubutu

    rubuta "Length (tsawo): "
    rubuta rubutu.tsawo()

    rubuta "Uppercase (babba): "
    rubuta rubutu.babba()

    rubuta "Lowercase (ƙarami): "
    rubuta rubutu.karami()

    rubuta "Substring (yanki 0-5): "
    rubuta rubutu.yanki(0, 5)

    rubuta "Substring (yanki 6-12): "
    rubuta rubutu.yanki(6, 12)

    rubuta ""
    rubuta "=== String Indexing ==="
    rubuta "First character [0]: "
    rubuta rubutu[0]

    rubuta "Seventh character [6]: "
    rubuta rubutu[6]

    rubuta "Last character [-1]: "
    rubuta rubutu[-1]

    rubuta ""
    rubuta "=== String Split ==="
    naɗa kalmomi = rubutu.raba(" ")
    rubuta "Split by space: "
    rubuta kalmomi

    rubuta "Iterating over words:"
    ga kalma cikin kalmomi {
        rubuta kalma
    }

    rubuta ""
    rubuta "=== Working with Hausa Text ==="
    naɗa hausa = "Yaya kake? Lafiya lau!"
    rubuta hausa
    rubuta "Length: "
    rubuta hausa.tsawo()

    naɗa parts = hausa.raba("? ")
    rubuta "Split sentences:"
    ga part cikin parts {
        rubuta part
    }
ƙare
