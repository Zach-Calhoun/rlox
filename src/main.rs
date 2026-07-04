use std::{env::args, fs, io::Write};
pub mod rlox;


fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("Failed to read file");
    println!("Running script from file: {}", path);
    println!("Source code:\n{}", source);
}

fn get_tokens(source: &str) -> Vec<rlox::Token> {
    // Placeholder implementation for tokenization
    // In a real interpreter, this would involve lexical analysis
    let mut scanner = rlox::RloxScanner::new(source.to_string());
    scanner.scan_tokens(raise_error);
    scanner.tokens
}

fn run(source: &str) -> Result<bool, &str> {
    let tokens = get_tokens(source);
    
    // get tokens somehow
    for token in tokens {
        println!("{}", token);
    }

    return Ok(true);
}
fn run_prompt() {
    loop {
        let mut line = String::new();
        std::io::stdout().write_all(b"> ").expect("Failed to write prompt");
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin().read_line(&mut line).expect("Failed to read keyboard input");    
        if line.trim().is_empty() {
            break;
        }
        let run_result = run(&line);
        match run_result {
            Ok(res) => {
                println!("Result: {:?}", res);
            }
            Err(e) => {
                report(1,1, &line, &e);
            }
        } 
    }
}

fn raise_error(line: usize, col: usize, message: &str) {
    report(line, col, "", message);
}

fn report(line: usize, col: usize, what: &str, message: &str) {
    eprintln!("Error: {}", message);
    eprintln!("");
    eprintln!("{} | {}", line, what);
    eprintln!("{:>col$}^--Here", col);
    eprintln!("[line {}:{}] Error {}: {}", line, col, &what, &message);
}

fn main() {
    println!("Init rlox interpreter");

    let args: Vec<String> = args().into_iter().collect();
   
    
    let script_path = args.get(1);
    for(i, arg) in args.iter().enumerate() {
        println!("arg {}: {}", i, arg);
    }

    match script_path {
        Some(path) => {
            run_file(&path);
        }
        None => {
            println!("No script file provided. Entering interactive mode");
            run_prompt();
        }
    }
}
