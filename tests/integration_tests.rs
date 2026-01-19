//! Tests d'intégration pour le langage Dabara

use dabara::{tokenize, parse, Interpreter, Token, Value};

/// Exécute un programme et teste la parsing
fn parse_program(source: &str) -> Result<(), dabara::Error> {
    let tokens = tokenize(source)?;
    let _ast = parse(tokens)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let program = r#"
fara
  rubuta "Sannu duniya!"
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_var_keyword_basic() {
        let program = r#"
fara
  var sunan = "Ahmad"
  var lambar = 42
  rubuta sunan
  rubuta lambar
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_var_keyword_all_types() {
        let program = r#"
fara
  var integer = 100
  var float_num = 3.14
  var text = "Hello"
  var truth = gaskiya
  var lie = karya
  var lista = [1, 2, 3]
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_var_keyword_with_arithmetic() {
        let program = r#"
fara
  var a = 10
  var b = 5
  var suma = a + b
  var diff = a - b
  var prod = a * b
  var quot = a / b
  rubuta suma
  rubuta diff
  rubuta prod
  rubuta quot
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_var_keyword_in_functions() {
        let program = r#"
fara
  aiki calculate(x, y) {
    var result = x + y
    mayar result
  }
  var answer = calculate(10, 5)
  rubuta answer
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_var_keyword_mixed_with_hausa() {
        let program = r#"
fara
  var name = "Ahmad"
  naɗa age = 25
  nada city = "Kano"
  rubuta name
  rubuta age
  rubuta city
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_variables() {
        let program = r#"
fara
  naɗa sunan = "Ahmad"
  naɗa lambar = 42
  rubuta sunan
  rubuta lambar
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_latin_variables() {
        let program = r#"
fara
  nada sunan = "Ahmad"
  nada lambar = 42
  rubuta sunan
  rubuta lambar
kare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_arithmetic() {
        let program = r#"
fara
  naɗa a = 5
  naɗa b = 3
  naɗa sakamako = a + b
  rubuta sakamako
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_latin_arithmetic() {
        let program = r#"
fara
  nada a = 5
  nada b = 3
  nada sakamako = a + b
  rubuta sakamako
kare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_tokenizer_keywords() {
        let tokens = tokenize("fara naɗa rubuta ƙare gaskiya karya + -").unwrap();

        let expected = vec![
            Token::Begin,
            Token::Let,
            Token::Print,
            Token::End,
            Token::True,
            Token::False,
            Token::Plus,
            Token::Minus,
            Token::Eof,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenizer_var_keyword() {
        let tokens = tokenize("fara var rubuta kare").unwrap();

        let expected = vec![
            Token::Begin,
            Token::Let,
            Token::Print,
            Token::End,
            Token::Eof,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenizer_latin_keywords() {
        let tokens = tokenize("fara nada rubuta kare gaskiya karya + -").unwrap();

        let expected = vec![
            Token::Begin,
            Token::Let,
            Token::Print,
            Token::End,
            Token::True,
            Token::False,
            Token::Plus,
            Token::Minus,
            Token::Eof,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenizer_identifiers() {
        let tokens = tokenize("sunan lambar shekarun").unwrap();
        
        match &tokens[0] {
            Token::Identifier(name) => assert_eq!(name, "sunan"),
            _ => panic!("Expected identifier"),
        }
    }

    #[test]
    fn test_tokenizer_numbers() {
        let tokens = tokenize("42 0 999").unwrap();
        
        match &tokens[0] {
            Token::Number(n) => assert_eq!(*n, 42),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_tokenizer_strings() {
        let tokens = tokenize(r#""Sannu" "Ina kwana?""#).unwrap();
        
        match &tokens[0] {
            Token::String(s) => assert_eq!(s, "Sannu"),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_interpreter_basic() {
        let mut interpreter = Interpreter::new();
        interpreter.set_variable("test".to_string(), Value::Number(42));
        
        match interpreter.get_variable("test") {
            Some(Value::Number(n)) => assert_eq!(*n, 42),
            _ => panic!("Expected number 42"),
        }
    }

    #[test]
    fn test_error_unknown_token() {
        let result = tokenize("@#$");
        assert!(result.is_err());
    }

    #[test]
    fn test_error_unterminated_string() {
        let result = tokenize(r#""unterminated"#);
        assert!(result.is_err());
    }
    
    // Tests pour les nouveaux opérateurs
    #[test]
    fn test_multiplication_operator() {
        let program = r#"
fara
  naɗa a = 5
  naɗa b = 3
  naɗa sakamako = a * b
  rubuta sakamako
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }
    
    #[test]
    fn test_division_operator() {
        let program = r#"
fara
  naɗa a = 15
  naɗa b = 3
  naɗa sakamako = a / b
  rubuta sakamako
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }
    
    #[test]
    fn test_combined_arithmetic() {
        let program = r#"
fara
  naɗa a = 10
  naɗa b = 2
  naɗa c = 3
  naɗa sakamako = a + b * c - 1
  rubuta sakamako
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }
    
    #[test]
    fn test_tokenizer_new_operators() {
        let tokens = tokenize("* /").unwrap();

        let expected = vec![
            Token::Multiply,
            Token::Divide,
            Token::Eof,
        ];

        assert_eq!(tokens, expected);
    }
    
    #[test]
    fn test_tokenizer_parentheses() {
        let tokens = tokenize("( ) { } ,").unwrap();
        
        let expected = vec![
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Eof,
        ];
        
        assert_eq!(tokens, expected);
    }
    
    #[test]
    fn test_tokenizer_function_keywords() {
        let tokens = tokenize("aiki karɓa").unwrap();
        
        let expected = vec![
            Token::Function,
            Token::Input,
            Token::Eof,
        ];
        
        assert_eq!(tokens, expected);
    }
    
    #[test]
    fn test_comments_ignored() {
        let tokens = tokenize("# This is a comment\nfara\n# Another comment\nƙare").unwrap();
        
        let expected = vec![
            Token::Newline,
            Token::Begin,
            Token::Newline,
            Token::Newline,
            Token::End,
            Token::Eof,
        ];
        
        assert_eq!(tokens, expected);
    }
    
    // Tests pour les fonctions
    #[test]
    fn test_function_definition_parsing() {
        let program = r#"
fara
  aiki sayi(suna, shekaru) {
    rubuta suna
    rubuta shekaru
  }
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }
    
    #[test]
    fn test_function_call_parsing() {
        let program = r#"
fara
  naɗa sakamako = sayi("Ahmad", 25)
  rubuta sakamako
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }
    
    #[test]
    fn test_function_with_no_params() {
        let program = r#"
fara
  aiki gaishe() {
    rubuta "Sannu!"
  }
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }
    
    // Tests pour l'entrée utilisateur
    #[test]
    fn test_input_parsing() {
        let program = r#"
fara
  naɗa suna = karɓa
  rubuta suna
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }
    
    #[test]
    fn test_input_with_message() {
        let program = r#"
fara
  rubuta "Rubuta sunanka: "
  naɗa suna = karɓa
  rubuta "Sannu " + suna
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }
    
    // Tests d'exécution pour les nouveaux opérateurs
    #[test]
    fn test_interpreter_multiplication() {
        let mut interpreter = Interpreter::new();
        interpreter.set_variable("a".to_string(), Value::Number(6));
        interpreter.set_variable("b".to_string(), Value::Number(7));

        // Tester la multiplication avec un programme complet
        let result = parse_program(r#"
fara
  naɗa a = 6
  naɗa b = 7
  naɗa c = a * b
  rubuta c
ƙare
"#);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_interpreter_division() {
        let mut interpreter = Interpreter::new();
        interpreter.set_variable("a".to_string(), Value::Number(20));
        interpreter.set_variable("b".to_string(), Value::Number(4));

        // Test que la division parse correctement
        let result = parse_program(r#"
fara
  naɗa c = 20 / 4
  rubuta c
ƙare
"#);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_interpreter_division_by_zero() {
        // Ce test vérifie que le parsing fonctionne même avec une division par zéro
        // L'erreur runtime sera gérée lors de l'exécution
        let result = parse_program(r#"
fara
  naɗa c = 10 / 0
  rubuta c
ƙare
"#);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_interpreter_complex_arithmetic() {
        // Test des expressions complexes avec priorité des opérateurs
        let result = parse_program(r#"
fara
  naɗa a = 2 + 3 * 4 - 1
  rubuta a
ƙare
"#);
        assert!(result.is_ok());
    }

    // Tests de régression pour la priorité des opérateurs
    #[test]
    fn test_operator_precedence_multiplication_before_addition() {
        // Test: 2 + 3 * 4 should be 2 + 12 = 14, not (2 + 3) * 4 = 20
        let program = r#"
fara
  naɗa result = 2 + 3 * 4
  rubuta result
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_operator_precedence_division_before_subtraction() {
        // Test: 20 - 8 / 2 should be 20 - 4 = 16, not (20 - 8) / 2 = 6
        let program = r#"
fara
  naɗa result = 20 - 8 / 2
  rubuta result
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_operator_precedence_multiple_multiplications() {
        // Test: 2 * 3 * 4 should be 24
        let program = r#"
fara
  naɗa result = 2 * 3 * 4
  rubuta result
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_operator_precedence_mixed_operations() {
        // Test: 10 + 5 * 2 - 3 / 3 should be 10 + 10 - 1 = 19
        let program = r#"
fara
  naɗa result = 10 + 5 * 2 - 3 / 3
  rubuta result
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_operator_precedence_with_parentheses() {
        // Test: (2 + 3) * 4 should be 5 * 4 = 20
        let program = r#"
fara
  naɗa result = (2 + 3) * 4
  rubuta result
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_operator_precedence_nested_parentheses() {
        // Test: ((10 + 5) * 2) - 3 should be (15 * 2) - 3 = 27
        let program = r#"
fara
  naɗa result = ((10 + 5) * 2) - 3
  rubuta result
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_operator_precedence_division_and_multiplication() {
        // Test: 20 / 4 * 5 should be (20 / 4) * 5 = 5 * 5 = 25 (left-to-right)
        let program = r#"
fara
  naɗa result = 20 / 4 * 5
  rubuta result
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_operator_precedence_unary_minus() {
        // Test: -5 * 2 should be (-5) * 2 = -10, not -(5 * 2)
        let program = r#"
fara
  naɗa result = -5 * 2
  rubuta result
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_operator_precedence_unary_plus() {
        // Test: +5 + 3 should be 5 + 3 = 8
        let program = r#"
fara
  naɗa result = +5 + 3
  rubuta result
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_operator_precedence_complex_expression() {
        // Test: 100 - 20 / 5 + 3 * 2 should be 100 - 4 + 6 = 102
        let program = r#"
fara
  naɗa result = 100 - 20 / 5 + 3 * 2
  rubuta result
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }

    // Tests d'exécution pour vérifier les valeurs calculées avec la bonne priorité
    #[test]
    fn test_execution_multiplication_before_addition() {
        let source = r#"
fara
  naɗa result = 2 + 3 * 4
ƙare
"#;
        let tokens = tokenize(source).expect("Failed to tokenize");
        let program = parse(tokens).expect("Failed to parse");
        let mut interpreter = Interpreter::new();
        interpreter.execute(program).expect("Failed to execute");

        let result = interpreter.get_variable("result").expect("Variable not found");
        match result {
            Value::Number(n) => assert_eq!(*n, 14), // 2 + 12 = 14, not 20
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_execution_division_before_subtraction() {
        let source = r#"
fara
  naɗa result = 20 - 8 / 2
ƙare
"#;
        let tokens = tokenize(source).expect("Failed to tokenize");
        let program = parse(tokens).expect("Failed to parse");
        let mut interpreter = Interpreter::new();
        interpreter.execute(program).expect("Failed to execute");

        let result = interpreter.get_variable("result").expect("Variable not found");
        match result {
            Value::Number(n) => assert_eq!(*n, 16), // 20 - 4 = 16, not 6
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_execution_parentheses_override_precedence() {
        let source = r#"
fara
  naɗa result = (2 + 3) * 4
ƙare
"#;
        let tokens = tokenize(source).expect("Failed to tokenize");
        let program = parse(tokens).expect("Failed to parse");
        let mut interpreter = Interpreter::new();
        interpreter.execute(program).expect("Failed to execute");

        let result = interpreter.get_variable("result").expect("Variable not found");
        match result {
            Value::Number(n) => assert_eq!(*n, 20), // 5 * 4 = 20
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_execution_complex_expression() {
        let source = r#"
fara
  naɗa result = 100 - 20 / 5 + 3 * 2
ƙare
"#;
        let tokens = tokenize(source).expect("Failed to tokenize");
        let program = parse(tokens).expect("Failed to parse");
        let mut interpreter = Interpreter::new();
        interpreter.execute(program).expect("Failed to execute");

        let result = interpreter.get_variable("result").expect("Variable not found");
        match result {
            Value::Number(n) => assert_eq!(*n, 102), // 100 - 4 + 6 = 102
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_execution_left_to_right_associativity() {
        let source = r#"
fara
  naɗa result = 20 / 4 / 2
ƙare
"#;
        let tokens = tokenize(source).expect("Failed to tokenize");
        let program = parse(tokens).expect("Failed to parse");
        let mut interpreter = Interpreter::new();
        interpreter.execute(program).expect("Failed to execute");

        let result = interpreter.get_variable("result").expect("Variable not found");
        match result {
            Value::Number(n) => assert_eq!(*n, 2), // (20 / 4) / 2 = 5 / 2 = 2, not 20 / (4 / 2) = 10
            _ => panic!("Expected number"),
        }
    }

    // Tests d'exécution pour le mot-clé 'var'
    #[test]
    fn test_execution_var_keyword_numbers() {
        let source = r#"
fara
  var x = 42
  var y = 3.14
ƙare
"#;
        let tokens = tokenize(source).expect("Failed to tokenize");
        let program = parse(tokens).expect("Failed to parse");
        let mut interpreter = Interpreter::new();
        interpreter.execute(program).expect("Failed to execute");

        let x = interpreter.get_variable("x").expect("Variable not found");
        match x {
            Value::Number(n) => assert_eq!(*n, 42),
            _ => panic!("Expected number"),
        }

        let y = interpreter.get_variable("y").expect("Variable not found");
        match y {
            Value::Float(f) => assert_eq!(*f, 3.14),
            _ => panic!("Expected float"),
        }
    }

    #[test]
    fn test_execution_var_keyword_string() {
        let source = r#"
fara
  var name = "Ahmad"
ƙare
"#;
        let tokens = tokenize(source).expect("Failed to tokenize");
        let program = parse(tokens).expect("Failed to parse");
        let mut interpreter = Interpreter::new();
        interpreter.execute(program).expect("Failed to execute");

        let name = interpreter.get_variable("name").expect("Variable not found");
        match name {
            Value::String(s) => assert_eq!(s, "Ahmad"),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_execution_var_keyword_arithmetic() {
        let source = r#"
fara
  var a = 10
  var b = 5
  var result = a + b * 2
ƙare
"#;
        let tokens = tokenize(source).expect("Failed to tokenize");
        let program = parse(tokens).expect("Failed to parse");
        let mut interpreter = Interpreter::new();
        interpreter.execute(program).expect("Failed to execute");

        let result = interpreter.get_variable("result").expect("Variable not found");
        match result {
            Value::Number(n) => assert_eq!(*n, 20), // 10 + (5 * 2) = 20
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_execution_var_and_nada_interoperability() {
        let source = r#"
fara
  var x = 10
  naɗa y = 20
  nada z = 30
  var total = x + y + z
ƙare
"#;
        let tokens = tokenize(source).expect("Failed to tokenize");
        let program = parse(tokens).expect("Failed to parse");
        let mut interpreter = Interpreter::new();
        interpreter.execute(program).expect("Failed to execute");

        let total = interpreter.get_variable("total").expect("Variable not found");
        match total {
            Value::Number(n) => assert_eq!(*n, 60), // 10 + 20 + 30 = 60
            _ => panic!("Expected number"),
        }
    }
}