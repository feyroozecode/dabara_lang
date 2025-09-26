//! Lexer pour le langage Dabara
//! 
//! Ce module implémente la tokenisation du code source Dabara,
//! avec support complet des caractères Unicode haoussa.

use crate::error::Error;

/// Types de tokens dans Dabara
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Mots-clés Hausa
    Begin,      // fara
    End,        // ƙare  
    Print,      // rubuta
    Let,        // naɗa
    True,       // gaskiya
    False,      // karya
    Plus,       // ƙara
    Minus,      // rage
    
    // Littéraux
    Identifier(String),
    Number(i64),
    String(String),
    
    // Opérateurs
    Equals,     // =
    
    // Délimiteurs
    Newline,
    Eof,
}

impl Token {
    /// Convertit un mot en token si c'est un mot-clé
    fn from_keyword(word: &str) -> Option<Token> {
        match word {
            // Versions originales avec caractères haoussa
            "fara" => Some(Token::Begin),
            "ƙare" => Some(Token::End),
            "rubuta" => Some(Token::Print),
            "naɗa" => Some(Token::Let),
            "gaskiya" => Some(Token::True),
            "karya" => Some(Token::False),
            "ƙara" => Some(Token::Plus),
            "rage" => Some(Token::Minus),
            
            // Versions alternatives avec caractères latins
            "kare" => Some(Token::End),      // Alternative pour ƙare
            "nada" => Some(Token::Let),      // Alternative pour naɗa  
            "kara" => Some(Token::Plus),     // Alternative pour ƙara
            
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
    
    /// Lit un nombre
    fn read_number(&mut self) -> i64 {
        let mut number_str = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                number_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        number_str.parse().unwrap_or(0)
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
                
                Some('=') => {
                    self.advance();
                    return Ok(Token::Equals);
                }
                
                Some('+') => {
                    self.advance();
                    return Ok(Token::Plus);
                }
                
                Some('"') => {
                    let string_val = self.read_string()?;
                    return Ok(Token::String(string_val));
                }
                
                Some(ch) if ch.is_ascii_digit() => {
                    let number = self.read_number();
                    return Ok(Token::Number(number));
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