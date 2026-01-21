//! Interpréteur pour le langage Dabara
//! 
//! Ce module implémente le moteur d'exécution qui évalue l'AST
//! et gère l'environnement des variables.

use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

use crate::parser::{Program, Statement, Expression, BinaryOperator, UnaryOperator};
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
    /// Nombre flottant
    Float(f64),
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
            Value::Float(f) => f.to_string(),
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
            Value::Float(_) => "lambar mai daɗewa",
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
    pub functions: HashMap<String, Function>,
    /// Control flow state for loops
    loop_control: Option<LoopControl>,
}

/// État de contrôle pour les boucles
#[derive(Debug, Clone)]
enum LoopControl {
    Break,
    Continue,
}

impl Interpreter {
    /// Crée un nouveau interpréteur
    pub fn new() -> Self {
        let mut interpreter = Interpreter {
            scope_stack: vec![HashMap::new()], // Commence avec le scope global
            functions: HashMap::new(),
            loop_control: None,
        };
        
        // Register standard library functions
        // TODO: Initialize stdlib when native function support is complete
        // stdlib::register_stdlib(&mut interpreter).unwrap_or_else(|e| {
        //     eprintln!("Warning: Failed to load stdlib: {}", e);
        // });
        
        interpreter
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
    pub fn set_variable_value(&mut self, name: String, value: Value) {
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
                    Value::Float(f) => f != 0.0 && !f.is_nan(),
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

            Statement::While { condition, body } => {
                loop {
                    // Reset loop control at start of each iteration
                    self.loop_control = None;
                    
                    let condition_value = self.evaluate_expression(condition.clone())?;

                    // Évaluer la condition comme booléenne
                    let is_true = match condition_value {
                        Value::Boolean(b) => b,
                        Value::Number(n) => n != 0,
                        Value::Float(f) => f != 0.0 && !f.is_nan(),
                        Value::String(s) => !s.is_empty(),
                        Value::List(l) => !l.is_empty(),
                    };

                    if !is_true {
                        break;
                    }

                    // Exécuter le corps de la boucle
                    for statement in body.clone() {
                        if let Some(return_value) = self.execute_statement(statement)? {
                            return Ok(Some(return_value));
                        }
                        
                        // Handle break/continue
                        if let Some(control) = &self.loop_control {
                            match control {
                                LoopControl::Break => break, // Exit while loop
                                LoopControl::Continue => break, // Continue to next iteration
                            }
                        }
                    }
                    
                    // If continue was called, continue to next iteration
                    if matches!(self.loop_control, Some(LoopControl::Continue)) {
                        continue;
                    }
                    
                    // If break was called, exit the loop
                    if matches!(self.loop_control, Some(LoopControl::Break)) {
                        break;
                    }
                }
                
                // Clear loop control when exiting loop
                self.loop_control = None;
                Ok(None)
            }

            Statement::For { variable, iterable, body } => {
                let iterable_value = self.evaluate_expression(iterable)?;

                match iterable_value {
                    Value::List(elements) => {
                        for element in elements {
                            // Reset loop control at start of each iteration
                            self.loop_control = None;
                            
                            self.set_variable_value(variable.clone(), element);

                            for statement in body.clone() {
                                if let Some(return_value) = self.execute_statement(statement)? {
                                    return Ok(Some(return_value));
                                }
                                
                                // Handle break/continue
                                if let Some(control) = &self.loop_control {
                                    match control {
                                        LoopControl::Break => break, // Exit for loop
                                        LoopControl::Continue => break, // Continue to next iteration
                                    }
                                }
                            }
                            
                            // If continue was called, continue to next iteration
                            if matches!(self.loop_control, Some(LoopControl::Continue)) {
                                continue;
                            }
                            
                            // If break was called, exit the loop
                            if matches!(self.loop_control, Some(LoopControl::Break)) {
                                break;
                            }
                        }
                    }
                    _ => {
                        return Err(Error::runtime_error(
                            "Don loop yana bukata jeri (For loop requires a list)"
                        ));
                    }
                }
                
                // Clear loop control when exiting loop
                self.loop_control = None;
                Ok(None)
            }

            Statement::Break => {
                self.loop_control = Some(LoopControl::Break);
                Ok(None)
            }

            Statement::Continue => {
                self.loop_control = Some(LoopControl::Continue);
                Ok(None)
            }
        }
    }
    
    /// Évalue une expression
    fn evaluate_expression(&mut self, expression: Expression) -> Result<Value, Error> {
        match expression {
            Expression::Number(n) => Ok(Value::Number(n)),
            Expression::Float(f) => Ok(Value::Float(f)),
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
            Expression::Index { object, index } => {
                let obj_value = self.evaluate_expression(*object)?;
                let idx_value = self.evaluate_expression(*index)?;

                match (obj_value, idx_value) {
                    (Value::List(elements), Value::Number(idx)) => {
                        let index = self.normalize_index(idx, elements.len())?;
                        elements.get(index)
                            .cloned()
                            .ok_or_else(|| Error::runtime_error("Lamba ya wuce iyaka (Index out of bounds)"))
                    }
                    (Value::String(s), Value::Number(idx)) => {
                        let chars: Vec<char> = s.chars().collect();
                        let index = self.normalize_index(idx, chars.len())?;
                        chars.get(index)
                            .map(|c| Value::String(c.to_string()))
                            .ok_or_else(|| Error::runtime_error("Lamba ya wuce iyaka (Index out of bounds)"))
                    }
                    _ => {
                        Err(Error::runtime_error("Indexing yana bukata jeri ko jimla (Indexing requires a list or string)"))
                    }
                }
            }
            Expression::MethodCall { receiver, method, arguments } => {
                self.call_method(*receiver, method, arguments)
            }
            Expression::UnaryOp { operator, operand } => {
                let operand_val = self.evaluate_expression(*operand)?;
                match (operator, operand_val) {
                    (UnaryOperator::Negate, Value::Number(n)) => Ok(Value::Number(-n)),
                    (UnaryOperator::Negate, Value::Float(f)) => Ok(Value::Float(-f)),
                    (UnaryOperator::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
                    _ => Err(Error::runtime_error("Invalid unary operation")),
                }
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

            // Float arithmetic
            (Value::Float(a), BinaryOperator::Add, Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Number(a), BinaryOperator::Add, Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
            (Value::Float(a), BinaryOperator::Add, Value::Number(b)) => Ok(Value::Float(a + b as f64)),

            (Value::Float(a), BinaryOperator::Subtract, Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Number(a), BinaryOperator::Subtract, Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
            (Value::Float(a), BinaryOperator::Subtract, Value::Number(b)) => Ok(Value::Float(a - b as f64)),

            (Value::Float(a), BinaryOperator::Multiply, Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Number(a), BinaryOperator::Multiply, Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
            (Value::Float(a), BinaryOperator::Multiply, Value::Number(b)) => Ok(Value::Float(a * b as f64)),

            (Value::Float(a), BinaryOperator::Divide, Value::Float(b)) => {
                if b == 0.0 {
                    Err(Error::runtime_error("Ba za a iya raba da sifili ba (Division par zéro)"))
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            (Value::Number(a), BinaryOperator::Divide, Value::Float(b)) => {
                if b == 0.0 {
                    Err(Error::runtime_error("Ba za a iya raba da sifili ba (Division par zéro)"))
                } else {
                    Ok(Value::Float(a as f64 / b))
                }
            }
            (Value::Float(a), BinaryOperator::Divide, Value::Number(b)) => {
                if b == 0 {
                    Err(Error::runtime_error("Ba za a iya raba da sifili ba (Division par zéro)"))
                } else {
                    Ok(Value::Float(a / b as f64))
                }
            }

            // Float comparisons
            (Value::Float(a), BinaryOperator::Equal, Value::Float(b)) => Ok(Value::Boolean((a - b).abs() < f64::EPSILON)),
            (Value::Float(a), BinaryOperator::NotEqual, Value::Float(b)) => Ok(Value::Boolean((a - b).abs() >= f64::EPSILON)),
            (Value::Float(a), BinaryOperator::Less, Value::Float(b)) => Ok(Value::Boolean(a < b)),
            (Value::Float(a), BinaryOperator::Greater, Value::Float(b)) => Ok(Value::Boolean(a > b)),
            (Value::Float(a), BinaryOperator::LessEqual, Value::Float(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Float(a), BinaryOperator::GreaterEqual, Value::Float(b)) => Ok(Value::Boolean(a >= b)),

            // Mixed int/float comparisons
            (Value::Number(a), BinaryOperator::Less, Value::Float(b)) => Ok(Value::Boolean((a as f64) < b)),
            (Value::Float(a), BinaryOperator::Less, Value::Number(b)) => Ok(Value::Boolean(a < (b as f64))),
            (Value::Number(a), BinaryOperator::Greater, Value::Float(b)) => Ok(Value::Boolean((a as f64) > b)),
            (Value::Float(a), BinaryOperator::Greater, Value::Number(b)) => Ok(Value::Boolean(a > (b as f64))),
            (Value::Number(a), BinaryOperator::LessEqual, Value::Float(b)) => Ok(Value::Boolean((a as f64) <= b)),
            (Value::Float(a), BinaryOperator::LessEqual, Value::Number(b)) => Ok(Value::Boolean(a <= (b as f64))),
            (Value::Number(a), BinaryOperator::GreaterEqual, Value::Float(b)) => Ok(Value::Boolean((a as f64) >= b)),
            (Value::Float(a), BinaryOperator::GreaterEqual, Value::Number(b)) => Ok(Value::Boolean(a >= (b as f64))),

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
                } else if let Ok(float_num) = trimmed.parse::<f64>() {
                    Ok(Value::Float(float_num))
                } else {
                    Ok(Value::String(trimmed.to_string()))
                }
            }
            Err(_) => Err(Error::runtime_error("Ba za a iya karba shigarwa ba (Impossible de lire l'entrée)"))
        }
    }

    /// Normalise un index (gère les indices négatifs)
    fn normalize_index(&self, idx: i64, len: usize) -> Result<usize, Error> {
        if idx < 0 {
            let positive = (len as i64) + idx;
            if positive < 0 {
                return Err(Error::runtime_error("Lamba ya wuce iyaka (Index out of bounds)"));
            }
            Ok(positive as usize)
        } else {
            if idx as usize >= len {
                return Err(Error::runtime_error("Lamba ya wuce iyaka (Index out of bounds)"));
            }
            Ok(idx as usize)
        }
    }

    /// Appelle une méthode sur une valeur
    fn call_method(&mut self, receiver: Expression, method: String, arguments: Vec<Expression>) -> Result<Value, Error> {
        let receiver_value = self.evaluate_expression(receiver)?;

        match (receiver_value, method.as_str()) {
            // String methods
            (Value::String(s), "tsawo") => {
                Ok(Value::Number(s.chars().count() as i64))
            }
            (Value::String(s), "babba") => {
                Ok(Value::String(s.to_uppercase()))
            }
            (Value::String(s), "ƙarami") | (Value::String(s), "karami") => {
                Ok(Value::String(s.to_lowercase()))
            }
            (Value::String(s), "yanki") => {
                if arguments.len() != 2 {
                    return Err(Error::runtime_error("yanki yana bukata arguments 2 (yanki requires 2 arguments)"));
                }
                let start_val = self.evaluate_expression(arguments[0].clone())?;
                let end_val = self.evaluate_expression(arguments[1].clone())?;

                match (start_val, end_val) {
                    (Value::Number(start), Value::Number(end)) => {
                        let chars: Vec<char> = s.chars().collect();
                        let start_idx = start.max(0) as usize;
                        let end_idx = (end.max(0) as usize).min(chars.len());

                        if start_idx <= end_idx && end_idx <= chars.len() {
                            let substring: String = chars[start_idx..end_idx].iter().collect();
                            Ok(Value::String(substring))
                        } else {
                            Err(Error::runtime_error("Yanki ya wuce iyaka (Substring out of bounds)"))
                        }
                    }
                    _ => Err(Error::runtime_error("yanki yana bukata lambobi (yanki requires numbers)"))
                }
            }
            (Value::String(s), "raba") => {
                if arguments.len() != 1 {
                    return Err(Error::runtime_error("raba yana bukata argument 1 (raba requires 1 argument)"));
                }
                let sep_val = self.evaluate_expression(arguments[0].clone())?;

                match sep_val {
                    Value::String(sep) => {
                        let parts: Vec<Value> = s.split(&sep)
                            .map(|p| Value::String(p.to_string()))
                            .collect();
                        Ok(Value::List(parts))
                    }
                    _ => Err(Error::runtime_error("raba yana bukata jimla (raba requires a string)"))
                }
            }

            // List methods
            (Value::List(elements), "tsawo") => {
                Ok(Value::Number(elements.len() as i64))
            }
            (Value::List(mut elements), "ƙara") | (Value::List(mut elements), "kara") => {
                if arguments.len() != 1 {
                    return Err(Error::runtime_error("ƙara yana bukata argument 1 (ƙara requires 1 argument)"));
                }
                let item = self.evaluate_expression(arguments[0].clone())?;
                elements.push(item);
                Ok(Value::List(elements))
            }
            (Value::List(mut elements), "cire") => {
                elements.pop()
                    .ok_or_else(|| Error::runtime_error("Ba za a iya cire daga jerin komai ba (Cannot pop from empty list)"))
            }
            (Value::List(elements), "haɗa") | (Value::List(elements), "hada") => {
                if arguments.len() != 1 {
                    return Err(Error::runtime_error("haɗa yana bukata argument 1 (haɗa requires 1 argument)"));
                }
                let sep_val = self.evaluate_expression(arguments[0].clone())?;

                match sep_val {
                    Value::String(sep) => {
                        let joined = elements.iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(&sep);
                        Ok(Value::String(joined))
                    }
                    _ => Err(Error::runtime_error("haɗa yana bukata jimla (haɗa requires a string)"))
                }
            }

            _ => Err(Error::runtime_error(&format!("Method '{}' ba a gani ba (Method not found)", method)))
        }
    }
}