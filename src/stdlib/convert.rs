//! Type conversion functions for Dabara
//! 
//! Provides functions to convert between different data types

use crate::interpreter::{Interpreter, Value, Function};
use crate::error::Error;

/// Register all type conversion functions with the interpreter
pub fn register_conversion_functions(interpreter: &mut Interpreter) -> Result<(), Error> {
    register_function(interpreter, "int", int)?;
    register_function(interpreter, "float", float)?;
    register_function(interpreter, "string", string)?;
    register_function(interpreter, "list", list)?;
    register_function(interpreter, "bool", bool_fn)?;
    
    Ok(())
}

/// Register a single function with the interpreter
fn register_function(interpreter: &mut Interpreter, name: &str, func: fn(&[Value]) -> Result<Value, Error>) -> Result<(), Error> {
    let function = Function {
        parameters: vec![], // Will be checked at runtime
        body: vec![], // Native implementation
    };
    
    interpreter.functions.insert(name.to_string(), function);
    
    // TODO: Store native function pointers separately
    // This is a placeholder - we need to extend the interpreter to support native functions
    
    Ok(())
}

/// Convert to integer: int(abu) → lambar
fn int(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("int yana bukata daya kawai (int requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::Float(f) => Ok(Value::Number(*f as i64)),
        Value::String(s) => {
            match s.trim().parse::<i64>() {
                Ok(n) => Ok(Value::Number(n)),
                Err(_) => Err(Error::runtime_error("ba za a iya canza jimla zuwa lambar (cannot convert string to integer)")),
            }
        }
        Value::Boolean(b) => Ok(Value::Number(if *b { 1 } else { 0 })),
        Value::List(_) => Err(Error::runtime_error("ba za a iya canza jeri zuwa lambar (cannot convert list to integer)")),
    }
}

/// Convert to float: float(abu) → lambar mai daɗewa
fn float(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("float yana bukata daya kawai (float requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Float(*n as f64)),
        Value::Float(f) => Ok(Value::Float(*f)),
        Value::String(s) => {
            match s.trim().parse::<f64>() {
                Ok(f) => Ok(Value::Float(f)),
                Err(_) => Err(Error::runtime_error("ba za a iya canza jimla zuwa float (cannot convert string to float)")),
            }
        }
        Value::Boolean(b) => Ok(Value::Float(if *b { 1.0 } else { 0.0 })),
        Value::List(_) => Err(Error::runtime_error("ba za a iya canza jeri zuwa float (cannot convert list to float)")),
    }
}

/// Convert to string: string(abu) → jimla
fn string(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("string yana bukata daya kawai (string requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::String(n.to_string())),
        Value::Float(f) => Ok(Value::String(f.to_string())),
        Value::String(s) => Ok(Value::String(s.clone())),
        Value::Boolean(b) => Ok(Value::String(if *b { "gaskiya".to_string() } else { "karya".to_string() })),
        Value::List(elements) => {
            let strings: Vec<String> = elements.iter()
                .map(|v| match v {
                    Value::Number(n) => n.to_string(),
                    Value::Float(f) => f.to_string(),
                    Value::String(s) => format!("\"{}\"", s),
                    Value::Boolean(b) => if *b { "gaskiya".to_string() } else { "karya".to_string() },
                    Value::List(_) => "[...]".to_string(),
                })
                .collect();
            Ok(Value::String(format!("[{}]", strings.join(", "))))
        }
    }
}

/// Convert to list: list(abu) → jeri
fn list(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("list yana bukata daya kawai (list requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::List(elements) => Ok(Value::List(elements.clone())),
        Value::String(s) => {
            let chars: Vec<Value> = s.chars()
                .map(|c| Value::String(c.to_string()))
                .collect();
            Ok(Value::List(chars))
        }
        Value::Number(n) => {
            // Convert number to its digits
            let digits: Vec<Value> = n.to_string()
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| Value::String(c.to_string()))
                .collect();
            Ok(Value::List(digits))
        }
        Value::Float(f) => {
            // Convert float to its characters
            let chars: Vec<Value> = f.to_string()
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
                .map(|c| Value::String(c.to_string()))
                .collect();
            Ok(Value::List(chars))
        }
        Value::Boolean(b) => Ok(Value::List(vec![Value::Boolean(*b)])),
    }
}

/// Convert to boolean: bool(abu) → gaskiya ko karya
fn bool_fn(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("bool yana bukata daya kawai (bool requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::Boolean(b) => Ok(Value::Boolean(*b)),
        Value::Number(n) => Ok(Value::Boolean(*n != 0)),
        Value::Float(f) => Ok(Value::Boolean(*f != 0.0 && !f.is_nan())),
        Value::String(s) => Ok(Value::Boolean(!s.trim().is_empty() && s.trim() != "karya")),
        Value::List(elements) => Ok(Value::Boolean(!elements.is_empty())),
    }
}
