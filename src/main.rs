mod lang;
mod eval;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "osd")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generates a truth table from the input file
    Generate {
        file_path: String,
    },
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate { file_path } => {
            let source_code = match std::fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    return;
                }
            };
        
            let mut lexer = lang::Lexer::new(source_code);
            let mut tokens = vec![];
            while let Ok(token) = lexer.get_next_token() {
                if token == lang::TokenKind::EOF {
                    break;
                }
                tokens.push(token);
            }
        
            let mut parser = lang::Parser::new(tokens);
            match parser.parse_program() {
                Ok(program) => {
                    eval::print_truth_table(&program);
                }
                Err(err) => println!("Error: {}", err),
            }
        }
    }

    
}
