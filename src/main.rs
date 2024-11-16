mod lang;

fn main() {
    let source = r#"
    INPUTS a, b
    OUTPUTS out
    AND gate1 IN(a, b) OUT(temp)
    NOT gate2 IN(temp) OUT(out)
    "#.to_string();

    let mut lexer = lang::Lexer::new(source);
    
    loop {
        match lexer.get_next_token() {
            Ok(lang::TokenKind::EOF) => break,
            Ok(token) => println!("{:?}", token),
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                break;
            }
        }
    }
}
