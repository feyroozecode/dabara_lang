# TEST 011: List Operations
# Demonstrates list indexing and methods

fara
    naɗa jeri = [10, 20, 30, 40, 50]

    rubuta "=== List Indexing ==="
    rubuta "Original list: "
    rubuta jeri

    rubuta "First element (jeri[0]): "
    rubuta jeri[0]

    rubuta "Third element (jeri[2]): "
    rubuta jeri[2]

    rubuta "Last element (jeri[-1]): "
    rubuta jeri[-1]

    rubuta "Second from last (jeri[-2]): "
    rubuta jeri[-2]

    rubuta ""
    rubuta "=== List Methods ==="

    rubuta "Length (tsawo): "
    rubuta jeri.tsawo()

    rubuta "Adding element with ƙara: "
    naɗa sababbi = jeri.kara(60)
    rubuta sababbi

    rubuta "Pop element with cire: "
    naɗa lambar = sababbi.cire()
    rubuta "Popped: "
    rubuta lambar

    rubuta ""
    rubuta "=== List Join ==="
    naɗa kalmomi = ["Sannu", "Duniya", "daga", "Dabara"]
    rubuta "Words list: "
    rubuta kalmomi

    rubuta "Joined with spaces: "
    naɗa jimla = kalmomi.hada(" ")
    rubuta jimla

    rubuta "Joined with hyphens: "
    rubuta kalmomi.hada("-")
ƙare
