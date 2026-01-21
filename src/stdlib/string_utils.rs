//! String utility functions for Dabara
//! 
//! Provides functions for string manipulation and analysis

use crate::interpreter::{Interpreter, Value, Function};
use crate::error::Error;

/// Register all string utility functions with the interpreter
pub fn register_string_functions(interpreter: &mut Interpreter) -> Result<(), Error> {
    register_function(interpreter, "contains", contains)?;
    register_function(interpreter, "starts_with", starts_with)?;
    register_function(interpreter, "ends_with", ends_with)?;
    register_function(interpreter, "replace", replace)?;
    register_function(interpreter, "trim", trim)?;
    
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

/// Check if string contains substring: contains(rubutu, substring) → gaskiya ko karya
fn contains(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(Error::runtime_error("contains yana bukata gwargwajen 2 (contains requires exactly 2 arguments)"));
    }
    
    let text = match &args[0] {
        Value::String(s) => s,
        _ => return Err(Error::runtime_error("contains na farko yana bukata jimla (contains first argument must be a string)")),
    };
    
    let substring = match &args[1] {
        Value::String(s) => s,
        _ => return Err(Error::runtime_error("contains na biyu yana bukata jimla (contains second argument must be a string)")),
    };
    
    Ok(Value::Boolean(text.contains(substring)))
}

/// Check if string starts with prefix: starts_with(rubutu, awali) → gaskiya ko karya
fn starts_with(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(Error::runtime_error("starts_with yana bukata gwargwajen 2 (starts_with requires exactly 2 arguments)"));
    }
    
    let text = match &args[0] {
        Value::String(s) => s,
        _ => return Err(Error::runtime_error("starts_with na farko yana bukata jimla (starts_with first argument must be a string)")),
    };
    
    let prefix = match &args[1] {
        Value::String(s) => s,
        _ => return Err(Error::runtime_error("starts_with na biyu yana bukata jimla (starts_with second argument must be a string)")),
    };
    
    Ok(Value::Boolean(text.starts_with(prefix)))
}

/// Check if string ends with suffix: ends_with(rubutu, karshe) → gaskiya ko karya
fn ends_with(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(Error::runtime_error("ends_with yana bukata gwargwajen 2 (ends_with requires exactly 2 arguments)"));
    }
    
    let text = match &args[0] {
        Value::String(s) => s,
        _ => return Err(Error::runtime_error("ends_with na farko yana bukata jimla (ends_with first argument must be a string)")),
    };
    
    let suffix = match &args[1] {
        Value::String(s) => s,
        _ => return Err(Error::runtime_error("ends_with na biyu yana bukata jimla (ends_with second argument must be a string)")),
    };
    
    Ok(Value::Boolean(text.ends_with(suffix)))
}

/// Replace text in string: replace(rubutu, tsoho, sabo) → jimla
fn replace(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 3 {
        return Err(Error::runtime_error("replace yana bukata gwargwajen 3 (replace requires exactly 3 arguments)"));
    }
    
    let text = match &args[0] {
        Value::String(s) => s,
        _ => return Err(Error::runtime_error("replace na farko yana bukata jimla (replace first argument must be a string)")),
    };
    
    let old = match &args[1] {
        Value::String(s) => s,
        _ => return Err(Error::runtime_error("replace na biyu yana bukata jimla (replace second argument must be a string)")),
    };
    
    let new = match &args[2] {
        Value::String(s) => s,
        _ => return Err(Error::runtime_error("replace na uku yana bukata jimla (replace third argument must be a string)")),
    };
    
    Ok(Value::String(text.replace(old, new)))
}

/// Remove whitespace from string: trim(rubutu) → jimla
fn trim(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("trim yana bukata daya kawai (trim requires exactly one argument)"));
    }
    
    let text = match &args[0] {
        Value::String(s) => s,
        _ => return Err(Error::runtime_error("trim yana bukata jimla (trim requires a string)")),
    };
    
    Ok(Value::String(text.trim().to_string()))
}
