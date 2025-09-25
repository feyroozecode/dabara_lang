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
}