//! Parser pour le langage Dabara
//! 
//! Ce module implémente l'analyseur syntaxique qui convertit les tokens
//! en arbre syntaxique abstrait (AST).

use crate::lexer::Token;
use crate::error::Error;

/// Représente un programme Dabara complet
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Types de statements dans Dabara
#[derive(Debug, Clone)]
pub enum Statement {
    /// Déclaration de variable: naɗa nom = expression
    Let { name: String, value: Expression },
    /// Instruction d'affichage: rubuta expression
    Print(Expression),
}

/// Types d'expressions dans Dabara
#[derive(Debug, Clone)]
pub enum Expression {
    /// Identificateur de variable
    Identifier(String),
    /// Nombre entier
    Number(i64),
    /// Chaîne de caractères
    String(String),
    /// Valeur booléenne
    Boolean(bool),
    /// Opération binaire
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
}

/// Opérateurs binaires supportés
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    /// Addition (ƙara)
    Add,
    /// Soustraction (rage)
    Subtract,
    /// Concaténation (+)
    Concat,
}

/// Parser pour construire l'AST
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    current_token: Token,
}

impl Parser {
    /// Crée un nouveau parser
    pub fn new(mut tokens: Vec<Token>) -> Result<Self, Error> {
        // Enlever les newlines inutiles au début et à la fin
        let original_len = tokens.len();
        tokens.retain(|token| *token != Token::Newline || original_len <= 2);
        
        if tokens.is_empty() {
            tokens.push(Token::Eof);
        }
        
        let current_token = tokens[0].clone();
        
        Ok(Parser {
            tokens,
            position: 0,
            current_token,
        })
    }
    
    /// Avance au token suivant
    fn advance(&mut self) -> Result<(), Error> {
        self.position += 1;
        
        if self.position < self.tokens.len() {
            self.current_token = self.tokens[self.position].clone();
        } else {
            self.current_token = Token::Eof;
        }
        
        Ok(())
    }
    
    /// Vérifie si le token actuel correspond au type attendu
    fn expect_token(&mut self, expected: Token) -> Result<(), Error> {
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(&expected) {
            self.advance()
        } else {
            Err(Error::unexpected_token(
                &format!("{:?}", expected),
                &format!("{:?}", self.current_token)
            ))
        }
    }
    
    /// Parse un programme complet
    pub fn parse_program(&mut self) -> Result<Program, Error> {
        // Skip les newlines au début
        while self.current_token == Token::Newline {
            self.advance()?;
        }
        
        // Attendre 'fara'
        self.expect_token(Token::Begin)?;
        
        // Skip les newlines après 'fara'
        while self.current_token == Token::Newline {
            self.advance()?;
        }
        
        let mut statements = Vec::new();
        
        // Parse les statements jusqu'à 'ƙare'
        while self.current_token != Token::End && self.current_token != Token::Eof {
            if self.current_token == Token::Newline {
                self.advance()?;
                continue;
            }
            
            let statement = self.parse_statement()?;
            statements.push(statement);
            
            // Skip les newlines après le statement
            while self.current_token == Token::Newline {
                self.advance()?;
            }
        }
        
        // Attendre 'ƙare'
        self.expect_token(Token::End)?;
        
        Ok(Program { statements })
    }
    
    /// Parse un statement
    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match &self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Print => self.parse_print_statement(),
            _ => Err(Error::expected_statement()),
        }
    }
    
    /// Parse une déclaration de variable: naɗa nom = expression
    fn parse_let_statement(&mut self) -> Result<Statement, Error> {
        self.advance()?; // Consommer 'naɗa'
        
        let name = match &self.current_token {
            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance()?;
                var_name
            }
            _ => return Err(Error::unexpected_token("identifier", &format!("{:?}", self.current_token))),
        };
        
        self.expect_token(Token::Equals)?;
        
        let value = self.parse_expression()?;
        
        Ok(Statement::Let { name, value })
    }
    
    /// Parse une instruction d'affichage: rubuta expression
    fn parse_print_statement(&mut self) -> Result<Statement, Error> {
        self.advance()?; // Consommer 'rubuta'
        
        let expression = self.parse_expression()?;
        
        Ok(Statement::Print(expression))
    }
    
    /// Parse une expression
    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_additive_expression()
    }
    
    /// Parse une expression additive (gère + et -)
    fn parse_additive_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_primary_expression()?;
        
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let operator = match self.current_token {
                Token::Plus => {
                    self.advance()?;
                    // Déterminer s'il s'agit d'addition ou de concaténation
                    // sera déterminé lors de l'évaluation selon les types
                    BinaryOperator::Add
                }
                Token::Minus => {
                    self.advance()?;
                    BinaryOperator::Subtract
                }
                _ => unreachable!(),
            };
            
            let right = self.parse_primary_expression()?;
            
            left = Expression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    /// Parse une expression primaire (littéraux, identificateurs)
    fn parse_primary_expression(&mut self) -> Result<Expression, Error> {
        match &self.current_token.clone() {
            Token::Number(n) => {
                let value = *n;
                self.advance()?;
                Ok(Expression::Number(value))
            }
            
            Token::String(s) => {
                let value = s.clone();
                self.advance()?;
                Ok(Expression::String(value))
            }
            
            Token::True => {
                self.advance()?;
                Ok(Expression::Boolean(true))
            }
            
            Token::False => {
                self.advance()?;
                Ok(Expression::Boolean(false))
            }
            
            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance()?;
                Ok(Expression::Identifier(var_name))
            }
            
            _ => Err(Error::expected_expression()),
        }
    }
}

/// Fonction utilitaire pour parser des tokens
pub fn parse(tokens: Vec<Token>) -> Result<Program, Error> {
    let mut parser = Parser::new(tokens)?;
    parser.parse_program()
}