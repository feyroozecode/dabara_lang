# TEST 014: Using 'var' keyword (v0.4.0+)
# Demonstrates the new 'var' keyword for variable declaration
# 'var' is now the primary keyword (naɗa/nada are deprecated)

fara
    rubuta "=== Test: var Keyword ==="

    # Basic variable declaration with 'var'
    var name = "Ahmad"
    var age = 25
    var height = 1.75

    rubuta "Name: "
    rubuta name
    rubuta "Age: "
    rubuta age
    rubuta "Height: "
    rubuta height

    # Boolean values
    var is_student = gaskiya
    var is_teacher = karya

    rubuta "Is student: "
    rubuta is_student
    rubuta "Is teacher: "
    rubuta is_teacher

    # Lists
    var numbers = [1, 2, 3, 4, 5]
    rubuta "Numbers: "
    rubuta numbers

    # Arithmetic with 'var'
    var a = 10
    var b = 5
    var sum = a + b
    var product = a * b

    rubuta "Sum of 10 + 5: "
    rubuta sum
    rubuta "Product of 10 * 5: "
    rubuta product

    # String concatenation
    var greeting = "Sannu " + name
    rubuta greeting

    # Backward compatibility test
    nada x = 100
    nada y = 200
    var z = x + y

    rubuta "Backward compatibility (nada): "
    rubuta z

    rubuta "=== Test Complete! ==="
ƙare
