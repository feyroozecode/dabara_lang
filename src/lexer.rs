//! Lexer pour le langage Dabara
//! 
//! Ce module implémente la tokenisation du code source Dabara,
//! avec support complet des caractères Unicode haoussa.

use crate::error::Error;

#[cfg(feature = "serde")]
use serde::Serialize;

/// Types de tokens dans Dabara
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Token {
    // Mots-clés Hausa
    Begin,      // fara
    End,        // ƙare
    Print,      // rubuta
    Let,        // var (primary), naɗa/nada (deprecated)
    True,       // gaskiya
    False,      // karya
    Function,   // aiki
    Input,      // karɓa
    Return,     // mayar
    
    // Conditions en hausa naturel
    If,         // idan
    Else,       // amma
    ElseIf,     // ammaina

    // Boucles (loops)
    While,      // maimaita
    For,        // ga
    In,         // cikin
    Break,      // katse
    Continue,   // ci_gaba

    // Comparaisons
    Equal,      // == (daidai)
    NotEqual,   // != (ba daidai ba)
    Less,       // < (ƙasa)
    Greater,    // > (sama)
    LessEqual,  // <= (ƙasa ko daidai)
    GreaterEqual, // >= (sama ko daidai)
    
    // Méthodes inspirées de Ruby (en haoussa)
    Dot,        // . (pour appel de méthode)
    
    // Délimiteurs pour fonctions et listes
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    LeftBracket, // [ (pour listes)
    RightBracket, // ] (pour listes)
    Comma,      // ,
    
    // Littéraux
    Identifier(String),
    Number(i64),
    Float(f64),
    String(String),
    
    // Opérateurs standard
    Plus,       // + (ƙara / kara)
    Minus,      // - (rage)
    Multiply,   // * (ninka)
    Divide,     // / (raba)
    Equals,     // =
    
    // Délimiteurs
    Newline,
    Eof,
}

impl Token {
    /// Convertit un mot en token si c'est un mot-clé
    fn from_keyword(word: &str) -> Option<Token> {
        match word {
            // Primary universal keyword
            "var" => Some(Token::Let),       // Primary keyword for variable declaration

            // Versions originales avec caractères haoussa (deprecated but supported)
            "fara" => Some(Token::Begin),
            "ƙare" => Some(Token::End),
            "rubuta" => Some(Token::Print),
            "naɗa" => Some(Token::Let),      // Deprecated: use 'var' instead
            "gaskiya" => Some(Token::True),
            "karya" => Some(Token::False),
            "aiki" => Some(Token::Function),
            "karɓa" => Some(Token::Input),
            "mayar" => Some(Token::Return),

            // Note: Removed Hausa operator keywords (ƙara, rage, ninka, raba)
            // to avoid conflicts with method names. Use symbols: +, -, *, /

            // Conditions en hausa naturel
            "idan" => Some(Token::If),
            "amma" => Some(Token::Else),
            "ammaina" => Some(Token::ElseIf),

            // Boucles (loops)
            "maimaita" => Some(Token::While),
            "ga" => Some(Token::For),
            "cikin" => Some(Token::In),
            "katse" => Some(Token::Break),
            "ci_gaba" => Some(Token::Continue),

            // Versions alternatives avec caractères latins (deprecated but supported)
            "kare" => Some(Token::End),      // Alternative pour ƙare
            "nada" => Some(Token::Let),      // Deprecated: use 'var' instead
            // Note: "kara" removed from keywords to allow it as method name

            _ => None,
        }
    }
}

/// Lexer pour tokeniser le code source Dabara
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,  
}

impl Lexer {
    /// Crée un nouveau lexer
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }
    
    /// Avance au caractère suivant
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }
    
    /// Regarde le caractère suivant sans avancer
    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }
    
    /// Ignore les espaces (mais pas les newlines)
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    /// Ignore les commentaires (lignes commençant par #)
    fn skip_comment(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }
    
    /// Lit un nombre (entier ou flottant)
    fn read_number(&mut self) -> Token {
        let mut number_str = String::new();
        let mut is_float = false;

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                number_str.push(ch);
                self.advance();
            } else if ch == '.' && self.peek().map_or(false, |c| c.is_ascii_digit()) {
                // C'est un point décimal suivi d'un chiffre
                is_float = true;
                number_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            Token::Float(number_str.parse().unwrap_or(0.0))
        } else {
            Token::Number(number_str.parse().unwrap_or(0))
        }
    }
    
    /// Lit une chaîne de caractères
    fn read_string(&mut self) -> Result<String, Error> {
        let mut string_val = String::new();
        self.advance(); // Skip opening quote
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // Skip closing quote
                return Ok(string_val);
            }
            string_val.push(ch);
            self.advance();
        }
        
        Err(Error::unterminated_string())
    }
    
    /// Lit un identificateur ou mot-clé
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphabetic() || ch == '_' || ch.is_ascii_digit() || self.is_hausa_char(ch) {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        identifier
    }
    
    /// Vérifie si un caractère est un caractère spécial haoussa
    fn is_hausa_char(&self, ch: char) -> bool {
        match ch {
            'ɓ' | 'ɗ' | 'ƙ' | 'ƴ' | 'ʔ' => true,
            _ => false,
        }
    }
    
    /// Lit le prochain token
    pub fn next_token(&mut self) -> Result<Token, Error> {
        loop {
            match self.current_char {
                None => return Ok(Token::Eof),
                
                Some(' ') | Some('\t') | Some('\r') => {
                    self.skip_whitespace();
                }
                
                Some('\n') => {
                    self.advance();
                    return Ok(Token::Newline);
                }
                
                Some('=') if self.peek() == Some('=') => {
                    self.advance(); // Consommer le premier '='
                    self.advance(); // Consommer le second '='
                    return Ok(Token::Equal);
                }
                
                Some('=') => {
                    self.advance();
                    return Ok(Token::Equals);
                }
                
                Some('+') => {
                    self.advance();
                    return Ok(Token::Plus);
                }
                
                Some('-') => {
                    self.advance();
                    return Ok(Token::Minus);
                }
                
                Some('*') => {
                    self.advance();
                    return Ok(Token::Multiply);
                }
                
                Some('/') => {
                    self.advance();
                    return Ok(Token::Divide);
                }
                
                Some('#') => {
                    self.skip_comment();
                    continue; // Ignorer et continuer
                }
                
                Some('(') => {
                    self.advance();
                    return Ok(Token::LeftParen);
                }
                
                Some(')') => {
                    self.advance();
                    return Ok(Token::RightParen);
                }
                
                Some('{') => {
                    self.advance();
                    return Ok(Token::LeftBrace);
                }
                
                Some('}') => {
                    self.advance();
                    return Ok(Token::RightBrace);
                }
                
                Some(',') => {
                    self.advance();
                    return Ok(Token::Comma);
                }
                
                Some('[') => {
                    self.advance();
                    return Ok(Token::LeftBracket);
                }
                
                Some(']') => {
                    self.advance();
                    return Ok(Token::RightBracket);
                }
                
                Some('!') if self.peek() == Some('=') => {
                    self.advance(); // Consommer '!'
                    self.advance(); // Consommer '='
                    return Ok(Token::NotEqual);
                }
                
                Some('<') if self.peek() == Some('=') => {
                    self.advance(); // Consommer '<'
                    self.advance(); // Consommer '='
                    return Ok(Token::LessEqual);
                }
                
                Some('<') => {
                    self.advance();
                    return Ok(Token::Less);
                }
                
                Some('>') if self.peek() == Some('=') => {
                    self.advance(); // Consommer '>'
                    self.advance(); // Consommer '='
                    return Ok(Token::GreaterEqual);
                }
                
                Some('>') => {
                    self.advance();
                    return Ok(Token::Greater);
                }
                
                Some('"') => {
                    let string_val = self.read_string()?;
                    return Ok(Token::String(string_val));
                }
                
                Some('.') if !self.peek().map_or(false, |c| c.is_ascii_digit()) => {
                    // C'est un point de méthode, pas un nombre décimal
                    self.advance();
                    return Ok(Token::Dot);
                }

                Some(ch) if ch.is_ascii_digit() || (ch == '.' && self.peek().map_or(false, |c| c.is_ascii_digit())) => {
                    return Ok(self.read_number());
                }

                Some(ch) if ch.is_alphabetic() || self.is_hausa_char(ch) => {
                    let identifier = self.read_identifier();
                    
                    // Vérifier si c'est un mot-clé
                    if let Some(token) = Token::from_keyword(&identifier) {
                        return Ok(token);
                    } else {
                        return Ok(Token::Identifier(identifier));
                    }
                }
                
                Some(ch) => {
                    return Err(Error::unknown_token(&ch.to_string()));
                }
            }
        }
    }
    
    /// Tokenise tout le code source
    pub fn tokenize_all(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            let is_eof = token == Token::Eof;
            tokens.push(token);
            
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
}

/// Fonction utilitaire pour tokeniser une chaîne
pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize_all()
}