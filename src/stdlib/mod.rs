//! Standard Library for Dabara Programming Language
//! 
//! This module contains all built-in functions and utilities
//! that are available to Dabara programs by default.

pub mod math;
pub mod convert;
pub mod string_utils;
pub mod list_utils;

use crate::interpreter::Interpreter;
use crate::error::Error;

/// Register all standard library functions with the interpreter
pub fn register_stdlib(interpreter: &mut Interpreter) -> Result<(), Error> {
    // Register math functions
    math::register_math_functions(interpreter)?;
    
    // Register type conversion functions
    convert::register_conversion_functions(interpreter)?;
    
    // Register string utilities
    string_utils::register_string_functions(interpreter)?;
    
    // Register list utilities
    list_utils::register_list_functions(interpreter)?;
    
    Ok(())
}
