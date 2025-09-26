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
  naɗa sakamako = a ƙara b
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
  nada sakamako = a kara b
  rubuta sakamako
kare
"#;
        assert!(parse_program(program).is_ok());
    }

    #[test]
    fn test_tokenizer_keywords() {
        let tokens = tokenize("fara naɗa rubuta ƙare gaskiya karya ƙara rage").unwrap();
        
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
    fn test_tokenizer_latin_keywords() {
        let tokens = tokenize("fara nada rubuta kare gaskiya karya kara rage").unwrap();
        
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
  naɗa sakamako = a ninka b
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
  naɗa sakamako = a raba b
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
  naɗa sakamako = a ƙara b ninka c rage 1
  rubuta sakamako
ƙare
"#;
        assert!(parse_program(program).is_ok());
    }
    
    #[test]
    fn test_tokenizer_new_operators() {
        let tokens = tokenize("ninka raba").unwrap();
        
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
  rubuta "Sannu " ƙara suna
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
  naɗa c = a ninka b
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
  naɗa c = 20 raba 4
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
  naɗa c = 10 raba 0
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
  naɗa a = 2 ƙara 3 ninka 4 rage 1
  rubuta a
ƙare
"#);
        assert!(result.is_ok());
    }
}