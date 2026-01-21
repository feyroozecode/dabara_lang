//! Mathematical functions for Dabara
//! 
//! Provides basic arithmetic, trigonometry, and mathematical constants

use crate::interpreter::{Interpreter, Value, Function};
use crate::error::Error;

/// Register all mathematical functions with the interpreter
pub fn register_math_functions(interpreter: &mut Interpreter) -> Result<(), Error> {
    // Basic arithmetic functions
    register_function(interpreter, "abs", abs)?;
    register_function(interpreter, "sqrt", sqrt)?;
    register_function(interpreter, "pow", pow)?;
    register_function(interpreter, "min", min)?;
    register_function(interpreter, "max", max)?;
    
    // Rounding functions
    register_function(interpreter, "floor", floor)?;
    register_function(interpreter, "ceil", ceil)?;
    register_function(interpreter, "round", round)?;
    
    // Trigonometric functions
    register_function(interpreter, "sin", sin)?;
    register_function(interpreter, "cos", cos)?;
    register_function(interpreter, "tan", tan)?;
    register_function(interpreter, "asin", asin)?;
    register_function(interpreter, "acos", acos)?;
    register_function(interpreter, "atan", atan)?;
    
    // Mathematical constants
    interpreter.set_variable_value("DABARAN_PI".to_string(), Value::Float(std::f64::consts::PI));
    interpreter.set_variable_value("DABARAN_E".to_string(), Value::Float(std::f64::consts::E));
    
    Ok(())
}

/// Register a single function with the interpreter
fn register_function(interpreter: &mut Interpreter, name: &str, func: fn(&[Value]) -> Result<Value, Error>) -> Result<(), Error> {
    let function = Function {
        parameters: vec![], // Will be checked at runtime
        body: vec![], // Native implementation
    };
    
    // Store the function in a special native functions map
    // For now, we'll use the regular function registry
    interpreter.functions.insert(name.to_string(), function);
    
    // TODO: Store native function pointers separately
    // This is a placeholder - we need to extend the interpreter to support native functions
    
    Ok(())
}

// Mathematical function implementations

/// Absolute value: abs(lamba) → lamba
fn abs(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("abs yana bukata daya kawai (abs requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        Value::Float(f) => Ok(Value::Float(f.abs())),
        _ => Err(Error::runtime_error("abs yana bukata lamba (abs requires a number)")),
    }
}

/// Square root: sqrt(lamba) → lamba
fn sqrt(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("sqrt yana bukata daya kawai (sqrt requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::Number(n) => {
            if *n < 0 {
                return Err(Error::runtime_error("sqrt ba ya iya aiki da lamba negatif (sqrt cannot work with negative numbers)"));
            }
            Ok(Value::Float((*n as f64).sqrt()))
        }
        Value::Float(f) => {
            if *f < 0.0 {
                return Err(Error::runtime_error("sqrt ba ya iya aiki da lamba negatif (sqrt cannot work with negative numbers)"));
            }
            Ok(Value::Float(f.sqrt()))
        }
        _ => Err(Error::runtime_error("sqrt yana bukata lamba (sqrt requires a number)")),
    }
}

/// Power: pow(base, exp) → lamba
fn pow(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(Error::runtime_error("pow yana bukata gwargwajen 2 (pow requires exactly 2 arguments)"));
    }
    
    let base = match &args[0] {
        Value::Number(n) => *n as f64,
        Value::Float(f) => *f,
        _ => return Err(Error::runtime_error("pow na farko yana bukata lamba (pow first argument requires a number)")),
    };
    
    let exp = match &args[1] {
        Value::Number(n) => *n as f64,
        Value::Float(f) => *f,
        _ => return Err(Error::runtime_error("pow na biyu yana bukata lamba (pow second argument requires a number)")),
    };
    
    Ok(Value::Float(base.powf(exp)))
}

/// Minimum: min(a, b, ...) → lamba
fn min(args: &[Value]) -> Result<Value, Error> {
    if args.is_empty() {
        return Err(Error::runtime_error("min yana bukata aƙalla daya (min requires at least one argument)"));
    }
    
    let mut min_val = match &args[0] {
        Value::Number(n) => *n as f64,
        Value::Float(f) => *f,
        _ => return Err(Error::runtime_error("min duka sunayen suna bukata lamba (min all arguments must be numbers)")),
    };
    
    for arg in args.iter().skip(1) {
        let val = match arg {
            Value::Number(n) => *n as f64,
            Value::Float(f) => *f,
            _ => return Err(Error::runtime_error("min duka sunayen suna bukata lamba (min all arguments must be numbers)")),
        };
        
        if val < min_val {
            min_val = val;
        }
    }
    
    // Return as Number if it's an integer value
    if min_val.fract() == 0.0 && min_val >= i64::MIN as f64 && min_val <= i64::MAX as f64 {
        Ok(Value::Number(min_val as i64))
    } else {
        Ok(Value::Float(min_val))
    }
}

/// Maximum: max(a, b, ...) → lamba
fn max(args: &[Value]) -> Result<Value, Error> {
    if args.is_empty() {
        return Err(Error::runtime_error("max yana bukata aƙalla daya (max requires at least one argument)"));
    }
    
    let mut max_val = match &args[0] {
        Value::Number(n) => *n as f64,
        Value::Float(f) => *f,
        _ => return Err(Error::runtime_error("max duka sunayen suna bukata lamba (max all arguments must be numbers)")),
    };
    
    for arg in args.iter().skip(1) {
        let val = match arg {
            Value::Number(n) => *n as f64,
            Value::Float(f) => *f,
            _ => return Err(Error::runtime_error("max duka sunayen suna bukata lamba (max all arguments must be numbers)")),
        };
        
        if val > max_val {
            max_val = val;
        }
    }
    
    // Return as Number if it's an integer value
    if max_val.fract() == 0.0 && max_val >= i64::MIN as f64 && max_val <= i64::MAX as f64 {
        Ok(Value::Number(max_val as i64))
    } else {
        Ok(Value::Float(max_val))
    }
}

/// Floor: floor(lamba) → lamba
fn floor(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("floor yana bukata daya kawai (floor requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::Float(f) => Ok(Value::Number(f.floor() as i64)),
        _ => Err(Error::runtime_error("floor yana bukata lamba (floor requires a number)")),
    }
}

/// Ceiling: ceil(lamba) → lamba
fn ceil(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("ceil yana bukata daya kawai (ceil requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::Float(f) => Ok(Value::Number(f.ceil() as i64)),
        _ => Err(Error::runtime_error("ceil yana bukata lamba (ceil requires a number)")),
    }
}

/// Round: round(lamba) → lamba
fn round(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("round yana bukata daya kawai (round requires exactly one argument)"));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::Float(f) => Ok(Value::Number(f.round() as i64)),
        _ => Err(Error::runtime_error("round yana bukata lamba (round requires a number)")),
    }
}

/// Sine: sin(kwana) → lamba
fn sin(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("sin yana bukata daya kawai (sin requires exactly one argument)"));
    }
    
    let radians = match &args[0] {
        Value::Number(n) => *n as f64,
        Value::Float(f) => *f,
        _ => return Err(Error::runtime_error("sin yana bukata lamba (sin requires a number)")),
    };
    
    Ok(Value::Float(radians.sin()))
}

/// Cosine: cos(kwana) → lamba
fn cos(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("cos yana bukata daya kawai (cos requires exactly one argument)"));
    }
    
    let radians = match &args[0] {
        Value::Number(n) => *n as f64,
        Value::Float(f) => *f,
        _ => return Err(Error::runtime_error("cos yana bukata lamba (cos requires a number)")),
    };
    
    Ok(Value::Float(radians.cos()))
}

/// Tangent: tan(kwana) → lamba
fn tan(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("tan yana bukata daya kawai (tan requires exactly one argument)"));
    }
    
    let radians = match &args[0] {
        Value::Number(n) => *n as f64,
        Value::Float(f) => *f,
        _ => return Err(Error::runtime_error("tan yana bukata lamba (tan requires a number)")),
    };
    
    Ok(Value::Float(radians.tan()))
}

/// Arcsine: asin(x) → lamba
fn asin(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("asin yana bukata daya kawai (asin requires exactly one argument)"));
    }
    
    let value = match &args[0] {
        Value::Number(n) => *n as f64,
        Value::Float(f) => *f,
        _ => return Err(Error::runtime_error("asin yana bukata lamba (asin requires a number)")),
    };
    
    if value < -1.0 || value > 1.0 {
        return Err(Error::runtime_error("asin yana bukata tsakanin -1 da 1 (asin requires value between -1 and 1)"));
    }
    
    Ok(Value::Float(value.asin()))
}

/// Arccosine: acos(x) → lamba
fn acos(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("acos yana bukata daya kawai (acos requires exactly one argument)"));
    }
    
    let value = match &args[0] {
        Value::Number(n) => *n as f64,
        Value::Float(f) => *f,
        _ => return Err(Error::runtime_error("acos yana bukata lamba (acos requires a number)")),
    };
    
    if value < -1.0 || value > 1.0 {
        return Err(Error::runtime_error("acos yana bukata tsakanin -1 da 1 (acos requires value between -1 and 1)"));
    }
    
    Ok(Value::Float(value.acos()))
}

/// Arctangent: atan(x) → lamba
fn atan(args: &[Value]) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(Error::runtime_error("atan yana bukata daya kawai (atan requires exactly one argument)"));
    }
    
    let value = match &args[0] {
        Value::Number(n) => *n as f64,
        Value::Float(f) => *f,
        _ => return Err(Error::runtime_error("atan yana bukata lamba (atan requires a number)")),
    };
    
    Ok(Value::Float(value.atan()))
}
