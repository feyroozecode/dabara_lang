//! List utility functions for Dabara
//! 
//! Provides functions for list manipulation and analysis

use crate::interpreter::{Interpreter, Value, Function};
use crate::error::Error;

/// Register all list utility functions with the interpreter
pub fn register_list_functions(interpreter: &mut Interpreter) -> Result<(), Error> {
    register_function(interpreter, "sort", sort)?;
    register_function(interpreter, "reverse", reverse)?;
    register_function(interpreter, "filter", filter)?;
    register_function(interpreter, "map", map)?;
    register_function(interpreter, "sum", sum)?;
    
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

/// Sort list: sort(jeri) → jeri
fn sort(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("sort yana bukata daya kawai (sort requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::List(elements) => {
            let mut sorted = elements.clone();
            
            // Simple sort for mixed types - strings first, then numbers, then booleans
            sorted.sort_by(|a, b| {
                match (a, b) {
                    (Value::String(a), Value::String(b)) => a.cmp(b),
                    (Value::String(_), _) => std::cmp::Ordering::Less,
                    (_, Value::String(_)) => std::cmp::Ordering::Greater,
                    (Value::Number(a), Value::Number(b)) => a.cmp(b),
                    (Value::Number(_), _) => std::cmp::Ordering::Less,
                    (_, Value::Number(_)) => std::cmp::Ordering::Greater,
                    (Value::Float(a), Value::Float(b)) => a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal),
                    (Value::Float(_), _) => std::cmp::Ordering::Less,
                    (_, Value::Float(_)) => std::cmp::Ordering::Greater,
                    (Value::Boolean(a), Value::Boolean(b)) => a.cmp(b),
                    (Value::Boolean(_), Value::List(_)) => std::cmp::Ordering::Less,
                    (Value::List(_), Value::Boolean(_)) => std::cmp::Ordering::Greater,
                    (Value::List(_), Value::List(_)) => std::cmp::Ordering::Equal,
                }
            });
            
            Ok(Value::List(sorted))
        }
        _ => Err(Error::runtime_error("sort yana bukata jeri (sort requires a list)")),
    }
}

/// Reverse list: reverse(jeri) → jeri
fn reverse(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("reverse yana bukata daya kawai (reverse requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::List(elements) => {
            let mut reversed = elements.clone();
            reversed.reverse();
            Ok(Value::List(reversed))
        }
        _ => Err(Error::runtime_error("reverse yana bukata jeri (reverse requires a list)")),
    }
}

/// Filter list with function: filter(jeri, aiki) → jeri
fn filter(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(Error::runtime_error("filter yana bukata gwargwajen 2 (filter requires exactly 2 arguments)"));
    }
    
    let _list = match &args[0] {
        Value::List(elements) => elements,
        _ => return Err(Error::runtime_error("filter na farko yana bukata jeri (filter first argument must be a list)")),
    };
    
    // TODO: Implement function calling for filter
    // For now, return an error since we can't call user functions yet
    Err(Error::runtime_error("filter aiki ba a aiwatar ba (filter function not yet implemented)"))
}

/// Transform list elements: map(jeri, aiki) → jeri
fn map(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(Error::runtime_error("map yana bukata gwargwajen 2 (map requires exactly 2 arguments)"));
    }
    
    let _list = match &args[0] {
        Value::List(elements) => elements,
        _ => return Err(Error::runtime_error("map na farko yana bukata jeri (map first argument must be a list)")),
    };
    
    // TODO: Implement function calling for map
    // For now, return an error since we can't call user functions yet
    Err(Error::runtime_error("map aiki ba a aiwatar ba (map function not yet implemented)"))
}

/// Sum all numbers in list: sum(jeri) → lamba
fn sum(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("sum yana bukata daya kawai (sum requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::List(elements) => {
            let mut total = 0.0;
            let mut has_float = false;
            
            for element in elements {
                match element {
                    Value::Number(n) => total += *n as f64,
                    Value::Float(f) => {
                        total += f;
                        has_float = true;
                    }
                    _ => return Err(Error::runtime_error("sum duka abubuwan suna bukata lamba (sum all elements must be numbers)")),
                }
            }
            
            if has_float {
                Ok(Value::Float(total))
            } else {
                Ok(Value::Number(total as i64))
            }
        }
        _ => Err(Error::runtime_error("sum yana bukata jeri (sum requires a list)")),
    }
}
