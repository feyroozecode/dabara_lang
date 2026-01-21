//! # Dabara Programming Language v0.0.1
//! 
//! Dabara est un langage de programmation utilisant des mots-clés en haoussa,
//! conçu pour démocratiser l'accès à la programmation pour les communautés Hausa.
//! 
//! ## Modules
//! 
//! - `lexer`: Tokenisation du code source avec support Unicode Hausa
//! - `parser`: Construction de l'arbre syntaxique abstrait (AST)
//! - `interpreter`: Moteur d'exécution des programmes Dabara
//! - `error`: Gestion des erreurs avec messages en haoussa
//! - `stdlib`: Bibliothèque standard avec fonctions utilitaires

pub mod error;
pub mod lexer;
pub mod parser;
pub mod interpreter;
pub mod stdlib;

pub use error::Error;
pub use lexer::{Token, tokenize};
pub use parser::{Statement, Expression, BinaryOperator, Program, parse};
pub use interpreter::{Value, Interpreter};