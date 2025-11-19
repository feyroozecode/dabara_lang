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
    /// Liste de valeurs
    List(Vec<Value>),
}

impl Value {
    /// Convertit une valeur en chaîne pour l'affichage
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => if *b { "gaskiya".to_string() } else { "karya".to_string() },
            Value::List(elements) => {
                let strings: Vec<String> = elements.iter()
                    .map(|v| v.to_string())
                    .collect();
                format!("[{}]", strings.join(", "))
            },
        }
    }
    
    /// Retourne le type de la valeur pour les messages d'erreur
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "lambar",
            Value::String(_) => "jimla",
            Value::Boolean(_) => "gaskiya ko karya",
            Value::List(_) => "jerin abu",
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
    /// Stack de scopes pour les variables (0 = global, top = local)
    scope_stack: Vec<HashMap<String, Value>>,
    /// Fonctions définies par l'utilisateur
    functions: HashMap<String, Function>,
}

impl Interpreter {
    /// Crée un nouveau interpréteur
    pub fn new() -> Self {
        Interpreter {
            scope_stack: vec![HashMap::new()], // Commence avec le scope global
            functions: HashMap::new(),
        }
    }
    
    /// Pousse un nouveau scope local
    fn push_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
    }
    
    /// Pop le scope courant
    fn pop_scope(&mut self) {
        if self.scope_stack.len() > 1 {
            self.scope_stack.pop();
        }
    }
    
    /// Récupère une variable (cherche du scope local vers le global)
    fn get_variable_value(&self, name: &str) -> Option<Value> {
        // Chercher du scope le plus local au global
        for scope in self.scope_stack.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }
    
    /// Définit une variable dans le scope courant
    fn set_variable_value(&mut self, name: String, value: Value) {
        if let Some(current_scope) = self.scope_stack.last_mut() {
            current_scope.insert(name, value);
        }
    }
    
    /// Exécute un programme complet
    pub fn execute(&mut self, program: Program) -> Result<(), Error> {
        for statement in program.statements {
            if let Some(_) = self.execute_statement(statement)? {
                // Retour au niveau global n'a pas de sens, on l'ignore
            }
        }
        Ok(())
    }
    
    /// Exécute un statement et retourne Some(Value) si c'est un return
    fn execute_statement(&mut self, statement: Statement) -> Result<Option<Value>, Error> {
        match statement {
            Statement::Let { name, value } => {
                let evaluated_value = self.evaluate_expression(value)?;
                self.set_variable_value(name, evaluated_value);
                Ok(None)
            }
            
            Statement::Print(expression) => {
                let value = self.evaluate_expression(expression)?;
                println!("{}", value);
                Ok(None)
            }
            
            Statement::FunctionDef { name, parameters, body } => {
                let function = Function { parameters, body };
                self.functions.insert(name, function);
                Ok(None)
            }
            
            Statement::If { condition, then_branch, else_branch } => {
                let condition_value = self.evaluate_expression(condition)?;
                
                // Évaluer la condition comme booléenne
                let is_true = match condition_value {
                    Value::Boolean(b) => b,
                    Value::Number(n) => n != 0,
                    Value::String(s) => !s.is_empty(),
                    Value::List(l) => !l.is_empty(),
                };
                
                if is_true {
                    // Exécuter la branche then
                    for statement in then_branch {
                        if let Some(return_value) = self.execute_statement(statement)? {
                            return Ok(Some(return_value));
                        }
                    }
                } else if let Some(else_stmt) = else_branch {
                    // Exécuter la branche else
                    if let Some(return_value) = self.execute_statement(*else_stmt)? {
                        return Ok(Some(return_value));
                    }
                }
                
                Ok(None)
            }
            
            Statement::Return(expression) => {
                let value = self.evaluate_expression(expression)?;
                Ok(Some(value))
            }
            
            Statement::Expression(expression) => {
                // Exécuter l'expression et ignorer le résultat
                // Utile pour les appels de fonctions standalone
                self.evaluate_expression(expression)?;
                Ok(None)
            }
        }
    }
    
    /// Évalue une expression
    fn evaluate_expression(&mut self, expression: Expression) -> Result<Value, Error> {
        match expression {
            Expression::Number(n) => Ok(Value::Number(n)),
            Expression::String(s) => Ok(Value::String(s)),
            Expression::Boolean(b) => Ok(Value::Boolean(b)),
            Expression::List(elements) => {
                let mut values = Vec::new();
                for element in elements {
                    let value = self.evaluate_expression(element)?;
                    values.push(value);
                }
                Ok(Value::List(values))
            }
            Expression::Identifier(name) => {
                        self.get_variable_value(&name)
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
            Expression::MethodCall { receiver: _, method: _, arguments: _ } => {
                // TODO: Implémenter les appels de méthode
                Err(Error::runtime_error("Les appels de méthode ne sont pas encore implémentés"))
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
            
            // Opérateurs de comparaison
            (Value::Number(a), BinaryOperator::Equal, Value::Number(b)) => {
                Ok(Value::Boolean(a == b))
            }
            (Value::String(ref a), BinaryOperator::Equal, Value::String(ref b)) => {
                Ok(Value::Boolean(a == b))
            }
            (Value::Boolean(a), BinaryOperator::Equal, Value::Boolean(b)) => {
                Ok(Value::Boolean(a == b))
            }
            
            (Value::Number(a), BinaryOperator::NotEqual, Value::Number(b)) => {
                Ok(Value::Boolean(a != b))
            }
            (Value::String(ref a), BinaryOperator::NotEqual, Value::String(ref b)) => {
                Ok(Value::Boolean(a != b))
            }
            (Value::Boolean(a), BinaryOperator::NotEqual, Value::Boolean(b)) => {
                Ok(Value::Boolean(a != b))
            }
            
            (Value::Number(a), BinaryOperator::Less, Value::Number(b)) => {
                Ok(Value::Boolean(a < b))
            }
            (Value::Number(a), BinaryOperator::Greater, Value::Number(b)) => {
                Ok(Value::Boolean(a > b))
            }
            (Value::Number(a), BinaryOperator::LessEqual, Value::Number(b)) => {
                Ok(Value::Boolean(a <= b))
            }
            (Value::Number(a), BinaryOperator::GreaterEqual, Value::Number(b)) => {
                Ok(Value::Boolean(a >= b))
            }
            
            // Opérations invalides
            (left, op, right) => {
                let op_name = match op {
                    BinaryOperator::Add => "ƙara",
                    BinaryOperator::Subtract => "rage",
                    BinaryOperator::Multiply => "ninka",
                    BinaryOperator::Divide => "raba",
                    BinaryOperator::Concat => "+",
                    BinaryOperator::Equal => "==",
                    BinaryOperator::NotEqual => "!=",
                    BinaryOperator::Less => "<",
                    BinaryOperator::Greater => ">",
                    BinaryOperator::LessEqual => "<=",
                    BinaryOperator::GreaterEqual => ">=",
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
        // Cherche dans le scope global (index 0)
        self.scope_stack.get(0)?.get(name)
    }
    
    /// Définit une variable (pour les tests)
    pub fn set_variable(&mut self, name: String, value: Value) {
        if let Some(global_scope) = self.scope_stack.get_mut(0) {
            global_scope.insert(name, value);
        }
    }
    
    /// Efface toutes les variables
    pub fn clear_variables(&mut self) {
        self.scope_stack.clear();
        self.scope_stack.push(HashMap::new()); // Recréer le scope global
    }
    
    /// Appelle une fonction définie par l'utilisateur
    fn call_function(&mut self, name: String, arguments: Vec<Expression>) -> Result<Value, Error> {
        // Récupérer la définition de la fonction
        let function = self.functions.get(&name)
            .cloned()
            .ok_or_else(|| Error::runtime_error(&format!("Fonction '{}' ba a gani ba (fonction non trouvée)", name)))?;
        
        // Vérifier le nombre d'arguments
        if arguments.len() != function.parameters.len() {
            return Err(Error::runtime_error(&format!(
                "Fonction '{}' tana bukata {} argument(s), amma {} an bayar (fonction '{}' attend {} argument(s), mais {} fourni(s))",
                name, function.parameters.len(), arguments.len(), name, function.parameters.len(), arguments.len()
            )));
        }
        
        // Évaluer les arguments
        let mut arg_values = Vec::new();
        for arg in arguments {
            let value = self.evaluate_expression(arg)?;
            arg_values.push(value);
        }
        
        // Créer un nouveau scope local
        self.push_scope();
        
        // Lier les paramètres aux valeurs des arguments
        for (param, arg_value) in function.parameters.iter().zip(arg_values.iter()) {
            self.set_variable_value(param.clone(), arg_value.clone());
        }
        
        // Exécuter le corps de la fonction
        let mut return_value = Value::Number(0); // Valeur par défaut
        for statement in function.body {
            if let Some(value) = self.execute_statement(statement)? {
                // Un return a été exécuté
                return_value = value;
                break;
            }
        }
        
        // Nettoyer le scope local
        self.pop_scope();
        
        Ok(return_value)
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