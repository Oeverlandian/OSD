mod lang;
mod eval;

fn main() {

    let source_code = match std::fs::read_to_string("program_1.osd") {
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
        Ok(program) => println!("{:?}", program),
        Err(err) => println!("Error: {}", err),
    }

}
