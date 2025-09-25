//! Gestion des erreurs pour le langage Dabara
//! 
//! Ce module définit les types d'erreurs et fournit des messages d'erreur en haoussa
//! pour une meilleure accessibilité aux utilisateurs locaux.

use std::fmt;

/// Types d'erreurs dans Dabara
#[derive(Debug, Clone)]
pub enum Error {
    /// Erreur de tokenisation (lexer)
    LexError(String),
    /// Erreur d'analyse syntaxique (parser)
    ParseError(String),
    /// Erreur d'exécution (runtime)
    RuntimeError(String),
    /// Erreur de fichier
    FileError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::LexError(msg) => write!(f, "Kuskure na Tokenization: {}", msg),
            Error::ParseError(msg) => write!(f, "Kuskure na Syntax: {}", msg),
            Error::RuntimeError(msg) => write!(f, "Kuskure na Runtime: {}", msg),
            Error::FileError(msg) => write!(f, "Kuskure na Fayil: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Messages d'erreur en haoussa
impl Error {
    pub fn unknown_token(token: &str) -> Self {
        Error::LexError(format!("Ba a gane kalmar '{}'", token))
    }
    
    pub fn unexpected_token(expected: &str, found: &str) -> Self {
        Error::ParseError(format!("Ana tsammanin '{}', amma an samu '{}'", expected, found))
    }
    
    pub fn variable_not_found(name: &str) -> Self {
        Error::RuntimeError(format!("Babu irin wannan mai canjin '{}' da aka rubuta", name))
    }
    
    pub fn invalid_operation(op: &str, left_type: &str, right_type: &str) -> Self {
        Error::RuntimeError(format!(
            "Ba za a iya amfani da '{}' tsakanin {} da {}",
            op, left_type, right_type
        ))
    }
    
    pub fn file_not_found(filename: &str) -> Self {
        Error::FileError(format!("Ba za a iya samun fayil '{}' ba", filename))
    }
    
    pub fn unterminated_string() -> Self {
        Error::LexError("Babu ƙarshen jimlar da aka rubuta".to_string())
    }
    
    pub fn expected_statement() -> Self {
        Error::ParseError("Ana tsammanin statement".to_string())
    }
    
    pub fn expected_expression() -> Self {
        Error::ParseError("Ana tsammanin expression".to_string())
    }
}