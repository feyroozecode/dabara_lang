//! Tests unitaires pour l'exécution de fonctions

use dabara::{lexer::tokenize, parser::parse, interpreter::Interpreter};

#[test]
fn test_simple_function_call() {
    let source = r#"
fara
  aiki gaya() {
    rubuta "Sannu!"
  }
  gaya()
ƙare
"#;
    
    let tokens = tokenize(source).expect("Failed to tokenize");
    let program = parse(tokens).expect("Failed to parse");
    let mut interpreter = Interpreter::new();
    
    // Should not panic
    interpreter.execute(program).expect("Failed to execute");
}

#[test]
fn test_function_with_return() {
    let source = r#"
fara
  aiki jimla(a, b) {
    mayar a + b
  }
  naɗa sakamakon = jimla(5, 3)
  rubuta sakamakon
ƙare
"#;
    
    let tokens = tokenize(source).expect("Failed to tokenize");
    let program = parse(tokens).expect("Failed to parse");
    let mut interpreter = Interpreter::new();
    
    interpreter.execute(program).expect("Failed to execute");
    
    // Verify the result was stored
    let result = interpreter.get_variable("sakamakon").expect("Variable not found");
    match result {
        dabara::interpreter::Value::Number(n) => assert_eq!(*n, 8),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_function_with_parameters() {
    let source = r#"
fara
  aiki darabawa(a, b) {
    mayar a * b
  }
  naɗa sakamakon = darabawa(4, 7)
ƙare
"#;
    
    let tokens = tokenize(source).expect("Failed to tokenize");
    let program = parse(tokens).expect("Failed to parse");
    let mut interpreter = Interpreter::new();
    
    interpreter.execute(program).expect("Failed to execute");
    
    let result = interpreter.get_variable("sakamakon").expect("Variable not found");
    match result {
        dabara::interpreter::Value::Number(n) => assert_eq!(*n, 28),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_local_scope_isolation() {
    let source = r#"
fara
  naɗa x = 10
  aiki canja() {
    naɗa x = 20
    mayar x
  }
  naɗa sakamakon = canja()
  rubuta x
ƙare
"#;
    
    let tokens = tokenize(source).expect("Failed to tokenize");
    let program = parse(tokens).expect("Failed to parse");
    let mut interpreter = Interpreter::new();
    
    interpreter.execute(program).expect("Failed to execute");
    
    // x should still be 10 in global scope
    let x = interpreter.get_variable("x").expect("Variable not found");
    match x {
        dabara::interpreter::Value::Number(n) => assert_eq!(*n, 10),
        _ => panic!("Expected number"),
    }
    
    // sakamakon should be 20 (returned from function)
    let result = interpreter.get_variable("sakamakon").expect("Variable not found");
    match result {
        dabara::interpreter::Value::Number(n) => assert_eq!(*n, 20),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_recursive_function() {
    let source = r#"
fara
  aiki factorial(n) {
    idan n == 0 {
      mayar 1
    } amma {
      mayar n * factorial(n - 1)
    }
  }
  naɗa sakamakon = factorial(5)
ƙare
"#;
    
    let tokens = tokenize(source).expect("Failed to tokenize");
    let program = parse(tokens).expect("Failed to parse");
    let mut interpreter = Interpreter::new();
    
    interpreter.execute(program).expect("Failed to execute");
    
    let result = interpreter.get_variable("sakamakon").expect("Variable not found");
    match result {
        dabara::interpreter::Value::Number(n) => assert_eq!(*n, 120), // 5! = 120
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_function_calling_function() {
    let source = r#"
fara
  aiki double(x) {
    mayar x * 2
  }
  aiki quadruple(x) {
    mayar double(double(x))
  }
  naɗa sakamakon = quadruple(3)
ƙare
"#;
    
    let tokens = tokenize(source).expect("Failed to tokenize");
    let program = parse(tokens).expect("Failed to parse");
    let mut interpreter = Interpreter::new();
    
    interpreter.execute(program).expect("Failed to execute");
    
    let result = interpreter.get_variable("sakamakon").expect("Variable not found");
    match result {
        dabara::interpreter::Value::Number(n) => assert_eq!(*n, 12), // 3 * 4 = 12
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_early_return_in_conditional() {
    let source = r#"
fara
  aiki max(a, b) {
    idan a > b {
      mayar a
    } amma {
      mayar b
    }
  }
  naɗa sakamakon = max(10, 5)
ƙare
"#;
    
    let tokens = tokenize(source).expect("Failed to tokenize");
    let program = parse(tokens).expect("Failed to parse");
    let mut interpreter = Interpreter::new();
    
    interpreter.execute(program).expect("Failed to execute");
    
    let result = interpreter.get_variable("sakamakon").expect("Variable not found");
    match result {
        dabara::interpreter::Value::Number(n) => assert_eq!(*n, 10),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_function_with_string_params() {
    let source = r#"
fara
  aiki haɗa(a, b) {
    mayar a + b
  }
  naɗa sakamakon = haɗa("Sannu ", "Duniya")
ƙare
"#;
    
    let tokens = tokenize(source).expect("Failed to tokenize");
    let program = parse(tokens).expect("Failed to parse");
    let mut interpreter = Interpreter::new();
    
    interpreter.execute(program).expect("Failed to execute");
    
    let result = interpreter.get_variable("sakamakon").expect("Variable not found");
    match result {
        dabara::interpreter::Value::String(s) => assert_eq!(s, "Sannu Duniya"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_function_without_return() {
    let source = r#"
fara
  aiki gaya() {
    naɗa x = 42
  }
  naɗa sakamakon = gaya()
ƙare
"#;
    
    let tokens = tokenize(source).expect("Failed to tokenize");
    let program = parse(tokens).expect("Failed to parse");
    let mut interpreter = Interpreter::new();
    
    interpreter.execute(program).expect("Failed to execute");
    
    // Function without explicit return should return 0 by default
    let result = interpreter.get_variable("sakamakon").expect("Variable not found");
    match result {
        dabara::interpreter::Value::Number(n) => assert_eq!(*n, 0),
        _ => panic!("Expected number"),
    }
}
