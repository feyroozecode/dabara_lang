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
    /// Définition de fonction: aiki nom(paramètres) { corps }
    FunctionDef {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    /// Condition if/else/elseif: idan condition { ... } amma { ... }
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    /// Statement de retour: mayar expression
    Return(Expression),
    /// Statement d'expression (comme appel de fonction standalone)
    Expression(Expression),
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
    /// Liste de valeurs: [element1, element2, ...]
    List(Vec<Expression>),
    /// Opération binaire
    BinaryOp {                                   
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    /// Appel de fonction   /// e.g => ayki gaydaMutane (["Musa", "Issa"])
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    /// Appel de mètode (style Ruby en haoussa)  /// e.g => yin mutane("Musa")
    MethodCall {
        receiver: Box<Expression>,
        method: String,
        arguments: Vec<Expression>,
    },
    /// Entrée utilisateur
    Input,
}

/// Opérateurs binaires supportés
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    /// Addition (+)
    Add,
    /// Soustraction (-)
    Subtract,
    /// Multiplication (*)
    Multiply,
    /// Division (/)
    Divide,
    /// Concaténation (+)
    Concat,
    /// Égalité (==)
    Equal,
    /// Inégalité (!=)
    NotEqual,
    /// Inférieur (<)
    Less,
    /// Supérieur (>)
    Greater,
    /// Inférieur ou égal (<=)
    LessEqual,
    /// Supérieur ou égal (>=)
    GreaterEqual,
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
    
    /// Vérifie si c'est un appel de fonction sans parenthèses
    /// (le token suivant est un argument potentiel)
    fn is_function_call_without_parens(&self) -> bool {
        matches!(self.current_token, 
            Token::Number(_) | 
            Token::String(_) | 
            Token::Identifier(_) | 
            Token::True | 
            Token::False | 
            Token::Input |
            Token::LeftBracket
        )
    }
    
    /// Vérifie si on est à la fin d'une expression
    fn is_end_of_expression(&self) -> bool {
        matches!(self.current_token,
            Token::Newline |
            Token::Eof |
            Token::RightParen |
            Token::RightBrace |
            Token::RightBracket |
            Token::End |
            Token::Plus |
            Token::Minus |
            Token::Multiply |
            Token::Divide |
            Token::Dot |
            Token::Equal |
            Token::NotEqual |
            Token::Less |
            Token::Greater |
            Token::LessEqual |
            Token::GreaterEqual |
            Token::Else |
            Token::ElseIf
        )
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
            Token::Function => self.parse_function_definition(),
            Token::If => self.parse_if_statement(),
            Token::Return => self.parse_return_statement(),
            // Si c'est un identificateur, cela peut être un appel de fonction
            Token::Identifier(_) => {
                let expression = self.parse_expression()?;
                Ok(Statement::Expression(expression))
            }
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
    
    /// Parse une définition de fonction: aiki nom(paramètres) { corps }
    fn parse_function_definition(&mut self) -> Result<Statement, Error> {
        self.advance()?; // Consommer 'aiki'
        
        let name = match &self.current_token {
            Token::Identifier(name) => {
                let func_name = name.clone();
                self.advance()?;
                func_name
            }
            _ => return Err(Error::unexpected_token("identifier", &format!("{:?}", self.current_token))),
        };
        
        self.expect_token(Token::LeftParen)?;
        
        let mut parameters = Vec::new();
        
        // Parse les paramètres
        while self.current_token != Token::RightParen {
            if let Token::Identifier(param) = &self.current_token {
                parameters.push(param.clone());
                self.advance()?;
                
                if self.current_token == Token::Comma {
                    self.advance()?;
                }
            } else {
                return Err(Error::unexpected_token("parameter", &format!("{:?}", self.current_token)));
            }
        }
        
        self.expect_token(Token::RightParen)?;
        self.expect_token(Token::LeftBrace)?;
        
        // Skip les newlines après {
        while self.current_token == Token::Newline {
            self.advance()?;
        }
        
        let mut body = Vec::new();
        
        // Parse le corps de la fonction
        while self.current_token != Token::RightBrace && self.current_token != Token::Eof {
            if self.current_token == Token::Newline {
                self.advance()?;
                continue;
            }
            
            let statement = self.parse_statement()?;
            body.push(statement);
            
            // Skip les newlines après le statement
            while self.current_token == Token::Newline {
                self.advance()?;
            }
        }
        
        self.expect_token(Token::RightBrace)?;
        
        Ok(Statement::FunctionDef { name, parameters, body })
    }
    
    /// Parse une condition if/else/elseif: idan condition { ... } amma { ... }
    fn parse_if_statement(&mut self) -> Result<Statement, Error> {
        self.advance()?; // Consommer 'idan'
        
        let condition = self.parse_expression()?;
        
        self.expect_token(Token::LeftBrace)?;
        
        // Skip les newlines après {
        while self.current_token == Token::Newline {
            self.advance()?;
        }
        
        let mut then_branch = Vec::new();
        
        // Parse le corps du if
        while self.current_token != Token::RightBrace && self.current_token != Token::Eof {
            if self.current_token == Token::Newline {
                self.advance()?;
                continue;
            }
            
            let statement = self.parse_statement()?;
            then_branch.push(statement);
            
            // Skip les newlines après le statement
            while self.current_token == Token::Newline {
                self.advance()?;
            }
        }
        
        self.expect_token(Token::RightBrace)?;
        
        // Vérifier s'il y a une clause else ou elseif
        let else_branch = if self.current_token == Token::Else {
            self.advance()?; // Consommer 'amma'
            
            self.expect_token(Token::LeftBrace)?;
            
            // Skip les newlines après {
            while self.current_token == Token::Newline {
                self.advance()?;
            }
            
            let mut else_statements = Vec::new();
            
            // Parse le corps du else
            while self.current_token != Token::RightBrace && self.current_token != Token::Eof {
                if self.current_token == Token::Newline {
                    self.advance()?;
                    continue;
                }
                
                let statement = self.parse_statement()?;
                else_statements.push(statement);
                
                // Skip les newlines après le statement
                while self.current_token == Token::Newline {
                    self.advance()?;
                }
            }
            
            self.expect_token(Token::RightBrace)?;
            
            // Créer un If statement fictif avec condition true pour exécuter else_statements
            Some(Box::new(Statement::If {
                condition: Expression::Boolean(true),
                then_branch: else_statements,
                else_branch: None,
            }))
        } else if self.current_token == Token::ElseIf {
            // Parse elseif comme un if imbriqué
            let elseif_statement = self.parse_if_statement()?;
            Some(Box::new(elseif_statement))
        } else {
            None
        };
        
        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    
    /// Parse un statement de retour: mayar expression
    fn parse_return_statement(&mut self) -> Result<Statement, Error> {
        self.advance()?; // Consommer 'mayar'
        
        let expression = self.parse_expression()?;
        
        Ok(Statement::Return(expression))
    }
    
    /// Parse une expression
    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_comparison_expression()
    }
    
    /// Parse une expression de comparaison (==, !=, <, >, <=, >=)
    fn parse_comparison_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_additive_expression()?;
        
        while matches!(self.current_token, 
            Token::Equal | Token::NotEqual | Token::Less | 
            Token::Greater | Token::LessEqual | Token::GreaterEqual
        ) {
            let operator = match self.current_token {
                Token::Equal => {
                    self.advance()?;
                    BinaryOperator::Equal
                }
                Token::NotEqual => {
                    self.advance()?;
                    BinaryOperator::NotEqual
                }
                Token::Less => {
                    self.advance()?;
                    BinaryOperator::Less
                }
                Token::Greater => {
                    self.advance()?;
                    BinaryOperator::Greater
                }
                Token::LessEqual => {
                    self.advance()?;
                    BinaryOperator::LessEqual
                }
                Token::GreaterEqual => {
                    self.advance()?;
                    BinaryOperator::GreaterEqual
                }
                _ => unreachable!(),
            };
            
            let right = self.parse_additive_expression()?;
            
            left = Expression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    /// Parse une expression additive (gère +, -, *, /)
    fn parse_additive_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_multiplicative_expression()?;
        
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let operator = match self.current_token {
                Token::Plus => {
                    self.advance()?;
                    BinaryOperator::Add
                }
                Token::Minus => {
                    self.advance()?;
                    BinaryOperator::Subtract
                }
                _ => unreachable!(),
            };
            
            let right = self.parse_multiplicative_expression()?;
            
            left = Expression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    /// Parse une expression multiplicative (gère * et /)
    fn parse_multiplicative_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_primary_expression()?;
        
        while matches!(self.current_token, Token::Multiply | Token::Divide) {
            let operator = match self.current_token {
                Token::Multiply => {
                    self.advance()?;
                    BinaryOperator::Multiply
                }
                Token::Divide => {
                    self.advance()?;
                    BinaryOperator::Divide
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
    
    /// Parse une expression primaire (littéraux, identificateurs, appels de fonction)
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
            
            Token::Input => {
                self.advance()?;
                Ok(Expression::Input)
            }
            
            Token::LeftBracket => {
                self.parse_list_expression()
            }
            
            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance()?;
                
                // Vérifier s'il s'agit d'un appel de fonction
                // Syntaxe avec parenthèses : fonction(arg1, arg2)
                if self.current_token == Token::LeftParen {
                    self.advance()?; // Consommer '('
                    
                    let mut arguments = Vec::new();
                    
                    // Parse les arguments
                    while self.current_token != Token::RightParen {
                        let arg = self.parse_comparison_expression()?;
                        arguments.push(arg);
                        
                        if self.current_token == Token::Comma {
                            self.advance()?;
                        }
                    }
                    
                    self.expect_token(Token::RightParen)?;
                    
                    Ok(Expression::FunctionCall {
                        name: var_name,
                        arguments,
                    })
                }
                // Syntaxe sans parenthèses : fonction arg1 arg2 (style Ruby/Python)
                else if self.is_function_call_without_parens() {
                    let mut arguments = Vec::new();
                    
                    // Parse les arguments jusqu'à un délimiteur ou fin de ligne
                    while !self.is_end_of_expression() {
                        let arg = self.parse_primary_expression()?;
                        arguments.push(arg);
                        
                        // Les arguments sont séparés par des espaces ou virgules
                        if self.current_token == Token::Comma {
                            self.advance()?;
                        }
                    }
                    
                    Ok(Expression::FunctionCall {
                        name: var_name,
                        arguments,
                    })
                } else {
                    Ok(Expression::Identifier(var_name))
                }
            }
            
            _ => Err(Error::expected_expression()),
        }
    }
    
    /// Parse une liste: [element1, element2, ...]
    fn parse_list_expression(&mut self) -> Result<Expression, Error> {
        self.advance()?; // Consommer '['
        
        let mut elements = Vec::new();
        
        // Parse les éléments de la liste
        while self.current_token != Token::RightBracket && self.current_token != Token::Eof {
            let element = self.parse_expression()?;
            elements.push(element);
            
            if self.current_token == Token::Comma {
                self.advance()?;
            } else if self.current_token != Token::RightBracket {
                return Err(Error::unexpected_token(
                    ", ou ]", 
                    &format!("{:?}", self.current_token)
                ));
            }
        }
        
        self.expect_token(Token::RightBracket)?;
        
        Ok(Expression::List(elements))
    }
}

/// Fonction utilitaire pour parser des tokens
pub fn parse(tokens: Vec<Token>) -> Result<Program, Error> {
    let mut parser = Parser::new(tokens)?; 
    parser.parse_program()
}