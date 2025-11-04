# Changelog - Dabara Programming Language

## [0.2.0] - YYYY-MM-DD

### Added
- Implemented Hausa arithmetic operators in lexer:
  - `ƙara` for addition (+)
  - `rage` for subtraction (-)
  - `ninka` for multiplication (*)
  - `raba` for division (/)
- Added Latin alternatives for arithmetic operators:
  - `kara` as alternative for `ƙara`
- Added comprehensive roadmap documentation
- Added function call execution support (in progress)

### Fixed
- Fixed all failing integration tests
- Resolved duplicate pattern issue in lexer
- Fixed syntax error in Token::from_keyword function

### Changed
- Updated documentation to include roadmap
- Improved error messages for arithmetic operations
- Enhanced test coverage for new operators

### Deprecated
- None

### Removed
- None

### Security
- None

## [0.1.3] - 2024-10-14

### Added
- Initial release with basic language features
- Hausa syntax keywords
- Variable declaration and assignment
- Basic arithmetic operations
- String concatenation
- Boolean values
- Conditional statements
- Function definitions
- User input handling
- List support
- Unicode Hausa character support
- Localized error messages in Hausa