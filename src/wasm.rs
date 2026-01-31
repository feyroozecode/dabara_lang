//! WASM bindings for Dabara Programming Language
//!
//! This module provides WebAssembly bindings to run Dabara code in browsers.

use wasm_bindgen::prelude::*;
use crate::{tokenize, parse, Interpreter};

/// Result type for WASM operations
pub type WasmResult = Result<String, JsValue>;

/// WASM-compatible runtime for Dabara
///
/// This struct wraps the interpreter and provides JavaScript-accessible methods
/// for running Dabara code in web browsers.
#[wasm_bindgen]
pub struct DabaraRuntime {
    interpreter: Interpreter,
    output_buffer: String,
}

#[wasm_bindgen]
impl DabaraRuntime {
    /// Creates a new Dabara runtime instance
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const runtime = new DabaraRuntime();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> DabaraRuntime {
        // Set up panic hook for better error messages in browser console
        #[cfg(feature = "wasm")]
        console_error_panic_hook::set_once();

        DabaraRuntime {
            interpreter: Interpreter::new(),
            output_buffer: String::new(),
        }
    }

    /// Runs Dabara source code and returns the output
    ///
    /// # Arguments
    /// * `source` - The Dabara source code to execute
    ///
    /// # Returns
    /// * `Ok(String)` - The captured output from the program
    /// * `Err(JsValue)` - Error message if execution fails
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const runtime = new DabaraRuntime();
    /// try {
    ///     const output = runtime.run_code('fara\n  rubuta "Sannu!"\nkare');
    ///     console.log(output); // "Sannu!"
    /// } catch (e) {
    ///     console.error(e);
    /// }
    /// ```
    #[wasm_bindgen]
    pub fn run_code(&mut self, source: &str) -> WasmResult {
        self.output_buffer.clear();

        // Tokenize
        let tokens = match tokenize(source) {
            Ok(t) => t,
            Err(e) => return Err(JsValue::from_str(&e.to_string())),
        };

        // Parse
        let program = match parse(tokens) {
            Ok(p) => p,
            Err(e) => return Err(JsValue::from_str(&e.to_string())),
        };

        // Execute with output capture
        match self.interpreter.execute_with_output(program, &mut self.output_buffer) {
            Ok(_) => Ok(self.output_buffer.clone()),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    /// Tokenizes Dabara source code and returns tokens as JSON
    ///
    /// # Arguments
    /// * `source` - The Dabara source code to tokenize
    ///
    /// # Returns
    /// * `Ok(JsValue)` - Array of token objects
    /// * `Err(JsValue)` - Error message if tokenization fails
    #[wasm_bindgen]
    pub fn tokenize(&self, source: &str) -> Result<JsValue, JsValue> {
        match tokenize(source) {
            Ok(tokens) => {
                serde_wasm_bindgen::to_value(&tokens)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            }
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    /// Parses Dabara source code and returns AST as JSON
    ///
    /// # Arguments
    /// * `source` - The Dabara source code to parse
    ///
    /// # Returns
    /// * `Ok(JsValue)` - AST representation as JSON
    /// * `Err(JsValue)` - Error message if parsing fails
    #[wasm_bindgen]
    pub fn parse(&self, source: &str) -> Result<JsValue, JsValue> {
        let tokens = match tokenize(source) {
            Ok(t) => t,
            Err(e) => return Err(JsValue::from_str(&e.to_string())),
        };

        match parse(tokens) {
            Ok(program) => {
                serde_wasm_bindgen::to_value(&program)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            }
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    /// Clears all variables and functions from the interpreter
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.interpreter.clear_variables();
        self.output_buffer.clear();
    }

    /// Gets the value of a variable
    ///
    /// # Arguments
    /// * `name` - The variable name
    ///
    /// # Returns
    /// * `Some(String)` - The variable value as string
    /// * `None` - If variable doesn't exist
    #[wasm_bindgen]
    pub fn get_variable(&self, name: &str) -> Option<String> {
        self.interpreter.get_variable(name).map(|v| v.to_string())
    }
}

impl Default for DabaraRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility function to run Dabara code directly
///
/// This is a convenience function that creates a temporary runtime
/// and executes the provided code.
///
/// # Arguments
/// * `source` - The Dabara source code to execute
///
/// # Returns
/// * `Ok(String)` - The captured output
/// * `Err(JsValue)` - Error message if execution fails
#[wasm_bindgen]
pub fn run_dabara(source: &str) -> WasmResult {
    let mut runtime = DabaraRuntime::new();
    runtime.run_code(source)
}

/// Returns the version of the Dabara interpreter
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
