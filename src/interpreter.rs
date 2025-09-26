//! Interpréteur pour le langage Dabara
//! 
//! Ce module implémente le moteur d'exécution qui évalue l'AST
//! et gère l'environnement des variables.

use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

use crate::parser::{Program, Statement, Expression, BinaryOperator};
use crate::error::Error;

/// Représente une fonction définie par l'utilisateur
#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<String>,
    pub body: Vec<Statement>,
}

/// Types de valeurs dans Dabara
#[derive(Debug, Clone)]
pub enum Value {
    /// Nombre entier
    Number(i64),
    /// Chaîne de caractères
    String(String),
    /// Valeur booléenne
    Boolean(bool),
}

impl Value {
    /// Convertit une valeur en chaîne pour l'affichage
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => if *b { "gaskiya".to_string() } else { "karya".to_string() },
        }
    }
    
    /// Retourne le type de la valeur pour les messages d'erreur
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "lambar",
            Value::String(_) => "jimla",
            Value::Boolean(_) => "gaskiya ko karya",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Interpréteur pour exécuter les programmes Dabara
pub struct Interpreter {
    /// Environnement des variables
    variables: HashMap<String, Value>,
    /// Fonctions définies par l'utilisateur
    functions: HashMap<String, Function>,
}

impl Interpreter {
    /// Crée un nouveau interpréteur
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
    
    /// Exécute un programme complet
    pub fn execute(&mut self, program: Program) -> Result<(), Error> {
        for statement in program.statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }
    
    /// Exécute un statement
    fn execute_statement(&mut self, statement: Statement) -> Result<(), Error> {
        match statement {
            Statement::Let { name, value } => {
                let evaluated_value = self.evaluate_expression(value)?;
                self.variables.insert(name, evaluated_value);
                Ok(())
            }
            
            Statement::Print(expression) => {
                let value = self.evaluate_expression(expression)?;
                println!("{}", value);
                Ok(())
            }
            
            Statement::FunctionDef { name, parameters, body } => {
                let function = Function { parameters, body };
                self.functions.insert(name, function);
                Ok(())
            }
        }
    }
    
    /// Évalue une expression
    fn evaluate_expression(&mut self, expression: Expression) -> Result<Value, Error> {
        match expression {
            Expression::Number(n) => Ok(Value::Number(n)),
            
            Expression::String(s) => Ok(Value::String(s)),
            
            Expression::Boolean(b) => Ok(Value::Boolean(b)),
            
            Expression::Identifier(name) => {
                self.variables.get(&name)
                    .cloned()
                    .ok_or_else(|| Error::variable_not_found(&name))
            }
            
            Expression::BinaryOp { left, operator, right } => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;
                
                self.evaluate_binary_operation(left_val, operator, right_val)
            }
            
            Expression::FunctionCall { name, arguments } => {
                self.call_function(name, arguments)
            }
            
            Expression::Input => {
                self.get_user_input()
            }
        }
    }
    
    /// Évalue une opération binaire
    fn evaluate_binary_operation(
        &self,
        left: Value,
        operator: BinaryOperator,
        right: Value,
    ) -> Result<Value, Error> {
        match (left, operator, right) {
            // Addition de nombres
            (Value::Number(a), BinaryOperator::Add, Value::Number(b)) => {
                Ok(Value::Number(a + b))
            }
            
            // Soustraction de nombres
            (Value::Number(a), BinaryOperator::Subtract, Value::Number(b)) => {
                Ok(Value::Number(a - b))
            }
            
            // Multiplication de nombres
            (Value::Number(a), BinaryOperator::Multiply, Value::Number(b)) => {
                Ok(Value::Number(a * b))
            }
            
            // Division de nombres
            (Value::Number(a), BinaryOperator::Divide, Value::Number(b)) => {
                if b == 0 {
                    Err(Error::runtime_error("Ba za a iya raba da sifili ba (Division par zéro)"))
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            
            // Concaténation de chaînes avec +
            (Value::String(a), BinaryOperator::Add, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            
            // Concaténation chaîne + nombre
            (Value::String(a), BinaryOperator::Add, Value::Number(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            
            // Concaténation nombre + chaîne
            (Value::Number(a), BinaryOperator::Add, Value::String(b)) => {
                Ok(Value::String(format!("{}{}", a, b)))
            }
            
            // Concaténation avec booléens
            (Value::String(a), BinaryOperator::Add, Value::Boolean(b)) => {
                let bool_str = if b { "gaskiya" } else { "karya" };
                Ok(Value::String(format!("{}{}", a, bool_str)))
            }
            
            (Value::Boolean(a), BinaryOperator::Add, Value::String(b)) => {
                let bool_str = if a { "gaskiya" } else { "karya" };
                Ok(Value::String(format!("{}{}", bool_str, b)))
            }
            
            // Concaténation explicite
            (left, BinaryOperator::Concat, right) => {
                Ok(Value::String(format!("{}{}", left.to_string(), right.to_string())))
            }
            
            // Opérations invalides
            (left, op, right) => {
                let op_name = match op {
                    BinaryOperator::Add => "ƙara",
                    BinaryOperator::Subtract => "rage",
                    BinaryOperator::Multiply => "ninka",
                    BinaryOperator::Divide => "raba",
                    BinaryOperator::Concat => "+",
                };
                
                Err(Error::invalid_operation(
                    op_name,
                    left.type_name(),
                    right.type_name(),
                ))
            }
        }
    }
    
    /// Retourne la valeur d'une variable (pour les tests)
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    
    /// Définit une variable (pour les tests)
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    /// Efface toutes les variables
    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }
    
    /// Appelle une fonction définie par l'utilisateur
    fn call_function(&mut self, name: String, _arguments: Vec<Expression>) -> Result<Value, Error> {
        // TODO: Implémenter l'appel de fonction avec un environnement local
        // Pour l'instant, retourner une erreur
        Err(Error::runtime_error(&format!("Fonction '{}' ba ta da aiki ba tukuna (pas encore implémentée)", name)))
    }
    
    /// Récupère l'entrée utilisateur
    fn get_user_input(&mut self) -> Result<Value, Error> {
        print!("Rubuta abu: "); // "Écris quelque chose: "
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                
                // Essayer de parser comme nombre d'abord
                if let Ok(num) = trimmed.parse::<i64>() {
                    Ok(Value::Number(num))
                } else {
                    Ok(Value::String(trimmed.to_string()))
                }
            }
            Err(_) => Err(Error::runtime_error("Ba za a iya karba shigarwa ba (Impossible de lire l'entrée)"))
        }
    }
}