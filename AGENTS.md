# AGENTS.md

This file contains guidelines and commands for agentic coding agents working on the Dabara codebase.

## Build, Test, and Lint Commands

### Build
- `cargo build` - Debug build
- `cargo build --release` - Optimized release build
- `cargo install --path .` - Install locally

### Testing
- `cargo test --verbose` - Run all tests with output
- `cargo test <test_name>` - Run single test by name
- `cargo test -- --nocapture` - Run tests with printed output
- `cargo test --test integration_tests` - Run integration tests only
- `./test_examples.sh` - Test all example programs

### Linting and Formatting
- `cargo fmt -- --check` - Check formatting without modifying files
- `cargo fmt` - Format all Rust files
- `cargo clippy -- -D warnings` - Run linter with warnings as errors

### Running Programs
- `cargo run examples/<file.ha>` - Run example program
- `export DABARA_DEBUG=1 && dabara program.ha` - Debug mode (shows tokens, AST)

## Code Style Guidelines

### Language and Edition
- Rust 2021 edition
- UTF-8 encoding (required for Hausa Unicode characters)

### Imports
- Use `use crate::module::item;` for internal modules
- Group external dependencies together at top
- Keep imports alphabetized within groups

### Formatting
- Use `rustfmt` (enforced by CI)
- 4-space indentation
- Maximum line length: 100 characters

### Naming Conventions
- **Functions/variables**: `snake_case`
- **Types/structs/enums**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case` (files: module_name.rs)

### Types
- Prefer `i64` for integers, `f64` for floats
- Use `String` for owned strings, `&str` for borrowed
- Use `Vec<T>` for lists, `HashMap<K, V>` for maps
- Use `Option<T>` for optional values
- Use `Result<T, Error>` for fallible operations

### Error Handling
- Custom `Error` enum in `src/error.rs` with variants: `LexError`, `ParseError`, `RuntimeError`, `FileError`
- All error messages in Hausa (for user-facing errors)
- Use `?` operator for error propagation
- Return `Result<T, Error>` from fallible functions
- Use constructor methods on Error enum: `Error::unknown_token()`, `Error::variable_not_found()`, etc.

### Code Organization
- Module structure: `src/lexer.rs`, `src/parser.rs`, `src/interpreter.rs`, `src/error.rs`
- Re-export public API in `src/lib.rs`
- Tests in `tests/integration_tests.rs` and inline `#[cfg(test)]` modules

### Documentation
- Use `//!` for module-level documentation
- Use `///` for public items
- Document in French with Hausa examples
- Include usage examples in doc comments

### Unicode Support
- Full support for Hausa characters: ƙ, ɗ, ɓ, ƴ, ʔ
- Accept both Unicode and Latin variants (e.g., `ƙare`/`kare`, `naɗa`/`nada`)
- Use char comparison for Unicode-aware lexing

### Enums and Structs
- Derive `Debug` and `Clone` for AST types
- Use pub fields for simple data types
- Implement `Display` for user-facing types

### Testing Patterns
- Write unit tests in `#[cfg(test)]` modules within source files
- Write integration tests in `tests/` directory
- Test both Unicode and Latin variants of keywords
- Include error path testing

### Pattern Matching
- Use match expressions for Token parsing
- Handle all variants explicitly with `_` for unused
- Prefer pattern matching over if-else for enums

### Performance
- Use references to avoid unnecessary clones
- Iterators with `.collect()` preferred over manual loops
- Borrowing over ownership where possible

### AST Node Patterns
- All AST nodes derive `Debug` and `Clone`
- Use `Box<Expression>` for recursive expression types
- Use `Vec<Statement>` for statement blocks
- Struct variants use named fields for clarity

### Parser Implementation
- Recursive descent parsing pattern
- One lookahead token (peek) for decisions
- Match expressions on current token type
- Return `Result<..., Error>` from parse methods
- Consume tokens after matching with `self.advance()`

### Interpreter Implementation
- Variable scopes stored in `Vec<HashMap<String, Value>>`
- Use `HashMap<String, Function>` for function definitions
- Evaluate expressions recursively
- Implement `Display` trait for `Value` enum
- Use `format!` for string interpolation

### Lexer Implementation
- Iterate through `Vec<char>` for Unicode support
- Check `peek()` before consuming multi-char tokens
- Separate methods for reading numbers, strings, identifiers
- Return `Result<Token, Error>` from `next_token()`
- Skip whitespace and comments in loop

## Project Context

Dabara is a programming language with Hausa syntax designed for accessibility. Key priorities:
1. Maintain Unicode Hausa character support (ƙ, ɗ, ɓ, ƴ, ʔ)
2. Keep error messages in Hausa for user-facing errors
3. Accept both Unicode and Latin keyword variants (e.g., `ƙare`/`kare`)
4. Document features in French with Hausa examples
5. Add tests for all new features (both parsing and execution)
6. Test both Unicode and Latin keyword variants
