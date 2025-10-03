//! Dabara Programming Language v0.0.1
//! 
//! Interface en ligne de commande pour exécuter des programmes Dabara.
//! Usage: dabara <fichier.ha>

use clap::{Arg, Command};
use std::fs;
use std::process;

use dabara::{tokenize, parse, Interpreter, Error};

fn main() {
    let matches = Command::new("dabara")
        .version("0.1.2")
        .about("Dabara Programming Language - Hausa syntax for everyone / Yaren shirye-shirye ta Dabara - Kalmar Hausa don kowa")
        .long_about("Dabara est un langage de programmation utilisant des mots-clés en haoussa.\nIl permet aux locuteurs Hausa d'apprendre et d'utiliser la programmation dans leur langue maternelle.")
        .arg(Arg::new("file")
             .help("Le fichier .ha à exécuter / Fayil .ha da za a gudana")
             .required(true)
             .index(1)
             .value_name("FICHIER"))
        .get_matches();

    let filename = matches.get_one::<String>("file").unwrap();
    
    // Vérifier l'extension du fichier
    if !filename.ends_with(".ha") {
        eprintln!("Kuskure: Fayil {} bai da extension .ha ba", filename);
        eprintln!("Ana buƙatar fayil mai ƙarshe da .ha");
        process::exit(1);
    }
    
    // Exécuter le programme
    if let Err(error) = run_program(filename) {
        eprintln!("{}", error);
        process::exit(1);
    }
}

/// Exécute un programme Dabara
fn run_program(filename: &str) -> Result<(), Error> {
    // Lire le fichier
    let source = fs::read_to_string(filename)
        .map_err(|_| Error::file_not_found(filename))?;
    
    // Afficher le contenu du fichier en mode debug
    if std::env::var("DABARA_DEBUG").is_ok() {
        println!("=== Code source ===");
        println!("{}", source);
        println!("===================");
    }
    
    // Tokeniser
    let tokens = tokenize(&source)?;
    
    if std::env::var("DABARA_DEBUG").is_ok() {
        println!("=== Tokens ===");
        for token in &tokens {
            println!("{:?}", token);
        }
        println!("==============");
    }
    
    // Parser
    let ast = parse(tokens)?;
    
    if std::env::var("DABARA_DEBUG").is_ok() {
        println!("=== AST ===");
        println!("{:#?}", ast);
        println!("===========");
    }
    
    // Exécuter
    let mut interpreter = Interpreter::new();
    interpreter.execute(ast)?;
    
    Ok(())
}
